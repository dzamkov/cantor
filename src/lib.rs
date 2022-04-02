#![feature(const_trait_impl)]
#![feature(const_option)]
extern crate self as cantor;
pub use cantor_macros::*;

/// Provides the number of values for a type, as well as a 1-to-1 mapping between integers and
/// those values. The ordering of integers in this mapping must be homomorphic to the ordering
/// of values according to [`Ord`].
pub trait Finite: Ord + Clone + Sized {
    /// The number of valid values of this type.
    const COUNT: u32;

    /// Gets a unique integer representation for the given value. This defines a 1-to-1 mapping
    /// between values of this type and non-negative integers less than [`Finite::COUNT`].
    fn index_of(value: &Self) -> u32;

    /// Gets the value with the given index as returned by [`Finite::index_of`], or returns
    /// [`None`] if the index is out of bounds.
    fn nth(index: u32) -> Option<Self>;
}

impl const Finite for bool {
    const COUNT: u32 = 2;

    fn index_of(value: &Self) -> u32 {
        *value as u32
    }

    fn nth(index: u32) -> Option<Self> {
        match index {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }
}

impl const Finite for u8 {
    const COUNT: u32 = 1 << 8;

    fn index_of(value: &Self) -> u32 {
        *value as u32
    }

    fn nth(index: u32) -> Option<Self> {
        if index < Self::COUNT {
            Some(index as u8)
        } else {
            None
        }
    }
}

impl const Finite for u16 {
    const COUNT: u32 = 1 << 16;

    fn index_of(value: &Self) -> u32 {
        *value as u32
    }

    fn nth(index: u32) -> Option<Self> {
        if index < Self::COUNT {
            Some(index as u16)
        } else {
            None
        }
    }
}

impl<T: ~const Finite> const Finite for Option<T> {
    const COUNT: u32 = 1 + T::COUNT;

    fn index_of(value: &Self) -> u32 {
        match value {
            Some(value) => 1 + T::index_of(value),
            None => 0
        }
    }

    fn nth(index: u32) -> Option<Self> {
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
    const COUNT: u32 = A::COUNT * B::COUNT;

    fn index_of(value: &Self) -> u32 {
        A::index_of(&value.0) * B::COUNT + B::index_of(&value.1)
    }

    fn nth(index: u32) -> Option<Self> {
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

#[cfg(test)]
mod tests;