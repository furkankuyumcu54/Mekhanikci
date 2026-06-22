# Mekhanikçi

**Local-first AI-powered conveyor design assistant.**

Describe a conveyor system in natural language. Mekhanikçi generates a structured JSON design, runs it through a deterministic Rust engine, and produces a 3D-printable OpenSCAD model — all on your machine, no cloud required.

```
"Design a 2 meter conveyor with 500 mm belt width and NEMA23 motor"

  →  Structured JSON  →  Rust Engine  →  OpenSCAD  →  STL + BOM
```

---

## Motivation

Conveyor design follows well-known parametric patterns: frame length, belt width, motor frame, roller diameter, support legs. Despite this, every design iteration requires manual CAD modelling. Existing AI CAD tools are cloud-dependent, non-deterministic, or generate geometry directly from the LLM — producing unreliable output.

Mekhanikçi constrains the LLM to structured JSON only. CAD generation is deterministic Rust code. Same input always produces the same output.

---

## MVP Scope

The first release supports **one thing only**: conveyor system design from natural language.

| Included | Not Included (future) |
|---|---|
| Conveyor belt systems | Bearings, frames, actuators |
| OpenSCAD backend | CadQuery, FreeCAD backends |
| Ollama + Qwen3.5 4B | Any other LLM provider |
| Ratatui terminal UI | GUI, web UI |
| STL export | DXF, STEP export |
| BOM generation | Full assembly drawings |

---

## Technology Stack

| Layer | Technology |
|---|---|
| Language | Rust |
| Terminal UI | Ratatui + Crossterm |
| Local AI | Ollama + Qwen3.5 4B |
| Serialization | Serde + Serde JSON |
| Configuration | TOML |
| CAD Backend | OpenSCAD CLI |
| Build System | Cargo |

---

## Project Status

Pre-alpha. Architecture and planning phase.

---

## License

MIT — see [LICENSE](LICENSE).
