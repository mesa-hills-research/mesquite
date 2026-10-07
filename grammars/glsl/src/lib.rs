//! The `glsl` tree-sitter grammar for the ported runtime: its tables (decoded from
//! `tables.bin`) and lexer (`lex.rs`), converted from the grammar's `src/parser.c`.
//!
//! Upstream: tree-sitter-glsl 0.2.0
//! (<https://github.com/tree-sitter-grammars/tree-sitter-glsl>, tag `v0.2.0`). Generated:
//! do not edit by hand.
#![forbid(unsafe_code)]

mod lex;

use std::sync::LazyLock;

use tree_sitter_language::{LanguageFn, LanguageTables, LexFn, ScannerCreateFn};

/// The grammar's tables, written by the converter from the compiled C grammar.
static BLOB: &[u8] = include_bytes!("tables.bin");

static TABLES: LazyLock<LanguageTables> = LazyLock::new(decode);

/// The grammar, for `tree_sitter::Language::new(LANGUAGE_GLSL)` or `LANGUAGE_GLSL.into()`,
/// as with tree-sitter-glsl's `tree_sitter_glsl::LANGUAGE_GLSL`.
pub const LANGUAGE_GLSL: LanguageFn = LanguageFn::from_fn(language);

/// The grammar's tables (the C `tree_sitter_glsl()`), decoded on first use.
pub fn language() -> &'static LanguageTables {
    &TABLES
}

fn decode() -> LanguageTables {
    let keyword_lex_fn: Option<LexFn> = Some(lex::ts_lex_keywords);
    let scanner_create: Option<ScannerCreateFn> = None;
    LanguageTables::decode(BLOB, lex::ts_lex, keyword_lex_fn, scanner_create)
}

#[cfg(test)]
mod tests {
    #[test]
    fn language_constant() {
        let language = tree_sitter::Language::new(super::LANGUAGE_GLSL);
        assert_eq!(language, super::LANGUAGE_GLSL.into());
        assert!(std::ptr::eq(
            super::LANGUAGE_GLSL.tables(),
            super::language()
        ));
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&language).unwrap();
        assert!(parser.parse("", None).is_some());
    }

    #[test]
    fn tables_decode() {
        let language = super::language();
        assert_eq!(language.abi_version, 14);
        assert_eq!(language.state_count, 2418);
        assert_eq!(language.symbol_count, 407);
    }
}
