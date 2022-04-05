#![allow(missing_docs)]
#![doc(hidden)]

/// Encapsulates the required operations for arrays required by this crate.
pub trait Array<T> {
    fn new(f: impl FnMut(usize) -> T) -> Self;
    fn as_slice(&self) -> &[T];
    fn as_slice_mut(&mut self) -> &mut [T];
}

impl<T, const N: usize> Array<T> for [T; N] {
    fn new(f: impl FnMut(usize) -> T) -> Self {
        array_init::array_init(f)
    }

    fn as_slice(&self) -> &[T] {
        self
    }

    fn as_slice_mut(&mut self) -> &mut [T] {
        self
    }
}