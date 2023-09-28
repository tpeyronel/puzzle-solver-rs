use super::common::{NodeData, Shape, Vec2};
use std::ops::{Deref, Index, IndexMut};

struct BoardState {
    width: usize,
    height: usize,
    nodes: Box<[NodeData]>,
}

impl BoardState {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            nodes: vec![NodeData::empty(); width * height].into_boxed_slice(),
        }
    }
}

impl Index<Vec2> for BoardState {
    type Output = NodeData;

    fn index(&self, pos: Vec2) -> &Self::Output {
        return &self.nodes[pos.x as usize + pos.y as usize * self.width];
    }
}

impl IndexMut<Vec2> for BoardState {
    fn index_mut(&mut self, pos: Vec2) -> &mut Self::Output {
        return &mut self.nodes[pos.x as usize + pos.y as usize * self.width];
    }
}

struct PlacedShape {
    shape: Shape,
    pos: Vec2,
}

pub struct Board {
    state: BoardState,
    placements: Vec<PlacedShape>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            state: BoardState::new(width, height),
            placements: vec![],
        }
    }

    pub fn fits_at(&self, shape: Shape, pos: Vec2) -> bool {
        for n in shape.deref() {
            if self.state[pos + n.pos].intersects(n.data) {
                return false;
            }
        }

        return true;
    }

    pub fn put_at(&mut self, shape: Shape, pos: Vec2) {
        for n in shape.deref() {
            self.state[pos + n.pos].insert(n.data);
        }

        self.placements.push(PlacedShape { shape, pos });
    }

    pub fn remove_last(&mut self) {
        let PlacedShape { shape, pos } = self
            .placements
            .pop()
            .expect("tried to remove_last() with no pieces");

        for n in shape.deref() {
            self.state[pos + n.pos].remove(n.data);
        }
    }
}
