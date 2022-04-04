Cantor is a general toolkit for working with types that have a small number of values (typically,
but not exclusively `enum`s). This crate defines the `Finite` trait and implements several
efficient zero-allocation algorithms on top of it.

## Applications

* Iterating over possible values
* Value compression
* Array-based maps
* Bitmap sets

## Example
```rust
// Define a "Finite" type
#[derive(Finite, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum MyType {
	A,
	B(bool),
	C(bool, bool)
}

// Value compression
let value = MyType::B(false);
assert_eq!(size_of_val(&value), 3);
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
```