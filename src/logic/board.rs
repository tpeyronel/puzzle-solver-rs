use super::{
    common::{Shape, Vec2},
    node_matrix::NodeMatrix,
};

struct PlacedShape {
    shape: Shape,
    pos: Vec2,
}

pub struct Board {
    matrix: NodeMatrix,
    placements: Vec<PlacedShape>,
}

impl Board {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            matrix: NodeMatrix::new(width, height),
            placements: vec![],
        }
    }

    pub fn fits_at(&self, shape: &Shape, pos: Vec2) -> bool {
        let bb_top_right = pos + shape.bb_top_right;
        if bb_top_right.x >= self.matrix.width() || bb_top_right.y >= self.matrix.height() {
            return false;
        }

        for n in &shape.nodes {
            if self.matrix[pos + n.pos].intersects(n.data) {
                return false;
            }
        }

        return true;
    }

    pub fn put_at(&mut self, shape: &Shape, pos: Vec2) {
        for n in &shape.nodes {
            self.matrix[pos + n.pos].insert(n.data);
        }

        self.placements.push(PlacedShape {
            shape: shape.clone(),
            pos,
        });
    }

    pub fn remove_last(&mut self) {
        let PlacedShape { shape, pos } = self.placements.pop().expect("tried to remove_last() with no pieces");

        for n in shape.nodes {
            self.matrix[pos + n.pos].remove(n.data);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::logic::{board::Board, common::Vec2, digits};

    #[test]
    fn digit0_fits() {
        let b = Board::new(2, 3);
        let d0 = digits::digit0();

        assert!(b.fits_at(&d0, Vec2::new(0, 0)));
        assert!(!b.fits_at(&d0, Vec2::new(1, 0)));
    }

    #[test]
    fn digit1_fits() {
        let b = Board::new(1, 3);
        let d1 = digits::digit1();

        assert!(b.fits_at(&d1, Vec2::new(0, 0)));
        assert!(!b.fits_at(&d1, Vec2::new(1, 0)));
    }

    #[test]
    fn digits_0_and_1_fit() {
        let mut b = Board::new(3, 3);
        let d0 = digits::digit0();
        let d1 = digits::digit1().rotated_ccw().rotated_ccw().rotated_ccw();

        assert!(b.fits_at(&d0, Vec2::new(1, 0)));
        b.put_at(&d0, (1, 0).into());

        assert!(b.fits_at(&d1, Vec2::new(0, 1)));
    }
}
