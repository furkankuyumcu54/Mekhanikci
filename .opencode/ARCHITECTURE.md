# Architecture

## System Overview

Mekhanikçi is a 3-crate Rust workspace. Data flows in one direction through five conceptual layers:

```
┌──────────────────────────────────────────────────────────┐
│                      mekhanikci-tui                        │
│  Ratatui + Crossterm (chat interface, file preview)      │
└──────────────────────┬───────────────────────────────────┘
                       │ natural language prompt
                       ▼
┌──────────────────────────────────────────────────────────┐
│                      mekhanikci-llm                        │
│  Ollama HTTP client, prompt manager, JSON validation     │
│  ┌────────────────────────────────────────────────────┐  │
│  │  Qwen 3.5 4B (local via Ollama)                    │  │
│  │  Temperature 0.0, JSON mode, few-shot prompt       │  │
│  │                                                    │  │
│  │  Step 1: Extract User Requirements from prompt     │  │
│  │  Step 2: Apply engineering rules → Design Spec     │  │
│  │  (single LLM call for MVP)                         │  │
│  └────────────────────────────────────────────────────┘  │
└──────────────────────┬───────────────────────────────────┘
                       │ User Requirements → Design Spec
                       ▼
┌──────────────────────────────────────────────────────────┐
│                      mekhanikci-core                       │
│                                                          │
│  ┌────────────────────────────────────────────────────┐  │
│  │  1. Design Specification Layer                     │  │
│  │     Parse & validate spec (e.g. ConveyorDesign)    │  │
│  │     Resolve parameters, apply defaults, unit conv. │  │
│  └──────────────────┬─────────────────────────────────┘  │
│                     │ validated spec                     │
│  ┌──────────────────▼─────────────────────────────────┐  │
│  │  2. CAD Model Layer                                │  │
│  │     Transform spec into backend-independent CAD    │  │
│  │     Build assembly tree (CadAssembly, CadPart,     │  │
│  │     CadPrimitive, Transform)                       │  │
│  └──────────────────┬─────────────────────────────────┘  │
│                     │ CAD Model tree                     │
│  ┌──────────────────▼─────────────────────────────────┐  │
│  │  3. CAD Backend Layer                              │  │
│  │     Walk CAD model tree → output format            │  │
│  │     ┌─ OpenSCADBackend (MVP)                       │  │
│  │     ├─ CadQueryBackend (future)                    │  │
│  │     └─ FreeCADBackend (future)                     │  │
│  └────────────────────────────────────────────────────┘  │
└──────────────────────┬───────────────────────────────────┘
                       │ .scad / .stl / .csv
                       ▼
                  ./output/<timestamp>/
```

The LLM never generates CAD code. It extracts **User Requirements** from the natural language prompt — the raw problem statement: length, product weight, throughput, speed.

The LLM then applies engineering rules to transform those requirements into a **Mechanical Design Specification** — a structured description of design intent with engineering parameters (motor frame, roller diameter, extrusion profile).

For MVP, both steps happen in a single LLM call. The conceptual separation enables future fine-tuned models to handle the engineering decision-making step independently.

Rust transforms the spec into a **backend-independent CAD Model** (assemblies, parts, primitives, transforms).

A **CAD Backend** walks the CAD Model and produces the final file format.

---

## Layer Detail

### Layer 0: User Requirements

**File:** `mekhanikci-llm/src/requirements/` (future)

User Requirements are the raw problem statement extracted from natural language. They describe **what the user needs**, not **how to build it**.

```json
{
  "length_mm": 10000,
  "product_weight_kg": 1.0,
  "throughput_per_minute": 15,
  "belt_width_mm": 400
}
```

Key properties:
- **Problem-oriented** — describes the task, not the solution.
- **Engineering-agnostic** — no motor choices, no extrusion profiles, no roller sizes.
- **LLM-extracted** — the LLM parses these from natural language.
- **Transformation input** — engineering rules turn requirements into a Design Spec.

For MVP, User Requirements are extracted and transformed in a single LLM call. The requirements concept is documented here so the architecture can evolve toward explicit requirements→spec transformation with fine-tuned models.

### Layer 1: Mechanical Design Specification

**File:** `mekhanikci-core/src/design/`

The spec is the contract between the LLM and the engine. It captures design intent — the engineering solution — without any geometry or CAD details.

```rust
// Example: ConveyorDesign (MVP)
struct ConveyorDesign {
    length_mm: f64,
    belt_width_mm: f64,
    motor: MotorSpec,
    frame_extrusion: String,
    roller_diameter_mm: f64,
    height_mm: f64,
    support_legs: bool,
    belt_type: String,
    load_capacity_kg: Option<f64>,
    speed_m_per_s: Option<f64>,
}
```

Key properties:
- **No geometry** — no coordinates, no CSG operations, no transformation matrices.
- **No backend details** — no OpenSCAD references, no rendering hints.
- **Human-readable** — an engineer can read and edit the JSON directly.
- **Validatable** — Serde deserialization + range checks catch errors before any computation.

Every machine type has its own spec struct. All specs share a common trait:

```rust
trait DesignSpec {
    type CadModel: CadAssembly;
    fn to_cad_model(self) -> Result<Self::CadModel>;
}
```

### Layer 2: CAD Model

**File:** `mekhanikci-core/src/cad/`

A backend-independent representation of 3D geometry built from primitives, parts, assemblies, and transforms.

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
    // Future: Sphere, Polyhedron, Extrude, etc.
}

struct CadAssembly {
    name: String,
    children: Vec<(CadNode, Transform)>,
}

struct Transform {
    translation: [f64; 3],
    rotation: [f64; 3],
}
```

Key properties:
- **No backend dependencies** — pure data, no OpenSCAD strings, no CadQuery API calls.
- **Tree structure** — assemblies contain parts or sub-assemblies, each with a transform.
- **Serializable** — can be saved as JSON for debugging and replay.
- **Extensible** — new primitives can be added without changing backends.

### Layer 3: CAD Backend

**File:** `mekhanikci-core/src/backend/`

A trait that renders a CAD Model into a specific output format:

```rust
trait CadBackend {
    fn render(&self, model: &CadAssembly, output_dir: &Path) -> Result<OutputFiles>;
}
```

| Backend | Status | Output |
|---------|--------|--------|
| `OpenSCADBackend` | MVP | `.scad`, `.stl` |
| `CadQueryBackend` | Future | `.step` |
| `FreeCADBackend` | Future | `.fcstd`, `.step` |

Backends are selected by configuration. The engine does not know which backend will render; it only produces the CAD Model.

---

## Crate Structure

### `mekhanikci-core`

The entire deterministic pipeline. No I/O except final file writes. No async.

```
src/
├── design/
│   ├── mod.rs          # DesignSpec trait
│   ├── conveyor.rs     # ConveyorDesign struct + impl DesignSpec
│   └── motor.rs        # MotorSpec sub-types
├── cad/
│   ├── mod.rs          # CadNode, CadPart, CadPrimitive, CadAssembly, Transform
│   └── visitor.rs      # Tree-walking utilities (for backends)
├── backend/
│   ├── mod.rs          # CadBackend trait
│   └── openscad.rs     # OpenSCADBackend: CAD model → .scad → .stl
├── output.rs           # Output directory creation, file writing
└── lib.rs
```

### `mekhanikci-llm`

Connected to Ollama. Handles prompt construction, JSON extraction, validation, retry.

```
src/
├── client.rs           # HTTP client to Ollama REST API
├── prompt.rs           # System prompt + few-shot examples + user prompt
├── parser.rs           # JSON extraction from LLM response
├── validation.rs       # Field-level range checks
└── lib.rs
```

### `mekhanikci-tui`

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

## Data Flow (Conveyor Example)

```
User: "Design a 10 meter conveyor that transports 1 kg products at 15 per minute"

  │
  ▼  mekhanikci-tui captures prompt, sends to mekhanikci-llm
  │
  ▼  mekhanikci-llm calls Ollama with system prompt + user text
  │
  ▼  LLM extracts User Requirements:
  │    length_mm: 10000, product_weight_kg: 1.0,
  │    throughput_per_minute: 15
  │
  ▼  LLM applies engineering rules → Design Spec:
  │    length_mm: 10000, belt_width_mm: 400,
  │    motor.frame: "nema23", roller_diameter_mm: 60,
  │    frame_extrusion: "40x40", motor_power_w: 180
  │
  ▼  Ollama returns JSON → deserialize → validate → retry if bad
  │
  ▼  ConveyorDesign { length_mm: 10000, belt_width_mm: 400, ... }
  │
  ▼  mekhanikci-core::design::conveyor::to_cad_model()
  │     ↓
  │     frame_rails    → 2x Box primitives with transforms
  │     cross_braces   → Nx Box primitives
  │     drive_roller   → 1x Cylinder primitive
  │     idler_roller   → 1x Cylinder primitive
  │     belt_surface   → 1x Box primitive
  │     motor_mount    → 1x CadAssembly (plate + bolt pattern)
  │     support_legs   → 4x CadAssembly (leg + foot + brace)
  │     ↓
  │     CadAssembly("belt_conveyor", [...children])
  │
  ▼  mekhanikci-core::backend::openscad::render()
  │     ↓
  │     Walk CadAssembly tree → generate .scad string
  │     Call openscad CLI → produce .stl
  │     Write BOM.csv alongside
  │
  ▼  output/2025-06-21_14-30-00/
       design.json       # Original ConveyorDesign (for replay)
       conveyor.scad     # Generated OpenSCAD
       conveyor.stl      # Rendered STL
       bom.csv           # Bill of Materials
```

---

## Backend Selection

```toml
[backend]
type = "openscad"       # or "cadquery", "freecad" (future)

[openscad]
binary_path = "/usr/bin/openscad"
```

The engine always produces the same CAD Model regardless of backend. Only the final render step changes.

---

## Future: Fine-Tuned Design Models

The User Requirements → Design Spec transformation is currently handled by the general-purpose Qwen 3.5 4B model in a single pass. This is a pragmatic MVP tradeoff.

A future fine-tuned model could:

- **Accept User Requirements as structured input** and output a Design Spec with engineering-appropriate selections (motor sizing, roller diameter, frame profile based on load and speed).
- **Explain engineering decisions** — "selected NEMA23 because required torque is 1.2 Nm at 180W".
- **Flag constraint violations** — "belt width 400mm with 10kg load requires a 60x60 frame, not 40x40".
- **Generate multiple alternatives** — "design A uses NEMA23 with belt drive, design B uses NEMA34 with chain drive".

The architecture supports this upgrade path without changes to the CAD Model or backend layers. The `mekhanikci-llm` crate would gain a dedicated `requirements` module, and the prompt template would be split into two stages.

---

## Determination Guarantee

Identical input → identical output at every layer:

1. **Specification:** Same JSON + same engine version → same CAD Model.
2. **CAD Model:** Same tree → same backend output (OpenSCAD is deterministic).
3. **LLM:** Temperature 0.0, JSON mode, fixed prompt. Not perfectly deterministic (LLMs aren't), but retry + validation handles variance.

---

## Key Design Decisions

| Decision | Rationale |
|----------|-----------|
| Requirements are explicit concept | Separates problem from solution, enables future fine-tuned models |
| Spec is backend-agnostic | Future backends don't require prompt changes |
| CAD Model is explicit | Separates "what to build" from "how to render" |
| Single-pass tree walk | No constraint solver, no graph library needed for MVP |
| LLM → Spec only | LLM never touches geometry or CAD code |
| MVP: single LLM call | Extracts requirements AND applies engineering rules in one step; fine-tuned models can split this later |
