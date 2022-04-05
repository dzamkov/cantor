use crate::array::Array;
use crate::*;
use core::ops::{Index, IndexMut};

/// A complete mapping from keys of type `K` to values of type `V`, implemented using an array
/// indexed by [`Finite::index_of`] of the key.
///
/// # Example
/// ```
/// use cantor::{Finite, ArrayMap};
///
/// // Define key type
/// #[derive(Finite, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
/// enum MyType {
///     A,
///     B(bool),
///     C(bool, bool)
/// };
///
/// // Initialize map
/// let mut map = ArrayMap::new(|x: MyType| match x {
///     MyType::A => false,
///     MyType::B(a) => a,
///     MyType::C(a, _) => a,
/// });
///
/// // Use map
/// map[MyType::C(true, true)] = false;
/// assert_eq!(map[MyType::A], false);
/// assert_eq!(map[MyType::B(true)], true);
/// assert_eq!(map[MyType::C(true, true)], false);
/// ```
pub struct ArrayMap<K: ArrayFinite<V>, V>(K::Array);

/// The trait required to use [`ArrayMap`]. Theoretically, this should apply to all
/// [`Finite`] types, but due to limitations in const generics, a blanket implementation is not
/// currently possible.
///
/// This is automatically implemented on concrete types that derive [`Finite`]. It can also be
/// implemented on a particular concrete type using [`impl_concrete_finite`].
#[doc(hidden)]
#[allow(clippy::missing_safety_doc)] // Should never be manually implemented.
pub unsafe trait ArrayFinite<V>: Finite {
    #[allow(missing_docs)]
    type Array: Array<V>;
}

impl<K: ArrayFinite<V>, V> ArrayMap<K, V> {
    /// Constructs a new [`ArrayMap`] with initial values populated using the given function.
    pub fn new(mut f: impl FnMut(K) -> V) -> Self {
        ArrayMap(K::Array::new(|k| {
            f(unsafe { K::nth(k).unwrap_unchecked() })
        }))
    }

    /// Constructs a new [`ArrayMap`] from an array of values, each corresponding to the key
    /// determined by [`Finite::nth`].
    /// 
    /// # Example
    /// ```
    /// use cantor::*;
    /// let map = ArrayMap::from([1, 3]);
    /// assert_eq!(map[false], 1);
    /// assert_eq!(map[true], 3);
    /// ```
    pub fn from(array: K::Array) -> Self {
        Self(array)
    }

    /// Applies a mapping function the values of this map.
    pub fn map_with_key<N>(&self, mut f: impl FnMut(K, &V) -> N) -> ArrayMap<K, N>
    where
        K: ArrayFinite<N>,
    {
        ArrayMap(<K as ArrayFinite<N>>::Array::new(|k| unsafe {
            f(
                K::nth(k).unwrap_unchecked(),
                self.0.as_slice().get_unchecked(k),
            )
        }))
    }

    /// Applies a mapping function the values of this map.
    pub fn map<N>(&self, mut f: impl FnMut(&V) -> N) -> ArrayMap<K, N>
    where
        K: ArrayFinite<N>,
    {
        self.map_with_key(|_, v| f(v))
    }
}

impl<K: ArrayFinite<V>, V: Default> Default for ArrayMap<K, V> {
    fn default() -> Self {
        ArrayMap(K::Array::new(|_| Default::default()))
    }
}

impl<K: ArrayFinite<V>, V> Index<K> for ArrayMap<K, V> {
    type Output = V;
    fn index(&self, index: K) -> &Self::Output {
        let index = K::index_of(index);
        unsafe { self.0.as_slice().get_unchecked(index) }
    }
}

impl<K: ArrayFinite<V>, V> IndexMut<K> for ArrayMap<K, V> {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        let index = K::index_of(index);
        unsafe { self.0.as_slice_mut().get_unchecked_mut(index) }
    }
}

impl<K: CompressFinite + ArrayFinite<V>, V> Index<Compress<K>> for ArrayMap<K, V> {
    type Output = V;
    fn index(&self, index: Compress<K>) -> &Self::Output {
        let index = Compress::index_of(index);
        unsafe { self.0.as_slice().get_unchecked(index) }
    }
}

impl<K: CompressFinite + ArrayFinite<V>, V> IndexMut<Compress<K>> for ArrayMap<K, V> {
    fn index_mut(&mut self, index: Compress<K>) -> &mut Self::Output {
        let index = Compress::index_of(index);
        unsafe { self.0.as_slice_mut().get_unchecked_mut(index) }
    }
}

#[test]
fn test_map_with_key() {
    let map = ArrayMap::new(|x| if x { 1 } else { 0 });
    let map = map.map_with_key(|k, v| if k { *v * 2 } else { *v + 5 });
    assert_eq!(map[false], 5);
    assert_eq!(map[true], 2);
}
