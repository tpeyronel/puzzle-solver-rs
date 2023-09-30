use std::cmp::Ordering;
use std::collections::HashMap;

pub type Vec2 = glam::u32::UVec2;
pub type Vec2i = glam::i32::IVec2;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct NodeData: u8 {
        const EDGE_RIGHT   = 1 << 0;
        const EDGE_UP      = 1 << 1;
        const VERTEX_RIGHT = 1 << 2;
        const VERTEX_UP    = 1 << 3;
        const VERTEX_LEFT  = 1 << 4;
        const VERTEX_DOWN  = 1 << 5;

        const ALL_VERTICES = Self::from_bits_retain(
            Self::VERTEX_RIGHT.bits() | Self::VERTEX_UP.bits() | Self::VERTEX_LEFT.bits() | Self::VERTEX_DOWN.bits()
        ).bits();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Node {
    pub data: NodeData,
    pub pos: Vec2,
}

#[derive(Debug, Clone)]
pub struct Shape {
    pub nodes: Vec<Node>,
    pub bb_top_right: Vec2,
}

pub type RawNode = ((u32, u32), Vec<NodeData>);

impl Shape {
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

        let bottom_left = nodes_map.keys().fold(Vec2i::MAX, |acc, pos| Vec2i::min(acc, *pos));

        let mut nodes = nodes_map
            .drain()
            .map(|(pos, data)| Node {
                data,
                pos: (pos - bottom_left).as_uvec2(),
            })
            .collect::<Vec<Node>>();

        Node::sort(&mut nodes);

        Self::from(nodes)
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        assert_eq!(self.nodes.len(), other.nodes.len());

        return Iterator::zip(self.nodes.iter(), other.nodes.iter()).all(|(a, b)| a == b);
    }
}

impl Node {
    fn node_pos_cmp(a: &Node, b: &Node) -> Ordering {
        a.pos.x.cmp(&b.pos.x).then_with(|| a.pos.y.cmp(&b.pos.y))
    }

    pub fn sort(nodes: &mut Vec<Node>) {
        nodes.sort_by(Self::node_pos_cmp);
    }

    pub fn is_sorted(nodes: &Vec<Node>) -> bool {
        nodes.windows(2).all(|w| Self::node_pos_cmp(&w[0], &w[1]).is_lt())
    }

    pub fn bb_top_right<'a, I>(nodes: I) -> Vec2
    where
        I: IntoIterator<Item = &'a Node>,
    {
        let mut tr = Vec2::MIN;

        for n in nodes.into_iter() {
            tr = Vec2 {
                x: u32::max(tr.x, n.pos.x + n.data.contains(NodeData::EDGE_RIGHT) as u32),
                y: u32::max(tr.y, n.pos.y + n.data.contains(NodeData::EDGE_UP) as u32),
            };
        }

        tr
    }

    pub fn bb_bottom_left<'a, I>(nodes: I) -> Vec2
    where
        I: IntoIterator<Item = &'a Node>,
    {
        let mut bl = Vec2::MAX;

        for n in nodes.into_iter() {
            assert!(!n.data.is_empty());

            bl = Vec2 {
                x: u32::min(bl.x, n.pos.x),
                y: u32::min(bl.y, n.pos.y),
            };
        }

        bl
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

        let bb_top_right: Vec2 = Node::bb_top_right(&nodes);

        Self { nodes, bb_top_right }
    }
}

impl From<Vec<Node>> for Shape {
    fn from(nodes: Vec<Node>) -> Self {
        let bb_top_right = Node::bb_top_right(&nodes);

        Self { nodes, bb_top_right }
    }
}

#[cfg(test)]
mod tests {
    use crate::logic::digits::{digit1, digits, digit1_rot_ccw};

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
