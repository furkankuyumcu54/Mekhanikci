mod visitor;

pub use visitor::{walk_assembly, walk_node, CadVisitor};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CadNode {
    Part(CadPart),
    Assembly(CadAssembly),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CadPart {
    pub name: String,
    pub primitives: Vec<CadPrimitive>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CadPrimitive {
    Box { x: f64, y: f64, z: f64 },
    Cylinder { r: f64, h: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CadAssembly {
    pub name: String,
    pub children: Vec<Child>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Child {
    pub node: Box<CadNode>,
    pub transform: Transform,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Transform {
    pub translation: [f64; 3],
    pub rotation: [f64; 3],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_assembly() {
        let assembly = CadAssembly {
            name: "test".into(),
            children: vec![],
        };
        assert_eq!(assembly.name, "test");
        assert!(assembly.children.is_empty());
    }

    #[test]
    fn test_part_with_primitives() {
        let part = CadPart {
            name: "test_part".into(),
            primitives: vec![
                CadPrimitive::Box {
                    x: 10.0,
                    y: 20.0,
                    z: 30.0,
                },
                CadPrimitive::Cylinder { r: 5.0, h: 15.0 },
            ],
        };
        assert_eq!(part.primitives.len(), 2);
    }

    #[test]
    fn test_nested_assembly() {
        let part = CadPart {
            name: "child_part".into(),
            primitives: vec![],
        };
        let child = Child {
            node: Box::new(CadNode::Part(part)),
            transform: Transform::default(),
        };
        let assembly = CadAssembly {
            name: "parent".into(),
            children: vec![child],
        };
        assert_eq!(assembly.children.len(), 1);
    }

    #[test]
    fn test_transform_default() {
        let t = Transform::default();
        assert_eq!(t.translation, [0.0, 0.0, 0.0]);
        assert_eq!(t.rotation, [0.0, 0.0, 0.0]);
    }
}
