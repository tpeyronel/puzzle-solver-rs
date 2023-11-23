use serde::Serialize;

use crate::logic::solver::Solution;

use super::{common::Vec2, node::NodeData, shape::Shape};

#[derive(Debug, Clone, Serialize)]
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

    #[allow(unused)]
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

impl From<Shape> for NodeMatrix {
    fn from(shape: Shape) -> Self {
        let nodes = shape.mesh().nodes();

        let top_right_pos = nodes.iter().map(|n| n.pos).reduce(Vec2::max).unwrap_or(Vec2::ZERO);

        let mut mat = Self::new(top_right_pos.x + 1, top_right_pos.y + 1);

        for n in nodes {
            mat[n.pos] = n.data;
        }

        mat
    }
}

impl From<&Solution> for NodeMatrix {
    fn from(solution: &Solution) -> Self {
        let top_right_pos = solution
            .placed_shapes
            .iter()
            .map(|(pos, sh)| sh.mesh().nodes().iter().map(|n| *pos + n.pos))
            .flatten()
            .reduce(Vec2::max)
            .unwrap_or(Vec2::ZERO);

        let mut mat = Self::new(top_right_pos.x + 1, top_right_pos.y + 1);

        for (pos, shape) in &solution.placed_shapes {
            let mesh = shape.mesh();

            for n in mesh.nodes() {
                mat[*pos + n.pos] = n.data;
            }
        }

        mat
    }
}

#[cfg(test)]
mod tests {
    use crate::logic::{common::Vec2, digits, node::NodeData, node_matrix::NodeMatrix};

    #[test]
    fn digit2_to_node_matrix() {
        let digit2 = digits::digit2();

        let matrix = NodeMatrix::from(digit2.clone());

        assert_eq!(matrix.width, 2);
        assert_eq!(matrix.height, 3);

        for x in 0..matrix.width {
            for y in 0..matrix.height {
                let pos = Vec2::new(x, y);

                if let Some(node) = digit2.mesh().nodes().into_iter().find(|n| n.pos == pos) {
                    assert_eq!(matrix[pos], node.data);
                } else {
                    assert_eq!(matrix[pos], NodeData::empty());
                }
            }
        }
    }
}
