use super::{common::Vec2, node::NodeData};

#[derive(Debug, Clone)]
pub struct NodeMatrix {
    width: u32,
    height: u32,
    data: Box<[NodeData]>,
}

impl NodeMatrix {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![NodeData::empty(); (width * height) as usize].into_boxed_slice(),
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn is_empty(&self) -> bool {
        self.data.iter().all(|nd| nd.is_empty())
    }

    fn calc_index(&self, x: u32, y: u32) -> usize {
        (y + x * self.height) as usize
    }
}

impl std::ops::Index<Vec2> for NodeMatrix {
    type Output = NodeData;

    fn index(&self, pos: Vec2) -> &Self::Output {
        return &self.data[self.calc_index(pos.x, pos.y)];
    }
}

impl std::ops::IndexMut<Vec2> for NodeMatrix {
    fn index_mut(&mut self, pos: Vec2) -> &mut Self::Output {
        return &mut self.data[self.calc_index(pos.x, pos.y)];
    }
}

impl std::ops::Index<(u32, u32)> for NodeMatrix {
    type Output = NodeData;

    fn index(&self, pos: (u32, u32)) -> &Self::Output {
        return &self.data[self.calc_index(pos.0, pos.1)];
    }
}

impl std::ops::IndexMut<(u32, u32)> for NodeMatrix {
    fn index_mut(&mut self, pos: (u32, u32)) -> &mut Self::Output {
        return &mut self.data[self.calc_index(pos.0, pos.1)];
    }
}
