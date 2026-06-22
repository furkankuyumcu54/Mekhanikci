# VISION

## Long-Term Vision

Mekhanikci aims to become the standard local-first mechanical design assistant — starting with conveyor systems and expanding to the full range of mechanical primitives that follow parametric patterns.

**An engineer states operational requirements — length, weight, throughput — and a manufacturable CAD model emerges with correct engineering decisions already made.**

## Target Users (MVP)

| Tier | Users | Need |
|------|-------|------|
| Primary | Automation engineers | Design conveyor systems, work with standardized components, need accurate BOMs |
| Secondary | Mechanical engineers | Rapid conveyor prototyping, deterministic repeatable output |
| Tertiary | Makers & students | Small-scale conveyors, 3D-printable parts without CAD |

## Design Philosophy

| Principle | Meaning |
|-----------|---------|
| Requirements-Driven | User states what they need; system derives the engineering solution |
| Determinism | LLM is a parser, not a designer. Geometry decisions are Rust code. Same input → same output |
| Local-First | No telemetry, no cloud, no subscription. Fully offline |
| Engineer-Controlled | LLM proposes; engineer disposes. Every parameter inspectable and overridable |

## MVP Scope

### Pipeline

```
Prompt → LLM (Qwen 3.5 4B) → Design Spec (ConveyorDesign JSON)
         → CAD Model (CadAssembly tree) → OpenSCAD Backend → STL + BOM
```

### In Scope

| Category | Details |
|----------|---------|
| Machine type | Flat belt conveyors. NEMA 17/23/34 motors. 40x40/30x60 extrusions. 4-post support legs. Underneath/side/end motor mount |
| Design spec | `ConveyorDesign` — length, belt width, motor, frame_extrusion, roller_diameter, height, support_legs, belt_type, load_capacity, speed |
| CAD Model | `CadAssembly`, `CadPart`, `CadPrimitive` (Box, Cylinder), `Transform` tree |
| Backend | OpenSCAD walker → `.scad` → `.stl` + BOM `.csv` |
| Tech | Rust, Ratatui+Crossterm, Ollama+Qwen 3.5 4B, Serde, TOML |
| Platform | Linux (primary), macOS (secondary), Windows (tertiary) |
| Error handling | No panics. LLM JSON: 3 retries with error context. Errors shown in TUI |
| Testing | Unit tests, integration (spec→STL), mock LLM server tests |

### Out of Scope (MVP)

| Area | Deferred |
|------|----------|
| Machine types | Linear actuator, gantry, gearbox, lead screw |
| Backends | CadQuery, FreeCAD |
| Export formats | STEP, DXF, SVG, 3MF |
| Engine features | Constraint solver, design graph, interference detection, FEA, tolerance analysis, kinematics |
| LLM features | Multi-model fallback, OpenAI API, incremental diff, multi-language |
| TUI features | Multi-session tabs, 3D preview, parameter editor, undo/redo, project save/load |
| Persistence | Design revision history, project directory format |

## Acceptance Criteria

MVP is complete when:

1. End-to-end pipeline works — user types `"2 meter conveyor, 500mm belt, NEMA23"` and receives `.scad`, `.stl`, `.csv`
2. LLM produces valid spec on >90% of first attempts (50 varied descriptions)
3. All layers exercised — no layer bypassed
4. Generated OpenSCAD compiles without errors (`openscad -o /dev/null`)
5. STL files importable into slicer or CAD tool
6. BOM CSV accurate in quantities and descriptions
7. TUI functional — type, see results, exit. Keybindings work
8. No panics on any input, including deliberately malformed

## Known Limitations (MVP)

- Follow-up prompts regenerate from scratch (no incremental diff)
- Only flat belts (cleated/v-belt accepted but generate flat geometry)
- No load-based sizing (roller diameter, extrusion, motor power are fixed)
- Motor mount is visual only (simplified NEMA outline)
- Fixed belt thickness (5mm)
- English only
- TUI blocks during OpenSCAD rendering (1-5s)

## Success Metrics

| Metric | Target |
|--------|--------|
| Natural language → STL | < 30 seconds |
| Valid spec on first attempt | > 90% |
| Determinism | 100% |
| Supported configurations | Flat belt, NEMA motors, multiple lengths/widths |

---

# Roadmap

## Phase 1 — Conveyor MVP

**Goal:** Working application that translates natural-language conveyor descriptions into STL via the full layered architecture.

| Stage | Deliverable | Target |
|-------|-------------|--------|
| 1.1 — Spec & CAD Model | `mekanikci-core` library: `ConveyorDesign` → CAD Model tree | Q3 2025 |
| 1.2 — OpenSCAD Backend | `mekanikci-core` binary: `design.json` → STL + BOM | Q3 2025 |
| 1.3 — LLM Integration | `mekanikci-llm` crate: natural language → `ConveyorDesign` | Q3 2025 |
| 1.4 — Terminal UI | `mekanikci-tui`: interactive terminal app | Q4 2025 |
| 1.5 — Hardening | v0.1.0 release (error audit, edge cases, docs, CI) | Q4 2025 |

## Phase 2 — Spec & CAD Evolution

Second machine type, common primitives library, parametric extrusion profiles, refined CAD Model.

## Phase 3 — Backend Expansion

CadQuery (STEP export), FreeCAD (FCStd), backend selection via config.

## Phase 4 — Multi-Machine Assemblies

Composite designs, assembly constraints, BOM aggregation, design graph.

## Phase 5 — Advanced Features

Project save/load, revision history, interference detection, plugin system, GUI option.

| Milestone | Target |
|-----------|--------|
| Spec → CAD Model pipeline | Q3 2025 |
| OpenSCAD rendering | Q3 2025 |
| LLM integration | Q3 2025 |
| Terminal UI MVP | Q4 2025 |
| v0.1.0 Release | Q4 2025 |
| Second machine type | 2026 |
| CadQuery / FreeCAD backends | 2026+ |
