//! Rust's global allocator replaces `ts_malloc`, `ts_calloc`, `ts_realloc`, and
//! `ts_free`. Use Box/Vec/Arc; allocation failure uses Rust's normal OOM policy.
//! Custom C allocator hooks are deliberately not exposed by this pure Rust port.
pub(crate) use std::boxed::Box;
pub(crate) use std::sync::Arc;
