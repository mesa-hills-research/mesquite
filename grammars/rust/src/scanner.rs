//! The grammar's external scanner: a translation of its C `scanner.c`.
//!
//! STUB WRITTEN BY THE TS_PORT HOST: the scanner has not been translated yet. Replace
//! this file with a faithful translation of the C scanner. The generated `lib.rs` uses
//! only `create()`.

use ts_port_tables::{ExternalScanner, Lexer};

/// The scanner's state (C's `payload`).
pub(crate) struct Scanner;

impl ExternalScanner for Scanner {
    fn scan(&mut self, _lexer: &mut dyn Lexer, _valid_symbols: &[bool]) -> bool {
        panic!("this grammar's external scanner has not been translated yet (scanner.rs is a stub)")
    }

    fn serialize(&mut self, _buffer: &mut [u8]) -> usize {
        0
    }

    fn deserialize(&mut self, _buffer: &[u8]) {}
}

/// Creates a scanner (C's `tree_sitter_<grammar>_external_scanner_create`).
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    Box::new(Scanner)
}
