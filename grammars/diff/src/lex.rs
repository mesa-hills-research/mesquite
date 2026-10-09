//! The `diff` grammar's lexer: `ts_lex`, transliterated from
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

const anon_sym_AT_AT: Symbol = 24;
const anon_sym_AT_AT2: Symbol = 25;
const anon_sym_Binary: Symbol = 12;
const anon_sym_DASH: Symbol = 30;
const anon_sym_DASH_DASH: Symbol = 31;
const anon_sym_DASH_DASH_DASH: Symbol = 22;
const anon_sym_DASH_DASH_DASH_DASH: Symbol = 32;
const anon_sym_DOT_DOT: Symbol = 17;
const anon_sym_PERCENT: Symbol = 21;
const anon_sym_PLUS: Symbol = 27;
const anon_sym_PLUS_PLUS: Symbol = 28;
const anon_sym_PLUS_PLUS_PLUS: Symbol = 23;
const anon_sym_PLUS_PLUS_PLUS_PLUS: Symbol = 29;
const anon_sym_POUND: Symbol = 34;
const anon_sym_and: Symbol = 14;
const anon_sym_deleted: Symbol = 5;
const anon_sym_diff: Symbol = 2;
const anon_sym_differ: Symbol = 15;
const anon_sym_file: Symbol = 6;
const anon_sym_files: Symbol = 13;
const anon_sym_from: Symbol = 10;
const anon_sym_index: Symbol = 16;
const anon_sym_index2: Symbol = 19;
const anon_sym_mode: Symbol = 7;
const anon_sym_new: Symbol = 4;
const anon_sym_old: Symbol = 8;
const anon_sym_rename: Symbol = 9;
const anon_sym_similarity: Symbol = 18;
const anon_sym_to: Symbol = 11;
const aux_sym_command_token1: Symbol = 3;
const aux_sym_filename_token1: Symbol = 36;
const aux_sym_location_token1: Symbol = 26;
const aux_sym_similarity_token1: Symbol = 20;
const aux_sym_source_token1: Symbol = 1;
const sym_commit: Symbol = 37;
const sym_context: Symbol = 33;
const sym_linerange: Symbol = 35;
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
                if eof { state = 88; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (10, 89), (13, 1), (35, 204), (37, 151), (43, 159), (45, 162), (46, 4), (64, 5),
                    (66, 38), (97, 54), (100, 18), (102, 39), (105, 56), (109, 61), (110, 20), (111, 46),
                    (114, 29), (115, 37), (116, 59), (98, 81), (99, 81), (101, 81),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 83; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            1 => {
                if lookahead == 10 { state = 89; lexer.advance(false); continue; }
                return result;
            }
            2 => {
                if lookahead == 43 { state = 3; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 205; lexer.advance(false); continue; }
                return result;
            }
            3 => {
                if lookahead == 43 { state = 153; lexer.advance(false); continue; }
                return result;
            }
            4 => {
                if lookahead == 46 { state = 108; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                if lookahead == 64 { state = 155; lexer.advance(false); continue; }
                return result;
            }
            6 => {
                if lookahead == 64 { state = 156; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if lookahead == 97 { state = 62; lexer.advance(false); continue; }
                return result;
            }
            8 => {
                if lookahead == 97 { state = 212; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 11 || lookahead == 12 || lookahead == 32 { state = 8; lexer.advance(true); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) { state = 214; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if lookahead == 97 { state = 64; lexer.advance(false); continue; }
                return result;
            }
            10 => {
                if lookahead == 97 { state = 53; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if lookahead == 100 { state = 103; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if lookahead == 100 { state = 97; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if lookahead == 100 { state = 93; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if lookahead == 100 { state = 211; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 11 || lookahead == 12 || lookahead == 32 { state = 14; lexer.advance(true); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) { state = 214; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if lookahead == 100 { state = 23; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if lookahead == 100 { state = 24; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if lookahead == 100 { state = 28; lexer.advance(false); continue; }
                return result;
            }
            18 => {
                if lookahead == 101 { state = 45; lexer.advance(false); continue; }
                if lookahead == 105 { state = 34; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 80; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if lookahead == 101 { state = 45; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 80; lexer.advance(false); continue; }
                return result;
            }
            20 => {
                if lookahead == 101 { state = 68; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                if lookahead == 101 { state = 67; lexer.advance(false); continue; }
                return result;
            }
            22 => {
                if lookahead == 101 { state = 95; lexer.advance(false); continue; }
                return result;
            }
            23 => {
                if lookahead == 101 { state = 69; lexer.advance(false); continue; }
                return result;
            }
            24 => {
                if lookahead == 101 { state = 96; lexer.advance(false); continue; }
                return result;
            }
            25 => {
                if lookahead == 101 { state = 98; lexer.advance(false); continue; }
                return result;
            }
            26 => {
                if lookahead == 101 { state = 94; lexer.advance(false); continue; }
                return result;
            }
            27 => {
                if lookahead == 101 { state = 65; lexer.advance(false); continue; }
                return result;
            }
            28 => {
                if lookahead == 101 { state = 70; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                if lookahead == 101 { state = 57; lexer.advance(false); continue; }
                return result;
            }
            30 => {
                if lookahead == 101 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            31 => {
                if lookahead == 101 { state = 13; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                if lookahead == 102 { state = 90; lexer.advance(false); continue; }
                return result;
            }
            33 => {
                if lookahead == 102 { state = 44; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 11 || lookahead == 12 || lookahead == 32 { state = 33; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 101 { state = 81; lexer.advance(false); continue; }
                return result;
            }
            34 => {
                if lookahead == 102 { state = 32; lexer.advance(false); continue; }
                return result;
            }
            35 => {
                if lookahead == 102 { state = 36; lexer.advance(false); continue; }
                return result;
            }
            36 => {
                if lookahead == 102 { state = 30; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                if lookahead == 105 { state = 52; lexer.advance(false); continue; }
                return result;
            }
            38 => {
                if lookahead == 105 { state = 55; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                if lookahead == 105 { state = 47; lexer.advance(false); continue; }
                if lookahead == 114 { state = 60; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 80; lexer.advance(false); continue; }
                return result;
            }
            40 => {
                if lookahead == 105 { state = 66; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                if lookahead == 105 { state = 50; lexer.advance(false); continue; }
                return result;
            }
            42 => {
                if lookahead == 105 { state = 35; lexer.advance(false); continue; }
                return result;
            }
            43 => {
                if lookahead == 105 { state = 48; lexer.advance(false); continue; }
                return result;
            }
            44 => {
                if lookahead == 105 { state = 49; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 80; lexer.advance(false); continue; }
                return result;
            }
            45 => {
                if lookahead == 108 { state = 21; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 79; lexer.advance(false); continue; }
                return result;
            }
            46 => {
                if lookahead == 108 { state = 12; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                if lookahead == 108 { state = 22; lexer.advance(false); continue; }
                return result;
            }
            48 => {
                if lookahead == 108 { state = 26; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                if lookahead == 108 { state = 27; lexer.advance(false); continue; }
                return result;
            }
            50 => {
                if lookahead == 108 { state = 9; lexer.advance(false); continue; }
                return result;
            }
            51 => {
                if lookahead == 109 { state = 99; lexer.advance(false); continue; }
                return result;
            }
            52 => {
                if lookahead == 109 { state = 41; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                if lookahead == 109 { state = 25; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                if lookahead == 110 { state = 11; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 80; lexer.advance(false); continue; }
                return result;
            }
            55 => {
                if lookahead == 110 { state = 7; lexer.advance(false); continue; }
                return result;
            }
            56 => {
                if lookahead == 110 { state = 15; lexer.advance(false); continue; }
                return result;
            }
            57 => {
                if lookahead == 110 { state = 10; lexer.advance(false); continue; }
                return result;
            }
            58 => {
                if lookahead == 110 { state = 17; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                if lookahead == 111 { state = 100; lexer.advance(false); continue; }
                return result;
            }
            60 => {
                if lookahead == 111 { state = 51; lexer.advance(false); continue; }
                return result;
            }
            61 => {
                if lookahead == 111 { state = 16; lexer.advance(false); continue; }
                return result;
            }
            62 => {
                if lookahead == 114 { state = 71; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                if lookahead == 114 { state = 105; lexer.advance(false); continue; }
                return result;
            }
            64 => {
                if lookahead == 114 { state = 40; lexer.advance(false); continue; }
                return result;
            }
            65 => {
                if lookahead == 115 { state = 102; lexer.advance(false); continue; }
                return result;
            }
            66 => {
                if lookahead == 116 { state = 72; lexer.advance(false); continue; }
                return result;
            }
            67 => {
                if lookahead == 116 { state = 31; lexer.advance(false); continue; }
                return result;
            }
            68 => {
                if lookahead == 119 { state = 92; lexer.advance(false); continue; }
                return result;
            }
            69 => {
                if lookahead == 120 { state = 107; lexer.advance(false); continue; }
                return result;
            }
            70 => {
                if lookahead == 120 { state = 110; lexer.advance(false); continue; }
                return result;
            }
            71 => {
                if lookahead == 121 { state = 101; lexer.advance(false); continue; }
                return result;
            }
            72 => {
                if lookahead == 121 { state = 109; lexer.advance(false); continue; }
                return result;
            }
            73 => {
                if lookahead == 9 || lookahead == 11 || lookahead == 12 || lookahead == 32 { state = 73; lexer.advance(true); continue; }
                if lookahead == 45 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 91; lexer.advance(false); continue; }
                return result;
            }
            74 => {
                if 48 <= lookahead && lookahead <= 57 { state = 205; lexer.advance(false); continue; }
                return result;
            }
            75 => {
                if 48 <= lookahead && lookahead <= 57 { state = 206; lexer.advance(false); continue; }
                return result;
            }
            76 => {
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 248; lexer.advance(false); continue; }
                return result;
            }
            77 => {
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 76; lexer.advance(false); continue; }
                return result;
            }
            78 => {
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 77; lexer.advance(false); continue; }
                return result;
            }
            79 => {
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 78; lexer.advance(false); continue; }
                return result;
            }
            80 => {
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 79; lexer.advance(false); continue; }
                return result;
            }
            81 => {
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 80; lexer.advance(false); continue; }
                return result;
            }
            82 => {
                if eof { state = 88; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (10, 89), (13, 1), (35, 204), (43, 159), (45, 162), (64, 166), (66, 184), (100, 177),
                    (105, 192), (110, 174), (111, 187), (114, 179), (115, 183), (9, 165), (11, 165), (12, 165),
                    (32, 165),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead != 0 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            83 => {
                if eof { state = 88; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (10, 89), (13, 1), (37, 151), (46, 4), (64, 6), (97, 54), (100, 19), (102, 39),
                    (105, 58), (109, 61), (110, 20), (111, 46), (114, 29), (116, 59), (98, 81), (99, 81),
                    (101, 81),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 83; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            84 => {
                if eof { state = 88; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (10, 89), (13, 1), (43, 2), (45, 74), (64, 6), (100, 42), (102, 43), (105, 58),
                    (109, 61),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 85; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 150; lexer.advance(false); continue; }
                return result;
            }
            85 => {
                if eof { state = 88; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (10, 89), (13, 1), (64, 6), (100, 42), (102, 43), (105, 58), (109, 61), (43, 74),
                    (45, 74),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 85; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 150; lexer.advance(false); continue; }
                return result;
            }
            86 => {
                if eof { state = 88; lexer.advance(false); continue; }
                if lookahead == 10 { state = 89; lexer.advance(false); continue; }
                if lookahead == 13 { state = 1; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 86; lexer.advance(true); continue; }
                if lookahead != 0 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            87 => {
                if eof { state = 88; lexer.advance(false); continue; }
                if lookahead == 10 { state = 89; lexer.advance(false); continue; }
                if lookahead == 13 { state = 1; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 157; lexer.advance(false); continue; }
                if lookahead != 0 { state = 158; lexer.advance(false); continue; }
                return result;
            }
            88 => {
                result = true; lexer.set_result_symbol(ts_builtin_sym_end); lexer.mark_end();
                return result;
            }
            89 => {
                result = true; lexer.set_result_symbol(aux_sym_source_token1); lexer.mark_end();
                return result;
            }
            90 => {
                result = true; lexer.set_result_symbol(anon_sym_diff); lexer.mark_end();
                return result;
            }
            91 => {
                result = true; lexer.set_result_symbol(aux_sym_command_token1); lexer.mark_end();
                if lookahead == 45 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 91; lexer.advance(false); continue; }
                return result;
            }
            92 => {
                result = true; lexer.set_result_symbol(anon_sym_new); lexer.mark_end();
                return result;
            }
            93 => {
                result = true; lexer.set_result_symbol(anon_sym_deleted); lexer.mark_end();
                return result;
            }
            94 => {
                result = true; lexer.set_result_symbol(anon_sym_file); lexer.mark_end();
                return result;
            }
            95 => {
                result = true; lexer.set_result_symbol(anon_sym_file); lexer.mark_end();
                if lookahead == 115 { state = 102; lexer.advance(false); continue; }
                return result;
            }
            96 => {
                result = true; lexer.set_result_symbol(anon_sym_mode); lexer.mark_end();
                return result;
            }
            97 => {
                result = true; lexer.set_result_symbol(anon_sym_old); lexer.mark_end();
                return result;
            }
            98 => {
                result = true; lexer.set_result_symbol(anon_sym_rename); lexer.mark_end();
                return result;
            }
            99 => {
                result = true; lexer.set_result_symbol(anon_sym_from); lexer.mark_end();
                return result;
            }
            100 => {
                result = true; lexer.set_result_symbol(anon_sym_to); lexer.mark_end();
                return result;
            }
            101 => {
                result = true; lexer.set_result_symbol(anon_sym_Binary); lexer.mark_end();
                return result;
            }
            102 => {
                result = true; lexer.set_result_symbol(anon_sym_files); lexer.mark_end();
                return result;
            }
            103 => {
                result = true; lexer.set_result_symbol(anon_sym_and); lexer.mark_end();
                return result;
            }
            104 => {
                result = true; lexer.set_result_symbol(anon_sym_and); lexer.mark_end();
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 32 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            105 => {
                result = true; lexer.set_result_symbol(anon_sym_differ); lexer.mark_end();
                return result;
            }
            106 => {
                result = true; lexer.set_result_symbol(anon_sym_differ); lexer.mark_end();
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 32 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            107 => {
                result = true; lexer.set_result_symbol(anon_sym_index); lexer.mark_end();
                return result;
            }
            108 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT_DOT); lexer.mark_end();
                return result;
            }
            109 => {
                result = true; lexer.set_result_symbol(anon_sym_similarity); lexer.mark_end();
                return result;
            }
            110 => {
                result = true; lexer.set_result_symbol(anon_sym_index2); lexer.mark_end();
                return result;
            }
            111 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 248; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 149; lexer.advance(false); continue; }
                return result;
            }
            112 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 215; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 150; lexer.advance(false); continue; }
                return result;
            }
            113 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 76; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 111; lexer.advance(false); continue; }
                return result;
            }
            114 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 216; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 112; lexer.advance(false); continue; }
                return result;
            }
            115 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 77; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 113; lexer.advance(false); continue; }
                return result;
            }
            116 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 217; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 114; lexer.advance(false); continue; }
                return result;
            }
            117 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 78; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 115; lexer.advance(false); continue; }
                return result;
            }
            118 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 218; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 116; lexer.advance(false); continue; }
                return result;
            }
            119 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 79; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 117; lexer.advance(false); continue; }
                return result;
            }
            120 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 219; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 118; lexer.advance(false); continue; }
                return result;
            }
            121 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 80; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 119; lexer.advance(false); continue; }
                return result;
            }
            122 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 220; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 120; lexer.advance(false); continue; }
                return result;
            }
            123 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 221; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 122; lexer.advance(false); continue; }
                return result;
            }
            124 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 222; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 123; lexer.advance(false); continue; }
                return result;
            }
            125 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 223; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 124; lexer.advance(false); continue; }
                return result;
            }
            126 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 224; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 125; lexer.advance(false); continue; }
                return result;
            }
            127 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 225; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            128 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 226; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 127; lexer.advance(false); continue; }
                return result;
            }
            129 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 227; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 128; lexer.advance(false); continue; }
                return result;
            }
            130 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 228; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 129; lexer.advance(false); continue; }
                return result;
            }
            131 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 229; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 130; lexer.advance(false); continue; }
                return result;
            }
            132 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 230; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 131; lexer.advance(false); continue; }
                return result;
            }
            133 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 231; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 132; lexer.advance(false); continue; }
                return result;
            }
            134 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 232; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 133; lexer.advance(false); continue; }
                return result;
            }
            135 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 233; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 134; lexer.advance(false); continue; }
                return result;
            }
            136 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 234; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            137 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 235; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 136; lexer.advance(false); continue; }
                return result;
            }
            138 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 236; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 137; lexer.advance(false); continue; }
                return result;
            }
            139 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 237; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            140 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 238; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 139; lexer.advance(false); continue; }
                return result;
            }
            141 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 239; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 140; lexer.advance(false); continue; }
                return result;
            }
            142 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 240; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 141; lexer.advance(false); continue; }
                return result;
            }
            143 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 241; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 142; lexer.advance(false); continue; }
                return result;
            }
            144 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 242; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 143; lexer.advance(false); continue; }
                return result;
            }
            145 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 243; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 144; lexer.advance(false); continue; }
                return result;
            }
            146 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 244; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 145; lexer.advance(false); continue; }
                return result;
            }
            147 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 245; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 146; lexer.advance(false); continue; }
                return result;
            }
            148 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 246; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 147; lexer.advance(false); continue; }
                return result;
            }
            149 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 97 <= lookahead && lookahead <= 102 { state = 247; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 148; lexer.advance(false); continue; }
                return result;
            }
            150 => {
                result = true; lexer.set_result_symbol(aux_sym_similarity_token1); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 { state = 150; lexer.advance(false); continue; }
                return result;
            }
            151 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT); lexer.mark_end();
                return result;
            }
            152 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_DASH_DASH); lexer.mark_end();
                if lookahead == 45 { state = 164; lexer.advance(false); continue; }
                return result;
            }
            153 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS_PLUS_PLUS); lexer.mark_end();
                return result;
            }
            154 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS_PLUS_PLUS); lexer.mark_end();
                if lookahead == 43 { state = 161; lexer.advance(false); continue; }
                return result;
            }
            155 => {
                result = true; lexer.set_result_symbol(anon_sym_AT_AT); lexer.mark_end();
                return result;
            }
            156 => {
                result = true; lexer.set_result_symbol(anon_sym_AT_AT2); lexer.mark_end();
                return result;
            }
            157 => {
                result = true; lexer.set_result_symbol(aux_sym_location_token1); lexer.mark_end();
                if lookahead == 9 || lookahead == 11 || lookahead == 12 || lookahead == 32 { state = 157; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) { state = 158; lexer.advance(false); continue; }
                return result;
            }
            158 => {
                result = true; lexer.set_result_symbol(aux_sym_location_token1); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 158; lexer.advance(false); continue; }
                return result;
            }
            159 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                if lookahead == 43 { state = 160; lexer.advance(false); continue; }
                return result;
            }
            160 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS_PLUS); lexer.mark_end();
                if lookahead == 43 { state = 154; lexer.advance(false); continue; }
                return result;
            }
            161 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS_PLUS_PLUS_PLUS); lexer.mark_end();
                return result;
            }
            162 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 163; lexer.advance(false); continue; }
                return result;
            }
            163 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_DASH); lexer.mark_end();
                if lookahead == 45 { state = 152; lexer.advance(false); continue; }
                return result;
            }
            164 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_DASH_DASH_DASH); lexer.mark_end();
                return result;
            }
            165 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 89), (13, 1), (100, 178), (110, 174), (111, 187), (114, 179), (9, 165), (11, 165),
                    (12, 165), (32, 165),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead != 0 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            166 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 64 { state = 155; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            167 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 97 { state = 195; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            168 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 97 { state = 191; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            169 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 97 { state = 196; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            170 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 100 { state = 97; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            171 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 100 { state = 93; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            172 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 100 { state = 176; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            173 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 101 { state = 98; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            174 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 101 { state = 199; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            175 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 101 { state = 198; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            176 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 101 { state = 200; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            177 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 101 { state = 188; lexer.advance(false); continue; }
                if lookahead == 105 { state = 182; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            178 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 101 { state = 188; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            179 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 101 { state = 194; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            180 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 101 { state = 171; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            181 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 102 { state = 90; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            182 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 102 { state = 181; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            183 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 105 { state = 190; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            184 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 105 { state = 193; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            185 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 105 { state = 197; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            186 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 105 { state = 189; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            187 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 108 { state = 170; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            188 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 108 { state = 175; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            189 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 108 { state = 169; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            190 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 109 { state = 186; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            191 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 109 { state = 173; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            192 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 110 { state = 172; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            193 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 110 { state = 167; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            194 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 110 { state = 168; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            195 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 114 { state = 201; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            196 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 114 { state = 185; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            197 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 116 { state = 202; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            198 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 116 { state = 180; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            199 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 119 { state = 92; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            200 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 120 { state = 107; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            201 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 121 { state = 101; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            202 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead == 121 { state = 109; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            203 => {
                result = true; lexer.set_result_symbol(sym_context); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            204 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND); lexer.mark_end();
                return result;
            }
            205 => {
                result = true; lexer.set_result_symbol(sym_linerange); lexer.mark_end();
                if lookahead == 44 { state = 75; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 205; lexer.advance(false); continue; }
                return result;
            }
            206 => {
                result = true; lexer.set_result_symbol(sym_linerange); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 { state = 206; lexer.advance(false); continue; }
                return result;
            }
            207 => {
                result = true; lexer.set_result_symbol(aux_sym_filename_token1); lexer.mark_end();
                if lookahead == 100 { state = 104; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 32 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            208 => {
                result = true; lexer.set_result_symbol(aux_sym_filename_token1); lexer.mark_end();
                if lookahead == 101 { state = 213; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 32 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            209 => {
                result = true; lexer.set_result_symbol(aux_sym_filename_token1); lexer.mark_end();
                if lookahead == 102 { state = 208; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 32 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            210 => {
                result = true; lexer.set_result_symbol(aux_sym_filename_token1); lexer.mark_end();
                if lookahead == 102 { state = 209; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 32 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            211 => {
                result = true; lexer.set_result_symbol(aux_sym_filename_token1); lexer.mark_end();
                if lookahead == 105 { state = 210; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 32 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            212 => {
                result = true; lexer.set_result_symbol(aux_sym_filename_token1); lexer.mark_end();
                if lookahead == 110 { state = 207; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 32 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            213 => {
                result = true; lexer.set_result_symbol(aux_sym_filename_token1); lexer.mark_end();
                if lookahead == 114 { state = 106; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 32 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            214 => {
                result = true; lexer.set_result_symbol(aux_sym_filename_token1); lexer.mark_end();
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 32 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            215 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                return result;
            }
            216 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 215; lexer.advance(false); continue; }
                return result;
            }
            217 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 216; lexer.advance(false); continue; }
                return result;
            }
            218 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 217; lexer.advance(false); continue; }
                return result;
            }
            219 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 218; lexer.advance(false); continue; }
                return result;
            }
            220 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 219; lexer.advance(false); continue; }
                return result;
            }
            221 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 220; lexer.advance(false); continue; }
                return result;
            }
            222 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 221; lexer.advance(false); continue; }
                return result;
            }
            223 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 222; lexer.advance(false); continue; }
                return result;
            }
            224 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 223; lexer.advance(false); continue; }
                return result;
            }
            225 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 224; lexer.advance(false); continue; }
                return result;
            }
            226 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 225; lexer.advance(false); continue; }
                return result;
            }
            227 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 226; lexer.advance(false); continue; }
                return result;
            }
            228 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            229 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 228; lexer.advance(false); continue; }
                return result;
            }
            230 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 229; lexer.advance(false); continue; }
                return result;
            }
            231 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 230; lexer.advance(false); continue; }
                return result;
            }
            232 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 231; lexer.advance(false); continue; }
                return result;
            }
            233 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 232; lexer.advance(false); continue; }
                return result;
            }
            234 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 233; lexer.advance(false); continue; }
                return result;
            }
            235 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 234; lexer.advance(false); continue; }
                return result;
            }
            236 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 235; lexer.advance(false); continue; }
                return result;
            }
            237 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 236; lexer.advance(false); continue; }
                return result;
            }
            238 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 237; lexer.advance(false); continue; }
                return result;
            }
            239 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 238; lexer.advance(false); continue; }
                return result;
            }
            240 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 239; lexer.advance(false); continue; }
                return result;
            }
            241 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 240; lexer.advance(false); continue; }
                return result;
            }
            242 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 241; lexer.advance(false); continue; }
                return result;
            }
            243 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 242; lexer.advance(false); continue; }
                return result;
            }
            244 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 243; lexer.advance(false); continue; }
                return result;
            }
            245 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 244; lexer.advance(false); continue; }
                return result;
            }
            246 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 245; lexer.advance(false); continue; }
                return result;
            }
            247 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 246; lexer.advance(false); continue; }
                return result;
            }
            248 => {
                result = true; lexer.set_result_symbol(sym_commit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 102 { state = 247; lexer.advance(false); continue; }
                return result;
            }
            _ => return false,
        }
    }
}
