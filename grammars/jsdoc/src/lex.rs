//! The `jsdoc` grammar's lexer: `ts_lex`, transliterated from
//! its `src/parser.c`, with the symbols and character sets they use.
//!
//! Generated from the grammar's `src/parser.c`: do not edit by hand.
//!
//! Each C `case` is a `match` arm, run once per character: `ADVANCE(n)` and
//! `SKIP(n)` set the state, call `lexer.advance(skip)` and start the next round
//! (C's `goto next_state`), `ACCEPT_TOKEN` sets the result and marks the end, and
//! `END_STATE()` returns the result.
#![allow(non_upper_case_globals, unreachable_code, clippy::all)]

use tree_sitter_language::{Lexer, StateId, Symbol};

const anon_sym_COLON: Symbol = 7;
const anon_sym_DOT: Symbol = 9;
const anon_sym_LBRACE: Symbol = 1;
const anon_sym_LBRACK: Symbol = 12;
const anon_sym_POUND: Symbol = 10;
const anon_sym_RBRACE: Symbol = 2;
const anon_sym_RBRACK: Symbol = 13;
const anon_sym_SLASH: Symbol = 8;
const anon_sym_SLASH2: Symbol = 17;
const anon_sym_TILDE: Symbol = 11;
const sym__begin: Symbol = 16;
const sym__inline_tag_false_positive: Symbol = 3;
const sym__text: Symbol = 15;
const sym_identifier: Symbol = 14;
const sym_tag_name: Symbol = 6;
const sym_tag_name_with_argument: Symbol = 4;
const sym_tag_name_with_type: Symbol = 5;
const ts_builtin_sym_end: Symbol = 0;

/// `ADVANCE_MAP`: the state paired with the first key equal to `lookahead`.
fn advance_map(map: &[(u16, StateId)], lookahead: i32) -> Option<StateId> {
    map.iter()
        .find(|&&(key, _)| i32::from(key) == lookahead)
        .map(|&(_, state)| state)
}

/// C's `ts_lex`: lexes one token from lex state `state`; returns whether it
/// accepted one.
#[rustfmt::skip]
pub(crate) fn ts_lex(lexer: &mut dyn Lexer, mut state: StateId) -> bool {
    let mut result = false;
    loop {
        // C's `start:` label (reached again after each `next_state:` advance).
        let lookahead = lexer.lookahead();
        let eof = lexer.eof();
        match state {
            0 => {
                if eof { state = 21; lexer.advance(false); continue; }
                if lookahead == 10 { state = 19; lexer.advance(true); continue; }
                if lookahead == 35 { state = 131; lexer.advance(false); continue; }
                if lookahead == 46 { state = 130; lexer.advance(false); continue; }
                if lookahead == 47 { state = 129; lexer.advance(false); continue; }
                if lookahead == 58 { state = 127; lexer.advance(false); continue; }
                if lookahead == 64 { state = 17; lexer.advance(false); continue; }
                if lookahead == 91 { state = 133; lexer.advance(false); continue; }
                if lookahead == 93 { state = 134; lexer.advance(false); continue; }
                if lookahead == 123 { state = 23; lexer.advance(false); continue; }
                if lookahead == 125 { state = 24; lexer.advance(false); continue; }
                if lookahead == 126 { state = 132; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 20; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 42 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            1 => {
                if lookahead == 10 { state = 1; lexer.advance(true); continue; }
                if lookahead == 42 { state = 1; lexer.advance(true); continue; }
                if lookahead == 47 { state = 139; lexer.advance(false); continue; }
                if lookahead == 64 { state = 18; lexer.advance(false); continue; }
                if lookahead == 93 { state = 134; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 1; lexer.advance(true); continue; }
                if 11 <= lookahead && lookahead <= 13 { state = 2; lexer.advance(true); continue; }
                return result;
            }
            2 => {
                if lookahead == 10 { state = 1; lexer.advance(true); continue; }
                if lookahead == 47 { state = 139; lexer.advance(false); continue; }
                if lookahead == 64 { state = 18; lexer.advance(false); continue; }
                if lookahead == 93 { state = 134; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 2; lexer.advance(true); continue; }
                return result;
            }
            3 => {
                if lookahead == 10 { state = 3; lexer.advance(true); continue; }
                if lookahead == 42 { state = 3; lexer.advance(true); continue; }
                if lookahead == 47 { state = 140; lexer.advance(false); continue; }
                if lookahead == 64 { state = 17; lexer.advance(false); continue; }
                if lookahead == 91 { state = 133; lexer.advance(false); continue; }
                if lookahead == 123 { state = 22; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 3; lexer.advance(true); continue; }
                if 11 <= lookahead && lookahead <= 13 { state = 4; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 125 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            4 => {
                if lookahead == 10 { state = 3; lexer.advance(true); continue; }
                if lookahead == 47 { state = 140; lexer.advance(false); continue; }
                if lookahead == 64 { state = 17; lexer.advance(false); continue; }
                if lookahead == 91 { state = 133; lexer.advance(false); continue; }
                if lookahead == 123 { state = 22; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 4; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 42 && lookahead != 125 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                if lookahead == 10 { state = 5; lexer.advance(true); continue; }
                if lookahead == 35 { state = 131; lexer.advance(false); continue; }
                if lookahead == 42 { state = 5; lexer.advance(true); continue; }
                if lookahead == 46 { state = 130; lexer.advance(false); continue; }
                if lookahead == 47 { state = 140; lexer.advance(false); continue; }
                if lookahead == 58 { state = 127; lexer.advance(false); continue; }
                if lookahead == 64 { state = 17; lexer.advance(false); continue; }
                if lookahead == 126 { state = 132; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 5; lexer.advance(true); continue; }
                if 11 <= lookahead && lookahead <= 13 { state = 6; lexer.advance(true); continue; }
                if lookahead != 0 && lookahead != 123 && lookahead != 125 && lookahead != 126 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            6 => {
                if lookahead == 10 { state = 5; lexer.advance(true); continue; }
                if lookahead == 35 { state = 131; lexer.advance(false); continue; }
                if lookahead == 46 { state = 130; lexer.advance(false); continue; }
                if lookahead == 47 { state = 140; lexer.advance(false); continue; }
                if lookahead == 58 { state = 127; lexer.advance(false); continue; }
                if lookahead == 64 { state = 17; lexer.advance(false); continue; }
                if lookahead == 126 { state = 132; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 6; lexer.advance(true); continue; }
                if lookahead != 0 && lookahead != 42 && lookahead != 123 && lookahead != 125 && lookahead != 126 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if lookahead == 10 { state = 5; lexer.advance(true); continue; }
                if lookahead == 35 { state = 131; lexer.advance(false); continue; }
                if lookahead == 46 { state = 130; lexer.advance(false); continue; }
                if lookahead == 47 { state = 128; lexer.advance(false); continue; }
                if lookahead == 58 { state = 127; lexer.advance(false); continue; }
                if lookahead == 64 { state = 17; lexer.advance(false); continue; }
                if lookahead == 126 { state = 132; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 6; lexer.advance(true); continue; }
                if lookahead != 0 && lookahead != 42 && lookahead != 123 && lookahead != 125 && lookahead != 126 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            8 => {
                if lookahead == 10 { state = 8; lexer.advance(true); continue; }
                if lookahead == 42 { state = 8; lexer.advance(true); continue; }
                if lookahead == 47 { state = 140; lexer.advance(false); continue; }
                if lookahead == 64 { state = 17; lexer.advance(false); continue; }
                if lookahead == 123 { state = 23; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 8; lexer.advance(true); continue; }
                if 11 <= lookahead && lookahead <= 13 { state = 9; lexer.advance(true); continue; }
                if lookahead != 0 && lookahead != 125 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if lookahead == 10 { state = 8; lexer.advance(true); continue; }
                if lookahead == 47 { state = 140; lexer.advance(false); continue; }
                if lookahead == 64 { state = 17; lexer.advance(false); continue; }
                if lookahead == 123 { state = 23; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 9; lexer.advance(true); continue; }
                if lookahead != 0 && lookahead != 42 && lookahead != 125 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            10 => {
                if lookahead == 10 { state = 10; lexer.advance(true); continue; }
                if lookahead == 35 { state = 131; lexer.advance(false); continue; }
                if lookahead == 42 { state = 10; lexer.advance(true); continue; }
                if lookahead == 46 { state = 130; lexer.advance(false); continue; }
                if lookahead == 47 { state = 140; lexer.advance(false); continue; }
                if lookahead == 64 { state = 17; lexer.advance(false); continue; }
                if lookahead == 126 { state = 132; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 10; lexer.advance(true); continue; }
                if 11 <= lookahead && lookahead <= 13 { state = 11; lexer.advance(true); continue; }
                if lookahead != 0 && lookahead != 123 && lookahead != 125 && lookahead != 126 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if lookahead == 10 { state = 10; lexer.advance(true); continue; }
                if lookahead == 35 { state = 131; lexer.advance(false); continue; }
                if lookahead == 46 { state = 130; lexer.advance(false); continue; }
                if lookahead == 47 { state = 140; lexer.advance(false); continue; }
                if lookahead == 64 { state = 17; lexer.advance(false); continue; }
                if lookahead == 126 { state = 132; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 11; lexer.advance(true); continue; }
                if lookahead != 0 && lookahead != 42 && lookahead != 123 && lookahead != 125 && lookahead != 126 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if lookahead == 10 { state = 12; lexer.advance(true); continue; }
                if lookahead == 42 { state = 12; lexer.advance(true); continue; }
                if lookahead == 123 { state = 23; lexer.advance(false); continue; }
                if lookahead == 125 { state = 24; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 12; lexer.advance(true); continue; }
                if 11 <= lookahead && lookahead <= 13 { state = 13; lexer.advance(true); continue; }
                if lookahead != 0 && lookahead != 64 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if lookahead == 10 { state = 12; lexer.advance(true); continue; }
                if lookahead == 123 { state = 23; lexer.advance(false); continue; }
                if lookahead == 125 { state = 24; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 13; lexer.advance(true); continue; }
                if lookahead != 0 && lookahead != 42 && lookahead != 64 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if lookahead == 10 { state = 14; lexer.advance(true); continue; }
                if lookahead == 42 { state = 14; lexer.advance(true); continue; }
                if lookahead == 47 { state = 140; lexer.advance(false); continue; }
                if lookahead == 64 { state = 17; lexer.advance(false); continue; }
                if lookahead == 123 { state = 22; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 14; lexer.advance(true); continue; }
                if 11 <= lookahead && lookahead <= 13 { state = 15; lexer.advance(true); continue; }
                if lookahead != 0 && lookahead != 125 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if lookahead == 10 { state = 14; lexer.advance(true); continue; }
                if lookahead == 47 { state = 140; lexer.advance(false); continue; }
                if lookahead == 64 { state = 17; lexer.advance(false); continue; }
                if lookahead == 123 { state = 22; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 15; lexer.advance(true); continue; }
                if lookahead != 0 && lookahead != 42 && lookahead != 125 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if lookahead == 42 { state = 137; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 123 && lookahead != 125 { state = 16; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if let Some(next) = advance_map(&[
                    (97, 43), (98, 86), (99, 33), (101, 121), (102, 66), (109, 64), (110, 34), (112, 38),
                    (114, 50), (115, 36), (116, 63),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 100 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            18 => {
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if eof { state = 21; lexer.advance(false); continue; }
                if lookahead == 10 { state = 19; lexer.advance(true); continue; }
                if lookahead == 35 { state = 131; lexer.advance(false); continue; }
                if lookahead == 42 { state = 19; lexer.advance(true); continue; }
                if lookahead == 46 { state = 130; lexer.advance(false); continue; }
                if lookahead == 47 { state = 141; lexer.advance(false); continue; }
                if lookahead == 58 { state = 127; lexer.advance(false); continue; }
                if lookahead == 64 { state = 17; lexer.advance(false); continue; }
                if lookahead == 91 { state = 133; lexer.advance(false); continue; }
                if lookahead == 93 { state = 134; lexer.advance(false); continue; }
                if lookahead == 123 { state = 23; lexer.advance(false); continue; }
                if lookahead == 125 { state = 24; lexer.advance(false); continue; }
                if lookahead == 126 { state = 132; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 19; lexer.advance(true); continue; }
                if 11 <= lookahead && lookahead <= 13 { state = 20; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                if lookahead != 0 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            20 => {
                if eof { state = 21; lexer.advance(false); continue; }
                if lookahead == 10 { state = 19; lexer.advance(true); continue; }
                if lookahead == 35 { state = 131; lexer.advance(false); continue; }
                if lookahead == 46 { state = 130; lexer.advance(false); continue; }
                if lookahead == 47 { state = 141; lexer.advance(false); continue; }
                if lookahead == 58 { state = 127; lexer.advance(false); continue; }
                if lookahead == 64 { state = 17; lexer.advance(false); continue; }
                if lookahead == 91 { state = 133; lexer.advance(false); continue; }
                if lookahead == 93 { state = 134; lexer.advance(false); continue; }
                if lookahead == 123 { state = 23; lexer.advance(false); continue; }
                if lookahead == 125 { state = 24; lexer.advance(false); continue; }
                if lookahead == 126 { state = 132; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 20; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 42 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                result = true; lexer.set_result_symbol(ts_builtin_sym_end); lexer.mark_end();
                return result;
            }
            22 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACE); lexer.mark_end();
                return result;
            }
            23 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACE); lexer.mark_end();
                if lookahead != 0 && lookahead != 64 && lookahead != 125 { state = 26; lexer.advance(false); continue; }
                return result;
            }
            24 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACE); lexer.mark_end();
                return result;
            }
            25 => {
                result = true; lexer.set_result_symbol(sym__inline_tag_false_positive); lexer.mark_end();
                return result;
            }
            26 => {
                result = true; lexer.set_result_symbol(sym__inline_tag_false_positive); lexer.mark_end();
                if lookahead == 125 { state = 25; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 64 { state = 26; lexer.advance(false); continue; }
                return result;
            }
            27 => {
                result = true; lexer.set_result_symbol(sym_tag_name_with_argument); lexer.mark_end();
                if lookahead == 101 { state = 105; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            28 => {
                result = true; lexer.set_result_symbol(sym_tag_name_with_argument); lexer.mark_end();
                if lookahead == 115 { state = 94; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                result = true; lexer.set_result_symbol(sym_tag_name_with_argument); lexer.mark_end();
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            30 => {
                result = true; lexer.set_result_symbol(sym_tag_name_with_type); lexer.mark_end();
                if lookahead == 100 { state = 56; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            31 => {
                result = true; lexer.set_result_symbol(sym_tag_name_with_type); lexer.mark_end();
                if lookahead == 115 { state = 32; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                result = true; lexer.set_result_symbol(sym_tag_name_with_type); lexer.mark_end();
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            33 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 97 { state = 74; lexer.advance(false); continue; }
                if lookahead == 111 { state = 79; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            34 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 97 { state = 76; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            35 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 97 { state = 75; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            36 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 97 { state = 115; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 97 { state = 44; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            38 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 97 { state = 106; lexer.advance(false); continue; }
                if lookahead == 114 { state = 88; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 97 { state = 72; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            40 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 97 { state = 107; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 97 { state = 47; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            42 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 98 { state = 37; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            43 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 99 { state = 45; lexer.advance(false); continue; }
                if lookahead == 108 { state = 67; lexer.advance(false); continue; }
                if lookahead == 112 { state = 65; lexer.advance(false); continue; }
                if lookahead == 117 { state = 62; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            44 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 99 { state = 71; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            45 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 99 { state = 55; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            46 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 99 { state = 118; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 99 { state = 51; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            48 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 99 { state = 116; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 100 { state = 107; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            50 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 101 { state = 112; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            51 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 101 { state = 29; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            52 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 101 { state = 80; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 101 { state = 28; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 101 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            55 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 101 { state = 110; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            56 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 101 { state = 60; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            57 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 101 { state = 83; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            58 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 101 { state = 107; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 101 { state = 85; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            60 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 102 { state = 29; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            61 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 102 { state = 70; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            62 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 103 { state = 77; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 104 { state = 99; lexer.advance(false); continue; }
                if lookahead == 121 { state = 95; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            64 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 105 { state = 124; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            65 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 105 { state = 29; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            66 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 105 { state = 101; lexer.advance(false); continue; }
                if lookahead == 117 { state = 82; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            67 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 105 { state = 40; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            68 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 105 { state = 108; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            69 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 105 { state = 91; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            70 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 105 { state = 58; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            71 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 107 { state = 29; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            72 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 108 { state = 29; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            73 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 108 { state = 42; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            74 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 108 { state = 73; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            75 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 109 { state = 29; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            76 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 109 { state = 53; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            77 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 109 { state = 59; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            78 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 110 { state = 29; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            79 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 110 { state = 109; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            80 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 110 { state = 49; lexer.advance(false); continue; }
                if lookahead == 114 { state = 84; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            81 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 110 { state = 31; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            82 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 110 { state = 48; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            83 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 110 { state = 111; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            84 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 110 { state = 39; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            85 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 110 { state = 114; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            86 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 111 { state = 102; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            87 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 111 { state = 122; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            88 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 111 { state = 93; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            89 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 111 { state = 123; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            90 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 111 { state = 104; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            91 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 111 { state = 78; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            92 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 111 { state = 97; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            93 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 112 { state = 27; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            94 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 112 { state = 41; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            95 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 112 { state = 54; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            96 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 112 { state = 90; lexer.advance(false); continue; }
                if lookahead == 116 { state = 52; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            97 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 114 { state = 29; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            98 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 114 { state = 120; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            99 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 114 { state = 87; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            100 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 114 { state = 89; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            101 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 114 { state = 58; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            102 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 114 { state = 100; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            103 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 114 { state = 81; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            104 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 114 { state = 114; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            105 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 114 { state = 113; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            106 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 114 { state = 35; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            107 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 115 { state = 29; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            108 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 115 { state = 61; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            109 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 115 { state = 117; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            110 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 115 { state = 107; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            111 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 116 { state = 29; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            112 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 116 { state = 119; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            113 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 116 { state = 125; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            114 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 116 { state = 107; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            115 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 116 { state = 68; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            116 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 116 { state = 69; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            117 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 116 { state = 98; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            118 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 116 { state = 92; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            119 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 117 { state = 103; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            120 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 117 { state = 46; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            121 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 118 { state = 57; lexer.advance(false); continue; }
                if lookahead == 120 { state = 96; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            122 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 119 { state = 31; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            123 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 119 { state = 107; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            124 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 120 { state = 58; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            125 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if lookahead == 121 { state = 29; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            126 => {
                result = true; lexer.set_result_symbol(sym_tag_name); lexer.mark_end();
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            127 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON); lexer.mark_end();
                return result;
            }
            128 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH); lexer.mark_end();
                return result;
            }
            129 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH); lexer.mark_end();
                if lookahead == 42 { state = 139; lexer.advance(false); continue; }
                return result;
            }
            130 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                return result;
            }
            131 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND); lexer.mark_end();
                return result;
            }
            132 => {
                result = true; lexer.set_result_symbol(anon_sym_TILDE); lexer.mark_end();
                return result;
            }
            133 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK); lexer.mark_end();
                return result;
            }
            134 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACK); lexer.mark_end();
                return result;
            }
            135 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            136 => {
                result = true; lexer.set_result_symbol(sym__text); lexer.mark_end();
                if lookahead == 42 { state = 137; lexer.advance(false); continue; }
                if lookahead == 47 { state = 136; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 123 && lookahead != 125 { state = 136; lexer.advance(false); continue; }
                return result;
            }
            137 => {
                result = true; lexer.set_result_symbol(sym__text); lexer.mark_end();
                if lookahead == 42 { state = 137; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 47 && lookahead != 123 && lookahead != 125 { state = 16; lexer.advance(false); continue; }
                return result;
            }
            138 => {
                result = true; lexer.set_result_symbol(sym__text); lexer.mark_end();
                if lookahead == 47 { state = 138; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 42 && lookahead != 123 && lookahead != 125 { state = 136; lexer.advance(false); continue; }
                return result;
            }
            139 => {
                result = true; lexer.set_result_symbol(sym__begin); lexer.mark_end();
                if lookahead == 42 { state = 139; lexer.advance(false); continue; }
                return result;
            }
            140 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH2); lexer.mark_end();
                return result;
            }
            141 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH2); lexer.mark_end();
                if lookahead == 42 { state = 139; lexer.advance(false); continue; }
                return result;
            }
            _ => return false,
        }
    }
}
