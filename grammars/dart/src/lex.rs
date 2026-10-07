//! The `dart` grammar's lexer: `ts_lex` and `ts_lex_keywords`, transliterated from
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

const anon_sym_AMP: Symbol = 94;
const anon_sym_AMP_AMP: Symbol = 85;
const anon_sym_AMP_EQ: Symbol = 78;
const anon_sym_AT: Symbol = 22;
const anon_sym_BANG: Symbol = 91;
const anon_sym_BANG_EQ: Symbol = 86;
const anon_sym_CARET: Symbol = 93;
const anon_sym_CARET_EQ: Symbol = 79;
const anon_sym_COLON: Symbol = 60;
const anon_sym_COMMA: Symbol = 19;
const anon_sym_DASH: Symbol = 99;
const anon_sym_DASH_DASH: Symbol = 105;
const anon_sym_DASH_EQ: Symbol = 70;
const anon_sym_DOLLAR: Symbol = 48;
const anon_sym_DOT: Symbol = 21;
const anon_sym_DOT_DOT: Symbol = 106;
const anon_sym_DOT_DOT_DOT: Symbol = 61;
const anon_sym_DOT_DOT_DOT_QMARK: Symbol = 62;
const anon_sym_DQUOTE: Symbol = 43;
const anon_sym_DQUOTE_DQUOTE_DQUOTE: Symbol = 45;
const anon_sym_EQ: Symbol = 68;
const anon_sym_EQ_EQ: Symbol = 17;
const anon_sym_EQ_GT: Symbol = 110;
const anon_sym_Function: Symbol = 24;
const anon_sym_GT: Symbol = 32;
const anon_sym_GT_EQ: Symbol = 89;
const anon_sym_GT_GT: Symbol = 97;
const anon_sym_GT_GT_EQ: Symbol = 76;
const anon_sym_GT_GT_GT: Symbol = 96;
const anon_sym_GT_GT_GT_EQ: Symbol = 77;
const anon_sym_LBRACE: Symbol = 28;
const anon_sym_LBRACK: Symbol = 26;
const anon_sym_LBRACK_RBRACK: Symbol = 57;
const anon_sym_LBRACK_RBRACK_EQ: Symbol = 58;
const anon_sym_LF: Symbol = 4;
const anon_sym_LPAREN: Symbol = 15;
const anon_sym_LT: Symbol = 31;
const anon_sym_LT_EQ: Symbol = 88;
const anon_sym_LT_LT: Symbol = 95;
const anon_sym_LT_LT_EQ: Symbol = 75;
const anon_sym_PERCENT: Symbol = 102;
const anon_sym_PERCENT_EQ: Symbol = 73;
const anon_sym_PIPE: Symbol = 92;
const anon_sym_PIPE_EQ: Symbol = 80;
const anon_sym_PIPE_PIPE: Symbol = 84;
const anon_sym_PLUS: Symbol = 98;
const anon_sym_PLUS_EQ: Symbol = 69;
const anon_sym_PLUS_PLUS: Symbol = 104;
const anon_sym_POUND: Symbol = 55;
const anon_sym_POUND_BANG: Symbol = 2;
const anon_sym_QMARK: Symbol = 23;
const anon_sym_QMARK_DOT: Symbol = 82;
const anon_sym_QMARK_DOT_DOT: Symbol = 107;
const anon_sym_QMARK_QMARK: Symbol = 83;
const anon_sym_QMARK_QMARK_EQ: Symbol = 81;
const anon_sym_RBRACE: Symbol = 29;
const anon_sym_RBRACK: Symbol = 27;
const anon_sym_RPAREN: Symbol = 16;
const anon_sym_SEMI: Symbol = 6;
const anon_sym_SLASH: Symbol = 101;
const anon_sym_SLASH_EQ: Symbol = 72;
const anon_sym_SLASH_SLASH: Symbol = 154;
const anon_sym_SLASH_SLASH_SLASH: Symbol = 156;
const anon_sym_SQUOTE: Symbol = 44;
const anon_sym_SQUOTE_SQUOTE_SQUOTE: Symbol = 46;
const anon_sym_STAR: Symbol = 100;
const anon_sym_STAR_EQ: Symbol = 71;
const anon_sym_TILDE: Symbol = 56;
const anon_sym_TILDE_SLASH: Symbol = 103;
const anon_sym_TILDE_SLASH_EQ: Symbol = 74;
const anon_sym_abstract: Symbol = 137;
const anon_sym_as: Symbol = 10;
const anon_sym_assert: Symbol = 130;
const anon_sym_async: Symbol = 109;
const anon_sym_async_STAR: Symbol = 111;
const anon_sym_augment: Symbol = 7;
const anon_sym_await: Symbol = 65;
const anon_sym_base: Symbol = 141;
const anon_sym_break: Symbol = 125;
const anon_sym_case: Symbol = 63;
const anon_sym_catch: Symbol = 123;
const anon_sym_class: Symbol = 144;
const anon_sym_const: Symbol = 59;
const anon_sym_continue: Symbol = 126;
const anon_sym_covariant: Symbol = 114;
const anon_sym_default: Symbol = 120;
const anon_sym_deferred: Symbol = 9;
const anon_sym_do: Symbol = 118;
const anon_sym_dynamic: Symbol = 153;
const anon_sym_else: Symbol = 64;
const anon_sym_enum: Symbol = 150;
const anon_sym_export: Symbol = 11;
const anon_sym_extends: Symbol = 33;
const anon_sym_extension: Symbol = 148;
const anon_sym_external: Symbol = 136;
const anon_sym_factory: Symbol = 138;
const anon_sym_final: Symbol = 115;
const anon_sym_finally: Symbol = 124;
const anon_sym_for: Symbol = 66;
const anon_sym_get: Symbol = 131;
const anon_sym_hide: Symbol = 20;
const anon_sym_if: Symbol = 14;
const anon_sym_implements: Symbol = 147;
const anon_sym_import: Symbol = 8;
const anon_sym_in: Symbol = 34;
const anon_sym_inline: Symbol = 143;
const anon_sym_inout: Symbol = 36;
const anon_sym_interface: Symbol = 142;
const anon_sym_is: Symbol = 90;
const anon_sym_late: Symbol = 134;
const anon_sym_library: Symbol = 5;
const anon_sym_mixin: Symbol = 145;
const anon_sym_native: Symbol = 133;
const anon_sym_new: Symbol = 113;
const anon_sym_of: Symbol = 13;
const anon_sym_on: Symbol = 122;
const anon_sym_operator: Symbol = 139;
const anon_sym_out: Symbol = 35;
const anon_sym_part: Symbol = 12;
const anon_sym_r_DQUOTE: Symbol = 47;
const anon_sym_r_DQUOTE_DQUOTE_DQUOTE: Symbol = 50;
const anon_sym_r_SQUOTE: Symbol = 49;
const anon_sym_r_SQUOTE_SQUOTE_SQUOTE: Symbol = 51;
const anon_sym_required: Symbol = 30;
const anon_sym_rethrow: Symbol = 127;
const anon_sym_return: Symbol = 128;
const anon_sym_sealed: Symbol = 140;
const anon_sym_set: Symbol = 132;
const anon_sym_show: Symbol = 18;
const anon_sym_static: Symbol = 135;
const anon_sym_super: Symbol = 87;
const anon_sym_switch: Symbol = 119;
const anon_sym_sync_STAR: Symbol = 112;
const anon_sym_this: Symbol = 108;
const anon_sym_throw: Symbol = 67;
const anon_sym_try: Symbol = 121;
const anon_sym_type: Symbol = 149;
const anon_sym_typedef: Symbol = 151;
const anon_sym_var: Symbol = 116;
const anon_sym_when: Symbol = 152;
const anon_sym_while: Symbol = 117;
const anon_sym_with: Symbol = 146;
const anon_sym_yield: Symbol = 129;
const aux_sym__sub_string_test_token1: Symbol = 52;
const aux_sym_comment_token1: Symbol = 155;
const aux_sym_comment_token2: Symbol = 157;
const aux_sym_script_tag_token1: Symbol = 3;
const sym__name: Symbol = 1;
const sym__unused_escape_sequence: Symbol = 54;
const sym_decimal_floating_point_literal: Symbol = 42;
const sym_decimal_integer_literal: Symbol = 40;
const sym_false: Symbol = 39;
const sym_hex_integer_literal: Symbol = 41;
const sym_identifier_dollar_escaped: Symbol = 53;
const sym_null_literal: Symbol = 37;
const sym_true: Symbol = 38;
const sym_void_type: Symbol = 25;
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
                if eof { state = 97; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 207), (34, 148), (35, 172), (36, 155), (37, 235), (38, 215), (39, 150), (40, 107),
                    (41, 108), (42, 231), (43, 224), (44, 110), (45, 228), (46, 112), (47, 233), (48, 142),
                    (58, 177), (59, 103), (60, 134), (61, 182), (62, 140), (63, 121), (64, 118), (91, 129),
                    (92, 82), (93, 130), (94, 212), (97, 253), (114, 247), (115, 256), (123, 131), (124, 209),
                    (125, 132), (126, 174),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 94; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 143; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            1 => {
                if lookahead == 10 { state = 102; lexer.advance(false); continue; }
                if lookahead == 47 { state = 52; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 1; lexer.advance(true); continue; }
                return result;
            }
            2 => {
                if let Some(next) = advance_map(&[
                    (33, 207), (34, 148), (35, 171), (37, 234), (38, 214), (39, 150), (40, 107), (41, 108),
                    (42, 230), (43, 223), (44, 110), (45, 227), (46, 114), (47, 232), (48, 142), (58, 177),
                    (59, 103), (60, 135), (61, 182), (62, 141), (63, 122), (64, 118), (91, 129), (93, 130),
                    (94, 211), (97, 254), (114, 247), (123, 131), (124, 210), (125, 132), (126, 174),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 2; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 143; lexer.advance(false); continue; }
                if 36 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            3 => {
                if let Some(next) = advance_map(&[
                    (33, 207), (34, 148), (35, 171), (39, 150), (40, 107), (41, 108), (43, 47), (44, 110),
                    (45, 227), (46, 115), (47, 52), (48, 142), (58, 177), (60, 137), (61, 64), (62, 139),
                    (63, 119), (64, 118), (91, 129), (93, 130), (97, 254), (114, 247), (123, 131), (125, 132),
                    (126, 173),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 3; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 143; lexer.advance(false); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            4 => {
                if let Some(next) = advance_map(&[
                    (33, 207), (34, 148), (37, 234), (38, 214), (39, 150), (40, 107), (41, 108), (42, 230),
                    (43, 222), (44, 110), (45, 226), (46, 113), (47, 232), (58, 177), (59, 103), (60, 135),
                    (61, 182), (62, 141), (63, 122), (91, 129), (93, 130), (94, 211), (97, 253), (114, 247),
                    (115, 256), (123, 131), (124, 210), (125, 132), (126, 54),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 4; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                if let Some(next) = advance_map(&[
                    (33, 207), (34, 148), (37, 234), (38, 214), (39, 150), (40, 107), (41, 108), (42, 230),
                    (43, 222), (44, 110), (45, 226), (46, 113), (47, 232), (58, 177), (59, 103), (60, 135),
                    (61, 182), (62, 141), (63, 122), (91, 129), (93, 130), (94, 211), (97, 254), (114, 247),
                    (123, 131), (124, 210), (125, 132), (126, 54),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 5; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            6 => {
                if let Some(next) = advance_map(&[
                    (33, 207), (34, 148), (37, 234), (38, 214), (39, 150), (40, 107), (41, 108), (42, 230),
                    (43, 222), (44, 110), (45, 226), (46, 111), (47, 232), (58, 177), (60, 135), (61, 65),
                    (62, 141), (63, 126), (91, 129), (93, 130), (94, 211), (97, 254), (114, 247), (124, 210),
                    (125, 132), (126, 54),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 6; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if let Some(next) = advance_map(&[
                    (33, 207), (37, 235), (38, 215), (40, 107), (41, 108), (42, 231), (43, 224), (44, 110),
                    (45, 228), (46, 111), (47, 233), (58, 177), (60, 134), (61, 182), (62, 140), (63, 125),
                    (91, 129), (93, 130), (94, 212), (97, 254), (124, 209), (125, 132), (126, 55),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 7; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            8 => {
                if let Some(next) = advance_map(&[
                    (33, 207), (37, 235), (38, 215), (40, 107), (41, 108), (42, 231), (43, 225), (44, 110),
                    (45, 229), (46, 113), (47, 233), (58, 177), (59, 103), (60, 134), (61, 182), (62, 140),
                    (63, 121), (91, 129), (93, 130), (94, 212), (97, 254), (124, 209), (125, 132), (126, 55),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 8; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if let Some(next) = advance_map(&[
                    (33, 207), (37, 235), (38, 215), (40, 107), (42, 231), (43, 224), (44, 110), (45, 228),
                    (46, 113), (47, 233), (59, 103), (60, 134), (61, 182), (62, 140), (63, 121), (91, 129),
                    (94, 212), (97, 253), (115, 256), (123, 131), (124, 209), (126, 55),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 9; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            10 => {
                if let Some(next) = advance_map(&[
                    (33, 207), (37, 235), (38, 215), (40, 107), (42, 231), (43, 225), (44, 110), (45, 229),
                    (46, 113), (47, 233), (59, 103), (60, 134), (61, 182), (62, 140), (63, 121), (91, 129),
                    (94, 212), (97, 253), (115, 256), (123, 131), (124, 209), (126, 55),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 10; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if let Some(next) = advance_map(&[
                    (33, 207), (37, 234), (38, 214), (40, 107), (41, 108), (42, 230), (43, 223), (44, 110),
                    (45, 227), (46, 111), (47, 232), (58, 177), (60, 135), (61, 65), (62, 141), (63, 126),
                    (91, 129), (93, 130), (94, 211), (97, 254), (124, 210), (125, 132), (126, 54),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 11; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if let Some(next) = advance_map(&[
                    (33, 207), (37, 234), (38, 214), (40, 107), (41, 108), (42, 230), (43, 222), (44, 110),
                    (45, 226), (46, 111), (47, 232), (58, 177), (60, 135), (61, 65), (62, 141), (63, 126),
                    (91, 129), (93, 130), (94, 211), (97, 253), (115, 256), (123, 131), (124, 210), (125, 132),
                    (126, 54),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 12; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if let Some(next) = advance_map(&[
                    (33, 207), (37, 234), (38, 214), (40, 107), (42, 230), (43, 223), (44, 110), (45, 227),
                    (46, 113), (47, 232), (59, 103), (60, 135), (61, 65), (62, 141), (63, 122), (91, 129),
                    (94, 211), (97, 253), (115, 256), (123, 131), (124, 210), (126, 54),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 13; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if lookahead == 33 { state = 98; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if let Some(next) = advance_map(&[
                    (33, 206), (34, 148), (35, 171), (38, 41), (39, 150), (40, 107), (41, 108), (42, 230),
                    (43, 47), (44, 110), (45, 227), (46, 115), (47, 52), (48, 142), (58, 177), (59, 103),
                    (60, 133), (61, 182), (62, 138), (63, 119), (64, 118), (91, 129), (93, 130), (97, 254),
                    (114, 247), (123, 131), (124, 84), (125, 132), (126, 173),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 15; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 143; lexer.advance(false); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if let Some(next) = advance_map(&[
                    (33, 206), (34, 148), (36, 154), (38, 41), (39, 149), (41, 108), (44, 110), (47, 52),
                    (61, 181), (63, 119), (92, 82), (93, 130), (97, 81), (124, 84), (125, 132),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 17; lexer.advance(true); continue; }
                return result;
            }
            17 => {
                if let Some(next) = advance_map(&[
                    (33, 206), (34, 148), (36, 154), (38, 41), (39, 149), (41, 108), (44, 110), (47, 52),
                    (61, 181), (63, 119), (93, 130), (97, 81), (124, 84), (125, 132),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 17; lexer.advance(true); continue; }
                return result;
            }
            18 => {
                if let Some(next) = advance_map(&[
                    (33, 206), (37, 59), (38, 42), (40, 107), (41, 108), (42, 61), (43, 62), (44, 110),
                    (45, 63), (46, 113), (47, 53), (58, 177), (59, 103), (60, 136), (61, 183), (62, 75),
                    (63, 123), (64, 118), (91, 129), (93, 130), (94, 66), (123, 131), (124, 68), (125, 132),
                    (126, 57),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 18; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if let Some(next) = advance_map(&[
                    (33, 206), (37, 59), (38, 60), (40, 107), (41, 108), (42, 61), (43, 62), (44, 110),
                    (45, 63), (46, 113), (47, 53), (58, 177), (59, 103), (60, 136), (61, 183), (62, 75),
                    (63, 123), (91, 129), (93, 130), (94, 66), (97, 255), (115, 256), (123, 131), (124, 67),
                    (125, 132), (126, 57),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 19; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            20 => {
                if let Some(next) = advance_map(&[
                    (33, 206), (40, 107), (44, 110), (46, 113), (47, 52), (59, 103), (60, 133), (61, 74),
                    (63, 120), (91, 129), (97, 255), (115, 256), (123, 131),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 20; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                if let Some(next) = advance_map(&[
                    (33, 206), (40, 107), (44, 110), (46, 111), (47, 52), (59, 103), (60, 133), (61, 183),
                    (63, 124), (91, 129), (97, 255), (115, 256), (123, 131),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 21; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            22 => {
                if lookahead == 33 { state = 206; lexer.advance(false); continue; }
                if lookahead == 40 { state = 107; lexer.advance(false); continue; }
                if lookahead == 46 { state = 111; lexer.advance(false); continue; }
                if lookahead == 47 { state = 52; lexer.advance(false); continue; }
                if lookahead == 60 { state = 133; lexer.advance(false); continue; }
                if lookahead == 63 { state = 124; lexer.advance(false); continue; }
                if lookahead == 91 { state = 129; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 22; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            23 => {
                if let Some(next) = advance_map(&[
                    (33, 58), (34, 148), (35, 171), (39, 150), (40, 107), (41, 108), (44, 110), (45, 226),
                    (46, 116), (47, 52), (48, 142), (58, 177), (60, 137), (61, 64), (62, 139), (64, 118),
                    (91, 129), (93, 130), (97, 254), (114, 247), (123, 131), (125, 132),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 23; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 143; lexer.advance(false); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            24 => {
                if let Some(next) = advance_map(&[
                    (33, 58), (37, 234), (38, 214), (40, 107), (41, 108), (42, 230), (43, 222), (44, 110),
                    (45, 226), (46, 113), (47, 232), (58, 177), (59, 103), (60, 135), (61, 65), (62, 141),
                    (63, 127), (93, 130), (94, 211), (97, 254), (124, 210), (125, 132), (126, 54),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 24; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            25 => {
                if let Some(next) = advance_map(&[
                    (33, 58), (37, 234), (38, 214), (40, 107), (41, 108), (42, 230), (43, 222), (44, 110),
                    (45, 226), (46, 111), (47, 232), (58, 177), (60, 135), (61, 65), (62, 141), (63, 128),
                    (93, 130), (94, 211), (97, 254), (124, 210), (125, 132), (126, 54),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 25; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            26 => {
                if let Some(next) = advance_map(&[
                    (33, 58), (37, 234), (38, 214), (40, 107), (42, 230), (43, 222), (44, 110), (45, 226),
                    (46, 113), (47, 232), (59, 103), (60, 135), (61, 65), (62, 141), (63, 127), (94, 211),
                    (97, 253), (115, 256), (123, 131), (124, 210), (126, 54),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 26; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            27 => {
                if let Some(next) = advance_map(&[
                    (33, 58), (37, 234), (38, 214), (41, 108), (42, 230), (43, 223), (44, 110), (45, 227),
                    (46, 50), (47, 232), (58, 177), (59, 103), (60, 135), (61, 65), (62, 141), (63, 127),
                    (93, 130), (94, 211), (97, 254), (124, 210), (125, 132), (126, 54),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 27; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            28 => {
                if let Some(next) = advance_map(&[
                    (33, 58), (37, 234), (38, 214), (41, 108), (42, 230), (43, 223), (44, 110), (45, 227),
                    (47, 232), (58, 177), (60, 135), (61, 65), (62, 141), (63, 128), (93, 130), (94, 211),
                    (97, 254), (124, 210), (125, 132), (126, 54),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 28; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                if let Some(next) = advance_map(&[
                    (33, 58), (37, 234), (38, 214), (42, 230), (43, 223), (44, 110), (45, 227), (46, 50),
                    (47, 232), (59, 103), (60, 135), (61, 65), (62, 141), (63, 127), (94, 211), (97, 253),
                    (115, 256), (123, 131), (124, 210), (126, 54),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 29; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            30 => {
                if lookahead == 34 { state = 148; lexer.advance(false); continue; }
                if lookahead == 36 { state = 154; lexer.advance(false); continue; }
                if lookahead == 39 { state = 149; lexer.advance(false); continue; }
                if lookahead == 47 { state = 164; lexer.advance(false); continue; }
                if lookahead == 92 { state = 166; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 160; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 65 || 90 < lookahead) && lookahead != 95 && (lookahead < 97 || 123 < lookahead) { state = 159; lexer.advance(false); continue; }
                return result;
            }
            31 => {
                if lookahead == 34 { state = 151; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                if lookahead == 34 { state = 157; lexer.advance(false); continue; }
                return result;
            }
            33 => {
                if lookahead == 34 { state = 147; lexer.advance(false); continue; }
                if lookahead == 36 { state = 154; lexer.advance(false); continue; }
                if lookahead == 39 { state = 150; lexer.advance(false); continue; }
                if lookahead == 47 { state = 164; lexer.advance(false); continue; }
                if lookahead == 92 { state = 166; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 161; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 65 || 90 < lookahead) && lookahead != 95 && (lookahead < 97 || 123 < lookahead) { state = 159; lexer.advance(false); continue; }
                return result;
            }
            34 => {
                if lookahead == 34 { state = 147; lexer.advance(false); continue; }
                if lookahead == 36 { state = 154; lexer.advance(false); continue; }
                if lookahead == 39 { state = 150; lexer.advance(false); continue; }
                if lookahead == 47 { state = 52; lexer.advance(false); continue; }
                if lookahead == 92 { state = 82; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 35; lexer.advance(true); continue; }
                return result;
            }
            35 => {
                if lookahead == 34 { state = 147; lexer.advance(false); continue; }
                if lookahead == 36 { state = 154; lexer.advance(false); continue; }
                if lookahead == 39 { state = 150; lexer.advance(false); continue; }
                if lookahead == 47 { state = 52; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 35; lexer.advance(true); continue; }
                return result;
            }
            36 => {
                if lookahead == 34 { state = 147; lexer.advance(false); continue; }
                if lookahead == 36 { state = 154; lexer.advance(false); continue; }
                if lookahead == 39 { state = 149; lexer.advance(false); continue; }
                if lookahead == 47 { state = 164; lexer.advance(false); continue; }
                if lookahead == 92 { state = 166; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 162; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 65 || 90 < lookahead) && lookahead != 95 && (lookahead < 97 || 123 < lookahead) { state = 159; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                if lookahead == 34 { state = 147; lexer.advance(false); continue; }
                if lookahead == 36 { state = 154; lexer.advance(false); continue; }
                if lookahead == 39 { state = 149; lexer.advance(false); continue; }
                if lookahead == 47 { state = 52; lexer.advance(false); continue; }
                if lookahead == 92 { state = 82; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 38; lexer.advance(true); continue; }
                return result;
            }
            38 => {
                if lookahead == 34 { state = 147; lexer.advance(false); continue; }
                if lookahead == 36 { state = 154; lexer.advance(false); continue; }
                if lookahead == 39 { state = 149; lexer.advance(false); continue; }
                if lookahead == 47 { state = 52; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 38; lexer.advance(true); continue; }
                return result;
            }
            39 => {
                if lookahead == 36 { state = 167; lexer.advance(false); continue; }
                return result;
            }
            40 => {
                if let Some(next) = advance_map(&[
                    (37, 234), (38, 213), (40, 107), (42, 230), (43, 222), (44, 110), (45, 226), (46, 111),
                    (47, 232), (59, 103), (60, 135), (61, 64), (62, 141), (63, 119), (91, 76), (94, 211),
                    (97, 254), (124, 208), (126, 174),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 40; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                if lookahead == 38 { state = 202; lexer.advance(false); continue; }
                return result;
            }
            42 => {
                if lookahead == 38 { state = 202; lexer.advance(false); continue; }
                if lookahead == 61 { state = 193; lexer.advance(false); continue; }
                return result;
            }
            43 => {
                if let Some(next) = advance_map(&[
                    (38, 41), (40, 107), (41, 108), (44, 110), (46, 111), (47, 52), (58, 177), (59, 103),
                    (60, 133), (61, 183), (62, 138), (63, 119), (93, 130), (123, 131), (124, 84), (125, 132),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 43; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            44 => {
                if lookahead == 39 { state = 152; lexer.advance(false); continue; }
                return result;
            }
            45 => {
                if lookahead == 39 { state = 158; lexer.advance(false); continue; }
                return result;
            }
            46 => {
                if let Some(next) = advance_map(&[
                    (41, 108), (44, 110), (47, 52), (58, 177), (59, 103), (61, 183), (63, 119), (93, 130),
                    (97, 255), (115, 256), (123, 131), (125, 132),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 46; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                if lookahead == 43 { state = 238; lexer.advance(false); continue; }
                return result;
            }
            48 => {
                if lookahead == 46 { state = 179; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                if lookahead == 46 { state = 242; lexer.advance(false); continue; }
                return result;
            }
            50 => {
                if lookahead == 46 { state = 240; lexer.advance(false); continue; }
                return result;
            }
            51 => {
                if lookahead == 46 { state = 178; lexer.advance(false); continue; }
                return result;
            }
            52 => {
                if lookahead == 47 { state = 259; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                if lookahead == 47 { state = 259; lexer.advance(false); continue; }
                if lookahead == 61 { state = 187; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                if lookahead == 47 { state = 236; lexer.advance(false); continue; }
                return result;
            }
            55 => {
                if lookahead == 47 { state = 237; lexer.advance(false); continue; }
                return result;
            }
            56 => {
                if lookahead == 47 { state = 164; lexer.advance(false); continue; }
                if lookahead == 92 { state = 163; lexer.advance(false); continue; }
                if lookahead == 123 { state = 131; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 165; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 167; lexer.advance(false); continue; }
                if lookahead != 0 { state = 159; lexer.advance(false); continue; }
                return result;
            }
            57 => {
                if lookahead == 47 { state = 73; lexer.advance(false); continue; }
                return result;
            }
            58 => {
                if lookahead == 61 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                if lookahead == 61 { state = 188; lexer.advance(false); continue; }
                return result;
            }
            60 => {
                if lookahead == 61 { state = 193; lexer.advance(false); continue; }
                return result;
            }
            61 => {
                if lookahead == 61 { state = 186; lexer.advance(false); continue; }
                return result;
            }
            62 => {
                if lookahead == 61 { state = 184; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                if lookahead == 61 { state = 185; lexer.advance(false); continue; }
                return result;
            }
            64 => {
                if lookahead == 61 { state = 109; lexer.advance(false); continue; }
                return result;
            }
            65 => {
                if lookahead == 61 { state = 109; lexer.advance(false); continue; }
                if lookahead == 62 { state = 244; lexer.advance(false); continue; }
                return result;
            }
            66 => {
                if lookahead == 61 { state = 194; lexer.advance(false); continue; }
                return result;
            }
            67 => {
                if lookahead == 61 { state = 195; lexer.advance(false); continue; }
                return result;
            }
            68 => {
                if lookahead == 61 { state = 195; lexer.advance(false); continue; }
                if lookahead == 124 { state = 201; lexer.advance(false); continue; }
                return result;
            }
            69 => {
                if lookahead == 61 { state = 190; lexer.advance(false); continue; }
                return result;
            }
            70 => {
                if lookahead == 61 { state = 191; lexer.advance(false); continue; }
                if lookahead == 62 { state = 72; lexer.advance(false); continue; }
                return result;
            }
            71 => {
                if lookahead == 61 { state = 196; lexer.advance(false); continue; }
                return result;
            }
            72 => {
                if lookahead == 61 { state = 192; lexer.advance(false); continue; }
                return result;
            }
            73 => {
                if lookahead == 61 { state = 189; lexer.advance(false); continue; }
                return result;
            }
            74 => {
                if lookahead == 62 { state = 244; lexer.advance(false); continue; }
                return result;
            }
            75 => {
                if lookahead == 62 { state = 70; lexer.advance(false); continue; }
                return result;
            }
            76 => {
                if lookahead == 93 { state = 175; lexer.advance(false); continue; }
                return result;
            }
            77 => {
                if lookahead == 95 { state = 77; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 143; lexer.advance(false); continue; }
                return result;
            }
            78 => {
                if lookahead == 95 { state = 78; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 144; lexer.advance(false); continue; }
                return result;
            }
            79 => {
                if lookahead == 95 { state = 79; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 145; lexer.advance(false); continue; }
                return result;
            }
            80 => {
                if lookahead == 95 { state = 80; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 146; lexer.advance(false); continue; }
                return result;
            }
            81 => {
                if lookahead == 115 { state = 104; lexer.advance(false); continue; }
                return result;
            }
            82 => {
                if lookahead == 117 { state = 83; lexer.advance(false); continue; }
                if lookahead == 120 { state = 93; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 55 { state = 170; lexer.advance(false); continue; }
                if lookahead != 0 { state = 168; lexer.advance(false); continue; }
                return result;
            }
            83 => {
                if lookahead == 123 { state = 91; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 92; lexer.advance(false); continue; }
                return result;
            }
            84 => {
                if lookahead == 124 { state = 201; lexer.advance(false); continue; }
                return result;
            }
            85 => {
                if lookahead == 125 { state = 168; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 85; lexer.advance(false); continue; }
                return result;
            }
            86 => {
                if lookahead == 43 || lookahead == 45 { state = 88; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 146; lexer.advance(false); continue; }
                return result;
            }
            87 => {
                if 48 <= lookahead && lookahead <= 57 { state = 145; lexer.advance(false); continue; }
                return result;
            }
            88 => {
                if 48 <= lookahead && lookahead <= 57 { state = 146; lexer.advance(false); continue; }
                return result;
            }
            89 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 168; lexer.advance(false); continue; }
                return result;
            }
            90 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 144; lexer.advance(false); continue; }
                return result;
            }
            91 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 85; lexer.advance(false); continue; }
                return result;
            }
            92 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 93; lexer.advance(false); continue; }
                return result;
            }
            93 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 89; lexer.advance(false); continue; }
                return result;
            }
            94 => {
                if eof { state = 97; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 207), (34, 148), (35, 172), (36, 155), (37, 235), (38, 215), (39, 150), (40, 107),
                    (41, 108), (42, 231), (43, 224), (44, 110), (45, 228), (46, 112), (47, 233), (48, 142),
                    (58, 177), (59, 103), (60, 134), (61, 182), (62, 140), (63, 121), (64, 118), (91, 129),
                    (93, 130), (94, 212), (97, 253), (114, 247), (115, 256), (123, 131), (124, 209), (125, 132),
                    (126, 174),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 94; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 143; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            95 => {
                if eof { state = 97; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 207), (34, 148), (35, 14), (37, 235), (38, 215), (39, 150), (40, 107), (41, 108),
                    (42, 231), (43, 224), (44, 110), (45, 228), (46, 113), (47, 233), (58, 177), (59, 103),
                    (60, 134), (61, 182), (62, 140), (63, 121), (64, 118), (91, 129), (93, 130), (94, 212),
                    (97, 254), (114, 247), (123, 131), (124, 209), (125, 132), (126, 55),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 95; lexer.advance(true); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            96 => {
                if eof { state = 97; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 206), (34, 148), (35, 171), (39, 150), (40, 107), (43, 47), (45, 227), (46, 117),
                    (47, 52), (48, 142), (59, 103), (60, 133), (63, 124), (64, 118), (91, 129), (97, 254),
                    (114, 247), (123, 131), (125, 132), (126, 173),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 96; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 143; lexer.advance(false); continue; }
                if lookahead == 36 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            97 => {
                result = true; lexer.set_result_symbol(ts_builtin_sym_end); lexer.mark_end();
                return result;
            }
            98 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND_BANG); lexer.mark_end();
                return result;
            }
            99 => {
                result = true; lexer.set_result_symbol(aux_sym_script_tag_token1); lexer.mark_end();
                if lookahead == 47 { state = 100; lexer.advance(false); continue; }
                if lookahead == 9 || 11 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 99; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) { state = 101; lexer.advance(false); continue; }
                return result;
            }
            100 => {
                result = true; lexer.set_result_symbol(aux_sym_script_tag_token1); lexer.mark_end();
                if lookahead == 47 { state = 262; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 101; lexer.advance(false); continue; }
                return result;
            }
            101 => {
                result = true; lexer.set_result_symbol(aux_sym_script_tag_token1); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 { state = 101; lexer.advance(false); continue; }
                return result;
            }
            102 => {
                result = true; lexer.set_result_symbol(anon_sym_LF); lexer.mark_end();
                if lookahead == 10 { state = 102; lexer.advance(false); continue; }
                return result;
            }
            103 => {
                result = true; lexer.set_result_symbol(anon_sym_SEMI); lexer.mark_end();
                return result;
            }
            104 => {
                result = true; lexer.set_result_symbol(anon_sym_as); lexer.mark_end();
                return result;
            }
            105 => {
                result = true; lexer.set_result_symbol(anon_sym_as); lexer.mark_end();
                if lookahead == 121 { state = 252; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            106 => {
                result = true; lexer.set_result_symbol(anon_sym_as); lexer.mark_end();
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            107 => {
                result = true; lexer.set_result_symbol(anon_sym_LPAREN); lexer.mark_end();
                return result;
            }
            108 => {
                result = true; lexer.set_result_symbol(anon_sym_RPAREN); lexer.mark_end();
                return result;
            }
            109 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ_EQ); lexer.mark_end();
                return result;
            }
            110 => {
                result = true; lexer.set_result_symbol(anon_sym_COMMA); lexer.mark_end();
                return result;
            }
            111 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                return result;
            }
            112 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                if lookahead == 46 { state = 241; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 145; lexer.advance(false); continue; }
                return result;
            }
            113 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                if lookahead == 46 { state = 240; lexer.advance(false); continue; }
                return result;
            }
            114 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                if lookahead == 46 { state = 240; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 145; lexer.advance(false); continue; }
                return result;
            }
            115 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                if lookahead == 46 { state = 48; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 145; lexer.advance(false); continue; }
                return result;
            }
            116 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                if lookahead == 46 { state = 51; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 145; lexer.advance(false); continue; }
                return result;
            }
            117 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 { state = 145; lexer.advance(false); continue; }
                return result;
            }
            118 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                return result;
            }
            119 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK); lexer.mark_end();
                return result;
            }
            120 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK); lexer.mark_end();
                if lookahead == 46 { state = 198; lexer.advance(false); continue; }
                return result;
            }
            121 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK); lexer.mark_end();
                if lookahead == 46 { state = 198; lexer.advance(false); continue; }
                if lookahead == 63 { state = 200; lexer.advance(false); continue; }
                return result;
            }
            122 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK); lexer.mark_end();
                if lookahead == 46 { state = 198; lexer.advance(false); continue; }
                if lookahead == 63 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            123 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK); lexer.mark_end();
                if lookahead == 46 { state = 198; lexer.advance(false); continue; }
                if lookahead == 63 { state = 71; lexer.advance(false); continue; }
                return result;
            }
            124 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK); lexer.mark_end();
                if lookahead == 46 { state = 197; lexer.advance(false); continue; }
                return result;
            }
            125 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK); lexer.mark_end();
                if lookahead == 46 { state = 197; lexer.advance(false); continue; }
                if lookahead == 63 { state = 200; lexer.advance(false); continue; }
                return result;
            }
            126 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK); lexer.mark_end();
                if lookahead == 46 { state = 197; lexer.advance(false); continue; }
                if lookahead == 63 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            127 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK); lexer.mark_end();
                if lookahead == 46 { state = 49; lexer.advance(false); continue; }
                if lookahead == 63 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            128 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK); lexer.mark_end();
                if lookahead == 63 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            129 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK); lexer.mark_end();
                return result;
            }
            130 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACK); lexer.mark_end();
                return result;
            }
            131 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACE); lexer.mark_end();
                return result;
            }
            132 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACE); lexer.mark_end();
                return result;
            }
            133 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                return result;
            }
            134 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 60 { state = 217; lexer.advance(false); continue; }
                if lookahead == 61 { state = 204; lexer.advance(false); continue; }
                return result;
            }
            135 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 60 { state = 216; lexer.advance(false); continue; }
                if lookahead == 61 { state = 204; lexer.advance(false); continue; }
                return result;
            }
            136 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 60 { state = 69; lexer.advance(false); continue; }
                return result;
            }
            137 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 61 { state = 204; lexer.advance(false); continue; }
                return result;
            }
            138 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                return result;
            }
            139 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                if lookahead == 61 { state = 205; lexer.advance(false); continue; }
                return result;
            }
            140 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                if lookahead == 61 { state = 205; lexer.advance(false); continue; }
                if lookahead == 62 { state = 220; lexer.advance(false); continue; }
                return result;
            }
            141 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                if lookahead == 61 { state = 205; lexer.advance(false); continue; }
                if lookahead == 62 { state = 221; lexer.advance(false); continue; }
                return result;
            }
            142 => {
                result = true; lexer.set_result_symbol(sym_decimal_integer_literal); lexer.mark_end();
                if lookahead == 46 { state = 87; lexer.advance(false); continue; }
                if lookahead == 95 { state = 77; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 86; lexer.advance(false); continue; }
                if lookahead == 88 || lookahead == 120 { state = 90; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 143; lexer.advance(false); continue; }
                return result;
            }
            143 => {
                result = true; lexer.set_result_symbol(sym_decimal_integer_literal); lexer.mark_end();
                if lookahead == 46 { state = 87; lexer.advance(false); continue; }
                if lookahead == 95 { state = 77; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 86; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 143; lexer.advance(false); continue; }
                return result;
            }
            144 => {
                result = true; lexer.set_result_symbol(sym_hex_integer_literal); lexer.mark_end();
                if lookahead == 95 { state = 78; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 144; lexer.advance(false); continue; }
                return result;
            }
            145 => {
                result = true; lexer.set_result_symbol(sym_decimal_floating_point_literal); lexer.mark_end();
                if lookahead == 95 { state = 79; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 86; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 145; lexer.advance(false); continue; }
                return result;
            }
            146 => {
                result = true; lexer.set_result_symbol(sym_decimal_floating_point_literal); lexer.mark_end();
                if lookahead == 95 { state = 80; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 146; lexer.advance(false); continue; }
                return result;
            }
            147 => {
                result = true; lexer.set_result_symbol(anon_sym_DQUOTE); lexer.mark_end();
                return result;
            }
            148 => {
                result = true; lexer.set_result_symbol(anon_sym_DQUOTE); lexer.mark_end();
                if lookahead == 34 { state = 31; lexer.advance(false); continue; }
                return result;
            }
            149 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTE); lexer.mark_end();
                return result;
            }
            150 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTE); lexer.mark_end();
                if lookahead == 39 { state = 44; lexer.advance(false); continue; }
                return result;
            }
            151 => {
                result = true; lexer.set_result_symbol(anon_sym_DQUOTE_DQUOTE_DQUOTE); lexer.mark_end();
                return result;
            }
            152 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTE_SQUOTE_SQUOTE); lexer.mark_end();
                return result;
            }
            153 => {
                result = true; lexer.set_result_symbol(anon_sym_r_DQUOTE); lexer.mark_end();
                if lookahead == 34 { state = 32; lexer.advance(false); continue; }
                return result;
            }
            154 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR); lexer.mark_end();
                return result;
            }
            155 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR); lexer.mark_end();
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            156 => {
                result = true; lexer.set_result_symbol(anon_sym_r_SQUOTE); lexer.mark_end();
                if lookahead == 39 { state = 45; lexer.advance(false); continue; }
                return result;
            }
            157 => {
                result = true; lexer.set_result_symbol(anon_sym_r_DQUOTE_DQUOTE_DQUOTE); lexer.mark_end();
                return result;
            }
            158 => {
                result = true; lexer.set_result_symbol(anon_sym_r_SQUOTE_SQUOTE_SQUOTE); lexer.mark_end();
                return result;
            }
            159 => {
                result = true; lexer.set_result_symbol(aux_sym__sub_string_test_token1); lexer.mark_end();
                return result;
            }
            160 => {
                result = true; lexer.set_result_symbol(aux_sym__sub_string_test_token1); lexer.mark_end();
                if lookahead == 34 { state = 148; lexer.advance(false); continue; }
                if lookahead == 36 { state = 154; lexer.advance(false); continue; }
                if lookahead == 39 { state = 149; lexer.advance(false); continue; }
                if lookahead == 47 { state = 164; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 160; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 65 || 90 < lookahead) && lookahead != 95 && (lookahead < 97 || 123 < lookahead) { state = 159; lexer.advance(false); continue; }
                return result;
            }
            161 => {
                result = true; lexer.set_result_symbol(aux_sym__sub_string_test_token1); lexer.mark_end();
                if lookahead == 34 { state = 147; lexer.advance(false); continue; }
                if lookahead == 36 { state = 154; lexer.advance(false); continue; }
                if lookahead == 39 { state = 150; lexer.advance(false); continue; }
                if lookahead == 47 { state = 164; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 161; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 65 || 90 < lookahead) && lookahead != 95 && (lookahead < 97 || 123 < lookahead) { state = 159; lexer.advance(false); continue; }
                return result;
            }
            162 => {
                result = true; lexer.set_result_symbol(aux_sym__sub_string_test_token1); lexer.mark_end();
                if lookahead == 34 { state = 147; lexer.advance(false); continue; }
                if lookahead == 36 { state = 154; lexer.advance(false); continue; }
                if lookahead == 39 { state = 149; lexer.advance(false); continue; }
                if lookahead == 47 { state = 164; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 162; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 65 || 90 < lookahead) && lookahead != 95 && (lookahead < 97 || 123 < lookahead) { state = 159; lexer.advance(false); continue; }
                return result;
            }
            163 => {
                result = true; lexer.set_result_symbol(aux_sym__sub_string_test_token1); lexer.mark_end();
                if lookahead == 36 { state = 167; lexer.advance(false); continue; }
                return result;
            }
            164 => {
                result = true; lexer.set_result_symbol(aux_sym__sub_string_test_token1); lexer.mark_end();
                if lookahead == 47 { state = 259; lexer.advance(false); continue; }
                return result;
            }
            165 => {
                result = true; lexer.set_result_symbol(aux_sym__sub_string_test_token1); lexer.mark_end();
                if lookahead == 47 { state = 164; lexer.advance(false); continue; }
                if lookahead == 92 { state = 163; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 165; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 65 || 90 < lookahead) && lookahead != 95 && (lookahead < 97 || 123 < lookahead) { state = 159; lexer.advance(false); continue; }
                return result;
            }
            166 => {
                result = true; lexer.set_result_symbol(aux_sym__sub_string_test_token1); lexer.mark_end();
                if lookahead == 117 { state = 83; lexer.advance(false); continue; }
                if lookahead == 120 { state = 93; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 55 { state = 170; lexer.advance(false); continue; }
                if lookahead != 0 { state = 168; lexer.advance(false); continue; }
                return result;
            }
            167 => {
                result = true; lexer.set_result_symbol(sym_identifier_dollar_escaped); lexer.mark_end();
                if lookahead == 92 { state = 39; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 167; lexer.advance(false); continue; }
                return result;
            }
            168 => {
                result = true; lexer.set_result_symbol(sym__unused_escape_sequence); lexer.mark_end();
                return result;
            }
            169 => {
                result = true; lexer.set_result_symbol(sym__unused_escape_sequence); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 55 { state = 168; lexer.advance(false); continue; }
                return result;
            }
            170 => {
                result = true; lexer.set_result_symbol(sym__unused_escape_sequence); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 55 { state = 169; lexer.advance(false); continue; }
                return result;
            }
            171 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND); lexer.mark_end();
                return result;
            }
            172 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND); lexer.mark_end();
                if lookahead == 33 { state = 98; lexer.advance(false); continue; }
                return result;
            }
            173 => {
                result = true; lexer.set_result_symbol(anon_sym_TILDE); lexer.mark_end();
                return result;
            }
            174 => {
                result = true; lexer.set_result_symbol(anon_sym_TILDE); lexer.mark_end();
                if lookahead == 47 { state = 236; lexer.advance(false); continue; }
                return result;
            }
            175 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK_RBRACK); lexer.mark_end();
                if lookahead == 61 { state = 176; lexer.advance(false); continue; }
                return result;
            }
            176 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK_RBRACK_EQ); lexer.mark_end();
                return result;
            }
            177 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON); lexer.mark_end();
                return result;
            }
            178 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT_DOT_DOT); lexer.mark_end();
                return result;
            }
            179 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT_DOT_DOT); lexer.mark_end();
                if lookahead == 63 { state = 180; lexer.advance(false); continue; }
                return result;
            }
            180 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT_DOT_DOT_QMARK); lexer.mark_end();
                return result;
            }
            181 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                return result;
            }
            182 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                if lookahead == 61 { state = 109; lexer.advance(false); continue; }
                if lookahead == 62 { state = 244; lexer.advance(false); continue; }
                return result;
            }
            183 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                if lookahead == 62 { state = 244; lexer.advance(false); continue; }
                return result;
            }
            184 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS_EQ); lexer.mark_end();
                return result;
            }
            185 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_EQ); lexer.mark_end();
                return result;
            }
            186 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR_EQ); lexer.mark_end();
                return result;
            }
            187 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_EQ); lexer.mark_end();
                return result;
            }
            188 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT_EQ); lexer.mark_end();
                return result;
            }
            189 => {
                result = true; lexer.set_result_symbol(anon_sym_TILDE_SLASH_EQ); lexer.mark_end();
                return result;
            }
            190 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT_EQ); lexer.mark_end();
                return result;
            }
            191 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT_EQ); lexer.mark_end();
                return result;
            }
            192 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT_GT_EQ); lexer.mark_end();
                return result;
            }
            193 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP_EQ); lexer.mark_end();
                return result;
            }
            194 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET_EQ); lexer.mark_end();
                return result;
            }
            195 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE_EQ); lexer.mark_end();
                return result;
            }
            196 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK_QMARK_EQ); lexer.mark_end();
                return result;
            }
            197 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK_DOT); lexer.mark_end();
                return result;
            }
            198 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK_DOT); lexer.mark_end();
                if lookahead == 46 { state = 242; lexer.advance(false); continue; }
                return result;
            }
            199 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK_QMARK); lexer.mark_end();
                return result;
            }
            200 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK_QMARK); lexer.mark_end();
                if lookahead == 61 { state = 196; lexer.advance(false); continue; }
                return result;
            }
            201 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE_PIPE); lexer.mark_end();
                return result;
            }
            202 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP_AMP); lexer.mark_end();
                return result;
            }
            203 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG_EQ); lexer.mark_end();
                return result;
            }
            204 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_EQ); lexer.mark_end();
                return result;
            }
            205 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_EQ); lexer.mark_end();
                return result;
            }
            206 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG); lexer.mark_end();
                return result;
            }
            207 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG); lexer.mark_end();
                if lookahead == 61 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            208 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                return result;
            }
            209 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                if lookahead == 61 { state = 195; lexer.advance(false); continue; }
                if lookahead == 124 { state = 201; lexer.advance(false); continue; }
                return result;
            }
            210 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                if lookahead == 124 { state = 201; lexer.advance(false); continue; }
                return result;
            }
            211 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET); lexer.mark_end();
                return result;
            }
            212 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET); lexer.mark_end();
                if lookahead == 61 { state = 194; lexer.advance(false); continue; }
                return result;
            }
            213 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP); lexer.mark_end();
                return result;
            }
            214 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP); lexer.mark_end();
                if lookahead == 38 { state = 202; lexer.advance(false); continue; }
                return result;
            }
            215 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP); lexer.mark_end();
                if lookahead == 38 { state = 202; lexer.advance(false); continue; }
                if lookahead == 61 { state = 193; lexer.advance(false); continue; }
                return result;
            }
            216 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT); lexer.mark_end();
                return result;
            }
            217 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT); lexer.mark_end();
                if lookahead == 61 { state = 190; lexer.advance(false); continue; }
                return result;
            }
            218 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT_GT); lexer.mark_end();
                return result;
            }
            219 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT_GT); lexer.mark_end();
                if lookahead == 61 { state = 192; lexer.advance(false); continue; }
                return result;
            }
            220 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT); lexer.mark_end();
                if lookahead == 61 { state = 191; lexer.advance(false); continue; }
                if lookahead == 62 { state = 219; lexer.advance(false); continue; }
                return result;
            }
            221 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT); lexer.mark_end();
                if lookahead == 62 { state = 218; lexer.advance(false); continue; }
                return result;
            }
            222 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                return result;
            }
            223 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                if lookahead == 43 { state = 238; lexer.advance(false); continue; }
                return result;
            }
            224 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                if lookahead == 43 { state = 238; lexer.advance(false); continue; }
                if lookahead == 61 { state = 184; lexer.advance(false); continue; }
                return result;
            }
            225 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                if lookahead == 61 { state = 184; lexer.advance(false); continue; }
                return result;
            }
            226 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                return result;
            }
            227 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 239; lexer.advance(false); continue; }
                return result;
            }
            228 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 239; lexer.advance(false); continue; }
                if lookahead == 61 { state = 185; lexer.advance(false); continue; }
                return result;
            }
            229 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 61 { state = 185; lexer.advance(false); continue; }
                return result;
            }
            230 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                return result;
            }
            231 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                if lookahead == 61 { state = 186; lexer.advance(false); continue; }
                return result;
            }
            232 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH); lexer.mark_end();
                if lookahead == 47 { state = 259; lexer.advance(false); continue; }
                return result;
            }
            233 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH); lexer.mark_end();
                if lookahead == 47 { state = 259; lexer.advance(false); continue; }
                if lookahead == 61 { state = 187; lexer.advance(false); continue; }
                return result;
            }
            234 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT); lexer.mark_end();
                return result;
            }
            235 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT); lexer.mark_end();
                if lookahead == 61 { state = 188; lexer.advance(false); continue; }
                return result;
            }
            236 => {
                result = true; lexer.set_result_symbol(anon_sym_TILDE_SLASH); lexer.mark_end();
                return result;
            }
            237 => {
                result = true; lexer.set_result_symbol(anon_sym_TILDE_SLASH); lexer.mark_end();
                if lookahead == 61 { state = 189; lexer.advance(false); continue; }
                return result;
            }
            238 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS_PLUS); lexer.mark_end();
                return result;
            }
            239 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_DASH); lexer.mark_end();
                return result;
            }
            240 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT_DOT); lexer.mark_end();
                return result;
            }
            241 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT_DOT); lexer.mark_end();
                if lookahead == 46 { state = 179; lexer.advance(false); continue; }
                return result;
            }
            242 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK_DOT_DOT); lexer.mark_end();
                return result;
            }
            243 => {
                result = true; lexer.set_result_symbol(anon_sym_async); lexer.mark_end();
                if lookahead == 42 { state = 245; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            244 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ_GT); lexer.mark_end();
                return result;
            }
            245 => {
                result = true; lexer.set_result_symbol(anon_sym_async_STAR); lexer.mark_end();
                return result;
            }
            246 => {
                result = true; lexer.set_result_symbol(anon_sym_sync_STAR); lexer.mark_end();
                return result;
            }
            247 => {
                result = true; lexer.set_result_symbol(sym__name); lexer.mark_end();
                if lookahead == 34 { state = 153; lexer.advance(false); continue; }
                if lookahead == 39 { state = 156; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            248 => {
                result = true; lexer.set_result_symbol(sym__name); lexer.mark_end();
                if lookahead == 42 { state = 246; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            249 => {
                result = true; lexer.set_result_symbol(sym__name); lexer.mark_end();
                if lookahead == 99 { state = 248; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            250 => {
                result = true; lexer.set_result_symbol(sym__name); lexer.mark_end();
                if lookahead == 99 { state = 243; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            251 => {
                result = true; lexer.set_result_symbol(sym__name); lexer.mark_end();
                if lookahead == 110 { state = 249; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            252 => {
                result = true; lexer.set_result_symbol(sym__name); lexer.mark_end();
                if lookahead == 110 { state = 250; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            253 => {
                result = true; lexer.set_result_symbol(sym__name); lexer.mark_end();
                if lookahead == 115 { state = 105; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            254 => {
                result = true; lexer.set_result_symbol(sym__name); lexer.mark_end();
                if lookahead == 115 { state = 106; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            255 => {
                result = true; lexer.set_result_symbol(sym__name); lexer.mark_end();
                if lookahead == 115 { state = 257; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            256 => {
                result = true; lexer.set_result_symbol(sym__name); lexer.mark_end();
                if lookahead == 121 { state = 251; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            257 => {
                result = true; lexer.set_result_symbol(sym__name); lexer.mark_end();
                if lookahead == 121 { state = 252; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            258 => {
                result = true; lexer.set_result_symbol(sym__name); lexer.mark_end();
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            259 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_SLASH); lexer.mark_end();
                if lookahead == 47 { state = 267; lexer.advance(false); continue; }
                return result;
            }
            260 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_SLASH); lexer.mark_end();
                if lookahead == 47 { state = 268; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 266; lexer.advance(false); continue; }
                return result;
            }
            261 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_SLASH); lexer.mark_end();
                if lookahead == 47 { state = 269; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 273; lexer.advance(false); continue; }
                return result;
            }
            262 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_SLASH); lexer.mark_end();
                if lookahead == 47 { state = 270; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 101; lexer.advance(false); continue; }
                return result;
            }
            263 => {
                result = true; lexer.set_result_symbol(aux_sym_comment_token1); lexer.mark_end();
                if lookahead == 47 { state = 264; lexer.advance(false); continue; }
                if lookahead == 9 || 11 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 263; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) { state = 266; lexer.advance(false); continue; }
                return result;
            }
            264 => {
                result = true; lexer.set_result_symbol(aux_sym_comment_token1); lexer.mark_end();
                if lookahead == 47 { state = 260; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 266; lexer.advance(false); continue; }
                return result;
            }
            265 => {
                result = true; lexer.set_result_symbol(aux_sym_comment_token1); lexer.mark_end();
                if lookahead == 9 || 11 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 263; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 47 { state = 266; lexer.advance(false); continue; }
                return result;
            }
            266 => {
                result = true; lexer.set_result_symbol(aux_sym_comment_token1); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 { state = 266; lexer.advance(false); continue; }
                return result;
            }
            267 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_SLASH_SLASH); lexer.mark_end();
                return result;
            }
            268 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_SLASH_SLASH); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 { state = 266; lexer.advance(false); continue; }
                return result;
            }
            269 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_SLASH_SLASH); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 { state = 273; lexer.advance(false); continue; }
                return result;
            }
            270 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_SLASH_SLASH); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 { state = 101; lexer.advance(false); continue; }
                return result;
            }
            271 => {
                result = true; lexer.set_result_symbol(aux_sym_comment_token2); lexer.mark_end();
                if lookahead == 47 { state = 272; lexer.advance(false); continue; }
                if lookahead == 9 || 11 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 271; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) { state = 273; lexer.advance(false); continue; }
                return result;
            }
            272 => {
                result = true; lexer.set_result_symbol(aux_sym_comment_token2); lexer.mark_end();
                if lookahead == 47 { state = 261; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 273; lexer.advance(false); continue; }
                return result;
            }
            273 => {
                result = true; lexer.set_result_symbol(aux_sym_comment_token2); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 { state = 273; lexer.advance(false); continue; }
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
                    (70, 1), (97, 2), (98, 3), (99, 4), (100, 5), (101, 6), (102, 7), (103, 8),
                    (104, 9), (105, 10), (108, 11), (109, 12), (110, 13), (111, 14), (112, 15), (114, 16),
                    (115, 17), (116, 18), (118, 19), (119, 20), (121, 21),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 0; lexer.advance(true); continue; }
                return result;
            }
            1 => {
                if lookahead == 117 { state = 22; lexer.advance(false); continue; }
                return result;
            }
            2 => {
                if lookahead == 98 { state = 23; lexer.advance(false); continue; }
                if lookahead == 115 { state = 24; lexer.advance(false); continue; }
                if lookahead == 117 { state = 25; lexer.advance(false); continue; }
                if lookahead == 119 { state = 26; lexer.advance(false); continue; }
                return result;
            }
            3 => {
                if lookahead == 97 { state = 27; lexer.advance(false); continue; }
                if lookahead == 114 { state = 28; lexer.advance(false); continue; }
                return result;
            }
            4 => {
                if lookahead == 97 { state = 29; lexer.advance(false); continue; }
                if lookahead == 108 { state = 30; lexer.advance(false); continue; }
                if lookahead == 111 { state = 31; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                if lookahead == 101 { state = 32; lexer.advance(false); continue; }
                if lookahead == 111 { state = 33; lexer.advance(false); continue; }
                if lookahead == 121 { state = 34; lexer.advance(false); continue; }
                return result;
            }
            6 => {
                if lookahead == 108 { state = 35; lexer.advance(false); continue; }
                if lookahead == 110 { state = 36; lexer.advance(false); continue; }
                if lookahead == 120 { state = 37; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if lookahead == 97 { state = 38; lexer.advance(false); continue; }
                if lookahead == 105 { state = 39; lexer.advance(false); continue; }
                if lookahead == 111 { state = 40; lexer.advance(false); continue; }
                return result;
            }
            8 => {
                if lookahead == 101 { state = 41; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if lookahead == 105 { state = 42; lexer.advance(false); continue; }
                return result;
            }
            10 => {
                if lookahead == 102 { state = 43; lexer.advance(false); continue; }
                if lookahead == 109 { state = 44; lexer.advance(false); continue; }
                if lookahead == 110 { state = 45; lexer.advance(false); continue; }
                if lookahead == 115 { state = 46; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if lookahead == 97 { state = 47; lexer.advance(false); continue; }
                if lookahead == 105 { state = 48; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if lookahead == 105 { state = 49; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if lookahead == 97 { state = 50; lexer.advance(false); continue; }
                if lookahead == 101 { state = 51; lexer.advance(false); continue; }
                if lookahead == 117 { state = 52; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if lookahead == 102 { state = 53; lexer.advance(false); continue; }
                if lookahead == 110 { state = 54; lexer.advance(false); continue; }
                if lookahead == 112 { state = 55; lexer.advance(false); continue; }
                if lookahead == 117 { state = 56; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if lookahead == 97 { state = 57; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if lookahead == 101 { state = 58; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if lookahead == 101 { state = 59; lexer.advance(false); continue; }
                if lookahead == 104 { state = 60; lexer.advance(false); continue; }
                if lookahead == 116 { state = 61; lexer.advance(false); continue; }
                if lookahead == 117 { state = 62; lexer.advance(false); continue; }
                if lookahead == 119 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            18 => {
                if lookahead == 104 { state = 64; lexer.advance(false); continue; }
                if lookahead == 114 { state = 65; lexer.advance(false); continue; }
                if lookahead == 121 { state = 66; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if lookahead == 97 { state = 67; lexer.advance(false); continue; }
                if lookahead == 111 { state = 68; lexer.advance(false); continue; }
                return result;
            }
            20 => {
                if lookahead == 104 { state = 69; lexer.advance(false); continue; }
                if lookahead == 105 { state = 70; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                if lookahead == 105 { state = 71; lexer.advance(false); continue; }
                return result;
            }
            22 => {
                if lookahead == 110 { state = 72; lexer.advance(false); continue; }
                return result;
            }
            23 => {
                if lookahead == 115 { state = 73; lexer.advance(false); continue; }
                return result;
            }
            24 => {
                if lookahead == 115 { state = 74; lexer.advance(false); continue; }
                return result;
            }
            25 => {
                if lookahead == 103 { state = 75; lexer.advance(false); continue; }
                return result;
            }
            26 => {
                if lookahead == 97 { state = 76; lexer.advance(false); continue; }
                return result;
            }
            27 => {
                if lookahead == 115 { state = 77; lexer.advance(false); continue; }
                return result;
            }
            28 => {
                if lookahead == 101 { state = 78; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                if lookahead == 115 { state = 79; lexer.advance(false); continue; }
                if lookahead == 116 { state = 80; lexer.advance(false); continue; }
                return result;
            }
            30 => {
                if lookahead == 97 { state = 81; lexer.advance(false); continue; }
                return result;
            }
            31 => {
                if lookahead == 110 { state = 82; lexer.advance(false); continue; }
                if lookahead == 118 { state = 83; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                if lookahead == 102 { state = 84; lexer.advance(false); continue; }
                return result;
            }
            33 => {
                result = true; lexer.set_result_symbol(anon_sym_do); lexer.mark_end();
                return result;
            }
            34 => {
                if lookahead == 110 { state = 85; lexer.advance(false); continue; }
                return result;
            }
            35 => {
                if lookahead == 115 { state = 86; lexer.advance(false); continue; }
                return result;
            }
            36 => {
                if lookahead == 117 { state = 87; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                if lookahead == 112 { state = 88; lexer.advance(false); continue; }
                if lookahead == 116 { state = 89; lexer.advance(false); continue; }
                return result;
            }
            38 => {
                if lookahead == 99 { state = 90; lexer.advance(false); continue; }
                if lookahead == 108 { state = 91; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                if lookahead == 110 { state = 92; lexer.advance(false); continue; }
                return result;
            }
            40 => {
                if lookahead == 114 { state = 93; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                if lookahead == 116 { state = 94; lexer.advance(false); continue; }
                return result;
            }
            42 => {
                if lookahead == 100 { state = 95; lexer.advance(false); continue; }
                return result;
            }
            43 => {
                result = true; lexer.set_result_symbol(anon_sym_if); lexer.mark_end();
                return result;
            }
            44 => {
                if lookahead == 112 { state = 96; lexer.advance(false); continue; }
                return result;
            }
            45 => {
                result = true; lexer.set_result_symbol(anon_sym_in); lexer.mark_end();
                if lookahead == 108 { state = 97; lexer.advance(false); continue; }
                if lookahead == 111 { state = 98; lexer.advance(false); continue; }
                if lookahead == 116 { state = 99; lexer.advance(false); continue; }
                return result;
            }
            46 => {
                result = true; lexer.set_result_symbol(anon_sym_is); lexer.mark_end();
                return result;
            }
            47 => {
                if lookahead == 116 { state = 100; lexer.advance(false); continue; }
                return result;
            }
            48 => {
                if lookahead == 98 { state = 101; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                if lookahead == 120 { state = 102; lexer.advance(false); continue; }
                return result;
            }
            50 => {
                if lookahead == 116 { state = 103; lexer.advance(false); continue; }
                return result;
            }
            51 => {
                if lookahead == 119 { state = 104; lexer.advance(false); continue; }
                return result;
            }
            52 => {
                if lookahead == 108 { state = 105; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                result = true; lexer.set_result_symbol(anon_sym_of); lexer.mark_end();
                return result;
            }
            54 => {
                result = true; lexer.set_result_symbol(anon_sym_on); lexer.mark_end();
                return result;
            }
            55 => {
                if lookahead == 101 { state = 106; lexer.advance(false); continue; }
                return result;
            }
            56 => {
                if lookahead == 116 { state = 107; lexer.advance(false); continue; }
                return result;
            }
            57 => {
                if lookahead == 114 { state = 108; lexer.advance(false); continue; }
                return result;
            }
            58 => {
                if lookahead == 113 { state = 109; lexer.advance(false); continue; }
                if lookahead == 116 { state = 110; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                if lookahead == 97 { state = 111; lexer.advance(false); continue; }
                if lookahead == 116 { state = 112; lexer.advance(false); continue; }
                return result;
            }
            60 => {
                if lookahead == 111 { state = 113; lexer.advance(false); continue; }
                return result;
            }
            61 => {
                if lookahead == 97 { state = 114; lexer.advance(false); continue; }
                return result;
            }
            62 => {
                if lookahead == 112 { state = 115; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                if lookahead == 105 { state = 116; lexer.advance(false); continue; }
                return result;
            }
            64 => {
                if lookahead == 105 { state = 117; lexer.advance(false); continue; }
                if lookahead == 114 { state = 118; lexer.advance(false); continue; }
                return result;
            }
            65 => {
                if lookahead == 117 { state = 119; lexer.advance(false); continue; }
                if lookahead == 121 { state = 120; lexer.advance(false); continue; }
                return result;
            }
            66 => {
                if lookahead == 112 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            67 => {
                if lookahead == 114 { state = 122; lexer.advance(false); continue; }
                return result;
            }
            68 => {
                if lookahead == 105 { state = 123; lexer.advance(false); continue; }
                return result;
            }
            69 => {
                if lookahead == 101 { state = 124; lexer.advance(false); continue; }
                if lookahead == 105 { state = 125; lexer.advance(false); continue; }
                return result;
            }
            70 => {
                if lookahead == 116 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            71 => {
                if lookahead == 101 { state = 127; lexer.advance(false); continue; }
                return result;
            }
            72 => {
                if lookahead == 99 { state = 128; lexer.advance(false); continue; }
                return result;
            }
            73 => {
                if lookahead == 116 { state = 129; lexer.advance(false); continue; }
                return result;
            }
            74 => {
                if lookahead == 101 { state = 130; lexer.advance(false); continue; }
                return result;
            }
            75 => {
                if lookahead == 109 { state = 131; lexer.advance(false); continue; }
                return result;
            }
            76 => {
                if lookahead == 105 { state = 132; lexer.advance(false); continue; }
                return result;
            }
            77 => {
                if lookahead == 101 { state = 133; lexer.advance(false); continue; }
                return result;
            }
            78 => {
                if lookahead == 97 { state = 134; lexer.advance(false); continue; }
                return result;
            }
            79 => {
                if lookahead == 101 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            80 => {
                if lookahead == 99 { state = 136; lexer.advance(false); continue; }
                return result;
            }
            81 => {
                if lookahead == 115 { state = 137; lexer.advance(false); continue; }
                return result;
            }
            82 => {
                if lookahead == 115 { state = 138; lexer.advance(false); continue; }
                if lookahead == 116 { state = 139; lexer.advance(false); continue; }
                return result;
            }
            83 => {
                if lookahead == 97 { state = 140; lexer.advance(false); continue; }
                return result;
            }
            84 => {
                if lookahead == 97 { state = 141; lexer.advance(false); continue; }
                if lookahead == 101 { state = 142; lexer.advance(false); continue; }
                return result;
            }
            85 => {
                if lookahead == 97 { state = 143; lexer.advance(false); continue; }
                return result;
            }
            86 => {
                if lookahead == 101 { state = 144; lexer.advance(false); continue; }
                return result;
            }
            87 => {
                if lookahead == 109 { state = 145; lexer.advance(false); continue; }
                return result;
            }
            88 => {
                if lookahead == 111 { state = 146; lexer.advance(false); continue; }
                return result;
            }
            89 => {
                if lookahead == 101 { state = 147; lexer.advance(false); continue; }
                return result;
            }
            90 => {
                if lookahead == 116 { state = 148; lexer.advance(false); continue; }
                return result;
            }
            91 => {
                if lookahead == 115 { state = 149; lexer.advance(false); continue; }
                return result;
            }
            92 => {
                if lookahead == 97 { state = 150; lexer.advance(false); continue; }
                return result;
            }
            93 => {
                result = true; lexer.set_result_symbol(anon_sym_for); lexer.mark_end();
                return result;
            }
            94 => {
                result = true; lexer.set_result_symbol(anon_sym_get); lexer.mark_end();
                return result;
            }
            95 => {
                if lookahead == 101 { state = 151; lexer.advance(false); continue; }
                return result;
            }
            96 => {
                if lookahead == 108 { state = 152; lexer.advance(false); continue; }
                if lookahead == 111 { state = 153; lexer.advance(false); continue; }
                return result;
            }
            97 => {
                if lookahead == 105 { state = 154; lexer.advance(false); continue; }
                return result;
            }
            98 => {
                if lookahead == 117 { state = 155; lexer.advance(false); continue; }
                return result;
            }
            99 => {
                if lookahead == 101 { state = 156; lexer.advance(false); continue; }
                return result;
            }
            100 => {
                if lookahead == 101 { state = 157; lexer.advance(false); continue; }
                return result;
            }
            101 => {
                if lookahead == 114 { state = 158; lexer.advance(false); continue; }
                return result;
            }
            102 => {
                if lookahead == 105 { state = 159; lexer.advance(false); continue; }
                return result;
            }
            103 => {
                if lookahead == 105 { state = 160; lexer.advance(false); continue; }
                return result;
            }
            104 => {
                result = true; lexer.set_result_symbol(anon_sym_new); lexer.mark_end();
                return result;
            }
            105 => {
                if lookahead == 108 { state = 161; lexer.advance(false); continue; }
                return result;
            }
            106 => {
                if lookahead == 114 { state = 162; lexer.advance(false); continue; }
                return result;
            }
            107 => {
                result = true; lexer.set_result_symbol(anon_sym_out); lexer.mark_end();
                return result;
            }
            108 => {
                if lookahead == 116 { state = 163; lexer.advance(false); continue; }
                return result;
            }
            109 => {
                if lookahead == 117 { state = 164; lexer.advance(false); continue; }
                return result;
            }
            110 => {
                if lookahead == 104 { state = 165; lexer.advance(false); continue; }
                if lookahead == 117 { state = 166; lexer.advance(false); continue; }
                return result;
            }
            111 => {
                if lookahead == 108 { state = 167; lexer.advance(false); continue; }
                return result;
            }
            112 => {
                result = true; lexer.set_result_symbol(anon_sym_set); lexer.mark_end();
                return result;
            }
            113 => {
                if lookahead == 119 { state = 168; lexer.advance(false); continue; }
                return result;
            }
            114 => {
                if lookahead == 116 { state = 169; lexer.advance(false); continue; }
                return result;
            }
            115 => {
                if lookahead == 101 { state = 170; lexer.advance(false); continue; }
                return result;
            }
            116 => {
                if lookahead == 116 { state = 171; lexer.advance(false); continue; }
                return result;
            }
            117 => {
                if lookahead == 115 { state = 172; lexer.advance(false); continue; }
                return result;
            }
            118 => {
                if lookahead == 111 { state = 173; lexer.advance(false); continue; }
                return result;
            }
            119 => {
                if lookahead == 101 { state = 174; lexer.advance(false); continue; }
                return result;
            }
            120 => {
                result = true; lexer.set_result_symbol(anon_sym_try); lexer.mark_end();
                return result;
            }
            121 => {
                if lookahead == 101 { state = 175; lexer.advance(false); continue; }
                return result;
            }
            122 => {
                result = true; lexer.set_result_symbol(anon_sym_var); lexer.mark_end();
                return result;
            }
            123 => {
                if lookahead == 100 { state = 176; lexer.advance(false); continue; }
                return result;
            }
            124 => {
                if lookahead == 110 { state = 177; lexer.advance(false); continue; }
                return result;
            }
            125 => {
                if lookahead == 108 { state = 178; lexer.advance(false); continue; }
                return result;
            }
            126 => {
                if lookahead == 104 { state = 179; lexer.advance(false); continue; }
                return result;
            }
            127 => {
                if lookahead == 108 { state = 180; lexer.advance(false); continue; }
                return result;
            }
            128 => {
                if lookahead == 116 { state = 181; lexer.advance(false); continue; }
                return result;
            }
            129 => {
                if lookahead == 114 { state = 182; lexer.advance(false); continue; }
                return result;
            }
            130 => {
                if lookahead == 114 { state = 183; lexer.advance(false); continue; }
                return result;
            }
            131 => {
                if lookahead == 101 { state = 184; lexer.advance(false); continue; }
                return result;
            }
            132 => {
                if lookahead == 116 { state = 185; lexer.advance(false); continue; }
                return result;
            }
            133 => {
                result = true; lexer.set_result_symbol(anon_sym_base); lexer.mark_end();
                return result;
            }
            134 => {
                if lookahead == 107 { state = 186; lexer.advance(false); continue; }
                return result;
            }
            135 => {
                result = true; lexer.set_result_symbol(anon_sym_case); lexer.mark_end();
                return result;
            }
            136 => {
                if lookahead == 104 { state = 187; lexer.advance(false); continue; }
                return result;
            }
            137 => {
                if lookahead == 115 { state = 188; lexer.advance(false); continue; }
                return result;
            }
            138 => {
                if lookahead == 116 { state = 189; lexer.advance(false); continue; }
                return result;
            }
            139 => {
                if lookahead == 105 { state = 190; lexer.advance(false); continue; }
                return result;
            }
            140 => {
                if lookahead == 114 { state = 191; lexer.advance(false); continue; }
                return result;
            }
            141 => {
                if lookahead == 117 { state = 192; lexer.advance(false); continue; }
                return result;
            }
            142 => {
                if lookahead == 114 { state = 193; lexer.advance(false); continue; }
                return result;
            }
            143 => {
                if lookahead == 109 { state = 194; lexer.advance(false); continue; }
                return result;
            }
            144 => {
                result = true; lexer.set_result_symbol(anon_sym_else); lexer.mark_end();
                return result;
            }
            145 => {
                result = true; lexer.set_result_symbol(anon_sym_enum); lexer.mark_end();
                return result;
            }
            146 => {
                if lookahead == 114 { state = 195; lexer.advance(false); continue; }
                return result;
            }
            147 => {
                if lookahead == 110 { state = 196; lexer.advance(false); continue; }
                if lookahead == 114 { state = 197; lexer.advance(false); continue; }
                return result;
            }
            148 => {
                if lookahead == 111 { state = 198; lexer.advance(false); continue; }
                return result;
            }
            149 => {
                if lookahead == 101 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            150 => {
                if lookahead == 108 { state = 200; lexer.advance(false); continue; }
                return result;
            }
            151 => {
                result = true; lexer.set_result_symbol(anon_sym_hide); lexer.mark_end();
                return result;
            }
            152 => {
                if lookahead == 101 { state = 201; lexer.advance(false); continue; }
                return result;
            }
            153 => {
                if lookahead == 114 { state = 202; lexer.advance(false); continue; }
                return result;
            }
            154 => {
                if lookahead == 110 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            155 => {
                if lookahead == 116 { state = 204; lexer.advance(false); continue; }
                return result;
            }
            156 => {
                if lookahead == 114 { state = 205; lexer.advance(false); continue; }
                return result;
            }
            157 => {
                result = true; lexer.set_result_symbol(anon_sym_late); lexer.mark_end();
                return result;
            }
            158 => {
                if lookahead == 97 { state = 206; lexer.advance(false); continue; }
                return result;
            }
            159 => {
                if lookahead == 110 { state = 207; lexer.advance(false); continue; }
                return result;
            }
            160 => {
                if lookahead == 118 { state = 208; lexer.advance(false); continue; }
                return result;
            }
            161 => {
                result = true; lexer.set_result_symbol(sym_null_literal); lexer.mark_end();
                return result;
            }
            162 => {
                if lookahead == 97 { state = 209; lexer.advance(false); continue; }
                return result;
            }
            163 => {
                result = true; lexer.set_result_symbol(anon_sym_part); lexer.mark_end();
                return result;
            }
            164 => {
                if lookahead == 105 { state = 210; lexer.advance(false); continue; }
                return result;
            }
            165 => {
                if lookahead == 114 { state = 211; lexer.advance(false); continue; }
                return result;
            }
            166 => {
                if lookahead == 114 { state = 212; lexer.advance(false); continue; }
                return result;
            }
            167 => {
                if lookahead == 101 { state = 213; lexer.advance(false); continue; }
                return result;
            }
            168 => {
                result = true; lexer.set_result_symbol(anon_sym_show); lexer.mark_end();
                return result;
            }
            169 => {
                if lookahead == 105 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            170 => {
                if lookahead == 114 { state = 215; lexer.advance(false); continue; }
                return result;
            }
            171 => {
                if lookahead == 99 { state = 216; lexer.advance(false); continue; }
                return result;
            }
            172 => {
                result = true; lexer.set_result_symbol(anon_sym_this); lexer.mark_end();
                return result;
            }
            173 => {
                if lookahead == 119 { state = 217; lexer.advance(false); continue; }
                return result;
            }
            174 => {
                result = true; lexer.set_result_symbol(sym_true); lexer.mark_end();
                return result;
            }
            175 => {
                result = true; lexer.set_result_symbol(anon_sym_type); lexer.mark_end();
                if lookahead == 100 { state = 218; lexer.advance(false); continue; }
                return result;
            }
            176 => {
                result = true; lexer.set_result_symbol(sym_void_type); lexer.mark_end();
                return result;
            }
            177 => {
                result = true; lexer.set_result_symbol(anon_sym_when); lexer.mark_end();
                return result;
            }
            178 => {
                if lookahead == 101 { state = 219; lexer.advance(false); continue; }
                return result;
            }
            179 => {
                result = true; lexer.set_result_symbol(anon_sym_with); lexer.mark_end();
                return result;
            }
            180 => {
                if lookahead == 100 { state = 220; lexer.advance(false); continue; }
                return result;
            }
            181 => {
                if lookahead == 105 { state = 221; lexer.advance(false); continue; }
                return result;
            }
            182 => {
                if lookahead == 97 { state = 222; lexer.advance(false); continue; }
                return result;
            }
            183 => {
                if lookahead == 116 { state = 223; lexer.advance(false); continue; }
                return result;
            }
            184 => {
                if lookahead == 110 { state = 224; lexer.advance(false); continue; }
                return result;
            }
            185 => {
                result = true; lexer.set_result_symbol(anon_sym_await); lexer.mark_end();
                return result;
            }
            186 => {
                result = true; lexer.set_result_symbol(anon_sym_break); lexer.mark_end();
                return result;
            }
            187 => {
                result = true; lexer.set_result_symbol(anon_sym_catch); lexer.mark_end();
                return result;
            }
            188 => {
                result = true; lexer.set_result_symbol(anon_sym_class); lexer.mark_end();
                return result;
            }
            189 => {
                result = true; lexer.set_result_symbol(anon_sym_const); lexer.mark_end();
                return result;
            }
            190 => {
                if lookahead == 110 { state = 225; lexer.advance(false); continue; }
                return result;
            }
            191 => {
                if lookahead == 105 { state = 226; lexer.advance(false); continue; }
                return result;
            }
            192 => {
                if lookahead == 108 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            193 => {
                if lookahead == 114 { state = 228; lexer.advance(false); continue; }
                return result;
            }
            194 => {
                if lookahead == 105 { state = 229; lexer.advance(false); continue; }
                return result;
            }
            195 => {
                if lookahead == 116 { state = 230; lexer.advance(false); continue; }
                return result;
            }
            196 => {
                if lookahead == 100 { state = 231; lexer.advance(false); continue; }
                if lookahead == 115 { state = 232; lexer.advance(false); continue; }
                return result;
            }
            197 => {
                if lookahead == 110 { state = 233; lexer.advance(false); continue; }
                return result;
            }
            198 => {
                if lookahead == 114 { state = 234; lexer.advance(false); continue; }
                return result;
            }
            199 => {
                result = true; lexer.set_result_symbol(sym_false); lexer.mark_end();
                return result;
            }
            200 => {
                result = true; lexer.set_result_symbol(anon_sym_final); lexer.mark_end();
                if lookahead == 108 { state = 235; lexer.advance(false); continue; }
                return result;
            }
            201 => {
                if lookahead == 109 { state = 236; lexer.advance(false); continue; }
                return result;
            }
            202 => {
                if lookahead == 116 { state = 237; lexer.advance(false); continue; }
                return result;
            }
            203 => {
                if lookahead == 101 { state = 238; lexer.advance(false); continue; }
                return result;
            }
            204 => {
                result = true; lexer.set_result_symbol(anon_sym_inout); lexer.mark_end();
                return result;
            }
            205 => {
                if lookahead == 102 { state = 239; lexer.advance(false); continue; }
                return result;
            }
            206 => {
                if lookahead == 114 { state = 240; lexer.advance(false); continue; }
                return result;
            }
            207 => {
                result = true; lexer.set_result_symbol(anon_sym_mixin); lexer.mark_end();
                return result;
            }
            208 => {
                if lookahead == 101 { state = 241; lexer.advance(false); continue; }
                return result;
            }
            209 => {
                if lookahead == 116 { state = 242; lexer.advance(false); continue; }
                return result;
            }
            210 => {
                if lookahead == 114 { state = 243; lexer.advance(false); continue; }
                return result;
            }
            211 => {
                if lookahead == 111 { state = 244; lexer.advance(false); continue; }
                return result;
            }
            212 => {
                if lookahead == 110 { state = 245; lexer.advance(false); continue; }
                return result;
            }
            213 => {
                if lookahead == 100 { state = 246; lexer.advance(false); continue; }
                return result;
            }
            214 => {
                if lookahead == 99 { state = 247; lexer.advance(false); continue; }
                return result;
            }
            215 => {
                result = true; lexer.set_result_symbol(anon_sym_super); lexer.mark_end();
                return result;
            }
            216 => {
                if lookahead == 104 { state = 248; lexer.advance(false); continue; }
                return result;
            }
            217 => {
                result = true; lexer.set_result_symbol(anon_sym_throw); lexer.mark_end();
                return result;
            }
            218 => {
                if lookahead == 101 { state = 249; lexer.advance(false); continue; }
                return result;
            }
            219 => {
                result = true; lexer.set_result_symbol(anon_sym_while); lexer.mark_end();
                return result;
            }
            220 => {
                result = true; lexer.set_result_symbol(anon_sym_yield); lexer.mark_end();
                return result;
            }
            221 => {
                if lookahead == 111 { state = 250; lexer.advance(false); continue; }
                return result;
            }
            222 => {
                if lookahead == 99 { state = 251; lexer.advance(false); continue; }
                return result;
            }
            223 => {
                result = true; lexer.set_result_symbol(anon_sym_assert); lexer.mark_end();
                return result;
            }
            224 => {
                if lookahead == 116 { state = 252; lexer.advance(false); continue; }
                return result;
            }
            225 => {
                if lookahead == 117 { state = 253; lexer.advance(false); continue; }
                return result;
            }
            226 => {
                if lookahead == 97 { state = 254; lexer.advance(false); continue; }
                return result;
            }
            227 => {
                if lookahead == 116 { state = 255; lexer.advance(false); continue; }
                return result;
            }
            228 => {
                if lookahead == 101 { state = 256; lexer.advance(false); continue; }
                return result;
            }
            229 => {
                if lookahead == 99 { state = 257; lexer.advance(false); continue; }
                return result;
            }
            230 => {
                result = true; lexer.set_result_symbol(anon_sym_export); lexer.mark_end();
                return result;
            }
            231 => {
                if lookahead == 115 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            232 => {
                if lookahead == 105 { state = 259; lexer.advance(false); continue; }
                return result;
            }
            233 => {
                if lookahead == 97 { state = 260; lexer.advance(false); continue; }
                return result;
            }
            234 => {
                if lookahead == 121 { state = 261; lexer.advance(false); continue; }
                return result;
            }
            235 => {
                if lookahead == 121 { state = 262; lexer.advance(false); continue; }
                return result;
            }
            236 => {
                if lookahead == 101 { state = 263; lexer.advance(false); continue; }
                return result;
            }
            237 => {
                result = true; lexer.set_result_symbol(anon_sym_import); lexer.mark_end();
                return result;
            }
            238 => {
                result = true; lexer.set_result_symbol(anon_sym_inline); lexer.mark_end();
                return result;
            }
            239 => {
                if lookahead == 97 { state = 264; lexer.advance(false); continue; }
                return result;
            }
            240 => {
                if lookahead == 121 { state = 265; lexer.advance(false); continue; }
                return result;
            }
            241 => {
                result = true; lexer.set_result_symbol(anon_sym_native); lexer.mark_end();
                return result;
            }
            242 => {
                if lookahead == 111 { state = 266; lexer.advance(false); continue; }
                return result;
            }
            243 => {
                if lookahead == 101 { state = 267; lexer.advance(false); continue; }
                return result;
            }
            244 => {
                if lookahead == 119 { state = 268; lexer.advance(false); continue; }
                return result;
            }
            245 => {
                result = true; lexer.set_result_symbol(anon_sym_return); lexer.mark_end();
                return result;
            }
            246 => {
                result = true; lexer.set_result_symbol(anon_sym_sealed); lexer.mark_end();
                return result;
            }
            247 => {
                result = true; lexer.set_result_symbol(anon_sym_static); lexer.mark_end();
                return result;
            }
            248 => {
                result = true; lexer.set_result_symbol(anon_sym_switch); lexer.mark_end();
                return result;
            }
            249 => {
                if lookahead == 102 { state = 269; lexer.advance(false); continue; }
                return result;
            }
            250 => {
                if lookahead == 110 { state = 270; lexer.advance(false); continue; }
                return result;
            }
            251 => {
                if lookahead == 116 { state = 271; lexer.advance(false); continue; }
                return result;
            }
            252 => {
                result = true; lexer.set_result_symbol(anon_sym_augment); lexer.mark_end();
                return result;
            }
            253 => {
                if lookahead == 101 { state = 272; lexer.advance(false); continue; }
                return result;
            }
            254 => {
                if lookahead == 110 { state = 273; lexer.advance(false); continue; }
                return result;
            }
            255 => {
                result = true; lexer.set_result_symbol(anon_sym_default); lexer.mark_end();
                return result;
            }
            256 => {
                if lookahead == 100 { state = 274; lexer.advance(false); continue; }
                return result;
            }
            257 => {
                result = true; lexer.set_result_symbol(anon_sym_dynamic); lexer.mark_end();
                return result;
            }
            258 => {
                result = true; lexer.set_result_symbol(anon_sym_extends); lexer.mark_end();
                return result;
            }
            259 => {
                if lookahead == 111 { state = 275; lexer.advance(false); continue; }
                return result;
            }
            260 => {
                if lookahead == 108 { state = 276; lexer.advance(false); continue; }
                return result;
            }
            261 => {
                result = true; lexer.set_result_symbol(anon_sym_factory); lexer.mark_end();
                return result;
            }
            262 => {
                result = true; lexer.set_result_symbol(anon_sym_finally); lexer.mark_end();
                return result;
            }
            263 => {
                if lookahead == 110 { state = 277; lexer.advance(false); continue; }
                return result;
            }
            264 => {
                if lookahead == 99 { state = 278; lexer.advance(false); continue; }
                return result;
            }
            265 => {
                result = true; lexer.set_result_symbol(anon_sym_library); lexer.mark_end();
                return result;
            }
            266 => {
                if lookahead == 114 { state = 279; lexer.advance(false); continue; }
                return result;
            }
            267 => {
                if lookahead == 100 { state = 280; lexer.advance(false); continue; }
                return result;
            }
            268 => {
                result = true; lexer.set_result_symbol(anon_sym_rethrow); lexer.mark_end();
                return result;
            }
            269 => {
                result = true; lexer.set_result_symbol(anon_sym_typedef); lexer.mark_end();
                return result;
            }
            270 => {
                result = true; lexer.set_result_symbol(anon_sym_Function); lexer.mark_end();
                return result;
            }
            271 => {
                result = true; lexer.set_result_symbol(anon_sym_abstract); lexer.mark_end();
                return result;
            }
            272 => {
                result = true; lexer.set_result_symbol(anon_sym_continue); lexer.mark_end();
                return result;
            }
            273 => {
                if lookahead == 116 { state = 281; lexer.advance(false); continue; }
                return result;
            }
            274 => {
                result = true; lexer.set_result_symbol(anon_sym_deferred); lexer.mark_end();
                return result;
            }
            275 => {
                if lookahead == 110 { state = 282; lexer.advance(false); continue; }
                return result;
            }
            276 => {
                result = true; lexer.set_result_symbol(anon_sym_external); lexer.mark_end();
                return result;
            }
            277 => {
                if lookahead == 116 { state = 283; lexer.advance(false); continue; }
                return result;
            }
            278 => {
                if lookahead == 101 { state = 284; lexer.advance(false); continue; }
                return result;
            }
            279 => {
                result = true; lexer.set_result_symbol(anon_sym_operator); lexer.mark_end();
                return result;
            }
            280 => {
                result = true; lexer.set_result_symbol(anon_sym_required); lexer.mark_end();
                return result;
            }
            281 => {
                result = true; lexer.set_result_symbol(anon_sym_covariant); lexer.mark_end();
                return result;
            }
            282 => {
                result = true; lexer.set_result_symbol(anon_sym_extension); lexer.mark_end();
                return result;
            }
            283 => {
                if lookahead == 115 { state = 285; lexer.advance(false); continue; }
                return result;
            }
            284 => {
                result = true; lexer.set_result_symbol(anon_sym_interface); lexer.mark_end();
                return result;
            }
            285 => {
                result = true; lexer.set_result_symbol(anon_sym_implements); lexer.mark_end();
                return result;
            }
            _ => return false,
        }
    }
}
