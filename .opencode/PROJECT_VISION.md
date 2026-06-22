# Project Vision

## Long-Term Vision

Mekhanikçi aims to become the standard local-first mechanical design assistant — starting with conveyor systems and expanding to the full range of mechanical primitives that follow parametric patterns.

The ultimate vision: **An engineer states operational requirements — length, weight, throughput — and a manufacturable CAD model emerges with correct engineering decisions already made.**

## MVP Focus: Conveyor Systems

Conveyors are the ideal first primitive because they are:

- **Parametric:** Length, width, motor, rollers — all are independent parameters with known engineering relationships.
- **Self-contained:** A conveyor does not require other primitives to be useful. It is a complete mechanical system.
- **High-demand:** Conveyor design is a daily task for automation engineers. A working conveyor MVP validates the entire pipeline (NL → JSON → engine → CAD) with real utility.
- **Composable foundation:** The frame, roller, belt, and motor-mount sub-components map directly to future primitives (frames → structural profiles, rollers → shafts, motor mounts → bearing blocks).

## Target Users (MVP)

### Primary: Automation Engineers
- Design conveyor systems for material handling
- Work with standardized components (NEMA motors, flat belts, extrusion profiles)
- Need accurate BOMs for procurement
- Iterate frequently between design revisions

### Secondary: Mechanical Engineers
- Need rapid conveyor prototyping
- Value deterministic, repeatable output
- Want to automate repetitive modelling tasks

### Tertiary: Makers and Students
- Design small-scale conveyor systems for projects
- Generate 3D-printable parts without learning CAD

## Engineering Workflow

### Current (Problem)
1. Engineer needs a 2m conveyor with 500mm belt
2. Opens CAD software, models frame, rollers, motor mount
3. Generates BOM manually from parts list
4. Customer requests "shorten by 300mm" → repeat steps 2-3

### Mekhanikçi Workflow (MVP)
1. Engineer types: "2m conveyor, 500mm belt, NEMA23"
2. LLM extracts user requirements and makes engineering decisions
3. Mekhanikçi returns STL + BOM in < 30 seconds
4. Engineer types: "shorten to 1.7m"
5. New STL + BOM generated instantly

## Design Philosophy

### Requirements-Driven
- The user states what they need (length, weight, throughput).
- The system derives the engineering solution (motor size, roller diameter, frame profile).
- For MVP, the LLM handles both extraction and derivation in one step.

### Determinism
- The LLM is a parser, not a designer.
- Every geometry decision is made by Rust code using engineering rules.
- Identical input → identical output, always.

### Local-First
- No telemetry, no cloud dependency, no subscription.
- The application functions fully offline.
- Design data never leaves the machine.

### Engineer-Controlled
- The LLM proposes; the engineer disposes.
- Every parameter is inspectable and overridable in the generated JSON.
- The generated OpenSCAD is readable and manually editable.

### Future: Fine-Tuned Design Models

The general-purpose Qwen 3.5 4B handles requirements extraction and engineering decisions jointly. A future fine-tuned model could:

- Accept structured requirements as input.
- Apply domain-specific engineering rules (load→motor, speed→roller, length→frame).
- Output a validated Design Spec with traceable decisions.
- Explain why each component was selected.
- Flag when requirements exceed standard design limits (e.g., "10m with 500kg requires a structural steel frame, not 40x40 extrusion").

This would make engineering decisions auditable, tunable, and independent of the general-purpose LLM. The rest of the pipeline — CAD Model, backends, TUI — remains unchanged.

## Success Metrics (MVP)

| Metric | Target |
|---|---|---|
| Natural language → STL | < 30 seconds |
| Requirements → valid Design Spec (first attempt) | > 90% |
| Determinism (same input → same output) | 100% |
| Supported conveyor configurations | Flat belt, NEMA motors, multiple lengths/widths |

## What Mekhanikçi Is Not (MVP)

- **Not a general CAD tool.** Only conveyor systems.
- **Not a cloud service.** Runs entirely offline.
- **Not a simulation tool.** No FEA, no kinematics.
