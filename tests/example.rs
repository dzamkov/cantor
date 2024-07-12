use cantor::*;
use core::mem::size_of_val;

#[test]
fn main() {
    // Define a "Finite" type
    #[derive(Finite, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
    enum MyType {
        A,
        B(bool),
        C(bool, bool)
    }

    // Value iteration
    let mut num_values = 0;
    for _ in MyType::iter() {
        num_values += 1;
    }
    assert_eq!(num_values, 7);

    // Value compression
    let value = MyType::B(false);
    assert_eq!(size_of_val(&value), 2);
    let compressed = compress(value);
    assert_eq!(size_of_val(&compressed), 1);
    assert_eq!(value, compressed.expand());

    // Array map
    let mut map = ArrayMap::default();
    map[MyType::B(true)] = 1;
    map[MyType::C(true, true)] = 2;
    assert_eq!(map[MyType::A], 0);
    assert_eq!(map[MyType::B(true)], 1);
    assert_eq!(map[MyType::C(true, true)], 2);

    // Bitmap set
    let mut set = BitmapSet::none();
    set.include(MyType::A);
    set.include(MyType::B(false));
    set.include(MyType::C(true, false));
    assert_eq!(set.size(), 3);
    assert!(set.contains(MyType::B(false)));
    assert!(!set.contains(MyType::C(false, true)));
}