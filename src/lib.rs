mod test;

use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::ops::{self, Index, IndexMut};
use core::slice::{self, SliceIndex};
use std::collections::TryReserveError;
use std::mem::MaybeUninit;
use std::vec::Drain;
use delegate::delegate;
use std::cmp::Ordering;
use std::ops::RangeBounds;

pub trait IndexLike {
    fn to_index(&self) -> usize;
    fn from_index(i: usize) -> Self;
}

#[derive(Eq, PartialEq)]
pub struct KeyedVec<K: IndexLike, T> {
    phantom_for_k: PhantomData<K>, // used for the key
    vec: Vec<T>,
}

// Inherent methods
impl<K: IndexLike, T> KeyedVec<K, T> {
    pub const fn new() -> KeyedVec<K, T> {
        KeyedVec {
            phantom_for_k: PhantomData,
            vec: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> KeyedVec<K, T> {
        KeyedVec {
            phantom_for_k: PhantomData,
            vec: Vec::with_capacity(capacity),
        }
    }

    /// # Safety
    /// See Vec::from_raw_parts
    pub unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> KeyedVec<K, T> {
        KeyedVec {
            phantom_for_k: PhantomData,
            vec: Vec::from_raw_parts(ptr, length, capacity),
        }
    }
}

// Custom methods
impl<K: IndexLike, T> KeyedVec<K, T> {
    // TODO One day I will "overload" the iterator just fot the enumerate method but "today is not that day" (-- Aragorn, King of Gondor and Arnor)
    #[inline]
    pub fn enumerate(&self) -> impl Iterator<Item = (K, &T)> {
        self.vec.iter().enumerate().map(|(i, e)| (K::from_index(i), e))
    }
}

impl<K: IndexLike, T> KeyedVec<K, T> {
    delegate! {
        to self.vec {
            pub fn capacity(&self) -> usize;
            pub fn reserve(&mut self, additional: usize);
            pub fn reserve_exact(&mut self, additional: usize);
            pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError>;
            pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError>;
            pub fn shrink_to_fit(&mut self);
            pub fn shrink_to(&mut self, min_capacity: usize);
            //pub fn into_boxed_slice(mut self) -> Box<[T]>;
            pub fn truncate(&mut self, len: usize);
            pub fn as_slice(&self) -> &[T];
            pub fn as_mut_slice(&mut self) -> &mut [T];
            pub fn as_ptr(&self) -> *const T;
            pub fn as_mut_ptr(&mut self) -> *mut T;
        
            /// # Safety
            /// See Vec::set_len
            pub unsafe fn set_len(&mut self, new_len: usize);
            pub fn swap_remove(&mut self, index: usize) -> T;
            pub fn insert(&mut self, index: usize, element: T);
            pub fn remove(&mut self, index: usize) -> T;
            // TODO retain, retain_mut and dedup_by have a mut in their original declarations but rust say that it's useless here
            pub fn retain<F: FnMut(&T) -> bool>(&mut self, /*mut*/ f: F);
            pub fn retain_mut<F: FnMut(&mut T) -> bool>(&mut self, /*mut*/ f: F);
            pub fn dedup_by<F: FnMut(&mut T, &mut T) -> bool>(&mut self, /*mut*/ same_bucket: F);
            pub fn pop(&mut self) -> Option<T>;
            pub fn drain<R: RangeBounds<usize>>(&mut self, range: R) -> Drain<'_, T>;
            pub fn clear(&mut self);
            pub fn len(&self) -> usize;
            pub fn is_empty(&self) -> bool;
            //pub fn split_off(&mut self, at: usize) -> Self;
            pub fn resize_with<F: FnMut() -> T>(&mut self, new_len: usize, f: F);
            //pub fn leak<'a>(self) -> &'a mut [T];
            pub fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<T>];
        }
    }

    #[inline]
    pub fn append(&mut self, other: &mut Self) {
        self.vec.append(&mut other.vec)
    }

    #[inline]
    pub fn push(&mut self, value: T) -> K {
        self.vec.push(value);
        K::from_index(self.vec.len() - 1)
    }

    #[inline]
    #[must_use]
    pub fn get(&self, index: K) -> Option<&T> {
        self.vec.get(index.to_index())
    }

    #[inline]
    #[must_use]
    pub fn get_mut(&mut self, index: K) -> Option<&mut T> {
        self.vec.get_mut(index.to_index())
    }
}

impl<K: IndexLike, T: Clone> KeyedVec<K, T> {
    delegate! {
        to self.vec {
            pub fn resize(&mut self, new_len: usize, value: T);
            pub fn extend_from_slice(&mut self, other: &[T]);
            pub fn extend_from_within<R: RangeBounds<usize>>(&mut self, src: R);
        }
    }
}

// This code generalizes `extend_with_{element,default}`.
trait ExtendWith<T> {
    fn next(&mut self) -> T;
    fn last(self) -> T;
}
struct ExtendElement<T>(T);
impl<T: Clone> ExtendWith<T> for ExtendElement<T> {
    fn next(&mut self) -> T {
        self.0.clone()
    }
    fn last(self) -> T {
        self.0
    }
}

impl<K: IndexLike, T: PartialEq> KeyedVec<K, T> {
    delegate! {
        to self.vec {
            pub fn dedup(&mut self);
        }
    }
}

impl<K: IndexLike, T> ops::Deref for KeyedVec<K, T> {
    type Target = [T];
    delegate! {
        to self.vec {
            fn deref(&self) -> &[T];
        }
    }
}

impl<K: IndexLike, T: Clone> Clone for KeyedVec<K, T> {
    #[inline]
    fn clone(&self) -> KeyedVec<K, T> {
        KeyedVec {
            phantom_for_k: Default::default(),
            vec: self.vec.clone(),
        }
    }

    #[inline]
    fn clone_from(&mut self, other: &KeyedVec<K, T>) {
        self.vec.clone_from(&other.vec);
    }
}

impl<K: IndexLike, T: Hash> Hash for KeyedVec<K, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&**self, state)
    }
}

impl<K: IndexLike, T, I: SliceIndex<[T]>> Index<I> for KeyedVec<K, T> {
    type Output = I::Output;

    delegate! {
        to self.vec {
            fn index(&self, index: I) -> &Self::Output;
        }
    }
}

impl<K: IndexLike, T, I: SliceIndex<[T]>> IndexMut<I> for KeyedVec<K, T> {
    delegate! {
        to self.vec {
            fn index_mut(&mut self, index: I) -> &mut Self::Output;
        }
    }
}

// TODO don't know how to implement
/*
impl<K: IndexLike, T> FromIterator<T> for KeyedVec<K, T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> KeyedVec<K, T> {
        KeyedVec::from_iter(iter.into_iter())
    }
}
*/

// TODO don't know how to implement
/*
impl<K: IndexLike, T> IntoIterator for KeyedVec<K, T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}
*/

impl<'a, K: IndexLike, T> IntoIterator for &'a KeyedVec<K, T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.vec.iter()
    }
}

impl<'a, K: IndexLike, T> IntoIterator for &'a mut KeyedVec<K, T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.vec.iter_mut()
    }
}

impl<K: IndexLike, T> Extend<T> for KeyedVec<K, T> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.vec.extend(iter)
    }

    //fn extend_one(&mut self, item: T);
    //fn extend_reserve(&mut self, additional: usize);
}

impl<K: IndexLike, T> KeyedVec<K, T> {
    delegate! {
        to self.vec {
            pub fn splice<R: RangeBounds<usize>, I: IntoIterator<Item = T>>(&mut self, range: R, replace_with: I);
            //pub fn drain_filter<F: FnMut(&mut T) -> bool>(&mut self, filter: F);
        }
    }
}
#[cfg(not(no_global_oom_handling))]

impl<'a, K: IndexLike, T: Copy + 'a> Extend<&'a T> for KeyedVec<K, T> {
    delegate! {
        to self.vec {
            fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I);
            //fn extend_one(&mut self, &item: &'a T);
            //fn extend_reserve(&mut self, additional: usize);
        }
    }
}

// TODO K shouldn't have to implement PartialOrd
impl<K: IndexLike + PartialOrd, T: PartialOrd> PartialOrd for KeyedVec<K, T> {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.vec.partial_cmp(&other.vec)
    }
}

// TODO K shouldn't have to implement Ord
impl<K: IndexLike + Ord, T: Ord> Ord for KeyedVec<K, T> {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        self.vec.cmp(&other.vec)
    }
}

// TODO how to implement that?
/*
impl<K: IndexLike, T> Drop for KeyedVec<K, T> {
    fn drop(&mut self)
}
*/

// TODO should be const
impl<K: IndexLike, T> Default for KeyedVec<K, T> {
    fn default() -> KeyedVec<K, T> {
        KeyedVec {
            phantom_for_k: Default::default(),
            vec: Vec::new(),
        }
    }
}

impl<K: IndexLike, T: fmt::Debug> fmt::Debug for KeyedVec<K, T> {
    delegate! {
        to self.vec {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
        }
    }
}

impl<K: IndexLike, T> AsRef<KeyedVec<K, T>> for KeyedVec<K, T> {
    fn as_ref(&self) -> &KeyedVec<K, T> {
        self
    }
}

impl<K: IndexLike, T> AsMut<KeyedVec<K, T>> for KeyedVec<K, T> {
    fn as_mut(&mut self) -> &mut KeyedVec<K, T> {
        self
    }
}

// TODO not sure about this, what does that mean?
/*
impl<K: IndexLike, T> AsRef<[T]> for KeyedVex<K, T> {
    fn as_ref(&self) -> &KeyedVex<K, T> {
        self
    }
}
*/

// TODO not sure about this, what does that mean?
/*
impl<K: IndexLike, T> AsMut<[T]> for KeyedVex<K, T> {
    delegate! {
        to Self {
            fn as_mut(&mut self);
        }
    }
}
*/

impl<K: IndexLike, T: Clone> From<&[T]> for KeyedVec<K, T> {
    #[inline]
    fn from(s: &[T]) -> KeyedVec<K, T> {
        KeyedVec {
            phantom_for_k: Default::default(),
            vec: s.to_vec(),
        }
    }
}

impl<K: IndexLike, T: Clone> From<&mut [T]> for KeyedVec<K, T> {
    #[inline]
    fn from(s: &mut [T]) -> KeyedVec<K, T> {
        KeyedVec {
            phantom_for_k: Default::default(),
            vec: s.to_vec(),
        }
    }
}

impl<K: IndexLike, T, const N: usize> From<[T; N]> for KeyedVec<K, T> {
    #[inline]
    fn from(s: [T; N]) -> KeyedVec<K, T> {
        KeyedVec {
            phantom_for_k: Default::default(),
            vec: Vec::from(s),
        }
    }
}

// TODO not sure how to translate that
/*
impl<'a, K: IndexLike, T> From<Cow<'a, [T]>> for KeyedVex<K, T>
where
    [T]: ToOwned<Owned = KeyedVex<K, T>>,
{
    fn from(s: Cow<'a, [T]>) -> KeyedVex<K, T> {
        KeyedVex {
            phantom_for_k: Default::default(),
            vec: Vec::<T>::from(s),
        }
    }
}
*/

impl<K: IndexLike, T> From<Box<[T]>> for KeyedVec<K, T> {
    #[inline]
    fn from(s: Box<[T]>) -> Self {
        KeyedVec {
            phantom_for_k: Default::default(),
            vec: s.into_vec(),
        }
    }
}

// TODO does it make sense to have this?
/*/
impl<K: IndexLike, T> From<KeyedVex<K, T>> for Box<[T]> {
    fn from(v: KeyedVex<K, T>) -> Self {
        KeyedVex {
            phantom_for_k: Default::default(),
            vec: Box::<[T]>::from(v),
        }
    }
}
*/

// TODO I don't think it makes much sense to have this one
/*
impl<K> From<&str> for KeyedVex<K, u8> {
    delegate! {
        to Self {
            fn from(s: &str);
        }
    }
}
*/

// TODO I don't know how to convert that one
/*
impl<K: IndexLike, T, const N: usize> TryFrom<KeyedVex<K, T>> for [T; N] {
    type Error = KeyedVex<K, T>;
    fn try_from(mut vec: KeyedVex<K, T>) -> Result<[T; N], KeyedVex<K, T>>;
}
*/
