//! `Array(T)` is `Vec<T>`. Use slices for borrowed arrays and `mem::take` for
//! ownership transfer. Capacity and length are never conflated.
pub(crate) type Array<T> = Vec<T>;
