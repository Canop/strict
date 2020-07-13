
use {
    std::{
        convert::TryFrom,
        num::NonZeroUsize,
        ops::{
            Deref,
            Index,
            IndexMut,
        },
        slice,
    },
};

#[derive(Debug, Clone)]
pub struct NotEnoughElementsError;

/// a mostly costless wrapping of a vec, ensuring there's always at least one element.
///
/// Follow the semantics of Vec (differing methods have a different name).
///
#[derive(Debug, Clone)]
pub struct NonEmptyVec<T> {
    vec: Vec<T>,
}

impl<T> NonEmptyVec<T> {

    #[inline]
    pub fn len(&self) -> NonZeroUsize {
        unsafe {
            NonZeroUsize::new_unchecked(self.vec.len())
        }
    }

    #[inline]
    pub fn has_len(&self, len: usize) -> bool {
        self.vec.len() == len
    }

    #[inline]
    pub fn first(&self) -> &T {
        unsafe {
            self.vec.get_unchecked(0)
        }
    }

    #[inline]
    pub fn first_mut(&mut self) -> &mut T {
        unsafe {
            self.vec.get_unchecked_mut(0)
        }
    }

    #[inline]
    pub fn last(&self) -> &T {
        unsafe {
            self.vec.get_unchecked(self.vec.len() - 1)
        }
    }

    #[inline]
    pub fn last_mut(&mut self) -> &mut T {
        let idx = self.vec.len() - 1;
        unsafe {
            self.vec.get_unchecked_mut(idx)
        }
    }

    /// take the first item, discard the rest
    #[inline]
    pub fn take(mut self) -> T {
        self.vec.drain(..).next().unwrap()
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        self.vec.push(value);
    }

    #[inline]
    pub fn insert(&mut self, insertion_idx: usize, value: T) {
        self.vec.insert(insertion_idx, value);
    }

    /// Removes the last element from a vector and returns it, or [`None`] if it
    /// contains only one element
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        if self.vec.len() == 1 {
            None
        } else {
            self.vec.pop()
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.vec
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.vec
    }

    #[inline]
    pub fn remove(&mut self, idx: usize) -> Result<T, NotEnoughElementsError> {
        if self.vec.len() == 1 {
            Err(NotEnoughElementsError)
        } else {
            Ok(self.vec.remove(idx))
        }
    }

    #[inline]
    pub fn swap_remove(&mut self, idx: usize) -> Result<T, NotEnoughElementsError> {
        if self.vec.len() == 1 {
            Err(NotEnoughElementsError)
        } else {
            Ok(self.vec.swap_remove(idx))
        }
    }

}

impl<T> TryFrom<Vec<T>> for NonEmptyVec<T> {
    type Error = NotEnoughElementsError;
    #[inline]
    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        if vec.is_empty() {
            Err(NotEnoughElementsError)
        } else {
            Ok(Self {
                vec,
            })
        }
    }
}

impl<T> From<T> for NonEmptyVec<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self {
            vec: vec![value],
        }
    }
}

impl<T> Deref for NonEmptyVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        self.vec.deref()
    }
}

impl<T, I: slice::SliceIndex<[T]>> Index<I> for NonEmptyVec<T> {
    type Output = I::Output;
    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        Index::index(self.as_slice(), index)
    }
}

impl<T, I: slice::SliceIndex<[T]>> IndexMut<I> for NonEmptyVec<T> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(self.as_mut_slice(), index)
    }
}

impl<'a, T> IntoIterator for &'a mut NonEmptyVec<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;
    #[inline]
    fn into_iter(self) -> slice::IterMut<'a, T> {
        self.vec.iter_mut()
    }
}

impl<'a, T> IntoIterator for &'a NonEmptyVec<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;
    #[inline]
    fn into_iter(self) -> slice::Iter<'a, T> {
        self.vec.iter()
    }
}

#[cfg(test)]
mod non_empty_vec_tests {

    use {
        super::*,
        std::convert::TryInto,
    };

    #[test]
    fn test_pop_push() {
        let mut vec: NonEmptyVec<usize> = vec![1, 2].try_into().unwrap();
        assert_eq!(vec.pop(), Some(2));
        assert_eq!(vec.pop(), None);
        assert_eq!(vec[0], 1);
        vec[0] = 0;
        assert_eq!(*vec.first(), 0);
        let first: &mut usize = vec.first_mut();
        *first = 4;
        assert_eq!(vec[0], 4);
    }
}

