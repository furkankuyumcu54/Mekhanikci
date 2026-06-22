# MVP Scope

## Core Architecture

The MVP implements the full 4-layer pipeline for **one machine type only**: belt conveyors.

```
Prompt → LLM (Qwen 3.5 4B) → Design Spec (ConveyorDesign JSON)
         → CAD Model (CadAssembly tree) → OpenSCAD Backend → STL + BOM
```

All four layers are implemented. No shortcuts that skip layers.

---

## What Is In Scope

### Machine Type
- Flat belt conveyor systems.
- NEMA frame motors: 17, 23, 34.
- Standard aluminum extrusion frames (40x40, 30x60).
- Support legs (4-post configuration with cross-braces and foot pads).
- Motor mount positions: underneath, side, end.

### Design Specification (ConveyorDesign)
| Field | Type | Range/Values | Default |
|-------|------|-------------|---------|
| `length_mm` | f64 | 100–10000 | required |
| `belt_width_mm` | f64 | 10–2000 | required |
| `motor.frame` | string | nema17, nema23, nema34 | required |
| `motor.mount` | string | underneath, side, end | required |
| `frame_extrusion` | string | NNxNN pattern | "40x40" |
| `roller_diameter_mm` | f64 | 10–200 | 50.0 |
| `height_mm` | f64 | 100–2000 | 900.0 |
| `support_legs` | bool | — | true |
| `belt_type` | string | flat, cleated, v-belt | "flat" |
| `load_capacity_kg` | Option<f64> | — | null |
| `speed_m_per_s` | Option<f64> | — | null |

### CAD Model Layer
- `CadAssembly` — named node with transformable children
- `CadPart` — named leaf node with primitives
- `CadPrimitive` — `Box {x,y,z}`, `Cylinder {r,h}`
- `Transform` — `translation [x,y,z]`, `rotation [x,y,z]`
- Tree-walking visitor for backend traversal

### OpenSCAD Backend
- Walks CAD Model tree → generates structured `.scad` with `module` definitions
- Calls OpenSCAD CLI → produces `.stl`
- Generates BOM `.csv`

### Technology
| Layer | Technology |
|-------|-----------|
| Language | Rust |
| Terminal UI | Ratatui + Crossterm |
| Local AI | Ollama + Qwen 3.5 4B |
| Serialization | Serde + Serde JSON |
| Configuration | TOML |
| CAD Backend | OpenSCAD CLI |
| Build System | Cargo workspace (3 crates) |

### Platform
- Linux (primary target)
- macOS (secondary)
- Windows (tertiary)

### Error Handling
- No panics on any input path
- LLM JSON validation: 3 retries with error context
- OpenSCAD CLI errors: shown to user
- All errors displayed in TUI, never crash

### Testing
- Unit tests for spec validation, CAD model generation, OpenSCAD code gen
- Integration tests: hand-written spec JSON → CAD Model → `.stl` via OpenSCAD
- LLM integration tests with mock HTTP server

---

## What Is Out Of Scope (MVP)

### Other Machine Types
| Machine | Status |
|---------|--------|
| Linear actuator | Future |
| Gantry system | Future |
| Gearbox | Future |
| Lead screw | Future |

### CAD Backends
| Backend | Status |
|---------|--------|
| CadQuery | Future |
| FreeCAD | Future |

### Export Formats
| Format | Status |
|--------|--------|
| STL (binary) | MVP |
| STEP | Future |
| DXF | Future |
| SVG | Future |
| 3MF | Future |

### Engine Features
| Feature | Status |
|---------|--------|
| Constraint solver | Future |
| Design graph (petgraph) | Future |
| Interference detection | Future |
| FEA / structural simulation | Future |
| Tolerance / fit computation | Future |
| Kinematic analysis | Future |

### LLM Features
| Feature | Status |
|---------|--------|
| Single model (Qwen 3.5 4B) | MVP |
| Multi-model fallback | Future |
| OpenAI API compatibility | Future |
| Incremental design diff | Future |
| Multi-language input | Future |

### TUI Features
| Feature | Status |
|---------|--------|
| Single-chat session | MVP |
| Scrollable conversation | MVP |
| Multi-session tabs | Future |
| 3D preview | Future (external viewer) |
| Design parameter editor | Future |
| Undo/redo | Future |
| Project save/load | Future |

### Persistence
| Feature | Status |
|---------|--------|
| In-memory conversation log | MVP |
| JSON-L session file | MVP |
| Timestamped output directory | MVP |
| Design revision history | Future |
| Project directory format | Future |

---

## Acceptance Criteria

The MVP is complete when:

1. **End-to-end pipeline works.** A user types `"2 meter conveyor, 500mm belt, NEMA23"` in the TUI and receives `.scad`, `.stl`, and `.csv` files.
2. **LLM produces valid spec on >90% of first attempts.** Measured against 50 varied conveyor descriptions.
3. **All layers are exercised.** The spec is validated, a CAD Model tree is built, and the OpenSCAD backend renders it. No layer is bypassed.
4. **Generated OpenSCAD compiles without errors.** Every design passes `openscad -o /dev/null`.
5. **STL files are valid.** Importable into a slicer or CAD tool.
6. **BOM CSV is accurate.** Quantities and descriptions match the design.
7. **TUI is functional.** User can type, see results, exit. All keybindings work.
8. **No panics.** Any input — including deliberately malformed — produces a handled error.

---

## Known Limitations (MVP)

- **Follow-up prompts regenerate from scratch.** No incremental diff against the previous design.
- **Only flat belts.** Cleated and V-belt types are accepted but generate flat belt geometry.
- **No load-based sizing.** Roller diameter, extrusion profile, and motor power are fixed regardless of load capacity.
- **Motor mount is visual only.** Simplified NEMA outline, no shaft/coupler/pulley.
- **Fixed belt thickness.** Always 5mm.
- **Single language.** English only.
- **TUI blocks during OpenSCAD rendering.** Typically 1–5 seconds.

---

## Non-Goals (MVP)

- Replacing traditional CAD software.
- Production-ready engineering drawings.
- FEA or structural simulation.
- Kinematic simulation.
- Multi-user collaboration.
- Cloud or web deployment.
