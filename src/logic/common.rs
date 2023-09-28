use std::ops::Deref;
use std::sync::Arc;

pub type Vec2 = glam::u32::UVec2;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct NodeData: u8 {
        const EDGE_RIGHT   = 1 << 0;
        const EDGE_UP      = 1 << 1;
        const VERTEX_RIGHT = 1 << 2;
        const VERTEX_UP    = 1 << 3;
        const VERTEX_LEFT  = 1 << 4;
        const VERTEX_DOWN  = 1 << 5;
    }
}

pub struct Node {
    pub data: NodeData,
    pub pos: Vec2,
}

pub struct Shape(Arc<Vec<Node>>);

impl Deref for Shape {
    type Target = Vec<Node>;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

pub type RawNode = ((u32, u32), Vec<NodeData>);

impl From<&[RawNode]> for Shape {
    fn from(data: &[RawNode]) -> Self {
        Self(Arc::new(
            data.iter()
                .map(|(pos, nds)| Node {
                    data: nds.iter().fold(NodeData::empty(), |acc, &nd| acc | nd ),
                    pos: Vec2::new(pos.0, pos.1),
                })
                .collect(),
        ))
    }
}
