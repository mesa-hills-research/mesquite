//! The `ocaml` tree-sitter grammar for the ported runtime: its tables (decoded from
//! `tables.bin`) and lexer (`lex.rs`), converted from the grammar's `src/parser.c`.
//!
//! Upstream: tree-sitter-ocaml 0.26.0 (<https://github.com/tree-sitter/tree-sitter-ocaml>,
//! tag `v0.26.0`). Generated: do not edit by hand, except the scanner (`scanner.rs`).
#![forbid(unsafe_code)]

mod lex;
mod scanner;

use std::sync::LazyLock;

use tree_sitter_language::{LanguageFn, LanguageTables, LexFn, ScannerCreateFn};

/// The grammar's tables, written by the converter from the compiled C grammar.
static BLOB: &[u8] = include_bytes!("tables.bin");

static TABLES: LazyLock<LanguageTables> = LazyLock::new(decode);

/// The grammar, for `tree_sitter::Language::new(LANGUAGE_OCAML)` or
/// `LANGUAGE_OCAML.into()`, as with tree-sitter-ocaml's
/// `tree_sitter_ocaml::LANGUAGE_OCAML`.
pub const LANGUAGE_OCAML: LanguageFn = LanguageFn::from_fn(language);

/// The OCaml (interface) grammar (crate `mesquite_ocaml_interface`), as with
/// tree-sitter-ocaml's `tree_sitter_ocaml::LANGUAGE_OCAML_INTERFACE`.
pub use tree_sitter_ocaml_interface::LANGUAGE as LANGUAGE_OCAML_INTERFACE;

/// The grammar's tables (the C `tree_sitter_ocaml()`), decoded on first use.
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
        let language = tree_sitter::Language::new(super::LANGUAGE_OCAML);
        assert_eq!(language, super::LANGUAGE_OCAML.into());
        assert!(std::ptr::eq(
            super::LANGUAGE_OCAML.tables(),
            super::language()
        ));
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&language).unwrap();
        assert!(parser.parse("", None).is_some());
    }

    #[test]
    fn reexported_ocaml_interface() {
        let reexported = tree_sitter::Language::new(super::LANGUAGE_OCAML_INTERFACE);
        assert_eq!(reexported, tree_sitter_ocaml_interface::LANGUAGE.into());
        assert_ne!(reexported, super::LANGUAGE_OCAML.into());
    }

    #[test]
    fn tables_decode() {
        let language = super::language();
        assert_eq!(language.abi_version, 15);
        assert_eq!(language.state_count, 13556);
        assert_eq!(language.symbol_count, 537);
    }
}
