use super::{common::Vec2, node_matrix::NodeMatrix, shape_mesh::ShapeMesh};

pub struct Board {
    matrix: NodeMatrix,
}

impl Board {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            matrix: NodeMatrix::new(width, height),
        }
    }

    pub fn fits_at(&self, mesh: &ShapeMesh, pos: Vec2) -> bool {
        let bb_top_right = pos + mesh.bb_top_right();
        if bb_top_right.x >= self.matrix.width() || bb_top_right.y >= self.matrix.height() {
            return false;
        }

        for n in mesh.nodes() {
            if self.matrix[pos + n.pos].intersects(n.data) {
                return false;
            }
        }

        return true;
    }

    pub fn insert_at(&mut self, mesh: &ShapeMesh, pos: Vec2) {
        for n in mesh.nodes() {
            self.matrix[pos + n.pos].insert(n.data);
        }
    }

    pub fn remove_at(&mut self, mesh: &ShapeMesh, pos: Vec2) {
        for n in mesh.nodes() {
            self.matrix[pos + n.pos].remove(n.data);
        }
    }

    pub fn width(&self) -> u32 {
        self.matrix.width()
    }

    pub fn height(&self) -> u32 {
        self.matrix.height()
    }

    #[allow(unused)]
    pub fn is_empty(&self) -> bool {
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

        assert!(b.fits_at(d0.mesh(), Vec2::new(0, 0)));
        assert!(!b.fits_at(d0.mesh(), Vec2::new(1, 0)));
    }

    #[test]
    fn digit1_fits() {
        let b = Board::new(1, 3);
        let d1 = digits::digit1();

        assert!(b.fits_at(d1.mesh(), Vec2::new(0, 0)));
        assert!(!b.fits_at(d1.mesh(), Vec2::new(1, 0)));
    }

    #[test]
    fn digits_0_and_1_fit() {
        let mut b = Board::new(3, 3);
        let d0 = digits::digit0();
        let d1 = digits::digit1().rotated_ccw().rotated_ccw().rotated_ccw();

        assert!(b.fits_at(d0.mesh(), Vec2::new(1, 0)));
        b.insert_at(d0.mesh(), (1, 0).into());

        assert!(b.fits_at(d1.mesh(), Vec2::new(0, 1)));
    }

    #[test]
    fn digits_0_1_4_7_8_fit() {
        let mut b = Board::new(5, 3);

        let digits: Vec<(Vec2, Shape)> = vec![
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
        ]
        .into_iter()
        .map(|(p, s)| (Vec2::from(p), s))
        .collect();

        for (p, d) in &digits {
            assert!(b.fits_at(d.mesh(), *p));
            b.insert_at(d.mesh(), *p);
        }

        for (p, d) in &digits {
            b.remove_at(d.mesh(), *p);
        }

        assert!(b.matrix.is_empty());
    }
}
