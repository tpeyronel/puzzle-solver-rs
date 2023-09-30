use super::common::{NodeData, Vec2};

#[derive(Debug, Clone)]
pub struct NodeMatrix {
    width: u32,
    height: u32,
    node_data: Box<[NodeData]>,
}

impl NodeMatrix {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            node_data: vec![NodeData::empty(); (width * height) as usize].into_boxed_slice(),
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    fn calc_index(&self, x: u32, y: u32) -> usize {
        (y + x * self.height) as usize
    }
}

impl std::ops::Index<Vec2> for NodeMatrix {
    type Output = NodeData;

    fn index(&self, pos: Vec2) -> &Self::Output {
        return &self.node_data[self.calc_index(pos.x, pos.y)];
    }
}

impl std::ops::IndexMut<Vec2> for NodeMatrix {
    fn index_mut(&mut self, pos: Vec2) -> &mut Self::Output {
        return &mut self.node_data[self.calc_index(pos.x, pos.y)];
    }
}

impl std::ops::Index<(u32, u32)> for NodeMatrix {
    type Output = NodeData;

    fn index(&self, pos: (u32, u32)) -> &Self::Output {
        return &self.node_data[self.calc_index(pos.0, pos.1)];
    }
}

impl std::ops::IndexMut<(u32, u32)> for NodeMatrix {
    fn index_mut(&mut self, pos: (u32, u32)) -> &mut Self::Output {
        return &mut self.node_data[self.calc_index(pos.0, pos.1)];
    }
}
