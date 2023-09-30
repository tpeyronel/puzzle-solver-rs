use super::{common::NodeData, shape::Shape};

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
                    NodeData::EDGE_UP,
                    NodeData::VERTEX_UP,
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
                    NodeData::VERTEX_RIGHT,
                    NodeData::VERTEX_UP,
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

#[rustfmt::skip]
pub fn digit5() -> Shape {
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
                    NodeData::EDGE_UP,
                    NodeData::VERTEX_RIGHT,
                    NodeData::VERTEX_UP,
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
                    NodeData::VERTEX_LEFT,
                    NodeData::VERTEX_DOWN,
                ],
            ),
            (
                (1, 2),
                vec![
                    NodeData::VERTEX_LEFT,
                ],
            ),
        ]
        .as_slice(),
    )
}

#[rustfmt::skip]
pub fn digit6() -> Shape {
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
                    NodeData::VERTEX_LEFT,
                    NodeData::VERTEX_DOWN,
                ],
            ),
            (
                (1, 2),
                vec![
                    NodeData::VERTEX_LEFT,
                ],
            ),
        ]
        .as_slice(),
    )
}

#[rustfmt::skip]
pub fn digit7() -> Shape {
    Shape::from(
        [
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
                ],
            ),
            (
                (1, 1),
                vec![
                    NodeData::EDGE_UP,
                    NodeData::VERTEX_UP,
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
pub fn digit8() -> Shape {
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
pub fn digit9() -> Shape {
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
                    NodeData::EDGE_UP,
                    NodeData::VERTEX_RIGHT,
                    NodeData::VERTEX_UP,
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

pub fn digits() -> Vec<Shape> {
    vec![
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
    ]
}

pub fn digit1_rot_ccw() -> Shape {
    Shape::from(
        [
            ((0, 0), vec![NodeData::EDGE_RIGHT, NodeData::VERTEX_RIGHT]),
            (
                (1, 0),
                vec![
                    NodeData::EDGE_RIGHT,
                    NodeData::VERTEX_RIGHT,
                    NodeData::VERTEX_UP,
                    NodeData::VERTEX_LEFT,
                ],
            ),
            ((2, 0), vec![NodeData::VERTEX_LEFT]),
        ]
        .as_slice(),
    )
}

#[cfg(test)]
mod tests {
    use crate::logic::common::Node;

    #[test]
    fn digits_are_sorted() {
        let digits = super::digits();

        for d in digits {
            assert!(Node::is_sorted(d.nodes()));
        }
    }

    #[test]
    fn digits_6_and_9_are_rotations() {
        let d6 = super::digit6();
        let d9 = super::digit9();

        assert_eq!(d6.rotated_ccw().rotated_ccw(), d9);
        assert_eq!(d9.rotated_ccw().rotated_ccw(), d6);
    }
}
