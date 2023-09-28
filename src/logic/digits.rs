use super::common::{NodeData, Shape};

#[rustfmt::skip]
pub fn digit0() -> Shape {
    Shape::from(
        [
            (
                (0, 0),
                vec![
                    NodeData::EDGE_RIGHT,
                    NodeData::EDGE_UP,
                    NodeData::VERTEX_RIGHT,
                    NodeData::VERTEX_UP,
                ],
            ),
            (
                (0, 1),
                vec![
                    NodeData::EDGE_RIGHT,
                    NodeData::EDGE_UP,
                    NodeData::VERTEX_RIGHT,
                    NodeData::VERTEX_UP,
                    NodeData::VERTEX_DOWN,
                ],
            ),
            (
                (0, 2),
                vec![
                    NodeData::EDGE_RIGHT,
                    NodeData::VERTEX_RIGHT,
                    NodeData::VERTEX_DOWN,
                ],
            ),
            (
                (1, 0),
                vec![
                    NodeData::EDGE_UP,
                    NodeData::VERTEX_UP,
                    NodeData::VERTEX_LEFT,
                ],
            ),
            (
                (1, 1),
                vec![
                    NodeData::EDGE_UP,
                    NodeData::VERTEX_UP,
                    NodeData::VERTEX_LEFT,
                    NodeData::VERTEX_DOWN,
                ],
            ),
            (
                (1, 2),
                vec![
                    NodeData::VERTEX_LEFT,
                    NodeData::VERTEX_DOWN,
                ],
            ),
        ]
        .as_slice(),
    )
}

#[rustfmt::skip]
pub fn digit1() -> Shape {
    Shape::from(
        [
            (
                (0, 0),
                vec![
                    NodeData::EDGE_UP,
                    NodeData::VERTEX_UP,
                ],
            ),
            (
                (0, 1),
                vec![
                    NodeData::EDGE_UP,
                    NodeData::VERTEX_RIGHT,
                    NodeData::VERTEX_UP,
                    NodeData::VERTEX_DOWN,
                ],
            ),
            (
                (0, 2),
                vec![
                    NodeData::VERTEX_DOWN,
                ],
            ),
        ]
        .as_slice(),
    )
}

#[rustfmt::skip]
pub fn digit2() -> Shape {
    Shape::from(
        [
            (
                (0, 0),
                vec![
                    NodeData::EDGE_RIGHT,
                    NodeData::EDGE_UP,
                    NodeData::VERTEX_RIGHT,
                    NodeData::VERTEX_UP,
                ],
            ),
            (
                (0, 1),
                vec![
                    NodeData::EDGE_RIGHT,
                    NodeData::VERTEX_RIGHT,
                    NodeData::VERTEX_DOWN,
                ],
            ),
            (
                (0, 2),
                vec![
                    NodeData::EDGE_RIGHT,
                    NodeData::VERTEX_RIGHT,
                ],
            ),
            (
                (1, 0),
                vec![
                    NodeData::VERTEX_LEFT,
                ],
            ),
            (
                (1, 1),
                vec![
                    NodeData::EDGE_UP,
                    NodeData::VERTEX_UP,
                    NodeData::VERTEX_LEFT,
                ],
            ),
            (
                (1, 2),
                vec![
                    NodeData::VERTEX_LEFT,
                    NodeData::VERTEX_DOWN,
                ],
            ),
        ]
        .as_slice(),
    )
}

#[rustfmt::skip]
pub fn digit3() -> Shape {
    Shape::from(
        [
            (
                (0, 0),
                vec![
                    NodeData::EDGE_RIGHT,
                    NodeData::VERTEX_RIGHT,
                ],
            ),
            (
                (0, 1),
                vec![
                    NodeData::EDGE_RIGHT,
                    NodeData::VERTEX_RIGHT,
                ],
            ),
            (
                (0, 2),
                vec![
                    NodeData::EDGE_RIGHT,
                    NodeData::VERTEX_RIGHT,
                ],
            ),
            (
                (1, 0),
                vec![
                    NodeData::EDGE_UP,
                    NodeData::VERTEX_UP,
                    NodeData::VERTEX_LEFT,
                ],
            ),
            (
                (1, 1),
                vec![
                    NodeData::EDGE_UP,
                    NodeData::VERTEX_UP,
                    NodeData::VERTEX_LEFT,
                    NodeData::VERTEX_DOWN,
                ],
            ),
            (
                (1, 2),
                vec![
                    NodeData::VERTEX_LEFT,
                    NodeData::VERTEX_DOWN,
                ],
            ),
        ]
        .as_slice(),
    )
}

#[rustfmt::skip]
pub fn digit4() -> Shape {
    Shape::from(
        [
            (
                (0, 1),
                vec![
                    NodeData::EDGE_RIGHT,
                    NodeData::EDGE_UP,
                    NodeData::VERTEX_RIGHT,
                    NodeData::VERTEX_UP,
                ],
            ),
            (
                (0, 2),
                vec![
                    NodeData::VERTEX_DOWN,
                ],
            ),
            (
                (1, 0),
                vec![
                    NodeData::EDGE_UP,
                    NodeData::VERTEX_UP,
                ],
            ),
            (
                (1, 1),
                vec![
                    NodeData::EDGE_UP,
                    NodeData::VERTEX_UP,
                    NodeData::VERTEX_LEFT,
                    NodeData::VERTEX_DOWN,
                ],
            ),
            (
                (1, 2),
                vec![
                    NodeData::VERTEX_DOWN,
                ],
            ),
        ]
        .as_slice(),
    )
}

pub fn digits() -> Vec<Shape> {
    vec![digit0(), digit1()]
}