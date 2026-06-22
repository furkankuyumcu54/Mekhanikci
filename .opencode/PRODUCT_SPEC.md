# Product Specification — Conveyor System Generation

## 1. Overview

Mekhanikçi accepts natural-language descriptions of conveyor belt systems and produces:
- A `ConveyorDesign` JSON file (the design record)
- An OpenSCAD source file (the CAD model)
- An STL file (the rendered 3D model)
- A CSV BOM file (bill of materials)

---

## 2. User Requirements

User Requirements are the raw problem statement extracted from natural language. They describe what the user needs, not the engineering solution.

```json
{
  "conveyor_length_mm": 10000,
  "product_weight_kg": 1.0,
  "throughput_per_minute": 15,
  "belt_width_mm": 400
}
```

For MVP, requirements are extracted and transformed into a Design Spec in a single LLM call. The requirements are not persisted as a separate file — they are embedded in the design intent captured by `design.json`.

---

## 3. ConveyorDesign JSON Schema

The ConveyorDesign combines **User Requirements** (what the user needs) with **Engineering Decisions** (how the system is built). Fields are annotated below.

### 3.1 Top-Level Structure

```rust
struct ConveyorDesign {
    /// ── User Requirements ──

    /// Conveyor belt length in mm (direction of travel).
    /// Range: 100 – 10000
    pub length_mm: f64,

    /// Belt width in mm (perpendicular to travel).
    /// Range: 10 – 2000
    pub belt_width_mm: f64,

    /// Design load capacity in kg (affects frame/roller sizing).
    /// Optional — if omitted, default sizing is used.
    pub load_capacity_kg: Option<f64>,

    /// Belt speed in m/s.
    /// Optional — informational in MVP (no motor sizing).
    pub speed_m_per_s: Option<f64>,

    /// ── Engineering Decisions ──

    /// Motor specification (derived from load + speed requirements).
    pub motor: MotorSpec,

    /// Frame extrusion profile cross-section in mm.
    /// Selected based on length and load capacity.
    /// Default: "40x40"
    pub frame_extrusion: String,

    /// Roller diameter in mm.
    /// Selected based on belt width and load capacity.
    /// Default: 50.0
    pub roller_diameter_mm: f64,

    /// Height from floor to belt surface in mm.
    /// Default: 900.0
    pub height_mm: f64,

    /// Whether to generate support legs.
    /// Default: true
    pub support_legs: bool,

    /// Belt surface type.
    /// Default: "flat"
    pub belt_type: String,
}
```

### 3.2 MotorSpec

```rust
struct MotorSpec {
    /// NEMA frame size (engineering decision based on load + speed).
    /// Allowed: "nema17", "nema23", "nema34"
    pub frame: String,

    /// Motor mounting position.
    /// Allowed: "underneath", "side", "end"
    pub mount: String,
}
```

### 3.3 Defaults

| Field | Default | Rationale |
|---|---|---|
| `frame_extrusion` | `"40x40"` | Common aluminum extrusion profile |
| `roller_diameter_mm` | `50.0` | Standard small conveyor roller |
| `height_mm` | `900.0` | Ergonomic working height |
| `support_legs` | `true` | Most conveyors need legs |
| `belt_type` | `"flat"` | Simplest and most common |

### 3.4 Validation Rules

Applied after deserialization:

- `length_mm`: 100 – 10000
- `belt_width_mm`: 10 – 2000
- `roller_diameter_mm`: 10 – 200
- `height_mm`: 100 – 2000
- `frame_extrusion`: must match pattern `NNxNN` (e.g. `40x40`, `30x60`)
- `motor.frame`: one of `nema17`, `nema23`, `nema34`
- `motor.mount`: one of `underneath`, `side`, `end`
- `belt_type`: one of `flat`, `cleated`, `v-belt`

### 3.5 JSON Example

```json
{
  "length_mm": 2000,
  "belt_width_mm": 500,
  "motor": {
    "frame": "nema23",
    "mount": "underneath"
  },
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

## 4. LLM Prompt Template

### 4.1 System Prompt

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

User: "Short conveyor, 300mm belt, NEMA17 motor, side mount, 800mm height"
Assistant:
{"length_mm":1000,"belt_width_mm":300,"motor":{"frame":"nema17","mount":"side"},"frame_extrusion":"40x40","roller_diameter_mm":50,"height_mm":800,"support_legs":true,"belt_type":"flat","load_capacity_kg":null,"speed_m_per_s":null}
```

### 4.2 User Prompt

The user's natural language description is appended verbatim as the final message.

### 4.3 Temperature

Fixed at 0.0. JSON mode enabled (`"format": "json"`).

---

## 5. OpenSCAD Code Generation

### 5.1 Module Structure

The generated `.scad` file contains one module per conveyor component:

```
conveyor.scad
├── module conveyor_frame()        — extrusion frame structure
├── module drive_roller()          — driven roller at one end
├── module idler_roller()          — free-spinning roller at other end
├── module belt_surface()          — flat belt surface
├── module motor_mount()           — NEMA motor mount bracket
├── module support_leg()           — single support leg (repeated 4x)
├── module conveyor_assembly()     — top-level union of all parts
└── conveyor_assembly();           — instantiation call
```

### 5.2 Frame Geometry

```
Frame: 2 parallel extrusions of length L, separated by belt_width + gap.
       Cross-braces every 500mm along the length.

  ┌──────────────────────────────────┐
  │  ╔══════════════════════════════╗ │
  │  ║   extrusion profile (40x40)  ║ │
  │  ╚══════════════════════════════╝ │
  │  ╔══════════════════════════════╗ │
  │  ║   extrusion profile (40x40)  ║ │
  │  ╚══════════════════════════════╝ │
  └──────────────────────────────────┘
  │←─────────── length_mm ──────────→│
```

### 5.3 Roller Geometry

```
Roller: cylinder at each end of the frame.
        diameter = roller_diameter_mm
        length   = belt_width_mm + 20

  ┌──────┐              ┌──────┐
  │  ════╪══════════════╪════  │  ← roller
  │      │              │      │
  │  ════╪══════════════╪════  │  ← roller
  └──────┘              └──────┘
```

Roller centre height matches the top face of the frame extrusions + gap for belt clearance.

### 5.4 Belt Surface

```
Belt: flat rectangular surface on top of the rollers.
      thickness = 5mm (fixed for MVP)
      extends from roller centre to roller centre.
```

### 5.5 Motor Mount

Motor mount geometry depends on `motor.mount`:

**Underneath mount:**
```
Plate bolted to the frame underside at the drive roller end.
Bolt pattern matches NEMA standard for the selected frame:
  - NEMA17: 4x M3 on 47.1mm × 47.1mm square
  - NEMA23: 4x M4 on 56.4mm × 56.4mm square
  - NEMA34: 4x M6 on 74.0mm × 74.0mm square
Shaft coupler passes through the plate to connect to the drive roller axle.
```

**Side mount:**
```
L-shaped bracket attached to the frame side face.
Motor shaft aligns with the drive roller axis.
```

**End mount:**
```
Plate mounted to the frame end face.
Motor shaft extends into the roller.
```

### 5.6 Support Legs

```
4 legs at the 4 corners of the frame.
Cross-braces between legs at 45°.
Foot pads at the base (50x50x5 plate).
Height adjusts to achieve the specified height_mm.
```

### 5.7 BOM CSV

Generated alongside the `.scad` file:

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

## 6. TUI Layout

```
┌──────────────────────────────────────────────────────────┐
│  Mekhanikçi v0.1                       [Esc: quit]       │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  You: design a 2m conveyor with 500mm belt and NEMA23    │
│                                                          │
│  ┌────────────────────────────────────────────────────┐  │
│  │ ✓ Conveyor generated                               │  │
│  │   Length: 2000 mm                                  │  │
│  │   Belt width: 500 mm                               │  │
│  │   Motor: NEMA23 (underneath)                       │  │
│  │                                                    │  │
│  │   Files:                                           │  │
│  │   conveyor.scad     ✓                               │  │
│  │   conveyor.stl      ✓                               │  │
│  │   bom.csv           ✓                               │  │
│  └────────────────────────────────────────────────────┘  │
│                                                          │
│  You: shorten to 1.7m                                    │
│  (waiting...)                                             │
│                                                          │
├──────────────────────────────────────────────────────────┤
│  > design a 2m conveyor with 500mm belt and NEMA23     │
│  [Ctrl+Enter to send]                                    │
└──────────────────────────────────────────────────────────┘
```

### Layout Regions

| Region | Widget | % Height |
|---|---|---|
| Top bar | Title + keybindings | 1 line |
| Chat area | Scrollable conversation view | ~80% |
| Input line | Text input with prompt prefix | 3 lines |
| Status bar | Loading indicator, errors, file paths | 1 line |

### Keybindings

| Key | Action |
|---|---|
| `Enter` | Submit input (when input area focused) |
| `Esc` | Exit application |
| `Ctrl+U` | Clear input line |
| `Tab` | Focus toggle (chat / input) |
| `PgUp/PgDn` | Scroll chat history |

---

## 7. Error Handling

### Error Types

| Scenario | User Facing Message | System Action |
|---|---|---|
| Ollama unreachable | "Cannot connect to Ollama. Is it running?" | Retry prompt |
| LLM returns invalid JSON | "Failed to parse design. Retrying..." | Re-prompt + last error |
| Validation failure | "Length must be between 100-10000 mm. Got: 0" | Re-prompt with error |
| OpenSCAD not found | "OpenSCAD binary not found at /usr/bin/openscad" | Show config instructions |
| OpenSCAD render failure | "OpenSCAD rendering failed." | Show stderr |
| All retries exhausted | "Unable to generate design after 3 attempts. Try rephrasing." | Return error to TUI |

### Retry Strategy

- LLM JSON extraction: 3 retries with exponential backoff (1s, 2s, 4s).
- OpenSCAD rendering: no retry (deterministic — failure indicates a bug).

---

## 8. Configuration

```toml
[llm]
model = "qwen3.5:4b"
ollama_url = "http://localhost:11434"
temperature = 0.0
retry_limit = 3

[openscad]
binary_path = "/usr/bin/openscad"
```

Config file path (first found wins):
1. `./mekhanikci.toml` (project-local)
2. `~/.config/mekhanikci/config.toml` (user-global)
3. Defaults as above

---

## 9. File Naming and Output Layout

```
output/
  conveyor_20250621_143000/
    design.json        # ConveyorDesign JSON (the record)
    conveyor.scad      # OpenSCAD source (human-readable, with comments)
    conveyor.stl       # Rendered STL (binary)
    bom.csv            # Bill of Materials
```

The timestamp directory ensures no accidental overwrites. The `design.json` file enables replay without the LLM.
