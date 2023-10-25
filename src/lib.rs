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

pub use crate::logic::{
    digits::{digit0, digit1, digit2, digit3, digit4, digit5, digit6, digit7, digit8, digit9, digits},
    node_matrix::NodeMatrix,
    solver::{solve, Solution, SolutionMessage},
};

pub fn dummy_node_matrix() -> NodeMatrix {
    let mut mat = NodeMatrix::new(2, 3);

    let digit2 = digit2();

    for n in digit2.mesh().nodes() {
        mat[n.pos] = n.data;
    }

    mat
}
