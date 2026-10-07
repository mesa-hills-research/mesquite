//! `Language::new` and `From<LanguageFn> for Language`: a [`Language`] from a grammar
//! crate's `LANGUAGE` constant, as in tree-sitter's Rust binding.

use crate::Language;
use tree_sitter_language::LanguageFn;

impl Language {
    /// The grammar of a grammar crate's `LANGUAGE` constant, e.g.
    /// `Language::new(tree_sitter_rust::LANGUAGE)`.
    #[must_use]
    pub fn new(builder: LanguageFn) -> Self {
        Self::from(builder.tables())
    }
}

impl From<LanguageFn> for Language {
    fn from(value: LanguageFn) -> Self {
        Self::new(value)
    }
}
