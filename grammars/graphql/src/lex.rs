//! The `graphql` grammar's lexer: `ts_lex`, transliterated from
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

const anon_sym_AMP: Symbol = 11;
const anon_sym_ARGUMENT_DEFINITION: Symbol = 51;
const anon_sym_AT: Symbol = 36;
const anon_sym_BANG: Symbol = 58;
const anon_sym_COLON: Symbol = 13;
const anon_sym_DOLLAR: Symbol = 21;
const anon_sym_DOT_DOT_DOT: Symbol = 33;
const anon_sym_DQUOTE: Symbol = 24;
const anon_sym_DQUOTE_DQUOTE_DQUOTE: Symbol = 22;
const anon_sym_ENUM: Symbol = 54;
const anon_sym_ENUM_VALUE: Symbol = 55;
const anon_sym_EQ: Symbol = 16;
const anon_sym_FIELD: Symbol = 42;
const anon_sym_FIELD_DEFINITION: Symbol = 50;
const anon_sym_FRAGMENT_DEFINITION: Symbol = 43;
const anon_sym_FRAGMENT_SPREAD: Symbol = 44;
const anon_sym_INLINE_FRAGMENT: Symbol = 45;
const anon_sym_INPUT_FIELD_DEFINITION: Symbol = 57;
const anon_sym_INPUT_OBJECT: Symbol = 56;
const anon_sym_INTERFACE: Symbol = 52;
const anon_sym_LBRACE: Symbol = 2;
const anon_sym_LBRACK: Symbol = 31;
const anon_sym_LPAREN: Symbol = 14;
const anon_sym_MUTATION: Symbol = 40;
const anon_sym_OBJECT: Symbol = 49;
const anon_sym_PIPE: Symbol = 17;
const anon_sym_QUERY: Symbol = 39;
const anon_sym_RBRACE: Symbol = 3;
const anon_sym_RBRACK: Symbol = 32;
const anon_sym_RPAREN: Symbol = 15;
const anon_sym_SCALAR: Symbol = 48;
const anon_sym_SCHEMA: Symbol = 47;
const anon_sym_SUBSCRIPTION: Symbol = 41;
const anon_sym_UNION: Symbol = 53;
const anon_sym_VARIABLE_DEFINITION: Symbol = 46;
const anon_sym_directive: Symbol = 37;
const anon_sym_enum: Symbol = 9;
const anon_sym_extend: Symbol = 4;
const anon_sym_false: Symbol = 29;
const anon_sym_fragment: Symbol = 34;
const anon_sym_implements: Symbol = 12;
const anon_sym_input: Symbol = 10;
const anon_sym_interface: Symbol = 7;
const anon_sym_mutation: Symbol = 19;
const anon_sym_on: Symbol = 35;
const anon_sym_query: Symbol = 18;
const anon_sym_repeatable: Symbol = 38;
const anon_sym_scalar: Symbol = 5;
const anon_sym_schema: Symbol = 1;
const anon_sym_subscription: Symbol = 20;
const anon_sym_true: Symbol = 28;
const anon_sym_type: Symbol = 6;
const anon_sym_union: Symbol = 8;
const aux_sym_string_value_token1: Symbol = 23;
const aux_sym_string_value_token2: Symbol = 25;
const sym_comma: Symbol = 61;
const sym_comment: Symbol = 60;
const sym_float_value: Symbol = 27;
const sym_int_value: Symbol = 26;
const sym_name: Symbol = 59;
const sym_null_value: Symbol = 30;
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
                if eof { state = 269; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 338), (34, 296), (35, 353), (36, 290), (38, 280), (40, 283), (41, 284), (44, 354),
                    (45, 8), (46, 7), (48, 300), (58, 282), (61, 285), (64, 316), (65, 134), (69, 114),
                    (70, 68), (73, 104), (77, 157), (79, 20), (81, 159), (83, 24), (85, 105), (86, 10),
                    (91, 310), (93, 311), (100, 205), (101, 228), (102, 171), (105, 217), (109, 259), (110, 260),
                    (111, 222), (113, 258), (114, 195), (115, 182), (116, 244), (117, 227), (123, 271), (124, 286),
                    (125, 272),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 65279 { state = 0; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 301; lexer.advance(false); continue; }
                return result;
            }
            1 => {
                if let Some(next) = advance_map(&[
                    (33, 338), (34, 296), (35, 353), (36, 290), (38, 280), (40, 283), (41, 284), (44, 354),
                    (46, 7), (58, 282), (61, 285), (64, 316), (91, 310), (93, 311), (123, 271), (124, 286),
                    (125, 272),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 65279 { state = 1; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 350; lexer.advance(false); continue; }
                return result;
            }
            2 => {
                if let Some(next) = advance_map(&[
                    (34, 296), (35, 353), (36, 290), (44, 354), (45, 8), (48, 300), (91, 310), (93, 311),
                    (102, 339), (110, 349), (116, 346), (123, 271),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 65279 { state = 2; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 301; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 350; lexer.advance(false); continue; }
                return result;
            }
            3 => {
                if lookahead == 34 { state = 291; lexer.advance(false); continue; }
                return result;
            }
            4 => {
                if lookahead == 34 { state = 295; lexer.advance(false); continue; }
                if lookahead == 35 { state = 353; lexer.advance(false); continue; }
                if lookahead == 44 { state = 354; lexer.advance(false); continue; }
                if lookahead == 64 { state = 316; lexer.advance(false); continue; }
                if lookahead == 111 { state = 345; lexer.advance(false); continue; }
                if lookahead == 123 { state = 271; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 65279 { state = 4; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 350; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                if lookahead == 34 { state = 268; lexer.advance(false); continue; }
                if lookahead != 0 { state = 294; lexer.advance(false); continue; }
                return result;
            }
            6 => {
                if lookahead == 46 { state = 312; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if lookahead == 46 { state = 6; lexer.advance(false); continue; }
                return result;
            }
            8 => {
                if lookahead == 48 { state = 300; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 301; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if lookahead == 65 { state = 327; lexer.advance(false); continue; }
                return result;
            }
            10 => {
                if lookahead == 65 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if lookahead == 65 { state = 66; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if lookahead == 65 { state = 97; lexer.advance(false); continue; }
                if lookahead == 72 { state = 43; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if lookahead == 65 { state = 22; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if lookahead == 65 { state = 150; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if lookahead == 65 { state = 27; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if lookahead == 65 { state = 98; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if lookahead == 65 { state = 30; lexer.advance(false); continue; }
                return result;
            }
            18 => {
                if lookahead == 65 { state = 137; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if lookahead == 65 { state = 67; lexer.advance(false); continue; }
                return result;
            }
            20 => {
                if lookahead == 66 { state = 91; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                if lookahead == 66 { state = 142; lexer.advance(false); continue; }
                return result;
            }
            22 => {
                if lookahead == 66 { state = 96; lexer.advance(false); continue; }
                return result;
            }
            23 => {
                if lookahead == 66 { state = 92; lexer.advance(false); continue; }
                return result;
            }
            24 => {
                if lookahead == 67 { state = 12; lexer.advance(false); continue; }
                if lookahead == 85 { state = 21; lexer.advance(false); continue; }
                return result;
            }
            25 => {
                if lookahead == 67 { state = 144; lexer.advance(false); continue; }
                return result;
            }
            26 => {
                if lookahead == 67 { state = 145; lexer.advance(false); continue; }
                return result;
            }
            27 => {
                if lookahead == 67 { state = 39; lexer.advance(false); continue; }
                return result;
            }
            28 => {
                if lookahead == 67 { state = 139; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                if lookahead == 68 { state = 322; lexer.advance(false); continue; }
                return result;
            }
            30 => {
                if lookahead == 68 { state = 324; lexer.advance(false); continue; }
                return result;
            }
            31 => {
                if lookahead == 68 { state = 44; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                if lookahead == 68 { state = 52; lexer.advance(false); continue; }
                return result;
            }
            33 => {
                if lookahead == 68 { state = 53; lexer.advance(false); continue; }
                if lookahead == 83 { state = 132; lexer.advance(false); continue; }
                return result;
            }
            34 => {
                if lookahead == 68 { state = 54; lexer.advance(false); continue; }
                return result;
            }
            35 => {
                if lookahead == 68 { state = 55; lexer.advance(false); continue; }
                return result;
            }
            36 => {
                if lookahead == 68 { state = 170; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                if lookahead == 69 { state = 93; lexer.advance(false); continue; }
                return result;
            }
            38 => {
                if lookahead == 69 { state = 25; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                if lookahead == 69 { state = 332; lexer.advance(false); continue; }
                return result;
            }
            40 => {
                if lookahead == 69 { state = 335; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                if lookahead == 69 { state = 167; lexer.advance(false); continue; }
                return result;
            }
            42 => {
                if lookahead == 69 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            43 => {
                if lookahead == 69 { state = 100; lexer.advance(false); continue; }
                return result;
            }
            44 => {
                if lookahead == 69 { state = 60; lexer.advance(false); continue; }
                return result;
            }
            45 => {
                if lookahead == 69 { state = 136; lexer.advance(false); continue; }
                return result;
            }
            46 => {
                if lookahead == 69 { state = 115; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                if lookahead == 69 { state = 117; lexer.advance(false); continue; }
                return result;
            }
            48 => {
                if lookahead == 69 { state = 17; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                if lookahead == 69 { state = 119; lexer.advance(false); continue; }
                return result;
            }
            50 => {
                if lookahead == 69 { state = 95; lexer.advance(false); continue; }
                return result;
            }
            51 => {
                if lookahead == 69 { state = 26; lexer.advance(false); continue; }
                return result;
            }
            52 => {
                if lookahead == 69 { state = 61; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                if lookahead == 69 { state = 62; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                if lookahead == 69 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            55 => {
                if lookahead == 69 { state = 64; lexer.advance(false); continue; }
                return result;
            }
            56 => {
                if lookahead == 69 { state = 169; lexer.advance(false); continue; }
                return result;
            }
            57 => {
                if lookahead == 70 { state = 81; lexer.advance(false); continue; }
                if lookahead == 79 { state = 23; lexer.advance(false); continue; }
                return result;
            }
            58 => {
                if lookahead == 70 { state = 141; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                if lookahead == 70 { state = 15; lexer.advance(false); continue; }
                return result;
            }
            60 => {
                if lookahead == 70 { state = 73; lexer.advance(false); continue; }
                return result;
            }
            61 => {
                if lookahead == 70 { state = 87; lexer.advance(false); continue; }
                return result;
            }
            62 => {
                if lookahead == 70 { state = 88; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                if lookahead == 70 { state = 89; lexer.advance(false); continue; }
                return result;
            }
            64 => {
                if lookahead == 70 { state = 90; lexer.advance(false); continue; }
                return result;
            }
            65 => {
                if lookahead == 71 { state = 161; lexer.advance(false); continue; }
                return result;
            }
            66 => {
                if lookahead == 71 { state = 102; lexer.advance(false); continue; }
                return result;
            }
            67 => {
                if lookahead == 71 { state = 103; lexer.advance(false); continue; }
                return result;
            }
            68 => {
                if lookahead == 73 { state = 37; lexer.advance(false); continue; }
                if lookahead == 82 { state = 11; lexer.advance(false); continue; }
                return result;
            }
            69 => {
                if lookahead == 73 { state = 124; lexer.advance(false); continue; }
                return result;
            }
            70 => {
                if lookahead == 73 { state = 133; lexer.advance(false); continue; }
                return result;
            }
            71 => {
                if lookahead == 73 { state = 116; lexer.advance(false); continue; }
                return result;
            }
            72 => {
                if lookahead == 73 { state = 13; lexer.advance(false); continue; }
                return result;
            }
            73 => {
                if lookahead == 73 { state = 118; lexer.advance(false); continue; }
                return result;
            }
            74 => {
                if lookahead == 73 { state = 125; lexer.advance(false); continue; }
                return result;
            }
            75 => {
                if lookahead == 73 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            76 => {
                if lookahead == 73 { state = 127; lexer.advance(false); continue; }
                return result;
            }
            77 => {
                if lookahead == 73 { state = 128; lexer.advance(false); continue; }
                return result;
            }
            78 => {
                if lookahead == 73 { state = 129; lexer.advance(false); continue; }
                return result;
            }
            79 => {
                if lookahead == 73 { state = 130; lexer.advance(false); continue; }
                return result;
            }
            80 => {
                if lookahead == 73 { state = 131; lexer.advance(false); continue; }
                return result;
            }
            81 => {
                if lookahead == 73 { state = 50; lexer.advance(false); continue; }
                return result;
            }
            82 => {
                if lookahead == 73 { state = 152; lexer.advance(false); continue; }
                return result;
            }
            83 => {
                if lookahead == 73 { state = 153; lexer.advance(false); continue; }
                return result;
            }
            84 => {
                if lookahead == 73 { state = 154; lexer.advance(false); continue; }
                return result;
            }
            85 => {
                if lookahead == 73 { state = 155; lexer.advance(false); continue; }
                return result;
            }
            86 => {
                if lookahead == 73 { state = 156; lexer.advance(false); continue; }
                return result;
            }
            87 => {
                if lookahead == 73 { state = 120; lexer.advance(false); continue; }
                return result;
            }
            88 => {
                if lookahead == 73 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            89 => {
                if lookahead == 73 { state = 122; lexer.advance(false); continue; }
                return result;
            }
            90 => {
                if lookahead == 73 { state = 123; lexer.advance(false); continue; }
                return result;
            }
            91 => {
                if lookahead == 74 { state = 38; lexer.advance(false); continue; }
                return result;
            }
            92 => {
                if lookahead == 74 { state = 51; lexer.advance(false); continue; }
                return result;
            }
            93 => {
                if lookahead == 76 { state = 29; lexer.advance(false); continue; }
                return result;
            }
            94 => {
                if lookahead == 76 { state = 71; lexer.advance(false); continue; }
                if lookahead == 80 { state = 160; lexer.advance(false); continue; }
                if lookahead == 84 { state = 45; lexer.advance(false); continue; }
                return result;
            }
            95 => {
                if lookahead == 76 { state = 36; lexer.advance(false); continue; }
                return result;
            }
            96 => {
                if lookahead == 76 { state = 56; lexer.advance(false); continue; }
                return result;
            }
            97 => {
                if lookahead == 76 { state = 18; lexer.advance(false); continue; }
                return result;
            }
            98 => {
                if lookahead == 76 { state = 162; lexer.advance(false); continue; }
                return result;
            }
            99 => {
                if lookahead == 77 { state = 334; lexer.advance(false); continue; }
                return result;
            }
            100 => {
                if lookahead == 77 { state = 9; lexer.advance(false); continue; }
                return result;
            }
            101 => {
                if lookahead == 77 { state = 46; lexer.advance(false); continue; }
                return result;
            }
            102 => {
                if lookahead == 77 { state = 47; lexer.advance(false); continue; }
                return result;
            }
            103 => {
                if lookahead == 77 { state = 49; lexer.advance(false); continue; }
                return result;
            }
            104 => {
                if lookahead == 78 { state = 94; lexer.advance(false); continue; }
                return result;
            }
            105 => {
                if lookahead == 78 { state = 69; lexer.advance(false); continue; }
                return result;
            }
            106 => {
                if lookahead == 78 { state = 333; lexer.advance(false); continue; }
                return result;
            }
            107 => {
                if lookahead == 78 { state = 320; lexer.advance(false); continue; }
                return result;
            }
            108 => {
                if lookahead == 78 { state = 321; lexer.advance(false); continue; }
                return result;
            }
            109 => {
                if lookahead == 78 { state = 330; lexer.advance(false); continue; }
                return result;
            }
            110 => {
                if lookahead == 78 { state = 331; lexer.advance(false); continue; }
                return result;
            }
            111 => {
                if lookahead == 78 { state = 323; lexer.advance(false); continue; }
                return result;
            }
            112 => {
                if lookahead == 78 { state = 326; lexer.advance(false); continue; }
                return result;
            }
            113 => {
                if lookahead == 78 { state = 337; lexer.advance(false); continue; }
                return result;
            }
            114 => {
                if lookahead == 78 { state = 158; lexer.advance(false); continue; }
                return result;
            }
            115 => {
                if lookahead == 78 { state = 148; lexer.advance(false); continue; }
                return result;
            }
            116 => {
                if lookahead == 78 { state = 41; lexer.advance(false); continue; }
                return result;
            }
            117 => {
                if lookahead == 78 { state = 149; lexer.advance(false); continue; }
                return result;
            }
            118 => {
                if lookahead == 78 { state = 82; lexer.advance(false); continue; }
                return result;
            }
            119 => {
                if lookahead == 78 { state = 146; lexer.advance(false); continue; }
                return result;
            }
            120 => {
                if lookahead == 78 { state = 83; lexer.advance(false); continue; }
                return result;
            }
            121 => {
                if lookahead == 78 { state = 84; lexer.advance(false); continue; }
                return result;
            }
            122 => {
                if lookahead == 78 { state = 85; lexer.advance(false); continue; }
                return result;
            }
            123 => {
                if lookahead == 78 { state = 86; lexer.advance(false); continue; }
                return result;
            }
            124 => {
                if lookahead == 79 { state = 106; lexer.advance(false); continue; }
                return result;
            }
            125 => {
                if lookahead == 79 { state = 107; lexer.advance(false); continue; }
                return result;
            }
            126 => {
                if lookahead == 79 { state = 108; lexer.advance(false); continue; }
                return result;
            }
            127 => {
                if lookahead == 79 { state = 109; lexer.advance(false); continue; }
                return result;
            }
            128 => {
                if lookahead == 79 { state = 110; lexer.advance(false); continue; }
                return result;
            }
            129 => {
                if lookahead == 79 { state = 111; lexer.advance(false); continue; }
                return result;
            }
            130 => {
                if lookahead == 79 { state = 112; lexer.advance(false); continue; }
                return result;
            }
            131 => {
                if lookahead == 79 { state = 113; lexer.advance(false); continue; }
                return result;
            }
            132 => {
                if lookahead == 80 { state = 140; lexer.advance(false); continue; }
                return result;
            }
            133 => {
                if lookahead == 80 { state = 151; lexer.advance(false); continue; }
                return result;
            }
            134 => {
                if lookahead == 82 { state = 65; lexer.advance(false); continue; }
                return result;
            }
            135 => {
                if lookahead == 82 { state = 164; lexer.advance(false); continue; }
                return result;
            }
            136 => {
                if lookahead == 82 { state = 59; lexer.advance(false); continue; }
                return result;
            }
            137 => {
                if lookahead == 82 { state = 328; lexer.advance(false); continue; }
                return result;
            }
            138 => {
                if lookahead == 82 { state = 72; lexer.advance(false); continue; }
                return result;
            }
            139 => {
                if lookahead == 82 { state = 70; lexer.advance(false); continue; }
                return result;
            }
            140 => {
                if lookahead == 82 { state = 48; lexer.advance(false); continue; }
                return result;
            }
            141 => {
                if lookahead == 82 { state = 19; lexer.advance(false); continue; }
                return result;
            }
            142 => {
                if lookahead == 83 { state = 28; lexer.advance(false); continue; }
                return result;
            }
            143 => {
                if lookahead == 84 { state = 165; lexer.advance(false); continue; }
                return result;
            }
            144 => {
                if lookahead == 84 { state = 329; lexer.advance(false); continue; }
                return result;
            }
            145 => {
                if lookahead == 84 { state = 336; lexer.advance(false); continue; }
                return result;
            }
            146 => {
                if lookahead == 84 { state = 325; lexer.advance(false); continue; }
                return result;
            }
            147 => {
                if lookahead == 84 { state = 14; lexer.advance(false); continue; }
                return result;
            }
            148 => {
                if lookahead == 84 { state = 168; lexer.advance(false); continue; }
                return result;
            }
            149 => {
                if lookahead == 84 { state = 166; lexer.advance(false); continue; }
                return result;
            }
            150 => {
                if lookahead == 84 { state = 74; lexer.advance(false); continue; }
                return result;
            }
            151 => {
                if lookahead == 84 { state = 75; lexer.advance(false); continue; }
                return result;
            }
            152 => {
                if lookahead == 84 { state = 76; lexer.advance(false); continue; }
                return result;
            }
            153 => {
                if lookahead == 84 { state = 77; lexer.advance(false); continue; }
                return result;
            }
            154 => {
                if lookahead == 84 { state = 78; lexer.advance(false); continue; }
                return result;
            }
            155 => {
                if lookahead == 84 { state = 79; lexer.advance(false); continue; }
                return result;
            }
            156 => {
                if lookahead == 84 { state = 80; lexer.advance(false); continue; }
                return result;
            }
            157 => {
                if lookahead == 85 { state = 147; lexer.advance(false); continue; }
                return result;
            }
            158 => {
                if lookahead == 85 { state = 99; lexer.advance(false); continue; }
                return result;
            }
            159 => {
                if lookahead == 85 { state = 42; lexer.advance(false); continue; }
                return result;
            }
            160 => {
                if lookahead == 85 { state = 143; lexer.advance(false); continue; }
                return result;
            }
            161 => {
                if lookahead == 85 { state = 101; lexer.advance(false); continue; }
                return result;
            }
            162 => {
                if lookahead == 85 { state = 40; lexer.advance(false); continue; }
                return result;
            }
            163 => {
                if lookahead == 86 { state = 16; lexer.advance(false); continue; }
                return result;
            }
            164 => {
                if lookahead == 89 { state = 319; lexer.advance(false); continue; }
                return result;
            }
            165 => {
                if lookahead == 95 { state = 57; lexer.advance(false); continue; }
                return result;
            }
            166 => {
                if lookahead == 95 { state = 33; lexer.advance(false); continue; }
                return result;
            }
            167 => {
                if lookahead == 95 { state = 58; lexer.advance(false); continue; }
                return result;
            }
            168 => {
                if lookahead == 95 { state = 32; lexer.advance(false); continue; }
                return result;
            }
            169 => {
                if lookahead == 95 { state = 34; lexer.advance(false); continue; }
                return result;
            }
            170 => {
                if lookahead == 95 { state = 35; lexer.advance(false); continue; }
                return result;
            }
            171 => {
                if lookahead == 97 { state = 211; lexer.advance(false); continue; }
                if lookahead == 114 { state = 172; lexer.advance(false); continue; }
                return result;
            }
            172 => {
                if lookahead == 97 { state = 204; lexer.advance(false); continue; }
                return result;
            }
            173 => {
                if lookahead == 97 { state = 270; lexer.advance(false); continue; }
                return result;
            }
            174 => {
                if lookahead == 97 { state = 181; lexer.advance(false); continue; }
                return result;
            }
            175 => {
                if lookahead == 97 { state = 185; lexer.advance(false); continue; }
                return result;
            }
            176 => {
                if lookahead == 97 { state = 241; lexer.advance(false); continue; }
                return result;
            }
            177 => {
                if lookahead == 97 { state = 255; lexer.advance(false); continue; }
                return result;
            }
            178 => {
                if lookahead == 97 { state = 214; lexer.advance(false); continue; }
                if lookahead == 104 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            179 => {
                if lookahead == 97 { state = 254; lexer.advance(false); continue; }
                return result;
            }
            180 => {
                if lookahead == 98 { state = 246; lexer.advance(false); continue; }
                return result;
            }
            181 => {
                if lookahead == 98 { state = 216; lexer.advance(false); continue; }
                return result;
            }
            182 => {
                if lookahead == 99 { state = 178; lexer.advance(false); continue; }
                if lookahead == 117 { state = 180; lexer.advance(false); continue; }
                return result;
            }
            183 => {
                if lookahead == 99 { state = 243; lexer.advance(false); continue; }
                return result;
            }
            184 => {
                if lookahead == 99 { state = 251; lexer.advance(false); continue; }
                return result;
            }
            185 => {
                if lookahead == 99 { state = 192; lexer.advance(false); continue; }
                return result;
            }
            186 => {
                if lookahead == 100 { state = 273; lexer.advance(false); continue; }
                return result;
            }
            187 => {
                if lookahead == 101 { state = 184; lexer.advance(false); continue; }
                return result;
            }
            188 => {
                if lookahead == 101 { state = 304; lexer.advance(false); continue; }
                return result;
            }
            189 => {
                if lookahead == 101 { state = 275; lexer.advance(false); continue; }
                return result;
            }
            190 => {
                if lookahead == 101 { state = 306; lexer.advance(false); continue; }
                return result;
            }
            191 => {
                if lookahead == 101 { state = 317; lexer.advance(false); continue; }
                return result;
            }
            192 => {
                if lookahead == 101 { state = 276; lexer.advance(false); continue; }
                return result;
            }
            193 => {
                if lookahead == 101 { state = 318; lexer.advance(false); continue; }
                return result;
            }
            194 => {
                if lookahead == 101 { state = 239; lexer.advance(false); continue; }
                return result;
            }
            195 => {
                if lookahead == 101 { state = 235; lexer.advance(false); continue; }
                return result;
            }
            196 => {
                if lookahead == 101 { state = 223; lexer.advance(false); continue; }
                return result;
            }
            197 => {
                if lookahead == 101 { state = 240; lexer.advance(false); continue; }
                return result;
            }
            198 => {
                if lookahead == 101 { state = 179; lexer.advance(false); continue; }
                return result;
            }
            199 => {
                if lookahead == 101 { state = 219; lexer.advance(false); continue; }
                return result;
            }
            200 => {
                if lookahead == 101 { state = 229; lexer.advance(false); continue; }
                return result;
            }
            201 => {
                if lookahead == 101 { state = 230; lexer.advance(false); continue; }
                return result;
            }
            202 => {
                if lookahead == 101 { state = 221; lexer.advance(false); continue; }
                return result;
            }
            203 => {
                if lookahead == 102 { state = 175; lexer.advance(false); continue; }
                return result;
            }
            204 => {
                if lookahead == 103 { state = 220; lexer.advance(false); continue; }
                return result;
            }
            205 => {
                if lookahead == 105 { state = 242; lexer.advance(false); continue; }
                return result;
            }
            206 => {
                if lookahead == 105 { state = 231; lexer.advance(false); continue; }
                return result;
            }
            207 => {
                if lookahead == 105 { state = 263; lexer.advance(false); continue; }
                return result;
            }
            208 => {
                if lookahead == 105 { state = 238; lexer.advance(false); continue; }
                return result;
            }
            209 => {
                if lookahead == 105 { state = 232; lexer.advance(false); continue; }
                return result;
            }
            210 => {
                if lookahead == 105 { state = 233; lexer.advance(false); continue; }
                return result;
            }
            211 => {
                if lookahead == 108 { state = 247; lexer.advance(false); continue; }
                return result;
            }
            212 => {
                if lookahead == 108 { state = 308; lexer.advance(false); continue; }
                return result;
            }
            213 => {
                if lookahead == 108 { state = 212; lexer.advance(false); continue; }
                return result;
            }
            214 => {
                if lookahead == 108 { state = 176; lexer.advance(false); continue; }
                return result;
            }
            215 => {
                if lookahead == 108 { state = 202; lexer.advance(false); continue; }
                return result;
            }
            216 => {
                if lookahead == 108 { state = 193; lexer.advance(false); continue; }
                return result;
            }
            217 => {
                if lookahead == 109 { state = 234; lexer.advance(false); continue; }
                if lookahead == 110 { state = 237; lexer.advance(false); continue; }
                return result;
            }
            218 => {
                if lookahead == 109 { state = 278; lexer.advance(false); continue; }
                return result;
            }
            219 => {
                if lookahead == 109 { state = 173; lexer.advance(false); continue; }
                return result;
            }
            220 => {
                if lookahead == 109 { state = 200; lexer.advance(false); continue; }
                return result;
            }
            221 => {
                if lookahead == 109 { state = 201; lexer.advance(false); continue; }
                return result;
            }
            222 => {
                if lookahead == 110 { state = 314; lexer.advance(false); continue; }
                return result;
            }
            223 => {
                if lookahead == 110 { state = 186; lexer.advance(false); continue; }
                return result;
            }
            224 => {
                if lookahead == 110 { state = 277; lexer.advance(false); continue; }
                return result;
            }
            225 => {
                if lookahead == 110 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            226 => {
                if lookahead == 110 { state = 289; lexer.advance(false); continue; }
                return result;
            }
            227 => {
                if lookahead == 110 { state = 206; lexer.advance(false); continue; }
                return result;
            }
            228 => {
                if lookahead == 110 { state = 257; lexer.advance(false); continue; }
                if lookahead == 120 { state = 253; lexer.advance(false); continue; }
                return result;
            }
            229 => {
                if lookahead == 110 { state = 249; lexer.advance(false); continue; }
                return result;
            }
            230 => {
                if lookahead == 110 { state = 252; lexer.advance(false); continue; }
                return result;
            }
            231 => {
                if lookahead == 111 { state = 224; lexer.advance(false); continue; }
                return result;
            }
            232 => {
                if lookahead == 111 { state = 225; lexer.advance(false); continue; }
                return result;
            }
            233 => {
                if lookahead == 111 { state = 226; lexer.advance(false); continue; }
                return result;
            }
            234 => {
                if lookahead == 112 { state = 215; lexer.advance(false); continue; }
                return result;
            }
            235 => {
                if lookahead == 112 { state = 198; lexer.advance(false); continue; }
                return result;
            }
            236 => {
                if lookahead == 112 { state = 189; lexer.advance(false); continue; }
                return result;
            }
            237 => {
                if lookahead == 112 { state = 261; lexer.advance(false); continue; }
                if lookahead == 116 { state = 197; lexer.advance(false); continue; }
                return result;
            }
            238 => {
                if lookahead == 112 { state = 256; lexer.advance(false); continue; }
                return result;
            }
            239 => {
                if lookahead == 114 { state = 264; lexer.advance(false); continue; }
                return result;
            }
            240 => {
                if lookahead == 114 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            241 => {
                if lookahead == 114 { state = 274; lexer.advance(false); continue; }
                return result;
            }
            242 => {
                if lookahead == 114 { state = 187; lexer.advance(false); continue; }
                return result;
            }
            243 => {
                if lookahead == 114 { state = 208; lexer.advance(false); continue; }
                return result;
            }
            244 => {
                if lookahead == 114 { state = 262; lexer.advance(false); continue; }
                if lookahead == 121 { state = 236; lexer.advance(false); continue; }
                return result;
            }
            245 => {
                if lookahead == 115 { state = 281; lexer.advance(false); continue; }
                return result;
            }
            246 => {
                if lookahead == 115 { state = 183; lexer.advance(false); continue; }
                return result;
            }
            247 => {
                if lookahead == 115 { state = 190; lexer.advance(false); continue; }
                return result;
            }
            248 => {
                if lookahead == 116 { state = 279; lexer.advance(false); continue; }
                return result;
            }
            249 => {
                if lookahead == 116 { state = 313; lexer.advance(false); continue; }
                return result;
            }
            250 => {
                if lookahead == 116 { state = 177; lexer.advance(false); continue; }
                return result;
            }
            251 => {
                if lookahead == 116 { state = 207; lexer.advance(false); continue; }
                return result;
            }
            252 => {
                if lookahead == 116 { state = 245; lexer.advance(false); continue; }
                return result;
            }
            253 => {
                if lookahead == 116 { state = 196; lexer.advance(false); continue; }
                return result;
            }
            254 => {
                if lookahead == 116 { state = 174; lexer.advance(false); continue; }
                return result;
            }
            255 => {
                if lookahead == 116 { state = 209; lexer.advance(false); continue; }
                return result;
            }
            256 => {
                if lookahead == 116 { state = 210; lexer.advance(false); continue; }
                return result;
            }
            257 => {
                if lookahead == 117 { state = 218; lexer.advance(false); continue; }
                return result;
            }
            258 => {
                if lookahead == 117 { state = 194; lexer.advance(false); continue; }
                return result;
            }
            259 => {
                if lookahead == 117 { state = 250; lexer.advance(false); continue; }
                return result;
            }
            260 => {
                if lookahead == 117 { state = 213; lexer.advance(false); continue; }
                return result;
            }
            261 => {
                if lookahead == 117 { state = 248; lexer.advance(false); continue; }
                return result;
            }
            262 => {
                if lookahead == 117 { state = 188; lexer.advance(false); continue; }
                return result;
            }
            263 => {
                if lookahead == 118 { state = 191; lexer.advance(false); continue; }
                return result;
            }
            264 => {
                if lookahead == 121 { state = 287; lexer.advance(false); continue; }
                return result;
            }
            265 => {
                if lookahead == 43 || lookahead == 45 { state = 267; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 303; lexer.advance(false); continue; }
                return result;
            }
            266 => {
                if 48 <= lookahead && lookahead <= 57 { state = 302; lexer.advance(false); continue; }
                return result;
            }
            267 => {
                if 48 <= lookahead && lookahead <= 57 { state = 303; lexer.advance(false); continue; }
                return result;
            }
            268 => {
                if lookahead != 0 && lookahead != 34 { state = 294; lexer.advance(false); continue; }
                return result;
            }
            269 => {
                result = true; lexer.set_result_symbol(ts_builtin_sym_end); lexer.mark_end();
                return result;
            }
            270 => {
                result = true; lexer.set_result_symbol(anon_sym_schema); lexer.mark_end();
                return result;
            }
            271 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACE); lexer.mark_end();
                return result;
            }
            272 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACE); lexer.mark_end();
                return result;
            }
            273 => {
                result = true; lexer.set_result_symbol(anon_sym_extend); lexer.mark_end();
                return result;
            }
            274 => {
                result = true; lexer.set_result_symbol(anon_sym_scalar); lexer.mark_end();
                return result;
            }
            275 => {
                result = true; lexer.set_result_symbol(anon_sym_type); lexer.mark_end();
                return result;
            }
            276 => {
                result = true; lexer.set_result_symbol(anon_sym_interface); lexer.mark_end();
                return result;
            }
            277 => {
                result = true; lexer.set_result_symbol(anon_sym_union); lexer.mark_end();
                return result;
            }
            278 => {
                result = true; lexer.set_result_symbol(anon_sym_enum); lexer.mark_end();
                return result;
            }
            279 => {
                result = true; lexer.set_result_symbol(anon_sym_input); lexer.mark_end();
                return result;
            }
            280 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP); lexer.mark_end();
                return result;
            }
            281 => {
                result = true; lexer.set_result_symbol(anon_sym_implements); lexer.mark_end();
                return result;
            }
            282 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON); lexer.mark_end();
                return result;
            }
            283 => {
                result = true; lexer.set_result_symbol(anon_sym_LPAREN); lexer.mark_end();
                return result;
            }
            284 => {
                result = true; lexer.set_result_symbol(anon_sym_RPAREN); lexer.mark_end();
                return result;
            }
            285 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                return result;
            }
            286 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                return result;
            }
            287 => {
                result = true; lexer.set_result_symbol(anon_sym_query); lexer.mark_end();
                return result;
            }
            288 => {
                result = true; lexer.set_result_symbol(anon_sym_mutation); lexer.mark_end();
                return result;
            }
            289 => {
                result = true; lexer.set_result_symbol(anon_sym_subscription); lexer.mark_end();
                return result;
            }
            290 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR); lexer.mark_end();
                return result;
            }
            291 => {
                result = true; lexer.set_result_symbol(anon_sym_DQUOTE_DQUOTE_DQUOTE); lexer.mark_end();
                return result;
            }
            292 => {
                result = true; lexer.set_result_symbol(aux_sym_string_value_token1); lexer.mark_end();
                if lookahead == 10 { state = 294; lexer.advance(false); continue; }
                if lookahead == 34 { state = 352; lexer.advance(false); continue; }
                if lookahead != 0 { state = 292; lexer.advance(false); continue; }
                return result;
            }
            293 => {
                result = true; lexer.set_result_symbol(aux_sym_string_value_token1); lexer.mark_end();
                if lookahead == 34 { state = 5; lexer.advance(false); continue; }
                if lookahead == 35 { state = 292; lexer.advance(false); continue; }
                if lookahead == 44 { state = 355; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 65279 { state = 293; lexer.advance(false); continue; }
                if lookahead != 0 { state = 294; lexer.advance(false); continue; }
                return result;
            }
            294 => {
                result = true; lexer.set_result_symbol(aux_sym_string_value_token1); lexer.mark_end();
                if lookahead == 34 { state = 5; lexer.advance(false); continue; }
                if lookahead != 0 { state = 294; lexer.advance(false); continue; }
                return result;
            }
            295 => {
                result = true; lexer.set_result_symbol(anon_sym_DQUOTE); lexer.mark_end();
                return result;
            }
            296 => {
                result = true; lexer.set_result_symbol(anon_sym_DQUOTE); lexer.mark_end();
                if lookahead == 34 { state = 3; lexer.advance(false); continue; }
                return result;
            }
            297 => {
                result = true; lexer.set_result_symbol(aux_sym_string_value_token2); lexer.mark_end();
                if lookahead == 35 { state = 298; lexer.advance(false); continue; }
                if lookahead == 44 { state = 356; lexer.advance(false); continue; }
                if lookahead == 9 || 11 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 65279 { state = 297; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 34 && lookahead != 35 && lookahead != 92 { state = 299; lexer.advance(false); continue; }
                return result;
            }
            298 => {
                result = true; lexer.set_result_symbol(aux_sym_string_value_token2); lexer.mark_end();
                if lookahead == 34 || lookahead == 92 { state = 353; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 298; lexer.advance(false); continue; }
                return result;
            }
            299 => {
                result = true; lexer.set_result_symbol(aux_sym_string_value_token2); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 && lookahead != 34 && lookahead != 92 { state = 299; lexer.advance(false); continue; }
                return result;
            }
            300 => {
                result = true; lexer.set_result_symbol(sym_int_value); lexer.mark_end();
                if lookahead == 46 { state = 266; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 265; lexer.advance(false); continue; }
                return result;
            }
            301 => {
                result = true; lexer.set_result_symbol(sym_int_value); lexer.mark_end();
                if lookahead == 46 { state = 266; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 265; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 301; lexer.advance(false); continue; }
                return result;
            }
            302 => {
                result = true; lexer.set_result_symbol(sym_float_value); lexer.mark_end();
                if lookahead == 69 || lookahead == 101 { state = 265; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 302; lexer.advance(false); continue; }
                return result;
            }
            303 => {
                result = true; lexer.set_result_symbol(sym_float_value); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 { state = 303; lexer.advance(false); continue; }
                return result;
            }
            304 => {
                result = true; lexer.set_result_symbol(anon_sym_true); lexer.mark_end();
                return result;
            }
            305 => {
                result = true; lexer.set_result_symbol(anon_sym_true); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 350; lexer.advance(false); continue; }
                return result;
            }
            306 => {
                result = true; lexer.set_result_symbol(anon_sym_false); lexer.mark_end();
                return result;
            }
            307 => {
                result = true; lexer.set_result_symbol(anon_sym_false); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 350; lexer.advance(false); continue; }
                return result;
            }
            308 => {
                result = true; lexer.set_result_symbol(sym_null_value); lexer.mark_end();
                return result;
            }
            309 => {
                result = true; lexer.set_result_symbol(sym_null_value); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 350; lexer.advance(false); continue; }
                return result;
            }
            310 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK); lexer.mark_end();
                return result;
            }
            311 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACK); lexer.mark_end();
                return result;
            }
            312 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT_DOT_DOT); lexer.mark_end();
                return result;
            }
            313 => {
                result = true; lexer.set_result_symbol(anon_sym_fragment); lexer.mark_end();
                return result;
            }
            314 => {
                result = true; lexer.set_result_symbol(anon_sym_on); lexer.mark_end();
                return result;
            }
            315 => {
                result = true; lexer.set_result_symbol(anon_sym_on); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 350; lexer.advance(false); continue; }
                return result;
            }
            316 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                return result;
            }
            317 => {
                result = true; lexer.set_result_symbol(anon_sym_directive); lexer.mark_end();
                return result;
            }
            318 => {
                result = true; lexer.set_result_symbol(anon_sym_repeatable); lexer.mark_end();
                return result;
            }
            319 => {
                result = true; lexer.set_result_symbol(anon_sym_QUERY); lexer.mark_end();
                return result;
            }
            320 => {
                result = true; lexer.set_result_symbol(anon_sym_MUTATION); lexer.mark_end();
                return result;
            }
            321 => {
                result = true; lexer.set_result_symbol(anon_sym_SUBSCRIPTION); lexer.mark_end();
                return result;
            }
            322 => {
                result = true; lexer.set_result_symbol(anon_sym_FIELD); lexer.mark_end();
                if lookahead == 95 { state = 31; lexer.advance(false); continue; }
                return result;
            }
            323 => {
                result = true; lexer.set_result_symbol(anon_sym_FRAGMENT_DEFINITION); lexer.mark_end();
                return result;
            }
            324 => {
                result = true; lexer.set_result_symbol(anon_sym_FRAGMENT_SPREAD); lexer.mark_end();
                return result;
            }
            325 => {
                result = true; lexer.set_result_symbol(anon_sym_INLINE_FRAGMENT); lexer.mark_end();
                return result;
            }
            326 => {
                result = true; lexer.set_result_symbol(anon_sym_VARIABLE_DEFINITION); lexer.mark_end();
                return result;
            }
            327 => {
                result = true; lexer.set_result_symbol(anon_sym_SCHEMA); lexer.mark_end();
                return result;
            }
            328 => {
                result = true; lexer.set_result_symbol(anon_sym_SCALAR); lexer.mark_end();
                return result;
            }
            329 => {
                result = true; lexer.set_result_symbol(anon_sym_OBJECT); lexer.mark_end();
                return result;
            }
            330 => {
                result = true; lexer.set_result_symbol(anon_sym_FIELD_DEFINITION); lexer.mark_end();
                return result;
            }
            331 => {
                result = true; lexer.set_result_symbol(anon_sym_ARGUMENT_DEFINITION); lexer.mark_end();
                return result;
            }
            332 => {
                result = true; lexer.set_result_symbol(anon_sym_INTERFACE); lexer.mark_end();
                return result;
            }
            333 => {
                result = true; lexer.set_result_symbol(anon_sym_UNION); lexer.mark_end();
                return result;
            }
            334 => {
                result = true; lexer.set_result_symbol(anon_sym_ENUM); lexer.mark_end();
                if lookahead == 95 { state = 163; lexer.advance(false); continue; }
                return result;
            }
            335 => {
                result = true; lexer.set_result_symbol(anon_sym_ENUM_VALUE); lexer.mark_end();
                return result;
            }
            336 => {
                result = true; lexer.set_result_symbol(anon_sym_INPUT_OBJECT); lexer.mark_end();
                return result;
            }
            337 => {
                result = true; lexer.set_result_symbol(anon_sym_INPUT_FIELD_DEFINITION); lexer.mark_end();
                return result;
            }
            338 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG); lexer.mark_end();
                return result;
            }
            339 => {
                result = true; lexer.set_result_symbol(sym_name); lexer.mark_end();
                if lookahead == 97 { state = 342; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 350; lexer.advance(false); continue; }
                return result;
            }
            340 => {
                result = true; lexer.set_result_symbol(sym_name); lexer.mark_end();
                if lookahead == 101 { state = 305; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 350; lexer.advance(false); continue; }
                return result;
            }
            341 => {
                result = true; lexer.set_result_symbol(sym_name); lexer.mark_end();
                if lookahead == 101 { state = 307; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 350; lexer.advance(false); continue; }
                return result;
            }
            342 => {
                result = true; lexer.set_result_symbol(sym_name); lexer.mark_end();
                if lookahead == 108 { state = 347; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 350; lexer.advance(false); continue; }
                return result;
            }
            343 => {
                result = true; lexer.set_result_symbol(sym_name); lexer.mark_end();
                if lookahead == 108 { state = 309; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 350; lexer.advance(false); continue; }
                return result;
            }
            344 => {
                result = true; lexer.set_result_symbol(sym_name); lexer.mark_end();
                if lookahead == 108 { state = 343; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 350; lexer.advance(false); continue; }
                return result;
            }
            345 => {
                result = true; lexer.set_result_symbol(sym_name); lexer.mark_end();
                if lookahead == 110 { state = 315; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 350; lexer.advance(false); continue; }
                return result;
            }
            346 => {
                result = true; lexer.set_result_symbol(sym_name); lexer.mark_end();
                if lookahead == 114 { state = 348; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 350; lexer.advance(false); continue; }
                return result;
            }
            347 => {
                result = true; lexer.set_result_symbol(sym_name); lexer.mark_end();
                if lookahead == 115 { state = 341; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 350; lexer.advance(false); continue; }
                return result;
            }
            348 => {
                result = true; lexer.set_result_symbol(sym_name); lexer.mark_end();
                if lookahead == 117 { state = 340; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 350; lexer.advance(false); continue; }
                return result;
            }
            349 => {
                result = true; lexer.set_result_symbol(sym_name); lexer.mark_end();
                if lookahead == 117 { state = 344; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 350; lexer.advance(false); continue; }
                return result;
            }
            350 => {
                result = true; lexer.set_result_symbol(sym_name); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 350; lexer.advance(false); continue; }
                return result;
            }
            351 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 10 { state = 294; lexer.advance(false); continue; }
                if lookahead == 34 { state = 353; lexer.advance(false); continue; }
                if lookahead != 0 { state = 292; lexer.advance(false); continue; }
                return result;
            }
            352 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 10 { state = 294; lexer.advance(false); continue; }
                if lookahead == 34 { state = 351; lexer.advance(false); continue; }
                if lookahead != 0 { state = 292; lexer.advance(false); continue; }
                return result;
            }
            353 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 { state = 353; lexer.advance(false); continue; }
                return result;
            }
            354 => {
                result = true; lexer.set_result_symbol(sym_comma); lexer.mark_end();
                return result;
            }
            355 => {
                result = true; lexer.set_result_symbol(sym_comma); lexer.mark_end();
                if lookahead == 34 { state = 5; lexer.advance(false); continue; }
                if lookahead != 0 { state = 294; lexer.advance(false); continue; }
                return result;
            }
            356 => {
                result = true; lexer.set_result_symbol(sym_comma); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 && lookahead != 34 && lookahead != 92 { state = 299; lexer.advance(false); continue; }
                return result;
            }
            _ => return false,
        }
    }
}
