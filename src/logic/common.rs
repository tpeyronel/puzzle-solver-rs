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

