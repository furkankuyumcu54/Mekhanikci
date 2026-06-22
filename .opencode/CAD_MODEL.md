# CAD Model

## Purpose

The CAD Model is a backend-independent representation of 3D geometry. It sits between the Design Specification (what to build) and the CAD Backend (how to render it).

A single CAD Model can be rendered by any backend — OpenSCAD, CadQuery, FreeCAD — without changes.

---

## Core Types

```rust
/// A node in the CAD tree. Can be a part (leaf) or assembly (branch).
enum CadNode {
    Part(CadPart),
    Assembly(CadAssembly),
}

/// A leaf node: a named collection of geometric primitives.
struct CadPart {
    name: String,
    primitives: Vec<CadPrimitive>,
}

/// Geometric primitives that define part shape.
/// MVP supports Box and Cylinder only. More added as needed.
enum CadPrimitive {
    /// Rectangular prism centered at origin.
    Box {
        x: f64,   // width (X axis)
        y: f64,   // depth (Y axis)
        z: f64,   // height (Z axis)
    },
    /// Cylinder along Z axis, centered at origin.
    Cylinder {
        r: f64,   // radius
        h: f64,   // height
    },
}

/// A branch node: a named collection of child nodes, each with a transform.
struct CadAssembly {
    name: String,
    children: Vec<Child>,
}

/// A child within an assembly, positioned and oriented by a transform.
struct Child {
    node: CadNode,
    transform: Transform,
}

/// 3D transformation applied to a child node.
struct Transform {
    translation: [f64; 3],  // [x, y, z] in mm
    rotation: [f64; 3],     // [x, y, z] in degrees
}
```

---

## Tree Structure

The CAD Model is a tree:

```
CadAssembly("belt_conveyor")
├── Child @ (0, 0, 0)
│   └── CadAssembly("frame")
│       ├── Child @ (0, -270, 0)
│       │   └── CadPart("left_rail")
│       │       └── Box { 2000, 40, 40 }
│       ├── Child @ (0, 270, 0)
│       │   └── CadPart("right_rail")
│       │       └── Box { 2000, 40, 40 }
│       └── Child @ varies
│           └── CadPart("cross_brace")
│               └── Box { 540, 40, 40 }
├── Child @ (0, 0, 0)
│   └── CadAssembly("rollers")
│       ├── Child @ (-900, 0, 20)
│       │   └── CadPart("drive_roller")
│       │       └── Cylinder { r: 25, h: 520 }
│       └── Child @ (900, 0, 20)
│           └── CadPart("idler_roller")
│               └── Cylinder { r: 25, h: 520 }
├── Child @ (0, 0, 25)
│   └── CadPart("belt_surface")
│       └── Box { 1800, 500, 5 }
├── Child @ (-900, 0, -400)
│   └── CadAssembly("motor_mount")
│       ├── CadPart("mount_plate")
│       │   └── Box { 100, 100, 8 }
│       └── CadPart("motor_body")
│           └── Box { 56, 56, 56 }
└── Child @ varies
    └── CadAssembly("support_leg")
        ├── CadPart("leg_post")
        │   └── Box { 40, 40, 860 }
        ├── CadPart("foot_pad")
        │   └── Box { 50, 50, 5 }
        └── CadPart("cross_brace")
            └── Box { ... rotated }
```

---

## Why This Exists

Without a CAD Model layer, the pipeline would be:

```
Design Spec → OpenSCAD string (direct)
```

This couples the spec to OpenSCAD. Adding a second backend means rewriting the spec-to-string logic for each backend.

With the CAD Model layer:

```
Design Spec → CAD Model → OpenSCAD (via OpenSCADBackend)
                        → CadQuery (via CadQueryBackend, future)
                        → FreeCAD (via FreeCADBackend, future)
```

The spec-to-CAD transformation is written once. Each backend only needs to walk the tree and emit its own format.

---

## Backend Trait

```rust
trait CadBackend {
    /// Render a CAD assembly tree into output files.
    fn render(&self, model: &CadAssembly, output_dir: &Path) -> Result<OutputFiles>;
}

struct OutputFiles {
    scad_path: Option<PathBuf>,
    stl_path: Option<PathBuf>,
    step_path: Option<PathBuf>,
    bom_path: Option<PathBuf>,
}
```

Each backend walks the same tree. The difference is only in output generation:

| Backend | Walk Strategy | Output |
|---------|--------------|--------|
| OpenSCAD | Depth-first, emit `module` + CSG calls | `.scad` → `.stl` |
| CadQuery | Depth-first, emit `cq.Workplane` calls | `.step` |
| FreeCAD | Depth-first, emit `App.ActiveDocument` API | `.fcstd` |

---

## Primitives Expansion Strategy

The primitive set starts minimal and grows only when needed:

| Primitive | MVP | When To Add |
|-----------|-----|-------------|
| `Box` | ✅ | — |
| `Cylinder` | ✅ | — |
| `Sphere` | ❌ | When machine type needs ball joints, casters |
| `Polyhedron` | ❌ | When custom mesh shapes are needed |
| `Extrude` | ❌ | When 2D profiles (e.g. custom extrusion) are needed |
| `Revolution` | ❌ | When lathe-cut shapes are needed |

Adding a primitive requires:
1. Add the variant to `CadPrimitive`
2. Update all backends to handle the new variant
3. Update the spec-to-CAD code to use it

---

## Serialization

The CAD Model is serializable for debugging, testing, and replay:

```json
{
  "name": "belt_conveyor",
  "children": [
    {
      "transform": { "translation": [0, 0, 0], "rotation": [0, 0, 0] },
      "node": {
        "type": "assembly",
        "name": "frame",
        "children": [...]
      }
    }
  ]
}
```

This allows loading a CAD Model from a file without re-running the spec-to-CAD transformation.

---

## Future Compatibility

The CAD Model is designed to support future backends without changes to the core types:

- **CadQuery:** Each primitive maps to a CadQuery operation (`box()`, `circle().extrude()`).
- **FreeCAD:** Each primitive maps to a FreeCAD `Part::Box` or `Part::Cylinder`.
- **STEP export:** Mapped through CadQuery rather than directly from the CAD Model (CadQuery handles STEP natively).

The tree structure also maps naturally to CAD assembly formats:
- OpenSCAD: nested `module` calls with `translate`/`rotate`
- STEP AP242: assembly tree with `NEXT_ASSEMBLY_USAGE_OCCURRENCE`
- Collada / glTF: scene graph with node hierarchy
