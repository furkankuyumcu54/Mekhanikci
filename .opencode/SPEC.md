# SPEC

## System Architecture

```
┌─────────────────────────────────────────────┐
│              mekhanikci-tui                    │
│  Ratatui + Crossterm (chat, file preview)    │
└─────────────────────┬───────────────────────┘
                      │ natural language prompt
                      ▼
┌─────────────────────────────────────────────┐
│              mekhanikci-llm                    │
│  Ollama HTTP client, prompt manager,         │
│  JSON extraction, validation, retry          │
│  ┌─────────────────────────────────────────┐ │
│  │  Qwen 3.5 4B (local via Ollama)        │ │
│  │  Temperature 0.0, JSON mode             │ │
│  │  Step 1: Extract User Requirements      │ │
│  │  Step 2: Apply engineering rules        │ │
│  │  (single LLM call for MVP)              │ │
│  └─────────────────────────────────────────┘ │
└─────────────────────┬───────────────────────┘
                      │ Design Spec (JSON)
                      ▼
┌─────────────────────────────────────────────┐
│              mekhanikci-core                   │
│                                              │
│  1. Design Specification Layer               │
│     Parse & validate spec, resolve params    │
│                                              │
│  2. CAD Model Layer                          │
│     Spec → backend-independent CAD tree      │
│                                              │
│  3. CAD Backend Layer                        │
│     Walk CAD tree → output format            │
│     ├─ OpenSCADBackend (MVP)                 │
│     ├─ CadQueryBackend (future)              │
│     └─ FreeCADBackend (future)               │
└─────────────────────┬───────────────────────┘
                      │ .scad / .stl / .csv
                      ▼
                 ./output/<timestamp>/
```

The LLM never generates CAD code. It produces structured JSON (Design Spec). Rust transforms the spec into a backend-independent CAD Model tree, and a backend walks that tree to produce files.

---

## Crate Structure

### mekhanikci-core

The deterministic pipeline. No I/O except final file writes. No async.

```
src/
├── design/
│   ├── mod.rs          # DesignSpec trait
│   ├── conveyor.rs     # ConveyorDesign struct + impl DesignSpec
│   └── motor.rs        # MotorSpec sub-types
├── cad/
│   ├── mod.rs          # CadNode, CadPart, CadPrimitive, CadAssembly, Transform
│   └── visitor.rs      # Tree-walking utilities
├── backend/
│   ├── mod.rs          # CadBackend trait
│   └── openscad.rs     # OpenSCADBackend: CAD model → .scad → .stl
├── output.rs           # Output directory creation, file writing
└── lib.rs
```

### mekhanikci-llm

Connected to Ollama. Prompt construction, JSON extraction, validation, retry.

```
src/
├── client.rs           # HTTP client to Ollama REST API
├── prompt.rs           # System prompt + few-shot examples + user prompt
├── parser.rs           # JSON extraction from LLM response
├── validation.rs       # Field-level range checks
└── lib.rs
```

### mekhanikci-tui

Terminal UI. Captures input, displays conversation, shows results.

```
src/
├── app.rs              # Main application state and event loop
├── widgets/
│   ├── chat.rs         # Scrollable chat view
│   ├── input.rs        # Multi-line text input
│   ├── status.rs       # Status bar with loading/error/results
│   └── bom.rs          # BOM table display
├── session.rs          # Conversation history (in-memory + JSON-L)
└── lib.rs
```

---

## Layer 1: Mechanical Design Specification

The Design Spec is the contract between the LLM and the engine. It captures engineering intent with no geometry or backend details.

### Principles

1. **No geometry** — no coordinates, no CSG operations, no transformation matrices
2. **No backend details** — no OpenSCAD references, no rendering hints
3. **Human-readable** — readable and editable in any text editor
4. **Validatable** — every field has type, range, and optional default
5. **Self-contained** — one file captures everything needed to reproduce a design

### DesignSpec Trait

```rust
trait DesignSpec {
    type CadModel: CadAssembly;
    fn to_cad_model(self) -> Result<Self::CadModel>;
}
```

Every machine type has its own spec struct. All implement the same trait.

### Validation (Two-Stage)

| Stage | What | How |
|-------|------|-----|
| Structural | JSON must deserialize to the correct type | Serde |
| Semantic | Range checks, enum checks, pattern checks | Custom validators |

Validation errors include field name, invalid value, and expected constraint. Fed back to the LLM on retry.

### Spec → CAD Model Relationship

| Spec Value | CAD Model Decision |
|------------|-------------------|
| `frame_extrusion: "40x40"` | Two Box primitives at frame edges |
| `length_mm: 2000` | Box length = 2000 |
| `roller_diameter_mm: 50` | Cylinder with r=25, at frame ends |
| `motor.frame: "nema23"` | Motor mount plate with NEMA23 bolt pattern |
| `support_legs: true` | Four leg assemblies at corners |

### Evolution Strategy

- **Adding fields**: New optional fields with defaults are backward compatible
- **Adding machine types**: New spec struct + `impl DesignSpec`. LLM prompt gains new schema
- **Deprecating fields**: Mark `#[deprecated]`, remove in next major version

---

## Layer 2: CAD Model

Backend-independent 3D geometry representation between Design Spec and CAD Backend.

### Core Types

```rust
enum CadNode {
    Part(CadPart),
    Assembly(CadAssembly),
}

struct CadPart {
    name: String,
    primitives: Vec<CadPrimitive>,
}

enum CadPrimitive {
    Box { x: f64, y: f64, z: f64 },
    Cylinder { r: f64, h: f64 },
    // Future: Sphere, Polyhedron, Extrude, Revolution
}

struct CadAssembly {
    name: String,
    children: Vec<Child>,
}

struct Child {
    node: CadNode,
    transform: Transform,
}

struct Transform {
    translation: [f64; 3],  // [x, y, z] mm
    rotation: [f64; 3],     // [x, y, z] degrees
}
```

### Tree Structure

```
CadAssembly("belt_conveyor")
├── CadAssembly("frame")
│   ├── CadPart("left_rail")   → Box { 2000, 40, 40 }
│   ├── CadPart("right_rail")  → Box { 2000, 40, 40 }
│   └── CadPart("cross_brace") → Box { 540, 40, 40 }  (×N)
├── CadAssembly("rollers")
│   ├── CadPart("drive_roller") → Cylinder { r:25, h:520 }
│   └── CadPart("idler_roller") → Cylinder { r:25, h:520 }
├── CadPart("belt_surface")     → Box { 1800, 500, 5 }
├── CadAssembly("motor_mount")  → mount_plate + motor_body
└── CadAssembly("support_leg")  → leg_post + foot_pad + cross_brace (×4)
```

### Serialization

The CAD Model is serializable to JSON for debugging, testing, and replay without re-running spec-to-CAD.

---

## Layer 3: CAD Backend

### Backend Trait

```rust
trait CadBackend {
    fn render(&self, model: &CadAssembly, output_dir: &Path) -> Result<OutputFiles>;
}
```

| Backend | Walk Strategy | Output |
|---------|--------------|--------|
| OpenSCAD (MVP) | Depth-first, emit `module` + CSG calls | `.scad` → `.stl` |
| CadQuery (future) | Depth-first, emit `cq.Workplane` calls | `.step` |
| FreeCAD (future) | Depth-first, emit `App.ActiveDocument` API | `.fcstd` |

### Primitives Expansion

| Primitive | MVP | Add When |
|-----------|-----|----------|
| Box | ✅ | — |
| Cylinder | ✅ | — |
| Sphere | ❌ | Ball joints, casters |
| Polyhedron | ❌ | Custom mesh shapes |
| Extrude | ❌ | Custom extrusion profiles |
| Revolution | ❌ | Lathe-cut shapes |

---

## ConveyorDesign Schema

### Top-Level Structure

```rust
struct ConveyorDesign {
    /// ── User Requirements ──
    /// Conveyor belt length in mm. Range: 100 – 10000
    pub length_mm: f64,
    /// Belt width in mm. Range: 10 – 2000
    pub belt_width_mm: f64,
    /// Design load capacity in kg (optional, affects sizing)
    pub load_capacity_kg: Option<f64>,
    /// Belt speed in m/s (optional, informational in MVP)
    pub speed_m_per_s: Option<f64>,

    /// ── Engineering Decisions ──
    pub motor: MotorSpec,
    /// Frame extrusion profile cross-section. Default: "40x40"
    pub frame_extrusion: String,
    /// Roller diameter in mm. Default: 50.0
    pub roller_diameter_mm: f64,
    /// Height floor→belt in mm. Default: 900.0
    pub height_mm: f64,
    /// Generate support legs. Default: true
    pub support_legs: bool,
    /// Belt surface type. Default: "flat"
    pub belt_type: String,
}

struct MotorSpec {
    pub frame: String,   // "nema17" | "nema23" | "nema34"
    pub mount: String,   // "underneath" | "side" | "end"
}
```

### Defaults

| Field | Default | Rationale |
|-------|---------|-----------|
| `frame_extrusion` | `"40x40"` | Common aluminum extrusion |
| `roller_diameter_mm` | `50.0` | Standard small conveyor roller |
| `height_mm` | `900.0` | Ergonomic working height |
| `support_legs` | `true` | Most conveyors need legs |
| `belt_type` | `"flat"` | Simplest and most common |

### Validation Rules

- `length_mm`: 100–10000
- `belt_width_mm`: 10–2000
- `roller_diameter_mm`: 10–200
- `height_mm`: 100–2000
- `frame_extrusion`: pattern `NNxNN` (e.g. `40x40`, `30x60`)
- `motor.frame`: one of `nema17`, `nema23`, `nema34`
- `motor.mount`: one of `underneath`, `side`, `end`
- `belt_type`: one of `flat`, `cleated`, `v-belt`

### JSON Example

```json
{
  "length_mm": 2000,
  "belt_width_mm": 500,
  "motor": { "frame": "nema23", "mount": "underneath" },
  "frame_extrusion": "40x40",
  "roller_diameter_mm": 50,
  "height_mm": 900,
  "support_legs": true,
  "belt_type": "flat",
  "load_capacity_kg": 50,
  "speed_m_per_s": 0.5
}
```

---

## LLM Prompt Template

### System Prompt

```
You are a conveyor design extractor. Your only job is to produce valid JSON
conforming to the ConveyorDesign schema below.

RULES:
- Output ONLY valid JSON. No explanation, no markdown.
- All lengths are in MILLIMETERS.
- All mass in KILOGRAMS.
- Speed in METERS PER SECOND.
- If the user omits a field with a default, include it with the default value.
- If the user omits an optional field, set it to null.

SCHEMA:
{
  "length_mm": <number 100-10000, required>,
  "belt_width_mm": <number 10-2000, required>,
  "motor": {
    "frame": <"nema17" | "nema23" | "nema34", required>,
    "mount": <"underneath" | "side" | "end", required>
  },
  "frame_extrusion": <string pattern "NNxNN", default "40x40">,
  "roller_diameter_mm": <number 10-200, default 50>,
  "height_mm": <number 100-2000, default 900>,
  "support_legs": <boolean, default true>,
  "belt_type": <"flat" | "cleated" | "v-belt", default "flat">,
  "load_capacity_kg": <number | null, optional>,
  "speed_m_per_s": <number | null, optional>
}

EXAMPLES:
User: "2 meter conveyor with 500 mm belt and NEMA23 motor"
Assistant:
{"length_mm":2000,"belt_width_mm":500,"motor":{"frame":"nema23","mount":"underneath"},"frame_extrusion":"40x40","roller_diameter_mm":50,"height_mm":900,"support_legs":true,"belt_type":"flat","load_capacity_kg":null,"speed_m_per_s":null}
```

Temperature: 0.0. JSON mode enabled (`"format": "json"`).

---

## OpenSCAD Generation

### Module Structure

```
conveyor.scad
├── module conveyor_frame()
├── module drive_roller()
├── module idler_roller()
├── module belt_surface()
├── module motor_mount()
├── module support_leg()
├── module conveyor_assembly()
└── conveyor_assembly();
```

### Frame

Two parallel extrusions of length L, separated by belt_width + gap. Cross-braces every 500mm.

### Rollers

Cylinder at each end of the frame. Diameter = `roller_diameter_mm`, length = `belt_width_mm + 20`.

### Belt

Flat rectangular surface on top of rollers. Thickness = 5mm (fixed for MVP).

### Motor Mount

| Mount Type | Geometry |
|------------|----------|
| Underneath | Plate bolted to frame underside. Bolt pattern per NEMA frame (NEMA17: 4×M3 on 47.1mm square, NEMA23: 4×M4 on 56.4mm, NEMA34: 4×M6 on 74.0mm) |
| Side | L-bracket on frame side face. Shaft aligns with roller axis |
| End | Plate on frame end face. Shaft extends into roller |

### Support Legs

4 legs at frame corners. Cross-braces at 45°. Foot pads 50×50×5mm. Height adjusts to `height_mm`.

### BOM CSV

```csv
Part,Quantity,Notes
Extrusion 40x40 (frame rail),2,length_mm
Extrusion 40x40 (cross-brace),N,every 500mm
Drive roller,1,belt_width_mm + 20mm
Idler roller,1,belt_width_mm + 20mm
Belt (flat),1,belt_width_mm x length_mm
Motor mount bracket,1,motor.frame
NEMA23 motor,1,frame: nema23
Support leg,4,height_mm
Leg foot pad,4,50x50x5
Leg cross-brace,4,
M3 bolt,16,fastener; qty varies by NEMA frame
```

---

## TUI Layout

```
┌──────────────────────────────────────────────────────────┐
│  Mekhanikci v0.1                       [Esc: quit]       │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  You: design a 2m conveyor with 500mm belt and NEMA23    │
│                                                          │
│  ┌────────────────────────────────────────────────────┐  │
│  │ ✓ Conveyor generated                               │  │
│  │   Length: 2000 mm  Belt width: 500 mm              │  │
│  │   Motor: NEMA23 (underneath)                       │  │
│  │   Files: conveyor.scad ✓  conveyor.stl ✓  bom.csv ✓│  │
│  └────────────────────────────────────────────────────┘  │
│                                                          │
│  You: shorten to 1.7m                                    │
│  (waiting...)                                             │
├──────────────────────────────────────────────────────────┤
│  > design a 2m conveyor with 500mm belt and NEMA23     │
│  [Ctrl+Enter to send]                                    │
└──────────────────────────────────────────────────────────┘
```

| Region | Widget | % Height |
|--------|--------|----------|
| Top bar | Title + keybindings | 1 line |
| Chat area | Scrollable conversation | ~80% |
| Input line | Text input with prompt prefix | 3 lines |
| Status bar | Loading, errors, file paths | 1 line |

| Key | Action |
|-----|--------|
| Enter | Submit input |
| Esc | Exit |
| Ctrl+U | Clear input |
| Tab | Focus toggle (chat / input) |
| PgUp/PgDn | Scroll history |

---

## Error Handling

| Scenario | Message | Action |
|----------|---------|--------|
| Ollama unreachable | "Cannot connect to Ollama. Is it running?" | Retry prompt |
| LLM returns invalid JSON | "Failed to parse design. Retrying..." | Re-prompt + last error |
| Validation failure | "Length must be 100-10000 mm. Got: 0" | Re-prompt with error |
| OpenSCAD not found | "OpenSCAD binary not found" | Show config instructions |
| Render failure | "OpenSCAD rendering failed." | Show stderr |
| All retries exhausted | "Unable to generate after 3 attempts." | Return error to TUI |

**Retry strategy:** LLM JSON extraction — 3 retries with exponential backoff (1s, 2s, 4s). OpenSCAD — no retry (deterministic, failure is a bug).

---

## Configuration

```toml
[llm]
model = "qwen3.5:4b"
ollama_url = "http://localhost:11434"
temperature = 0.0
retry_limit = 3

[openscad]
binary_path = "/usr/bin/openscad"
```

Config search order: `./mekhanikci.toml` → `~/.config/mekhanikci/config.toml` → defaults.

---

## Output Layout

```
output/
  conveyor_20250621_143000/
    design.json        # ConveyorDesign JSON (the record)
    conveyor.scad      # OpenSCAD source (human-readable)
    conveyor.stl       # Rendered STL (binary)
    bom.csv            # Bill of Materials
```

---

## Determinism Guarantee

Same input → same output at every layer:
1. **Specification:** Same JSON + same engine version → same CAD Model
2. **CAD Model:** Same tree → same backend output (OpenSCAD is deterministic)
3. **LLM:** Temperature 0.0, JSON mode, fixed prompt. Retry + validation handles variance

## Key Design Decisions

| Decision | Rationale |
|----------|-----------|
| Requirements as explicit concept | Separates problem from solution, enables future fine-tuned models |
| Spec is backend-agnostic | Future backends don't require prompt changes |
| CAD Model is explicit | Separates "what to build" from "how to render" |
| Single-pass tree walk | No constraint solver or graph library needed for MVP |
| LLM → Spec only | LLM never touches geometry or CAD code |
| MVP: single LLM call | Extracts requirements AND applies engineering rules in one step |
