use crate::cad::{CadAssembly, CadNode, CadPart, CadPrimitive};

pub trait CadVisitor {
    fn visit_assembly(&mut self, assembly: &CadAssembly);
    fn visit_part(&mut self, part: &CadPart);
    fn visit_primitive(&mut self, primitive: &CadPrimitive);
}

pub fn walk_assembly(assembly: &CadAssembly, visitor: &mut impl CadVisitor) {
    visitor.visit_assembly(assembly);
    for child in &assembly.children {
        walk_node(&child.node, visitor);
    }
}

pub fn walk_node(node: &CadNode, visitor: &mut impl CadVisitor) {
    match node {
        CadNode::Part(part) => {
            visitor.visit_part(part);
            for primitive in &part.primitives {
                visitor.visit_primitive(primitive);
            }
        }
        CadNode::Assembly(assembly) => {
            walk_assembly(assembly, visitor);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cad::{Child, Transform};

    struct CollectNames {
        names: Vec<String>,
    }

    impl CadVisitor for CollectNames {
        fn visit_assembly(&mut self, assembly: &CadAssembly) {
            self.names.push(format!("assembly:{}", assembly.name));
        }

        fn visit_part(&mut self, part: &CadPart) {
            self.names.push(format!("part:{}", part.name));
        }

        fn visit_primitive(&mut self, primitive: &CadPrimitive) {
            match primitive {
                CadPrimitive::Box { .. } => self.names.push("primitive:box".into()),
                CadPrimitive::Cylinder { .. } => self.names.push("primitive:cylinder".into()),
            }
        }
    }

    fn make_test_assembly() -> CadAssembly {
        let child_part = CadNode::Part(CadPart {
            name: "child".into(),
            primitives: vec![CadPrimitive::Box {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }],
        });

        let child = Child {
            node: Box::new(child_part),
            transform: Transform::default(),
        };

        CadAssembly {
            name: "root".into(),
            children: vec![child],
        }
    }

    #[test]
    fn test_walker_visits_all_nodes() {
        let assembly = make_test_assembly();
        let mut collector = CollectNames { names: vec![] };
        walk_assembly(&assembly, &mut collector);
        assert!(collector.names.contains(&"assembly:root".into()));
        assert!(collector.names.contains(&"part:child".into()));
        assert!(collector.names.contains(&"primitive:box".into()));
    }
}
