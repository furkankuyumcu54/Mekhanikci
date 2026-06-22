use std::path::Path;

use mekanikci_core::backend::{CadBackend, OpenSCADBackend};
use mekanikci_core::design::{ConveyorSpec, DesignSpec, MotorMount, MotorSpec, NemaFrame};

fn main() -> anyhow::Result<()> {
    let spec = ConveyorSpec {
        length_mm: 2000.0,
        belt_width_mm: 500.0,
        motor: MotorSpec {
            frame: NemaFrame::Nema23,
            mount: MotorMount::Underneath,
        },
        frame_extrusion: "40x40".into(),
        roller_diameter_mm: 50.0,
        height_mm: 900.0,
        support_legs: true,
        belt_type: "flat".into(),
        load_capacity_kg: None,
        speed_m_per_s: None,
    };

    println!("ConveyorSpec: {}mm x {}mm, at {}mm height", spec.length_mm, spec.belt_width_mm, spec.height_mm);

    let cad = spec.to_cad_model()?;
    println!("CAD model: {} ({} children)", cad.name, cad.children.len());

    let backend = OpenSCADBackend::new("/usr/bin/openscad");
    let output = backend.render(&cad, Path::new("./output"))?;

    if let Some(scad) = &output.scad_path {
        println!("SCAD: {}", scad.display());
    }
    if let Some(stl) = &output.stl_path {
        println!("STL:  {}", stl.display());
    }
    if let Some(bom) = &output.bom_path {
        println!("BOM:  {}", bom.display());
    }

    Ok(())
}
