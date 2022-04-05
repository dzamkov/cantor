use crate::uint::Unsigned;
use crate::*;

/// A compressed representation of a value of type `T`, implemented by storing its index
/// according [`Finite::index_of`] using the smallest integer type possible.
///
/// # Example
/// ```
/// use cantor::*;
/// use core::mem::size_of_val;
///
/// #[derive(Finite, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
/// enum MyType {
///     A,
///     B(bool),
///     C(bool, bool)
/// }
///
/// let value = MyType::B(false);
/// assert_eq!(size_of_val(&value), 3);
/// let compressed = compress(value);
/// assert_eq!(size_of_val(&compressed), 1);
/// assert_eq!(value, compressed.expand());
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Compress<T: CompressFinite>(T::Index);

/// The trait required to use [`Compress`] on a type. Theoretically, this should apply to all
/// [`Finite`] types, but due to limitations in const generics, a blanket implementation is not
/// currently possible.
///
/// This is automatically implemented on concrete types that derive [`Finite`]. It can also be
/// implemented on a particular concrete type using [`impl_concrete_finite`].
#[allow(clippy::missing_safety_doc)] // Should never be manually implemented.
pub unsafe trait CompressFinite: Finite {
    type Index: Unsigned;
}

impl<T: CompressFinite> Compress<T> {
    /// Constructs a compressed wrapper over the given value.
    pub fn new(value: T) -> Self {
        Compress(T::Index::from_usize_unchecked(T::index_of(value)))
    }

    /// Gets the expanded form of this compressed value.
    pub fn expand(&self) -> T {
        unsafe { T::nth(self.0.to_usize()).unwrap_unchecked() }
    }
}

/// Gets a compressed representation of the given value.
pub fn compress<T: CompressFinite>(value: T) -> Compress<T> {
    Compress::new(value)
}

unsafe impl<T: CompressFinite> Finite for Compress<T> {
    const COUNT: usize = T::COUNT;

    fn index_of(value: Self) -> usize {
        value.0.to_usize()
    }

    fn nth(index: usize) -> Option<Self> {
        if index < Self::COUNT {
            Some(Compress(T::Index::from_usize_unchecked(index)))
        } else {
            None
        }
    }
}

impl<T: CompressFinite> Clone for Compress<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T: CompressFinite> Copy for Compress<T> { }

#[test]
fn test_compress_zst() {
    assert_eq!(core::mem::size_of::<()>(), 0);
}