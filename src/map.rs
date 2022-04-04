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
pub unsafe trait ArrayFinite<V>: Finite {
    type Array: Array<V>;
}

impl<K: ArrayFinite<V>, V> ArrayMap<K, V> {
    pub fn new(mut f: impl FnMut(K) -> V) -> Self {
        ArrayMap(K::Array::new(|k| {
            f(unsafe { K::nth(k).unwrap_unchecked() })
        }))
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
        let index = K::index_of(&index);
        unsafe { self.0.as_slice().get_unchecked(index) }
    }
}

impl<K: ArrayFinite<V>, V> IndexMut<K> for ArrayMap<K, V> {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        let index = K::index_of(&index);
        unsafe { self.0.as_slice_mut().get_unchecked_mut(index) }
    }
}

impl<K: CompressFinite + ArrayFinite<V>, V> Index<Compress<K>> for ArrayMap<K, V> {
    type Output = V;
    fn index(&self, index: Compress<K>) -> &Self::Output {
        let index = Compress::index_of(&index);
        unsafe { self.0.as_slice().get_unchecked(index) }
    }
}

impl<K: CompressFinite + ArrayFinite<V>, V> IndexMut<Compress<K>> for ArrayMap<K, V> {
    fn index_mut(&mut self, index: Compress<K>) -> &mut Self::Output {
        let index = Compress::index_of(&index);
        unsafe { self.0.as_slice_mut().get_unchecked_mut(index) }
    }
}