//! The `yaml` grammar's lexer: `ts_lex`, transliterated from
//! its `src/parser.c`, with the symbols and character sets they use.
//!
//! Generated from the grammar's `src/parser.c`: do not edit by hand.
//!
//! Each C `case` is a `match` arm, run once per character: `ADVANCE(n)` and
//! `SKIP(n)` set the state, call `lexer.advance(skip)` and start the next round
//! (C's `goto next_state`), `ACCEPT_TOKEN` is `lexer.accept_token`, and
//! `END_STATE()` returns the result. C reads `eof` for every character; here only
//! the arms that test it call `lexer.eof()`.
#![allow(non_upper_case_globals, unreachable_code, clippy::all)]

use tree_sitter_language::{Lexer, StateId, Symbol};

const ts_builtin_sym_end: Symbol = 0;

/// C's `ts_lex`: lexes one token from lex state `state`; returns whether it
/// accepted one.
#[rustfmt::skip]
// Every state of this grammar sets `result` before returning it.
#[allow(unused_assignments)]
pub(crate) fn ts_lex(lexer: &mut dyn Lexer, mut state: StateId) -> bool {
    let mut result = false;
    loop {
        // C's `start:` label (reached again after each `next_state:` advance).
        let _lookahead = lexer.lookahead();
        match state {
            0 => {
                let eof = lexer.eof();
                result = true; lexer.accept_token(ts_builtin_sym_end);
                if eof { state = 1; lexer.advance(false); continue; }
                return result;
            }
            1 => {
                result = true; lexer.accept_token(ts_builtin_sym_end);
                return result;
            }
            _ => return false,
        }
    }
}
