#![no_std]
#![feature(const_trait_impl)]
#![feature(const_option)]
extern crate self as cantor;
pub mod uint;
mod compress;

pub use cantor_macros::*;
pub use compress::*;

/// Provides the number of values for a type, as well as a 1-to-1 mapping between the subset of
/// integers [0 .. N) and those values. The ordering of integers in this mapping is homomorphic to
/// the ordering of values according to [`Ord`] (i.e. `T::index_of(a) < T::index_of(b)` iff
/// `a < b`).
/// 
/// This trait may be automatically derived.
/// 
/// # Example
/// ```
/// #![feature(const_trait_impl)]
/// #![feature(const_option)]
/// use cantor::*;
/// 
/// #[derive(Finite, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
/// enum MyType {
///     A,
///     B(bool),
///     C(bool, bool)
/// }
/// 
/// assert_eq!(MyType::COUNT, 7);
/// assert_eq!(MyType::index_of(&MyType::B(false)), 1);
/// assert_eq!(MyType::nth(4), Some(MyType::C(false, true)));
/// ```
pub trait Finite: Ord + Clone + Sized {
    /// The number of valid values of this type.
    const COUNT: usize;

    /// Gets a unique integer representation for the given value. This defines a 1-to-1 mapping
    /// between values of this type and non-negative integers less than [`Finite::COUNT`].
    fn index_of(value: &Self) -> usize;

    /// Gets the value with the given index as returned by [`Finite::index_of`], or returns
    /// [`None`] if the index is out of bounds.
    fn nth(index: usize) -> Option<Self>;
}

impl const Finite for () {
    const COUNT: usize = 1;

    fn index_of(_: &Self) -> usize {
        0
    }

    fn nth(index: usize) -> Option<Self> {
        if index == 0 {
            Some(())
        } else {
            None
        }
    }
}

impl const Finite for bool {
    const COUNT: usize = 2;

    fn index_of(value: &Self) -> usize {
        *value as usize
    }

    fn nth(index: usize) -> Option<Self> {
        match index {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }
}

impl const Finite for u8 {
    const COUNT: usize = 1 << 8;

    fn index_of(value: &Self) -> usize {
        *value as usize
    }

    fn nth(index: usize) -> Option<Self> {
        if index < Self::COUNT {
            Some(index as u8)
        } else {
            None
        }
    }
}

impl const Finite for u16 {
    const COUNT: usize = 1 << 16;

    fn index_of(value: &Self) -> usize {
        *value as usize
    }

    fn nth(index: usize) -> Option<Self> {
        if index < Self::COUNT {
            Some(index as u16)
        } else {
            None
        }
    }
}

impl<T: ~const Finite> const Finite for Option<T> {
    const COUNT: usize = 1 + T::COUNT;

    fn index_of(value: &Self) -> usize {
        match value {
            Some(value) => 1 + T::index_of(value),
            None => 0
        }
    }

    fn nth(index: usize) -> Option<Self> {
        if index == 0 {
            Some(None)
        } else if index < Self::COUNT {
            Some(T::nth(index - 1))
        } else {
            None
        }
    }
}

impl<A: ~const Finite, B: ~const Finite> const Finite for (A, B) {
    const COUNT: usize = A::COUNT * B::COUNT;

    fn index_of(value: &Self) -> usize {
        A::index_of(&value.0) * B::COUNT + B::index_of(&value.1)
    }

    fn nth(index: usize) -> Option<Self> {
        if index < Self::COUNT {
            Some((
                A::nth(index / B::COUNT).unwrap(),
                B::nth(index % B::COUNT).unwrap(),
            ))
        } else {
            None
        }
    }
}

/// Implements helper traits for a concrete (i.e. non-parameteric) type that implements `Finite`.
#[macro_export]
macro_rules! impl_concrete_finite {
    ($t:ty) => {
        unsafe impl ::cantor::CompressFinite for $t {
            type Index = ::cantor::uint::Uint<{ ::cantor::uint::log2(<$t as Finite>::COUNT - 1) }>;
        }
    };
}

impl_concrete_finite!(());
impl_concrete_finite!(bool);
impl_concrete_finite!(u8);
impl_concrete_finite!(u16);

#[cfg(test)]
mod tests;