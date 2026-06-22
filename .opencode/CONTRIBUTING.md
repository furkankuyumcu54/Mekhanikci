# Contributing

## Branch Strategy

| Branch | Purpose | Protection |
|---|---|---|
| `main` | Stable, release-ready code | Protected |
| `develop` | Integration branch | Protected |
| `feat/<name>` | Feature branches, from `develop` | None |
| `fix/<name>` | Bug fix branches | None |
| `docs/<name>` | Documentation-only changes | None |

Feature branches are merged to `develop` via PR. Releases are merged from `develop` to `main`.

---

## Commit Conventions

[Conventional Commits](https://www.conventionalcommits.org/) v1.0.0:

```
<type>(<scope>): <description>
```

### Types

| Type | Usage |
|------|-------|
| `feat` | New feature for a crate or scope |
| `fix` | Bug fix |
| `docs` | Documentation (README, specs, guides) |
| `style` | Formatting, whitespace (no logic change) |
| `refactor` | Code change that is neither fix nor feature |
| `perf` | Performance improvement |
| `test` | Adding or fixing tests |

### Scopes (MVP)

| Scope | Crate / Area |
|---|---|
| `tui` | `mekhanikci-tui` |
| `llm` | `mekhanikci-llm` |
| `engine` | `mekhanikci-core` (design engine) |
| `openscad` | `mekhanikci-core` (OpenSCAD backend) |
| `dsl` | `ConveyorDesign` schema |
| `ci` | CI/CD |
| `docs` | Documentation |
| `project` | Project-wide (no single crate) |

### Examples

```
docs(project): initial architecture documentation
feat(engine): add roller geometry computation
fix(llm): handle Ollama timeout with retry backoff
test(engine): add conveyor edge case tests
```

---

## Pull Request Workflow

1. Rebase on `develop` before opening:
   ```
   git fetch origin && git rebase origin/develop
   ```
2. Run checks:
   ```
   cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test
   ```
3. PR title follows commit convention format.
4. PR body describes what, why, and how it was tested.

### Requirements

- One approval from a maintainer.
- All CI checks pass.
- No new clippy warnings or fmt violations.

### Merge

- **Squash merge** into `develop`.
- **Merge commit** for releases into `main`.

---

## Code Standards

- **No unsafe** without `// SAFETY:` comment.
- **No unwrap/expect** in production. Use `anyhow::Context` or proper error handling.
- **No panic** in library code. Use `Result`.
- **All public items must have doc comments.**
- **`cargo fmt`** before every commit.
- **Clippy** must pass with `-D warnings`.

---

## Testing Expectations

| Test Type | Required For |
|---|---|
| Unit tests | All public functions in `engine`, `dsl`, `openscad` |
| Integration tests | Pipeline (JSON → .scad → .stl) |
| Snapshot tests | OpenSCAD code generation |
| Error-path tests | Invalid input handling |

- New features include tests.
- Bug fixes include a regression test.

---

## Issue Tracking

- GitHub Issues with labels: `type/*`, `scope/*`, `priority/*`.
- Bug reports: expected, actual, reproduction steps, environment.
- Every PR links to at least one issue.
