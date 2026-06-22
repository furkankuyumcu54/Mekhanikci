# Mechanical Design Specification

## User Requirements vs Design Specification

The system separates **what the user needs** from **how it is built**.

| Concept | Description | Example Fields |
|---------|-------------|----------------|
| User Requirements | The problem statement — raw operational needs | length, product weight, throughput, belt width |
| Design Specification | The engineering solution — component choices and parameters | motor frame, roller diameter, extrusion profile, height |

### User Requirements (Problem)

Extracted from natural language by the LLM. These describe the task:

```json
{
  "conveyor_length_mm": 10000,
  "product_weight_kg": 1.0,
  "throughput_per_minute": 15,
  "belt_width_mm": 400
}
```

### Design Specification (Solution)

Derived from requirements by applying engineering rules. These describe the build:

```json
{
  "length_mm": 10000,
  "belt_width_mm": 400,
  "motor": { "frame": "nema23", "mount": "underneath" },
  "frame_extrusion": "40x40",
  "roller_diameter_mm": 60,
  "motor_power_w": 180
}
```

For MVP, the LLM produces the Design Specification directly — it extracts requirements and applies engineering rules in a single call. The two concepts are documented separately to prepare for future fine-tuned models that will handle the transformation explicitly.

---

## Purpose

The Mechanical Design Specification is the contract between the LLM and the CAD engine. It captures **design intent** — the engineering solution — without any geometry, coordinates, or backend-specific details.

The LLM's only job is to produce a valid Design Specification. It never generates OpenSCAD, never computes positions, never thinks about CSG operations.

---

## Principles

1. **No geometry.** No coordinates, no transformation matrices, no CSG operations.
2. **No backend details.** No OpenSCAD references, no rendering hints, no format-specific parameters.
3. **Human-readable.** An engineer can read, understand, and edit the spec in any text editor.
4. **Validatable.** Every field has a type, range, and optional default. Invalid specs are caught before any computation.
5. **Self-contained.** A spec file captures everything needed to reproduce a design. No external context required.

---

## Example: ConveyorDesign (MVP)

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

This spec says: "Build a 2m flat belt conveyor, 500mm wide, with a NEMA23 motor mounted underneath, on a 40x40 extrusion frame with 50mm rollers at 900mm height."

It does **not** say:
- Where the rollers are positioned in 3D space
- What the motor mount bracket looks like
- How the frame cross-braces are spaced
- What OpenSCAD modules to generate

Those decisions belong to the CAD Model layer.

---

## Spec Types

Every machine type has its own spec. All specs implement a common trait:

```rust
trait DesignSpec {
    type CadModel: CadAssembly;
    fn to_cad_model(self) -> Result<Self::CadModel>;
}
```

### MVP: ConveyorDesign

```rust
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

struct MotorSpec {
    frame: String,   // "nema17" | "nema23" | "nema34"
    mount: String,   // "underneath" | "side" | "end"
}
```

### Future Examples

```json
// Linear Actuator (future)
{
  "machine_type": "linear_actuator",
  "stroke_mm": 500,
  "load_kg": 100,
  "lead_screw_diameter_mm": 16,
  "lead_mm_per_rev": 5,
  "motor_frame": "nema23"
}

// Gantry System (future)
{
  "machine_type": "gantry",
  "x_travel_mm": 1000,
  "y_travel_mm": 500,
  "z_travel_mm": 200,
  "load_kg": 50,
  "frame_profile": "40x80"
}
```

---

## Validation

Validation is a two-stage process:

### Stage 1: Structural (Serde)
- JSON must deserialize to the correct type.
- Missing required fields → deserialization error.
- Wrong types → deserialization error.

### Stage 2: Semantic (Custom)
- Range checks: `length_mm` must be 100–10000.
- Enum checks: `motor.frame` must be one of the allowed values.
- Pattern checks: `frame_extrusion` must match `NNxNN`.
- Consistency checks: `belt_type` must be in the allowed set.

Validation errors include the field name, the invalid value, and the expected constraint. These are fed back to the LLM on retry.

---

## Evolution Strategy

### Adding Fields

New optional fields can be added to existing specs without breaking backward compatibility. Defaults ensure old specs still produce valid designs.

Example — adding belt tension:

```rust
// Before
struct ConveyorDesign {
    length_mm: f64,
    // ...
}

// After (backward compatible)
struct ConveyorDesign {
    length_mm: f64,
    belt_tension_n: Option<f64>,  // new, defaults to None
    // ...
}
```

### Adding Machine Types

New machine types get their own spec struct and `impl DesignSpec`. The LLM prompt gains a new schema and examples. The engine dispatches based on a `machine_type` discriminator:

```json
{
  "machine_type": "belt_conveyor",
  "length_mm": 2000,
  ...
}
```

Or, for MVP simplicity, the dispatch is implicit (conveyor is the only type).

### Deprecating Fields

Mark fields with `#[deprecated]` in a minor version. Remove in the next major version. The LLM prompt is updated to stop generating the old field.

---

## Future: Requirements-to-Spec Transformation

The User Requirements → Design Spec transformation is currently implicit inside the general-purpose LLM call. A future fine-tuned model could make this explicit:

```rust
trait RequirementsToSpec {
    type Requirements;
    type Spec: DesignSpec;
    fn derive_spec(requirements: Self::Requirements) -> Result<Self::Spec>;
}
```

A fine-tuned model would:

- Accept structured requirements as input (not raw text).
- Apply engineering heuristics (e.g., "roller diameter = belt_width × 0.15, minimum 50mm").
- Output a complete, validated Design Spec.
- Explain its decisions in natural language alongside the spec.

This does not change the spec structure, the CAD Model, or the backends. Only the `mekhanikci-llm` crate changes.

---

## Relationship to CAD Model

The Design Spec → CAD Model transformation is where engineering knowledge lives:

| Spec Value | CAD Model Decision |
|-----------|-------------------|
| `frame_extrusion: "40x40"` | Two Box primitives positioned at frame edges |
| `length_mm: 2000` | Box length = 2000 |
| `roller_diameter_mm: 50` | Cylinder with r=25, positioned at frame ends |
| `motor.frame: "nema23"` | Motor mount plate with NEMA23 bolt pattern |
| `support_legs: true` | Four leg assemblies at corners |

This transformation is pure Rust code — deterministic, testable, auditable.
