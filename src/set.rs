use crate::uint::Unsigned;
use crate::*;

/// A set of values of type `T`, implemented using a bitmap.
///
/// # Example
///
/// ```
/// use cantor::{Finite, Set, BitmapSet};
/// 
/// #[derive(Finite, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
/// enum MyType {
///     A,
///     B(bool),
///     C(bool, bool)
/// }
///
/// let mut set = BitmapSet::none();
/// set.include(MyType::A);
/// set.include(MyType::B(false));
/// set.include(MyType::C(true, false));
/// assert_eq!(set.size(), 3);
/// assert!(set.contains(MyType::B(false)));
/// assert!(!set.contains(MyType::C(false, true)));
/// ```
pub struct BitmapSet<T: BitmapFinite>(T::Bitmap);

/// The trait required to use [`BitmapSet`] with a type.
///
/// This is automatically implemented on concrete types that derive [`Finite`]. It can also be
/// implemented on a particular concrete type using [`impl_concrete_finite`].
#[doc(hidden)]
#[allow(clippy::missing_safety_doc)] // Should never be manually implemented.
pub unsafe trait BitmapFinite: Finite {
    #[allow(missing_docs)]
    type Bitmap: Unsigned;
}

impl<T: BitmapFinite> BitmapSet<T> {
    /// Constructs a new [`BitmapSet`] with initial membership determined using the given function.
    /// 
    /// # Example
    /// ```
    /// use cantor::{Finite, Set, BitmapSet};
    /// let set = BitmapSet::new(|x: bool| !x);
    /// assert_eq!(set.size(), 1);
    /// assert!(set.contains(false));
    /// ```
    pub fn new(mut f: impl FnMut(T) -> bool) -> Self {
        let mut bitmap = T::Bitmap::ZERO;
        for i in 0..T::COUNT {
            if f(unsafe { T::nth(i).unwrap_unchecked() }) {
                bitmap = bitmap | T::Bitmap::one_at(i);
            }
        }
        Self(bitmap)
    }

    /// The set of all possible values of `T`.
    pub fn all() -> Self {
        BitmapSet(T::Bitmap::ones(T::COUNT))
    }

    /// The empty set.
    pub fn none() -> Self {
        BitmapSet(T::Bitmap::ZERO)
    }

    /// The set consisting of only the given value.
    pub fn only(value: T) -> Self {
        BitmapSet(T::Bitmap::one_at(T::index_of(value)))
    }

    /// The number of values in this set.
    pub fn size(&self) -> usize {
        T::Bitmap::count_ones(self.0)
    }
}

impl<T: BitmapFinite> Default for BitmapSet<T> {
    fn default() -> Self {
        Self::none()
    }
}

/// A set of values of type `T`.
pub trait Set<T> {
    /// Determines whether the set contains the given value.
    fn contains(&self, value: T) -> bool;

    /// Ensures that the set includes the given value.
    fn include(&mut self, value: T);

    /// Ensures that the set excludes the given value.
    fn exclude(&mut self, value: T);
}

impl<T: BitmapFinite> Set<T> for BitmapSet<T> {
    fn contains(&self, value: T) -> bool {
        self.0 & T::Bitmap::one_at(T::index_of(value)) != T::Bitmap::ZERO
    }

    fn include(&mut self, value: T) {
        self.0 = self.0 | T::Bitmap::one_at(T::index_of(value));
    }

    fn exclude(&mut self, value: T) {
        self.0 = self.0 & !T::Bitmap::one_at(T::index_of(value));
    }
}

impl<T: CompressFinite + BitmapFinite> Set<Compress<T>> for BitmapSet<T> {
    fn contains(&self, value: Compress<T>) -> bool {
        self.0 & T::Bitmap::one_at(Compress::index_of(value)) != T::Bitmap::ZERO
    }

    fn include(&mut self, value: Compress<T>) {
        self.0 = self.0 | T::Bitmap::one_at(Compress::index_of(value));
    }

    fn exclude(&mut self, value: Compress<T>) {
        self.0 = self.0 & !T::Bitmap::one_at(Compress::index_of(value));
    }
}

impl<T: BitmapFinite> Iterator for BitmapSet<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.0.first_one() {
            self.0 = self.0 & !T::Bitmap::one_at(index);
            Some(unsafe { T::nth(index).unwrap_unchecked() })
        } else {
            None
        }
    }
}

impl<T: BitmapFinite> DoubleEndedIterator for BitmapSet<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.0.last_one() {
            self.0 = self.0 & !T::Bitmap::one_at(index);
            Some(unsafe { T::nth(index).unwrap_unchecked() })
        } else {
            None
        }
    }
}

impl<T: BitmapFinite> Clone for BitmapSet<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T: BitmapFinite> Copy for BitmapSet<T> {}

impl<T: core::fmt::Debug + BitmapFinite> core::fmt::Debug for BitmapSet<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_set().entries(*self).finish()
    }
}

#[test]
fn test_debug() {
    extern crate alloc;
    let mut set = BitmapSet::none();
    set.include(false);
    set.include(true);
    assert_eq!(alloc::format!("{:?}", set), "{false, true}");
}
