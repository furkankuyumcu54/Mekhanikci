# Roadmap

## Phase 1 — Conveyor MVP

**Goal:** Ship a working application that translates natural-language conveyor descriptions into STL files via the full layered architecture (Design Spec → CAD Model → OpenSCAD Backend).

### Stage 1.1 — Design Spec & CAD Model
- [ ] Define `DesignSpec` trait
- [ ] Define `ConveyorDesign` struct with Serde + validation
- [ ] Define CAD Model types: `CadAssembly`, `CadPart`, `CadPrimitive`, `Transform`
- [ ] Implement `ConveyorDesign::to_cad_model()` (frame, rollers, belt, legs, motor mount)
- [ ] Write unit tests: spec → CAD Model correctness

**Deliverable:** `mekhanikci-core` library that converts `ConveyorDesign` JSON to a CAD Model tree.

### Stage 1.2 — OpenSCAD Backend
- [ ] Define `CadBackend` trait
- [ ] Implement `OpenSCADBackend` (walk CAD tree → `.scad` string)
- [ ] Implement STL export via OpenSCAD CLI subprocess
- [ ] Implement BOM CSV export
- [ ] Write integration tests: spec → CAD Model → `.stl`

**Deliverable:** `mekhanikci-core` binary that reads `design.json` and produces STL + BOM.

### Stage 1.3 — LLM Integration
- [ ] Implement Ollama HTTP client (`mekhanikci-llm`)
- [ ] Engineer system prompt with `ConveyorDesign` schema + few-shot examples
- [ ] Implement JSON extraction and validation loop (3 retries)
- [ ] Write LLM integration tests (mock Ollama server)

**Deliverable:** `mekhanikci-llm` crate that accepts natural language and outputs `ConveyorDesign`.

### Stage 1.4 — Terminal UI
- [ ] Build main event loop (Ratatui + Crossterm)
- [ ] Build chat view + input area + status bar
- [ ] Wire TUI → LLM → Core pipeline end-to-end
- [ ] Display generated file paths and BOM
- [ ] Session history (in-memory + JSON-L file)

**Deliverable:** `mekhanikci-tui` — interactive terminal application.

### Stage 1.5 — Hardening
- [ ] Error handling audit (no panics on any input path)
- [ ] Edge case tests (zero-length, negative, missing fields, missing OpenSCAD)
- [ ] User documentation (README with example prompts)
- [ ] CI pipeline (fmt, clippy, test, integration)
- [ ] Packaging (cargo install)

**Deliverable:** v0.1.0 release.

---

## Phase 2 — Spec & CAD Evolution

- [ ] Second machine type (e.g. linear actuator, gantry)
- [ ] Common primitives library (bearing blocks, extrusion profiles, fasteners)
- [ ] Parametric extrusion profile system
- [ ] Refined CAD Model with more primitive types

---

## Phase 3 — Backend Expansion

- [ ] `CadQueryBackend` — STEP export
- [ ] `FreeCADBackend` — FCStd + STEP export
- [ ] Backend selection via config
- [ ] Output format selection (STL / STEP / DXF)

---

## Phase 4 — Multi-Machine Assemblies

- [ ] Composite designs (conveyor feeds into a linear actuator)
- [ ] Assembly-level constraints (concentric, coplanar)
- [ ] BOM aggregation across sub-assemblies
- [ ] Design graph for component relationships

---

## Phase 5 — Advanced Features

- [ ] Project save/load (full design state)
- [ ] Design revision history
- [ ] Interference detection
- [ ] Plugin system for community machine types
- [ ] GUI (non-terminal) option

---

## Milestone Summary

| Milestone | Target |
|-----------|--------|
| Spec → CAD Model pipeline | Q3 2025 |
| OpenSCAD rendering | Q3 2025 |
| LLM integration | Q3 2025 |
| Terminal UI MVP | Q4 2025 |
| v0.1.0 Release | Q4 2025 |
| Second machine type | 2026 |
| CadQuery / FreeCAD backends | 2026+ |
