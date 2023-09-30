use std::collections::HashMap;

use crate::logic::common::Vec2i;

use super::common::{Node, NodeData, UnnormalizedNode, Vec2};

#[derive(Debug, Clone)]
pub struct Shape {
    pub nodes: Vec<Node>,
    pub bb_top_right: Vec2,
}

pub type RawNode = ((u32, u32), Vec<NodeData>);

impl Shape {
    pub fn new(mut nodes: Vec<Node>) -> Self {
        let bb_top_right = Node::bb_top_right(&nodes);

        Node::sort(&mut nodes);
        assert!(!Node::has_duplicates(&nodes));

        Self { nodes, bb_top_right }
    }

    pub fn rotated_ccw(&self) -> Self {
        fn rotate_with(src: NodeData, dst: &mut NodeData, s: NodeData, r: NodeData) {
            if src.contains(s) {
                dst.insert(r);
            }
        }

        let mut nodes_map: HashMap<Vec2i, NodeData> = HashMap::new();

        for n in &self.nodes {
            let pos = Vec2i::new(-(n.pos.y as i32), n.pos.x as i32);

            if n.data.intersects(NodeData::ALL_VERTICES | NodeData::EDGE_RIGHT) {
                let mut rot_ndata = NodeData::empty();

                rotate_with(n.data, &mut rot_ndata, NodeData::VERTEX_RIGHT, NodeData::VERTEX_UP);
                rotate_with(n.data, &mut rot_ndata, NodeData::VERTEX_UP, NodeData::VERTEX_LEFT);
                rotate_with(n.data, &mut rot_ndata, NodeData::VERTEX_LEFT, NodeData::VERTEX_DOWN);
                rotate_with(n.data, &mut rot_ndata, NodeData::VERTEX_DOWN, NodeData::VERTEX_RIGHT);
                rotate_with(n.data, &mut rot_ndata, NodeData::EDGE_RIGHT, NodeData::EDGE_UP);

                nodes_map.entry(pos).or_insert(NodeData::empty()).insert(rot_ndata);
            }

            if n.data.contains(NodeData::EDGE_UP) {
                nodes_map
                    .entry(pos - Vec2i::new(1, 0))
                    .or_insert(NodeData::empty())
                    .insert(NodeData::EDGE_RIGHT);
            }
        }

        let nodes = nodes_map
            .drain()
            .map(|(pos, data)| UnnormalizedNode { data, pos })
            .collect::<Vec<UnnormalizedNode>>();

        Self::from(nodes)
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        assert_eq!(self.nodes.len(), other.nodes.len());

        return Iterator::zip(self.nodes.iter(), other.nodes.iter()).all(|(a, b)| a == b);
    }
}

impl From<&[RawNode]> for Shape {
    fn from(raw_nodes: &[RawNode]) -> Self {
        let nodes: Vec<Node> = raw_nodes
            .iter()
            .map(|(pos, nds)| Node {
                data: nds.iter().fold(NodeData::empty(), |acc, &nd| acc | nd),
                pos: Vec2::from(*pos),
            })
            .collect();

        Self::new(nodes)
    }
}

impl From<Vec<Node>> for Shape {
    fn from(nodes: Vec<Node>) -> Self {
        Self::new(nodes)
    }
}

impl From<Vec<UnnormalizedNode>> for Shape {
    fn from(unnorm_nodes: Vec<UnnormalizedNode>) -> Self {
        let bottom_left = unnorm_nodes.iter().map(|un| un.pos).fold(Vec2i::MAX, Vec2i::min);

        let nodes = unnorm_nodes
            .into_iter()
            .map(|un| Node {
                data: un.data,
                pos: (un.pos - bottom_left).as_uvec2(),
            })
            .collect::<Vec<Node>>();

        Self::new(nodes)
    }
}

#[cfg(test)]
mod tests {
    use crate::logic::digits::{digit1, digit1_rot_ccw, digits};

    #[test]
    fn rotated_ccw_works_for_digit1() {
        let digit1 = digit1();
        let digit1_rot_ccw = digit1_rot_ccw();

        assert_eq!(digit1.rotated_ccw(), digit1_rot_ccw);
    }

    #[test]
    fn rotated_ccw_works() {
        let digits = digits();

        for d in digits {
            let d_rot = d.rotated_ccw().rotated_ccw().rotated_ccw().rotated_ccw();

            assert_eq!(d.nodes.len(), d_rot.nodes.len());

            Iterator::zip(d.nodes.iter(), d_rot.nodes.iter()).for_each(|(a, b)| {
                assert_eq!(a, b);
            });
        }
    }
}
