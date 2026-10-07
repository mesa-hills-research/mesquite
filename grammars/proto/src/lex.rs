//! The `proto` grammar's lexer: `ts_lex`, transliterated from
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

const anon_sym_COLON: Symbol = 56;
const anon_sym_COMMA: Symbol = 20;
const anon_sym_DASH: Symbol = 18;
const anon_sym_DOT: Symbol = 11;
const anon_sym_DQUOTE: Symbol = 66;
const anon_sym_EQ: Symbol = 3;
const anon_sym_GT: Symbol = 31;
const anon_sym_LBRACE: Symbol = 16;
const anon_sym_LBRACK: Symbol = 19;
const anon_sym_LPAREN: Symbol = 10;
const anon_sym_LT: Symbol = 30;
const anon_sym_PLUS: Symbol = 55;
const anon_sym_RBRACE: Symbol = 17;
const anon_sym_RBRACK: Symbol = 21;
const anon_sym_RPAREN: Symbol = 12;
const anon_sym_SEMI: Symbol = 1;
const anon_sym_SLASH: Symbol = 57;
const anon_sym_SQUOTE: Symbol = 68;
const anon_sym_bool: Symbol = 42;
const anon_sym_bytes: Symbol = 46;
const anon_sym_double: Symbol = 44;
const anon_sym_edition: Symbol = 2;
const anon_sym_enum: Symbol = 15;
const anon_sym_export: Symbol = 13;
const anon_sym_extend: Symbol = 23;
const anon_sym_extensions: Symbol = 48;
const anon_sym_fixed32: Symbol = 38;
const anon_sym_fixed64: Symbol = 39;
const anon_sym_float: Symbol = 45;
const anon_sym_group: Symbol = 27;
const anon_sym_import: Symbol = 5;
const anon_sym_int32: Symbol = 32;
const anon_sym_int64: Symbol = 33;
const anon_sym_local: Symbol = 14;
const anon_sym_map: Symbol = 29;
const anon_sym_max: Symbol = 50;
const anon_sym_message: Symbol = 22;
const anon_sym_oneof: Symbol = 28;
const anon_sym_option: Symbol = 8;
const anon_sym_optional: Symbol = 24;
const anon_sym_package: Symbol = 9;
const anon_sym_public: Symbol = 7;
const anon_sym_repeated: Symbol = 26;
const anon_sym_required: Symbol = 25;
const anon_sym_reserved: Symbol = 47;
const anon_sym_returns: Symbol = 54;
const anon_sym_rpc: Symbol = 52;
const anon_sym_service: Symbol = 51;
const anon_sym_sfixed32: Symbol = 40;
const anon_sym_sfixed64: Symbol = 41;
const anon_sym_sint32: Symbol = 36;
const anon_sym_sint64: Symbol = 37;
const anon_sym_stream: Symbol = 53;
const anon_sym_string: Symbol = 43;
const anon_sym_syntax: Symbol = 4;
const anon_sym_to: Symbol = 49;
const anon_sym_uint32: Symbol = 34;
const anon_sym_uint64: Symbol = 35;
const anon_sym_weak: Symbol = 6;
const aux_sym_string_token1: Symbol = 67;
const aux_sym_string_token2: Symbol = 69;
const sym_comment: Symbol = 71;
const sym_decimal_lit: Symbol = 62;
const sym_escape_sequence: Symbol = 70;
const sym_false: Symbol = 61;
const sym_float_lit: Symbol = 65;
const sym_hex_lit: Symbol = 64;
const sym_identifier: Symbol = 58;
const sym_octal_lit: Symbol = 63;
const sym_reserved_identifier: Symbol = 59;
const sym_true: Symbol = 60;
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
                if eof { state = 204; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (34, 444), (39, 451), (40, 217), (41, 220), (43, 292), (44, 231), (45, 229), (46, 219),
                    (47, 294), (48, 436), (58, 293), (59, 205), (60, 249), (61, 207), (62, 250), (91, 230),
                    (92, 35), (93, 232), (98, 134), (100, 129), (101, 61), (102, 36), (103, 158), (105, 112),
                    (108, 130), (109, 37), (110, 38), (111, 120), (112, 41), (114, 65), (115, 73), (116, 131),
                    (117, 102), (119, 75), (123, 227), (125, 228),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 202; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 434; lexer.advance(false); continue; }
                return result;
            }
            1 => {
                if let Some(next) = advance_map(&[
                    (34, 444), (39, 451), (40, 217), (41, 220), (44, 231), (46, 218), (47, 294), (59, 205),
                    (62, 250), (91, 230), (93, 232), (123, 227), (125, 228),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 1; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            2 => {
                if let Some(next) = advance_map(&[
                    (34, 444), (39, 451), (43, 292), (45, 229), (46, 189), (47, 8), (48, 436), (58, 293),
                    (60, 249), (91, 230), (93, 232), (102, 315), (105, 372), (110, 316), (116, 399), (123, 227),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 2; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 434; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            3 => {
                if lookahead == 34 { state = 444; lexer.advance(false); continue; }
                if lookahead == 47 { state = 446; lexer.advance(false); continue; }
                if lookahead == 92 { state = 35; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 449; lexer.advance(false); continue; }
                if lookahead != 0 { state = 450; lexer.advance(false); continue; }
                return result;
            }
            4 => {
                if lookahead == 34 { state = 200; lexer.advance(false); continue; }
                if lookahead == 39 { state = 201; lexer.advance(false); continue; }
                if lookahead == 47 { state = 8; lexer.advance(false); continue; }
                if lookahead == 48 { state = 438; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 4; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 435; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 429; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                if lookahead == 34 { state = 428; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 5; lexer.advance(false); continue; }
                return result;
            }
            6 => {
                if lookahead == 39 { state = 451; lexer.advance(false); continue; }
                if lookahead == 47 { state = 453; lexer.advance(false); continue; }
                if lookahead == 92 { state = 35; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 456; lexer.advance(false); continue; }
                if lookahead != 0 { state = 457; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if lookahead == 39 { state = 428; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 7; lexer.advance(false); continue; }
                return result;
            }
            8 => {
                if lookahead == 42 { state = 10; lexer.advance(false); continue; }
                if lookahead == 47 { state = 462; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if lookahead == 42 { state = 9; lexer.advance(false); continue; }
                if lookahead == 47 { state = 461; lexer.advance(false); continue; }
                if lookahead != 0 { state = 10; lexer.advance(false); continue; }
                return result;
            }
            10 => {
                if lookahead == 42 { state = 9; lexer.advance(false); continue; }
                if lookahead != 0 { state = 10; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if lookahead == 46 { state = 442; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 188; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 11; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if let Some(next) = advance_map(&[
                    (46, 218), (47, 8), (59, 205), (98, 382), (100, 377), (101, 371), (102, 350), (103, 402),
                    (105, 370), (108, 378), (109, 310), (111, 369), (114, 326), (115, 347), (117, 356), (125, 228),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 12; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if let Some(next) = advance_map(&[
                    (46, 218), (47, 8), (59, 205), (98, 382), (100, 377), (102, 350), (103, 402), (105, 370),
                    (111, 390), (114, 330), (115, 347), (117, 356), (125, 228),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 13; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if let Some(next) = advance_map(&[
                    (46, 218), (47, 8), (98, 382), (100, 377), (102, 350), (103, 402), (105, 370), (114, 338),
                    (115, 347), (117, 356),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 14; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if let Some(next) = advance_map(&[
                    (46, 218), (47, 8), (98, 382), (100, 377), (102, 350), (103, 402), (105, 370), (115, 347),
                    (117, 356),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 15; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if let Some(next) = advance_map(&[
                    (46, 218), (47, 8), (98, 382), (100, 377), (102, 350), (105, 370), (115, 347), (117, 356),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 16; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if lookahead == 46 { state = 218; lexer.advance(false); continue; }
                if lookahead == 47 { state = 8; lexer.advance(false); continue; }
                if lookahead == 115 { state = 416; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 17; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            18 => {
                if lookahead == 46 { state = 189; lexer.advance(false); continue; }
                if lookahead == 47 { state = 8; lexer.advance(false); continue; }
                if lookahead == 48 { state = 436; lexer.advance(false); continue; }
                if lookahead == 105 { state = 121; lexer.advance(false); continue; }
                if lookahead == 110 { state = 38; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 18; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 434; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if lookahead == 47 { state = 8; lexer.advance(false); continue; }
                if lookahead == 59 { state = 205; lexer.advance(false); continue; }
                if lookahead == 111 { state = 395; lexer.advance(false); continue; }
                if lookahead == 114 { state = 343; lexer.advance(false); continue; }
                if lookahead == 125 { state = 228; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 19; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            20 => {
                if lookahead == 50 { state = 251; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                if lookahead == 50 { state = 259; lexer.advance(false); continue; }
                return result;
            }
            22 => {
                if lookahead == 50 { state = 255; lexer.advance(false); continue; }
                return result;
            }
            23 => {
                if lookahead == 50 { state = 263; lexer.advance(false); continue; }
                return result;
            }
            24 => {
                if lookahead == 50 { state = 267; lexer.advance(false); continue; }
                return result;
            }
            25 => {
                if lookahead == 51 { state = 20; lexer.advance(false); continue; }
                if lookahead == 54 { state = 30; lexer.advance(false); continue; }
                return result;
            }
            26 => {
                if lookahead == 51 { state = 21; lexer.advance(false); continue; }
                if lookahead == 54 { state = 31; lexer.advance(false); continue; }
                return result;
            }
            27 => {
                if lookahead == 51 { state = 22; lexer.advance(false); continue; }
                if lookahead == 54 { state = 32; lexer.advance(false); continue; }
                return result;
            }
            28 => {
                if lookahead == 51 { state = 23; lexer.advance(false); continue; }
                if lookahead == 54 { state = 33; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                if lookahead == 51 { state = 24; lexer.advance(false); continue; }
                if lookahead == 54 { state = 34; lexer.advance(false); continue; }
                return result;
            }
            30 => {
                if lookahead == 52 { state = 253; lexer.advance(false); continue; }
                return result;
            }
            31 => {
                if lookahead == 52 { state = 261; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                if lookahead == 52 { state = 257; lexer.advance(false); continue; }
                return result;
            }
            33 => {
                if lookahead == 52 { state = 265; lexer.advance(false); continue; }
                return result;
            }
            34 => {
                if lookahead == 52 { state = 269; lexer.advance(false); continue; }
                return result;
            }
            35 => {
                if lookahead == 85 { state = 199; lexer.advance(false); continue; }
                if lookahead == 117 { state = 195; lexer.advance(false); continue; }
                if lookahead == 120 { state = 193; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 460; lexer.advance(false); continue; }
                if lookahead != 0 { state = 458; lexer.advance(false); continue; }
                return result;
            }
            36 => {
                if lookahead == 97 { state = 108; lexer.advance(false); continue; }
                if lookahead == 105 { state = 186; lexer.advance(false); continue; }
                if lookahead == 108 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                if lookahead == 97 { state = 143; lexer.advance(false); continue; }
                if lookahead == 101 { state = 162; lexer.advance(false); continue; }
                return result;
            }
            38 => {
                if lookahead == 97 { state = 115; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                if lookahead == 97 { state = 91; lexer.advance(false); continue; }
                return result;
            }
            40 => {
                if lookahead == 97 { state = 185; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                if lookahead == 97 { state = 51; lexer.advance(false); continue; }
                if lookahead == 117 { state = 49; lexer.advance(false); continue; }
                return result;
            }
            42 => {
                if lookahead == 97 { state = 114; lexer.advance(false); continue; }
                return result;
            }
            43 => {
                if lookahead == 97 { state = 103; lexer.advance(false); continue; }
                return result;
            }
            44 => {
                if lookahead == 97 { state = 184; lexer.advance(false); continue; }
                if lookahead == 101 { state = 162; lexer.advance(false); continue; }
                return result;
            }
            45 => {
                if lookahead == 97 { state = 165; lexer.advance(false); continue; }
                return result;
            }
            46 => {
                if lookahead == 97 { state = 106; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                if lookahead == 97 { state = 171; lexer.advance(false); continue; }
                return result;
            }
            48 => {
                if lookahead == 97 { state = 92; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                if lookahead == 98 { state = 109; lexer.advance(false); continue; }
                return result;
            }
            50 => {
                if lookahead == 98 { state = 110; lexer.advance(false); continue; }
                return result;
            }
            51 => {
                if lookahead == 99 { state = 104; lexer.advance(false); continue; }
                return result;
            }
            52 => {
                if lookahead == 99 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                if lookahead == 99 { state = 211; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                if lookahead == 99 { state = 46; lexer.advance(false); continue; }
                return result;
            }
            55 => {
                if lookahead == 99 { state = 72; lexer.advance(false); continue; }
                return result;
            }
            56 => {
                if lookahead == 100 { state = 235; lexer.advance(false); continue; }
                return result;
            }
            57 => {
                if lookahead == 100 { state = 235; lexer.advance(false); continue; }
                if lookahead == 115 { state = 98; lexer.advance(false); continue; }
                return result;
            }
            58 => {
                if lookahead == 100 { state = 241; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                if lookahead == 100 { state = 239; lexer.advance(false); continue; }
                return result;
            }
            60 => {
                if lookahead == 100 { state = 281; lexer.advance(false); continue; }
                return result;
            }
            61 => {
                if lookahead == 100 { state = 100; lexer.advance(false); continue; }
                if lookahead == 110 { state = 176; lexer.advance(false); continue; }
                if lookahead == 120 { state = 146; lexer.advance(false); continue; }
                return result;
            }
            62 => {
                if lookahead == 100 { state = 100; lexer.advance(false); continue; }
                if lookahead == 110 { state = 176; lexer.advance(false); continue; }
                if lookahead == 120 { state = 147; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                if lookahead == 100 { state = 28; lexer.advance(false); continue; }
                return result;
            }
            64 => {
                if lookahead == 100 { state = 29; lexer.advance(false); continue; }
                return result;
            }
            65 => {
                if lookahead == 101 { state = 148; lexer.advance(false); continue; }
                if lookahead == 112 { state = 52; lexer.advance(false); continue; }
                return result;
            }
            66 => {
                if lookahead == 101 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            67 => {
                if lookahead == 101 { state = 430; lexer.advance(false); continue; }
                return result;
            }
            68 => {
                if lookahead == 101 { state = 432; lexer.advance(false); continue; }
                return result;
            }
            69 => {
                if lookahead == 101 { state = 275; lexer.advance(false); continue; }
                return result;
            }
            70 => {
                if lookahead == 101 { state = 233; lexer.advance(false); continue; }
                return result;
            }
            71 => {
                if lookahead == 101 { state = 216; lexer.advance(false); continue; }
                return result;
            }
            72 => {
                if lookahead == 101 { state = 287; lexer.advance(false); continue; }
                return result;
            }
            73 => {
                if lookahead == 101 { state = 151; lexer.advance(false); continue; }
                if lookahead == 102 { state = 101; lexer.advance(false); continue; }
                if lookahead == 105 { state = 123; lexer.advance(false); continue; }
                if lookahead == 116 { state = 152; lexer.advance(false); continue; }
                if lookahead == 121 { state = 124; lexer.advance(false); continue; }
                return result;
            }
            74 => {
                if lookahead == 101 { state = 151; lexer.advance(false); continue; }
                if lookahead == 121 { state = 124; lexer.advance(false); continue; }
                return result;
            }
            75 => {
                if lookahead == 101 { state = 43; lexer.advance(false); continue; }
                return result;
            }
            76 => {
                if lookahead == 101 { state = 58; lexer.advance(false); continue; }
                return result;
            }
            77 => {
                if lookahead == 101 { state = 116; lexer.advance(false); continue; }
                return result;
            }
            78 => {
                if lookahead == 101 { state = 59; lexer.advance(false); continue; }
                return result;
            }
            79 => {
                if lookahead == 101 { state = 159; lexer.advance(false); continue; }
                return result;
            }
            80 => {
                if lookahead == 101 { state = 60; lexer.advance(false); continue; }
                return result;
            }
            81 => {
                if lookahead == 101 { state = 153; lexer.advance(false); continue; }
                return result;
            }
            82 => {
                if lookahead == 101 { state = 132; lexer.advance(false); continue; }
                return result;
            }
            83 => {
                if lookahead == 101 { state = 47; lexer.advance(false); continue; }
                return result;
            }
            84 => {
                if lookahead == 101 { state = 42; lexer.advance(false); continue; }
                if lookahead == 105 { state = 122; lexer.advance(false); continue; }
                return result;
            }
            85 => {
                if lookahead == 101 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            86 => {
                if lookahead == 101 { state = 64; lexer.advance(false); continue; }
                return result;
            }
            87 => {
                if lookahead == 102 { state = 441; lexer.advance(false); continue; }
                return result;
            }
            88 => {
                if lookahead == 102 { state = 441; lexer.advance(false); continue; }
                if lookahead == 116 { state = 25; lexer.advance(false); continue; }
                return result;
            }
            89 => {
                if lookahead == 102 { state = 245; lexer.advance(false); continue; }
                return result;
            }
            90 => {
                if lookahead == 103 { state = 273; lexer.advance(false); continue; }
                return result;
            }
            91 => {
                if lookahead == 103 { state = 70; lexer.advance(false); continue; }
                return result;
            }
            92 => {
                if lookahead == 103 { state = 71; lexer.advance(false); continue; }
                return result;
            }
            93 => {
                if lookahead == 105 { state = 53; lexer.advance(false); continue; }
                return result;
            }
            94 => {
                if lookahead == 105 { state = 55; lexer.advance(false); continue; }
                return result;
            }
            95 => {
                if lookahead == 105 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            96 => {
                if lookahead == 105 { state = 157; lexer.advance(false); continue; }
                return result;
            }
            97 => {
                if lookahead == 105 { state = 139; lexer.advance(false); continue; }
                return result;
            }
            98 => {
                if lookahead == 105 { state = 140; lexer.advance(false); continue; }
                return result;
            }
            99 => {
                if lookahead == 105 { state = 141; lexer.advance(false); continue; }
                return result;
            }
            100 => {
                if lookahead == 105 { state = 173; lexer.advance(false); continue; }
                return result;
            }
            101 => {
                if lookahead == 105 { state = 187; lexer.advance(false); continue; }
                return result;
            }
            102 => {
                if lookahead == 105 { state = 128; lexer.advance(false); continue; }
                return result;
            }
            103 => {
                if lookahead == 107 { state = 210; lexer.advance(false); continue; }
                return result;
            }
            104 => {
                if lookahead == 107 { state = 48; lexer.advance(false); continue; }
                return result;
            }
            105 => {
                if lookahead == 108 { state = 271; lexer.advance(false); continue; }
                return result;
            }
            106 => {
                if lookahead == 108 { state = 223; lexer.advance(false); continue; }
                return result;
            }
            107 => {
                if lookahead == 108 { state = 237; lexer.advance(false); continue; }
                return result;
            }
            108 => {
                if lookahead == 108 { state = 164; lexer.advance(false); continue; }
                return result;
            }
            109 => {
                if lookahead == 108 { state = 93; lexer.advance(false); continue; }
                return result;
            }
            110 => {
                if lookahead == 108 { state = 69; lexer.advance(false); continue; }
                return result;
            }
            111 => {
                if lookahead == 109 { state = 149; lexer.advance(false); continue; }
                return result;
            }
            112 => {
                if lookahead == 109 { state = 149; lexer.advance(false); continue; }
                if lookahead == 110 { state = 88; lexer.advance(false); continue; }
                return result;
            }
            113 => {
                if lookahead == 109 { state = 225; lexer.advance(false); continue; }
                return result;
            }
            114 => {
                if lookahead == 109 { state = 289; lexer.advance(false); continue; }
                return result;
            }
            115 => {
                if lookahead == 110 { state = 441; lexer.advance(false); continue; }
                return result;
            }
            116 => {
                if lookahead == 110 { state = 57; lexer.advance(false); continue; }
                return result;
            }
            117 => {
                if lookahead == 110 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            118 => {
                if lookahead == 110 { state = 206; lexer.advance(false); continue; }
                return result;
            }
            119 => {
                if lookahead == 110 { state = 212; lexer.advance(false); continue; }
                return result;
            }
            120 => {
                if lookahead == 110 { state = 82; lexer.advance(false); continue; }
                if lookahead == 112 { state = 169; lexer.advance(false); continue; }
                return result;
            }
            121 => {
                if lookahead == 110 { state = 87; lexer.advance(false); continue; }
                return result;
            }
            122 => {
                if lookahead == 110 { state = 90; lexer.advance(false); continue; }
                return result;
            }
            123 => {
                if lookahead == 110 { state = 172; lexer.advance(false); continue; }
                return result;
            }
            124 => {
                if lookahead == 110 { state = 170; lexer.advance(false); continue; }
                return result;
            }
            125 => {
                if lookahead == 110 { state = 160; lexer.advance(false); continue; }
                return result;
            }
            126 => {
                if lookahead == 110 { state = 56; lexer.advance(false); continue; }
                return result;
            }
            127 => {
                if lookahead == 110 { state = 161; lexer.advance(false); continue; }
                return result;
            }
            128 => {
                if lookahead == 110 { state = 174; lexer.advance(false); continue; }
                return result;
            }
            129 => {
                if lookahead == 111 { state = 181; lexer.advance(false); continue; }
                return result;
            }
            130 => {
                if lookahead == 111 { state = 54; lexer.advance(false); continue; }
                return result;
            }
            131 => {
                if lookahead == 111 { state = 285; lexer.advance(false); continue; }
                if lookahead == 114 { state = 180; lexer.advance(false); continue; }
                return result;
            }
            132 => {
                if lookahead == 111 { state = 89; lexer.advance(false); continue; }
                return result;
            }
            133 => {
                if lookahead == 111 { state = 105; lexer.advance(false); continue; }
                return result;
            }
            134 => {
                if lookahead == 111 { state = 133; lexer.advance(false); continue; }
                if lookahead == 121 { state = 168; lexer.advance(false); continue; }
                return result;
            }
            135 => {
                if lookahead == 111 { state = 45; lexer.advance(false); continue; }
                return result;
            }
            136 => {
                if lookahead == 111 { state = 154; lexer.advance(false); continue; }
                return result;
            }
            137 => {
                if lookahead == 111 { state = 177; lexer.advance(false); continue; }
                return result;
            }
            138 => {
                if lookahead == 111 { state = 117; lexer.advance(false); continue; }
                return result;
            }
            139 => {
                if lookahead == 111 { state = 118; lexer.advance(false); continue; }
                return result;
            }
            140 => {
                if lookahead == 111 { state = 127; lexer.advance(false); continue; }
                return result;
            }
            141 => {
                if lookahead == 111 { state = 119; lexer.advance(false); continue; }
                return result;
            }
            142 => {
                if lookahead == 111 { state = 156; lexer.advance(false); continue; }
                return result;
            }
            143 => {
                if lookahead == 112 { state = 247; lexer.advance(false); continue; }
                if lookahead == 120 { state = 286; lexer.advance(false); continue; }
                return result;
            }
            144 => {
                if lookahead == 112 { state = 243; lexer.advance(false); continue; }
                return result;
            }
            145 => {
                if lookahead == 112 { state = 52; lexer.advance(false); continue; }
                return result;
            }
            146 => {
                if lookahead == 112 { state = 136; lexer.advance(false); continue; }
                if lookahead == 116 { state = 77; lexer.advance(false); continue; }
                return result;
            }
            147 => {
                if lookahead == 112 { state = 136; lexer.advance(false); continue; }
                if lookahead == 116 { state = 85; lexer.advance(false); continue; }
                return result;
            }
            148 => {
                if lookahead == 112 { state = 83; lexer.advance(false); continue; }
                if lookahead == 113 { state = 179; lexer.advance(false); continue; }
                if lookahead == 115 { state = 81; lexer.advance(false); continue; }
                if lookahead == 116 { state = 178; lexer.advance(false); continue; }
                return result;
            }
            149 => {
                if lookahead == 112 { state = 142; lexer.advance(false); continue; }
                return result;
            }
            150 => {
                if lookahead == 112 { state = 175; lexer.advance(false); continue; }
                return result;
            }
            151 => {
                if lookahead == 114 { state = 182; lexer.advance(false); continue; }
                return result;
            }
            152 => {
                if lookahead == 114 { state = 84; lexer.advance(false); continue; }
                return result;
            }
            153 => {
                if lookahead == 114 { state = 183; lexer.advance(false); continue; }
                return result;
            }
            154 => {
                if lookahead == 114 { state = 166; lexer.advance(false); continue; }
                return result;
            }
            155 => {
                if lookahead == 114 { state = 125; lexer.advance(false); continue; }
                return result;
            }
            156 => {
                if lookahead == 114 { state = 167; lexer.advance(false); continue; }
                return result;
            }
            157 => {
                if lookahead == 114 { state = 78; lexer.advance(false); continue; }
                return result;
            }
            158 => {
                if lookahead == 114 { state = 137; lexer.advance(false); continue; }
                return result;
            }
            159 => {
                if lookahead == 115 { state = 279; lexer.advance(false); continue; }
                return result;
            }
            160 => {
                if lookahead == 115 { state = 291; lexer.advance(false); continue; }
                return result;
            }
            161 => {
                if lookahead == 115 { state = 283; lexer.advance(false); continue; }
                return result;
            }
            162 => {
                if lookahead == 115 { state = 163; lexer.advance(false); continue; }
                return result;
            }
            163 => {
                if lookahead == 115 { state = 39; lexer.advance(false); continue; }
                return result;
            }
            164 => {
                if lookahead == 115 { state = 68; lexer.advance(false); continue; }
                return result;
            }
            165 => {
                if lookahead == 116 { state = 277; lexer.advance(false); continue; }
                return result;
            }
            166 => {
                if lookahead == 116 { state = 221; lexer.advance(false); continue; }
                return result;
            }
            167 => {
                if lookahead == 116 { state = 209; lexer.advance(false); continue; }
                return result;
            }
            168 => {
                if lookahead == 116 { state = 79; lexer.advance(false); continue; }
                return result;
            }
            169 => {
                if lookahead == 116 { state = 95; lexer.advance(false); continue; }
                return result;
            }
            170 => {
                if lookahead == 116 { state = 40; lexer.advance(false); continue; }
                return result;
            }
            171 => {
                if lookahead == 116 { state = 76; lexer.advance(false); continue; }
                return result;
            }
            172 => {
                if lookahead == 116 { state = 26; lexer.advance(false); continue; }
                return result;
            }
            173 => {
                if lookahead == 116 { state = 97; lexer.advance(false); continue; }
                return result;
            }
            174 => {
                if lookahead == 116 { state = 27; lexer.advance(false); continue; }
                return result;
            }
            175 => {
                if lookahead == 116 { state = 99; lexer.advance(false); continue; }
                return result;
            }
            176 => {
                if lookahead == 117 { state = 113; lexer.advance(false); continue; }
                return result;
            }
            177 => {
                if lookahead == 117 { state = 144; lexer.advance(false); continue; }
                return result;
            }
            178 => {
                if lookahead == 117 { state = 155; lexer.advance(false); continue; }
                return result;
            }
            179 => {
                if lookahead == 117 { state = 96; lexer.advance(false); continue; }
                return result;
            }
            180 => {
                if lookahead == 117 { state = 67; lexer.advance(false); continue; }
                return result;
            }
            181 => {
                if lookahead == 117 { state = 50; lexer.advance(false); continue; }
                return result;
            }
            182 => {
                if lookahead == 118 { state = 94; lexer.advance(false); continue; }
                return result;
            }
            183 => {
                if lookahead == 118 { state = 80; lexer.advance(false); continue; }
                return result;
            }
            184 => {
                if lookahead == 120 { state = 286; lexer.advance(false); continue; }
                return result;
            }
            185 => {
                if lookahead == 120 { state = 208; lexer.advance(false); continue; }
                return result;
            }
            186 => {
                if lookahead == 120 { state = 66; lexer.advance(false); continue; }
                return result;
            }
            187 => {
                if lookahead == 120 { state = 86; lexer.advance(false); continue; }
                return result;
            }
            188 => {
                if lookahead == 43 || lookahead == 45 { state = 190; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 443; lexer.advance(false); continue; }
                return result;
            }
            189 => {
                if 48 <= lookahead && lookahead <= 57 { state = 442; lexer.advance(false); continue; }
                return result;
            }
            190 => {
                if 48 <= lookahead && lookahead <= 57 { state = 443; lexer.advance(false); continue; }
                return result;
            }
            191 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 458; lexer.advance(false); continue; }
                return result;
            }
            192 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 440; lexer.advance(false); continue; }
                return result;
            }
            193 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 191; lexer.advance(false); continue; }
                return result;
            }
            194 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 193; lexer.advance(false); continue; }
                return result;
            }
            195 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 194; lexer.advance(false); continue; }
                return result;
            }
            196 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 195; lexer.advance(false); continue; }
                return result;
            }
            197 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 196; lexer.advance(false); continue; }
                return result;
            }
            198 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 197; lexer.advance(false); continue; }
                return result;
            }
            199 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 198; lexer.advance(false); continue; }
                return result;
            }
            200 => {
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 5; lexer.advance(false); continue; }
                return result;
            }
            201 => {
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 7; lexer.advance(false); continue; }
                return result;
            }
            202 => {
                if eof { state = 204; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (34, 444), (39, 451), (40, 217), (41, 220), (43, 292), (44, 231), (45, 229), (46, 219),
                    (47, 294), (48, 436), (58, 293), (59, 205), (60, 249), (61, 207), (62, 250), (91, 230),
                    (93, 232), (98, 134), (100, 129), (101, 61), (102, 36), (103, 158), (105, 112), (108, 130),
                    (109, 37), (110, 38), (111, 120), (112, 41), (114, 65), (115, 73), (116, 131), (117, 102),
                    (119, 75), (123, 227), (125, 228),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 202; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 434; lexer.advance(false); continue; }
                return result;
            }
            203 => {
                if eof { state = 204; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (34, 444), (39, 451), (45, 229), (46, 218), (47, 8), (48, 438), (59, 205), (61, 207),
                    (101, 62), (105, 111), (108, 130), (109, 44), (111, 150), (112, 41), (114, 145), (115, 74),
                    (119, 75), (125, 228),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 203; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 435; lexer.advance(false); continue; }
                return result;
            }
            204 => {
                result = true; lexer.set_result_symbol(ts_builtin_sym_end); lexer.mark_end();
                return result;
            }
            205 => {
                result = true; lexer.set_result_symbol(anon_sym_SEMI); lexer.mark_end();
                return result;
            }
            206 => {
                result = true; lexer.set_result_symbol(anon_sym_edition); lexer.mark_end();
                return result;
            }
            207 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                return result;
            }
            208 => {
                result = true; lexer.set_result_symbol(anon_sym_syntax); lexer.mark_end();
                return result;
            }
            209 => {
                result = true; lexer.set_result_symbol(anon_sym_import); lexer.mark_end();
                return result;
            }
            210 => {
                result = true; lexer.set_result_symbol(anon_sym_weak); lexer.mark_end();
                return result;
            }
            211 => {
                result = true; lexer.set_result_symbol(anon_sym_public); lexer.mark_end();
                return result;
            }
            212 => {
                result = true; lexer.set_result_symbol(anon_sym_option); lexer.mark_end();
                return result;
            }
            213 => {
                result = true; lexer.set_result_symbol(anon_sym_option); lexer.mark_end();
                if lookahead == 97 { state = 360; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            214 => {
                result = true; lexer.set_result_symbol(anon_sym_option); lexer.mark_end();
                if lookahead == 97 { state = 107; lexer.advance(false); continue; }
                return result;
            }
            215 => {
                result = true; lexer.set_result_symbol(anon_sym_option); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            216 => {
                result = true; lexer.set_result_symbol(anon_sym_package); lexer.mark_end();
                return result;
            }
            217 => {
                result = true; lexer.set_result_symbol(anon_sym_LPAREN); lexer.mark_end();
                return result;
            }
            218 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                return result;
            }
            219 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 { state = 442; lexer.advance(false); continue; }
                return result;
            }
            220 => {
                result = true; lexer.set_result_symbol(anon_sym_RPAREN); lexer.mark_end();
                return result;
            }
            221 => {
                result = true; lexer.set_result_symbol(anon_sym_export); lexer.mark_end();
                return result;
            }
            222 => {
                result = true; lexer.set_result_symbol(anon_sym_export); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            223 => {
                result = true; lexer.set_result_symbol(anon_sym_local); lexer.mark_end();
                return result;
            }
            224 => {
                result = true; lexer.set_result_symbol(anon_sym_local); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            225 => {
                result = true; lexer.set_result_symbol(anon_sym_enum); lexer.mark_end();
                return result;
            }
            226 => {
                result = true; lexer.set_result_symbol(anon_sym_enum); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            227 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACE); lexer.mark_end();
                return result;
            }
            228 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACE); lexer.mark_end();
                return result;
            }
            229 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                return result;
            }
            230 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK); lexer.mark_end();
                return result;
            }
            231 => {
                result = true; lexer.set_result_symbol(anon_sym_COMMA); lexer.mark_end();
                return result;
            }
            232 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACK); lexer.mark_end();
                return result;
            }
            233 => {
                result = true; lexer.set_result_symbol(anon_sym_message); lexer.mark_end();
                return result;
            }
            234 => {
                result = true; lexer.set_result_symbol(anon_sym_message); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            235 => {
                result = true; lexer.set_result_symbol(anon_sym_extend); lexer.mark_end();
                return result;
            }
            236 => {
                result = true; lexer.set_result_symbol(anon_sym_extend); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            237 => {
                result = true; lexer.set_result_symbol(anon_sym_optional); lexer.mark_end();
                return result;
            }
            238 => {
                result = true; lexer.set_result_symbol(anon_sym_optional); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            239 => {
                result = true; lexer.set_result_symbol(anon_sym_required); lexer.mark_end();
                return result;
            }
            240 => {
                result = true; lexer.set_result_symbol(anon_sym_required); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            241 => {
                result = true; lexer.set_result_symbol(anon_sym_repeated); lexer.mark_end();
                return result;
            }
            242 => {
                result = true; lexer.set_result_symbol(anon_sym_repeated); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            243 => {
                result = true; lexer.set_result_symbol(anon_sym_group); lexer.mark_end();
                return result;
            }
            244 => {
                result = true; lexer.set_result_symbol(anon_sym_group); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            245 => {
                result = true; lexer.set_result_symbol(anon_sym_oneof); lexer.mark_end();
                return result;
            }
            246 => {
                result = true; lexer.set_result_symbol(anon_sym_oneof); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            247 => {
                result = true; lexer.set_result_symbol(anon_sym_map); lexer.mark_end();
                return result;
            }
            248 => {
                result = true; lexer.set_result_symbol(anon_sym_map); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            249 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                return result;
            }
            250 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                return result;
            }
            251 => {
                result = true; lexer.set_result_symbol(anon_sym_int32); lexer.mark_end();
                return result;
            }
            252 => {
                result = true; lexer.set_result_symbol(anon_sym_int32); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            253 => {
                result = true; lexer.set_result_symbol(anon_sym_int64); lexer.mark_end();
                return result;
            }
            254 => {
                result = true; lexer.set_result_symbol(anon_sym_int64); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            255 => {
                result = true; lexer.set_result_symbol(anon_sym_uint32); lexer.mark_end();
                return result;
            }
            256 => {
                result = true; lexer.set_result_symbol(anon_sym_uint32); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            257 => {
                result = true; lexer.set_result_symbol(anon_sym_uint64); lexer.mark_end();
                return result;
            }
            258 => {
                result = true; lexer.set_result_symbol(anon_sym_uint64); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            259 => {
                result = true; lexer.set_result_symbol(anon_sym_sint32); lexer.mark_end();
                return result;
            }
            260 => {
                result = true; lexer.set_result_symbol(anon_sym_sint32); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            261 => {
                result = true; lexer.set_result_symbol(anon_sym_sint64); lexer.mark_end();
                return result;
            }
            262 => {
                result = true; lexer.set_result_symbol(anon_sym_sint64); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            263 => {
                result = true; lexer.set_result_symbol(anon_sym_fixed32); lexer.mark_end();
                return result;
            }
            264 => {
                result = true; lexer.set_result_symbol(anon_sym_fixed32); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            265 => {
                result = true; lexer.set_result_symbol(anon_sym_fixed64); lexer.mark_end();
                return result;
            }
            266 => {
                result = true; lexer.set_result_symbol(anon_sym_fixed64); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            267 => {
                result = true; lexer.set_result_symbol(anon_sym_sfixed32); lexer.mark_end();
                return result;
            }
            268 => {
                result = true; lexer.set_result_symbol(anon_sym_sfixed32); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            269 => {
                result = true; lexer.set_result_symbol(anon_sym_sfixed64); lexer.mark_end();
                return result;
            }
            270 => {
                result = true; lexer.set_result_symbol(anon_sym_sfixed64); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            271 => {
                result = true; lexer.set_result_symbol(anon_sym_bool); lexer.mark_end();
                return result;
            }
            272 => {
                result = true; lexer.set_result_symbol(anon_sym_bool); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            273 => {
                result = true; lexer.set_result_symbol(anon_sym_string); lexer.mark_end();
                return result;
            }
            274 => {
                result = true; lexer.set_result_symbol(anon_sym_string); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            275 => {
                result = true; lexer.set_result_symbol(anon_sym_double); lexer.mark_end();
                return result;
            }
            276 => {
                result = true; lexer.set_result_symbol(anon_sym_double); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            277 => {
                result = true; lexer.set_result_symbol(anon_sym_float); lexer.mark_end();
                return result;
            }
            278 => {
                result = true; lexer.set_result_symbol(anon_sym_float); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            279 => {
                result = true; lexer.set_result_symbol(anon_sym_bytes); lexer.mark_end();
                return result;
            }
            280 => {
                result = true; lexer.set_result_symbol(anon_sym_bytes); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            281 => {
                result = true; lexer.set_result_symbol(anon_sym_reserved); lexer.mark_end();
                return result;
            }
            282 => {
                result = true; lexer.set_result_symbol(anon_sym_reserved); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            283 => {
                result = true; lexer.set_result_symbol(anon_sym_extensions); lexer.mark_end();
                return result;
            }
            284 => {
                result = true; lexer.set_result_symbol(anon_sym_extensions); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            285 => {
                result = true; lexer.set_result_symbol(anon_sym_to); lexer.mark_end();
                return result;
            }
            286 => {
                result = true; lexer.set_result_symbol(anon_sym_max); lexer.mark_end();
                return result;
            }
            287 => {
                result = true; lexer.set_result_symbol(anon_sym_service); lexer.mark_end();
                return result;
            }
            288 => {
                result = true; lexer.set_result_symbol(anon_sym_rpc); lexer.mark_end();
                return result;
            }
            289 => {
                result = true; lexer.set_result_symbol(anon_sym_stream); lexer.mark_end();
                return result;
            }
            290 => {
                result = true; lexer.set_result_symbol(anon_sym_stream); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            291 => {
                result = true; lexer.set_result_symbol(anon_sym_returns); lexer.mark_end();
                return result;
            }
            292 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                return result;
            }
            293 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON); lexer.mark_end();
                return result;
            }
            294 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH); lexer.mark_end();
                if lookahead == 42 { state = 10; lexer.advance(false); continue; }
                if lookahead == 47 { state = 462; lexer.advance(false); continue; }
                return result;
            }
            295 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 50 { state = 252; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            296 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 50 { state = 260; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            297 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 50 { state = 256; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            298 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 50 { state = 264; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            299 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 50 { state = 268; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            300 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 51 { state = 295; lexer.advance(false); continue; }
                if lookahead == 54 { state = 305; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            301 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 51 { state = 296; lexer.advance(false); continue; }
                if lookahead == 54 { state = 306; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            302 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 51 { state = 297; lexer.advance(false); continue; }
                if lookahead == 54 { state = 307; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            303 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 51 { state = 298; lexer.advance(false); continue; }
                if lookahead == 54 { state = 308; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            304 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 51 { state = 299; lexer.advance(false); continue; }
                if lookahead == 54 { state = 309; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            305 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 52 { state = 254; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            306 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 52 { state = 262; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            307 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 52 { state = 258; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            308 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 52 { state = 266; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            309 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 52 { state = 270; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            310 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 97 { state = 388; lexer.advance(false); continue; }
                if lookahead == 101 { state = 405; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            311 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 97 { state = 349; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            312 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 97 { state = 364; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            313 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 97 { state = 359; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            314 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 97 { state = 410; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            315 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 97 { state = 361; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            316 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 97 { state = 365; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            317 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 97 { state = 414; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            318 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 98 { state = 362; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            319 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 99 { state = 313; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            320 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 100 { state = 236; lexer.advance(false); continue; }
                if lookahead == 115 { state = 354; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            321 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 100 { state = 242; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            322 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 100 { state = 240; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            323 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 100 { state = 282; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            324 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 100 { state = 303; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            325 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 100 { state = 304; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            326 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 101 { state = 392; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            327 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 101 { state = 324; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            328 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 101 { state = 276; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            329 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 101 { state = 234; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            330 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 101 { state = 393; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            331 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 101 { state = 431; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            332 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 101 { state = 433; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            333 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 101 { state = 366; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            334 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 101 { state = 321; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            335 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 101 { state = 403; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            336 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 101 { state = 322; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            337 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 101 { state = 396; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            338 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 101 { state = 394; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            339 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 101 { state = 317; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            340 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 101 { state = 323; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            341 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 101 { state = 381; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            342 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 101 { state = 312; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            343 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 101 { state = 407; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            344 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 101 { state = 325; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            345 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 102 { state = 427; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            346 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 102 { state = 246; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            347 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 102 { state = 357; lexer.advance(false); continue; }
                if lookahead == 105 { state = 375; lexer.advance(false); continue; }
                if lookahead == 116 { state = 397; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            348 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 103 { state = 274; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            349 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 103 { state = 329; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            350 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 105 { state = 425; lexer.advance(false); continue; }
                if lookahead == 108 { state = 380; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            351 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 105 { state = 373; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            352 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 105 { state = 400; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            353 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 105 { state = 385; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            354 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 105 { state = 386; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            355 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 105 { state = 387; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            356 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 105 { state = 376; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            357 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 105 { state = 426; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            358 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 108 { state = 272; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            359 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 108 { state = 224; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            360 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 108 { state = 238; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            361 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 108 { state = 408; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            362 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 108 { state = 328; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            363 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 109 { state = 226; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            364 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 109 { state = 290; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            365 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 110 { state = 427; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            366 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 110 { state = 320; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            367 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 110 { state = 213; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            368 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 110 { state = 215; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            369 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 110 { state = 341; lexer.advance(false); continue; }
                if lookahead == 112 { state = 413; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            370 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 110 { state = 409; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            371 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 110 { state = 420; lexer.advance(false); continue; }
                if lookahead == 120 { state = 391; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            372 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 110 { state = 345; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            373 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 110 { state = 348; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            374 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 110 { state = 404; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            375 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 110 { state = 415; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            376 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 110 { state = 417; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            377 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 111 { state = 419; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            378 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 111 { state = 319; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            379 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 111 { state = 358; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            380 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 111 { state = 314; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            381 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 111 { state = 346; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            382 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 111 { state = 379; lexer.advance(false); continue; }
                if lookahead == 121 { state = 412; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            383 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 111 { state = 398; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            384 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 111 { state = 421; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            385 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 111 { state = 367; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            386 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 111 { state = 374; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            387 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 111 { state = 368; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            388 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 112 { state = 248; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            389 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 112 { state = 244; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            390 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 112 { state = 413; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            391 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 112 { state = 383; lexer.advance(false); continue; }
                if lookahead == 116 { state = 333; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            392 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 112 { state = 339; lexer.advance(false); continue; }
                if lookahead == 113 { state = 422; lexer.advance(false); continue; }
                if lookahead == 115 { state = 337; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            393 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 112 { state = 339; lexer.advance(false); continue; }
                if lookahead == 113 { state = 422; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            394 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 112 { state = 339; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            395 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 112 { state = 418; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            396 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 114 { state = 424; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            397 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 114 { state = 351; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            398 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 114 { state = 411; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            399 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 114 { state = 423; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            400 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 114 { state = 336; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            401 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 114 { state = 342; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            402 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 114 { state = 384; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            403 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 115 { state = 280; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            404 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 115 { state = 284; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            405 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 115 { state = 406; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            406 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 115 { state = 311; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            407 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 115 { state = 337; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            408 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 115 { state = 332; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            409 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 116 { state = 300; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            410 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 116 { state = 278; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            411 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 116 { state = 222; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            412 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 116 { state = 335; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            413 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 116 { state = 353; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            414 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 116 { state = 334; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            415 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 116 { state = 301; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            416 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 116 { state = 401; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            417 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 116 { state = 302; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            418 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 116 { state = 355; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            419 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 117 { state = 318; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            420 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 117 { state = 363; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            421 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 117 { state = 389; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            422 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 117 { state = 352; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            423 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 117 { state = 331; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            424 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 118 { state = 340; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            425 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 120 { state = 327; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            426 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 120 { state = 344; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            427 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            428 => {
                result = true; lexer.set_result_symbol(sym_reserved_identifier); lexer.mark_end();
                return result;
            }
            429 => {
                result = true; lexer.set_result_symbol(sym_reserved_identifier); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 429; lexer.advance(false); continue; }
                return result;
            }
            430 => {
                result = true; lexer.set_result_symbol(sym_true); lexer.mark_end();
                return result;
            }
            431 => {
                result = true; lexer.set_result_symbol(sym_true); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            432 => {
                result = true; lexer.set_result_symbol(sym_false); lexer.mark_end();
                return result;
            }
            433 => {
                result = true; lexer.set_result_symbol(sym_false); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            434 => {
                result = true; lexer.set_result_symbol(sym_decimal_lit); lexer.mark_end();
                if lookahead == 46 { state = 442; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 188; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 434; lexer.advance(false); continue; }
                return result;
            }
            435 => {
                result = true; lexer.set_result_symbol(sym_decimal_lit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 { state = 435; lexer.advance(false); continue; }
                return result;
            }
            436 => {
                result = true; lexer.set_result_symbol(sym_octal_lit); lexer.mark_end();
                if lookahead == 46 { state = 442; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 188; lexer.advance(false); continue; }
                if lookahead == 88 || lookahead == 120 { state = 192; lexer.advance(false); continue; }
                if lookahead == 56 || lookahead == 57 { state = 11; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 55 { state = 437; lexer.advance(false); continue; }
                return result;
            }
            437 => {
                result = true; lexer.set_result_symbol(sym_octal_lit); lexer.mark_end();
                if lookahead == 46 { state = 442; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 188; lexer.advance(false); continue; }
                if lookahead == 56 || lookahead == 57 { state = 11; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 55 { state = 437; lexer.advance(false); continue; }
                return result;
            }
            438 => {
                result = true; lexer.set_result_symbol(sym_octal_lit); lexer.mark_end();
                if lookahead == 88 || lookahead == 120 { state = 192; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 55 { state = 439; lexer.advance(false); continue; }
                return result;
            }
            439 => {
                result = true; lexer.set_result_symbol(sym_octal_lit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 55 { state = 439; lexer.advance(false); continue; }
                return result;
            }
            440 => {
                result = true; lexer.set_result_symbol(sym_hex_lit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 440; lexer.advance(false); continue; }
                return result;
            }
            441 => {
                result = true; lexer.set_result_symbol(sym_float_lit); lexer.mark_end();
                return result;
            }
            442 => {
                result = true; lexer.set_result_symbol(sym_float_lit); lexer.mark_end();
                if lookahead == 69 || lookahead == 101 { state = 188; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 442; lexer.advance(false); continue; }
                return result;
            }
            443 => {
                result = true; lexer.set_result_symbol(sym_float_lit); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 { state = 443; lexer.advance(false); continue; }
                return result;
            }
            444 => {
                result = true; lexer.set_result_symbol(anon_sym_DQUOTE); lexer.mark_end();
                return result;
            }
            445 => {
                result = true; lexer.set_result_symbol(aux_sym_string_token1); lexer.mark_end();
                if lookahead == 10 { state = 450; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 34 && lookahead != 92 { state = 445; lexer.advance(false); continue; }
                return result;
            }
            446 => {
                result = true; lexer.set_result_symbol(aux_sym_string_token1); lexer.mark_end();
                if lookahead == 42 { state = 448; lexer.advance(false); continue; }
                if lookahead == 47 { state = 445; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 34 && lookahead != 92 { state = 450; lexer.advance(false); continue; }
                return result;
            }
            447 => {
                result = true; lexer.set_result_symbol(aux_sym_string_token1); lexer.mark_end();
                if lookahead == 42 { state = 447; lexer.advance(false); continue; }
                if lookahead == 47 { state = 450; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 34 && lookahead != 92 { state = 448; lexer.advance(false); continue; }
                return result;
            }
            448 => {
                result = true; lexer.set_result_symbol(aux_sym_string_token1); lexer.mark_end();
                if lookahead == 42 { state = 447; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 34 && lookahead != 92 { state = 448; lexer.advance(false); continue; }
                return result;
            }
            449 => {
                result = true; lexer.set_result_symbol(aux_sym_string_token1); lexer.mark_end();
                if lookahead == 47 { state = 446; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 449; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 34 && lookahead != 92 { state = 450; lexer.advance(false); continue; }
                return result;
            }
            450 => {
                result = true; lexer.set_result_symbol(aux_sym_string_token1); lexer.mark_end();
                if lookahead != 0 && lookahead != 34 && lookahead != 92 { state = 450; lexer.advance(false); continue; }
                return result;
            }
            451 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTE); lexer.mark_end();
                return result;
            }
            452 => {
                result = true; lexer.set_result_symbol(aux_sym_string_token2); lexer.mark_end();
                if lookahead == 10 { state = 457; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 39 && lookahead != 92 { state = 452; lexer.advance(false); continue; }
                return result;
            }
            453 => {
                result = true; lexer.set_result_symbol(aux_sym_string_token2); lexer.mark_end();
                if lookahead == 42 { state = 455; lexer.advance(false); continue; }
                if lookahead == 47 { state = 452; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 39 && lookahead != 92 { state = 457; lexer.advance(false); continue; }
                return result;
            }
            454 => {
                result = true; lexer.set_result_symbol(aux_sym_string_token2); lexer.mark_end();
                if lookahead == 42 { state = 454; lexer.advance(false); continue; }
                if lookahead == 47 { state = 457; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 39 && lookahead != 92 { state = 455; lexer.advance(false); continue; }
                return result;
            }
            455 => {
                result = true; lexer.set_result_symbol(aux_sym_string_token2); lexer.mark_end();
                if lookahead == 42 { state = 454; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 39 && lookahead != 92 { state = 455; lexer.advance(false); continue; }
                return result;
            }
            456 => {
                result = true; lexer.set_result_symbol(aux_sym_string_token2); lexer.mark_end();
                if lookahead == 47 { state = 453; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 456; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 39 && lookahead != 92 { state = 457; lexer.advance(false); continue; }
                return result;
            }
            457 => {
                result = true; lexer.set_result_symbol(aux_sym_string_token2); lexer.mark_end();
                if lookahead != 0 && lookahead != 39 && lookahead != 92 { state = 457; lexer.advance(false); continue; }
                return result;
            }
            458 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                return result;
            }
            459 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 { state = 458; lexer.advance(false); continue; }
                return result;
            }
            460 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 { state = 459; lexer.advance(false); continue; }
                return result;
            }
            461 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                return result;
            }
            462 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 { state = 462; lexer.advance(false); continue; }
                return result;
            }
            _ => return false,
        }
    }
}
