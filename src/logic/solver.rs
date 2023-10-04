use std::sync::Arc;

use super::{
    board::Board,
    common::Vec2,
    shape::{ShapeMetadata, ShapeWithMetadata},
};

struct Candidate {
    id: Arc<String>,
    variations: Vec<ShapeWithMetadata>,
    remaining: u32,
}

#[derive(Debug, Clone)]
pub struct Solution {
    placed_shapes: Vec<(Vec2, ShapeMetadata)>,
}

pub struct Solver {
    board: Board,
    total_remaining: u32,
    solution: Solution,
}

impl Solver {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            board: Board::new(width, height),
            total_remaining: 0,
            solution: Solution { placed_shapes: vec![] },
        }
    }

    pub fn solve(&mut self, shapes: Vec<ShapeWithMetadata>) -> Option<Solution> {
        let mut candidates = Self::to_candidates(shapes);

        self.total_remaining = candidates.iter().map(|c| c.remaining).sum();

        if self.solve_rec(&mut candidates, Vec2::ZERO) {
            Some(self.solution.clone())
        } else {
            None
        }
    }

    fn solve_rec(&mut self, candidates: &mut Vec<Candidate>, pos: Vec2) -> bool {
        if self.total_remaining == 0 {
            return true;
        }
        if pos.x >= self.board.width() || pos.y >= self.board.height() {
            return false;
        }

        if (8..=10).contains(&self.total_remaining) {
            println!("{}", self.total_remaining);
        }

        for i in 0..candidates.len() {
            if candidates[i].remaining == 0 {
                continue;
            }

            candidates[i].remaining -= 1;
            self.total_remaining -= 1;

            for j in 0..candidates[i].variations.len() {
                let v = &candidates[i].variations[j];

                if self.board.fits_at(&v.shape, pos) {
                    self.solution.placed_shapes.push((pos, v.metadata.clone()));
                    self.board.put_at(&v.shape, pos);

                    if self.solve_rec(candidates, pos) {
                        return true;
                    }

                    self.board.remove_last();
                    self.solution.placed_shapes.pop();
                }
            }

            candidates[i].remaining += 1;
            self.total_remaining += 1;
        }

        let next_pos = if pos.y + 1 == self.board.height() {
            Vec2::new(pos.x + 1, 0)
        } else {
            Vec2::new(pos.x, pos.y + 1)
        };

        return self.solve_rec(candidates, next_pos);
    }

    fn to_candidates(shapes: Vec<ShapeWithMetadata>) -> Vec<Candidate> {
        shapes
            .iter()
            .map(|s| Candidate {
                id: s.metadata.id.clone(),
                variations: Self::compute_variations(s),
                remaining: 1,
            })
            .collect()
    }

    fn compute_variations(shape: &ShapeWithMetadata) -> Vec<ShapeWithMetadata> {
        let mut variations = vec![];

        let mut s = shape.shape.clone();
        for i in 0..4 {
            variations.push(ShapeWithMetadata {
                shape: s.clone(),
                metadata: ShapeMetadata {
                    id: shape.metadata.id.clone(),
                    rot: (shape.metadata.rot + i) % 4,
                    flipped: shape.metadata.flipped,
                },
            });
            s = s.rotated_ccw();
        }

        let mut s = shape.shape.flipped_hor();
        for i in 0..4 {
            variations.push(ShapeWithMetadata {
                shape: s.clone(),
                metadata: ShapeMetadata {
                    id: shape.metadata.id.clone(),
                    rot: (shape.metadata.rot + i) % 4,
                    flipped: !shape.metadata.flipped,
                },
            });
            s = s.rotated_ccw();
        }

        variations
    }
}

#[cfg(test)]
mod tests {
    use crate::logic::{
        digits::{digit0, digit1, digit2, digit3, digit4, digit5, digit6, digit7, digit8, digit9},
        shape::{Shape, ShapeWithMetadata},
    };

    use super::Solver;

    fn with_metadata<I: IntoIterator<Item = (&'static str, Shape)>>(digits: I) -> Vec<ShapeWithMetadata> {
        digits
            .into_iter()
            .map(|(id, s)| ShapeWithMetadata::new(id.to_string(), s))
            .collect::<Vec<ShapeWithMetadata>>()
    }

    #[test]
    fn solve_works_for_digits_8() {
        let digits = with_metadata([("8", digit8())]);

        let mut solver = Solver::new(2, 3);

        let solution = solver.solve(digits);

        println!("{:?}", solution);
    }

    #[test]
    fn solve_works_for_digits_3_8() {
        let digits = with_metadata([("3", digit3()), ("8", digit8())]);

        let mut solver = Solver::new(3, 3);

        let solution = solver.solve(digits);

        println!("{:?}", solution);
    }

    #[test]
    fn solve_works_for_digits_0_1_7() {
        let digits = with_metadata([("0", digit0()), ("1", digit1()), ("7", digit7())]);

        let mut solver = Solver::new(3, 3);

        let solution = solver.solve(digits);

        println!("{:?}", solution);
    }

    #[test]
    fn solve_works_for_digits_6_9() {
        let digits = with_metadata([("9", digit9()), ("6", digit6())]);

        let mut solver = Solver::new(3, 3);

        let solution = solver.solve(digits);

        println!("{:?}", solution);
    }

    #[test]
    fn solve_works_for_digits_3_6_9() {
        let digits = with_metadata([("3", digit3()), ("9", digit9()), ("6", digit6())]);

        let mut solver = Solver::new(3, 4);

        let solution = solver.solve(digits);

        println!("{:?}", solution);
    }

    #[test]
    fn solve_works_for_digits_0_1_4_7_8() {
        let digits = with_metadata([
            ("0", digit0()),
            ("1", digit1()),
            ("4", digit4()),
            ("7", digit7()),
            ("8", digit8()),
        ]);

        let mut solver = Solver::new(5, 3);

        let solution = solver.solve(digits);

        println!("{:?}", solution.unwrap());
    }

    #[test]
    fn solve_works_for_all_digits() {
        let digits = with_metadata([
            ("0", digit0()),
            ("1", digit1()),
            ("2", digit2()),
            ("3", digit3()),
            ("4", digit4()),
            ("5", digit5()),
            ("6", digit6()),
            ("7", digit7()),
            ("8", digit8()),
            ("9", digit9()),
        ]);

        let mut solver = Solver::new(6, 5);

        let solution = solver.solve(digits);

        println!("{:?}", solution.unwrap());
    }
}
