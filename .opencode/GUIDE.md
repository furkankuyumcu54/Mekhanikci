# GUIDE

## Required Skills

### Rust — Intermediate

| Area | Required For |
|------|-------------|
| Error handling (`Result`, `anyhow`, `thiserror`) | Every module — no unwrap in production |
| Serde derive (`Serialize`, `Deserialize`) | `ConveyorDesign` schema, config |
| Unit and integration testing | All modules |
| Module and workspace organization | 3-crate workspace |
| CLI argument parsing (`clap`) | CLI entry points |
| Pattern matching and algebraic types | Design parameter resolution |
| `std::process::Command` | OpenSCAD subprocess |

### Ratatui / Crossterm — Beginner to Intermediate

| Area | Required For |
|------|-------------|
| Widget trait implementation | Chat view, input area, status bar, BOM table |
| Layout and constraint system | TUI layout |
| Event loop architecture | Main app loop |
| Terminal raw mode management | TUI lifecycle |

### OpenSCAD — Intermediate

| Area | Required For |
|------|-------------|
| CSG operations (`union`, `difference`, `intersection`) | Geometry generation |
| `module`, `translate`, `rotate`, `cube`, `cylinder` | Structured output |
| `$fn` parameter | STL resolution control |
| OpenSCAD CLI flags | Headless rendering |

### LLM Integration — Beginner to Intermediate

| Area | Required For |
|------|-------------|
| Ollama REST API (`/api/generate`, `/api/chat`) | LLM client |
| Prompt engineering for JSON | `ConveyorDesign` extraction |
| JSON mode (`format: "json"`) | Reliable JSON output |
| Few-shot prompting | Schema-compliant generation |
| Temperature and sampling | Output determinism tuning |

### CAD / Mechanical — Foundational

| Area | Required For |
|------|-------------|
| Parametric modelling | Defining conveyor via parameters |
| Coordinate systems | Frame/roller/motor positioning |
| CSG | OpenSCAD generation target |
| NEMA motor frame dimensions | Motor mount geometry |
| BOM structure | BOM CSV generation |

### Development Setup

- **Editor:** VS Code with rust-analyzer
- **Debugging:** `tracing` crate with file output
- **CAD viewing:** OpenSCAD GUI (for inspecting `.scad`)
- **AI testing:** Direct Ollama CLI (`ollama run qwen3.5:4b`)

---

## Branch Strategy

| Branch | Purpose | Protection |
|--------|---------|-----------|
| `main` | Stable, release-ready | Protected |
| `develop` | Integration branch | Protected |
| `feat/<name>` | Feature branches, from `develop` | None |
| `fix/<name>` | Bug fix branches | None |
| `docs/<name>` | Documentation-only changes | None |

Feature branches merge to `develop` via PR. Releases merge `develop` → `main`.

## Commit Conventions

[Conventional Commits](https://www.conventionalcommits.org/) v1.0.0:

```
<type>(<scope>): <description>
```

**Types:** `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`

**Scopes:** `tui` (mekanikci-tui), `llm` (mekanikci-llm), `engine` (mekanikci-core design), `openscad` (mekanikci-core backend), `dsl` (schema), `ci`, `docs`, `project`

**Examples:**
```
feat(engine): add roller geometry computation
fix(llm): handle Ollama timeout with retry backoff
docs(project): initial architecture documentation
```

## Pull Request Workflow

1. Rebase on `develop`: `git fetch origin && git rebase origin/develop`
2. Run checks: `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`
3. PR title follows commit convention. Body describes what, why, how tested.
4. One maintainer approval, all CI pass, no new clippy warnings.
5. **Squash merge** into `develop`. **Merge commit** for releases into `main`.

## Code Standards

- No `unsafe` without `// SAFETY:` comment
- No `unwrap`/`expect` in production. Use `anyhow::Context`
- No `panic` in library code. Use `Result`
- All public items must have doc comments
- `cargo fmt` before every commit
- Clippy must pass with `-D warnings`

## Testing

| Type | Required For |
|------|-------------|
| Unit tests | All public functions in engine, dsl, openscad |
| Integration | Pipeline (JSON → .scad → .stl) |
| Snapshot | OpenSCAD code generation |
| Error-path | Invalid input handling |

New features include tests. Bug fixes include a regression test.

## Issue Tracking

GitHub Issues with labels: `type/*`, `scope/*`, `priority/*`. Every PR links to at least one issue.
