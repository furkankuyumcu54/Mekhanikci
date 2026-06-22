use serde::{Deserialize, Serialize};

use crate::cad::{CadAssembly, CadNode, CadPart, CadPrimitive, Child, Transform};
use crate::design::motor::{MotorMount, MotorSpec, NemaFrame};
use crate::design::DesignSpec;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConveyorSpec {
    pub length_mm: f64,
    pub belt_width_mm: f64,
    pub roller_diameter_mm: f64,
    pub motor: MotorSpec,
    pub frame_extrusion: String,
    pub height_mm: f64,
    pub support_legs: bool,
    pub belt_type: String,
    pub load_capacity_kg: Option<f64>,
    pub speed_m_per_s: Option<f64>,
}

impl DesignSpec for ConveyorSpec {
    fn to_cad_model(&self) -> anyhow::Result<CadAssembly> {
        let extrusion_w = 20.0;
        let extrusion_h = 20.0;
        let roller_r = self.roller_diameter_mm / 2.0;
        let roller_len = self.belt_width_mm + 20.0;
        let belt_thick = 5.0;
        let roller_end_off = 50.0;
        let half_len = self.length_mm / 2.0;
        let roller_x = half_len - roller_end_off - roller_r;

        let frame_z = extrusion_h / 2.0;
        let roller_z = extrusion_h + roller_r;
        let belt_z = extrusion_h + self.roller_diameter_mm + belt_thick / 2.0;

        let rail_y_off = self.belt_width_mm / 2.0 + extrusion_w;

        let belt_len = self.length_mm - roller_end_off * 2.0 - roller_r * 2.0;

        // ---- Core frame and rollers ----
        let core = vec![
            Child {
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
            },
            Child {
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
            },
            Child {
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
            },
            Child {
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
            },
            Child {
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
            },
        ];

        // ---- Cross-braces ----
        let cross_brace_len = self.belt_width_mm + extrusion_w;
        let num_braces = ((self.length_mm / 500.0).ceil() as usize).max(1);
        let brace_spacing = self.length_mm / (num_braces as f64 + 1.0);
        let cross_braces: Vec<Child> = (0..num_braces)
            .map(|i| {
                let x = -half_len + brace_spacing * (i as f64 + 1.0);
                Child {
                    node: Box::new(CadNode::Part(CadPart {
                        name: format!("cross_brace_{}", i + 1),
                        primitives: vec![CadPrimitive::Box {
                            x: extrusion_w,
                            y: cross_brace_len,
                            z: extrusion_h,
                        }],
                    })),
                    transform: Transform {
                        translation: [x, 0.0, frame_z],
                        rotation: [0.0, 0.0, 0.0],
                    },
                }
            })
            .collect();

        // ---- Support legs ----
        let belt_top_z = extrusion_h + self.roller_diameter_mm + belt_thick;
        let leg_height = (self.height_mm - belt_top_z).max(0.0);
        let leg_parts: Vec<Child> = if self.support_legs {
            let corners = [
                (-half_len + extrusion_w / 2.0, -rail_y_off),
                (-half_len + extrusion_w / 2.0, rail_y_off),
                (half_len - extrusion_w / 2.0, -rail_y_off),
                (half_len - extrusion_w / 2.0, rail_y_off),
            ];
            corners
                .iter()
                .enumerate()
                .flat_map(|(i, &(cx, cy))| {
                    vec![
                        Child {
                            node: Box::new(CadNode::Part(CadPart {
                                name: format!("leg_post_{}", i + 1),
                                primitives: vec![CadPrimitive::Box {
                                    x: extrusion_w,
                                    y: extrusion_w,
                                    z: leg_height,
                                }],
                            })),
                            transform: Transform {
                                translation: [cx, cy, -leg_height / 2.0],
                                rotation: [0.0, 0.0, 0.0],
                            },
                        },
                        Child {
                            node: Box::new(CadNode::Part(CadPart {
                                name: format!("foot_pad_{}", i + 1),
                                primitives: vec![CadPrimitive::Box {
                                    x: 50.0,
                                    y: 50.0,
                                    z: 5.0,
                                }],
                            })),
                            transform: Transform {
                                translation: [cx, cy, -leg_height - 2.5],
                                rotation: [0.0, 0.0, 0.0],
                            },
                        },
                    ]
                })
                .collect()
        } else {
            vec![]
        };

        // ---- Motor mount ----
        let motor_size = nema_face_size(&self.motor.frame);
        let motor_len = nema_body_length(&self.motor.frame);
        let (mount_plate, motor_body) = match self.motor.mount {
            MotorMount::Underneath => (
                Child {
                    node: Box::new(CadNode::Part(CadPart {
                        name: "motor_mount_plate".into(),
                        primitives: vec![CadPrimitive::Box {
                            x: motor_size * 1.5,
                            y: motor_size,
                            z: 5.0,
                        }],
                    })),
                    transform: Transform {
                        translation: [-roller_x, 0.0, -2.5],
                        rotation: [0.0, 0.0, 0.0],
                    },
                },
                Child {
                    node: Box::new(CadNode::Part(CadPart {
                        name: "motor_body".into(),
                        primitives: vec![CadPrimitive::Box {
                            x: motor_size,
                            y: motor_size,
                            z: motor_len,
                        }],
                    })),
                    transform: Transform {
                        translation: [-roller_x, 0.0, -motor_len / 2.0 - 5.0],
                        rotation: [0.0, 0.0, 0.0],
                    },
                },
            ),
            MotorMount::Side => (
                Child {
                    node: Box::new(CadNode::Part(CadPart {
                        name: "motor_mount_plate".into(),
                        primitives: vec![CadPrimitive::Box {
                            x: motor_size,
                            y: 5.0,
                            z: motor_size,
                        }],
                    })),
                    transform: Transform {
                        translation: [
                            -roller_x,
                            -rail_y_off - extrusion_w / 2.0 - 2.5,
                            roller_z,
                        ],
                        rotation: [0.0, 0.0, 0.0],
                    },
                },
                Child {
                    node: Box::new(CadNode::Part(CadPart {
                        name: "motor_body".into(),
                        primitives: vec![CadPrimitive::Box {
                            x: motor_size,
                            y: motor_len,
                            z: motor_size,
                        }],
                    })),
                    transform: Transform {
                        translation: [
                            -roller_x,
                            -rail_y_off - extrusion_w / 2.0 - 5.0 - motor_len / 2.0,
                            roller_z,
                        ],
                        rotation: [0.0, 0.0, 0.0],
                    },
                },
            ),
            MotorMount::End => (
                Child {
                    node: Box::new(CadNode::Part(CadPart {
                        name: "motor_mount_plate".into(),
                        primitives: vec![CadPrimitive::Box {
                            x: 5.0,
                            y: motor_size,
                            z: motor_size,
                        }],
                    })),
                    transform: Transform {
                        translation: [-half_len - 2.5, 0.0, roller_z],
                        rotation: [0.0, 0.0, 0.0],
                    },
                },
                Child {
                    node: Box::new(CadNode::Part(CadPart {
                        name: "motor_body".into(),
                        primitives: vec![CadPrimitive::Box {
                            x: motor_len,
                            y: motor_size,
                            z: motor_size,
                        }],
                    })),
                    transform: Transform {
                        translation: [-half_len - 5.0 - motor_len / 2.0, 0.0, roller_z],
                        rotation: [0.0, 0.0, 0.0],
                    },
                },
            ),
        };

        let children: Vec<Child> = core
            .into_iter()
            .chain(cross_braces)
            .chain(leg_parts)
            .chain([mount_plate, motor_body])
            .collect();

        Ok(CadAssembly {
            name: "belt_conveyor".into(),
            children,
        })
    }
}

fn nema_face_size(frame: &NemaFrame) -> f64 {
    match frame {
        NemaFrame::Nema17 => 42.0,
        NemaFrame::Nema23 => 57.0,
        NemaFrame::Nema34 => 86.0,
    }
}

fn nema_body_length(frame: &NemaFrame) -> f64 {
    match frame {
        NemaFrame::Nema17 => 40.0,
        NemaFrame::Nema23 => 50.0,
        NemaFrame::Nema34 => 70.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cad::CadNode;

    fn default_spec() -> ConveyorSpec {
        ConveyorSpec {
            length_mm: 1000.0,
            belt_width_mm: 300.0,
            roller_diameter_mm: 50.0,
            ..Default::default()
        }
    }

    #[test]
    fn test_conveyor_spec_roundtrip() {
        let json = r#"{
            "length_mm": 1000.0,
            "belt_width_mm": 300.0,
            "roller_diameter_mm": 50.0
        }"#;

        let spec: ConveyorSpec = serde_json::from_str(json).unwrap();
        assert_eq!(spec.length_mm, 1000.0);
        assert_eq!(spec.belt_width_mm, 300.0);
        assert_eq!(spec.roller_diameter_mm, 50.0);
    }

    #[test]
    fn test_to_cad_model_creates_assembly() {
        let model = default_spec().to_cad_model().unwrap();
        assert_eq!(model.name, "belt_conveyor");
        // 5 core + 2 cross-braces + 0 legs + 2 motor = 9
        assert_eq!(model.children.len(), 9);
    }

    #[test]
    fn test_correct_part_names() {
        let model = default_spec().to_cad_model().unwrap();
        let names: Vec<&str> = model
            .children
            .iter()
            .filter_map(|c| match &*c.node {
                CadNode::Part(p) => Some(p.name.as_str()),
                _ => None,
            })
            .collect();
        assert!(names.contains(&"left_rail"));
        assert!(names.contains(&"right_rail"));
        assert!(names.contains(&"drive_roller"));
        assert!(names.contains(&"idler_roller"));
        assert!(names.contains(&"belt"));
    }

    #[test]
    fn test_no_legs_with_defaults() {
        let model = default_spec().to_cad_model().unwrap();
        for child in &model.children {
            if let CadNode::Part(p) = &*child.node {
                assert!(!p.name.starts_with("leg_"), "unexpected leg part");
            }
        }
    }

    #[test]
    fn test_support_legs_generated() {
        let spec = ConveyorSpec {
            length_mm: 1000.0,
            belt_width_mm: 300.0,
            roller_diameter_mm: 50.0,
            support_legs: true,
            height_mm: 900.0,
            ..Default::default()
        };

        let model = spec.to_cad_model().unwrap();
        let names: Vec<&str> = model
            .children
            .iter()
            .filter_map(|c| match &*c.node {
                CadNode::Part(p) => Some(p.name.as_str()),
                _ => None,
            })
            .collect();
        for i in 1..=4 {
            assert!(names.contains(&format!("leg_post_{}", i).as_str()));
            assert!(names.contains(&format!("foot_pad_{}", i).as_str()));
        }
    }

    #[test]
    fn test_cross_braces_generated() {
        let model = default_spec().to_cad_model().unwrap();
        let names: Vec<&str> = model
            .children
            .iter()
            .filter_map(|c| match &*c.node {
                CadNode::Part(p) => Some(p.name.as_str()),
                _ => None,
            })
            .collect();
        assert!(names.contains(&"cross_brace_1"));
        assert!(names.contains(&"cross_brace_2"));
    }

    #[test]
    fn test_motor_mount_generated() {
        let model = default_spec().to_cad_model().unwrap();
        let names: Vec<&str> = model
            .children
            .iter()
            .filter_map(|c| match &*c.node {
                CadNode::Part(p) => Some(p.name.as_str()),
                _ => None,
            })
            .collect();
        assert!(names.contains(&"motor_mount_plate"));
        assert!(names.contains(&"motor_body"));
    }

    #[test]
    fn test_motor_mount_side() {
        let spec = ConveyorSpec {
            length_mm: 1000.0,
            belt_width_mm: 300.0,
            roller_diameter_mm: 50.0,
            motor: MotorSpec {
                frame: NemaFrame::Nema23,
                mount: MotorMount::Side,
            },
            ..Default::default()
        };
        let model = spec.to_cad_model().unwrap();
        let names: Vec<&str> = model
            .children
            .iter()
            .filter_map(|c| match &*c.node {
                CadNode::Part(p) => Some(p.name.as_str()),
                _ => None,
            })
            .collect();
        assert!(names.contains(&"motor_mount_plate"));
        assert!(names.contains(&"motor_body"));
    }

    #[test]
    fn test_motor_mount_end() {
        let spec = ConveyorSpec {
            length_mm: 1000.0,
            belt_width_mm: 300.0,
            roller_diameter_mm: 50.0,
            motor: MotorSpec {
                frame: NemaFrame::Nema34,
                mount: MotorMount::End,
            },
            ..Default::default()
        };
        let model = spec.to_cad_model().unwrap();
        let names: Vec<&str> = model
            .children
            .iter()
            .filter_map(|c| match &*c.node {
                CadNode::Part(p) => Some(p.name.as_str()),
                _ => None,
            })
            .collect();
        assert!(names.contains(&"motor_mount_plate"));
        assert!(names.contains(&"motor_body"));
    }
}
