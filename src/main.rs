use crate::logic::{
    digits::{digit0, digit1, digit2, digit3, digit4, digit5, digit6, digit7, digit8, digit9},
    solver,
};

mod logic {
    pub mod board;
    pub mod common;
    pub mod digits;
    pub mod node;
    pub mod node_matrix;
    pub mod shape;
    pub mod shape_mesh;
    pub mod solver;
}

fn run_benchmark() {
    let digits = vec![
        digit0().rotated_ccw().rotated_ccw(),
        digit7(),
        digit1().rotated_ccw(),
        digit6(),
        digit2(),
        digit3(),
        digit4(),
        digit5(),
        digit8(),
        digit9(),
    ];

    for _ in 0..10 {
        solver::solve(6, 3, digits.clone());
    }
}

fn main() {
    run_benchmark();

    let digits = vec![
        digit0().rotated_ccw().rotated_ccw(),
        digit7(),
        digit1().rotated_ccw(),
        digit6(),
        digit2(),
        digit3(),
        digit4(),
        digit5(),
        digit8(),
        digit9(),
    ];

    let solution = solver::solve(6, 5, digits);

    println!("{}", solution.unwrap());
}
