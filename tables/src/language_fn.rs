//! [`LanguageFn`]: how a grammar crate exports its grammar (`LANGUAGE`).

use crate::LanguageTables;

/// A grammar crate's grammar, the counterpart of tree-sitter-language's `LanguageFn`.
/// Grammar crates export one as `LANGUAGE`, and the runtime makes a `Language` of it
/// with `Language::new(LANGUAGE)` or `LANGUAGE.into()`.
///
/// It wraps a safe function returning the grammar's tables (decoded on first use),
/// where tree-sitter's wraps a C function returning a raw pointer.
#[derive(Clone, Copy)]
pub struct LanguageFn(fn() -> &'static LanguageTables);

impl LanguageFn {
    /// Wraps a function that returns a grammar's tables.
    pub const fn from_fn(f: fn() -> &'static LanguageTables) -> Self {
        Self(f)
    }

    /// The wrapped function.
    #[must_use]
    pub const fn into_fn(self) -> fn() -> &'static LanguageTables {
        self.0
    }

    /// The grammar's tables (calls the wrapped function).
    #[must_use]
    pub fn tables(self) -> &'static LanguageTables {
        (self.0)()
    }
}
