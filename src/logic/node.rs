use super::common::{Vec2, Vec2i};

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
pub struct UnnormalizedNode {
    pub data: NodeData,
    pub pos: Vec2i,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Node {
    pub data: NodeData,
    pub pos: Vec2,
}

impl Node {
    pub fn align_to_origin(nodes: &mut [Node]) {
        let bottom_left = nodes.iter().map(|n: &Node| n.pos).fold(Vec2::MAX, Vec2::min);

        nodes.iter_mut().for_each(|n| {
            n.pos -= bottom_left;
        });
    }

    pub fn sort(nodes: &mut [Node]) {
        nodes.sort_by(Self::node_pos_cmp);
    }

    pub fn is_sorted(nodes: &[Node]) -> bool {
        nodes.windows(2).all(|w| Self::node_pos_cmp(&w[0], &w[1]).is_lt())
    }

    /// `nodes` must be sorted
    pub fn has_duplicates(nodes: &[Node]) -> bool {
        nodes.windows(2).any(|w| Self::node_pos_cmp(&w[0], &w[1]).is_eq())
    }

    pub fn has_empty_node(nodes: &[Node]) -> bool {
        nodes.iter().any(|n| n.data.is_empty())
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

    fn node_pos_cmp(a: &Node, b: &Node) -> std::cmp::Ordering {
        a.pos.x.cmp(&b.pos.x).then_with(|| a.pos.y.cmp(&b.pos.y))
    }
}
