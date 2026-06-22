# Architectural Decisions

## 1. Conveyor-First MVP

### Decision
The first release supports only conveyor belt systems. All other primitives (bearings, frames, gears, fasteners) are deferred.

### Reasoning
- **Validates the full pipeline** with one complete, useful system before adding complexity.
- **Conveyors are self-contained:** a single design produces a complete CAD model without depending on other primitives.
- **High user value:** conveyor design is a daily task for automation engineers.
- **Limits LLM complexity:** a single JSON schema with ~10 fields is far easier to extract reliably than a general-purpose mechanical DSL.
- **Limits test surface:** one geometry generator, one prompt template, one set of integration tests.

### Tradeoffs
- Users who need other systems (linear guides, gearboxes) cannot use the MVP. Acceptable — the pipeline architecture makes adding primitives mechanical once the conveyor path is proven.
- The `ConveyorDesign` schema is specific to conveyors and will not directly generalize. However, the pipeline structure (LLM → validated JSON → engine → OpenSCAD) is fully reusable.

---

## 2. Local-First Design

### Decision
The application runs entirely on the user's machine with no cloud dependency. All AI inference is local (Ollama), all file storage is local, no telemetry.

### Reasoning
- **Determinism control:** Cloud APIs change models and behavior without notice. A pinned local model guarantees consistent output.
- **Data privacy:** Mechanical designs are often proprietary. No data leaves the machine.
- **Offline operation:** Factory floors, clean rooms, and classified facilities may have no internet.
- **No subscription costs:** After the model download, there are no API fees.
- **Latency:** Local inference with a 4B model is typically 1-5 seconds — competitive with cloud round trips.

### Tradeoffs
- Requires a machine that can run a 4B model (8 GB+ RAM recommended).
- User must download the model (~2.5 GB) separately via `ollama pull`.

---

## 3. Rust as the Implementation Language

### Decision
The entire application is written in Rust.

### Reasoning
- **Safety and determinism:** Rust's ownership model eliminates memory bugs at compile time. For geometry generation where a bug produces silently wrong output, this is critical.
- **Performance:** Coordinate transformations, CSG tree generation, and string templating benefit from zero-cost abstractions.
- **Ecosystem:** Serde, Ratatui, Clap, Tokio, `reqwest` — mature libraries for every layer.
- **Subprocess management:** `std::process::Command` integrates cleanly with OpenSCAD CLI.
- **Cross-platform:** Linux, macOS, Windows as first-class targets.

### Tradeoffs
- Steeper learning curve and slower compile times than Python or JavaScript. Acceptable given the long-term nature of the project.

---

## 4. OpenSCAD as the CAD Backend

### Decision
CAD output is generated as OpenSCAD source code and rendered via the OpenSCAD CLI.

### Reasoning
- **Text-based:** Generating `.scad` from Rust is string templating with no FFI.
- **Deterministic:** Same `.scad` always produces same `.stl`. Core principle.
- **CSG-native:** Maps directly to how the engine describes geometry.
- **CLI-first:** `openscad -o file.stl file.scad` is a single subprocess call.
- **Free and open source:** No licensing restrictions.
- **STL export:** Sufficient for 3D printing and most engineering workflows.

### Tradeoffs
- No NURBS or freeform surfaces. Irrelevant for conveyor geometry (extrusions, cylinders, cubes).
- Slower than commercial kernels for large models. Conveyors are small enough (< 100 primitives) to render in seconds.
- No in-process API. Subprocess file I/O is acceptable for MVP.

---

## 5. LLM Only Produces Structured JSON

### Decision
The LLM is strictly constrained to output `ConveyorDesign` JSON. It never generates OpenSCAD.

### Reasoning
- **Determinism guarantee:** LLM-generated OpenSCAD would be non-deterministic and potentially invalid.
- **Validation chain:** JSON is validated by Serde deserialization + range checks. Invalid output is caught before any geometry work begins; re-prompt with error context.
- **Testability:** The engine is tested with hand-written JSON fixtures. No LLM needed.
- **Auditability:** The JSON is a human-readable design record.

### Tradeoffs
- Prompt engineering is required to make Qwen3.5 4B reliably output valid schema-conforming JSON. Mitigated by Ollama's JSON mode and few-shot examples.

---

## 6. Ratatui / Terminal UI

### Decision
The user interface is a terminal application built with Ratatui and Crossterm.

### Reasoning
- **Local-first:** No browser, no Electron, no web stack.
- **Engineer affinity:** Mechanical engineers are comfortable in the terminal.
- **SSH-friendly:** Can run on a workstation from a laptop.
- **Rapid development:** No CSS, no DOM — Ratatui widgets map directly to app data structures (chat log, BOM table).

### Tradeoffs
- No 3D viewport. Preview is limited to parameter summaries and generated file paths. Users open `.stl` in their own viewer.
- Smaller potential user base than a web app. Acceptable for the target audience.

---

## 7. No Graph Library — Direct Computation

### Decision
The MVP engine does not use petgraph or any graph library. Conveyor geometry is computed directly from the parameter set.

### Reasoning
- **Conveyors are not graphs:** A conveyor has a fixed structure (frame, 2 rollers, belt, legs, motor mount). No graph traversal or topological sort is needed.
- **Simpler is faster:** Direct computation is easier to understand, test, and debug than a generic graph solution.
- **Keep it for later:** If/when multi-primitive assemblies are added, petgraph can be introduced at that point.

### Tradeoffs
- Adding a second primitive type will require restructuring the engine. This is intentional — the restructuring will happen when we understand the real requirements.

---

## 8. OpenSCAD Code Generation via String Templating

### Decision
OpenSCAD source is generated using Rust `format!` and helper functions — no code generation framework.

### Reasoning
- **Simple target:** OpenSCAD syntax is straightforward. A conveyor model requires ~10 `module` definitions and ~50 lines of code.
- **Human-readable output:** String templates preserve control over formatting and comments in the `.scad` file.
- **Fast to implement:** No codegen framework dependency or learning curve.

### Tradeoffs
- No syntax validation at generation time. Mitigated by integration tests that compile every generated `.scad` file with OpenSCAD.

---

## 9. No Database — Filesystem Persistence

### Decision
Conversation history is stored as JSON-L files. No SQLite, no embedded database.

### Reasoning
- **Simplicity:** One file per session, append-only log format.
- **Transparency:** Users can read and edit history with any text editor.
- **No schema migrations:** Adding a log field is a code change.
- **No dependency:** One fewer crate to manage.

### Tradeoffs
- No query capability. Not needed for MVP — sessions are single-shot or linear conversations.
