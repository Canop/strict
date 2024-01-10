use std::{fmt, hash};

/// An ordered set of 1, 2 or 3 elements, allowing pattern matching.
///
/// Implements Copy, Clone, PartialEq, Eq, Debug, etc. if the element type does.
pub enum OneToThree<T> {
    One(T),
    Two(T, T),
    Three(T, T, T),
}

#[allow(clippy::len_without_is_empty)]
impl<T> OneToThree<T> {
    pub fn one(a: T) -> Self {
        Self::One(a)
    }
    pub fn two(a: T, b: T) -> Self {
        Self::Two(a, b)
    }
    pub fn three(a: T, b: T, c: T) -> Self {
        Self::Three(a, b, c)
    }
    pub fn len(&self) -> usize {
        match self {
            Self::One(_) => 1,
            Self::Two(_, _) => 2,
            Self::Three(_, _, _) => 3,
        }
    }
    pub fn iter(&self) -> OneToThreeIter<'_, T> {
        OneToThreeIter::new(self)
    }
    pub fn first(&self) -> &T {
        match self {
            Self::One(f) => f,
            Self::Two(f, _) => f,
            Self::Three(f, _, _) => f,
        }
    }
    pub fn first_mut(&mut self) -> &mut T {
        match self {
            Self::One(ref mut f) => f,
            Self::Two(ref mut f, _) => f,
            Self::Three(ref mut f, _, _) => f,
        }
    }
    pub fn get(&self, i: usize) -> Option<&T> {
        match (i, self) {
            (0, _) => Some(self.first()),
            (1, Self::Two(_, b)) => Some(b),
            (1, Self::Three(_, b, _)) => Some(b),
            (2, Self::Three(_, _, c)) => Some(c),
            _ => None,
        }
    }
    pub fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        match (i, self) {
            (0, Self::One(ref mut a)) => Some(a),
            (0, Self::Two(ref mut a, _)) => Some(a),
            (0, Self::Three(ref mut a, _, _)) => Some(a),
            (1, Self::Two(_, ref mut b)) => Some(b),
            (1, Self::Three(_, ref mut b, _)) => Some(b),
            (2, Self::Three(_, _, ref mut c)) => Some(c),
            _ => None,
        }
    }
    pub fn to_vec(self) -> Vec<T> {
        match self {
            Self::One(a) => vec![a],
            Self::Two(a, b) => vec![a, b],
            Self::Three(a, b, c) => vec![a, b, c],
        }
    }
    pub fn to_ref_vec(&self) -> Vec<&T> {
        match self {
            Self::One(a) => vec![a],
            Self::Two(a, b) => vec![a, b],
            Self::Three(a, b, c) => vec![a, b, c],
        }
    }
    pub fn sorted(self) -> Self
    where
        T: PartialOrd,
    {
        match self {
            Self::One(a) => Self::One(a),
            Self::Two(a, b) => {
                if a < b {
                    Self::Two(a, b)
                } else {
                    Self::Two(b, a)
                }
            }
            Self::Three(a, b, c) => {
                if a < b {
                    if b < c {
                        Self::Three(a, b, c)
                    } else if a < c {
                        Self::Three(a, c, b)
                    } else {
                        Self::Three(c, a, b)
                    }
                } else if a < c {
                    Self::Three(b, a, c)
                } else if b < c {
                    Self::Three(b, c, a)
                } else {
                    Self::Three(c, b, a)
                }
            }
        }
    }
    pub fn map<B, F>(self, f: F) -> OneToThree<B>
    where
        F: Fn(T) -> B,
    {
        match self {
            Self::One(a) => OneToThree::One(f(a)),
            Self::Two(a, b) => OneToThree::Two(f(a), f(b)),
            Self::Three(a, b, c) => OneToThree::Three(f(a), f(b), f(c)),
        }
    }
    pub fn try_map<B, E, F>(self, f: F) -> Result<OneToThree<B>, E>
    where
        F: Fn(T) -> Result<B, E>,
    {
        Ok(match self {
            Self::One(a) => OneToThree::One(f(a)?),
            Self::Two(a, b) => OneToThree::Two(f(a)?, f(b)?),
            Self::Three(a, b, c) => OneToThree::Three(f(a)?, f(b)?, f(c)?),
        })
    }
}

impl<T: Clone + Copy> Clone for OneToThree<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Copy> Copy for OneToThree<T> {}

impl<T: PartialEq> PartialEq for OneToThree<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::One(a), Self::One(b)) => a == b,
            (Self::Two(a, b), Self::Two(c, d)) => a == c && b == d,
            (Self::Three(a, b, c), Self::Three(d, e, f)) => a == d && b == e && c == f,
            _ => false,
        }
    }
}

impl<T: Eq> Eq for OneToThree<T> {}

impl<T: fmt::Debug> fmt::Debug for OneToThree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = self.to_ref_vec();
        f.debug_list().entries(v).finish()
    }
}

impl<T: hash::Hash> hash::Hash for OneToThree<T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        for e in self.iter() {
            e.hash(state);
        }
    }
}

pub struct OneToThreeIter<'i, T> {
    idx: usize,
    few: &'i OneToThree<T>,
}
impl<'i, T> OneToThreeIter<'i, T> {
    pub fn new(few: &'i OneToThree<T>) -> Self {
        Self { idx: 0, few }
    }
}
impl<'i, T> Iterator for OneToThreeIter<'i, T> {
    type Item = &'i T;
    fn next(&mut self) -> Option<Self::Item> {
        let i = self.idx;
        self.idx += 1;
        self.few.get(i)
    }
}

impl<'a, T> IntoIterator for &'a OneToThree<T> {
    type Item = &'a T;
    type IntoIter = OneToThreeIter<'a, T>;
    #[inline]
    fn into_iter(self) -> OneToThreeIter<'a, T> {
        self.iter()
    }
}

impl<T> TryFrom<Vec<T>> for OneToThree<T> {
    type Error = &'static str;
    fn try_from(mut v: Vec<T>) -> Result<Self, Self::Error> {
        let c = v.pop().ok_or("Empty vec")?;
        if let Some(b) = v.pop() {
            if let Some(a) = v.pop() {
                Ok(Self::Three(a, b, c))
            } else {
                Ok(Self::Two(b, c))
            }
        } else {
            Ok(Self::One(c))
        }
    }
}

impl<T> From<T> for OneToThree<T> {
    fn from(a: T) -> Self {
        Self::One(a)
    }
}
impl<T> From<(T, T)> for OneToThree<T> {
    fn from(t: (T, T)) -> Self {
        Self::Two(t.0, t.1)
    }
}
impl<T> From<(T, T, T)> for OneToThree<T> {
    fn from(t: (T, T, T)) -> Self {
        Self::Three(t.0, t.1, t.2)
    }
}

#[test]
fn test_sort() {
    assert_eq!(OneToThree::one(1).sorted(), OneToThree::one(1));

    assert_eq!(OneToThree::two(5, 2).sorted(), OneToThree::two(2, 5));
    assert_eq!(OneToThree::two(1, 2).sorted(), OneToThree::two(1, 2));

    assert_eq!(
        OneToThree::three(1, 2, 1).sorted(),
        OneToThree::three(1, 1, 2)
    );
    assert_eq!(
        OneToThree::three(3, 2, 1).sorted(),
        OneToThree::three(1, 2, 3)
    );
    assert_eq!(
        OneToThree::three(3, 2, 4).sorted(),
        OneToThree::three(2, 3, 4)
    );
    assert_eq!(
        OneToThree::three(1, 2, 3).sorted(),
        OneToThree::three(1, 2, 3)
    );
    assert_eq!(
        OneToThree::three(1, 3, 2).sorted(),
        OneToThree::three(1, 2, 3)
    );
    assert_eq!(
        OneToThree::three(2, 1, 3).sorted(),
        OneToThree::three(1, 2, 3)
    );
    assert_eq!(
        OneToThree::three(3, 1, 2).sorted(),
        OneToThree::three(1, 2, 3)
    );
}

#[test]
fn test_map() {
    assert_eq!(
        OneToThree::three(1, 2, 3).map(|x| x.to_string()),
        OneToThree::three("1".to_string(), "2".to_string(), "3".to_string()),
    );
}

#[test]
fn test_try_map() {
    assert_eq!(
        OneToThree::three("1", "-2", "3")
            .try_map(|x| x.parse())
            .unwrap(),
        OneToThree::three(1, -2, 3),
    );
    assert!(OneToThree::three("1", "-2", "3")
        .try_map::<usize, _, _>(|x| x.parse())
        .is_err());
}
