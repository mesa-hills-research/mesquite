//! The `toml` grammar's lexer: `ts_lex`, transliterated from
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

const anon_sym_COMMA: Symbol = 32;
const anon_sym_DOT: Symbol = 8;
const anon_sym_DQUOTE: Symbol = 10;
const anon_sym_DQUOTE2: Symbol = 12;
const anon_sym_DQUOTE_DQUOTE_DQUOTE: Symbol = 13;
const anon_sym_EQ: Symbol = 7;
const anon_sym_LBRACE: Symbol = 33;
const anon_sym_LBRACK: Symbol = 3;
const anon_sym_LBRACK_LBRACK: Symbol = 5;
const anon_sym_RBRACE: Symbol = 34;
const anon_sym_RBRACK: Symbol = 4;
const anon_sym_RBRACK_RBRACK: Symbol = 6;
const anon_sym_SQUOTE: Symbol = 17;
const anon_sym_SQUOTE2: Symbol = 19;
const anon_sym_SQUOTE_SQUOTE_SQUOTE: Symbol = 20;
const aux_sym__basic_string_token1: Symbol = 11;
const aux_sym__literal_string_token1: Symbol = 18;
const aux_sym__multiline_basic_string_token1: Symbol = 14;
const aux_sym_document_token1: Symbol = 1;
const aux_sym_float_token1: Symbol = 25;
const aux_sym_float_token2: Symbol = 26;
const aux_sym_integer_token1: Symbol = 21;
const aux_sym_integer_token2: Symbol = 22;
const aux_sym_integer_token3: Symbol = 23;
const aux_sym_integer_token4: Symbol = 24;
const sym__escape_line_ending: Symbol = 16;
const sym_bare_key: Symbol = 9;
const sym_boolean: Symbol = 27;
const sym_comment: Symbol = 2;
const sym_escape_sequence: Symbol = 15;
const sym_local_date: Symbol = 30;
const sym_local_date_time: Symbol = 29;
const sym_local_time: Symbol = 31;
const sym_offset_date_time: Symbol = 28;
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
                if eof { state = 77; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (10, 129), (13, 1), (34, 127), (35, 79), (39, 137), (43, 15), (44, 161), (45, 95),
                    (46, 86), (48, 92), (49, 90), (50, 89), (61, 85), (91, 81), (92, 5), (93, 82),
                    (102, 102), (105, 108), (110, 103), (116, 109), (123, 162), (125, 163),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 75; lexer.advance(true); continue; }
                if 51 <= lookahead && lookahead <= 57 { state = 91; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            1 => {
                if lookahead == 10 { state = 129; lexer.advance(false); continue; }
                return result;
            }
            2 => {
                if lookahead == 10 { state = 129; lexer.advance(false); continue; }
                if lookahead == 13 { state = 1; lexer.advance(false); continue; }
                if lookahead == 34 { state = 126; lexer.advance(false); continue; }
                if lookahead == 35 { state = 125; lexer.advance(false); continue; }
                if lookahead == 92 { state = 5; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 124; lexer.advance(false); continue; }
                if lookahead > 32 && lookahead != 127 { state = 125; lexer.advance(false); continue; }
                return result;
            }
            3 => {
                if lookahead == 10 { state = 129; lexer.advance(false); continue; }
                if lookahead == 13 { state = 1; lexer.advance(false); continue; }
                if lookahead == 35 { state = 135; lexer.advance(false); continue; }
                if lookahead == 39 { state = 136; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 134; lexer.advance(false); continue; }
                if lookahead > 32 && lookahead != 127 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            4 => {
                if lookahead == 10 { state = 131; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                if let Some(next) = advance_map(&[
                    (10, 131), (13, 4), (85, 74), (117, 70), (34, 130), (92, 130), (98, 130), (102, 130),
                    (110, 130), (114, 130), (116, 130),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            6 => {
                if lookahead == 10 { state = 78; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if let Some(next) = advance_map(&[
                    (10, 78), (13, 6), (34, 123), (35, 79), (39, 133), (44, 161), (48, 144), (49, 142),
                    (50, 141), (91, 80), (93, 82), (102, 30), (105, 35), (110, 31), (116, 37), (123, 162),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 7; lexer.advance(true); continue; }
                if 43 <= lookahead && lookahead <= 45 { state = 17; lexer.advance(false); continue; }
                if 51 <= lookahead && lookahead <= 57 { state = 143; lexer.advance(false); continue; }
                return result;
            }
            8 => {
                if lookahead == 34 { state = 128; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if lookahead == 35 { state = 79; lexer.advance(false); continue; }
                if lookahead == 39 { state = 136; lexer.advance(false); continue; }
                if lookahead == 46 { state = 86; lexer.advance(false); continue; }
                if lookahead == 93 { state = 29; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 10; lexer.advance(true); continue; }
                return result;
            }
            10 => {
                if lookahead == 35 { state = 79; lexer.advance(false); continue; }
                if lookahead == 46 { state = 86; lexer.advance(false); continue; }
                if lookahead == 93 { state = 29; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 10; lexer.advance(true); continue; }
                return result;
            }
            11 => {
                if lookahead == 39 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if lookahead == 45 { state = 18; lexer.advance(false); continue; }
                if lookahead == 58 { state = 46; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 13; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if lookahead == 45 { state = 18; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 13; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if lookahead == 45 { state = 20; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if lookahead == 48 { state = 139; lexer.advance(false); continue; }
                if lookahead == 105 { state = 35; lexer.advance(false); continue; }
                if lookahead == 110 { state = 31; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 147; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if lookahead == 48 { state = 159; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if lookahead == 48 { state = 146; lexer.advance(false); continue; }
                if lookahead == 105 { state = 35; lexer.advance(false); continue; }
                if lookahead == 110 { state = 31; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 145; lexer.advance(false); continue; }
                return result;
            }
            18 => {
                if lookahead == 48 { state = 50; lexer.advance(false); continue; }
                if lookahead == 49 { state = 43; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if lookahead == 48 { state = 156; lexer.advance(false); continue; }
                return result;
            }
            20 => {
                if lookahead == 48 { state = 51; lexer.advance(false); continue; }
                if lookahead == 51 { state = 42; lexer.advance(false); continue; }
                if lookahead == 49 || lookahead == 50 { state = 59; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                if lookahead == 50 { state = 44; lexer.advance(false); continue; }
                if lookahead == 48 || lookahead == 49 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            22 => {
                if lookahead == 50 { state = 45; lexer.advance(false); continue; }
                if lookahead == 48 || lookahead == 49 { state = 64; lexer.advance(false); continue; }
                return result;
            }
            23 => {
                if lookahead == 54 { state = 16; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 53 { state = 54; lexer.advance(false); continue; }
                return result;
            }
            24 => {
                if lookahead == 54 { state = 19; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 53 { state = 60; lexer.advance(false); continue; }
                return result;
            }
            25 => {
                if lookahead == 58 { state = 23; lexer.advance(false); continue; }
                return result;
            }
            26 => {
                if lookahead == 58 { state = 47; lexer.advance(false); continue; }
                return result;
            }
            27 => {
                if lookahead == 58 { state = 24; lexer.advance(false); continue; }
                return result;
            }
            28 => {
                if lookahead == 58 { state = 48; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                if lookahead == 93 { state = 84; lexer.advance(false); continue; }
                return result;
            }
            30 => {
                if lookahead == 97 { state = 34; lexer.advance(false); continue; }
                return result;
            }
            31 => {
                if lookahead == 97 { state = 36; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                if lookahead == 101 { state = 154; lexer.advance(false); continue; }
                return result;
            }
            33 => {
                if lookahead == 102 { state = 153; lexer.advance(false); continue; }
                return result;
            }
            34 => {
                if lookahead == 108 { state = 38; lexer.advance(false); continue; }
                return result;
            }
            35 => {
                if lookahead == 110 { state = 33; lexer.advance(false); continue; }
                return result;
            }
            36 => {
                if lookahead == 110 { state = 153; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                if lookahead == 114 { state = 39; lexer.advance(false); continue; }
                return result;
            }
            38 => {
                if lookahead == 115 { state = 32; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                if lookahead == 117 { state = 32; lexer.advance(false); continue; }
                return result;
            }
            40 => {
                if lookahead == 43 || lookahead == 45 { state = 58; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 152; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                if lookahead == 48 || lookahead == 49 { state = 150; lexer.advance(false); continue; }
                return result;
            }
            42 => {
                if lookahead == 48 || lookahead == 49 { state = 158; lexer.advance(false); continue; }
                return result;
            }
            43 => {
                if 48 <= lookahead && lookahead <= 50 { state = 14; lexer.advance(false); continue; }
                return result;
            }
            44 => {
                if 48 <= lookahead && lookahead <= 51 { state = 28; lexer.advance(false); continue; }
                return result;
            }
            45 => {
                if 48 <= lookahead && lookahead <= 51 { state = 26; lexer.advance(false); continue; }
                return result;
            }
            46 => {
                if 48 <= lookahead && lookahead <= 53 { state = 53; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                if 48 <= lookahead && lookahead <= 53 { state = 61; lexer.advance(false); continue; }
                return result;
            }
            48 => {
                if 48 <= lookahead && lookahead <= 53 { state = 65; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                if 48 <= lookahead && lookahead <= 55 { state = 149; lexer.advance(false); continue; }
                return result;
            }
            50 => {
                if 49 <= lookahead && lookahead <= 57 { state = 14; lexer.advance(false); continue; }
                return result;
            }
            51 => {
                if 49 <= lookahead && lookahead <= 57 { state = 158; lexer.advance(false); continue; }
                return result;
            }
            52 => {
                if 48 <= lookahead && lookahead <= 57 { state = 147; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                if 48 <= lookahead && lookahead <= 57 { state = 25; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                if 48 <= lookahead && lookahead <= 57 { state = 159; lexer.advance(false); continue; }
                return result;
            }
            55 => {
                if 48 <= lookahead && lookahead <= 57 { state = 160; lexer.advance(false); continue; }
                return result;
            }
            56 => {
                if 48 <= lookahead && lookahead <= 57 { state = 145; lexer.advance(false); continue; }
                return result;
            }
            57 => {
                if 48 <= lookahead && lookahead <= 57 { state = 151; lexer.advance(false); continue; }
                return result;
            }
            58 => {
                if 48 <= lookahead && lookahead <= 57 { state = 152; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                if 48 <= lookahead && lookahead <= 57 { state = 158; lexer.advance(false); continue; }
                return result;
            }
            60 => {
                if 48 <= lookahead && lookahead <= 57 { state = 156; lexer.advance(false); continue; }
                return result;
            }
            61 => {
                if 48 <= lookahead && lookahead <= 57 { state = 155; lexer.advance(false); continue; }
                return result;
            }
            62 => {
                if 48 <= lookahead && lookahead <= 57 { state = 157; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                if 48 <= lookahead && lookahead <= 57 { state = 28; lexer.advance(false); continue; }
                return result;
            }
            64 => {
                if 48 <= lookahead && lookahead <= 57 { state = 26; lexer.advance(false); continue; }
                return result;
            }
            65 => {
                if 48 <= lookahead && lookahead <= 57 { state = 27; lexer.advance(false); continue; }
                return result;
            }
            66 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 130; lexer.advance(false); continue; }
                return result;
            }
            67 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 148; lexer.advance(false); continue; }
                return result;
            }
            68 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 66; lexer.advance(false); continue; }
                return result;
            }
            69 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 68; lexer.advance(false); continue; }
                return result;
            }
            70 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 69; lexer.advance(false); continue; }
                return result;
            }
            71 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 70; lexer.advance(false); continue; }
                return result;
            }
            72 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 71; lexer.advance(false); continue; }
                return result;
            }
            73 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 72; lexer.advance(false); continue; }
                return result;
            }
            74 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 73; lexer.advance(false); continue; }
                return result;
            }
            75 => {
                if eof { state = 77; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (10, 78), (13, 6), (34, 123), (35, 79), (39, 133), (43, 15), (44, 161), (45, 95),
                    (46, 86), (48, 92), (49, 90), (50, 89), (61, 85), (91, 81), (93, 82), (102, 102),
                    (105, 108), (110, 103), (116, 109), (123, 162), (125, 163),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 75; lexer.advance(true); continue; }
                if 51 <= lookahead && lookahead <= 57 { state = 91; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            76 => {
                if eof { state = 77; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (10, 78), (13, 6), (34, 122), (35, 79), (39, 132), (44, 161), (46, 86), (61, 85),
                    (91, 81), (93, 82), (125, 163),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 76; lexer.advance(true); continue; }
                if lookahead == 45 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            77 => {
                result = true; lexer.set_result_symbol(ts_builtin_sym_end); lexer.mark_end();
                return result;
            }
            78 => {
                result = true; lexer.set_result_symbol(aux_sym_document_token1); lexer.mark_end();
                return result;
            }
            79 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead > 8 && (lookahead < 10 || 31 < lookahead) && lookahead != 127 { state = 79; lexer.advance(false); continue; }
                return result;
            }
            80 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK); lexer.mark_end();
                return result;
            }
            81 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK); lexer.mark_end();
                if lookahead == 91 { state = 83; lexer.advance(false); continue; }
                return result;
            }
            82 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACK); lexer.mark_end();
                return result;
            }
            83 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK_LBRACK); lexer.mark_end();
                return result;
            }
            84 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACK_RBRACK); lexer.mark_end();
                return result;
            }
            85 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                return result;
            }
            86 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                return result;
            }
            87 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if lookahead == 45 { state = 96; lexer.advance(false); continue; }
                if lookahead == 58 { state = 46; lexer.advance(false); continue; }
                if lookahead == 95 { state = 119; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 91; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            88 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if lookahead == 45 { state = 96; lexer.advance(false); continue; }
                if lookahead == 58 { state = 46; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 93; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            89 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if lookahead == 45 { state = 96; lexer.advance(false); continue; }
                if lookahead == 95 { state = 119; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 51 { state = 87; lexer.advance(false); continue; }
                if 52 <= lookahead && lookahead <= 57 { state = 91; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            90 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if lookahead == 45 { state = 96; lexer.advance(false); continue; }
                if lookahead == 95 { state = 119; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 87; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            91 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if lookahead == 45 { state = 96; lexer.advance(false); continue; }
                if lookahead == 95 { state = 119; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 91; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            92 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if lookahead == 45 { state = 96; lexer.advance(false); continue; }
                if lookahead == 98 { state = 113; lexer.advance(false); continue; }
                if lookahead == 111 { state = 115; lexer.advance(false); continue; }
                if lookahead == 120 { state = 120; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 88; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            93 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if lookahead == 45 { state = 96; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 93; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            94 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if lookahead == 45 { state = 97; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            95 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if lookahead == 48 { state = 121; lexer.advance(false); continue; }
                if lookahead == 105 { state = 108; lexer.advance(false); continue; }
                if lookahead == 110 { state = 103; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 101; lexer.advance(false); continue; }
                if lookahead == 45 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            96 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if lookahead == 48 { state = 117; lexer.advance(false); continue; }
                if lookahead == 49 { state = 114; lexer.advance(false); continue; }
                if lookahead == 45 || 50 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            97 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if lookahead == 48 { state = 116; lexer.advance(false); continue; }
                if lookahead == 51 { state = 112; lexer.advance(false); continue; }
                if lookahead == 49 || lookahead == 50 { state = 118; lexer.advance(false); continue; }
                if lookahead == 45 || 52 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            98 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if lookahead == 95 { state = 113; lexer.advance(false); continue; }
                if lookahead == 48 || lookahead == 49 { state = 98; lexer.advance(false); continue; }
                if lookahead == 45 || 50 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            99 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if lookahead == 95 { state = 115; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 55 { state = 99; lexer.advance(false); continue; }
                if lookahead == 45 || lookahead == 56 || lookahead == 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            100 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if lookahead == 95 { state = 120; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 100; lexer.advance(false); continue; }
                if lookahead == 45 || 71 <= lookahead && lookahead <= 90 || 103 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            101 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if lookahead == 95 { state = 119; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 101; lexer.advance(false); continue; }
                if lookahead == 45 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            102 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if lookahead == 97 { state = 106; lexer.advance(false); continue; }
                if lookahead == 45 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            103 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if lookahead == 97 { state = 107; lexer.advance(false); continue; }
                if lookahead == 45 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            104 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if lookahead == 101 { state = 121; lexer.advance(false); continue; }
                if lookahead == 45 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            105 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if lookahead == 102 { state = 121; lexer.advance(false); continue; }
                if lookahead == 45 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            106 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if lookahead == 108 { state = 110; lexer.advance(false); continue; }
                if lookahead == 45 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            107 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if lookahead == 110 { state = 121; lexer.advance(false); continue; }
                if lookahead == 45 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            108 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if lookahead == 110 { state = 105; lexer.advance(false); continue; }
                if lookahead == 45 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            109 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if lookahead == 114 { state = 111; lexer.advance(false); continue; }
                if lookahead == 45 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            110 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if lookahead == 115 { state = 104; lexer.advance(false); continue; }
                if lookahead == 45 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            111 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if lookahead == 117 { state = 104; lexer.advance(false); continue; }
                if lookahead == 45 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            112 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if lookahead == 48 || lookahead == 49 { state = 121; lexer.advance(false); continue; }
                if lookahead == 45 || 50 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            113 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if lookahead == 48 || lookahead == 49 { state = 98; lexer.advance(false); continue; }
                if lookahead == 45 || 50 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            114 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 50 { state = 94; lexer.advance(false); continue; }
                if lookahead == 45 || 51 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            115 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 55 { state = 99; lexer.advance(false); continue; }
                if lookahead == 45 || lookahead == 56 || lookahead == 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            116 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if 49 <= lookahead && lookahead <= 57 { state = 121; lexer.advance(false); continue; }
                if lookahead == 45 || lookahead == 48 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            117 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if 49 <= lookahead && lookahead <= 57 { state = 94; lexer.advance(false); continue; }
                if lookahead == 45 || lookahead == 48 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            118 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 { state = 121; lexer.advance(false); continue; }
                if lookahead == 45 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            119 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 { state = 101; lexer.advance(false); continue; }
                if lookahead == 45 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            120 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 100; lexer.advance(false); continue; }
                if lookahead == 45 || 71 <= lookahead && lookahead <= 90 || lookahead == 95 || 103 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            121 => {
                result = true; lexer.set_result_symbol(sym_bare_key); lexer.mark_end();
                if lookahead == 45 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            122 => {
                result = true; lexer.set_result_symbol(anon_sym_DQUOTE); lexer.mark_end();
                return result;
            }
            123 => {
                result = true; lexer.set_result_symbol(anon_sym_DQUOTE); lexer.mark_end();
                if lookahead == 34 { state = 8; lexer.advance(false); continue; }
                return result;
            }
            124 => {
                result = true; lexer.set_result_symbol(aux_sym__basic_string_token1); lexer.mark_end();
                if lookahead == 35 { state = 125; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 124; lexer.advance(false); continue; }
                if lookahead > 32 && lookahead != 34 && lookahead != 35 && lookahead != 92 && lookahead != 127 { state = 125; lexer.advance(false); continue; }
                return result;
            }
            125 => {
                result = true; lexer.set_result_symbol(aux_sym__basic_string_token1); lexer.mark_end();
                if lookahead > 8 && (lookahead < 10 || 31 < lookahead) && lookahead != 34 && lookahead != 92 && lookahead != 127 { state = 125; lexer.advance(false); continue; }
                return result;
            }
            126 => {
                result = true; lexer.set_result_symbol(anon_sym_DQUOTE2); lexer.mark_end();
                return result;
            }
            127 => {
                result = true; lexer.set_result_symbol(anon_sym_DQUOTE2); lexer.mark_end();
                if lookahead == 34 { state = 8; lexer.advance(false); continue; }
                return result;
            }
            128 => {
                result = true; lexer.set_result_symbol(anon_sym_DQUOTE_DQUOTE_DQUOTE); lexer.mark_end();
                return result;
            }
            129 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_basic_string_token1); lexer.mark_end();
                return result;
            }
            130 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                return result;
            }
            131 => {
                result = true; lexer.set_result_symbol(sym__escape_line_ending); lexer.mark_end();
                return result;
            }
            132 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTE); lexer.mark_end();
                return result;
            }
            133 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTE); lexer.mark_end();
                if lookahead == 39 { state = 11; lexer.advance(false); continue; }
                return result;
            }
            134 => {
                result = true; lexer.set_result_symbol(aux_sym__literal_string_token1); lexer.mark_end();
                if lookahead == 35 { state = 135; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 134; lexer.advance(false); continue; }
                if lookahead > 32 && lookahead != 39 && lookahead != 127 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            135 => {
                result = true; lexer.set_result_symbol(aux_sym__literal_string_token1); lexer.mark_end();
                if lookahead > 8 && (lookahead < 10 || 31 < lookahead) && lookahead != 39 && lookahead != 127 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            136 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTE2); lexer.mark_end();
                return result;
            }
            137 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTE2); lexer.mark_end();
                if lookahead == 39 { state = 11; lexer.advance(false); continue; }
                return result;
            }
            138 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTE_SQUOTE_SQUOTE); lexer.mark_end();
                return result;
            }
            139 => {
                result = true; lexer.set_result_symbol(aux_sym_integer_token1); lexer.mark_end();
                return result;
            }
            140 => {
                result = true; lexer.set_result_symbol(aux_sym_integer_token1); lexer.mark_end();
                if lookahead == 45 { state = 18; lexer.advance(false); continue; }
                if lookahead == 46 { state = 57; lexer.advance(false); continue; }
                if lookahead == 58 { state = 46; lexer.advance(false); continue; }
                if lookahead == 95 { state = 56; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 40; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 143; lexer.advance(false); continue; }
                return result;
            }
            141 => {
                result = true; lexer.set_result_symbol(aux_sym_integer_token1); lexer.mark_end();
                if lookahead == 45 { state = 18; lexer.advance(false); continue; }
                if lookahead == 46 { state = 57; lexer.advance(false); continue; }
                if lookahead == 95 { state = 56; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 40; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 51 { state = 140; lexer.advance(false); continue; }
                if 52 <= lookahead && lookahead <= 57 { state = 143; lexer.advance(false); continue; }
                return result;
            }
            142 => {
                result = true; lexer.set_result_symbol(aux_sym_integer_token1); lexer.mark_end();
                if lookahead == 45 { state = 18; lexer.advance(false); continue; }
                if lookahead == 46 { state = 57; lexer.advance(false); continue; }
                if lookahead == 95 { state = 56; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 40; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 140; lexer.advance(false); continue; }
                return result;
            }
            143 => {
                result = true; lexer.set_result_symbol(aux_sym_integer_token1); lexer.mark_end();
                if lookahead == 45 { state = 18; lexer.advance(false); continue; }
                if lookahead == 46 { state = 57; lexer.advance(false); continue; }
                if lookahead == 95 { state = 56; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 40; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 143; lexer.advance(false); continue; }
                return result;
            }
            144 => {
                result = true; lexer.set_result_symbol(aux_sym_integer_token1); lexer.mark_end();
                if lookahead == 45 { state = 18; lexer.advance(false); continue; }
                if lookahead == 46 { state = 57; lexer.advance(false); continue; }
                if lookahead == 98 { state = 41; lexer.advance(false); continue; }
                if lookahead == 111 { state = 49; lexer.advance(false); continue; }
                if lookahead == 120 { state = 67; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 40; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 12; lexer.advance(false); continue; }
                return result;
            }
            145 => {
                result = true; lexer.set_result_symbol(aux_sym_integer_token1); lexer.mark_end();
                if lookahead == 46 { state = 57; lexer.advance(false); continue; }
                if lookahead == 95 { state = 56; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 40; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 145; lexer.advance(false); continue; }
                return result;
            }
            146 => {
                result = true; lexer.set_result_symbol(aux_sym_integer_token1); lexer.mark_end();
                if lookahead == 46 { state = 57; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 40; lexer.advance(false); continue; }
                return result;
            }
            147 => {
                result = true; lexer.set_result_symbol(aux_sym_integer_token1); lexer.mark_end();
                if lookahead == 95 { state = 52; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 147; lexer.advance(false); continue; }
                return result;
            }
            148 => {
                result = true; lexer.set_result_symbol(aux_sym_integer_token2); lexer.mark_end();
                if lookahead == 95 { state = 67; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 148; lexer.advance(false); continue; }
                return result;
            }
            149 => {
                result = true; lexer.set_result_symbol(aux_sym_integer_token3); lexer.mark_end();
                if lookahead == 95 { state = 49; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 55 { state = 149; lexer.advance(false); continue; }
                return result;
            }
            150 => {
                result = true; lexer.set_result_symbol(aux_sym_integer_token4); lexer.mark_end();
                if lookahead == 95 { state = 41; lexer.advance(false); continue; }
                if lookahead == 48 || lookahead == 49 { state = 150; lexer.advance(false); continue; }
                return result;
            }
            151 => {
                result = true; lexer.set_result_symbol(aux_sym_float_token1); lexer.mark_end();
                if lookahead == 95 { state = 57; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 40; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 151; lexer.advance(false); continue; }
                return result;
            }
            152 => {
                result = true; lexer.set_result_symbol(aux_sym_float_token1); lexer.mark_end();
                if lookahead == 95 { state = 58; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 152; lexer.advance(false); continue; }
                return result;
            }
            153 => {
                result = true; lexer.set_result_symbol(aux_sym_float_token2); lexer.mark_end();
                return result;
            }
            154 => {
                result = true; lexer.set_result_symbol(sym_boolean); lexer.mark_end();
                return result;
            }
            155 => {
                result = true; lexer.set_result_symbol(sym_offset_date_time); lexer.mark_end();
                return result;
            }
            156 => {
                result = true; lexer.set_result_symbol(sym_local_date_time); lexer.mark_end();
                if lookahead == 46 { state = 62; lexer.advance(false); continue; }
                if lookahead == 43 || lookahead == 45 { state = 22; lexer.advance(false); continue; }
                if lookahead == 90 || lookahead == 122 { state = 155; lexer.advance(false); continue; }
                return result;
            }
            157 => {
                result = true; lexer.set_result_symbol(sym_local_date_time); lexer.mark_end();
                if lookahead == 43 || lookahead == 45 { state = 22; lexer.advance(false); continue; }
                if lookahead == 90 || lookahead == 122 { state = 155; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 157; lexer.advance(false); continue; }
                return result;
            }
            158 => {
                result = true; lexer.set_result_symbol(sym_local_date); lexer.mark_end();
                if lookahead == 32 || lookahead == 84 || lookahead == 116 { state = 21; lexer.advance(false); continue; }
                return result;
            }
            159 => {
                result = true; lexer.set_result_symbol(sym_local_time); lexer.mark_end();
                if lookahead == 46 { state = 55; lexer.advance(false); continue; }
                return result;
            }
            160 => {
                result = true; lexer.set_result_symbol(sym_local_time); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 { state = 160; lexer.advance(false); continue; }
                return result;
            }
            161 => {
                result = true; lexer.set_result_symbol(anon_sym_COMMA); lexer.mark_end();
                return result;
            }
            162 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACE); lexer.mark_end();
                return result;
            }
            163 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACE); lexer.mark_end();
                return result;
            }
            _ => return false,
        }
    }
}
