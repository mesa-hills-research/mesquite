//! Subtree reference counts use `Arc`, including its acquire/release protocol.
//! Stack arena reference counts are non-atomic and confined to one parser.
pub(crate) use std::sync::atomic::{AtomicUsize, Ordering};
pub(crate) fn atomic_load(value: &AtomicUsize) -> usize {
    value.load(Ordering::Relaxed)
}
