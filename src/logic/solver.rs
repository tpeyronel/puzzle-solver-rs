use std::fmt::Display;

use super::{board::Board, common::Vec2, shape::Shape};

struct Candidate {
    id: String,
    variations: Vec<Shape>,
}

#[derive(Debug, Clone)]
pub struct SolutionWithBorrows<'a> {
    placed_shapes: Vec<(Vec2, &'a Shape)>,
}

#[derive(Debug, Clone)]
pub struct Solution {
    pub placed_shapes: Vec<(Vec2, Shape)>,
}

impl<'a> From<SolutionWithBorrows<'a>> for Solution {
    fn from(solution: SolutionWithBorrows<'a>) -> Self {
        Self {
            placed_shapes: solution
                .placed_shapes
                .into_iter()
                .map(|(p, s)| (p, s.clone()))
                .collect(),
        }
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const COL_WIDTH: usize = 8;

        let mut output = format!(
            "{0: ^w$}   {1: ^w$}   {2: ^w$}   {3: ^w$}\n",
            "shape",
            "rot",
            "pos",
            "flipped",
            w = COL_WIDTH,
        );

        for (p, s) in &self.placed_shapes {
            output += &format!(
                "{0: ^w$}   {1: ^w$}   {2: ^w$}   {3: ^w$}\n",
                format!("\"{}\"", s.metadata().id),
                s.metadata().rot,
                format!("({:>2},{:>2})", p.x, p.y),
                if s.metadata().flipped { "y" } else { "n" },
                w = COL_WIDTH,
            );
        }

        f.write_str(&output)
    }
}

pub struct Solver<'a> {
    board: Board,
    total_remaining: u32,
    solution: SolutionWithBorrows<'a>,
}

impl<'a> Solver<'a> {
    pub fn solve(width: u32, height: u32, shapes: Vec<Shape>) -> Option<Solution> {
        let (candidates, mut remaining) = Self::to_candidates(shapes);

        let mut solver = Solver::new(width, height, remaining.iter().sum());

        if solver.solve_rec(&candidates, &mut remaining, Vec2::ZERO) {
            Some(solver.solution.into())
        } else {
            None
        }
    }

    fn new(width: u32, height: u32, total_remaining: u32) -> Self {
        Self {
            board: Board::new(width, height),
            total_remaining,
            solution: SolutionWithBorrows { placed_shapes: vec![] },
        }
    }

    fn solve_rec(&mut self, candidates: &'a Vec<Candidate>, remaining: &mut Vec<u32>, pos: Vec2) -> bool {
        if self.total_remaining == 0 {
            return true;
        }
        if pos.x >= self.board.width() || pos.y >= self.board.height() {
            return false;
        }

        if (7..).contains(&self.total_remaining) {
            println!("{}", self.total_remaining);
        }

        for (i, c) in candidates.iter().enumerate() {
            if remaining[i] == 0 {
                continue;
            }

            remaining[i] -= 1;
            self.total_remaining -= 1;

            for v in &c.variations {
                if self.board.fits_at(v.mesh(), pos) {
                    self.solution.placed_shapes.push((pos, v));
                    self.board.insert_at(v.mesh(), pos);

                    if self.solve_rec(candidates, remaining, pos) {
                        return true;
                    }

                    self.board.remove_at(v.mesh(), pos);
                    self.solution.placed_shapes.pop();
                }
            }

            self.total_remaining += 1;
            remaining[i] += 1;
        }

        let next_pos = if pos.y + 1 == self.board.height() {
            Vec2::new(pos.x + 1, 0)
        } else {
            Vec2::new(pos.x, pos.y + 1)
        };

        return self.solve_rec(candidates, remaining, next_pos);
    }

    fn to_candidates(shapes: Vec<Shape>) -> (Vec<Candidate>, Vec<u32>) {
        shapes
            .iter()
            .map(|s| {
                (
                    Candidate {
                        id: s.metadata().id.clone(),
                        variations: Self::compute_variations(s),
                    },
                    1,
                )
            })
            .collect::<Vec<(Candidate, u32)>>()
            .into_iter()
            .unzip()
    }

    fn compute_variations(shape: &Shape) -> Vec<Shape> {
        fn push_rotations(mut shape: Shape, variations: &mut Vec<Shape>) {
            for _ in 0..4 {
                let next = shape.rotated_ccw();
                variations.push(shape);
                shape = next;
            }
        }

        let mut variations = vec![];

        push_rotations(shape.clone(), &mut variations);
        push_rotations(shape.flipped_hor(), &mut variations);

        variations
    }
}

#[cfg(test)]
mod tests {
    use crate::logic::digits::{digit0, digit1, digit2, digit3, digit4, digit5, digit6, digit7, digit8, digit9};

    use super::Solver;

    #[test]
    fn solve_works_for_digits_8() {
        let digits = vec![digit8()];

        let solution = Solver::solve(2, 3, digits);

        println!("{:?}", solution.unwrap());
    }

    #[test]
    fn solve_works_for_digits_3_8() {
        let digits = vec![digit3(), digit8()];

        let solution = Solver::solve(3, 3, digits);

        println!("{:?}", solution.unwrap());
    }

    #[test]
    fn solve_works_for_digits_0_1_7() {
        let digits = vec![digit0(), digit1(), digit7()];

        let solution = Solver::solve(3, 3, digits);

        println!("{:?}", solution.unwrap());
    }

    #[test]
    fn solve_works_for_digits_6_9() {
        let digits = vec![digit9(), digit6()];

        let solution = Solver::solve(3, 3, digits);

        println!("{:?}", solution.unwrap());
    }

    #[test]
    fn solve_works_for_digits_3_6_9() {
        let digits = vec![digit3(), digit9(), digit6()];

        let solution = Solver::solve(3, 4, digits);

        println!("{:?}", solution.unwrap());
    }

    #[test]
    fn solve_works_for_digits_0_1_4_7_8() {
        let digits = vec![digit0(), digit1(), digit4(), digit7(), digit8()];

        let solution = Solver::solve(5, 3, digits);

        println!("{:?}", solution.unwrap());
    }

    #[test]
    fn solve_works_for_all_digits() {
        let digits = vec![
            digit0(),
            digit1(),
            digit2(),
            digit3(),
            digit4(),
            digit5(),
            digit6(),
            digit7(),
            digit8(),
            digit9(),
        ];

        let solution = Solver::solve(6, 5, digits);

        println!("{:?}", solution.unwrap());
    }
}
