use serde::{Deserialize, Serialize};

use crate::cad::{CadAssembly, CadNode, CadPart, CadPrimitive, Child, Transform};
use crate::design::{DesignSpec, MotorSpec};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConveyorSpec {
    pub length_mm: f64,
    pub belt_width_mm: f64,
    pub motor: MotorSpec,
    pub frame_extrusion: String,
    pub roller_diameter_mm: f64,
    pub height_mm: f64,
    pub support_legs: bool,
    pub belt_type: String,
    pub load_capacity_kg: Option<f64>,
    pub speed_m_per_s: Option<f64>,
}

impl DesignSpec for ConveyorSpec {
    fn to_cad_model(&self) -> anyhow::Result<CadAssembly> {
        let (extrusion_w, extrusion_h) = parse_extrusion(&self.frame_extrusion);
        let roller_r = self.roller_diameter_mm / 2.0;
        let roller_len = self.belt_width_mm + 20.0;
        let belt_thick = 5.0;
        let roller_end_off = 50.0;
        let half_len = self.length_mm / 2.0;
        let roller_x = half_len - roller_end_off - roller_r;

        let leg_h = self.height_mm - extrusion_h - self.roller_diameter_mm - belt_thick;
        let leg_z = leg_h / 2.0;
        let frame_z = leg_h + extrusion_h / 2.0;
        let roller_z = leg_h + extrusion_h + roller_r;
        let belt_z = leg_h + extrusion_h + self.roller_diameter_mm + belt_thick / 2.0;

        let rail_y_off = self.belt_width_mm / 2.0 + extrusion_w;

        let mut children: Vec<Child> = Vec::new();

        // Frame
        let mut frame_children = Vec::new();
        frame_children.push(Child {
            node: Box::new(CadNode::Part(CadPart {
                name: "left_rail".into(),
                primitives: vec![CadPrimitive::Box {
                    x: self.length_mm,
                    y: extrusion_w,
                    z: extrusion_h,
                }],
            })),
            transform: Transform {
                translation: [0.0, -rail_y_off, frame_z],
                rotation: [0.0, 0.0, 0.0],
            },
        });
        frame_children.push(Child {
            node: Box::new(CadNode::Part(CadPart {
                name: "right_rail".into(),
                primitives: vec![CadPrimitive::Box {
                    x: self.length_mm,
                    y: extrusion_w,
                    z: extrusion_h,
                }],
            })),
            transform: Transform {
                translation: [0.0, rail_y_off, frame_z],
                rotation: [0.0, 0.0, 0.0],
            },
        });

        let brace_span = (rail_y_off * 2.0 - extrusion_w).max(0.0);
        let num_braces = (self.length_mm / 500.0).ceil() as usize;
        let brace_spacing = self.length_mm / (num_braces as f64 + 1.0);
        for i in 0..num_braces {
            let x = -half_len + brace_spacing * (i as f64 + 1.0);
            frame_children.push(Child {
                node: Box::new(CadNode::Part(CadPart {
                    name: format!("cross_brace_{}", i + 1),
                    primitives: vec![CadPrimitive::Box {
                        x: extrusion_w,
                        y: brace_span,
                        z: extrusion_h,
                    }],
                })),
                transform: Transform {
                    translation: [x, 0.0, frame_z],
                    rotation: [0.0, 0.0, 0.0],
                },
            });
        }

        children.push(Child {
            node: Box::new(CadNode::Assembly(CadAssembly {
                name: "frame".into(),
                children: frame_children,
            })),
            transform: Transform::default(),
        });

        // Rollers
        let mut roller_children = Vec::new();
        roller_children.push(Child {
            node: Box::new(CadNode::Part(CadPart {
                name: "drive_roller".into(),
                primitives: vec![CadPrimitive::Cylinder {
                    r: roller_r,
                    h: roller_len,
                }],
            })),
            transform: Transform {
                translation: [-roller_x, 0.0, roller_z],
                rotation: [90.0, 0.0, 0.0],
            },
        });
        roller_children.push(Child {
            node: Box::new(CadNode::Part(CadPart {
                name: "idler_roller".into(),
                primitives: vec![CadPrimitive::Cylinder {
                    r: roller_r,
                    h: roller_len,
                }],
            })),
            transform: Transform {
                translation: [roller_x, 0.0, roller_z],
                rotation: [90.0, 0.0, 0.0],
            },
        });

        children.push(Child {
            node: Box::new(CadNode::Assembly(CadAssembly {
                name: "rollers".into(),
                children: roller_children,
            })),
            transform: Transform::default(),
        });

        // Belt
        let belt_len = self.length_mm - roller_end_off * 2.0 - roller_r * 2.0;
        children.push(Child {
            node: Box::new(CadNode::Part(CadPart {
                name: "belt".into(),
                primitives: vec![CadPrimitive::Box {
                    x: belt_len.max(0.0),
                    y: self.belt_width_mm,
                    z: belt_thick,
                }],
            })),
            transform: Transform {
                translation: [0.0, 0.0, belt_z],
                rotation: [0.0, 0.0, 0.0],
            },
        });

        // Motor mount (simplified plate underneath drive roller end)
        let motor_z = leg_h - 4.0;
        children.push(Child {
            node: Box::new(CadNode::Part(CadPart {
                name: "motor_mount".into(),
                primitives: vec![CadPrimitive::Box {
                    x: 120.0,
                    y: 100.0,
                    z: 8.0,
                }],
            })),
            transform: Transform {
                translation: [-roller_x, 0.0, motor_z],
                rotation: [0.0, 0.0, 0.0],
            },
        });

        // Legs
        if self.support_legs {
            let leg_w = 40.0;
            let leg_x = half_len - 100.0;
            let leg_y = rail_y_off + extrusion_w / 2.0;
            for (lx, ly) in &[(-leg_x, -leg_y), (-leg_x, leg_y), (leg_x, -leg_y), (leg_x, leg_y)] {
                children.push(Child {
                    node: Box::new(CadNode::Part(CadPart {
                        name: format!("leg_{}_{}", if *lx < 0.0 { "rear" } else { "front" }, if *ly < 0.0 { "left" } else { "right" }),
                        primitives: vec![CadPrimitive::Box {
                            x: leg_w,
                            y: leg_w,
                            z: leg_h.max(0.0),
                        }],
                    })),
                    transform: Transform {
                        translation: [*lx, *ly, leg_z],
                        rotation: [0.0, 0.0, 0.0],
                    },
                });
            }
        }

        Ok(CadAssembly {
            name: "belt_conveyor".into(),
            children,
        })
    }
}

fn parse_extrusion(s: &str) -> (f64, f64) {
    let parts: Vec<&str> = s.split('x').collect();
    if parts.len() == 2 {
        let w: f64 = parts[0].parse().unwrap_or(40.0);
        let h: f64 = parts[1].parse().unwrap_or(40.0);
        (w, h)
    } else {
        (40.0, 40.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cad::CadNode;
    use crate::design::{MotorMount, NemaFrame};

    #[test]
    fn test_conveyor_spec_roundtrip() {
        let json = r#"{
            "length_mm": 2000.0,
            "belt_width_mm": 500.0,
            "motor": { "frame": "nema23", "mount": "underneath" },
            "frame_extrusion": "40x40",
            "roller_diameter_mm": 50.0,
            "height_mm": 900.0,
            "support_legs": true,
            "belt_type": "flat",
            "load_capacity_kg": 50.0,
            "speed_m_per_s": 0.5
        }"#;

        let spec: ConveyorSpec = serde_json::from_str(json).unwrap();
        assert_eq!(spec.length_mm, 2000.0);
        assert_eq!(spec.motor.frame, NemaFrame::Nema23);
        assert_eq!(spec.belt_width_mm, 500.0);
        assert_eq!(spec.belt_type, "flat");
        assert_eq!(spec.load_capacity_kg, Some(50.0));
        assert_eq!(spec.speed_m_per_s, Some(0.5));
    }

    #[test]
    fn test_to_cad_model_creates_assembly() {
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

        let model = spec.to_cad_model().unwrap();
        assert_eq!(model.name, "belt_conveyor");
        assert!(!model.children.is_empty());
    }

    #[test]
    fn test_to_cad_model_no_legs() {
        let spec = ConveyorSpec {
            length_mm: 1000.0,
            belt_width_mm: 300.0,
            motor: MotorSpec {
                frame: NemaFrame::Nema17,
                mount: MotorMount::Side,
            },
            frame_extrusion: "40x40".into(),
            roller_diameter_mm: 40.0,
            height_mm: 800.0,
            support_legs: false,
            belt_type: "flat".into(),
            load_capacity_kg: None,
            speed_m_per_s: None,
        };

        let model = spec.to_cad_model().unwrap();
        let num_legs: usize = model
            .children
            .iter()
            .filter(|c| matches!(&*c.node, CadNode::Part(p) if p.name.starts_with("leg_")))
            .count();
        assert_eq!(num_legs, 0);
    }

    #[test]
    fn test_parse_extrusion_standard() {
        assert_eq!(parse_extrusion("40x40"), (40.0, 40.0));
        assert_eq!(parse_extrusion("30x60"), (30.0, 60.0));
    }

    #[test]
    fn test_parse_extrusion_fallback() {
        assert_eq!(parse_extrusion("invalid"), (40.0, 40.0));
    }
}
