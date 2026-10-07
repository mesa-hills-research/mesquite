//! The `html` grammar's lexer: `ts_lex`, transliterated from
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

const anon_sym_DQUOTE: Symbol = 14;
const anon_sym_EQ: Symbol = 8;
const anon_sym_GT: Symbol = 3;
const anon_sym_LT: Symbol = 5;
const anon_sym_LT_BANG: Symbol = 1;
const anon_sym_LT_SLASH: Symbol = 7;
const anon_sym_SLASH_GT: Symbol = 6;
const anon_sym_SQUOTE: Symbol = 12;
const aux_sym_doctype_token1: Symbol = 2;
const aux_sym_quoted_attribute_value_token1: Symbol = 13;
const aux_sym_quoted_attribute_value_token2: Symbol = 15;
const sym__doctype: Symbol = 4;
const sym_attribute_name: Symbol = 9;
const sym_attribute_value: Symbol = 10;
const sym_entity: Symbol = 11;
const sym_text: Symbol = 16;
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
                if eof { state = 18; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (34, 73), (38, 3), (39, 70), (47, 6), (60, 24), (61, 27), (62, 22), (68, 9),
                    (100, 9),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 0; lexer.advance(true); continue; }
                return result;
            }
            1 => {
                if lookahead == 34 { state = 73; lexer.advance(false); continue; }
                if lookahead == 39 { state = 70; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 1; lexer.advance(true); continue; }
                if lookahead != 0 && (lookahead < 60 || 62 < lookahead) { state = 29; lexer.advance(false); continue; }
                return result;
            }
            2 => {
                if lookahead == 34 { state = 73; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 74; lexer.advance(false); continue; }
                if lookahead != 0 { state = 75; lexer.advance(false); continue; }
                return result;
            }
            3 => {
                if lookahead == 35 { state = 12; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 69; lexer.advance(false); continue; }
                return result;
            }
            4 => {
                if lookahead == 39 { state = 70; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 71; lexer.advance(false); continue; }
                if lookahead != 0 { state = 72; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                if lookahead == 47 { state = 6; lexer.advance(false); continue; }
                if lookahead == 61 { state = 27; lexer.advance(false); continue; }
                if lookahead == 62 { state = 22; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 5; lexer.advance(true); continue; }
                if lookahead != 0 && lookahead != 34 && lookahead != 39 && (lookahead < 60 || 62 < lookahead) { state = 28; lexer.advance(false); continue; }
                return result;
            }
            6 => {
                if lookahead == 62 { state = 25; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if lookahead == 67 || lookahead == 99 { state = 11; lexer.advance(false); continue; }
                return result;
            }
            8 => {
                if lookahead == 69 || lookahead == 101 { state = 23; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if lookahead == 79 || lookahead == 111 { state = 7; lexer.advance(false); continue; }
                return result;
            }
            10 => {
                if lookahead == 80 || lookahead == 112 { state = 8; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if lookahead == 84 || lookahead == 116 { state = 13; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if lookahead == 88 || lookahead == 120 { state = 16; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 35; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if lookahead == 89 || lookahead == 121 { state = 10; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 14; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 38 && lookahead != 60 && lookahead != 62 { state = 76; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 20; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 62 { state = 21; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 40; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if eof { state = 18; lexer.advance(false); continue; }
                if lookahead == 38 { state = 3; lexer.advance(false); continue; }
                if lookahead == 60 { state = 24; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 17; lexer.advance(true); continue; }
                if lookahead != 0 && lookahead != 62 { state = 76; lexer.advance(false); continue; }
                return result;
            }
            18 => {
                result = true; lexer.set_result_symbol(ts_builtin_sym_end); lexer.mark_end();
                return result;
            }
            19 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_BANG); lexer.mark_end();
                return result;
            }
            20 => {
                result = true; lexer.set_result_symbol(aux_sym_doctype_token1); lexer.mark_end();
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 20; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 62 { state = 21; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                result = true; lexer.set_result_symbol(aux_sym_doctype_token1); lexer.mark_end();
                if lookahead != 0 && lookahead != 62 { state = 21; lexer.advance(false); continue; }
                return result;
            }
            22 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                return result;
            }
            23 => {
                result = true; lexer.set_result_symbol(sym__doctype); lexer.mark_end();
                return result;
            }
            24 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 33 { state = 19; lexer.advance(false); continue; }
                if lookahead == 47 { state = 26; lexer.advance(false); continue; }
                return result;
            }
            25 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_GT); lexer.mark_end();
                return result;
            }
            26 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_SLASH); lexer.mark_end();
                return result;
            }
            27 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                return result;
            }
            28 => {
                result = true; lexer.set_result_symbol(sym_attribute_name); lexer.mark_end();
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 32 && lookahead != 34 && lookahead != 39 && lookahead != 47 && (lookahead < 60 || 62 < lookahead) { state = 28; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                result = true; lexer.set_result_symbol(sym_attribute_value); lexer.mark_end();
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 32 && lookahead != 34 && lookahead != 39 && (lookahead < 60 || 62 < lookahead) { state = 29; lexer.advance(false); continue; }
                return result;
            }
            30 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                return result;
            }
            31 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 31; lexer.advance(false); continue; }
                return result;
            }
            33 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 32; lexer.advance(false); continue; }
                return result;
            }
            34 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 33; lexer.advance(false); continue; }
                return result;
            }
            35 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 34; lexer.advance(false); continue; }
                return result;
            }
            36 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 31; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 36; lexer.advance(false); continue; }
                return result;
            }
            38 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 37; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 38; lexer.advance(false); continue; }
                return result;
            }
            40 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 39; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 31; lexer.advance(false); continue; }
                return result;
            }
            42 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 41; lexer.advance(false); continue; }
                return result;
            }
            43 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 42; lexer.advance(false); continue; }
                return result;
            }
            44 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 43; lexer.advance(false); continue; }
                return result;
            }
            45 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 44; lexer.advance(false); continue; }
                return result;
            }
            46 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 45; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 46; lexer.advance(false); continue; }
                return result;
            }
            48 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 47; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 48; lexer.advance(false); continue; }
                return result;
            }
            50 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 49; lexer.advance(false); continue; }
                return result;
            }
            51 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 50; lexer.advance(false); continue; }
                return result;
            }
            52 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 51; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 52; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 53; lexer.advance(false); continue; }
                return result;
            }
            55 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 54; lexer.advance(false); continue; }
                return result;
            }
            56 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 55; lexer.advance(false); continue; }
                return result;
            }
            57 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 56; lexer.advance(false); continue; }
                return result;
            }
            58 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 57; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 58; lexer.advance(false); continue; }
                return result;
            }
            60 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 59; lexer.advance(false); continue; }
                return result;
            }
            61 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 60; lexer.advance(false); continue; }
                return result;
            }
            62 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 61; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 62; lexer.advance(false); continue; }
                return result;
            }
            64 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            65 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 64; lexer.advance(false); continue; }
                return result;
            }
            66 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 65; lexer.advance(false); continue; }
                return result;
            }
            67 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 66; lexer.advance(false); continue; }
                return result;
            }
            68 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 67; lexer.advance(false); continue; }
                return result;
            }
            69 => {
                result = true; lexer.set_result_symbol(sym_entity); lexer.mark_end();
                if lookahead == 59 { state = 30; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 68; lexer.advance(false); continue; }
                return result;
            }
            70 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTE); lexer.mark_end();
                return result;
            }
            71 => {
                result = true; lexer.set_result_symbol(aux_sym_quoted_attribute_value_token1); lexer.mark_end();
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 71; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 39 { state = 72; lexer.advance(false); continue; }
                return result;
            }
            72 => {
                result = true; lexer.set_result_symbol(aux_sym_quoted_attribute_value_token1); lexer.mark_end();
                if lookahead != 0 && lookahead != 39 { state = 72; lexer.advance(false); continue; }
                return result;
            }
            73 => {
                result = true; lexer.set_result_symbol(anon_sym_DQUOTE); lexer.mark_end();
                return result;
            }
            74 => {
                result = true; lexer.set_result_symbol(aux_sym_quoted_attribute_value_token2); lexer.mark_end();
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 74; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 34 { state = 75; lexer.advance(false); continue; }
                return result;
            }
            75 => {
                result = true; lexer.set_result_symbol(aux_sym_quoted_attribute_value_token2); lexer.mark_end();
                if lookahead != 0 && lookahead != 34 { state = 75; lexer.advance(false); continue; }
                return result;
            }
            76 => {
                result = true; lexer.set_result_symbol(sym_text); lexer.mark_end();
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 14; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 38 && lookahead != 60 && lookahead != 62 { state = 76; lexer.advance(false); continue; }
                return result;
            }
            _ => return false,
        }
    }
}
