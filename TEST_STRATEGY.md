# Test Strategy — Smoke Tests

Only the minimum tests required to verify the first working prototype.

---

## 1. ConveyorSpec → CadAssembly

| # | Test | Assert |
|---|------|--------|
| 1 | Default spec calls `to_cad_model()` without error | Returns `Ok` |
| 2 | Result is an assembly named `"belt_conveyor"` | `name == "belt_conveyor"` |
| 3 | Assembly has top-level children | `children` not empty |
| 4 | `support_legs: false` produces a different (smaller) child count | Fewer children than with legs |

## 2. CadAssembly → OpenSCAD

| # | Test | Assert |
|---|------|--------|
| 5 | `generate_openscad` output contains `module belt_conveyor()` | Module declaration present |
| 6 | Output contains `belt_conveyor();` | Module invocation present |
| 7 | Output contains `$fn = 32;` | Resolution setting present |
| 8 | A part with a Box primitive produces a `cube(...)` line | Box rendered |
| 9 | A part with a Cylinder primitive produces a `cylinder(...)` line | Cylinder rendered |
| 10 | A non-identity transform emits a `translate(...)` or `rotate(...)` | Transform rendered |
| 11 | An Assembly with children emits `union() { ... }` | Nesting structure correct |

## 3. OpenSCAD → STL

| # | Test | Assert |
|---|------|--------|
| 12 | `render()` writes the `.scad` file to disk | `conveyor.scad` exists, size > 0 |
| 13 | `render()` writes the `.stl` file to disk | `conveyor.stl` exists, size > 0 |
| 14 | STL file begins with `"solid "` or has valid binary header | Format is valid |
| 15 | STL contains at least one facet (`facet normal` or binary equivalent) | Non-empty geometry |
| 16 | `render()` with a non-existent binary path returns an `Err` | Graceful error, no panic |

## 4. `cargo run`

| # | Test | Assert |
|---|------|--------|
| 17 | `cargo run` exits with code 0 | Success exit |
| 18 | `cargo run` prints a line containing `"STL:"` | STL path printed to stdout |
| 19 | The printed STL path points to an existing file with size > 0 | Valid STL output |

---

## Summary

**19 smoke tests** total. No edge cases, no validation, no regressions, no BOM, no visitors, no serialization variants, no golden files. Just enough to confirm the pipeline is connected and produces output.
