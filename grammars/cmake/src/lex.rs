//! The `cmake` grammar's lexer: `ts_lex`, transliterated from
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

const anon_sym_BSLASH_SEMI: Symbol = 6;
const anon_sym_BSLASHn: Symbol = 4;
const anon_sym_BSLASHr: Symbol = 3;
const anon_sym_BSLASHt: Symbol = 2;
const anon_sym_CACHE: Symbol = 12;
const anon_sym_DOLLAR: Symbol = 8;
const anon_sym_DQUOTE: Symbol = 16;
const anon_sym_ENV: Symbol = 11;
const anon_sym_LBRACE: Symbol = 9;
const anon_sym_LPAREN: Symbol = 14;
const anon_sym_RBRACE: Symbol = 10;
const anon_sym_RPAREN: Symbol = 15;
const anon_sym_SEMI: Symbol = 5;
const aux_sym__quoted_text_token1: Symbol = 17;
const aux_sym__unquoted_text_token1: Symbol = 18;
const aux_sym__untrimmed_argument_token1: Symbol = 13;
const aux_sym_endwhile_command_token1: Symbol = 20;
const aux_sym_if_command_token1: Symbol = 19;
const aux_sym_variable_token1: Symbol = 7;
const sym__escape_identity: Symbol = 1;
const sym_block: Symbol = 33;
const sym_else: Symbol = 23;
const sym_elseif: Symbol = 22;
const sym_endblock: Symbol = 34;
const sym_endforeach: Symbol = 26;
const sym_endfunction: Symbol = 30;
const sym_endif: Symbol = 24;
const sym_endmacro: Symbol = 32;
const sym_endwhile: Symbol = 28;
const sym_foreach: Symbol = 25;
const sym_function: Symbol = 29;
const sym_identifier: Symbol = 35;
const sym_if: Symbol = 21;
const sym_macro: Symbol = 31;
const sym_while: Symbol = 27;
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
                if eof { state = 20; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (34, 36), (36, 28), (40, 34), (41, 35), (59, 25), (67, 41), (69, 42), (92, 8),
                    (123, 29), (9, 33), (32, 33),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 10 <= lookahead && lookahead <= 13 { state = 33; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 34 || 36 < lookahead) { state = 40; lexer.advance(false); continue; }
                return result;
            }
            1 => {
                if let Some(next) = advance_map(&[
                    (34, 36), (36, 28), (40, 34), (41, 35), (59, 25), (67, 41), (69, 42), (92, 8),
                    (123, 29),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 33; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 34 || 36 < lookahead) { state = 40; lexer.advance(false); continue; }
                return result;
            }
            2 => {
                if lookahead == 34 { state = 36; lexer.advance(false); continue; }
                if lookahead == 36 { state = 28; lexer.advance(false); continue; }
                if lookahead == 40 { state = 34; lexer.advance(false); continue; }
                if lookahead == 41 { state = 35; lexer.advance(false); continue; }
                if lookahead == 59 { state = 25; lexer.advance(false); continue; }
                if lookahead == 92 { state = 8; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 33; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 34 || 36 < lookahead) { state = 40; lexer.advance(false); continue; }
                return result;
            }
            3 => {
                if lookahead == 34 { state = 36; lexer.advance(false); continue; }
                if lookahead == 36 { state = 28; lexer.advance(false); continue; }
                if lookahead == 41 { state = 35; lexer.advance(false); continue; }
                if lookahead == 59 { state = 25; lexer.advance(false); continue; }
                if lookahead == 92 { state = 8; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 34 || 36 < lookahead) && lookahead != 40 && lookahead != 41 { state = 40; lexer.advance(false); continue; }
                return result;
            }
            4 => {
                if lookahead == 34 { state = 36; lexer.advance(false); continue; }
                if lookahead == 36 { state = 28; lexer.advance(false); continue; }
                if lookahead == 59 { state = 25; lexer.advance(false); continue; }
                if lookahead == 67 { state = 38; lexer.advance(false); continue; }
                if lookahead == 69 { state = 39; lexer.advance(false); continue; }
                if lookahead == 92 { state = 8; lexer.advance(false); continue; }
                if lookahead == 123 { state = 29; lexer.advance(false); continue; }
                if lookahead != 0 { state = 37; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                if lookahead == 34 { state = 36; lexer.advance(false); continue; }
                if lookahead == 36 { state = 28; lexer.advance(false); continue; }
                if lookahead == 59 { state = 25; lexer.advance(false); continue; }
                if lookahead == 92 { state = 8; lexer.advance(false); continue; }
                if lookahead != 0 { state = 37; lexer.advance(false); continue; }
                return result;
            }
            6 => {
                if lookahead == 36 { state = 28; lexer.advance(false); continue; }
                if lookahead == 40 { state = 34; lexer.advance(false); continue; }
                if lookahead == 59 { state = 25; lexer.advance(false); continue; }
                if lookahead == 92 { state = 8; lexer.advance(false); continue; }
                if lookahead == 125 { state = 30; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 44; lexer.advance(false); continue; }
                if lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 27; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if lookahead == 36 { state = 28; lexer.advance(false); continue; }
                if lookahead == 41 { state = 35; lexer.advance(false); continue; }
                if lookahead == 59 { state = 25; lexer.advance(false); continue; }
                if lookahead == 67 { state = 41; lexer.advance(false); continue; }
                if lookahead == 69 { state = 42; lexer.advance(false); continue; }
                if lookahead == 92 { state = 8; lexer.advance(false); continue; }
                if lookahead == 123 { state = 29; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 34 || 36 < lookahead) && lookahead != 40 && lookahead != 41 { state = 40; lexer.advance(false); continue; }
                return result;
            }
            8 => {
                if lookahead == 59 { state = 26; lexer.advance(false); continue; }
                if lookahead == 110 { state = 24; lexer.advance(false); continue; }
                if lookahead == 114 { state = 23; lexer.advance(false); continue; }
                if lookahead == 116 { state = 22; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 48 || 57 < lookahead) && (lookahead < 65 || 90 < lookahead) && (lookahead < 97 || 122 < lookahead) { state = 21; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if lookahead == 67 { state = 11; lexer.advance(false); continue; }
                return result;
            }
            10 => {
                if lookahead == 69 { state = 32; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if lookahead == 72 { state = 10; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if lookahead == 86 { state = 31; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if let Some(next) = advance_map(&[
                    (66, 103), (98, 103), (69, 104), (101, 104), (70, 118), (102, 118), (73, 87), (105, 87),
                    (77, 63), (109, 63), (87, 92), (119, 92),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 33; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if let Some(next) = advance_map(&[
                    (66, 103), (98, 103), (69, 111), (101, 111), (70, 118), (102, 118), (73, 87), (105, 87),
                    (77, 63), (109, 63), (87, 92), (119, 92),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 33; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if let Some(next) = advance_map(&[
                    (66, 103), (98, 103), (69, 113), (101, 113), (70, 118), (102, 118), (73, 87), (105, 87),
                    (77, 63), (109, 63), (87, 92), (119, 92),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 33; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if let Some(next) = advance_map(&[
                    (66, 103), (98, 103), (69, 114), (101, 114), (70, 118), (102, 118), (73, 87), (105, 87),
                    (77, 63), (109, 63), (87, 92), (119, 92),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 33; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if let Some(next) = advance_map(&[
                    (66, 103), (98, 103), (69, 115), (101, 115), (70, 118), (102, 118), (73, 87), (105, 87),
                    (77, 63), (109, 63), (87, 92), (119, 92),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 33; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            18 => {
                if let Some(next) = advance_map(&[
                    (66, 103), (98, 103), (69, 116), (101, 116), (70, 118), (102, 118), (73, 87), (105, 87),
                    (77, 63), (109, 63), (87, 92), (119, 92),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 33; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if eof { state = 20; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (125, 30), (66, 103), (98, 103), (70, 118), (102, 118), (73, 87), (105, 87), (77, 63),
                    (109, 63), (87, 92), (119, 92),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 33; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            20 => {
                result = true; lexer.set_result_symbol(ts_builtin_sym_end); lexer.mark_end();
                return result;
            }
            21 => {
                result = true; lexer.set_result_symbol(sym__escape_identity); lexer.mark_end();
                return result;
            }
            22 => {
                result = true; lexer.set_result_symbol(anon_sym_BSLASHt); lexer.mark_end();
                return result;
            }
            23 => {
                result = true; lexer.set_result_symbol(anon_sym_BSLASHr); lexer.mark_end();
                return result;
            }
            24 => {
                result = true; lexer.set_result_symbol(anon_sym_BSLASHn); lexer.mark_end();
                return result;
            }
            25 => {
                result = true; lexer.set_result_symbol(anon_sym_SEMI); lexer.mark_end();
                return result;
            }
            26 => {
                result = true; lexer.set_result_symbol(anon_sym_BSLASH_SEMI); lexer.mark_end();
                return result;
            }
            27 => {
                result = true; lexer.set_result_symbol(aux_sym_variable_token1); lexer.mark_end();
                return result;
            }
            28 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR); lexer.mark_end();
                return result;
            }
            29 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACE); lexer.mark_end();
                return result;
            }
            30 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACE); lexer.mark_end();
                return result;
            }
            31 => {
                result = true; lexer.set_result_symbol(anon_sym_ENV); lexer.mark_end();
                return result;
            }
            32 => {
                result = true; lexer.set_result_symbol(anon_sym_CACHE); lexer.mark_end();
                return result;
            }
            33 => {
                result = true; lexer.set_result_symbol(aux_sym__untrimmed_argument_token1); lexer.mark_end();
                return result;
            }
            34 => {
                result = true; lexer.set_result_symbol(anon_sym_LPAREN); lexer.mark_end();
                return result;
            }
            35 => {
                result = true; lexer.set_result_symbol(anon_sym_RPAREN); lexer.mark_end();
                return result;
            }
            36 => {
                result = true; lexer.set_result_symbol(anon_sym_DQUOTE); lexer.mark_end();
                return result;
            }
            37 => {
                result = true; lexer.set_result_symbol(aux_sym__quoted_text_token1); lexer.mark_end();
                return result;
            }
            38 => {
                result = true; lexer.set_result_symbol(aux_sym__quoted_text_token1); lexer.mark_end();
                if lookahead == 65 { state = 9; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                result = true; lexer.set_result_symbol(aux_sym__quoted_text_token1); lexer.mark_end();
                if lookahead == 78 { state = 12; lexer.advance(false); continue; }
                return result;
            }
            40 => {
                result = true; lexer.set_result_symbol(aux_sym__unquoted_text_token1); lexer.mark_end();
                return result;
            }
            41 => {
                result = true; lexer.set_result_symbol(aux_sym__unquoted_text_token1); lexer.mark_end();
                if lookahead == 65 { state = 9; lexer.advance(false); continue; }
                return result;
            }
            42 => {
                result = true; lexer.set_result_symbol(aux_sym__unquoted_text_token1); lexer.mark_end();
                if lookahead == 78 { state = 12; lexer.advance(false); continue; }
                return result;
            }
            43 => {
                result = true; lexer.set_result_symbol(aux_sym__unquoted_text_token1); lexer.mark_end();
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 48; lexer.advance(false); continue; }
                return result;
            }
            44 => {
                result = true; lexer.set_result_symbol(aux_sym_if_command_token1); lexer.mark_end();
                return result;
            }
            45 => {
                result = true; lexer.set_result_symbol(aux_sym_endwhile_command_token1); lexer.mark_end();
                if lookahead == 36 { state = 28; lexer.advance(false); continue; }
                if lookahead == 59 { state = 25; lexer.advance(false); continue; }
                if lookahead == 67 { state = 41; lexer.advance(false); continue; }
                if lookahead == 69 { state = 42; lexer.advance(false); continue; }
                if lookahead == 92 { state = 8; lexer.advance(false); continue; }
                if lookahead == 123 { state = 29; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 43; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 34 || 36 < lookahead) && lookahead != 40 && lookahead != 41 { state = 40; lexer.advance(false); continue; }
                return result;
            }
            46 => {
                result = true; lexer.set_result_symbol(aux_sym_endwhile_command_token1); lexer.mark_end();
                if lookahead == 36 { state = 28; lexer.advance(false); continue; }
                if lookahead == 59 { state = 25; lexer.advance(false); continue; }
                if lookahead == 92 { state = 8; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 43; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 34 || 36 < lookahead) && lookahead != 40 && lookahead != 41 { state = 40; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                result = true; lexer.set_result_symbol(aux_sym_endwhile_command_token1); lexer.mark_end();
                if lookahead == 41 { state = 35; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 48; lexer.advance(false); continue; }
                return result;
            }
            48 => {
                result = true; lexer.set_result_symbol(aux_sym_endwhile_command_token1); lexer.mark_end();
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 48; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                result = true; lexer.set_result_symbol(sym_if); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            50 => {
                result = true; lexer.set_result_symbol(sym_elseif); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            51 => {
                result = true; lexer.set_result_symbol(sym_else); lexer.mark_end();
                if lookahead == 73 || lookahead == 105 { state = 89; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            52 => {
                result = true; lexer.set_result_symbol(sym_endif); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                result = true; lexer.set_result_symbol(sym_foreach); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                result = true; lexer.set_result_symbol(sym_endforeach); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            55 => {
                result = true; lexer.set_result_symbol(sym_while); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            56 => {
                result = true; lexer.set_result_symbol(sym_endwhile); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            57 => {
                result = true; lexer.set_result_symbol(sym_function); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            58 => {
                result = true; lexer.set_result_symbol(sym_endfunction); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                result = true; lexer.set_result_symbol(sym_macro); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            60 => {
                result = true; lexer.set_result_symbol(sym_endmacro); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            61 => {
                result = true; lexer.set_result_symbol(sym_block); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            62 => {
                result = true; lexer.set_result_symbol(sym_endblock); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 65 || lookahead == 97 { state = 71; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 66 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            64 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 65 || lookahead == 97 { state = 70; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 66 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            65 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 65 || lookahead == 97 { state = 73; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 66 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            66 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 65 || lookahead == 97 { state = 74; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 66 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            67 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 66 || lookahead == 98 { state = 107; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            68 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 67 || lookahead == 99 { state = 101; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            69 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 67 || lookahead == 99 { state = 131; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            70 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 67 || lookahead == 99 { state = 93; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            71 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 67 || lookahead == 99 { state = 127; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            72 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 67 || lookahead == 99 { state = 102; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            73 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 67 || lookahead == 99 { state = 94; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            74 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 67 || lookahead == 99 { state = 128; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            75 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 67 || lookahead == 99 { state = 132; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            76 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 68 || lookahead == 100 { state = 134; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            77 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 68 || lookahead == 100 { state = 108; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            78 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 68 || lookahead == 100 { state = 67; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            79 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 68 || lookahead == 100 { state = 97; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            80 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 68 || lookahead == 100 { state = 91; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            81 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 68 || lookahead == 100 { state = 90; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            82 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 69 || lookahead == 101 { state = 55; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            83 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 69 || lookahead == 101 { state = 51; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            84 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 69 || lookahead == 101 { state = 56; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            85 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 69 || lookahead == 101 { state = 64; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            86 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 69 || lookahead == 101 { state = 65; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            87 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 70 || lookahead == 102 { state = 49; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            88 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 70 || lookahead == 102 { state = 52; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            89 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 70 || lookahead == 102 { state = 50; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            90 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 70 || lookahead == 102 { state = 133; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            91 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 70 || lookahead == 102 { state = 125; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            92 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 72 || lookahead == 104 { state = 96; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            93 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 72 || lookahead == 104 { state = 53; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            94 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 72 || lookahead == 104 { state = 54; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            95 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 72 || lookahead == 104 { state = 100; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            96 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 73 || lookahead == 105 { state = 105; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            97 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 73 || lookahead == 105 { state = 88; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            98 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 73 || lookahead == 105 { state = 121; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            99 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 73 || lookahead == 105 { state = 123; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            100 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 73 || lookahead == 105 { state = 106; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            101 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 75 || lookahead == 107 { state = 61; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            102 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 75 || lookahead == 107 { state = 62; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            103 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 76 || lookahead == 108 { state = 122; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            104 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 76 || lookahead == 108 { state = 130; lexer.advance(false); continue; }
                if lookahead == 78 || lookahead == 110 { state = 79; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            105 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 76 || lookahead == 108 { state = 82; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            106 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 76 || lookahead == 108 { state = 84; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            107 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 76 || lookahead == 108 { state = 124; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            108 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 77 || lookahead == 109 { state = 66; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            109 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 78 || lookahead == 110 { state = 57; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            110 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 78 || lookahead == 110 { state = 58; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            111 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 78 || lookahead == 110 { state = 76; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            112 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 78 || lookahead == 110 { state = 69; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            113 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 78 || lookahead == 110 { state = 80; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            114 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 78 || lookahead == 110 { state = 77; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            115 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 78 || lookahead == 110 { state = 78; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            116 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 78 || lookahead == 110 { state = 81; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            117 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 78 || lookahead == 110 { state = 75; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            118 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 79 || lookahead == 111 { state = 126; lexer.advance(false); continue; }
                if lookahead == 85 || lookahead == 117 { state = 112; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            119 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 79 || lookahead == 111 { state = 59; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            120 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 79 || lookahead == 111 { state = 60; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            121 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 79 || lookahead == 111 { state = 109; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            122 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 79 || lookahead == 111 { state = 68; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            123 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 79 || lookahead == 111 { state = 110; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            124 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 79 || lookahead == 111 { state = 72; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            125 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 79 || lookahead == 111 { state = 129; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            126 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 82 || lookahead == 114 { state = 85; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            127 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 82 || lookahead == 114 { state = 119; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            128 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 82 || lookahead == 114 { state = 120; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            129 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 82 || lookahead == 114 { state = 86; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            130 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 83 || lookahead == 115 { state = 83; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            131 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 84 || lookahead == 116 { state = 98; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            132 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 84 || lookahead == 116 { state = 99; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            133 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 85 || lookahead == 117 { state = 117; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            134 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 87 || lookahead == 119 { state = 95; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            135 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            _ => return false,
        }
    }
}
