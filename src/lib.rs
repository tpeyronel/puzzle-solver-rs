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

use logic::digits::digit2;

pub use crate::logic::{
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
