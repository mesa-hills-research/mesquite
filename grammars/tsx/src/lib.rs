//! The `tsx` tree-sitter grammar for the ported runtime: its tables (decoded from
//! `tables.bin`) and lexer (`lex.rs`), converted from the grammar's `src/parser.c`.
//!
//! Upstream: tree-sitter-typescript 0.23.2
//! (<https://github.com/tree-sitter/tree-sitter-typescript>, tag `v0.23.2`). Generated: do
//! not edit by hand (the scanner is typescript's `scanner.rs`).
#![forbid(unsafe_code)]

mod lex;
#[path = "../../typescript/src/scanner.rs"]
mod scanner;

use std::sync::LazyLock;

use tree_sitter_language::{LanguageFn, LanguageTables, LexFn, ScannerCreateFn};

/// The grammar's tables, written by the converter from the compiled C grammar.
static BLOB: &[u8] = include_bytes!("tables.bin");

static TABLES: LazyLock<LanguageTables> = LazyLock::new(decode);

/// The grammar, for `tree_sitter::Language::new(LANGUAGE)` or `LANGUAGE.into()`.
/// `mesquite_typescript` re-exports it as `tree_sitter_typescript::LANGUAGE_TSX`, as
/// in tree-sitter-typescript.
pub const LANGUAGE: LanguageFn = LanguageFn::from_fn(language);

/// The grammar's tables (the C `tree_sitter_tsx()`), decoded on first use.
pub fn language() -> &'static LanguageTables {
    &TABLES
}

fn decode() -> LanguageTables {
    let keyword_lex_fn: Option<LexFn> = Some(lex::ts_lex_keywords);
    let scanner_create: Option<ScannerCreateFn> = Some(scanner::create);
    LanguageTables::decode(BLOB, lex::ts_lex, keyword_lex_fn, scanner_create)
}

#[cfg(test)]
mod tests {
    #[test]
    fn language_constant() {
        let language = tree_sitter::Language::new(super::LANGUAGE);
        assert_eq!(language, super::LANGUAGE.into());
        assert!(std::ptr::eq(super::LANGUAGE.tables(), super::language()));
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&language).unwrap();
        assert!(parser.parse("", None).is_some());
    }

    #[test]
    fn tables_decode() {
        let language = super::language();
        assert_eq!(language.abi_version, 14);
        assert_eq!(language.state_count, 5986);
        assert_eq!(language.symbol_count, 393);
    }
}
