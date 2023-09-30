use std::collections::HashMap;

use crate::logic::common::Vec2i;

use super::common::{Node, NodeData, UnnormalizedNode, Vec2};

#[derive(Debug, Clone)]
pub struct Shape {
    nodes: Box<[Node]>,
    bb_top_right: Vec2,
}

pub type RawNode = ((u32, u32), Vec<NodeData>);

impl Shape {
    pub fn new(mut nodes: Vec<Node>) -> Self {
        assert!(!Node::has_empty_node(&nodes));
        Node::align_to_origin(&mut nodes);
        Node::sort(&mut nodes);
        assert!(!Node::has_duplicates(&nodes));

        let bb_top_right = Node::bb_top_right(&nodes);

        Self {
            nodes: nodes.into_boxed_slice(),
            bb_top_right,
        }
    }

    pub fn rotated_ccw(&self) -> Self {
        self.flat_map(|n, out| {
            let pos = Vec2i::new(-(n.pos.y as i32), n.pos.x as i32);

            if n.data.intersects(NodeData::ALL_VERTICES | NodeData::EDGE_RIGHT) {
                let mut rot_ndata = NodeData::empty();

                Self::replace_into(n.data, NodeData::VERTEX_RIGHT, &mut rot_ndata, NodeData::VERTEX_UP);
                Self::replace_into(n.data, NodeData::VERTEX_UP, &mut rot_ndata, NodeData::VERTEX_LEFT);
                Self::replace_into(n.data, NodeData::VERTEX_LEFT, &mut rot_ndata, NodeData::VERTEX_DOWN);
                Self::replace_into(n.data, NodeData::VERTEX_DOWN, &mut rot_ndata, NodeData::VERTEX_RIGHT);
                Self::replace_into(n.data, NodeData::EDGE_RIGHT, &mut rot_ndata, NodeData::EDGE_UP);

                out.push(UnnormalizedNode { data: rot_ndata, pos });
            }

            if n.data.contains(NodeData::EDGE_UP) {
                out.push(UnnormalizedNode {
                    data: NodeData::EDGE_RIGHT,
                    pos: pos - Vec2i::new(1, 0),
                });
            }
        })
    }

    pub fn flipped_hor(&self) -> Self {
        self.flat_map(|n, out| {
            let pos = Vec2i::new(-(n.pos.x as i32), n.pos.y as i32);

            if n.data.intersects(NodeData::EDGE_UP | NodeData::ALL_VERTICES) {
                let mut flipped_ndata =
                    NodeData::intersection(n.data, NodeData::EDGE_UP | NodeData::VERTEX_UP | NodeData::VERTEX_DOWN);

                Self::replace_into(
                    n.data,
                    NodeData::VERTEX_RIGHT,
                    &mut flipped_ndata,
                    NodeData::VERTEX_LEFT,
                );

                Self::replace_into(
                    n.data,
                    NodeData::VERTEX_LEFT,
                    &mut flipped_ndata,
                    NodeData::VERTEX_RIGHT,
                );

                out.push(UnnormalizedNode {
                    data: flipped_ndata,
                    pos,
                });
            }

            if n.data.contains(NodeData::EDGE_RIGHT) {
                out.push(UnnormalizedNode {
                    data: NodeData::EDGE_RIGHT,
                    pos: pos - Vec2i::new(1, 0),
                });
            }
        })
    }

    fn replace_into(src: NodeData, s: NodeData, dst: &mut NodeData, r: NodeData) {
        if src.contains(s) {
            dst.insert(r);
        }
    }

    fn flat_map<F: FnMut(&Node, &mut Vec<UnnormalizedNode>)>(&self, mut f: F) -> Self {
        let mut nodes_map: HashMap<Vec2i, NodeData> = HashMap::new();

        for n in self.nodes() {
            let mut uns = vec![];
            f(n, &mut uns);

            for un in uns {
                assert!(!un.data.is_empty());

                nodes_map.entry(un.pos).or_insert(NodeData::empty()).insert(un.data);
            }
        }

        let unnorm_nodes = nodes_map
            .drain()
            .map(|(pos, data)| UnnormalizedNode { data, pos })
            .collect::<Vec<UnnormalizedNode>>();

        Self::from(unnorm_nodes)
    }

    pub fn nodes(&self) -> &[Node] {
        self.nodes.as_ref()
    }

    pub fn bb_top_right(&self) -> Vec2 {
        self.bb_top_right
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
    use crate::logic::{
        common::{NodeData, Vec2},
        digits::{digit1, digit1_rot_ccw, digit2, digit5, digit8, digits},
    };

    use super::Shape;

    #[test]
    #[should_panic]
    fn new_fails_with_duplicate_nodes() {
        let _ = Shape::from(
            [
                ((0, 0), vec![NodeData::VERTEX_RIGHT]),
                ((0, 0), vec![NodeData::EDGE_RIGHT]),
            ]
            .as_slice(),
        );
    }

    #[test]
    #[should_panic]
    fn new_fails_with_empty_nodes() {
        let _ = Shape::from([((0, 0), vec![NodeData::empty()])].as_slice());
    }

    #[test]
    fn new_correctly_aligns_to_origin() {
        let s = Shape::from([((2, 1), vec![NodeData::VERTEX_RIGHT])].as_slice());

        assert_eq!(s.nodes.len(), 1);
        assert_eq!(s.nodes[0].pos, Vec2::ZERO);
        assert_eq!(s.nodes[0].data, NodeData::VERTEX_RIGHT);
    }

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

            assert_eq!(d, d_rot);
        }
    }

    #[test]
    fn flipped_hor_is_involution() {
        let digits = digits();

        for d in digits {
            let d_flipped = d.flipped_hor().flipped_hor();

            assert_eq!(d, d_flipped);
        }
    }

    #[test]
    fn flipped_hor_works() {
        assert_eq!(digit2(), digit5().flipped_hor());
        assert_eq!(digit2().flipped_hor(), digit5());
        assert_eq!(digit8().flipped_hor(), digit8());
    }
}
