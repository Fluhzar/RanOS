//! # Sparse Vector
//!
//!

#![warn(missing_docs)]

use std::slice::{Iter, IterMut};

/// Type used for the values inside `SparseVec`'s internal `Vec`.
///
/// The first element of the tuple is the index of the sparse vector's element, which is the second element of the tuple.
pub type IndVal<T> = (usize, T);

/// Enum used to represent different values of [`SparseVec`][0]. Mostly used as a return type for some
/// [`SparseVec`][0] functions.
///
/// [0]: ./struct.SparseVec.html
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Element<T> {
    /// This variant represents an element in the sparse vector that has a value.
    Value(T),
    /// This variant represents an element in the sparse vector that doesn't have a value.
    Empty,
}

/// Type used as a return value for [`SparseVec::get_ref`][0].
///
/// [0]: ./struct.SparseVec.html#method.get_ref
pub type SparseVecElement<'a, T> = Element<&'a IndVal<T>>;

/// Type used as a return value for [`SparseVec::get_mut`][0].
///
/// [0]: ./struct.SparseVec.html#method.get_mut
pub type SparseVecElementMut<'a, T> = Element<&'a mut IndVal<T>>;

/// # Sparse Vector
///
/// This data structure is a collection of sparse data (i.e. a lot of empty values between important elements) represented in a
/// condense format.
///
/// Internally this struct uses [`Vec`][0] to hold the elements, granting dense and efficient storage of the sparse data and
/// simplifying the overhead required to write this library.
///
/// # Example
///
/// Imagine, if you will, you have a dataset consisting of the following indices and values:
///
/// | ind | value |
/// |:---:|:-----:|
/// |   0 |    24 |
/// |   7 |    16 |
/// |  18 |   164 |
///
/// Representing this dataset of sparse values would involve something along the lines of:
///
/// ```
/// let mut spvec = SparseVec::new();
///
/// spvec.insert(0, 24);
/// spvec.insert(7, 16);
/// spvec.insert(18, 164);
/// ```
///
/// Which, looking from the internal point of view, would yield a vec with the values of:
///
/// | Vec ind | (Sparse Vec ind, value) |
/// |:-------:|:-----------------------:|
/// |       0 |                 (0, 24) |
/// |       1 |                 (7, 16) |
/// |       2 |               (18, 164) |
///
/// [0]: https://doc.rust-lang.org/std/vec/struct.Vec.html
#[derive(Debug, Default, Clone)]
pub struct SparseVec<T> {
    buf: Vec<IndVal<T>>,
}

impl<T> SparseVec<T> {
    /// Creates a new [`SparseVector`][0] object
    ///
    /// [0]: ./struct.SparseVector.html
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    /// Inserts an element at the selected index.
    ///
    /// # Note
    ///
    /// If a value already exists at the given index then it is overwritten by the inserted value.
    pub fn insert(&mut self, ind: usize, val: T) {
        if self.buf.len() > 0 {
            let mut insert_before = None;

            for (i, v) in self.buf.iter_mut().enumerate() {
                if v.0 == ind {
                    v.1 = val;
                    return;
                }

                if v.0 > ind {
                    insert_before = Some(i);
                    break;
                }
            }

            if let Some(before) = insert_before {
                self.buf.insert(before, (ind, val));
            } else {
                self.buf.push((ind, val));
            }
        } else {
            self.buf.push((ind, val));
        }
    }

    /// Removes a value at the given index, returning the value if it existed.
    pub fn remove(&mut self, ind: usize) -> Option<IndVal<T>> {
        let mut remove = None;
        for (i, v) in self.buf.iter().enumerate() {
            if v.0 == ind {
                remove = Some(i);
                break;
            }
        }

        let out = if let Some(i) = remove {
            Some(self.buf.remove(i))
        } else {
            None
        };

        out
    }

    /// Returns the maximum index of all currently stored sparse data points.
    pub fn max_ind(&self) -> usize {
        if self.buf.len() > 0 {
            self.buf.last().unwrap().0
        } else {
            0
        }
    }

    /// Returns an immutable reference to the element at the given index.
    ///
    /// # Return
    ///
    /// * If the element exists, then Some(Value(element)) is returned.
    /// * If the element doesn't exist, then Some(Empty) is returned.
    pub fn get_ref(&self, ind: usize) -> SparseVecElement<T> {
        if let Some(dat) = self.buf.iter().find(|v| v.0 == ind) {
            SparseVecElement::Value(dat)
        } else {
            SparseVecElement::Empty
        }
    }

    /// Returns a mutable reference to the element at the given index.
    ///
    /// # Return
    ///
    /// * If the element exists, then Some(Value(element)) is returned.
    /// * If the element doesn't exist, then Some(Empty) is returned.
    pub fn get_mut(&mut self, ind: usize) -> SparseVecElementMut<T> {
        if let Some(dat) = self.buf.iter_mut().find(|v| v.0 == ind) {
            SparseVecElementMut::Value(dat)
        } else {
            SparseVecElementMut::Empty
        }
    }

    /// Returns an immutable iterator to the internal buffer of values.
    ///
    /// # Note
    ///
    /// The elements that the returned iterator will iterate over are of type [`IndVal`][0]. Be sure to read its documentation
    /// to know what is actually being iterated over.
    ///
    /// [0]: ./type.IndVal.html
    pub fn iter(&self) -> Iter<IndVal<T>> {
        self.buf.iter()
    }

    /// Returns a mutable iterator to the internal buffer of values.
    ///
    /// # Note
    ///
    /// The elements that the returned iterator will iterate over are of type [`IndVal`][0]. Be sure to read its documentation
    /// to know what is actually being iterated over.
    ///
    /// [0]: ./type.IndVal.html
    pub fn iter_mut(&mut self) -> IterMut<IndVal<T>> {
        self.buf.iter_mut()
    }

    /// Returns a draining iterator to the internal buffer of values.
    /// 
    /// # Note
    /// 
    /// See [`Vec::drain`][0] for more info about draining iterators and how
    /// they're used.
    /// 
    /// [0]: std::vec::Vec::drain
    pub fn drain<R>(&mut self, range: R) -> std::vec::Drain<IndVal<T>>
    where
        R: std::ops::RangeBounds<usize>,
    {
        self.buf.drain(range)
    }

    /// Consumes `self` and returns a tuple of the internal structure of the [`SparseVec`][0] which is just a simple [`Vec`][1].
    ///
    /// [0]: ./struct.SparseVec.html
    /// [1]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    pub fn into_parts(self) -> Vec<IndVal<T>> {
        self.buf
    }
}

impl<T> Into<Vec<T>> for SparseVec<T>
where
    T: Default,
{
    fn into(mut self) -> Vec<T> {
        let mut vec = Vec::new();
        let mut count = 0;
        for (i, e) in self.drain(0..) {
            while count < i {
                vec.push(Default::default());
                count += 1;
            }

            vec.push(e);
        }

        vec
    }
}

#[cfg(test)]
mod sparse_vec_test {
    use super::*;

    #[test]
    fn test_insert() {
        {
            let mut spvec = SparseVec::new();

            spvec.insert(0, 0);

            assert_eq!(spvec.max_ind(), 0); // Test max_ind

            let vec = spvec.into_parts();

            assert_eq!(vec.len(), 1);
            assert_eq!(*vec.first().unwrap(), (0, 0));
        }

        {
            let mut spvec = SparseVec::new();

            spvec.insert(0, 0);
            spvec.insert(10, 11);
            spvec.insert(5, 5);
            spvec.insert(10, 10);

            assert_eq!(spvec.max_ind(), 10); // Test max_ind

            let vec = spvec.into_parts();

            assert_eq!(vec.len(), 3);
            assert_eq!(vec[0], (0, 0));
            assert_eq!(vec[1], (5, 5));
            assert_eq!(vec[2], (10, 10));
        }
    }

    #[test]
    fn test_remove() {
        // Remove only element
        {
            let mut spvec = SparseVec::new();

            spvec.insert(0, 0);
            spvec.remove(0);

            let vec = spvec.into_parts();

            assert_eq!(vec.len(), 0);
        }

        // Remove first element
        {
            let mut spvec = SparseVec::new();

            spvec.insert(0, 0);
            spvec.insert(5, 5);
            spvec.insert(10, 10);

            spvec.remove(0);

            let vec = spvec.into_parts();

            assert_eq!(vec.len(), 2);
            assert_eq!(vec[0], (5, 5));
            assert_eq!(vec[1], (10, 10));
        }

        // Remove middle element
        {
            let mut spvec = SparseVec::new();

            spvec.insert(0, 0);
            spvec.insert(5, 5);
            spvec.insert(10, 10);

            spvec.remove(5);

            let vec = spvec.into_parts();

            assert_eq!(vec.len(), 2);
            assert_eq!(vec[0], (0, 0));
            assert_eq!(vec[1], (10, 10));
        }

        // Remove last element
        {
            let mut spvec = SparseVec::new();

            spvec.insert(0, 0);
            spvec.insert(5, 5);
            spvec.insert(10, 10);

            spvec.remove(10);

            let vec = spvec.into_parts();

            assert_eq!(vec.len(), 2);
            assert_eq!(vec[0], (0, 0));
            assert_eq!(vec[1], (5, 5));
        }

        // Remove non-element
        {
            let mut spvec = SparseVec::new();

            spvec.insert(0, 0);
            spvec.insert(5, 5);
            spvec.insert(10, 10);

            spvec.remove(3);

            let vec = spvec.into_parts();

            assert_eq!(vec.len(), 3);
            assert_eq!(vec[0], (0, 0));
            assert_eq!(vec[1], (5, 5));
            assert_eq!(vec[2], (10, 10));
        }
    }

    #[test]
    fn test_get_ref() {
        // Get element
        {
            let mut spvec = SparseVec::new();

            spvec.insert(0, 0);

            let element = spvec.get_ref(0);

            assert_eq!(element, SparseVecElement::Value(&(0, 0)));
        }

        // Get non-element
        {
            let mut spvec = SparseVec::new();

            spvec.insert(0, 0);

            let element = spvec.get_ref(1);

            assert_eq!(element, SparseVecElement::Empty);
        }
    }

    #[test]
    fn test_get_mut() {
        // Get element
        {
            let mut spvec = SparseVec::new();

            spvec.insert(0, 0);

            let mut element = spvec.get_mut(0);

            assert_eq!(element, SparseVecElementMut::Value(&mut (0, 0)));

            if let SparseVecElementMut::Value(val) = &mut element {
                val.1 = 2;
            }

            assert_eq!(element, SparseVecElementMut::Value(&mut (0, 2)));
        }

        // Get non-element
        {
            let mut spvec = SparseVec::new();

            spvec.insert(0, 0);

            let element = spvec.get_mut(1);

            assert_eq!(element, SparseVecElementMut::Empty);
        }
    }

    #[test]
    fn test_iter() {
        let mut spvec = SparseVec::new();

        spvec.insert(0, 0);
        spvec.insert(5, 5);
        spvec.insert(10, 10);

        let expected = [0, 5, 10];

        for (i, (ind, val)) in spvec.iter().enumerate() {
            assert_eq!(expected[i], *ind);
            assert_eq!(expected[i], *val);
        }
    }

    #[test]
    fn test_iter_mut() {
        let mut spvec = SparseVec::new();

        spvec.insert(0, 0);
        spvec.insert(5, 5);
        spvec.insert(10, 10);

        for (_, val) in spvec.iter_mut() {
            *val += 2;
        }

        let expected = [0, 5, 10];

        for (i, (ind, val)) in spvec.iter().enumerate() {
            assert_eq!(expected[i], *ind);
            assert_eq!(expected[i], *val - 2);
        }
    }
}
