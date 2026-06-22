# Mekhanikçi — Required Knowledge (MVP Scope)

---

## Rust

**Level:** Intermediate

| Area | Required For |
|---|---|
| Error handling (`Result`, `anyhow`, `thiserror`) | Every module — no unwrap in production |
| Serde derive (`Serialize`, `Deserialize`) | `ConveyorDesign` schema, config |
| Unit and integration testing | All modules |
| Module and workspace organization | 3-crate workspace |
| CLI argument parsing (`clap`) | CLI entry points |
| Pattern matching and algebraic types | Design parameter resolution |
| `std::process::Command` | OpenSCAD subprocess execution |

---

## Ratatui / Crossterm

**Level:** Beginner to Intermediate

| Area | Required For |
|---|---|
| Widget trait implementation | Chat view, input area, status bar, BOM table |
| Layout and constraint system | TUI layout (chat + input split) |
| Event loop architecture | Main app loop |
| Terminal raw mode management | TUI lifecycle |
| Polling vs blocking input | Crossterm event handling |

---

## OpenSCAD

**Level:** Intermediate

| Area | Required For |
|---|---|
| CSG operations (`union`, `difference`, `intersection`) | Conveyor geometry generation |
| `module` and `function` definitions | Structured OpenSCAD code output |
| `translate`, `rotate`, `cube`, `cylinder`, `hull` | Essential geometry primitives |
| `$fn` parameter | STL resolution control |
| OpenSCAD CLI flags | Headless rendering (`-o`) |
| OpenSCAD output formats | STL export |

---

## Local AI / LLM Integration

**Level:** Beginner to Intermediate

| Area | Required For |
|---|---|
| Ollama REST API (`/api/generate`, `/api/chat`) | LLM client implementation |
| Prompt engineering for structured JSON | `ConveyorDesign` extraction |
| JSON mode (`format: "json"`) | Reliable JSON output |
| Few-shot prompting | Schema-compliant generation |
| Temperature and sampling parameters | Output determinism tuning |

---

## CAD / Mechanical Engineering Concepts

**Level:** Foundational

| Area | Required For |
|---|---|
| Parametric modelling | Defining conveyor via parameters |
| Coordinate systems | Frame/roller/motor positioning |
| CSG (Constructive Solid Geometry) | OpenSCAD generation target |
| NEMA motor frame dimensions | Motor mount geometry |
| Bill of Materials structure | BOM CSV generation |
| Units and dimensional analysis | Parameter resolution |

---

## Recommended Development Setup

- **Editor:** VS Code with rust-analyzer
- **Debugging:** `tracing` crate with file output
- **CAD viewing:** OpenSCAD GUI (for inspecting generated `.scad`)
- **AI testing:** Direct Ollama CLI (`ollama run qwen3.5:4b`)
