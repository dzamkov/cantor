use core::ops::{BitAnd, BitOr, Not};

/// A compact, generic unsigned integer with at least the given number of bits.
pub type Uint<const N: usize> = <NumBits<'static, N> as HasUint>::Uint;

/// A type that implements [`HasUint`] for all sizes that have an [`Unsigned`] implementation.
/// 
/// This has a lifetime parameter in order to work around issues with
/// [trivial constraints](https://github.com/rust-lang/rust/issues/48214).
pub struct NumBits<'a, const N: usize>(&'a ());

/// Defines the [`Uint`] backing type for a certain number of bits.
pub trait HasUint {
    type Uint: Unsigned;
}

/// Encapsulates the required operations for unsigned integers required by this crate.
pub trait Unsigned:
    Ord + Clone + Copy + BitOr<Self, Output = Self> + BitAnd<Self, Output = Self> + Not<Output = Self>
{
    const ZERO: Self;
    fn from_usize_unchecked(source: usize) -> Self;
    fn to_usize(self) -> usize;
    fn ones(n: usize) -> Self;
    fn one_at(i: usize) -> Self;
    fn count_ones(self) -> usize;
    fn first_one(self) -> Option<usize>;
    fn last_one(self) -> Option<usize>;
}

/// A zero-sized type that implements [`Unsigned`].
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct u0;

impl BitOr<u0> for u0 {
    type Output = u0;
    fn bitor(self, _: u0) -> Self::Output {
        u0
    }
}

impl BitAnd<u0> for u0 {
    type Output = u0;
    fn bitand(self, _: u0) -> Self::Output {
        u0
    }
}

impl Not for u0 {
    type Output = u0;
    fn not(self) -> Self::Output {
        u0
    }
}

impl Unsigned for u0 {
    const ZERO: Self = u0;

    fn from_usize_unchecked(_: usize) -> Self {
        u0
    }

    fn to_usize(self) -> usize {
        0
    }

    fn ones(_: usize) -> Self {
        u0
    }

    fn one_at(_: usize) -> Self {
        u0
    }

    fn count_ones(self) -> usize {
        0
    }

    fn first_one(self) -> Option<usize> {
        None
    }

    fn last_one(self) -> Option<usize> {
        None
    }
}

macro_rules! impl_unsigned {
    ($t:ty) => {
        impl Unsigned for $t {
            const ZERO: Self = 0;

            fn from_usize_unchecked(source: usize) -> Self {
                source as $t
            }

            fn to_usize(self) -> usize {
                self as usize
            }

            fn ones(n: usize) -> Self {
                (1 << n) - 1
            }

            fn one_at(i: usize) -> Self {
                1 << i
            }

            fn count_ones(self) -> usize {
                Self::count_ones(self) as usize
            }

            fn first_one(self) -> Option<usize> {
                let res = self.trailing_zeros();
                if res < Self::BITS {
                    Some(res as usize)
                } else {
                    None
                }
            }

            fn last_one(self) -> Option<usize> {
                let res = self.leading_zeros();
                if res < Self::BITS {
                    Some((Self::BITS - res - 1) as usize)
                } else {
                    None
                }
            }
        }
    };
}

impl_unsigned!(u8);
impl_unsigned!(u16);
impl_unsigned!(u32);
impl_unsigned!(u64);
impl_unsigned!(u128);

/// Computes the log-base-2 of an integer, rounding up if necessary.
pub const fn log2(n: usize) -> usize {
    // TODO: Replace once int_log is stablized (https://github.com/rust-lang/rust/issues/70887)
    (64 - n.leading_zeros()) as usize
}

macro_rules! impl_uint_for {
    ($n:expr, $uint:ty) => {
        impl<'a> HasUint for NumBits<'a, $n> {
            type Uint = $uint;
        }
    };
}

impl_uint_for!(0, u0);
impl_uint_for!(1, u8);
impl_uint_for!(2, u8);
impl_uint_for!(3, u8);
impl_uint_for!(4, u8);
impl_uint_for!(5, u8);
impl_uint_for!(6, u8);
impl_uint_for!(7, u8);
impl_uint_for!(8, u8);
impl_uint_for!(9, u16);
impl_uint_for!(10, u16);
impl_uint_for!(11, u16);
impl_uint_for!(12, u16);
impl_uint_for!(13, u16);
impl_uint_for!(14, u16);
impl_uint_for!(15, u16);
impl_uint_for!(16, u16);
impl_uint_for!(17, u32);
impl_uint_for!(18, u32);
impl_uint_for!(19, u32);
impl_uint_for!(20, u32);
impl_uint_for!(21, u32);
impl_uint_for!(22, u32);
impl_uint_for!(23, u32);
impl_uint_for!(24, u32);
impl_uint_for!(25, u32);
impl_uint_for!(26, u32);
impl_uint_for!(27, u32);
impl_uint_for!(28, u32);
impl_uint_for!(29, u32);
impl_uint_for!(30, u32);
impl_uint_for!(31, u32);
impl_uint_for!(32, u32);
impl_uint_for!(33, u64);
impl_uint_for!(34, u64);
impl_uint_for!(35, u64);
impl_uint_for!(36, u64);
impl_uint_for!(37, u64);
impl_uint_for!(38, u64);
impl_uint_for!(39, u64);
impl_uint_for!(40, u64);
impl_uint_for!(41, u64);
impl_uint_for!(42, u64);
impl_uint_for!(43, u64);
impl_uint_for!(44, u64);
impl_uint_for!(45, u64);
impl_uint_for!(46, u64);
impl_uint_for!(47, u64);
impl_uint_for!(48, u64);
impl_uint_for!(49, u64);
impl_uint_for!(50, u64);
impl_uint_for!(51, u64);
impl_uint_for!(52, u64);
impl_uint_for!(53, u64);
impl_uint_for!(54, u64);
impl_uint_for!(55, u64);
impl_uint_for!(56, u64);
impl_uint_for!(57, u64);
impl_uint_for!(58, u64);
impl_uint_for!(59, u64);
impl_uint_for!(60, u64);
impl_uint_for!(61, u64);
impl_uint_for!(62, u64);
impl_uint_for!(63, u64);
impl_uint_for!(64, u64);
impl_uint_for!(65, u128);
impl_uint_for!(66, u128);
impl_uint_for!(67, u128);
impl_uint_for!(68, u128);
impl_uint_for!(69, u128);
impl_uint_for!(70, u128);
impl_uint_for!(71, u128);
impl_uint_for!(72, u128);
impl_uint_for!(73, u128);
impl_uint_for!(74, u128);
impl_uint_for!(75, u128);
impl_uint_for!(76, u128);
impl_uint_for!(77, u128);
impl_uint_for!(78, u128);
impl_uint_for!(79, u128);
impl_uint_for!(80, u128);
impl_uint_for!(81, u128);
impl_uint_for!(82, u128);
impl_uint_for!(83, u128);
impl_uint_for!(84, u128);
impl_uint_for!(85, u128);
impl_uint_for!(86, u128);
impl_uint_for!(87, u128);
impl_uint_for!(88, u128);
impl_uint_for!(89, u128);
impl_uint_for!(90, u128);
impl_uint_for!(91, u128);
impl_uint_for!(92, u128);
impl_uint_for!(93, u128);
impl_uint_for!(94, u128);
impl_uint_for!(95, u128);
impl_uint_for!(96, u128);
impl_uint_for!(97, u128);
impl_uint_for!(98, u128);
impl_uint_for!(99, u128);
impl_uint_for!(100, u128);
impl_uint_for!(101, u128);
impl_uint_for!(102, u128);
impl_uint_for!(103, u128);
impl_uint_for!(104, u128);
impl_uint_for!(105, u128);
impl_uint_for!(106, u128);
impl_uint_for!(107, u128);
impl_uint_for!(108, u128);
impl_uint_for!(109, u128);
impl_uint_for!(110, u128);
impl_uint_for!(111, u128);
impl_uint_for!(112, u128);
impl_uint_for!(113, u128);
impl_uint_for!(114, u128);
impl_uint_for!(115, u128);
impl_uint_for!(116, u128);
impl_uint_for!(117, u128);
impl_uint_for!(118, u128);
impl_uint_for!(119, u128);
impl_uint_for!(120, u128);
impl_uint_for!(121, u128);
impl_uint_for!(122, u128);
impl_uint_for!(123, u128);
impl_uint_for!(124, u128);
impl_uint_for!(125, u128);
impl_uint_for!(126, u128);
impl_uint_for!(127, u128);
impl_uint_for!(128, u128);
