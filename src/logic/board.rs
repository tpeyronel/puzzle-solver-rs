use super::{common::Vec2, node_matrix::NodeMatrix, shape::Shape};

struct ShapePlacement {
    pos: Vec2,
    shape: Shape,
}

pub struct Board {
    matrix: NodeMatrix,
    placements: Vec<ShapePlacement>,
}

impl Board {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            matrix: NodeMatrix::new(width, height),
            placements: vec![],
        }
    }

    pub fn fits_at(&self, shape: &Shape, pos: Vec2) -> bool {
        let bb_top_right = pos + shape.bb_top_right();
        if bb_top_right.x >= self.matrix.width() || bb_top_right.y >= self.matrix.height() {
            return false;
        }

        for n in shape.nodes() {
            if self.matrix[pos + n.pos].intersects(n.data) {
                return false;
            }
        }

        return true;
    }

    pub fn put_at(&mut self, shape: &Shape, pos: Vec2) {
        for n in shape.nodes() {
            self.matrix[pos + n.pos].insert(n.data);
        }

        self.placements.push(ShapePlacement {
            pos,
            shape: shape.clone(),
        });
    }

    pub fn remove_last(&mut self) {
        let ShapePlacement { pos, shape } = self.placements.pop().expect("tried to remove_last() with no pieces");

        for n in shape.nodes() {
            self.matrix[pos + n.pos].remove(n.data);
        }
    }

    pub fn width(&self) -> u32 {
        self.matrix.width()
    }

    pub fn height(&self) -> u32 {
        self.matrix.height()
    }

    pub fn is_empty(&self) -> bool {
        assert_eq!(self.matrix.is_empty(), self.placements.is_empty());

        self.matrix.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::logic::{board::Board, common::Vec2, digits, shape::Shape};

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

    #[test]
    fn digits_0_1_4_7_8_fit() {
        let mut b = Board::new(5, 3);

        let digits: Vec<((u32, u32), Shape)> = vec![
            ((0, 0), digits::digit0().rotated_ccw().rotated_ccw()),
            (
                (0, 0),
                digits::digit7().flipped_hor().rotated_ccw().rotated_ccw().rotated_ccw(),
            ),
            (
                (1, 1),
                digits::digit4().flipped_hor().rotated_ccw().rotated_ccw().rotated_ccw(),
            ),
            ((1, 0), digits::digit1().rotated_ccw().rotated_ccw().rotated_ccw()),
            ((3, 0), digits::digit8()),
        ];

        for (p, d) in &digits {
            let pos = Vec2::from(*p);

            assert!(b.fits_at(&d, pos));
            b.put_at(d, pos);
        }

        for _ in 0..digits.len() {
            b.remove_last();
        }

        assert!(b.matrix.is_empty());
        assert!(b.placements.is_empty());
    }
}
