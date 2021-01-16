//! This module contains a single public function that will generate an unique
//! identifier. Thanks to the underlying implementation using the [`uid`]
//! this function is thread-safe crate.

use uid::Id as IdT;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct T(());

type IdGenerator = IdT<T>;

/// Simply generates a unique identifier value.
pub fn generate() -> usize {
    IdGenerator::new().get()
}
