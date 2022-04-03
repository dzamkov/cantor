use crate::*;

/// Ensures that the integer mapping of the given [`Finite`] is a valid bijection of the given
/// size.
#[allow(dead_code)]
fn validate<F: Finite>(expected: usize) {
    assert_eq!(expected, F::COUNT);
    for i in 0..F::COUNT {
        assert_eq!(i, F::index_of(&F::nth(i).unwrap()));
    }
    for i in 0..(F::COUNT - 1) {
        assert!(F::nth(i).unwrap() < F::nth(i + 1).unwrap());
    }
    assert!(F::nth(expected + 1).is_none());
}

#[derive(Finite, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue
}

#[test]
fn test_color() {
    validate::<Color>(3);
    validate::<Option<Color>>(4);
    validate::<(Color, Color)>(9);
}

#[derive(Finite, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Tile {
    Empty,
    Horizontal(Color),
    Vertical(Color),
    Cross {
        horizontal: Color,
        vertical: Color,
        is_horizontal_above: bool
    }
}

#[test]
fn test_tile() {
    validate::<Tile>(1 + 3 + 3 + 3 * 3 * 2);
}