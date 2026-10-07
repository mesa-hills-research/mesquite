//! The `lua` grammar's lexer: `ts_lex` and `ts_lex_keywords`, transliterated from
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

const anon_sym_AMP: Symbol = 53;
const anon_sym_CARET: Symbol = 62;
const anon_sym_COLON: Symbol = 25;
const anon_sym_COLON_COLON: Symbol = 7;
const anon_sym_COMMA: Symbol = 6;
const anon_sym_DASH: Symbol = 57;
const anon_sym_DASH_DASH: Symbol = 65;
const anon_sym_DOT: Symbol = 24;
const anon_sym_DOT_DOT: Symbol = 61;
const anon_sym_DQUOTE: Symbol = 33;
const anon_sym_EQ: Symbol = 5;
const anon_sym_EQ_EQ: Symbol = 48;
const anon_sym_GT: Symbol = 28;
const anon_sym_GT_EQ: Symbol = 50;
const anon_sym_GT_GT: Symbol = 55;
const anon_sym_LBRACE: Symbol = 43;
const anon_sym_LBRACK: Symbol = 41;
const anon_sym_LPAREN: Symbol = 39;
const anon_sym_LT: Symbol = 27;
const anon_sym_LT_EQ: Symbol = 47;
const anon_sym_LT_LT: Symbol = 54;
const anon_sym_PERCENT: Symbol = 60;
const anon_sym_PIPE: Symbol = 51;
const anon_sym_PLUS: Symbol = 56;
const anon_sym_POUND: Symbol = 64;
const anon_sym_RBRACE: Symbol = 44;
const anon_sym_RBRACK: Symbol = 42;
const anon_sym_RPAREN: Symbol = 40;
const anon_sym_SEMI: Symbol = 4;
const anon_sym_SLASH: Symbol = 58;
const anon_sym_SLASH_SLASH: Symbol = 59;
const anon_sym_SQUOTE: Symbol = 34;
const anon_sym_STAR: Symbol = 26;
const anon_sym_TILDE: Symbol = 52;
const anon_sym_TILDE_EQ: Symbol = 49;
const anon_sym_and: Symbol = 46;
const anon_sym_do: Symbol = 10;
const anon_sym_else: Symbol = 18;
const anon_sym_elseif: Symbol = 17;
const anon_sym_end: Symbol = 11;
const anon_sym_for: Symbol = 19;
const anon_sym_function: Symbol = 21;
const anon_sym_global: Symbol = 23;
const anon_sym_goto: Symbol = 9;
const anon_sym_if: Symbol = 15;
const anon_sym_in: Symbol = 20;
const anon_sym_local: Symbol = 22;
const anon_sym_not: Symbol = 63;
const anon_sym_or: Symbol = 45;
const anon_sym_repeat: Symbol = 13;
const anon_sym_return: Symbol = 3;
const anon_sym_then: Symbol = 16;
const anon_sym_until: Symbol = 14;
const anon_sym_while: Symbol = 12;
const aux_sym__doublequote_string_content_token1: Symbol = 35;
const aux_sym__singlequote_string_content_token1: Symbol = 36;
const aux_sym_comment_token1: Symbol = 66;
const sym_break_statement: Symbol = 8;
const sym_escape_sequence: Symbol = 37;
const sym_false: Symbol = 30;
const sym_hash_bang_line: Symbol = 2;
const sym_identifier: Symbol = 1;
const sym_nil: Symbol = 29;
const sym_number: Symbol = 32;
const sym_true: Symbol = 31;
const sym_vararg_expression: Symbol = 38;
const ts_builtin_sym_end: Symbol = 0;

#[rustfmt::skip]
static sym_escape_sequence_character_set_1: [CharacterRange; 12] = [
    CharacterRange::new(10, 10), CharacterRange::new(34, 34), CharacterRange::new(39, 39), CharacterRange::new(48, 57), CharacterRange::new(92, 92), CharacterRange::new(97, 98),
    CharacterRange::new(102, 102), CharacterRange::new(110, 110), CharacterRange::new(114, 114), CharacterRange::new(116, 118), CharacterRange::new(120, 120), CharacterRange::new(122, 122),
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
                if eof { state = 21; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (34, 42), (35, 78), (37, 75), (38, 68), (39, 43), (40, 55), (41, 56), (42, 30),
                    (43, 71), (44, 25), (45, 72), (46, 28), (47, 73), (48, 35), (58, 29), (59, 23),
                    (60, 31), (61, 24), (62, 33), (91, 57), (92, 6), (93, 58), (94, 77), (123, 59),
                    (124, 65), (125, 60), (126, 67),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 18; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 36; lexer.advance(false); continue; }
                if lookahead > 32 && (lookahead < 123 || 159 < lookahead) { state = 79; lexer.advance(false); continue; }
                return result;
            }
            1 => {
                if lookahead == 34 { state = 42; lexer.advance(false); continue; }
                if lookahead == 45 { state = 45; lexer.advance(false); continue; }
                if lookahead == 92 { state = 6; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 44; lexer.advance(false); continue; }
                if lookahead != 0 { state = 46; lexer.advance(false); continue; }
                return result;
            }
            2 => {
                if lookahead == 39 { state = 43; lexer.advance(false); continue; }
                if lookahead == 45 { state = 48; lexer.advance(false); continue; }
                if lookahead == 92 { state = 6; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 47; lexer.advance(false); continue; }
                if lookahead != 0 { state = 49; lexer.advance(false); continue; }
                return result;
            }
            3 => {
                if lookahead == 46 { state = 4; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 38; lexer.advance(false); continue; }
                return result;
            }
            4 => {
                if lookahead == 46 { state = 54; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                if lookahead == 46 { state = 15; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 37; lexer.advance(false); continue; }
                return result;
            }
            6 => {
                if lookahead == 117 { state = 7; lexer.advance(false); continue; }
                if lookahead == 120 { state = 17; lexer.advance(false); continue; }
                if lookahead == 122 { state = 51; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 53; lexer.advance(false); continue; }
                if set_contains(&sym_escape_sequence_character_set_1, lookahead) { state = 50; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if lookahead == 123 { state = 16; lexer.advance(false); continue; }
                return result;
            }
            8 => {
                if lookahead == 125 { state = 50; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 8; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if lookahead == 43 || lookahead == 45 { state = 13; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 41; lexer.advance(false); continue; }
                return result;
            }
            10 => {
                if lookahead == 76 || lookahead == 108 { state = 34; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if lookahead == 76 || lookahead == 108 { state = 10; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if lookahead == 48 || lookahead == 49 { state = 39; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if 48 <= lookahead && lookahead <= 57 { state = 41; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 50; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 40; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 8; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 14; lexer.advance(false); continue; }
                return result;
            }
            18 => {
                if eof { state = 21; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (34, 42), (35, 78), (37, 75), (38, 68), (39, 43), (40, 55), (41, 56), (42, 30),
                    (43, 71), (44, 25), (45, 72), (46, 28), (47, 73), (48, 35), (58, 29), (59, 23),
                    (60, 31), (61, 24), (62, 33), (91, 57), (93, 58), (94, 77), (123, 59), (124, 65),
                    (125, 60), (126, 67),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 18; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 36; lexer.advance(false); continue; }
                if lookahead > 32 && (lookahead < 91 || 94 < lookahead) && (lookahead < 123 || 159 < lookahead) { state = 79; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if eof { state = 21; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (34, 42), (35, 78), (39, 43), (40, 55), (41, 56), (45, 72), (46, 3), (48, 35),
                    (59, 23), (62, 32), (91, 57), (123, 59), (125, 60), (126, 66),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 19; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 36; lexer.advance(false); continue; }
                if lookahead > 32 && (lookahead < 37 || 62 < lookahead) && (lookahead < 91 || 94 < lookahead) && (lookahead < 123 || 159 < lookahead) { state = 79; lexer.advance(false); continue; }
                return result;
            }
            20 => {
                if eof { state = 21; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (34, 42), (35, 22), (37, 75), (38, 68), (39, 43), (40, 55), (41, 56), (42, 30),
                    (43, 71), (44, 25), (45, 72), (46, 27), (47, 73), (58, 29), (59, 23), (60, 31),
                    (61, 24), (62, 33), (91, 57), (93, 58), (94, 77), (123, 59), (124, 65), (125, 60),
                    (126, 67),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 20; lexer.advance(true); continue; }
                if lookahead > 32 && (lookahead < 37 || 62 < lookahead) && (lookahead < 91 || 94 < lookahead) && (lookahead < 123 || 159 < lookahead) { state = 79; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                result = true; lexer.set_result_symbol(ts_builtin_sym_end); lexer.mark_end();
                return result;
            }
            22 => {
                result = true; lexer.set_result_symbol(sym_hash_bang_line); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 { state = 22; lexer.advance(false); continue; }
                return result;
            }
            23 => {
                result = true; lexer.set_result_symbol(anon_sym_SEMI); lexer.mark_end();
                return result;
            }
            24 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                if lookahead == 61 { state = 62; lexer.advance(false); continue; }
                return result;
            }
            25 => {
                result = true; lexer.set_result_symbol(anon_sym_COMMA); lexer.mark_end();
                return result;
            }
            26 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON_COLON); lexer.mark_end();
                return result;
            }
            27 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                if lookahead == 46 { state = 76; lexer.advance(false); continue; }
                return result;
            }
            28 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                if lookahead == 46 { state = 76; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 38; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON); lexer.mark_end();
                if lookahead == 58 { state = 26; lexer.advance(false); continue; }
                return result;
            }
            30 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                return result;
            }
            31 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 60 { state = 69; lexer.advance(false); continue; }
                if lookahead == 61 { state = 61; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                return result;
            }
            33 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                if lookahead == 61 { state = 64; lexer.advance(false); continue; }
                if lookahead == 62 { state = 70; lexer.advance(false); continue; }
                return result;
            }
            34 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                return result;
            }
            35 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (46, 38), (66, 12), (98, 12), (69, 9), (101, 9), (73, 34), (105, 34), (76, 10),
                    (108, 10), (85, 11), (117, 11), (88, 5), (120, 5),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 36; lexer.advance(false); continue; }
                return result;
            }
            36 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (46, 38), (69, 9), (101, 9), (73, 34), (105, 34), (76, 10), (108, 10), (85, 11),
                    (117, 11),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 36; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (46, 40), (73, 34), (105, 34), (76, 10), (108, 10), (80, 9), (112, 9), (85, 11),
                    (117, 11),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 37; lexer.advance(false); continue; }
                return result;
            }
            38 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if lookahead == 69 || lookahead == 101 { state = 9; lexer.advance(false); continue; }
                if lookahead == 73 || lookahead == 105 { state = 34; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 38; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if lookahead == 73 || lookahead == 105 { state = 34; lexer.advance(false); continue; }
                if lookahead == 76 || lookahead == 108 { state = 10; lexer.advance(false); continue; }
                if lookahead == 85 || lookahead == 117 { state = 11; lexer.advance(false); continue; }
                if lookahead == 48 || lookahead == 49 { state = 39; lexer.advance(false); continue; }
                return result;
            }
            40 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if lookahead == 73 || lookahead == 105 { state = 34; lexer.advance(false); continue; }
                if lookahead == 80 || lookahead == 112 { state = 9; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 40; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if lookahead == 73 || lookahead == 105 { state = 34; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 41; lexer.advance(false); continue; }
                return result;
            }
            42 => {
                result = true; lexer.set_result_symbol(anon_sym_DQUOTE); lexer.mark_end();
                return result;
            }
            43 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTE); lexer.mark_end();
                return result;
            }
            44 => {
                result = true; lexer.set_result_symbol(aux_sym__doublequote_string_content_token1); lexer.mark_end();
                if lookahead == 45 { state = 45; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 44; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 34 && lookahead != 92 { state = 46; lexer.advance(false); continue; }
                return result;
            }
            45 => {
                result = true; lexer.set_result_symbol(aux_sym__doublequote_string_content_token1); lexer.mark_end();
                if lookahead == 45 { state = 46; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 34 && lookahead != 92 { state = 46; lexer.advance(false); continue; }
                return result;
            }
            46 => {
                result = true; lexer.set_result_symbol(aux_sym__doublequote_string_content_token1); lexer.mark_end();
                if lookahead != 0 && lookahead != 34 && lookahead != 92 { state = 46; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                result = true; lexer.set_result_symbol(aux_sym__singlequote_string_content_token1); lexer.mark_end();
                if lookahead == 45 { state = 48; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 47; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 39 && lookahead != 92 { state = 49; lexer.advance(false); continue; }
                return result;
            }
            48 => {
                result = true; lexer.set_result_symbol(aux_sym__singlequote_string_content_token1); lexer.mark_end();
                if lookahead == 45 { state = 49; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 39 && lookahead != 92 { state = 49; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                result = true; lexer.set_result_symbol(aux_sym__singlequote_string_content_token1); lexer.mark_end();
                if lookahead != 0 && lookahead != 39 && lookahead != 92 { state = 49; lexer.advance(false); continue; }
                return result;
            }
            50 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                return result;
            }
            51 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 51; lexer.advance(false); continue; }
                return result;
            }
            52 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 { state = 50; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 { state = 52; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                result = true; lexer.set_result_symbol(sym_vararg_expression); lexer.mark_end();
                return result;
            }
            55 => {
                result = true; lexer.set_result_symbol(anon_sym_LPAREN); lexer.mark_end();
                return result;
            }
            56 => {
                result = true; lexer.set_result_symbol(anon_sym_RPAREN); lexer.mark_end();
                return result;
            }
            57 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK); lexer.mark_end();
                return result;
            }
            58 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACK); lexer.mark_end();
                return result;
            }
            59 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACE); lexer.mark_end();
                return result;
            }
            60 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACE); lexer.mark_end();
                return result;
            }
            61 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_EQ); lexer.mark_end();
                return result;
            }
            62 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ_EQ); lexer.mark_end();
                return result;
            }
            63 => {
                result = true; lexer.set_result_symbol(anon_sym_TILDE_EQ); lexer.mark_end();
                return result;
            }
            64 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_EQ); lexer.mark_end();
                return result;
            }
            65 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                return result;
            }
            66 => {
                result = true; lexer.set_result_symbol(anon_sym_TILDE); lexer.mark_end();
                return result;
            }
            67 => {
                result = true; lexer.set_result_symbol(anon_sym_TILDE); lexer.mark_end();
                if lookahead == 61 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            68 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP); lexer.mark_end();
                return result;
            }
            69 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT); lexer.mark_end();
                return result;
            }
            70 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT); lexer.mark_end();
                return result;
            }
            71 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                return result;
            }
            72 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 80; lexer.advance(false); continue; }
                return result;
            }
            73 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH); lexer.mark_end();
                if lookahead == 47 { state = 74; lexer.advance(false); continue; }
                return result;
            }
            74 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_SLASH); lexer.mark_end();
                return result;
            }
            75 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT); lexer.mark_end();
                return result;
            }
            76 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT_DOT); lexer.mark_end();
                return result;
            }
            77 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET); lexer.mark_end();
                return result;
            }
            78 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND); lexer.mark_end();
                return result;
            }
            79 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead > 32 && lookahead != 34 && lookahead != 35 && (lookahead < 37 || 47 < lookahead) && (lookahead < 58 || 62 < lookahead) && (lookahead < 91 || 94 < lookahead) && (lookahead < 123 || 159 < lookahead) { state = 79; lexer.advance(false); continue; }
                return result;
            }
            80 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_DASH); lexer.mark_end();
                return result;
            }
            81 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_DASH); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 84; lexer.advance(false); continue; }
                return result;
            }
            82 => {
                result = true; lexer.set_result_symbol(aux_sym_comment_token1); lexer.mark_end();
                if lookahead == 45 { state = 83; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 11 || lookahead == 12 || lookahead == 32 { state = 82; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) { state = 84; lexer.advance(false); continue; }
                return result;
            }
            83 => {
                result = true; lexer.set_result_symbol(aux_sym_comment_token1); lexer.mark_end();
                if lookahead == 45 { state = 81; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 84; lexer.advance(false); continue; }
                return result;
            }
            84 => {
                result = true; lexer.set_result_symbol(aux_sym_comment_token1); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 84; lexer.advance(false); continue; }
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
                    (97, 1), (98, 2), (100, 3), (101, 4), (102, 5), (103, 6), (105, 7), (108, 8),
                    (110, 9), (111, 10), (114, 11), (116, 12), (117, 13), (119, 14),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 0; lexer.advance(true); continue; }
                return result;
            }
            1 => {
                if lookahead == 110 { state = 15; lexer.advance(false); continue; }
                return result;
            }
            2 => {
                if lookahead == 114 { state = 16; lexer.advance(false); continue; }
                return result;
            }
            3 => {
                if lookahead == 111 { state = 17; lexer.advance(false); continue; }
                return result;
            }
            4 => {
                if lookahead == 108 { state = 18; lexer.advance(false); continue; }
                if lookahead == 110 { state = 19; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                if lookahead == 97 { state = 20; lexer.advance(false); continue; }
                if lookahead == 111 { state = 21; lexer.advance(false); continue; }
                if lookahead == 117 { state = 22; lexer.advance(false); continue; }
                return result;
            }
            6 => {
                if lookahead == 108 { state = 23; lexer.advance(false); continue; }
                if lookahead == 111 { state = 24; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if lookahead == 102 { state = 25; lexer.advance(false); continue; }
                if lookahead == 110 { state = 26; lexer.advance(false); continue; }
                return result;
            }
            8 => {
                if lookahead == 111 { state = 27; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if lookahead == 105 { state = 28; lexer.advance(false); continue; }
                if lookahead == 111 { state = 29; lexer.advance(false); continue; }
                return result;
            }
            10 => {
                if lookahead == 114 { state = 30; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if lookahead == 101 { state = 31; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if lookahead == 104 { state = 32; lexer.advance(false); continue; }
                if lookahead == 114 { state = 33; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if lookahead == 110 { state = 34; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if lookahead == 104 { state = 35; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if lookahead == 100 { state = 36; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if lookahead == 101 { state = 37; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                result = true; lexer.set_result_symbol(anon_sym_do); lexer.mark_end();
                return result;
            }
            18 => {
                if lookahead == 115 { state = 38; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if lookahead == 100 { state = 39; lexer.advance(false); continue; }
                return result;
            }
            20 => {
                if lookahead == 108 { state = 40; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                if lookahead == 114 { state = 41; lexer.advance(false); continue; }
                return result;
            }
            22 => {
                if lookahead == 110 { state = 42; lexer.advance(false); continue; }
                return result;
            }
            23 => {
                if lookahead == 111 { state = 43; lexer.advance(false); continue; }
                return result;
            }
            24 => {
                if lookahead == 116 { state = 44; lexer.advance(false); continue; }
                return result;
            }
            25 => {
                result = true; lexer.set_result_symbol(anon_sym_if); lexer.mark_end();
                return result;
            }
            26 => {
                result = true; lexer.set_result_symbol(anon_sym_in); lexer.mark_end();
                return result;
            }
            27 => {
                if lookahead == 99 { state = 45; lexer.advance(false); continue; }
                return result;
            }
            28 => {
                if lookahead == 108 { state = 46; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                if lookahead == 116 { state = 47; lexer.advance(false); continue; }
                return result;
            }
            30 => {
                result = true; lexer.set_result_symbol(anon_sym_or); lexer.mark_end();
                return result;
            }
            31 => {
                if lookahead == 112 { state = 48; lexer.advance(false); continue; }
                if lookahead == 116 { state = 49; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                if lookahead == 101 { state = 50; lexer.advance(false); continue; }
                return result;
            }
            33 => {
                if lookahead == 117 { state = 51; lexer.advance(false); continue; }
                return result;
            }
            34 => {
                if lookahead == 116 { state = 52; lexer.advance(false); continue; }
                return result;
            }
            35 => {
                if lookahead == 105 { state = 53; lexer.advance(false); continue; }
                return result;
            }
            36 => {
                result = true; lexer.set_result_symbol(anon_sym_and); lexer.mark_end();
                return result;
            }
            37 => {
                if lookahead == 97 { state = 54; lexer.advance(false); continue; }
                return result;
            }
            38 => {
                if lookahead == 101 { state = 55; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                result = true; lexer.set_result_symbol(anon_sym_end); lexer.mark_end();
                return result;
            }
            40 => {
                if lookahead == 115 { state = 56; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                result = true; lexer.set_result_symbol(anon_sym_for); lexer.mark_end();
                return result;
            }
            42 => {
                if lookahead == 99 { state = 57; lexer.advance(false); continue; }
                return result;
            }
            43 => {
                if lookahead == 98 { state = 58; lexer.advance(false); continue; }
                return result;
            }
            44 => {
                if lookahead == 111 { state = 59; lexer.advance(false); continue; }
                return result;
            }
            45 => {
                if lookahead == 97 { state = 60; lexer.advance(false); continue; }
                return result;
            }
            46 => {
                result = true; lexer.set_result_symbol(sym_nil); lexer.mark_end();
                return result;
            }
            47 => {
                result = true; lexer.set_result_symbol(anon_sym_not); lexer.mark_end();
                return result;
            }
            48 => {
                if lookahead == 101 { state = 61; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                if lookahead == 117 { state = 62; lexer.advance(false); continue; }
                return result;
            }
            50 => {
                if lookahead == 110 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            51 => {
                if lookahead == 101 { state = 64; lexer.advance(false); continue; }
                return result;
            }
            52 => {
                if lookahead == 105 { state = 65; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                if lookahead == 108 { state = 66; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                if lookahead == 107 { state = 67; lexer.advance(false); continue; }
                return result;
            }
            55 => {
                result = true; lexer.set_result_symbol(anon_sym_else); lexer.mark_end();
                if lookahead == 105 { state = 68; lexer.advance(false); continue; }
                return result;
            }
            56 => {
                if lookahead == 101 { state = 69; lexer.advance(false); continue; }
                return result;
            }
            57 => {
                if lookahead == 116 { state = 70; lexer.advance(false); continue; }
                return result;
            }
            58 => {
                if lookahead == 97 { state = 71; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                result = true; lexer.set_result_symbol(anon_sym_goto); lexer.mark_end();
                return result;
            }
            60 => {
                if lookahead == 108 { state = 72; lexer.advance(false); continue; }
                return result;
            }
            61 => {
                if lookahead == 97 { state = 73; lexer.advance(false); continue; }
                return result;
            }
            62 => {
                if lookahead == 114 { state = 74; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                result = true; lexer.set_result_symbol(anon_sym_then); lexer.mark_end();
                return result;
            }
            64 => {
                result = true; lexer.set_result_symbol(sym_true); lexer.mark_end();
                return result;
            }
            65 => {
                if lookahead == 108 { state = 75; lexer.advance(false); continue; }
                return result;
            }
            66 => {
                if lookahead == 101 { state = 76; lexer.advance(false); continue; }
                return result;
            }
            67 => {
                result = true; lexer.set_result_symbol(sym_break_statement); lexer.mark_end();
                return result;
            }
            68 => {
                if lookahead == 102 { state = 77; lexer.advance(false); continue; }
                return result;
            }
            69 => {
                result = true; lexer.set_result_symbol(sym_false); lexer.mark_end();
                return result;
            }
            70 => {
                if lookahead == 105 { state = 78; lexer.advance(false); continue; }
                return result;
            }
            71 => {
                if lookahead == 108 { state = 79; lexer.advance(false); continue; }
                return result;
            }
            72 => {
                result = true; lexer.set_result_symbol(anon_sym_local); lexer.mark_end();
                return result;
            }
            73 => {
                if lookahead == 116 { state = 80; lexer.advance(false); continue; }
                return result;
            }
            74 => {
                if lookahead == 110 { state = 81; lexer.advance(false); continue; }
                return result;
            }
            75 => {
                result = true; lexer.set_result_symbol(anon_sym_until); lexer.mark_end();
                return result;
            }
            76 => {
                result = true; lexer.set_result_symbol(anon_sym_while); lexer.mark_end();
                return result;
            }
            77 => {
                result = true; lexer.set_result_symbol(anon_sym_elseif); lexer.mark_end();
                return result;
            }
            78 => {
                if lookahead == 111 { state = 82; lexer.advance(false); continue; }
                return result;
            }
            79 => {
                result = true; lexer.set_result_symbol(anon_sym_global); lexer.mark_end();
                return result;
            }
            80 => {
                result = true; lexer.set_result_symbol(anon_sym_repeat); lexer.mark_end();
                return result;
            }
            81 => {
                result = true; lexer.set_result_symbol(anon_sym_return); lexer.mark_end();
                return result;
            }
            82 => {
                if lookahead == 110 { state = 83; lexer.advance(false); continue; }
                return result;
            }
            83 => {
                result = true; lexer.set_result_symbol(anon_sym_function); lexer.mark_end();
                return result;
            }
            _ => return false,
        }
    }
}
