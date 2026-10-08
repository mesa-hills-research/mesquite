//! A safe Rust port of the tree-sitter 0.25.10 runtime.
//!
//! Most modules mirror the C source file of the same name (`parser.c` is `parser`)
//! and keep C's function names (`ts_parser_parse`), so the two read side by side.
//! `api` and `query_api` implement tree-sitter's Rust binding on top of them.
#![forbid(unsafe_code)]
// The port keeps C functions that the Rust API doesn't call, such as the
// `*_delete` functions and the logging and DOT-graph output.
#![allow(dead_code, unused_imports, unused_variables, unused_macros)]
// Preserve C private/public helper distinctions (e.g. __child versus _child).
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]

mod alloc;
mod api;
mod array;
mod atomic;
mod clock;
mod error_costs;
mod get_changed_ranges;
mod host;
mod language;
mod language_fn;
mod length;
mod lexer;
mod node;
mod parser;
mod point;
mod query;
mod query_api;
mod reduce_action;
mod reusable_node;
mod stack;
mod subtree;
mod tree;
mod tree_cursor;
mod ts_assert;
mod types;
mod unicode;

pub use api::*;
pub use language::Language;
pub use node::Node;
pub use parser::Parser;
pub use query_api::*;
pub use streaming_iterator::{StreamingIterator, StreamingIteratorMut};
pub use tree::Tree;
pub use tree_cursor::TreeCursor;
pub use tree_sitter_language::LanguageMetadata;
