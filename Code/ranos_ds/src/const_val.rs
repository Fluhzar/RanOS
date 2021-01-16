//! A wrapper around a value that shouldn't be changed.

/// This struct represents a value that is immutable, and cannot be changed
/// throughout its lifetime.
///
/// This type is more of a syntactic representation of intent rather than
/// forcing you to not change a value, as you could easily perform an operation
/// such as:
///
/// ```
/// # use ranos_ds::const_val::ConstVal;
/// let mut a = ConstVal::new(6);
/// a = ConstVal::new(a.unwrap() + 3);
/// ```
///
/// Thus there are no true protections against modification of the value, just
/// a way to show intent that a value won't be changed even in a context where
/// the owner is mutable.
#[derive(Debug)]
pub struct ConstVal<T> {
    val: T,
}

impl<T> ConstVal<T> {
    /// Creates a new [`ConstVal`] object, wrapping the provided value.
    pub fn new(val: T) -> Self {
        Self { val }
    }

    /// Returns an immutable reference to the wrapped value.
    pub fn get(&self) -> &T {
        &self.val
    }

    /// Unwraps the object into the inner value.
    pub fn unwrap(self) -> T {
        self.val
    }
}

impl<T> Clone for ConstVal<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self::new(self.val.clone())
    }
}

impl<T> From<T> for ConstVal<T> {
    fn from(v: T) -> Self {
        ConstVal::new(v)
    }
}
