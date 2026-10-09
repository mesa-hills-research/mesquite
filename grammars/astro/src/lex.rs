//! The `astro` grammar's lexer: `ts_lex`, transliterated from
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

const anon_sym_DASH_DASH_DASH: Symbol = 16;
const anon_sym_DASH_DASH_DASH2: Symbol = 17;
const anon_sym_DQUOTE: Symbol = 13;
const anon_sym_EQ: Symbol = 8;
const anon_sym_GT: Symbol = 3;
const anon_sym_LBRACE: Symbol = 18;
const anon_sym_LT: Symbol = 5;
const anon_sym_LT_BANG: Symbol = 1;
const anon_sym_LT_SLASH: Symbol = 7;
const anon_sym_RBRACE: Symbol = 19;
const anon_sym_SLASH_GT: Symbol = 6;
const anon_sym_SQUOTE: Symbol = 11;
const aux_sym_doctype_token1: Symbol = 2;
const aux_sym_quoted_attribute_value_token1: Symbol = 12;
const aux_sym_quoted_attribute_value_token2: Symbol = 14;
const sym__doctype: Symbol = 4;
const sym_attribute_name: Symbol = 9;
const sym_attribute_value: Symbol = 10;
const sym_text: Symbol = 15;
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
                if let Some(next) = advance_map(&[
                    (34, 36), (39, 33), (45, 6), (47, 10), (60, 27), (61, 30), (62, 25), (123, 44),
                    (125, 45), (68, 13), (100, 13),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 0; lexer.advance(true); continue; }
                return result;
            }
            1 => {
                if lookahead == 34 { state = 36; lexer.advance(false); continue; }
                if lookahead == 39 { state = 33; lexer.advance(false); continue; }
                if lookahead == 123 { state = 44; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 1; lexer.advance(true); continue; }
                if lookahead != 0 && (lookahead < 60 || 62 < lookahead) { state = 32; lexer.advance(false); continue; }
                return result;
            }
            2 => {
                if lookahead == 34 { state = 36; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 37; lexer.advance(false); continue; }
                if lookahead != 0 { state = 38; lexer.advance(false); continue; }
                return result;
            }
            3 => {
                if lookahead == 39 { state = 33; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 34; lexer.advance(false); continue; }
                if lookahead != 0 { state = 35; lexer.advance(false); continue; }
                return result;
            }
            4 => {
                if lookahead == 45 { state = 42; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                if lookahead == 45 { state = 43; lexer.advance(false); continue; }
                return result;
            }
            6 => {
                if lookahead == 45 { state = 4; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if lookahead == 45 { state = 5; lexer.advance(false); continue; }
                return result;
            }
            8 => {
                if lookahead == 45 { state = 7; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 8; lexer.advance(true); continue; }
                return result;
            }
            9 => {
                if lookahead == 47 { state = 10; lexer.advance(false); continue; }
                if lookahead == 61 { state = 30; lexer.advance(false); continue; }
                if lookahead == 62 { state = 25; lexer.advance(false); continue; }
                if lookahead == 123 { state = 44; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 9; lexer.advance(true); continue; }
                if lookahead != 0 && lookahead != 34 && lookahead != 39 && (lookahead < 60 || 62 < lookahead) { state = 31; lexer.advance(false); continue; }
                return result;
            }
            10 => {
                if lookahead == 62 { state = 28; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if lookahead == 67 || lookahead == 99 { state = 15; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if lookahead == 69 || lookahead == 101 { state = 26; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if lookahead == 79 || lookahead == 111 { state = 11; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if lookahead == 80 || lookahead == 112 { state = 12; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if lookahead == 84 || lookahead == 116 { state = 16; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if lookahead == 89 || lookahead == 121 { state = 14; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 17; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 60 && lookahead != 62 && lookahead != 123 && lookahead != 125 { state = 41; lexer.advance(false); continue; }
                return result;
            }
            18 => {
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 23; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 62 { state = 24; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if eof { state = 21; lexer.advance(false); continue; }
                if lookahead == 45 { state = 40; lexer.advance(false); continue; }
                if lookahead == 60 { state = 27; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 19; lexer.advance(true); continue; }
                if lookahead != 0 && lookahead != 62 && lookahead != 123 && lookahead != 125 { state = 41; lexer.advance(false); continue; }
                return result;
            }
            20 => {
                if eof { state = 21; lexer.advance(false); continue; }
                if lookahead == 60 { state = 27; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 20; lexer.advance(true); continue; }
                if lookahead != 0 && lookahead != 62 && lookahead != 123 && lookahead != 125 { state = 41; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                result = true; lexer.set_result_symbol(ts_builtin_sym_end); lexer.mark_end();
                return result;
            }
            22 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_BANG); lexer.mark_end();
                return result;
            }
            23 => {
                result = true; lexer.set_result_symbol(aux_sym_doctype_token1); lexer.mark_end();
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 23; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 62 { state = 24; lexer.advance(false); continue; }
                return result;
            }
            24 => {
                result = true; lexer.set_result_symbol(aux_sym_doctype_token1); lexer.mark_end();
                if lookahead != 0 && lookahead != 62 { state = 24; lexer.advance(false); continue; }
                return result;
            }
            25 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                return result;
            }
            26 => {
                result = true; lexer.set_result_symbol(sym__doctype); lexer.mark_end();
                return result;
            }
            27 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 33 { state = 22; lexer.advance(false); continue; }
                if lookahead == 47 { state = 29; lexer.advance(false); continue; }
                return result;
            }
            28 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_GT); lexer.mark_end();
                return result;
            }
            29 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_SLASH); lexer.mark_end();
                return result;
            }
            30 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                return result;
            }
            31 => {
                result = true; lexer.set_result_symbol(sym_attribute_name); lexer.mark_end();
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 32 && lookahead != 34 && lookahead != 39 && lookahead != 47 && (lookahead < 60 || 62 < lookahead) { state = 31; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                result = true; lexer.set_result_symbol(sym_attribute_value); lexer.mark_end();
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 32 && lookahead != 34 && lookahead != 39 && (lookahead < 60 || 62 < lookahead) { state = 32; lexer.advance(false); continue; }
                return result;
            }
            33 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTE); lexer.mark_end();
                return result;
            }
            34 => {
                result = true; lexer.set_result_symbol(aux_sym_quoted_attribute_value_token1); lexer.mark_end();
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 34; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 39 { state = 35; lexer.advance(false); continue; }
                return result;
            }
            35 => {
                result = true; lexer.set_result_symbol(aux_sym_quoted_attribute_value_token1); lexer.mark_end();
                if lookahead != 0 && lookahead != 39 { state = 35; lexer.advance(false); continue; }
                return result;
            }
            36 => {
                result = true; lexer.set_result_symbol(anon_sym_DQUOTE); lexer.mark_end();
                return result;
            }
            37 => {
                result = true; lexer.set_result_symbol(aux_sym_quoted_attribute_value_token2); lexer.mark_end();
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 37; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 34 { state = 38; lexer.advance(false); continue; }
                return result;
            }
            38 => {
                result = true; lexer.set_result_symbol(aux_sym_quoted_attribute_value_token2); lexer.mark_end();
                if lookahead != 0 && lookahead != 34 { state = 38; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                result = true; lexer.set_result_symbol(sym_text); lexer.mark_end();
                if lookahead == 45 { state = 42; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 17; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 60 && lookahead != 62 && lookahead != 123 && lookahead != 125 { state = 41; lexer.advance(false); continue; }
                return result;
            }
            40 => {
                result = true; lexer.set_result_symbol(sym_text); lexer.mark_end();
                if lookahead == 45 { state = 39; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 17; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 60 && lookahead != 62 && lookahead != 123 && lookahead != 125 { state = 41; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                result = true; lexer.set_result_symbol(sym_text); lexer.mark_end();
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 17; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 60 && lookahead != 62 && lookahead != 123 && lookahead != 125 { state = 41; lexer.advance(false); continue; }
                return result;
            }
            42 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_DASH_DASH); lexer.mark_end();
                return result;
            }
            43 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_DASH_DASH2); lexer.mark_end();
                return result;
            }
            44 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACE); lexer.mark_end();
                return result;
            }
            45 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACE); lexer.mark_end();
                return result;
            }
            _ => return false,
        }
    }
}
