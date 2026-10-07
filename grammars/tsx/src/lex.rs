//! The `tsx` grammar's lexer: `ts_lex` and `ts_lex_keywords`, transliterated from
//! its `src/parser.c`, with the symbols and character sets they use.
//!
//! Generated from the grammar's `src/parser.c`: do not edit by hand.
//!
//! Each C `case` is a `match` arm, run once per character: `ADVANCE(n)` and
//! `SKIP(n)` set the state, call `lexer.advance(skip)` and start the next round
//! (C's `goto next_state`), `ACCEPT_TOKEN` sets the result and marks the end, and
//! `END_STATE()` returns the result.
#![allow(non_upper_case_globals, unreachable_code, clippy::all)]

use tree_sitter_language::{CharacterRange, Lexer, StateId, Symbol, set_contains};

const anon_sym_AMP: Symbol = 85;
const anon_sym_AMP_AMP: Symbol = 80;
const anon_sym_AMP_AMP_EQ: Symbol = 76;
const anon_sym_AMP_EQ: Symbol = 70;
const anon_sym_AT: Symbol = 126;
const anon_sym_BANG: Symbol = 21;
const anon_sym_BANG_EQ: Symbol = 97;
const anon_sym_BANG_EQ_EQ: Symbol = 98;
const anon_sym_BQUOTE: Symbol = 111;
const anon_sym_CARET: Symbol = 86;
const anon_sym_CARET_EQ: Symbol = 69;
const anon_sym_COLON: Symbol = 40;
const anon_sym_COMMA: Symbol = 11;
const anon_sym_DASH: Symbol = 89;
const anon_sym_DASH_DASH: Symbol = 106;
const anon_sym_DASH_EQ: Symbol = 65;
const anon_sym_DASH_QMARK_COLON: Symbol = 153;
const anon_sym_DOLLAR_LBRACE: Symbol = 112;
const anon_sym_DOT: Symbol = 50;
const anon_sym_DOT_DOT_DOT: Symbol = 79;
const anon_sym_DQUOTE: Symbol = 53;
const anon_sym_EQ: Symbol = 7;
const anon_sym_EQ_EQ: Symbol = 95;
const anon_sym_EQ_EQ_EQ: Symbol = 96;
const anon_sym_EQ_GT: Symbol = 60;
const anon_sym_GT: Symbol = 48;
const anon_sym_GT_EQ: Symbol = 99;
const anon_sym_GT_GT: Symbol = 82;
const anon_sym_GT_GT_EQ: Symbol = 72;
const anon_sym_GT_GT_GT: Symbol = 83;
const anon_sym_GT_GT_GT_EQ: Symbol = 73;
const anon_sym_LBRACE: Symbol = 10;
const anon_sym_LBRACE_PIPE: Symbol = 163;
const anon_sym_LBRACK: Symbol = 45;
const anon_sym_LPAREN: Symbol = 26;
const anon_sym_LT: Symbol = 93;
const anon_sym_LT_EQ: Symbol = 94;
const anon_sym_LT_LT: Symbol = 84;
const anon_sym_LT_LT_EQ: Symbol = 74;
const anon_sym_LT_SLASH: Symbol = 51;
const anon_sym_PERCENT: Symbol = 91;
const anon_sym_PERCENT_EQ: Symbol = 68;
const anon_sym_PIPE: Symbol = 87;
const anon_sym_PIPE_EQ: Symbol = 71;
const anon_sym_PIPE_PIPE: Symbol = 81;
const anon_sym_PIPE_PIPE_EQ: Symbol = 77;
const anon_sym_PIPE_RBRACE: Symbol = 164;
const anon_sym_PLUS: Symbol = 88;
const anon_sym_PLUS_EQ: Symbol = 64;
const anon_sym_PLUS_PLUS: Symbol = 105;
const anon_sym_PLUS_QMARK_COLON: Symbol = 154;
const anon_sym_QMARK: Symbol = 131;
const anon_sym_QMARK_COLON: Symbol = 155;
const anon_sym_QMARK_DOT: Symbol = 61;
const anon_sym_QMARK_QMARK: Symbol = 100;
const anon_sym_QMARK_QMARK_EQ: Symbol = 78;
const anon_sym_RBRACE: Symbol = 12;
const anon_sym_RBRACK: Symbol = 46;
const anon_sym_RPAREN: Symbol = 28;
const anon_sym_SEMI: Symbol = 27;
const anon_sym_SLASH: Symbol = 90;
const anon_sym_SLASH2: Symbol = 113;
const anon_sym_SLASH_EQ: Symbol = 67;
const anon_sym_SLASH_GT: Symbol = 52;
const anon_sym_SQUOTE: Symbol = 54;
const anon_sym_STAR: Symbol = 4;
const anon_sym_STAR_EQ: Symbol = 66;
const anon_sym_STAR_STAR: Symbol = 92;
const anon_sym_STAR_STAR_EQ: Symbol = 75;
const anon_sym_TILDE: Symbol = 102;
const anon_sym_abstract: Symbol = 144;
const anon_sym_accessor: Symbol = 145;
const anon_sym_any: Symbol = 138;
const anon_sym_as: Symbol = 8;
const anon_sym_assert: Symbol = 17;
const anon_sym_asserts: Symbol = 156;
const anon_sym_async: Symbol = 58;
const anon_sym_await: Symbol = 29;
const anon_sym_boolean: Symbol = 140;
const anon_sym_break: Symbol = 35;
const anon_sym_case: Symbol = 41;
const anon_sym_catch: Symbol = 42;
const anon_sym_class: Symbol = 57;
const anon_sym_const: Symbol = 20;
const anon_sym_continue: Symbol = 36;
const anon_sym_debugger: Symbol = 37;
const anon_sym_declare: Symbol = 132;
const anon_sym_default: Symbol = 5;
const anon_sym_delete: Symbol = 104;
const anon_sym_do: Symbol = 33;
const anon_sym_else: Symbol = 22;
const anon_sym_enum: Symbol = 152;
const anon_sym_export: Symbol = 3;
const anon_sym_extends: Symbol = 148;
const anon_sym_finally: Symbol = 43;
const anon_sym_for: Symbol = 25;
const anon_sym_from: Symbol = 15;
const anon_sym_function: Symbol = 59;
const anon_sym_get: Symbol = 129;
const anon_sym_global: Symbol = 150;
const anon_sym_if: Symbol = 23;
const anon_sym_implements: Symbol = 149;
const anon_sym_import: Symbol = 14;
const anon_sym_in: Symbol = 30;
const anon_sym_infer: Symbol = 157;
const anon_sym_instanceof: Symbol = 101;
const anon_sym_interface: Symbol = 151;
const anon_sym_is: Symbol = 158;
const anon_sym_keyof: Symbol = 159;
const anon_sym_let: Symbol = 19;
const anon_sym_meta: Symbol = 119;
const anon_sym_module: Symbol = 137;
const anon_sym_namespace: Symbol = 9;
const anon_sym_never: Symbol = 162;
const anon_sym_new: Symbol = 62;
const anon_sym_number: Symbol = 139;
const anon_sym_object: Symbol = 143;
const anon_sym_of: Symbol = 31;
const anon_sym_override: Symbol = 136;
const anon_sym_private: Symbol = 134;
const anon_sym_protected: Symbol = 135;
const anon_sym_public: Symbol = 133;
const anon_sym_readonly: Symbol = 128;
const anon_sym_require: Symbol = 147;
const anon_sym_return: Symbol = 38;
const anon_sym_satisfies: Symbol = 146;
const anon_sym_set: Symbol = 130;
const anon_sym_static: Symbol = 127;
const anon_sym_string: Symbol = 141;
const anon_sym_switch: Symbol = 24;
const anon_sym_symbol: Symbol = 142;
const anon_sym_target: Symbol = 118;
const anon_sym_throw: Symbol = 39;
const anon_sym_try: Symbol = 34;
const anon_sym_type: Symbol = 6;
const anon_sym_typeof: Symbol = 13;
const anon_sym_unique: Symbol = 160;
const anon_sym_unknown: Symbol = 161;
const anon_sym_using: Symbol = 63;
const anon_sym_var: Symbol = 18;
const anon_sym_void: Symbol = 103;
const anon_sym_while: Symbol = 32;
const anon_sym_with: Symbol = 16;
const anon_sym_yield: Symbol = 44;
const sym_comment: Symbol = 110;
const sym_escape_sequence: Symbol = 109;
const sym_false: Symbol = 123;
const sym_hash_bang_line: Symbol = 2;
const sym_html_character_reference: Symbol = 47;
const sym_identifier: Symbol = 1;
const sym_jsx_identifier: Symbol = 49;
const sym_null: Symbol = 124;
const sym_number: Symbol = 116;
const sym_private_property_identifier: Symbol = 117;
const sym_regex_flags: Symbol = 115;
const sym_regex_pattern: Symbol = 114;
const sym_super: Symbol = 121;
const sym_this: Symbol = 120;
const sym_true: Symbol = 122;
const sym_undefined: Symbol = 125;
const sym_unescaped_double_jsx_string_fragment: Symbol = 55;
const sym_unescaped_double_string_fragment: Symbol = 107;
const sym_unescaped_single_jsx_string_fragment: Symbol = 56;
const sym_unescaped_single_string_fragment: Symbol = 108;
const ts_builtin_sym_end: Symbol = 0;

#[rustfmt::skip]
static extras_character_set_1: [CharacterRange; 10] = [
    CharacterRange::new(9, 13), CharacterRange::new(32, 32), CharacterRange::new(160, 160), CharacterRange::new(5760, 5760), CharacterRange::new(8192, 8203), CharacterRange::new(8232, 8233),
    CharacterRange::new(8239, 8239), CharacterRange::new(8287, 8288), CharacterRange::new(12288, 12288), CharacterRange::new(65279, 65279),
];

#[rustfmt::skip]
static sym_identifier_character_set_1: [CharacterRange; 14] = [
    CharacterRange::new(36, 36), CharacterRange::new(65, 90), CharacterRange::new(92, 92), CharacterRange::new(95, 95), CharacterRange::new(97, 122), CharacterRange::new(127, 159),
    CharacterRange::new(161, 5759), CharacterRange::new(5761, 8191), CharacterRange::new(8204, 8231), CharacterRange::new(8234, 8238), CharacterRange::new(8240, 8286), CharacterRange::new(8289, 12287),
    CharacterRange::new(12289, 65278), CharacterRange::new(65280, 1114111),
];

#[rustfmt::skip]
static sym_identifier_character_set_2: [CharacterRange; 15] = [
    CharacterRange::new(36, 36), CharacterRange::new(48, 57), CharacterRange::new(65, 90), CharacterRange::new(92, 92), CharacterRange::new(95, 95), CharacterRange::new(97, 122),
    CharacterRange::new(127, 159), CharacterRange::new(161, 5759), CharacterRange::new(5761, 8191), CharacterRange::new(8204, 8231), CharacterRange::new(8234, 8238), CharacterRange::new(8240, 8286),
    CharacterRange::new(8289, 12287), CharacterRange::new(12289, 65278), CharacterRange::new(65280, 1114111),
];

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
                if eof { state = 129; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 143), (34, 160), (35, 6), (36, 281), (37, 224), (38, 203), (39, 161), (40, 144),
                    (41, 146), (42, 132), (43, 216), (44, 140), (45, 220), (46, 156), (47, 266), (48, 271),
                    (58, 147), (59, 145), (60, 229), (61, 136), (62, 152), (63, 287), (64, 284), (91, 148),
                    (92, 82), (93, 149), (94, 206), (96, 264), (123, 139), (124, 209), (125, 141), (126, 240),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 272; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 126; lexer.advance(true); continue; }
                if lookahead > 64 { state = 282; lexer.advance(false); continue; }
                return result;
            }
            1 => {
                if lookahead == 10 { state = 34; lexer.advance(true); continue; }
                if lookahead == 47 { state = 24; lexer.advance(false); continue; }
                if lookahead == 91 { state = 81; lexer.advance(false); continue; }
                if lookahead == 92 { state = 125; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 267; lexer.advance(false); continue; }
                if lookahead != 0 { state = 268; lexer.advance(false); continue; }
                return result;
            }
            2 => {
                if let Some(next) = advance_map(&[
                    (33, 143), (34, 160), (35, 80), (37, 224), (38, 203), (39, 161), (40, 144), (41, 146),
                    (42, 132), (43, 215), (44, 140), (45, 219), (46, 156), (47, 222), (48, 271), (58, 147),
                    (59, 145), (60, 230), (61, 136), (62, 152), (63, 287), (64, 284), (91, 148), (92, 84),
                    (93, 149), (94, 206), (96, 264), (123, 139), (124, 209), (125, 141), (126, 240),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 272; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 2; lexer.advance(true); continue; }
                if lookahead > 35 { state = 282; lexer.advance(false); continue; }
                return result;
            }
            3 => {
                if let Some(next) = advance_map(&[
                    (33, 143), (34, 160), (35, 80), (37, 224), (38, 203), (39, 161), (40, 144), (41, 146),
                    (42, 132), (43, 215), (44, 140), (45, 219), (46, 156), (47, 222), (48, 271), (58, 147),
                    (59, 145), (60, 230), (61, 136), (62, 152), (63, 287), (64, 284), (91, 148), (92, 84),
                    (93, 149), (94, 206), (96, 264), (123, 138), (124, 208), (125, 141), (126, 240),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 272; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 3; lexer.advance(true); continue; }
                if lookahead > 35 { state = 282; lexer.advance(false); continue; }
                return result;
            }
            4 => {
                if let Some(next) = advance_map(&[
                    (33, 143), (37, 223), (38, 204), (40, 144), (41, 146), (42, 133), (43, 214), (44, 140),
                    (45, 218), (46, 155), (47, 221), (58, 147), (59, 145), (60, 231), (61, 77), (62, 153),
                    (63, 31), (91, 148), (92, 84), (93, 149), (94, 205), (96, 264), (123, 138), (124, 210),
                    (125, 141),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 97 <= lookahead && lookahead <= 122 { state = 269; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 5; lexer.advance(true); continue; }
                if lookahead > 35 && (lookahead < 37 || 64 < lookahead) && (lookahead < 96 || 126 < lookahead) { state = 282; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                if let Some(next) = advance_map(&[
                    (33, 143), (37, 223), (38, 204), (40, 144), (41, 146), (42, 133), (43, 214), (44, 140),
                    (45, 218), (46, 155), (47, 221), (58, 147), (59, 145), (60, 231), (61, 77), (62, 153),
                    (63, 31), (91, 148), (92, 84), (93, 149), (94, 205), (96, 264), (123, 138), (124, 210),
                    (125, 141),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 5; lexer.advance(true); continue; }
                if lookahead > 35 && (lookahead < 37 || 64 < lookahead) && (lookahead < 123 || 126 < lookahead) { state = 282; lexer.advance(false); continue; }
                return result;
            }
            6 => {
                if lookahead == 33 { state = 130; lexer.advance(false); continue; }
                if lookahead == 92 { state = 83; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 283; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if let Some(next) = advance_map(&[
                    (33, 142), (34, 160), (35, 80), (38, 202), (39, 161), (40, 144), (41, 146), (42, 131),
                    (43, 214), (44, 140), (45, 218), (46, 156), (47, 221), (48, 271), (58, 147), (59, 145),
                    (60, 227), (61, 137), (62, 151), (63, 286), (64, 284), (91, 148), (92, 84), (93, 149),
                    (96, 264), (123, 138), (124, 212), (125, 141), (126, 240),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 272; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 7; lexer.advance(true); continue; }
                if lookahead > 35 && (lookahead < 37 || 64 < lookahead) && (lookahead < 91 || 94 < lookahead) { state = 282; lexer.advance(false); continue; }
                return result;
            }
            8 => {
                if let Some(next) = advance_map(&[
                    (33, 142), (34, 160), (35, 80), (38, 202), (39, 161), (40, 144), (41, 146), (42, 131),
                    (43, 214), (44, 140), (45, 218), (46, 156), (47, 221), (48, 271), (60, 227), (63, 285),
                    (64, 284), (91, 148), (92, 84), (93, 149), (96, 264), (123, 139), (124, 207), (126, 240),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 272; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 8; lexer.advance(true); continue; }
                if lookahead > 35 && (lookahead < 37 || 64 < lookahead) && (lookahead < 91 || 94 < lookahead) && (lookahead < 123 || 126 < lookahead) { state = 282; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if let Some(next) = advance_map(&[
                    (34, 160), (35, 80), (38, 202), (39, 161), (40, 144), (42, 131), (43, 213), (44, 140),
                    (45, 217), (46, 30), (47, 24), (48, 271), (59, 145), (60, 227), (62, 151), (63, 285),
                    (64, 284), (91, 148), (92, 84), (93, 149), (96, 264), (123, 139), (124, 212), (125, 141),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 272; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 9; lexer.advance(true); continue; }
                if lookahead > 35 && (lookahead < 37 || 64 < lookahead) && (lookahead < 91 || 94 < lookahead) && (lookahead < 123 || 126 < lookahead) { state = 282; lexer.advance(false); continue; }
                return result;
            }
            10 => {
                if let Some(next) = advance_map(&[
                    (34, 160), (35, 80), (39, 161), (40, 144), (42, 131), (43, 213), (44, 140), (45, 217),
                    (46, 30), (47, 24), (48, 271), (59, 145), (60, 227), (64, 284), (91, 148), (92, 84),
                    (123, 138), (124, 94), (125, 141),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 272; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 10; lexer.advance(true); continue; }
                if lookahead > 35 && (lookahead < 37 || 64 < lookahead) && (lookahead < 91 || 94 < lookahead) && lookahead != 96 && (lookahead < 123 || 126 < lookahead) { state = 282; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if let Some(next) = advance_map(&[
                    (34, 160), (38, 202), (39, 161), (40, 144), (42, 131), (43, 213), (45, 217), (46, 101),
                    (47, 25), (48, 271), (60, 227), (62, 151), (63, 285), (91, 148), (92, 84), (96, 264),
                    (123, 139), (124, 207),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 272; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 11; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 { state = 280; lexer.advance(false); continue; }
                if lookahead > 126 { state = 282; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if lookahead == 34 { state = 160; lexer.advance(false); continue; }
                if lookahead == 38 { state = 16; lexer.advance(false); continue; }
                if lookahead == 47 { state = 163; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 162; lexer.advance(false); continue; }
                if lookahead != 0 { state = 164; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if lookahead == 34 { state = 160; lexer.advance(false); continue; }
                if lookahead == 47 { state = 24; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 13; lexer.advance(true); continue; }
                return result;
            }
            14 => {
                if lookahead == 34 { state = 160; lexer.advance(false); continue; }
                if lookahead == 47 { state = 243; lexer.advance(false); continue; }
                if lookahead == 92 { state = 85; lexer.advance(false); continue; }
                if lookahead == 10 || lookahead == 13 { state = 13; lexer.advance(true); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 246; lexer.advance(false); continue; }
                if lookahead != 0 { state = 248; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if lookahead == 35 { state = 97; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 76; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if lookahead == 35 { state = 97; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 76; lexer.advance(false); continue; }
                if lookahead != 0 { state = 164; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if lookahead == 35 { state = 97; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 76; lexer.advance(false); continue; }
                if lookahead != 0 { state = 170; lexer.advance(false); continue; }
                return result;
            }
            18 => {
                if lookahead == 36 { state = 86; lexer.advance(false); continue; }
                if lookahead == 38 { state = 15; lexer.advance(false); continue; }
                if lookahead == 47 { state = 24; lexer.advance(false); continue; }
                if lookahead == 60 { state = 228; lexer.advance(false); continue; }
                if lookahead == 92 { state = 85; lexer.advance(false); continue; }
                if lookahead == 96 { state = 264; lexer.advance(false); continue; }
                if lookahead == 123 { state = 138; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 19; lexer.advance(true); continue; }
                return result;
            }
            19 => {
                if lookahead == 36 { state = 86; lexer.advance(false); continue; }
                if lookahead == 38 { state = 15; lexer.advance(false); continue; }
                if lookahead == 47 { state = 24; lexer.advance(false); continue; }
                if lookahead == 60 { state = 228; lexer.advance(false); continue; }
                if lookahead == 96 { state = 264; lexer.advance(false); continue; }
                if lookahead == 123 { state = 138; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 19; lexer.advance(true); continue; }
                return result;
            }
            20 => {
                if let Some(next) = advance_map(&[
                    (38, 202), (40, 144), (43, 78), (44, 140), (45, 79), (46, 155), (47, 25), (58, 147),
                    (60, 227), (61, 134), (62, 151), (63, 37), (91, 148), (92, 84), (123, 138), (124, 207),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 20; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 280; lexer.advance(false); continue; }
                if lookahead > 126 { state = 282; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                if lookahead == 38 { state = 17; lexer.advance(false); continue; }
                if lookahead == 39 { state = 161; lexer.advance(false); continue; }
                if lookahead == 47 { state = 169; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 168; lexer.advance(false); continue; }
                if lookahead != 0 { state = 170; lexer.advance(false); continue; }
                return result;
            }
            22 => {
                if lookahead == 39 { state = 161; lexer.advance(false); continue; }
                if lookahead == 47 { state = 24; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 22; lexer.advance(true); continue; }
                return result;
            }
            23 => {
                if lookahead == 39 { state = 161; lexer.advance(false); continue; }
                if lookahead == 47 { state = 249; lexer.advance(false); continue; }
                if lookahead == 92 { state = 85; lexer.advance(false); continue; }
                if lookahead == 10 || lookahead == 13 { state = 22; lexer.advance(true); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 252; lexer.advance(false); continue; }
                if lookahead != 0 { state = 254; lexer.advance(false); continue; }
                return result;
            }
            24 => {
                if lookahead == 42 { state = 27; lexer.advance(false); continue; }
                if lookahead == 47 { state = 263; lexer.advance(false); continue; }
                return result;
            }
            25 => {
                if lookahead == 42 { state = 27; lexer.advance(false); continue; }
                if lookahead == 47 { state = 263; lexer.advance(false); continue; }
                if lookahead == 62 { state = 159; lexer.advance(false); continue; }
                return result;
            }
            26 => {
                if lookahead == 42 { state = 26; lexer.advance(false); continue; }
                if lookahead == 47 { state = 260; lexer.advance(false); continue; }
                if lookahead != 0 { state = 27; lexer.advance(false); continue; }
                return result;
            }
            27 => {
                if lookahead == 42 { state = 26; lexer.advance(false); continue; }
                if lookahead != 0 { state = 27; lexer.advance(false); continue; }
                return result;
            }
            28 => {
                if lookahead == 42 { state = 165; lexer.advance(false); continue; }
                if lookahead == 35 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 27; lexer.advance(false); continue; }
                if lookahead != 0 { state = 166; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                if lookahead == 42 { state = 171; lexer.advance(false); continue; }
                if lookahead == 35 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 27; lexer.advance(false); continue; }
                if lookahead != 0 { state = 172; lexer.advance(false); continue; }
                return result;
            }
            30 => {
                if lookahead == 46 { state = 32; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 277; lexer.advance(false); continue; }
                return result;
            }
            31 => {
                if lookahead == 46 { state = 175; lexer.advance(false); continue; }
                if lookahead == 63 { state = 238; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                if lookahead == 46 { state = 191; lexer.advance(false); continue; }
                return result;
            }
            33 => {
                if lookahead == 47 { state = 266; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 34; lexer.advance(true); continue; }
                return result;
            }
            34 => {
                if lookahead == 47 { state = 24; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 34; lexer.advance(true); continue; }
                return result;
            }
            35 => {
                if lookahead == 58 { state = 290; lexer.advance(false); continue; }
                return result;
            }
            36 => {
                if lookahead == 58 { state = 289; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                if lookahead == 58 { state = 291; lexer.advance(false); continue; }
                return result;
            }
            38 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 38; lexer.advance(false); continue; }
                return result;
            }
            40 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 39; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 40; lexer.advance(false); continue; }
                return result;
            }
            42 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 41; lexer.advance(false); continue; }
                return result;
            }
            43 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 38; lexer.advance(false); continue; }
                return result;
            }
            44 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 43; lexer.advance(false); continue; }
                return result;
            }
            45 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 44; lexer.advance(false); continue; }
                return result;
            }
            46 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 45; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 46; lexer.advance(false); continue; }
                return result;
            }
            48 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 38; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 48; lexer.advance(false); continue; }
                return result;
            }
            50 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 49; lexer.advance(false); continue; }
                return result;
            }
            51 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 50; lexer.advance(false); continue; }
                return result;
            }
            52 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 51; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 52; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 53; lexer.advance(false); continue; }
                return result;
            }
            55 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 54; lexer.advance(false); continue; }
                return result;
            }
            56 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 55; lexer.advance(false); continue; }
                return result;
            }
            57 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 56; lexer.advance(false); continue; }
                return result;
            }
            58 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 57; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 58; lexer.advance(false); continue; }
                return result;
            }
            60 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 59; lexer.advance(false); continue; }
                return result;
            }
            61 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 60; lexer.advance(false); continue; }
                return result;
            }
            62 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 61; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 62; lexer.advance(false); continue; }
                return result;
            }
            64 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            65 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 64; lexer.advance(false); continue; }
                return result;
            }
            66 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 65; lexer.advance(false); continue; }
                return result;
            }
            67 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 66; lexer.advance(false); continue; }
                return result;
            }
            68 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 67; lexer.advance(false); continue; }
                return result;
            }
            69 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 68; lexer.advance(false); continue; }
                return result;
            }
            70 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 69; lexer.advance(false); continue; }
                return result;
            }
            71 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 70; lexer.advance(false); continue; }
                return result;
            }
            72 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 71; lexer.advance(false); continue; }
                return result;
            }
            73 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 72; lexer.advance(false); continue; }
                return result;
            }
            74 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 73; lexer.advance(false); continue; }
                return result;
            }
            75 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 74; lexer.advance(false); continue; }
                return result;
            }
            76 => {
                if lookahead == 59 { state = 150; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 75; lexer.advance(false); continue; }
                return result;
            }
            77 => {
                if lookahead == 61 { state = 233; lexer.advance(false); continue; }
                return result;
            }
            78 => {
                if lookahead == 63 { state = 35; lexer.advance(false); continue; }
                return result;
            }
            79 => {
                if lookahead == 63 { state = 36; lexer.advance(false); continue; }
                return result;
            }
            80 => {
                if lookahead == 92 { state = 83; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 283; lexer.advance(false); continue; }
                return result;
            }
            81 => {
                if lookahead == 92 { state = 124; lexer.advance(false); continue; }
                if lookahead == 93 { state = 268; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 81; lexer.advance(false); continue; }
                return result;
            }
            82 => {
                if lookahead == 117 { state = 87; lexer.advance(false); continue; }
                if lookahead == 120 { state = 114; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 63 { state = 257; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 55 { state = 259; lexer.advance(false); continue; }
                if lookahead != 0 { state = 255; lexer.advance(false); continue; }
                return result;
            }
            83 => {
                if lookahead == 117 { state = 88; lexer.advance(false); continue; }
                return result;
            }
            84 => {
                if lookahead == 117 { state = 89; lexer.advance(false); continue; }
                return result;
            }
            85 => {
                if lookahead == 117 { state = 90; lexer.advance(false); continue; }
                if lookahead == 120 { state = 114; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 63 { state = 257; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 55 { state = 259; lexer.advance(false); continue; }
                if lookahead != 0 { state = 255; lexer.advance(false); continue; }
                return result;
            }
            86 => {
                if lookahead == 123 { state = 265; lexer.advance(false); continue; }
                return result;
            }
            87 => {
                if lookahead == 123 { state = 108; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 119; lexer.advance(false); continue; }
                return result;
            }
            88 => {
                if lookahead == 123 { state = 112; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 120; lexer.advance(false); continue; }
                return result;
            }
            89 => {
                if lookahead == 123 { state = 113; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            90 => {
                if lookahead == 123 { state = 115; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 111; lexer.advance(false); continue; }
                return result;
            }
            91 => {
                if lookahead == 125 { state = 282; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 91; lexer.advance(false); continue; }
                return result;
            }
            92 => {
                if lookahead == 125 { state = 283; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 92; lexer.advance(false); continue; }
                return result;
            }
            93 => {
                if lookahead == 125 { state = 255; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 93; lexer.advance(false); continue; }
                return result;
            }
            94 => {
                if lookahead == 125 { state = 293; lexer.advance(false); continue; }
                return result;
            }
            95 => {
                if lookahead == 125 { state = 256; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 95; lexer.advance(false); continue; }
                return result;
            }
            96 => {
                if lookahead == 43 || lookahead == 45 { state = 103; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 278; lexer.advance(false); continue; }
                return result;
            }
            97 => {
                if lookahead == 88 || lookahead == 120 { state = 110; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 42; lexer.advance(false); continue; }
                return result;
            }
            98 => {
                if lookahead == 48 || lookahead == 49 { state = 274; lexer.advance(false); continue; }
                return result;
            }
            99 => {
                if 48 <= lookahead && lookahead <= 55 { state = 275; lexer.advance(false); continue; }
                return result;
            }
            100 => {
                if 48 <= lookahead && lookahead <= 57 { state = 272; lexer.advance(false); continue; }
                return result;
            }
            101 => {
                if 48 <= lookahead && lookahead <= 57 { state = 277; lexer.advance(false); continue; }
                return result;
            }
            102 => {
                if 48 <= lookahead && lookahead <= 57 { state = 273; lexer.advance(false); continue; }
                return result;
            }
            103 => {
                if 48 <= lookahead && lookahead <= 57 { state = 278; lexer.advance(false); continue; }
                return result;
            }
            104 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 282; lexer.advance(false); continue; }
                return result;
            }
            105 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 283; lexer.advance(false); continue; }
                return result;
            }
            106 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 255; lexer.advance(false); continue; }
                return result;
            }
            107 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 276; lexer.advance(false); continue; }
                return result;
            }
            108 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 95; lexer.advance(false); continue; }
                return result;
            }
            109 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 256; lexer.advance(false); continue; }
                return result;
            }
            110 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 47; lexer.advance(false); continue; }
                return result;
            }
            111 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 114; lexer.advance(false); continue; }
                return result;
            }
            112 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 92; lexer.advance(false); continue; }
                return result;
            }
            113 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 91; lexer.advance(false); continue; }
                return result;
            }
            114 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 106; lexer.advance(false); continue; }
                return result;
            }
            115 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 93; lexer.advance(false); continue; }
                return result;
            }
            116 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 109; lexer.advance(false); continue; }
                return result;
            }
            117 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 105; lexer.advance(false); continue; }
                return result;
            }
            118 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 104; lexer.advance(false); continue; }
                return result;
            }
            119 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 116; lexer.advance(false); continue; }
                return result;
            }
            120 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 117; lexer.advance(false); continue; }
                return result;
            }
            121 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 118; lexer.advance(false); continue; }
                return result;
            }
            122 => {
                if lookahead != 0 && lookahead != 35 && (lookahead < 65 || 90 < lookahead) && (lookahead < 97 || 122 < lookahead) { state = 164; lexer.advance(false); continue; }
                return result;
            }
            123 => {
                if lookahead != 0 && lookahead != 35 && (lookahead < 65 || 90 < lookahead) && (lookahead < 97 || 122 < lookahead) { state = 170; lexer.advance(false); continue; }
                return result;
            }
            124 => {
                if lookahead != 0 && lookahead != 10 { state = 81; lexer.advance(false); continue; }
                return result;
            }
            125 => {
                if lookahead != 0 && lookahead != 10 { state = 268; lexer.advance(false); continue; }
                return result;
            }
            126 => {
                if eof { state = 129; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 143), (34, 160), (35, 6), (36, 281), (37, 224), (38, 203), (39, 161), (40, 144),
                    (41, 146), (42, 132), (43, 216), (44, 140), (45, 220), (46, 156), (47, 222), (48, 271),
                    (58, 147), (59, 145), (60, 229), (61, 136), (62, 152), (63, 287), (64, 284), (91, 148),
                    (92, 84), (93, 149), (94, 206), (96, 264), (123, 139), (124, 209), (125, 141), (126, 240),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 272; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 126; lexer.advance(true); continue; }
                if lookahead > 64 { state = 282; lexer.advance(false); continue; }
                return result;
            }
            127 => {
                if eof { state = 129; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 143), (34, 160), (35, 80), (37, 223), (38, 204), (39, 161), (40, 144), (41, 146),
                    (42, 133), (43, 214), (44, 140), (45, 218), (46, 157), (47, 221), (48, 271), (58, 147),
                    (59, 145), (60, 231), (61, 135), (62, 153), (63, 288), (64, 284), (91, 148), (92, 84),
                    (93, 149), (94, 205), (96, 264), (123, 138), (124, 211), (125, 141), (126, 240),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 272; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 127; lexer.advance(true); continue; }
                if lookahead > 35 { state = 282; lexer.advance(false); continue; }
                return result;
            }
            128 => {
                if eof { state = 129; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 142), (34, 160), (35, 6), (38, 202), (39, 161), (40, 144), (41, 146), (42, 131),
                    (43, 214), (44, 140), (45, 218), (46, 156), (47, 221), (48, 271), (58, 147), (59, 145),
                    (60, 227), (61, 137), (62, 151), (63, 285), (64, 284), (91, 148), (92, 84), (93, 149),
                    (96, 264), (123, 138), (124, 212), (125, 141), (126, 240),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 272; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 128; lexer.advance(true); continue; }
                if lookahead > 35 && (lookahead < 37 || 64 < lookahead) && (lookahead < 91 || 94 < lookahead) { state = 282; lexer.advance(false); continue; }
                return result;
            }
            129 => {
                result = true; lexer.set_result_symbol(ts_builtin_sym_end); lexer.mark_end();
                return result;
            }
            130 => {
                result = true; lexer.set_result_symbol(sym_hash_bang_line); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 { state = 130; lexer.advance(false); continue; }
                return result;
            }
            131 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                return result;
            }
            132 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                if lookahead == 42 { state = 226; lexer.advance(false); continue; }
                if lookahead == 61 { state = 178; lexer.advance(false); continue; }
                return result;
            }
            133 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                if lookahead == 42 { state = 225; lexer.advance(false); continue; }
                return result;
            }
            134 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                return result;
            }
            135 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                if lookahead == 61 { state = 233; lexer.advance(false); continue; }
                return result;
            }
            136 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                if lookahead == 61 { state = 233; lexer.advance(false); continue; }
                if lookahead == 62 { state = 174; lexer.advance(false); continue; }
                return result;
            }
            137 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                if lookahead == 62 { state = 174; lexer.advance(false); continue; }
                return result;
            }
            138 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACE); lexer.mark_end();
                return result;
            }
            139 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACE); lexer.mark_end();
                if lookahead == 124 { state = 292; lexer.advance(false); continue; }
                return result;
            }
            140 => {
                result = true; lexer.set_result_symbol(anon_sym_COMMA); lexer.mark_end();
                return result;
            }
            141 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACE); lexer.mark_end();
                return result;
            }
            142 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG); lexer.mark_end();
                return result;
            }
            143 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG); lexer.mark_end();
                if lookahead == 61 { state = 235; lexer.advance(false); continue; }
                return result;
            }
            144 => {
                result = true; lexer.set_result_symbol(anon_sym_LPAREN); lexer.mark_end();
                return result;
            }
            145 => {
                result = true; lexer.set_result_symbol(anon_sym_SEMI); lexer.mark_end();
                return result;
            }
            146 => {
                result = true; lexer.set_result_symbol(anon_sym_RPAREN); lexer.mark_end();
                return result;
            }
            147 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON); lexer.mark_end();
                return result;
            }
            148 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK); lexer.mark_end();
                return result;
            }
            149 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACK); lexer.mark_end();
                return result;
            }
            150 => {
                result = true; lexer.set_result_symbol(sym_html_character_reference); lexer.mark_end();
                return result;
            }
            151 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                return result;
            }
            152 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                if lookahead == 61 { state = 237; lexer.advance(false); continue; }
                if lookahead == 62 { state = 196; lexer.advance(false); continue; }
                return result;
            }
            153 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                if lookahead == 61 { state = 237; lexer.advance(false); continue; }
                if lookahead == 62 { state = 197; lexer.advance(false); continue; }
                return result;
            }
            154 => {
                result = true; lexer.set_result_symbol(sym_jsx_identifier); lexer.mark_end();
                if lookahead == 36 || lookahead == 45 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 154; lexer.advance(false); continue; }
                return result;
            }
            155 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                return result;
            }
            156 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                if lookahead == 46 { state = 32; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 277; lexer.advance(false); continue; }
                return result;
            }
            157 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 { state = 277; lexer.advance(false); continue; }
                return result;
            }
            158 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_SLASH); lexer.mark_end();
                return result;
            }
            159 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_GT); lexer.mark_end();
                return result;
            }
            160 => {
                result = true; lexer.set_result_symbol(anon_sym_DQUOTE); lexer.mark_end();
                return result;
            }
            161 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTE); lexer.mark_end();
                return result;
            }
            162 => {
                result = true; lexer.set_result_symbol(sym_unescaped_double_jsx_string_fragment); lexer.mark_end();
                if lookahead == 38 { state = 16; lexer.advance(false); continue; }
                if lookahead == 47 { state = 163; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 162; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 34 { state = 164; lexer.advance(false); continue; }
                return result;
            }
            163 => {
                result = true; lexer.set_result_symbol(sym_unescaped_double_jsx_string_fragment); lexer.mark_end();
                if lookahead == 38 { state = 122; lexer.advance(false); continue; }
                if lookahead == 42 { state = 166; lexer.advance(false); continue; }
                if lookahead == 47 { state = 167; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 34 { state = 164; lexer.advance(false); continue; }
                return result;
            }
            164 => {
                result = true; lexer.set_result_symbol(sym_unescaped_double_jsx_string_fragment); lexer.mark_end();
                if lookahead == 38 { state = 122; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 34 { state = 164; lexer.advance(false); continue; }
                return result;
            }
            165 => {
                result = true; lexer.set_result_symbol(sym_unescaped_double_jsx_string_fragment); lexer.mark_end();
                if lookahead == 38 { state = 28; lexer.advance(false); continue; }
                if lookahead == 42 { state = 165; lexer.advance(false); continue; }
                if lookahead == 47 { state = 164; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 34 { state = 166; lexer.advance(false); continue; }
                return result;
            }
            166 => {
                result = true; lexer.set_result_symbol(sym_unescaped_double_jsx_string_fragment); lexer.mark_end();
                if lookahead == 38 { state = 28; lexer.advance(false); continue; }
                if lookahead == 42 { state = 165; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 34 { state = 166; lexer.advance(false); continue; }
                return result;
            }
            167 => {
                result = true; lexer.set_result_symbol(sym_unescaped_double_jsx_string_fragment); lexer.mark_end();
                if lookahead == 38 { state = 261; lexer.advance(false); continue; }
                if lookahead == 10 || lookahead == 13 || lookahead == 8232 || lookahead == 8233 { state = 164; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 34 { state = 167; lexer.advance(false); continue; }
                return result;
            }
            168 => {
                result = true; lexer.set_result_symbol(sym_unescaped_single_jsx_string_fragment); lexer.mark_end();
                if lookahead == 38 { state = 17; lexer.advance(false); continue; }
                if lookahead == 47 { state = 169; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 168; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 38 && lookahead != 39 { state = 170; lexer.advance(false); continue; }
                return result;
            }
            169 => {
                result = true; lexer.set_result_symbol(sym_unescaped_single_jsx_string_fragment); lexer.mark_end();
                if lookahead == 38 { state = 123; lexer.advance(false); continue; }
                if lookahead == 42 { state = 172; lexer.advance(false); continue; }
                if lookahead == 47 { state = 173; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 38 && lookahead != 39 { state = 170; lexer.advance(false); continue; }
                return result;
            }
            170 => {
                result = true; lexer.set_result_symbol(sym_unescaped_single_jsx_string_fragment); lexer.mark_end();
                if lookahead == 38 { state = 123; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 38 && lookahead != 39 { state = 170; lexer.advance(false); continue; }
                return result;
            }
            171 => {
                result = true; lexer.set_result_symbol(sym_unescaped_single_jsx_string_fragment); lexer.mark_end();
                if lookahead == 38 { state = 29; lexer.advance(false); continue; }
                if lookahead == 42 { state = 171; lexer.advance(false); continue; }
                if lookahead == 47 { state = 170; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 38 && lookahead != 39 { state = 172; lexer.advance(false); continue; }
                return result;
            }
            172 => {
                result = true; lexer.set_result_symbol(sym_unescaped_single_jsx_string_fragment); lexer.mark_end();
                if lookahead == 38 { state = 29; lexer.advance(false); continue; }
                if lookahead == 42 { state = 171; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 38 && lookahead != 39 { state = 172; lexer.advance(false); continue; }
                return result;
            }
            173 => {
                result = true; lexer.set_result_symbol(sym_unescaped_single_jsx_string_fragment); lexer.mark_end();
                if lookahead == 38 { state = 262; lexer.advance(false); continue; }
                if lookahead == 10 || lookahead == 13 || lookahead == 8232 || lookahead == 8233 { state = 170; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 38 && lookahead != 39 { state = 173; lexer.advance(false); continue; }
                return result;
            }
            174 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ_GT); lexer.mark_end();
                return result;
            }
            175 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK_DOT); lexer.mark_end();
                return result;
            }
            176 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS_EQ); lexer.mark_end();
                return result;
            }
            177 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_EQ); lexer.mark_end();
                return result;
            }
            178 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR_EQ); lexer.mark_end();
                return result;
            }
            179 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_EQ); lexer.mark_end();
                return result;
            }
            180 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT_EQ); lexer.mark_end();
                return result;
            }
            181 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET_EQ); lexer.mark_end();
                return result;
            }
            182 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP_EQ); lexer.mark_end();
                return result;
            }
            183 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE_EQ); lexer.mark_end();
                return result;
            }
            184 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT_EQ); lexer.mark_end();
                return result;
            }
            185 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT_GT_EQ); lexer.mark_end();
                return result;
            }
            186 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT_EQ); lexer.mark_end();
                return result;
            }
            187 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR_STAR_EQ); lexer.mark_end();
                return result;
            }
            188 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP_AMP_EQ); lexer.mark_end();
                return result;
            }
            189 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE_PIPE_EQ); lexer.mark_end();
                return result;
            }
            190 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK_QMARK_EQ); lexer.mark_end();
                return result;
            }
            191 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT_DOT_DOT); lexer.mark_end();
                return result;
            }
            192 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP_AMP); lexer.mark_end();
                return result;
            }
            193 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP_AMP); lexer.mark_end();
                if lookahead == 61 { state = 188; lexer.advance(false); continue; }
                return result;
            }
            194 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE_PIPE); lexer.mark_end();
                return result;
            }
            195 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE_PIPE); lexer.mark_end();
                if lookahead == 61 { state = 189; lexer.advance(false); continue; }
                return result;
            }
            196 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT); lexer.mark_end();
                if lookahead == 61 { state = 184; lexer.advance(false); continue; }
                if lookahead == 62 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            197 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT); lexer.mark_end();
                if lookahead == 62 { state = 198; lexer.advance(false); continue; }
                return result;
            }
            198 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT_GT); lexer.mark_end();
                return result;
            }
            199 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT_GT); lexer.mark_end();
                if lookahead == 61 { state = 185; lexer.advance(false); continue; }
                return result;
            }
            200 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT); lexer.mark_end();
                return result;
            }
            201 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT); lexer.mark_end();
                if lookahead == 61 { state = 186; lexer.advance(false); continue; }
                return result;
            }
            202 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP); lexer.mark_end();
                return result;
            }
            203 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP); lexer.mark_end();
                if lookahead == 38 { state = 193; lexer.advance(false); continue; }
                if lookahead == 61 { state = 182; lexer.advance(false); continue; }
                return result;
            }
            204 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP); lexer.mark_end();
                if lookahead == 38 { state = 192; lexer.advance(false); continue; }
                return result;
            }
            205 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET); lexer.mark_end();
                return result;
            }
            206 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET); lexer.mark_end();
                if lookahead == 61 { state = 181; lexer.advance(false); continue; }
                return result;
            }
            207 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                return result;
            }
            208 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                if lookahead == 61 { state = 183; lexer.advance(false); continue; }
                if lookahead == 124 { state = 195; lexer.advance(false); continue; }
                return result;
            }
            209 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                if lookahead == 61 { state = 183; lexer.advance(false); continue; }
                if lookahead == 124 { state = 195; lexer.advance(false); continue; }
                if lookahead == 125 { state = 293; lexer.advance(false); continue; }
                return result;
            }
            210 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                if lookahead == 124 { state = 194; lexer.advance(false); continue; }
                return result;
            }
            211 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                if lookahead == 124 { state = 194; lexer.advance(false); continue; }
                if lookahead == 125 { state = 293; lexer.advance(false); continue; }
                return result;
            }
            212 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                if lookahead == 125 { state = 293; lexer.advance(false); continue; }
                return result;
            }
            213 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                return result;
            }
            214 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                if lookahead == 43 { state = 241; lexer.advance(false); continue; }
                return result;
            }
            215 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                if lookahead == 43 { state = 241; lexer.advance(false); continue; }
                if lookahead == 61 { state = 176; lexer.advance(false); continue; }
                return result;
            }
            216 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                if lookahead == 43 { state = 241; lexer.advance(false); continue; }
                if lookahead == 61 { state = 176; lexer.advance(false); continue; }
                if lookahead == 63 { state = 35; lexer.advance(false); continue; }
                return result;
            }
            217 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                return result;
            }
            218 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 242; lexer.advance(false); continue; }
                return result;
            }
            219 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 242; lexer.advance(false); continue; }
                if lookahead == 61 { state = 177; lexer.advance(false); continue; }
                return result;
            }
            220 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 242; lexer.advance(false); continue; }
                if lookahead == 61 { state = 177; lexer.advance(false); continue; }
                if lookahead == 63 { state = 36; lexer.advance(false); continue; }
                return result;
            }
            221 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH); lexer.mark_end();
                if lookahead == 42 { state = 27; lexer.advance(false); continue; }
                if lookahead == 47 { state = 263; lexer.advance(false); continue; }
                return result;
            }
            222 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH); lexer.mark_end();
                if lookahead == 42 { state = 27; lexer.advance(false); continue; }
                if lookahead == 47 { state = 263; lexer.advance(false); continue; }
                if lookahead == 61 { state = 179; lexer.advance(false); continue; }
                return result;
            }
            223 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT); lexer.mark_end();
                return result;
            }
            224 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT); lexer.mark_end();
                if lookahead == 61 { state = 180; lexer.advance(false); continue; }
                return result;
            }
            225 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR_STAR); lexer.mark_end();
                return result;
            }
            226 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR_STAR); lexer.mark_end();
                if lookahead == 61 { state = 187; lexer.advance(false); continue; }
                return result;
            }
            227 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                return result;
            }
            228 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 47 { state = 158; lexer.advance(false); continue; }
                return result;
            }
            229 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 47 { state = 158; lexer.advance(false); continue; }
                if lookahead == 60 { state = 201; lexer.advance(false); continue; }
                if lookahead == 61 { state = 232; lexer.advance(false); continue; }
                return result;
            }
            230 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 60 { state = 201; lexer.advance(false); continue; }
                if lookahead == 61 { state = 232; lexer.advance(false); continue; }
                return result;
            }
            231 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 60 { state = 200; lexer.advance(false); continue; }
                if lookahead == 61 { state = 232; lexer.advance(false); continue; }
                return result;
            }
            232 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_EQ); lexer.mark_end();
                return result;
            }
            233 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ_EQ); lexer.mark_end();
                if lookahead == 61 { state = 234; lexer.advance(false); continue; }
                return result;
            }
            234 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ_EQ_EQ); lexer.mark_end();
                return result;
            }
            235 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG_EQ); lexer.mark_end();
                if lookahead == 61 { state = 236; lexer.advance(false); continue; }
                return result;
            }
            236 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG_EQ_EQ); lexer.mark_end();
                return result;
            }
            237 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_EQ); lexer.mark_end();
                return result;
            }
            238 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK_QMARK); lexer.mark_end();
                return result;
            }
            239 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK_QMARK); lexer.mark_end();
                if lookahead == 61 { state = 190; lexer.advance(false); continue; }
                return result;
            }
            240 => {
                result = true; lexer.set_result_symbol(anon_sym_TILDE); lexer.mark_end();
                return result;
            }
            241 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS_PLUS); lexer.mark_end();
                return result;
            }
            242 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_DASH); lexer.mark_end();
                return result;
            }
            243 => {
                result = true; lexer.set_result_symbol(sym_unescaped_double_string_fragment); lexer.mark_end();
                if lookahead == 42 { state = 245; lexer.advance(false); continue; }
                if lookahead == 47 { state = 247; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 34 && lookahead != 92 { state = 248; lexer.advance(false); continue; }
                return result;
            }
            244 => {
                result = true; lexer.set_result_symbol(sym_unescaped_double_string_fragment); lexer.mark_end();
                if lookahead == 42 { state = 244; lexer.advance(false); continue; }
                if lookahead == 47 { state = 248; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 34 && lookahead != 92 { state = 245; lexer.advance(false); continue; }
                return result;
            }
            245 => {
                result = true; lexer.set_result_symbol(sym_unescaped_double_string_fragment); lexer.mark_end();
                if lookahead == 42 { state = 244; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 34 && lookahead != 92 { state = 245; lexer.advance(false); continue; }
                return result;
            }
            246 => {
                result = true; lexer.set_result_symbol(sym_unescaped_double_string_fragment); lexer.mark_end();
                if lookahead == 47 { state = 243; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) && lookahead != 10 && lookahead != 13 { state = 246; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 34 && lookahead != 92 { state = 248; lexer.advance(false); continue; }
                return result;
            }
            247 => {
                result = true; lexer.set_result_symbol(sym_unescaped_double_string_fragment); lexer.mark_end();
                if lookahead == 8232 || lookahead == 8233 { state = 248; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 34 && lookahead != 92 { state = 247; lexer.advance(false); continue; }
                return result;
            }
            248 => {
                result = true; lexer.set_result_symbol(sym_unescaped_double_string_fragment); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 34 && lookahead != 92 { state = 248; lexer.advance(false); continue; }
                return result;
            }
            249 => {
                result = true; lexer.set_result_symbol(sym_unescaped_single_string_fragment); lexer.mark_end();
                if lookahead == 42 { state = 251; lexer.advance(false); continue; }
                if lookahead == 47 { state = 253; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 39 && lookahead != 92 { state = 254; lexer.advance(false); continue; }
                return result;
            }
            250 => {
                result = true; lexer.set_result_symbol(sym_unescaped_single_string_fragment); lexer.mark_end();
                if lookahead == 42 { state = 250; lexer.advance(false); continue; }
                if lookahead == 47 { state = 254; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 39 && lookahead != 92 { state = 251; lexer.advance(false); continue; }
                return result;
            }
            251 => {
                result = true; lexer.set_result_symbol(sym_unescaped_single_string_fragment); lexer.mark_end();
                if lookahead == 42 { state = 250; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 39 && lookahead != 92 { state = 251; lexer.advance(false); continue; }
                return result;
            }
            252 => {
                result = true; lexer.set_result_symbol(sym_unescaped_single_string_fragment); lexer.mark_end();
                if lookahead == 47 { state = 249; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) && lookahead != 10 && lookahead != 13 { state = 252; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 39 && lookahead != 92 { state = 254; lexer.advance(false); continue; }
                return result;
            }
            253 => {
                result = true; lexer.set_result_symbol(sym_unescaped_single_string_fragment); lexer.mark_end();
                if lookahead == 8232 || lookahead == 8233 { state = 254; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 39 && lookahead != 92 { state = 253; lexer.advance(false); continue; }
                return result;
            }
            254 => {
                result = true; lexer.set_result_symbol(sym_unescaped_single_string_fragment); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 39 && lookahead != 92 { state = 254; lexer.advance(false); continue; }
                return result;
            }
            255 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                return result;
            }
            256 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                if lookahead == 92 { state = 84; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 282; lexer.advance(false); continue; }
                return result;
            }
            257 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                if lookahead == 10 || lookahead == 8232 || lookahead == 8233 { state = 255; lexer.advance(false); continue; }
                return result;
            }
            258 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 55 { state = 255; lexer.advance(false); continue; }
                return result;
            }
            259 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 55 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            260 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                return result;
            }
            261 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 10 || lookahead == 13 || lookahead == 8232 || lookahead == 8233 { state = 164; lexer.advance(false); continue; }
                if lookahead == 35 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 263; lexer.advance(false); continue; }
                if lookahead != 0 { state = 167; lexer.advance(false); continue; }
                return result;
            }
            262 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 10 || lookahead == 13 || lookahead == 8232 || lookahead == 8233 { state = 170; lexer.advance(false); continue; }
                if lookahead == 35 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 263; lexer.advance(false); continue; }
                if lookahead != 0 { state = 173; lexer.advance(false); continue; }
                return result;
            }
            263 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 8232 && lookahead != 8233 { state = 263; lexer.advance(false); continue; }
                return result;
            }
            264 => {
                result = true; lexer.set_result_symbol(anon_sym_BQUOTE); lexer.mark_end();
                return result;
            }
            265 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR_LBRACE); lexer.mark_end();
                return result;
            }
            266 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH2); lexer.mark_end();
                return result;
            }
            267 => {
                result = true; lexer.set_result_symbol(sym_regex_pattern); lexer.mark_end();
                if lookahead == 10 { state = 34; lexer.advance(true); continue; }
                if lookahead == 47 { state = 24; lexer.advance(false); continue; }
                if lookahead == 91 { state = 81; lexer.advance(false); continue; }
                if lookahead == 92 { state = 125; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 267; lexer.advance(false); continue; }
                if lookahead != 0 { state = 268; lexer.advance(false); continue; }
                return result;
            }
            268 => {
                result = true; lexer.set_result_symbol(sym_regex_pattern); lexer.mark_end();
                if lookahead == 91 { state = 81; lexer.advance(false); continue; }
                if lookahead == 92 { state = 125; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 47 { state = 268; lexer.advance(false); continue; }
                return result;
            }
            269 => {
                result = true; lexer.set_result_symbol(sym_regex_flags); lexer.mark_end();
                if lookahead == 92 { state = 84; lexer.advance(false); continue; }
                if 97 <= lookahead && lookahead <= 122 { state = 269; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 282; lexer.advance(false); continue; }
                return result;
            }
            270 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                return result;
            }
            271 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (46, 279), (48, 273), (95, 102), (110, 270), (66, 98), (98, 98), (69, 96), (101, 96),
                    (79, 99), (111, 99), (88, 107), (120, 107),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 272; lexer.advance(false); continue; }
                return result;
            }
            272 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if lookahead == 46 { state = 279; lexer.advance(false); continue; }
                if lookahead == 95 { state = 100; lexer.advance(false); continue; }
                if lookahead == 110 { state = 270; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 96; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 272; lexer.advance(false); continue; }
                return result;
            }
            273 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if lookahead == 95 { state = 102; lexer.advance(false); continue; }
                if lookahead == 110 { state = 270; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 273; lexer.advance(false); continue; }
                return result;
            }
            274 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if lookahead == 95 { state = 98; lexer.advance(false); continue; }
                if lookahead == 110 { state = 270; lexer.advance(false); continue; }
                if lookahead == 48 || lookahead == 49 { state = 274; lexer.advance(false); continue; }
                return result;
            }
            275 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if lookahead == 95 { state = 99; lexer.advance(false); continue; }
                if lookahead == 110 { state = 270; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 55 { state = 275; lexer.advance(false); continue; }
                return result;
            }
            276 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if lookahead == 95 { state = 107; lexer.advance(false); continue; }
                if lookahead == 110 { state = 270; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 276; lexer.advance(false); continue; }
                return result;
            }
            277 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if lookahead == 95 { state = 101; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 96; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 277; lexer.advance(false); continue; }
                return result;
            }
            278 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if lookahead == 95 { state = 103; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 278; lexer.advance(false); continue; }
                return result;
            }
            279 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if lookahead == 69 || lookahead == 101 { state = 96; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 277; lexer.advance(false); continue; }
                return result;
            }
            280 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 45 { state = 154; lexer.advance(false); continue; }
                if lookahead == 92 { state = 84; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 280; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 282; lexer.advance(false); continue; }
                return result;
            }
            281 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 84; lexer.advance(false); continue; }
                if lookahead == 123 { state = 265; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 282; lexer.advance(false); continue; }
                return result;
            }
            282 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 84; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 282; lexer.advance(false); continue; }
                return result;
            }
            283 => {
                result = true; lexer.set_result_symbol(sym_private_property_identifier); lexer.mark_end();
                if lookahead == 92 { state = 83; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 283; lexer.advance(false); continue; }
                return result;
            }
            284 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                return result;
            }
            285 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK); lexer.mark_end();
                return result;
            }
            286 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK); lexer.mark_end();
                if lookahead == 46 { state = 175; lexer.advance(false); continue; }
                return result;
            }
            287 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK); lexer.mark_end();
                if lookahead == 46 { state = 175; lexer.advance(false); continue; }
                if lookahead == 63 { state = 239; lexer.advance(false); continue; }
                return result;
            }
            288 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK); lexer.mark_end();
                if lookahead == 46 { state = 175; lexer.advance(false); continue; }
                if lookahead == 63 { state = 238; lexer.advance(false); continue; }
                return result;
            }
            289 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_QMARK_COLON); lexer.mark_end();
                return result;
            }
            290 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS_QMARK_COLON); lexer.mark_end();
                return result;
            }
            291 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK_COLON); lexer.mark_end();
                return result;
            }
            292 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACE_PIPE); lexer.mark_end();
                return result;
            }
            293 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE_RBRACE); lexer.mark_end();
                return result;
            }
            _ => return false,
        }
    }
}

/// C's `ts_lex_keywords`: lexes one token from lex state `state`; returns whether it
/// accepted one.
#[rustfmt::skip]
pub(crate) fn ts_lex_keywords(lexer: &mut dyn Lexer, mut state: StateId) -> bool {
    let mut result = false;
    loop {
        // C's `start:` label (reached again after each `next_state:` advance).
        let lookahead = lexer.lookahead();
        let _eof = lexer.eof();
        match state {
            0 => {
                if let Some(next) = advance_map(&[
                    (97, 1), (98, 2), (99, 3), (100, 4), (101, 5), (102, 6), (103, 7), (105, 8),
                    (107, 9), (108, 10), (109, 11), (110, 12), (111, 13), (112, 14), (114, 15), (115, 16),
                    (116, 17), (117, 18), (118, 19), (119, 20), (121, 21),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 0; lexer.advance(true); continue; }
                return result;
            }
            1 => {
                if lookahead == 98 { state = 22; lexer.advance(false); continue; }
                if lookahead == 99 { state = 23; lexer.advance(false); continue; }
                if lookahead == 110 { state = 24; lexer.advance(false); continue; }
                if lookahead == 115 { state = 25; lexer.advance(false); continue; }
                if lookahead == 119 { state = 26; lexer.advance(false); continue; }
                return result;
            }
            2 => {
                if lookahead == 111 { state = 27; lexer.advance(false); continue; }
                if lookahead == 114 { state = 28; lexer.advance(false); continue; }
                return result;
            }
            3 => {
                if lookahead == 97 { state = 29; lexer.advance(false); continue; }
                if lookahead == 108 { state = 30; lexer.advance(false); continue; }
                if lookahead == 111 { state = 31; lexer.advance(false); continue; }
                return result;
            }
            4 => {
                if lookahead == 101 { state = 32; lexer.advance(false); continue; }
                if lookahead == 111 { state = 33; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                if lookahead == 108 { state = 34; lexer.advance(false); continue; }
                if lookahead == 110 { state = 35; lexer.advance(false); continue; }
                if lookahead == 120 { state = 36; lexer.advance(false); continue; }
                return result;
            }
            6 => {
                if lookahead == 97 { state = 37; lexer.advance(false); continue; }
                if lookahead == 105 { state = 38; lexer.advance(false); continue; }
                if lookahead == 111 { state = 39; lexer.advance(false); continue; }
                if lookahead == 114 { state = 40; lexer.advance(false); continue; }
                if lookahead == 117 { state = 41; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if lookahead == 101 { state = 42; lexer.advance(false); continue; }
                if lookahead == 108 { state = 43; lexer.advance(false); continue; }
                return result;
            }
            8 => {
                if lookahead == 102 { state = 44; lexer.advance(false); continue; }
                if lookahead == 109 { state = 45; lexer.advance(false); continue; }
                if lookahead == 110 { state = 46; lexer.advance(false); continue; }
                if lookahead == 115 { state = 47; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if lookahead == 101 { state = 48; lexer.advance(false); continue; }
                return result;
            }
            10 => {
                if lookahead == 101 { state = 49; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if lookahead == 101 { state = 50; lexer.advance(false); continue; }
                if lookahead == 111 { state = 51; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if lookahead == 97 { state = 52; lexer.advance(false); continue; }
                if lookahead == 101 { state = 53; lexer.advance(false); continue; }
                if lookahead == 117 { state = 54; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if lookahead == 98 { state = 55; lexer.advance(false); continue; }
                if lookahead == 102 { state = 56; lexer.advance(false); continue; }
                if lookahead == 118 { state = 57; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if lookahead == 114 { state = 58; lexer.advance(false); continue; }
                if lookahead == 117 { state = 59; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if lookahead == 101 { state = 60; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if lookahead == 97 { state = 61; lexer.advance(false); continue; }
                if lookahead == 101 { state = 62; lexer.advance(false); continue; }
                if lookahead == 116 { state = 63; lexer.advance(false); continue; }
                if lookahead == 117 { state = 64; lexer.advance(false); continue; }
                if lookahead == 119 { state = 65; lexer.advance(false); continue; }
                if lookahead == 121 { state = 66; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if lookahead == 97 { state = 67; lexer.advance(false); continue; }
                if lookahead == 104 { state = 68; lexer.advance(false); continue; }
                if lookahead == 114 { state = 69; lexer.advance(false); continue; }
                if lookahead == 121 { state = 70; lexer.advance(false); continue; }
                return result;
            }
            18 => {
                if lookahead == 110 { state = 71; lexer.advance(false); continue; }
                if lookahead == 115 { state = 72; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if lookahead == 97 { state = 73; lexer.advance(false); continue; }
                if lookahead == 111 { state = 74; lexer.advance(false); continue; }
                return result;
            }
            20 => {
                if lookahead == 104 { state = 75; lexer.advance(false); continue; }
                if lookahead == 105 { state = 76; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                if lookahead == 105 { state = 77; lexer.advance(false); continue; }
                return result;
            }
            22 => {
                if lookahead == 115 { state = 78; lexer.advance(false); continue; }
                return result;
            }
            23 => {
                if lookahead == 99 { state = 79; lexer.advance(false); continue; }
                return result;
            }
            24 => {
                if lookahead == 121 { state = 80; lexer.advance(false); continue; }
                return result;
            }
            25 => {
                result = true; lexer.set_result_symbol(anon_sym_as); lexer.mark_end();
                if lookahead == 115 { state = 81; lexer.advance(false); continue; }
                if lookahead == 121 { state = 82; lexer.advance(false); continue; }
                return result;
            }
            26 => {
                if lookahead == 97 { state = 83; lexer.advance(false); continue; }
                return result;
            }
            27 => {
                if lookahead == 111 { state = 84; lexer.advance(false); continue; }
                return result;
            }
            28 => {
                if lookahead == 101 { state = 85; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                if lookahead == 115 { state = 86; lexer.advance(false); continue; }
                if lookahead == 116 { state = 87; lexer.advance(false); continue; }
                return result;
            }
            30 => {
                if lookahead == 97 { state = 88; lexer.advance(false); continue; }
                return result;
            }
            31 => {
                if lookahead == 110 { state = 89; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                if lookahead == 98 { state = 90; lexer.advance(false); continue; }
                if lookahead == 99 { state = 91; lexer.advance(false); continue; }
                if lookahead == 102 { state = 92; lexer.advance(false); continue; }
                if lookahead == 108 { state = 93; lexer.advance(false); continue; }
                return result;
            }
            33 => {
                result = true; lexer.set_result_symbol(anon_sym_do); lexer.mark_end();
                return result;
            }
            34 => {
                if lookahead == 115 { state = 94; lexer.advance(false); continue; }
                return result;
            }
            35 => {
                if lookahead == 117 { state = 95; lexer.advance(false); continue; }
                return result;
            }
            36 => {
                if lookahead == 112 { state = 96; lexer.advance(false); continue; }
                if lookahead == 116 { state = 97; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                if lookahead == 108 { state = 98; lexer.advance(false); continue; }
                return result;
            }
            38 => {
                if lookahead == 110 { state = 99; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                if lookahead == 114 { state = 100; lexer.advance(false); continue; }
                return result;
            }
            40 => {
                if lookahead == 111 { state = 101; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                if lookahead == 110 { state = 102; lexer.advance(false); continue; }
                return result;
            }
            42 => {
                if lookahead == 116 { state = 103; lexer.advance(false); continue; }
                return result;
            }
            43 => {
                if lookahead == 111 { state = 104; lexer.advance(false); continue; }
                return result;
            }
            44 => {
                result = true; lexer.set_result_symbol(anon_sym_if); lexer.mark_end();
                return result;
            }
            45 => {
                if lookahead == 112 { state = 105; lexer.advance(false); continue; }
                return result;
            }
            46 => {
                result = true; lexer.set_result_symbol(anon_sym_in); lexer.mark_end();
                if lookahead == 102 { state = 106; lexer.advance(false); continue; }
                if lookahead == 115 { state = 107; lexer.advance(false); continue; }
                if lookahead == 116 { state = 108; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                result = true; lexer.set_result_symbol(anon_sym_is); lexer.mark_end();
                return result;
            }
            48 => {
                if lookahead == 121 { state = 109; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                if lookahead == 116 { state = 110; lexer.advance(false); continue; }
                return result;
            }
            50 => {
                if lookahead == 116 { state = 111; lexer.advance(false); continue; }
                return result;
            }
            51 => {
                if lookahead == 100 { state = 112; lexer.advance(false); continue; }
                return result;
            }
            52 => {
                if lookahead == 109 { state = 113; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                if lookahead == 118 { state = 114; lexer.advance(false); continue; }
                if lookahead == 119 { state = 115; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                if lookahead == 108 { state = 116; lexer.advance(false); continue; }
                if lookahead == 109 { state = 117; lexer.advance(false); continue; }
                return result;
            }
            55 => {
                if lookahead == 106 { state = 118; lexer.advance(false); continue; }
                return result;
            }
            56 => {
                result = true; lexer.set_result_symbol(anon_sym_of); lexer.mark_end();
                return result;
            }
            57 => {
                if lookahead == 101 { state = 119; lexer.advance(false); continue; }
                return result;
            }
            58 => {
                if lookahead == 105 { state = 120; lexer.advance(false); continue; }
                if lookahead == 111 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                if lookahead == 98 { state = 122; lexer.advance(false); continue; }
                return result;
            }
            60 => {
                if lookahead == 97 { state = 123; lexer.advance(false); continue; }
                if lookahead == 113 { state = 124; lexer.advance(false); continue; }
                if lookahead == 116 { state = 125; lexer.advance(false); continue; }
                return result;
            }
            61 => {
                if lookahead == 116 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            62 => {
                if lookahead == 116 { state = 127; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                if lookahead == 97 { state = 128; lexer.advance(false); continue; }
                if lookahead == 114 { state = 129; lexer.advance(false); continue; }
                return result;
            }
            64 => {
                if lookahead == 112 { state = 130; lexer.advance(false); continue; }
                return result;
            }
            65 => {
                if lookahead == 105 { state = 131; lexer.advance(false); continue; }
                return result;
            }
            66 => {
                if lookahead == 109 { state = 132; lexer.advance(false); continue; }
                return result;
            }
            67 => {
                if lookahead == 114 { state = 133; lexer.advance(false); continue; }
                return result;
            }
            68 => {
                if lookahead == 105 { state = 134; lexer.advance(false); continue; }
                if lookahead == 114 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            69 => {
                if lookahead == 117 { state = 136; lexer.advance(false); continue; }
                if lookahead == 121 { state = 137; lexer.advance(false); continue; }
                return result;
            }
            70 => {
                if lookahead == 112 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            71 => {
                if lookahead == 100 { state = 139; lexer.advance(false); continue; }
                if lookahead == 105 { state = 140; lexer.advance(false); continue; }
                if lookahead == 107 { state = 141; lexer.advance(false); continue; }
                return result;
            }
            72 => {
                if lookahead == 105 { state = 142; lexer.advance(false); continue; }
                return result;
            }
            73 => {
                if lookahead == 114 { state = 143; lexer.advance(false); continue; }
                return result;
            }
            74 => {
                if lookahead == 105 { state = 144; lexer.advance(false); continue; }
                return result;
            }
            75 => {
                if lookahead == 105 { state = 145; lexer.advance(false); continue; }
                return result;
            }
            76 => {
                if lookahead == 116 { state = 146; lexer.advance(false); continue; }
                return result;
            }
            77 => {
                if lookahead == 101 { state = 147; lexer.advance(false); continue; }
                return result;
            }
            78 => {
                if lookahead == 116 { state = 148; lexer.advance(false); continue; }
                return result;
            }
            79 => {
                if lookahead == 101 { state = 149; lexer.advance(false); continue; }
                return result;
            }
            80 => {
                result = true; lexer.set_result_symbol(anon_sym_any); lexer.mark_end();
                return result;
            }
            81 => {
                if lookahead == 101 { state = 150; lexer.advance(false); continue; }
                return result;
            }
            82 => {
                if lookahead == 110 { state = 151; lexer.advance(false); continue; }
                return result;
            }
            83 => {
                if lookahead == 105 { state = 152; lexer.advance(false); continue; }
                return result;
            }
            84 => {
                if lookahead == 108 { state = 153; lexer.advance(false); continue; }
                return result;
            }
            85 => {
                if lookahead == 97 { state = 154; lexer.advance(false); continue; }
                return result;
            }
            86 => {
                if lookahead == 101 { state = 155; lexer.advance(false); continue; }
                return result;
            }
            87 => {
                if lookahead == 99 { state = 156; lexer.advance(false); continue; }
                return result;
            }
            88 => {
                if lookahead == 115 { state = 157; lexer.advance(false); continue; }
                return result;
            }
            89 => {
                if lookahead == 115 { state = 158; lexer.advance(false); continue; }
                if lookahead == 116 { state = 159; lexer.advance(false); continue; }
                return result;
            }
            90 => {
                if lookahead == 117 { state = 160; lexer.advance(false); continue; }
                return result;
            }
            91 => {
                if lookahead == 108 { state = 161; lexer.advance(false); continue; }
                return result;
            }
            92 => {
                if lookahead == 97 { state = 162; lexer.advance(false); continue; }
                return result;
            }
            93 => {
                if lookahead == 101 { state = 163; lexer.advance(false); continue; }
                return result;
            }
            94 => {
                if lookahead == 101 { state = 164; lexer.advance(false); continue; }
                return result;
            }
            95 => {
                if lookahead == 109 { state = 165; lexer.advance(false); continue; }
                return result;
            }
            96 => {
                if lookahead == 111 { state = 166; lexer.advance(false); continue; }
                return result;
            }
            97 => {
                if lookahead == 101 { state = 167; lexer.advance(false); continue; }
                return result;
            }
            98 => {
                if lookahead == 115 { state = 168; lexer.advance(false); continue; }
                return result;
            }
            99 => {
                if lookahead == 97 { state = 169; lexer.advance(false); continue; }
                return result;
            }
            100 => {
                result = true; lexer.set_result_symbol(anon_sym_for); lexer.mark_end();
                return result;
            }
            101 => {
                if lookahead == 109 { state = 170; lexer.advance(false); continue; }
                return result;
            }
            102 => {
                if lookahead == 99 { state = 171; lexer.advance(false); continue; }
                return result;
            }
            103 => {
                result = true; lexer.set_result_symbol(anon_sym_get); lexer.mark_end();
                return result;
            }
            104 => {
                if lookahead == 98 { state = 172; lexer.advance(false); continue; }
                return result;
            }
            105 => {
                if lookahead == 108 { state = 173; lexer.advance(false); continue; }
                if lookahead == 111 { state = 174; lexer.advance(false); continue; }
                return result;
            }
            106 => {
                if lookahead == 101 { state = 175; lexer.advance(false); continue; }
                return result;
            }
            107 => {
                if lookahead == 116 { state = 176; lexer.advance(false); continue; }
                return result;
            }
            108 => {
                if lookahead == 101 { state = 177; lexer.advance(false); continue; }
                return result;
            }
            109 => {
                if lookahead == 111 { state = 178; lexer.advance(false); continue; }
                return result;
            }
            110 => {
                result = true; lexer.set_result_symbol(anon_sym_let); lexer.mark_end();
                return result;
            }
            111 => {
                if lookahead == 97 { state = 179; lexer.advance(false); continue; }
                return result;
            }
            112 => {
                if lookahead == 117 { state = 180; lexer.advance(false); continue; }
                return result;
            }
            113 => {
                if lookahead == 101 { state = 181; lexer.advance(false); continue; }
                return result;
            }
            114 => {
                if lookahead == 101 { state = 182; lexer.advance(false); continue; }
                return result;
            }
            115 => {
                result = true; lexer.set_result_symbol(anon_sym_new); lexer.mark_end();
                return result;
            }
            116 => {
                if lookahead == 108 { state = 183; lexer.advance(false); continue; }
                return result;
            }
            117 => {
                if lookahead == 98 { state = 184; lexer.advance(false); continue; }
                return result;
            }
            118 => {
                if lookahead == 101 { state = 185; lexer.advance(false); continue; }
                return result;
            }
            119 => {
                if lookahead == 114 { state = 186; lexer.advance(false); continue; }
                return result;
            }
            120 => {
                if lookahead == 118 { state = 187; lexer.advance(false); continue; }
                return result;
            }
            121 => {
                if lookahead == 116 { state = 188; lexer.advance(false); continue; }
                return result;
            }
            122 => {
                if lookahead == 108 { state = 189; lexer.advance(false); continue; }
                return result;
            }
            123 => {
                if lookahead == 100 { state = 190; lexer.advance(false); continue; }
                return result;
            }
            124 => {
                if lookahead == 117 { state = 191; lexer.advance(false); continue; }
                return result;
            }
            125 => {
                if lookahead == 117 { state = 192; lexer.advance(false); continue; }
                return result;
            }
            126 => {
                if lookahead == 105 { state = 193; lexer.advance(false); continue; }
                return result;
            }
            127 => {
                result = true; lexer.set_result_symbol(anon_sym_set); lexer.mark_end();
                return result;
            }
            128 => {
                if lookahead == 116 { state = 194; lexer.advance(false); continue; }
                return result;
            }
            129 => {
                if lookahead == 105 { state = 195; lexer.advance(false); continue; }
                return result;
            }
            130 => {
                if lookahead == 101 { state = 196; lexer.advance(false); continue; }
                return result;
            }
            131 => {
                if lookahead == 116 { state = 197; lexer.advance(false); continue; }
                return result;
            }
            132 => {
                if lookahead == 98 { state = 198; lexer.advance(false); continue; }
                return result;
            }
            133 => {
                if lookahead == 103 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            134 => {
                if lookahead == 115 { state = 200; lexer.advance(false); continue; }
                return result;
            }
            135 => {
                if lookahead == 111 { state = 201; lexer.advance(false); continue; }
                return result;
            }
            136 => {
                if lookahead == 101 { state = 202; lexer.advance(false); continue; }
                return result;
            }
            137 => {
                result = true; lexer.set_result_symbol(anon_sym_try); lexer.mark_end();
                return result;
            }
            138 => {
                if lookahead == 101 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            139 => {
                if lookahead == 101 { state = 204; lexer.advance(false); continue; }
                return result;
            }
            140 => {
                if lookahead == 113 { state = 205; lexer.advance(false); continue; }
                return result;
            }
            141 => {
                if lookahead == 110 { state = 206; lexer.advance(false); continue; }
                return result;
            }
            142 => {
                if lookahead == 110 { state = 207; lexer.advance(false); continue; }
                return result;
            }
            143 => {
                result = true; lexer.set_result_symbol(anon_sym_var); lexer.mark_end();
                return result;
            }
            144 => {
                if lookahead == 100 { state = 208; lexer.advance(false); continue; }
                return result;
            }
            145 => {
                if lookahead == 108 { state = 209; lexer.advance(false); continue; }
                return result;
            }
            146 => {
                if lookahead == 104 { state = 210; lexer.advance(false); continue; }
                return result;
            }
            147 => {
                if lookahead == 108 { state = 211; lexer.advance(false); continue; }
                return result;
            }
            148 => {
                if lookahead == 114 { state = 212; lexer.advance(false); continue; }
                return result;
            }
            149 => {
                if lookahead == 115 { state = 213; lexer.advance(false); continue; }
                return result;
            }
            150 => {
                if lookahead == 114 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            151 => {
                if lookahead == 99 { state = 215; lexer.advance(false); continue; }
                return result;
            }
            152 => {
                if lookahead == 116 { state = 216; lexer.advance(false); continue; }
                return result;
            }
            153 => {
                if lookahead == 101 { state = 217; lexer.advance(false); continue; }
                return result;
            }
            154 => {
                if lookahead == 107 { state = 218; lexer.advance(false); continue; }
                return result;
            }
            155 => {
                result = true; lexer.set_result_symbol(anon_sym_case); lexer.mark_end();
                return result;
            }
            156 => {
                if lookahead == 104 { state = 219; lexer.advance(false); continue; }
                return result;
            }
            157 => {
                if lookahead == 115 { state = 220; lexer.advance(false); continue; }
                return result;
            }
            158 => {
                if lookahead == 116 { state = 221; lexer.advance(false); continue; }
                return result;
            }
            159 => {
                if lookahead == 105 { state = 222; lexer.advance(false); continue; }
                return result;
            }
            160 => {
                if lookahead == 103 { state = 223; lexer.advance(false); continue; }
                return result;
            }
            161 => {
                if lookahead == 97 { state = 224; lexer.advance(false); continue; }
                return result;
            }
            162 => {
                if lookahead == 117 { state = 225; lexer.advance(false); continue; }
                return result;
            }
            163 => {
                if lookahead == 116 { state = 226; lexer.advance(false); continue; }
                return result;
            }
            164 => {
                result = true; lexer.set_result_symbol(anon_sym_else); lexer.mark_end();
                return result;
            }
            165 => {
                result = true; lexer.set_result_symbol(anon_sym_enum); lexer.mark_end();
                return result;
            }
            166 => {
                if lookahead == 114 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            167 => {
                if lookahead == 110 { state = 228; lexer.advance(false); continue; }
                return result;
            }
            168 => {
                if lookahead == 101 { state = 229; lexer.advance(false); continue; }
                return result;
            }
            169 => {
                if lookahead == 108 { state = 230; lexer.advance(false); continue; }
                return result;
            }
            170 => {
                result = true; lexer.set_result_symbol(anon_sym_from); lexer.mark_end();
                return result;
            }
            171 => {
                if lookahead == 116 { state = 231; lexer.advance(false); continue; }
                return result;
            }
            172 => {
                if lookahead == 97 { state = 232; lexer.advance(false); continue; }
                return result;
            }
            173 => {
                if lookahead == 101 { state = 233; lexer.advance(false); continue; }
                return result;
            }
            174 => {
                if lookahead == 114 { state = 234; lexer.advance(false); continue; }
                return result;
            }
            175 => {
                if lookahead == 114 { state = 235; lexer.advance(false); continue; }
                return result;
            }
            176 => {
                if lookahead == 97 { state = 236; lexer.advance(false); continue; }
                return result;
            }
            177 => {
                if lookahead == 114 { state = 237; lexer.advance(false); continue; }
                return result;
            }
            178 => {
                if lookahead == 102 { state = 238; lexer.advance(false); continue; }
                return result;
            }
            179 => {
                result = true; lexer.set_result_symbol(anon_sym_meta); lexer.mark_end();
                return result;
            }
            180 => {
                if lookahead == 108 { state = 239; lexer.advance(false); continue; }
                return result;
            }
            181 => {
                if lookahead == 115 { state = 240; lexer.advance(false); continue; }
                return result;
            }
            182 => {
                if lookahead == 114 { state = 241; lexer.advance(false); continue; }
                return result;
            }
            183 => {
                result = true; lexer.set_result_symbol(sym_null); lexer.mark_end();
                return result;
            }
            184 => {
                if lookahead == 101 { state = 242; lexer.advance(false); continue; }
                return result;
            }
            185 => {
                if lookahead == 99 { state = 243; lexer.advance(false); continue; }
                return result;
            }
            186 => {
                if lookahead == 114 { state = 244; lexer.advance(false); continue; }
                return result;
            }
            187 => {
                if lookahead == 97 { state = 245; lexer.advance(false); continue; }
                return result;
            }
            188 => {
                if lookahead == 101 { state = 246; lexer.advance(false); continue; }
                return result;
            }
            189 => {
                if lookahead == 105 { state = 247; lexer.advance(false); continue; }
                return result;
            }
            190 => {
                if lookahead == 111 { state = 248; lexer.advance(false); continue; }
                return result;
            }
            191 => {
                if lookahead == 105 { state = 249; lexer.advance(false); continue; }
                return result;
            }
            192 => {
                if lookahead == 114 { state = 250; lexer.advance(false); continue; }
                return result;
            }
            193 => {
                if lookahead == 115 { state = 251; lexer.advance(false); continue; }
                return result;
            }
            194 => {
                if lookahead == 105 { state = 252; lexer.advance(false); continue; }
                return result;
            }
            195 => {
                if lookahead == 110 { state = 253; lexer.advance(false); continue; }
                return result;
            }
            196 => {
                if lookahead == 114 { state = 254; lexer.advance(false); continue; }
                return result;
            }
            197 => {
                if lookahead == 99 { state = 255; lexer.advance(false); continue; }
                return result;
            }
            198 => {
                if lookahead == 111 { state = 256; lexer.advance(false); continue; }
                return result;
            }
            199 => {
                if lookahead == 101 { state = 257; lexer.advance(false); continue; }
                return result;
            }
            200 => {
                result = true; lexer.set_result_symbol(sym_this); lexer.mark_end();
                return result;
            }
            201 => {
                if lookahead == 119 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            202 => {
                result = true; lexer.set_result_symbol(sym_true); lexer.mark_end();
                return result;
            }
            203 => {
                result = true; lexer.set_result_symbol(anon_sym_type); lexer.mark_end();
                if lookahead == 111 { state = 259; lexer.advance(false); continue; }
                return result;
            }
            204 => {
                if lookahead == 102 { state = 260; lexer.advance(false); continue; }
                return result;
            }
            205 => {
                if lookahead == 117 { state = 261; lexer.advance(false); continue; }
                return result;
            }
            206 => {
                if lookahead == 111 { state = 262; lexer.advance(false); continue; }
                return result;
            }
            207 => {
                if lookahead == 103 { state = 263; lexer.advance(false); continue; }
                return result;
            }
            208 => {
                result = true; lexer.set_result_symbol(anon_sym_void); lexer.mark_end();
                return result;
            }
            209 => {
                if lookahead == 101 { state = 264; lexer.advance(false); continue; }
                return result;
            }
            210 => {
                result = true; lexer.set_result_symbol(anon_sym_with); lexer.mark_end();
                return result;
            }
            211 => {
                if lookahead == 100 { state = 265; lexer.advance(false); continue; }
                return result;
            }
            212 => {
                if lookahead == 97 { state = 266; lexer.advance(false); continue; }
                return result;
            }
            213 => {
                if lookahead == 115 { state = 267; lexer.advance(false); continue; }
                return result;
            }
            214 => {
                if lookahead == 116 { state = 268; lexer.advance(false); continue; }
                return result;
            }
            215 => {
                result = true; lexer.set_result_symbol(anon_sym_async); lexer.mark_end();
                return result;
            }
            216 => {
                result = true; lexer.set_result_symbol(anon_sym_await); lexer.mark_end();
                return result;
            }
            217 => {
                if lookahead == 97 { state = 269; lexer.advance(false); continue; }
                return result;
            }
            218 => {
                result = true; lexer.set_result_symbol(anon_sym_break); lexer.mark_end();
                return result;
            }
            219 => {
                result = true; lexer.set_result_symbol(anon_sym_catch); lexer.mark_end();
                return result;
            }
            220 => {
                result = true; lexer.set_result_symbol(anon_sym_class); lexer.mark_end();
                return result;
            }
            221 => {
                result = true; lexer.set_result_symbol(anon_sym_const); lexer.mark_end();
                return result;
            }
            222 => {
                if lookahead == 110 { state = 270; lexer.advance(false); continue; }
                return result;
            }
            223 => {
                if lookahead == 103 { state = 271; lexer.advance(false); continue; }
                return result;
            }
            224 => {
                if lookahead == 114 { state = 272; lexer.advance(false); continue; }
                return result;
            }
            225 => {
                if lookahead == 108 { state = 273; lexer.advance(false); continue; }
                return result;
            }
            226 => {
                if lookahead == 101 { state = 274; lexer.advance(false); continue; }
                return result;
            }
            227 => {
                if lookahead == 116 { state = 275; lexer.advance(false); continue; }
                return result;
            }
            228 => {
                if lookahead == 100 { state = 276; lexer.advance(false); continue; }
                return result;
            }
            229 => {
                result = true; lexer.set_result_symbol(sym_false); lexer.mark_end();
                return result;
            }
            230 => {
                if lookahead == 108 { state = 277; lexer.advance(false); continue; }
                return result;
            }
            231 => {
                if lookahead == 105 { state = 278; lexer.advance(false); continue; }
                return result;
            }
            232 => {
                if lookahead == 108 { state = 279; lexer.advance(false); continue; }
                return result;
            }
            233 => {
                if lookahead == 109 { state = 280; lexer.advance(false); continue; }
                return result;
            }
            234 => {
                if lookahead == 116 { state = 281; lexer.advance(false); continue; }
                return result;
            }
            235 => {
                result = true; lexer.set_result_symbol(anon_sym_infer); lexer.mark_end();
                return result;
            }
            236 => {
                if lookahead == 110 { state = 282; lexer.advance(false); continue; }
                return result;
            }
            237 => {
                if lookahead == 102 { state = 283; lexer.advance(false); continue; }
                return result;
            }
            238 => {
                result = true; lexer.set_result_symbol(anon_sym_keyof); lexer.mark_end();
                return result;
            }
            239 => {
                if lookahead == 101 { state = 284; lexer.advance(false); continue; }
                return result;
            }
            240 => {
                if lookahead == 112 { state = 285; lexer.advance(false); continue; }
                return result;
            }
            241 => {
                result = true; lexer.set_result_symbol(anon_sym_never); lexer.mark_end();
                return result;
            }
            242 => {
                if lookahead == 114 { state = 286; lexer.advance(false); continue; }
                return result;
            }
            243 => {
                if lookahead == 116 { state = 287; lexer.advance(false); continue; }
                return result;
            }
            244 => {
                if lookahead == 105 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            245 => {
                if lookahead == 116 { state = 289; lexer.advance(false); continue; }
                return result;
            }
            246 => {
                if lookahead == 99 { state = 290; lexer.advance(false); continue; }
                return result;
            }
            247 => {
                if lookahead == 99 { state = 291; lexer.advance(false); continue; }
                return result;
            }
            248 => {
                if lookahead == 110 { state = 292; lexer.advance(false); continue; }
                return result;
            }
            249 => {
                if lookahead == 114 { state = 293; lexer.advance(false); continue; }
                return result;
            }
            250 => {
                if lookahead == 110 { state = 294; lexer.advance(false); continue; }
                return result;
            }
            251 => {
                if lookahead == 102 { state = 295; lexer.advance(false); continue; }
                return result;
            }
            252 => {
                if lookahead == 99 { state = 296; lexer.advance(false); continue; }
                return result;
            }
            253 => {
                if lookahead == 103 { state = 297; lexer.advance(false); continue; }
                return result;
            }
            254 => {
                result = true; lexer.set_result_symbol(sym_super); lexer.mark_end();
                return result;
            }
            255 => {
                if lookahead == 104 { state = 298; lexer.advance(false); continue; }
                return result;
            }
            256 => {
                if lookahead == 108 { state = 299; lexer.advance(false); continue; }
                return result;
            }
            257 => {
                if lookahead == 116 { state = 300; lexer.advance(false); continue; }
                return result;
            }
            258 => {
                result = true; lexer.set_result_symbol(anon_sym_throw); lexer.mark_end();
                return result;
            }
            259 => {
                if lookahead == 102 { state = 301; lexer.advance(false); continue; }
                return result;
            }
            260 => {
                if lookahead == 105 { state = 302; lexer.advance(false); continue; }
                return result;
            }
            261 => {
                if lookahead == 101 { state = 303; lexer.advance(false); continue; }
                return result;
            }
            262 => {
                if lookahead == 119 { state = 304; lexer.advance(false); continue; }
                return result;
            }
            263 => {
                result = true; lexer.set_result_symbol(anon_sym_using); lexer.mark_end();
                return result;
            }
            264 => {
                result = true; lexer.set_result_symbol(anon_sym_while); lexer.mark_end();
                return result;
            }
            265 => {
                result = true; lexer.set_result_symbol(anon_sym_yield); lexer.mark_end();
                return result;
            }
            266 => {
                if lookahead == 99 { state = 305; lexer.advance(false); continue; }
                return result;
            }
            267 => {
                if lookahead == 111 { state = 306; lexer.advance(false); continue; }
                return result;
            }
            268 => {
                result = true; lexer.set_result_symbol(anon_sym_assert); lexer.mark_end();
                if lookahead == 115 { state = 307; lexer.advance(false); continue; }
                return result;
            }
            269 => {
                if lookahead == 110 { state = 308; lexer.advance(false); continue; }
                return result;
            }
            270 => {
                if lookahead == 117 { state = 309; lexer.advance(false); continue; }
                return result;
            }
            271 => {
                if lookahead == 101 { state = 310; lexer.advance(false); continue; }
                return result;
            }
            272 => {
                if lookahead == 101 { state = 311; lexer.advance(false); continue; }
                return result;
            }
            273 => {
                if lookahead == 116 { state = 312; lexer.advance(false); continue; }
                return result;
            }
            274 => {
                result = true; lexer.set_result_symbol(anon_sym_delete); lexer.mark_end();
                return result;
            }
            275 => {
                result = true; lexer.set_result_symbol(anon_sym_export); lexer.mark_end();
                return result;
            }
            276 => {
                if lookahead == 115 { state = 313; lexer.advance(false); continue; }
                return result;
            }
            277 => {
                if lookahead == 121 { state = 314; lexer.advance(false); continue; }
                return result;
            }
            278 => {
                if lookahead == 111 { state = 315; lexer.advance(false); continue; }
                return result;
            }
            279 => {
                result = true; lexer.set_result_symbol(anon_sym_global); lexer.mark_end();
                return result;
            }
            280 => {
                if lookahead == 101 { state = 316; lexer.advance(false); continue; }
                return result;
            }
            281 => {
                result = true; lexer.set_result_symbol(anon_sym_import); lexer.mark_end();
                return result;
            }
            282 => {
                if lookahead == 99 { state = 317; lexer.advance(false); continue; }
                return result;
            }
            283 => {
                if lookahead == 97 { state = 318; lexer.advance(false); continue; }
                return result;
            }
            284 => {
                result = true; lexer.set_result_symbol(anon_sym_module); lexer.mark_end();
                return result;
            }
            285 => {
                if lookahead == 97 { state = 319; lexer.advance(false); continue; }
                return result;
            }
            286 => {
                result = true; lexer.set_result_symbol(anon_sym_number); lexer.mark_end();
                return result;
            }
            287 => {
                result = true; lexer.set_result_symbol(anon_sym_object); lexer.mark_end();
                return result;
            }
            288 => {
                if lookahead == 100 { state = 320; lexer.advance(false); continue; }
                return result;
            }
            289 => {
                if lookahead == 101 { state = 321; lexer.advance(false); continue; }
                return result;
            }
            290 => {
                if lookahead == 116 { state = 322; lexer.advance(false); continue; }
                return result;
            }
            291 => {
                result = true; lexer.set_result_symbol(anon_sym_public); lexer.mark_end();
                return result;
            }
            292 => {
                if lookahead == 108 { state = 323; lexer.advance(false); continue; }
                return result;
            }
            293 => {
                if lookahead == 101 { state = 324; lexer.advance(false); continue; }
                return result;
            }
            294 => {
                result = true; lexer.set_result_symbol(anon_sym_return); lexer.mark_end();
                return result;
            }
            295 => {
                if lookahead == 105 { state = 325; lexer.advance(false); continue; }
                return result;
            }
            296 => {
                result = true; lexer.set_result_symbol(anon_sym_static); lexer.mark_end();
                return result;
            }
            297 => {
                result = true; lexer.set_result_symbol(anon_sym_string); lexer.mark_end();
                return result;
            }
            298 => {
                result = true; lexer.set_result_symbol(anon_sym_switch); lexer.mark_end();
                return result;
            }
            299 => {
                result = true; lexer.set_result_symbol(anon_sym_symbol); lexer.mark_end();
                return result;
            }
            300 => {
                result = true; lexer.set_result_symbol(anon_sym_target); lexer.mark_end();
                return result;
            }
            301 => {
                result = true; lexer.set_result_symbol(anon_sym_typeof); lexer.mark_end();
                return result;
            }
            302 => {
                if lookahead == 110 { state = 326; lexer.advance(false); continue; }
                return result;
            }
            303 => {
                result = true; lexer.set_result_symbol(anon_sym_unique); lexer.mark_end();
                return result;
            }
            304 => {
                if lookahead == 110 { state = 327; lexer.advance(false); continue; }
                return result;
            }
            305 => {
                if lookahead == 116 { state = 328; lexer.advance(false); continue; }
                return result;
            }
            306 => {
                if lookahead == 114 { state = 329; lexer.advance(false); continue; }
                return result;
            }
            307 => {
                result = true; lexer.set_result_symbol(anon_sym_asserts); lexer.mark_end();
                return result;
            }
            308 => {
                result = true; lexer.set_result_symbol(anon_sym_boolean); lexer.mark_end();
                return result;
            }
            309 => {
                if lookahead == 101 { state = 330; lexer.advance(false); continue; }
                return result;
            }
            310 => {
                if lookahead == 114 { state = 331; lexer.advance(false); continue; }
                return result;
            }
            311 => {
                result = true; lexer.set_result_symbol(anon_sym_declare); lexer.mark_end();
                return result;
            }
            312 => {
                result = true; lexer.set_result_symbol(anon_sym_default); lexer.mark_end();
                return result;
            }
            313 => {
                result = true; lexer.set_result_symbol(anon_sym_extends); lexer.mark_end();
                return result;
            }
            314 => {
                result = true; lexer.set_result_symbol(anon_sym_finally); lexer.mark_end();
                return result;
            }
            315 => {
                if lookahead == 110 { state = 332; lexer.advance(false); continue; }
                return result;
            }
            316 => {
                if lookahead == 110 { state = 333; lexer.advance(false); continue; }
                return result;
            }
            317 => {
                if lookahead == 101 { state = 334; lexer.advance(false); continue; }
                return result;
            }
            318 => {
                if lookahead == 99 { state = 335; lexer.advance(false); continue; }
                return result;
            }
            319 => {
                if lookahead == 99 { state = 336; lexer.advance(false); continue; }
                return result;
            }
            320 => {
                if lookahead == 101 { state = 337; lexer.advance(false); continue; }
                return result;
            }
            321 => {
                result = true; lexer.set_result_symbol(anon_sym_private); lexer.mark_end();
                return result;
            }
            322 => {
                if lookahead == 101 { state = 338; lexer.advance(false); continue; }
                return result;
            }
            323 => {
                if lookahead == 121 { state = 339; lexer.advance(false); continue; }
                return result;
            }
            324 => {
                result = true; lexer.set_result_symbol(anon_sym_require); lexer.mark_end();
                return result;
            }
            325 => {
                if lookahead == 101 { state = 340; lexer.advance(false); continue; }
                return result;
            }
            326 => {
                if lookahead == 101 { state = 341; lexer.advance(false); continue; }
                return result;
            }
            327 => {
                result = true; lexer.set_result_symbol(anon_sym_unknown); lexer.mark_end();
                return result;
            }
            328 => {
                result = true; lexer.set_result_symbol(anon_sym_abstract); lexer.mark_end();
                return result;
            }
            329 => {
                result = true; lexer.set_result_symbol(anon_sym_accessor); lexer.mark_end();
                return result;
            }
            330 => {
                result = true; lexer.set_result_symbol(anon_sym_continue); lexer.mark_end();
                return result;
            }
            331 => {
                result = true; lexer.set_result_symbol(anon_sym_debugger); lexer.mark_end();
                return result;
            }
            332 => {
                result = true; lexer.set_result_symbol(anon_sym_function); lexer.mark_end();
                return result;
            }
            333 => {
                if lookahead == 116 { state = 342; lexer.advance(false); continue; }
                return result;
            }
            334 => {
                if lookahead == 111 { state = 343; lexer.advance(false); continue; }
                return result;
            }
            335 => {
                if lookahead == 101 { state = 344; lexer.advance(false); continue; }
                return result;
            }
            336 => {
                if lookahead == 101 { state = 345; lexer.advance(false); continue; }
                return result;
            }
            337 => {
                result = true; lexer.set_result_symbol(anon_sym_override); lexer.mark_end();
                return result;
            }
            338 => {
                if lookahead == 100 { state = 346; lexer.advance(false); continue; }
                return result;
            }
            339 => {
                result = true; lexer.set_result_symbol(anon_sym_readonly); lexer.mark_end();
                return result;
            }
            340 => {
                if lookahead == 115 { state = 347; lexer.advance(false); continue; }
                return result;
            }
            341 => {
                if lookahead == 100 { state = 348; lexer.advance(false); continue; }
                return result;
            }
            342 => {
                if lookahead == 115 { state = 349; lexer.advance(false); continue; }
                return result;
            }
            343 => {
                if lookahead == 102 { state = 350; lexer.advance(false); continue; }
                return result;
            }
            344 => {
                result = true; lexer.set_result_symbol(anon_sym_interface); lexer.mark_end();
                return result;
            }
            345 => {
                result = true; lexer.set_result_symbol(anon_sym_namespace); lexer.mark_end();
                return result;
            }
            346 => {
                result = true; lexer.set_result_symbol(anon_sym_protected); lexer.mark_end();
                return result;
            }
            347 => {
                result = true; lexer.set_result_symbol(anon_sym_satisfies); lexer.mark_end();
                return result;
            }
            348 => {
                result = true; lexer.set_result_symbol(sym_undefined); lexer.mark_end();
                return result;
            }
            349 => {
                result = true; lexer.set_result_symbol(anon_sym_implements); lexer.mark_end();
                return result;
            }
            350 => {
                result = true; lexer.set_result_symbol(anon_sym_instanceof); lexer.mark_end();
                return result;
            }
            _ => return false,
        }
    }
}
