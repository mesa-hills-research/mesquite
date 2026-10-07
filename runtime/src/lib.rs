//! A safe Rust translation of the tree-sitter 0.25.10 runtime.
//! See `runtime/PORTING.md` for the translation contract.
// Temporary skeleton allowances; remove unused allowances as units land.
#![allow(dead_code, unused_imports, unused_variables, unused_macros)]
// Preserve C private/public helper distinctions (e.g. __child versus _child).
#![allow(non_snake_case)]
// Stub bodies cannot yet demonstrate that these output buffers grow.
#![allow(clippy::ptr_arg)]
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
mod length;
mod lexer;
mod node;
mod parser;
mod point;
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
pub use tree::Tree;
pub use tree_cursor::TreeCursor;
pub use ts_port_tables::LanguageMetadata;
