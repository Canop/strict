use std::{
    num::NonZeroUsize,
    ops::{Deref, Index},
    slice,
};

use crate::NotEnoughElementsError;

/// a mostly costless wrapping of a slice, ensuring there's always at least one element.
///
/// Follow the semantics of slice (differing methods have a different name).
///
#[derive(Debug)]
pub struct NonEmptySlice<'a, T> {
    slice: &'a [T],
}

impl<'a, T> NonEmptySlice<'a, T> {
    #[inline]
    pub const fn len(&self) -> NonZeroUsize {
        unsafe { NonZeroUsize::new_unchecked(self.slice.len()) }
    }

    #[inline]
    pub fn has_len(&self, len: usize) -> bool {
        self.slice.len() == len
    }

    #[inline]
    pub fn first(&self) -> &T {
        unsafe { self.slice.get_unchecked(0) }
    }

    #[inline]
    pub fn last(&self) -> &T {
        unsafe { self.slice.get_unchecked(self.slice.len() - 1) }
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.slice
    }
}

impl<'a, T> TryFrom<&'a [T]> for NonEmptySlice<'a, T> {
    type Error = NotEnoughElementsError;

    #[inline]
    fn try_from(slice: &'a [T]) -> Result<Self, Self::Error> {
        if slice.is_empty() {
            Err(NotEnoughElementsError)
        } else {
            Ok(Self { slice })
        }
    }
}

impl<'a, T> From<&'a T> for NonEmptySlice<'a, T> {
    #[inline]
    fn from(value: &'a T) -> Self {
        Self {
            slice: slice::from_ref(value),
        }
    }
}

impl<'a, T> Deref for NonEmptySlice<'a, T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        self.slice.deref()
    }
}

impl<'a, T, I: slice::SliceIndex<[T]>> Index<I> for NonEmptySlice<'a, T> {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        Index::index(self.as_slice(), index)
    }
}

impl<'a, T> IntoIterator for &'a NonEmptySlice<'a, T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;
    #[inline]
    fn into_iter(self) -> slice::Iter<'a, T> {
        self.slice.iter()
    }
}

#[cfg(test)]
mod non_empty_slice_tests {}
