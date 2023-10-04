use logic::shape::{Shape, ShapeWithMetadata};

use crate::logic::{
    digits::{digit0, digit1, digit2, digit3, digit4, digit5, digit6, digit7, digit8, digit9},
    solver::Solver,
};

mod logic {
    pub mod board;
    pub mod common;
    pub mod digits;
    pub mod node_matrix;
    pub mod shape;
    pub mod solver;
}

// TODO: with_metadata is duplicated in solver.test.js
fn with_metadata<I: IntoIterator<Item = (&'static str, Shape)>>(digits: I) -> Vec<ShapeWithMetadata> {
    digits
        .into_iter()
        .map(|(id, s)| ShapeWithMetadata::new(id.to_string(), s))
        .collect::<Vec<ShapeWithMetadata>>()
}

fn main() {
    let digits = with_metadata([
        ("0", digit0().rotated_ccw().rotated_ccw()),
        ("7", digit7()),
        ("1", digit1().rotated_ccw()),
        ("6", digit6()),
        ("2", digit2()),
        ("3", digit3()),
        ("4", digit4()),
        ("5", digit5()),
        ("8", digit8()),
        ("9", digit9()),
    ]);

    let mut solver = Solver::new(6, 5);

    let solution = solver.solve(digits);

    println!("{:?}", solution.unwrap());
}

#[test]
fn test1() {
    println!("Test1!");
}

#[test]
fn test2() {
    println!("Test2!");
}
