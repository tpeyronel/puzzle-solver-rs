use crate::logic::{
    digits::{digit0, digit1, digit2, digit3, digit4, digit5, digit6, digit7, digit8, digit9},
    solver::Solver,
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

fn main() {
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
