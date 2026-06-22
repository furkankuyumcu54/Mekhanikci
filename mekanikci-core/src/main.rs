use std::path::Path;

use mekanikci_core::backend::{CadBackend, OpenSCADBackend};
use mekanikci_core::design::{ConveyorSpec, DesignSpec};

fn main() -> anyhow::Result<()> {
    let spec = ConveyorSpec {
        length_mm: 1000.0,
        belt_width_mm: 300.0,
        roller_diameter_mm: 50.0,
        support_legs: true,
        height_mm: 900.0,
        ..Default::default()
    };

    println!(
        "ConveyorSpec: {}mm x {}mm",
        spec.length_mm, spec.belt_width_mm
    );

    let cad = spec.to_cad_model()?;
    println!("CAD model: {} ({} children)", cad.name, cad.children.len());

    let backend = OpenSCADBackend;
    let output = backend.render(&cad, Path::new("./output"))?;

    if let Some(scad) = &output.scad_path {
        println!("SCAD: {}", scad.display());
    }

    if let Some(stl) = &output.stl_path {
        println!("STL:  {}", stl.display());
    }

    Ok(())
}
