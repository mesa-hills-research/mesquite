//! The `make` grammar's lexer: `ts_lex` and `ts_lex_keywords`, transliterated from
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

const anon_sym_AMP_COLON: Symbol = 4;
const anon_sym_AT: Symbol = 9;
const anon_sym_AT2: Symbol = 46;
const anon_sym_BANG_EQ: Symbol = 19;
const anon_sym_CARET: Symbol = 50;
const anon_sym_CARET2: Symbol = 57;
const anon_sym_COLON: Symbol = 3;
const anon_sym_COLON2: Symbol = 99;
const anon_sym_COLON_COLON: Symbol = 5;
const anon_sym_COLON_COLON_EQ: Symbol = 15;
const anon_sym_COLON_EQ: Symbol = 14;
const anon_sym_COMMA: Symbol = 38;
const anon_sym_D: Symbol = 60;
const anon_sym_DASH: Symbol = 10;
const anon_sym_DASHinclude: Symbol = 24;
const anon_sym_DOLLAR: Symbol = 40;
const anon_sym_DOLLAR_DOLLAR: Symbol = 41;
const anon_sym_DOTRECIPEPREFIX: Symbol = 18;
const anon_sym_DQUOTE: Symbol = 101;
const anon_sym_EQ: Symbol = 13;
const anon_sym_F: Symbol = 61;
const anon_sym_LBRACE: Symbol = 43;
const anon_sym_LPAREN: Symbol = 37;
const anon_sym_LPAREN2: Symbol = 42;
const anon_sym_LT: Symbol = 48;
const anon_sym_LT2: Symbol = 55;
const anon_sym_PERCENT: Symbol = 47;
const anon_sym_PERCENT2: Symbol = 54;
const anon_sym_PIPE: Symbol = 7;
const anon_sym_PLUS: Symbol = 11;
const anon_sym_PLUS2: Symbol = 51;
const anon_sym_PLUS_EQ: Symbol = 17;
const anon_sym_QMARK: Symbol = 49;
const anon_sym_QMARK2: Symbol = 56;
const anon_sym_QMARK_EQ: Symbol = 16;
const anon_sym_RBRACE: Symbol = 44;
const anon_sym_RPAREN: Symbol = 39;
const anon_sym_RPAREN2: Symbol = 104;
const anon_sym_SEMI: Symbol = 8;
const anon_sym_SEMI2: Symbol = 100;
const anon_sym_SLASH: Symbol = 52;
const anon_sym_SLASH2: Symbol = 58;
const anon_sym_SLASH_SLASH: Symbol = 108;
const anon_sym_SQUOTE: Symbol = 102;
const anon_sym_STAR: Symbol = 53;
const anon_sym_STAR2: Symbol = 59;
const anon_sym_TAB: Symbol = 105;
const anon_sym_VPATH: Symbol = 12;
const anon_sym_abspath: Symbol = 83;
const anon_sym_addprefix: Symbol = 79;
const anon_sym_addsuffix: Symbol = 78;
const anon_sym_and: Symbol = 92;
const anon_sym_basename: Symbol = 77;
const anon_sym_call: Symbol = 93;
const anon_sym_define: Symbol = 20;
const anon_sym_dir: Symbol = 74;
const anon_sym_else: Symbol = 32;
const anon_sym_endef: Symbol = 21;
const anon_sym_endif: Symbol = 31;
const anon_sym_error: Symbol = 84;
const anon_sym_eval: Symbol = 94;
const anon_sym_export: Symbol = 26;
const anon_sym_file: Symbol = 95;
const anon_sym_filter: Symbol = 66;
const anon_sym_filter_DASHout: Symbol = 67;
const anon_sym_findstring: Symbol = 65;
const anon_sym_firstword: Symbol = 72;
const anon_sym_flavor: Symbol = 88;
const anon_sym_foreach: Symbol = 89;
const anon_sym_if: Symbol = 90;
const anon_sym_ifdef: Symbol = 35;
const anon_sym_ifeq: Symbol = 33;
const anon_sym_ifndef: Symbol = 36;
const anon_sym_ifneq: Symbol = 34;
const anon_sym_include: Symbol = 22;
const anon_sym_info: Symbol = 86;
const anon_sym_join: Symbol = 80;
const anon_sym_lastword: Symbol = 73;
const anon_sym_notdir: Symbol = 75;
const anon_sym_or: Symbol = 91;
const anon_sym_origin: Symbol = 87;
const anon_sym_override: Symbol = 28;
const anon_sym_patsubst: Symbol = 63;
const anon_sym_private: Symbol = 30;
const anon_sym_realpath: Symbol = 82;
const anon_sym_shell: Symbol = 97;
const anon_sym_sinclude: Symbol = 23;
const anon_sym_sort: Symbol = 68;
const anon_sym_strip: Symbol = 64;
const anon_sym_subst: Symbol = 62;
const anon_sym_suffix: Symbol = 76;
const anon_sym_undefine: Symbol = 29;
const anon_sym_unexport: Symbol = 27;
const anon_sym_value: Symbol = 96;
const anon_sym_vpath: Symbol = 25;
const anon_sym_warning: Symbol = 85;
const anon_sym_wildcard: Symbol = 81;
const anon_sym_word: Symbol = 69;
const anon_sym_wordlist: Symbol = 71;
const anon_sym_words: Symbol = 70;
const aux_sym__ordinary_rule_token1: Symbol = 6;
const aux_sym__shell_text_without_split_token1: Symbol = 107;
const aux_sym__string_token1: Symbol = 103;
const aux_sym__thing_token1: Symbol = 2;
const aux_sym_list_token1: Symbol = 98;
const aux_sym_text_token1: Symbol = 109;
const aux_sym_variable_reference_token1: Symbol = 45;
const sym__rawline: Symbol = 106;
const sym_comment: Symbol = 110;
const sym_word: Symbol = 1;
const ts_builtin_sym_end: Symbol = 0;

#[rustfmt::skip]
static sym_word_character_set_1: [CharacterRange; 15] = [
    CharacterRange::new(33, 36), CharacterRange::new(38, 42), CharacterRange::new(44, 44), CharacterRange::new(48, 57), CharacterRange::new(59, 60), CharacterRange::new(62, 63),
    CharacterRange::new(69, 69), CharacterRange::new(91, 94), CharacterRange::new(96, 98), CharacterRange::new(102, 102), CharacterRange::new(110, 110), CharacterRange::new(114, 114),
    CharacterRange::new(116, 116), CharacterRange::new(118, 118), CharacterRange::new(123, 126),
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
                if eof { state = 107; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 95), (34, 168), (35, 233), (36, 136), (37, 150), (38, 93), (39, 169), (40, 138),
                    (41, 200), (42, 156), (43, 154), (44, 133), (45, 121), (46, 185), (47, 155), (58, 166),
                    (59, 167), (60, 151), (61, 123), (63, 152), (64, 149), (92, 3), (94, 153), (101, 196),
                    (123, 140), (124, 118), (125, 143),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 105; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            1 => {
                if let Some(next) = advance_map(&[
                    (9, 201), (34, 168), (35, 233), (36, 136), (39, 169), (45, 194), (46, 185), (92, 5),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 10 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 1; lexer.advance(true); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 47 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            2 => {
                if lookahead == 9 { state = 202; lexer.advance(false); continue; }
                if lookahead == 35 { state = 212; lexer.advance(false); continue; }
                if lookahead == 36 { state = 136; lexer.advance(false); continue; }
                if lookahead == 47 { state = 210; lexer.advance(false); continue; }
                if lookahead == 92 { state = 28; lexer.advance(false); continue; }
                if lookahead == 10 || lookahead == 13 { state = 2; lexer.advance(true); continue; }
                if lookahead == 11 || lookahead == 12 || lookahead == 32 { state = 207; lexer.advance(false); continue; }
                if lookahead != 0 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            3 => {
                if lookahead == 10 { state = 45; lexer.advance(true); continue; }
                if lookahead == 13 { state = 45; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 101; lexer.advance(false); continue; }
                if set_contains(&sym_word_character_set_1, lookahead) { state = 199; lexer.advance(false); continue; }
                return result;
            }
            4 => {
                if lookahead == 10 { state = 54; lexer.advance(true); continue; }
                if lookahead == 13 { state = 54; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 101; lexer.advance(false); continue; }
                if set_contains(&sym_word_character_set_1, lookahead) { state = 199; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                if lookahead == 10 { state = 1; lexer.advance(true); continue; }
                if lookahead == 13 { state = 1; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 101; lexer.advance(false); continue; }
                if set_contains(&sym_word_character_set_1, lookahead) { state = 199; lexer.advance(false); continue; }
                return result;
            }
            6 => {
                if lookahead == 10 { state = 49; lexer.advance(true); continue; }
                if lookahead == 13 { state = 49; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 101; lexer.advance(false); continue; }
                if set_contains(&sym_word_character_set_1, lookahead) { state = 199; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if lookahead == 10 { state = 47; lexer.advance(true); continue; }
                if lookahead == 13 { state = 47; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 101; lexer.advance(false); continue; }
                if set_contains(&sym_word_character_set_1, lookahead) { state = 199; lexer.advance(false); continue; }
                return result;
            }
            8 => {
                if lookahead == 10 { state = 163; lexer.advance(false); continue; }
                if lookahead == 13 { state = 164; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if lookahead == 10 { state = 163; lexer.advance(false); continue; }
                if lookahead == 13 { state = 164; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 101; lexer.advance(false); continue; }
                if set_contains(&sym_word_character_set_1, lookahead) { state = 199; lexer.advance(false); continue; }
                return result;
            }
            10 => {
                if lookahead == 10 { state = 163; lexer.advance(false); continue; }
                if lookahead == 13 { state = 164; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 213; lexer.advance(false); continue; }
                if lookahead != 0 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if let Some(next) = advance_map(&[
                    (10, 109), (13, 109), (35, 145), (36, 136), (37, 150), (40, 138), (41, 144), (42, 156),
                    (43, 154), (47, 155), (60, 151), (63, 152), (64, 149), (92, 144), (94, 153), (123, 141),
                    (9, 144), (11, 144), (12, 144), (32, 144),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead != 0 { state = 146; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if let Some(next) = advance_map(&[
                    (10, 109), (13, 109), (35, 147), (36, 136), (37, 150), (40, 139), (42, 156), (43, 154),
                    (47, 155), (60, 151), (63, 152), (64, 149), (92, 144), (94, 153), (123, 142), (9, 144),
                    (11, 144), (12, 144), (32, 144),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead != 0 { state = 148; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if lookahead == 10 { state = 63; lexer.advance(true); continue; }
                if lookahead == 13 { state = 63; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 101; lexer.advance(false); continue; }
                if set_contains(&sym_word_character_set_1, lookahead) { state = 199; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if lookahead == 10 { state = 80; lexer.advance(true); continue; }
                if lookahead == 13 { state = 144; lexer.advance(false); continue; }
                if lookahead == 35 { state = 145; lexer.advance(false); continue; }
                if lookahead == 36 { state = 136; lexer.advance(false); continue; }
                if lookahead == 37 { state = 150; lexer.advance(false); continue; }
                if lookahead == 40 { state = 138; lexer.advance(false); continue; }
                if lookahead == 41 { state = 135; lexer.advance(false); continue; }
                if lookahead == 42 { state = 156; lexer.advance(false); continue; }
                if lookahead == 43 { state = 154; lexer.advance(false); continue; }
                if lookahead == 44 { state = 134; lexer.advance(false); continue; }
                if lookahead == 47 { state = 155; lexer.advance(false); continue; }
                if lookahead == 60 { state = 151; lexer.advance(false); continue; }
                if lookahead == 63 { state = 152; lexer.advance(false); continue; }
                if lookahead == 64 { state = 149; lexer.advance(false); continue; }
                if lookahead == 92 { state = 144; lexer.advance(false); continue; }
                if lookahead == 94 { state = 153; lexer.advance(false); continue; }
                if lookahead == 123 { state = 141; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 144; lexer.advance(false); continue; }
                if lookahead != 0 { state = 146; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if lookahead == 10 { state = 219; lexer.advance(false); continue; }
                if lookahead == 13 { state = 220; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 226; lexer.advance(false); continue; }
                if lookahead != 0 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if lookahead == 10 { state = 88; lexer.advance(true); continue; }
                if lookahead == 13 { state = 144; lexer.advance(false); continue; }
                if lookahead == 35 { state = 147; lexer.advance(false); continue; }
                if lookahead == 36 { state = 136; lexer.advance(false); continue; }
                if lookahead == 37 { state = 150; lexer.advance(false); continue; }
                if lookahead == 40 { state = 139; lexer.advance(false); continue; }
                if lookahead == 42 { state = 156; lexer.advance(false); continue; }
                if lookahead == 43 { state = 154; lexer.advance(false); continue; }
                if lookahead == 47 { state = 155; lexer.advance(false); continue; }
                if lookahead == 60 { state = 151; lexer.advance(false); continue; }
                if lookahead == 63 { state = 152; lexer.advance(false); continue; }
                if lookahead == 64 { state = 149; lexer.advance(false); continue; }
                if lookahead == 92 { state = 144; lexer.advance(false); continue; }
                if lookahead == 94 { state = 153; lexer.advance(false); continue; }
                if lookahead == 123 { state = 142; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 144; lexer.advance(false); continue; }
                if lookahead != 0 { state = 148; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if lookahead == 10 { state = 88; lexer.advance(true); continue; }
                if lookahead == 13 { state = 88; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 213; lexer.advance(false); continue; }
                if lookahead != 0 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            18 => {
                if lookahead == 10 { state = 81; lexer.advance(true); continue; }
                if lookahead == 13 { state = 144; lexer.advance(false); continue; }
                if lookahead == 35 { state = 145; lexer.advance(false); continue; }
                if lookahead == 36 { state = 136; lexer.advance(false); continue; }
                if lookahead == 37 { state = 150; lexer.advance(false); continue; }
                if lookahead == 40 { state = 138; lexer.advance(false); continue; }
                if lookahead == 41 { state = 135; lexer.advance(false); continue; }
                if lookahead == 42 { state = 156; lexer.advance(false); continue; }
                if lookahead == 43 { state = 154; lexer.advance(false); continue; }
                if lookahead == 47 { state = 155; lexer.advance(false); continue; }
                if lookahead == 60 { state = 151; lexer.advance(false); continue; }
                if lookahead == 63 { state = 152; lexer.advance(false); continue; }
                if lookahead == 64 { state = 149; lexer.advance(false); continue; }
                if lookahead == 92 { state = 144; lexer.advance(false); continue; }
                if lookahead == 94 { state = 153; lexer.advance(false); continue; }
                if lookahead == 123 { state = 141; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 144; lexer.advance(false); continue; }
                if lookahead != 0 { state = 146; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if lookahead == 10 { state = 221; lexer.advance(false); continue; }
                if lookahead == 13 { state = 222; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 226; lexer.advance(false); continue; }
                if lookahead != 0 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            20 => {
                if lookahead == 10 { state = 56; lexer.advance(true); continue; }
                if lookahead == 13 { state = 56; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 101; lexer.advance(false); continue; }
                if set_contains(&sym_word_character_set_1, lookahead) { state = 199; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                if lookahead == 10 { state = 55; lexer.advance(true); continue; }
                if lookahead == 13 { state = 55; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 101; lexer.advance(false); continue; }
                if set_contains(&sym_word_character_set_1, lookahead) { state = 199; lexer.advance(false); continue; }
                return result;
            }
            22 => {
                if lookahead == 10 { state = 66; lexer.advance(true); continue; }
                if lookahead == 13 { state = 66; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 101; lexer.advance(false); continue; }
                if set_contains(&sym_word_character_set_1, lookahead) { state = 199; lexer.advance(false); continue; }
                return result;
            }
            23 => {
                if lookahead == 10 { state = 64; lexer.advance(true); continue; }
                if lookahead == 13 { state = 64; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 101; lexer.advance(false); continue; }
                if set_contains(&sym_word_character_set_1, lookahead) { state = 199; lexer.advance(false); continue; }
                return result;
            }
            24 => {
                if lookahead == 10 { state = 61; lexer.advance(true); continue; }
                if lookahead == 13 { state = 61; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 101; lexer.advance(false); continue; }
                if set_contains(&sym_word_character_set_1, lookahead) { state = 199; lexer.advance(false); continue; }
                return result;
            }
            25 => {
                if lookahead == 10 { state = 67; lexer.advance(true); continue; }
                if lookahead == 13 { state = 67; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 101; lexer.advance(false); continue; }
                if set_contains(&sym_word_character_set_1, lookahead) { state = 199; lexer.advance(false); continue; }
                return result;
            }
            26 => {
                if lookahead == 10 { state = 53; lexer.advance(true); continue; }
                if lookahead == 13 { state = 53; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 101; lexer.advance(false); continue; }
                if set_contains(&sym_word_character_set_1, lookahead) { state = 199; lexer.advance(false); continue; }
                return result;
            }
            27 => {
                if lookahead == 10 { state = 87; lexer.advance(true); continue; }
                if lookahead == 13 { state = 87; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 213; lexer.advance(false); continue; }
                if lookahead != 0 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            28 => {
                if lookahead == 10 { state = 2; lexer.advance(true); continue; }
                if lookahead == 13 { state = 2; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 213; lexer.advance(false); continue; }
                if lookahead != 0 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                if lookahead == 10 { state = 69; lexer.advance(true); continue; }
                if lookahead == 13 { state = 69; lexer.advance(true); continue; }
                return result;
            }
            30 => {
                if lookahead == 10 { state = 73; lexer.advance(true); continue; }
                if lookahead == 13 { state = 73; lexer.advance(true); continue; }
                return result;
            }
            31 => {
                if lookahead == 10 { state = 68; lexer.advance(true); continue; }
                if lookahead == 13 { state = 68; lexer.advance(true); continue; }
                if lookahead != 0 { state = 173; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                if lookahead == 10 { state = 91; lexer.advance(true); continue; }
                if lookahead == 13 { state = 91; lexer.advance(true); continue; }
                if lookahead != 0 { state = 173; lexer.advance(false); continue; }
                return result;
            }
            33 => {
                if lookahead == 10 { state = 77; lexer.advance(true); continue; }
                if lookahead == 13 { state = 77; lexer.advance(true); continue; }
                return result;
            }
            34 => {
                if lookahead == 10 { state = 77; lexer.advance(true); continue; }
                if lookahead == 35 { state = 144; lexer.advance(false); continue; }
                if lookahead == 37 { state = 150; lexer.advance(false); continue; }
                if lookahead == 40 { state = 138; lexer.advance(false); continue; }
                if lookahead == 42 { state = 156; lexer.advance(false); continue; }
                if lookahead == 43 { state = 154; lexer.advance(false); continue; }
                if lookahead == 47 { state = 155; lexer.advance(false); continue; }
                if lookahead == 60 { state = 151; lexer.advance(false); continue; }
                if lookahead == 63 { state = 152; lexer.advance(false); continue; }
                if lookahead == 64 { state = 149; lexer.advance(false); continue; }
                if lookahead == 92 { state = 144; lexer.advance(false); continue; }
                if lookahead == 94 { state = 153; lexer.advance(false); continue; }
                if lookahead == 123 { state = 140; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 144; lexer.advance(false); continue; }
                if lookahead != 0 { state = 144; lexer.advance(false); continue; }
                return result;
            }
            35 => {
                if lookahead == 10 { state = 76; lexer.advance(true); continue; }
                if lookahead == 13 { state = 76; lexer.advance(true); continue; }
                return result;
            }
            36 => {
                if let Some(next) = advance_map(&[
                    (10, 203), (13, 203), (35, 229), (92, 37), (101, 41), (9, 36), (11, 36), (12, 36),
                    (32, 36),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead != 0 { state = 42; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                if lookahead == 10 { state = 203; lexer.advance(false); continue; }
                if lookahead == 13 { state = 203; lexer.advance(false); continue; }
                if lookahead != 0 { state = 42; lexer.advance(false); continue; }
                return result;
            }
            38 => {
                if lookahead == 10 { state = 206; lexer.advance(false); continue; }
                if lookahead == 13 { state = 205; lexer.advance(false); continue; }
                if lookahead == 100 { state = 39; lexer.advance(false); continue; }
                if lookahead != 0 { state = 42; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                if lookahead == 10 { state = 206; lexer.advance(false); continue; }
                if lookahead == 13 { state = 205; lexer.advance(false); continue; }
                if lookahead == 101 { state = 40; lexer.advance(false); continue; }
                if lookahead != 0 { state = 42; lexer.advance(false); continue; }
                return result;
            }
            40 => {
                if lookahead == 10 { state = 206; lexer.advance(false); continue; }
                if lookahead == 13 { state = 205; lexer.advance(false); continue; }
                if lookahead == 102 { state = 130; lexer.advance(false); continue; }
                if lookahead != 0 { state = 42; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                if lookahead == 10 { state = 206; lexer.advance(false); continue; }
                if lookahead == 13 { state = 205; lexer.advance(false); continue; }
                if lookahead == 110 { state = 38; lexer.advance(false); continue; }
                if lookahead != 0 { state = 42; lexer.advance(false); continue; }
                return result;
            }
            42 => {
                if lookahead == 10 { state = 206; lexer.advance(false); continue; }
                if lookahead == 13 { state = 205; lexer.advance(false); continue; }
                if lookahead != 0 { state = 42; lexer.advance(false); continue; }
                return result;
            }
            43 => {
                if lookahead == 10 { state = 79; lexer.advance(true); continue; }
                if lookahead == 13 { state = 79; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 101; lexer.advance(false); continue; }
                if set_contains(&sym_word_character_set_1, lookahead) { state = 199; lexer.advance(false); continue; }
                return result;
            }
            44 => {
                if lookahead == 13 { state = 228; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 226; lexer.advance(false); continue; }
                if lookahead != 0 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            45 => {
                if let Some(next) = advance_map(&[
                    (33, 95), (34, 168), (35, 233), (36, 136), (37, 157), (38, 93), (39, 169), (40, 132),
                    (41, 135), (42, 162), (43, 122), (44, 133), (45, 121), (46, 185), (47, 161), (58, 111),
                    (59, 119), (60, 158), (61, 123), (63, 159), (64, 120), (92, 3), (94, 160), (101, 196),
                    (124, 118), (125, 143),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 45; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            46 => {
                if let Some(next) = advance_map(&[
                    (33, 95), (34, 168), (35, 233), (36, 136), (38, 93), (39, 169), (40, 138), (41, 200),
                    (43, 174), (58, 111), (61, 123), (63, 175), (92, 9), (9, 117), (32, 117),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 10 <= lookahead && lookahead <= 13 { state = 47; lexer.advance(true); continue; }
                if 37 <= lookahead && lookahead <= 42 || 45 <= lookahead && lookahead <= 57 || 64 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                if let Some(next) = advance_map(&[
                    (33, 95), (34, 168), (35, 233), (36, 136), (38, 93), (39, 169), (43, 174), (58, 111),
                    (61, 123), (63, 175), (92, 7),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 47; lexer.advance(true); continue; }
                if lookahead == 37 || lookahead == 42 || 45 <= lookahead && lookahead <= 57 || 64 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            48 => {
                if let Some(next) = advance_map(&[
                    (34, 168), (35, 233), (36, 136), (37, 157), (39, 169), (41, 200), (42, 162), (43, 122),
                    (47, 161), (60, 158), (63, 159), (64, 120), (92, 6), (94, 160),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 49; lexer.advance(true); continue; }
                if 45 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                if let Some(next) = advance_map(&[
                    (34, 168), (35, 233), (36, 136), (37, 157), (39, 169), (42, 162), (43, 122), (47, 161),
                    (60, 158), (63, 159), (64, 120), (92, 6), (94, 160),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 49; lexer.advance(true); continue; }
                if 45 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            50 => {
                if let Some(next) = advance_map(&[
                    (34, 168), (35, 233), (36, 136), (38, 93), (39, 169), (40, 138), (41, 200), (58, 112),
                    (92, 9), (9, 117), (32, 117),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 10 <= lookahead && lookahead <= 13 { state = 56; lexer.advance(true); continue; }
                if 37 <= lookahead && lookahead <= 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            51 => {
                if let Some(next) = advance_map(&[
                    (34, 168), (35, 233), (36, 136), (38, 93), (39, 169), (40, 138), (58, 112), (59, 119),
                    (92, 9), (124, 118), (9, 117), (32, 117), (10, 109), (13, 109),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 11 || lookahead == 12 { state = 55; lexer.advance(true); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            52 => {
                if let Some(next) = advance_map(&[
                    (34, 168), (35, 233), (36, 136), (38, 93), (39, 169), (40, 132), (41, 200), (58, 112),
                    (92, 26),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 53; lexer.advance(true); continue; }
                if 37 <= lookahead && lookahead <= 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                if let Some(next) = advance_map(&[
                    (34, 168), (35, 233), (36, 136), (38, 93), (39, 169), (40, 132), (58, 112), (92, 26),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 53; lexer.advance(true); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                if let Some(next) = advance_map(&[
                    (34, 168), (35, 233), (36, 136), (38, 93), (39, 169), (41, 135), (44, 133), (45, 194),
                    (46, 185), (58, 112), (92, 4),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 54; lexer.advance(true); continue; }
                if lookahead == 37 || 42 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            55 => {
                if let Some(next) = advance_map(&[
                    (34, 168), (35, 233), (36, 136), (38, 93), (39, 169), (58, 112), (59, 119), (92, 21),
                    (124, 118),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 55; lexer.advance(true); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            56 => {
                if lookahead == 34 { state = 168; lexer.advance(false); continue; }
                if lookahead == 35 { state = 233; lexer.advance(false); continue; }
                if lookahead == 36 { state = 136; lexer.advance(false); continue; }
                if lookahead == 38 { state = 93; lexer.advance(false); continue; }
                if lookahead == 39 { state = 169; lexer.advance(false); continue; }
                if lookahead == 58 { state = 112; lexer.advance(false); continue; }
                if lookahead == 92 { state = 20; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 56; lexer.advance(true); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            57 => {
                if let Some(next) = advance_map(&[
                    (34, 168), (35, 233), (36, 136), (39, 169), (40, 138), (41, 135), (44, 133), (58, 110),
                    (61, 123), (92, 24), (125, 143),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 61; lexer.advance(true); continue; }
                if lookahead == 37 || 42 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            58 => {
                if let Some(next) = advance_map(&[
                    (34, 168), (35, 233), (36, 136), (39, 169), (40, 138), (43, 174), (58, 113), (59, 119),
                    (61, 123), (63, 175), (92, 9), (124, 118), (9, 117), (32, 117), (10, 109), (13, 109),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 11 || lookahead == 12 { state = 63; lexer.advance(true); continue; }
                if lookahead == 37 || lookahead == 42 || 45 <= lookahead && lookahead <= 57 || 64 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                if let Some(next) = advance_map(&[
                    (34, 168), (35, 233), (36, 136), (39, 169), (40, 138), (58, 110), (59, 119), (92, 23),
                    (124, 118), (10, 109), (13, 109),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 64; lexer.advance(true); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            60 => {
                if let Some(next) = advance_map(&[
                    (34, 168), (35, 233), (36, 136), (39, 169), (40, 138), (58, 165), (59, 167), (92, 25),
                    (10, 109), (13, 109),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 67; lexer.advance(true); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            61 => {
                if let Some(next) = advance_map(&[
                    (34, 168), (35, 233), (36, 136), (39, 169), (41, 135), (44, 133), (58, 110), (61, 123),
                    (92, 24), (125, 143),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 61; lexer.advance(true); continue; }
                if lookahead == 37 || 42 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            62 => {
                if let Some(next) = advance_map(&[
                    (34, 168), (35, 233), (36, 136), (39, 169), (43, 174), (58, 113), (59, 119), (61, 123),
                    (63, 175), (92, 13), (124, 118), (10, 109), (13, 109),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 63; lexer.advance(true); continue; }
                if lookahead == 37 || lookahead == 42 || 45 <= lookahead && lookahead <= 57 || 64 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                if let Some(next) = advance_map(&[
                    (34, 168), (35, 233), (36, 136), (39, 169), (43, 174), (58, 113), (59, 119), (61, 123),
                    (63, 175), (92, 13), (124, 118),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 63; lexer.advance(true); continue; }
                if lookahead == 37 || lookahead == 42 || 45 <= lookahead && lookahead <= 57 || 64 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            64 => {
                if let Some(next) = advance_map(&[
                    (34, 168), (35, 233), (36, 136), (39, 169), (58, 110), (59, 119), (92, 23), (124, 118),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 64; lexer.advance(true); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            65 => {
                if let Some(next) = advance_map(&[
                    (34, 168), (35, 233), (36, 136), (39, 169), (59, 119), (92, 22), (124, 118), (9, 117),
                    (32, 117), (10, 109), (13, 109),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 11 || lookahead == 12 { state = 66; lexer.advance(true); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            66 => {
                if lookahead == 34 { state = 168; lexer.advance(false); continue; }
                if lookahead == 35 { state = 233; lexer.advance(false); continue; }
                if lookahead == 36 { state = 136; lexer.advance(false); continue; }
                if lookahead == 39 { state = 169; lexer.advance(false); continue; }
                if lookahead == 59 { state = 119; lexer.advance(false); continue; }
                if lookahead == 92 { state = 22; lexer.advance(false); continue; }
                if lookahead == 124 { state = 118; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 66; lexer.advance(true); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            67 => {
                if lookahead == 34 { state = 168; lexer.advance(false); continue; }
                if lookahead == 35 { state = 233; lexer.advance(false); continue; }
                if lookahead == 36 { state = 136; lexer.advance(false); continue; }
                if lookahead == 39 { state = 169; lexer.advance(false); continue; }
                if lookahead == 92 { state = 25; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 67; lexer.advance(true); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            68 => {
                if lookahead == 34 { state = 168; lexer.advance(false); continue; }
                if lookahead == 35 { state = 172; lexer.advance(false); continue; }
                if lookahead == 36 { state = 136; lexer.advance(false); continue; }
                if lookahead == 92 { state = 31; lexer.advance(false); continue; }
                if lookahead == 10 || lookahead == 13 { state = 68; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 170; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 39 { state = 173; lexer.advance(false); continue; }
                return result;
            }
            69 => {
                if lookahead == 35 { state = 233; lexer.advance(false); continue; }
                if lookahead == 36 { state = 136; lexer.advance(false); continue; }
                if lookahead == 41 { state = 135; lexer.advance(false); continue; }
                if lookahead == 44 { state = 133; lexer.advance(false); continue; }
                if lookahead == 47 { state = 92; lexer.advance(false); continue; }
                if lookahead == 92 { state = 29; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 69; lexer.advance(true); continue; }
                return result;
            }
            70 => {
                if lookahead == 35 { state = 233; lexer.advance(false); continue; }
                if lookahead == 36 { state = 136; lexer.advance(false); continue; }
                if lookahead == 47 { state = 92; lexer.advance(false); continue; }
                if lookahead == 92 { state = 8; lexer.advance(false); continue; }
                if lookahead == 10 || lookahead == 13 { state = 109; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 73; lexer.advance(true); continue; }
                return result;
            }
            71 => {
                if lookahead == 35 { state = 233; lexer.advance(false); continue; }
                if lookahead == 36 { state = 136; lexer.advance(false); continue; }
                if lookahead == 47 { state = 92; lexer.advance(false); continue; }
                if lookahead == 92 { state = 8; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 73; lexer.advance(true); continue; }
                return result;
            }
            72 => {
                if lookahead == 35 { state = 233; lexer.advance(false); continue; }
                if lookahead == 36 { state = 136; lexer.advance(false); continue; }
                if lookahead == 47 { state = 92; lexer.advance(false); continue; }
                if lookahead == 92 { state = 30; lexer.advance(true); continue; }
                if lookahead == 10 || lookahead == 13 { state = 109; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 73; lexer.advance(true); continue; }
                return result;
            }
            73 => {
                if lookahead == 35 { state = 233; lexer.advance(false); continue; }
                if lookahead == 36 { state = 136; lexer.advance(false); continue; }
                if lookahead == 47 { state = 92; lexer.advance(false); continue; }
                if lookahead == 92 { state = 30; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 73; lexer.advance(true); continue; }
                return result;
            }
            74 => {
                if lookahead == 35 { state = 233; lexer.advance(false); continue; }
                if lookahead == 43 { state = 97; lexer.advance(false); continue; }
                if lookahead == 58 { state = 94; lexer.advance(false); continue; }
                if lookahead == 61 { state = 123; lexer.advance(false); continue; }
                if lookahead == 63 { state = 98; lexer.advance(false); continue; }
                if lookahead == 92 { state = 35; lexer.advance(true); continue; }
                if lookahead == 9 || lookahead == 32 { state = 117; lexer.advance(false); continue; }
                if lookahead == 10 || lookahead == 13 { state = 109; lexer.advance(false); continue; }
                if lookahead == 11 || lookahead == 12 { state = 76; lexer.advance(true); continue; }
                return result;
            }
            75 => {
                if lookahead == 35 { state = 233; lexer.advance(false); continue; }
                if lookahead == 43 { state = 97; lexer.advance(false); continue; }
                if lookahead == 58 { state = 94; lexer.advance(false); continue; }
                if lookahead == 61 { state = 123; lexer.advance(false); continue; }
                if lookahead == 63 { state = 98; lexer.advance(false); continue; }
                if lookahead == 92 { state = 35; lexer.advance(true); continue; }
                if lookahead == 9 || lookahead == 32 { state = 117; lexer.advance(false); continue; }
                if 10 <= lookahead && lookahead <= 13 { state = 76; lexer.advance(true); continue; }
                return result;
            }
            76 => {
                if lookahead == 35 { state = 233; lexer.advance(false); continue; }
                if lookahead == 43 { state = 97; lexer.advance(false); continue; }
                if lookahead == 58 { state = 94; lexer.advance(false); continue; }
                if lookahead == 61 { state = 123; lexer.advance(false); continue; }
                if lookahead == 63 { state = 98; lexer.advance(false); continue; }
                if lookahead == 92 { state = 35; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 76; lexer.advance(true); continue; }
                return result;
            }
            77 => {
                if lookahead == 35 { state = 233; lexer.advance(false); continue; }
                if lookahead == 92 { state = 33; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 77; lexer.advance(true); continue; }
                return result;
            }
            78 => {
                if lookahead == 35 { state = 233; lexer.advance(false); continue; }
                if lookahead == 92 { state = 43; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 117; lexer.advance(false); continue; }
                if 10 <= lookahead && lookahead <= 13 { state = 79; lexer.advance(true); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            79 => {
                if lookahead == 35 { state = 233; lexer.advance(false); continue; }
                if lookahead == 92 { state = 43; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 79; lexer.advance(true); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            80 => {
                if lookahead == 35 { state = 225; lexer.advance(false); continue; }
                if lookahead == 36 { state = 136; lexer.advance(false); continue; }
                if lookahead == 41 { state = 135; lexer.advance(false); continue; }
                if lookahead == 44 { state = 134; lexer.advance(false); continue; }
                if lookahead == 47 { state = 223; lexer.advance(false); continue; }
                if lookahead == 92 { state = 15; lexer.advance(false); continue; }
                if lookahead == 10 || lookahead == 13 { state = 80; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 219; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 40 && lookahead != 41 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            81 => {
                if lookahead == 35 { state = 225; lexer.advance(false); continue; }
                if lookahead == 36 { state = 136; lexer.advance(false); continue; }
                if lookahead == 41 { state = 135; lexer.advance(false); continue; }
                if lookahead == 47 { state = 223; lexer.advance(false); continue; }
                if lookahead == 92 { state = 19; lexer.advance(false); continue; }
                if lookahead == 10 || lookahead == 13 { state = 81; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 221; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 40 && lookahead != 41 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            82 => {
                if let Some(next) = advance_map(&[
                    (35, 225), (36, 136), (47, 223), (92, 19), (9, 117), (32, 117), (10, 109), (13, 109),
                    (11, 221), (12, 221),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 40 && lookahead != 41 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            83 => {
                if lookahead == 35 { state = 225; lexer.advance(false); continue; }
                if lookahead == 36 { state = 136; lexer.advance(false); continue; }
                if lookahead == 47 { state = 223; lexer.advance(false); continue; }
                if lookahead == 92 { state = 19; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 117; lexer.advance(false); continue; }
                if lookahead == 10 || lookahead == 13 { state = 85; lexer.advance(true); continue; }
                if lookahead == 11 || lookahead == 12 { state = 221; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 40 && lookahead != 41 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            84 => {
                if let Some(next) = advance_map(&[
                    (35, 225), (36, 136), (47, 223), (92, 19), (10, 109), (13, 109), (9, 221), (11, 221),
                    (12, 221), (32, 221),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 40 && lookahead != 41 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            85 => {
                if lookahead == 35 { state = 225; lexer.advance(false); continue; }
                if lookahead == 36 { state = 136; lexer.advance(false); continue; }
                if lookahead == 47 { state = 223; lexer.advance(false); continue; }
                if lookahead == 92 { state = 19; lexer.advance(false); continue; }
                if lookahead == 10 || lookahead == 13 { state = 85; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 221; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 40 && lookahead != 41 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            86 => {
                if let Some(next) = advance_map(&[
                    (35, 212), (36, 136), (43, 122), (45, 121), (47, 210), (64, 120), (92, 27), (10, 108),
                    (13, 108), (9, 208), (11, 208), (12, 208), (32, 208),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead != 0 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            87 => {
                if lookahead == 35 { state = 212; lexer.advance(false); continue; }
                if lookahead == 36 { state = 136; lexer.advance(false); continue; }
                if lookahead == 43 { state = 122; lexer.advance(false); continue; }
                if lookahead == 45 { state = 121; lexer.advance(false); continue; }
                if lookahead == 47 { state = 210; lexer.advance(false); continue; }
                if lookahead == 64 { state = 120; lexer.advance(false); continue; }
                if lookahead == 92 { state = 27; lexer.advance(false); continue; }
                if lookahead == 10 || lookahead == 13 { state = 87; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 208; lexer.advance(false); continue; }
                if lookahead != 0 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            88 => {
                if lookahead == 35 { state = 212; lexer.advance(false); continue; }
                if lookahead == 36 { state = 136; lexer.advance(false); continue; }
                if lookahead == 47 { state = 210; lexer.advance(false); continue; }
                if lookahead == 92 { state = 17; lexer.advance(false); continue; }
                if lookahead == 10 || lookahead == 13 { state = 88; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 209; lexer.advance(false); continue; }
                if lookahead != 0 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            89 => {
                if let Some(next) = advance_map(&[
                    (35, 212), (36, 136), (47, 210), (92, 10), (10, 109), (13, 109), (9, 209), (11, 209),
                    (12, 209), (32, 209),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead != 0 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            90 => {
                if lookahead == 35 { state = 212; lexer.advance(false); continue; }
                if lookahead == 36 { state = 136; lexer.advance(false); continue; }
                if lookahead == 47 { state = 210; lexer.advance(false); continue; }
                if lookahead == 92 { state = 10; lexer.advance(false); continue; }
                if lookahead == 10 || lookahead == 13 { state = 88; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 209; lexer.advance(false); continue; }
                if lookahead != 0 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            91 => {
                if lookahead == 35 { state = 172; lexer.advance(false); continue; }
                if lookahead == 36 { state = 136; lexer.advance(false); continue; }
                if lookahead == 39 { state = 169; lexer.advance(false); continue; }
                if lookahead == 92 { state = 32; lexer.advance(false); continue; }
                if lookahead == 10 || lookahead == 13 { state = 91; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 171; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 34 || 36 < lookahead) { state = 173; lexer.advance(false); continue; }
                return result;
            }
            92 => {
                if lookahead == 47 { state = 215; lexer.advance(false); continue; }
                return result;
            }
            93 => {
                if lookahead == 58 { state = 114; lexer.advance(false); continue; }
                return result;
            }
            94 => {
                if lookahead == 58 { state = 96; lexer.advance(false); continue; }
                if lookahead == 61 { state = 124; lexer.advance(false); continue; }
                return result;
            }
            95 => {
                if lookahead == 61 { state = 129; lexer.advance(false); continue; }
                return result;
            }
            96 => {
                if lookahead == 61 { state = 125; lexer.advance(false); continue; }
                return result;
            }
            97 => {
                if lookahead == 61 { state = 127; lexer.advance(false); continue; }
                return result;
            }
            98 => {
                if lookahead == 61 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            99 => {
                if 48 <= lookahead && lookahead <= 57 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            100 => {
                if 48 <= lookahead && lookahead <= 57 { state = 101; lexer.advance(false); continue; }
                if set_contains(&sym_word_character_set_1, lookahead) { state = 199; lexer.advance(false); continue; }
                return result;
            }
            101 => {
                if 48 <= lookahead && lookahead <= 57 { state = 99; lexer.advance(false); continue; }
                return result;
            }
            102 => {
                if 48 <= lookahead && lookahead <= 57 { state = 213; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            103 => {
                if lookahead != 0 && lookahead != 10 && lookahead != 13 { state = 173; lexer.advance(false); continue; }
                return result;
            }
            104 => {
                if eof { state = 107; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (9, 201), (34, 168), (35, 233), (36, 136), (39, 169), (45, 194), (46, 185), (92, 5),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 10 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 104; lexer.advance(true); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 47 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            105 => {
                if eof { state = 107; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 95), (34, 168), (35, 233), (36, 136), (37, 157), (38, 93), (39, 169), (40, 132),
                    (41, 135), (42, 162), (43, 122), (44, 133), (45, 121), (46, 185), (47, 161), (58, 111),
                    (59, 119), (60, 158), (61, 123), (63, 159), (64, 120), (92, 3), (94, 160), (101, 196),
                    (124, 118), (125, 143),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 105; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            106 => {
                if eof { state = 107; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (34, 168), (35, 233), (36, 136), (38, 93), (39, 169), (41, 135), (44, 133), (45, 194),
                    (46, 185), (58, 112), (92, 4),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 106; lexer.advance(true); continue; }
                if lookahead == 37 || 42 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            107 => {
                result = true; lexer.set_result_symbol(ts_builtin_sym_end); lexer.mark_end();
                return result;
            }
            108 => {
                result = true; lexer.set_result_symbol(aux_sym__thing_token1); lexer.mark_end();
                if lookahead == 43 { state = 122; lexer.advance(false); continue; }
                if lookahead == 45 { state = 121; lexer.advance(false); continue; }
                if lookahead == 64 { state = 120; lexer.advance(false); continue; }
                if lookahead == 10 || lookahead == 13 { state = 108; lexer.advance(false); continue; }
                return result;
            }
            109 => {
                result = true; lexer.set_result_symbol(aux_sym__thing_token1); lexer.mark_end();
                if lookahead == 10 || lookahead == 13 { state = 109; lexer.advance(false); continue; }
                return result;
            }
            110 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON); lexer.mark_end();
                return result;
            }
            111 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON); lexer.mark_end();
                if lookahead == 58 { state = 116; lexer.advance(false); continue; }
                if lookahead == 61 { state = 124; lexer.advance(false); continue; }
                return result;
            }
            112 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON); lexer.mark_end();
                if lookahead == 58 { state = 115; lexer.advance(false); continue; }
                return result;
            }
            113 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON); lexer.mark_end();
                if lookahead == 58 { state = 96; lexer.advance(false); continue; }
                if lookahead == 61 { state = 124; lexer.advance(false); continue; }
                return result;
            }
            114 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP_COLON); lexer.mark_end();
                return result;
            }
            115 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON_COLON); lexer.mark_end();
                return result;
            }
            116 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON_COLON); lexer.mark_end();
                if lookahead == 61 { state = 125; lexer.advance(false); continue; }
                return result;
            }
            117 => {
                result = true; lexer.set_result_symbol(aux_sym__ordinary_rule_token1); lexer.mark_end();
                if lookahead == 9 || lookahead == 32 { state = 117; lexer.advance(false); continue; }
                return result;
            }
            118 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                return result;
            }
            119 => {
                result = true; lexer.set_result_symbol(anon_sym_SEMI); lexer.mark_end();
                return result;
            }
            120 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                return result;
            }
            121 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                return result;
            }
            122 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                return result;
            }
            123 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                return result;
            }
            124 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON_EQ); lexer.mark_end();
                return result;
            }
            125 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON_COLON_EQ); lexer.mark_end();
                return result;
            }
            126 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK_EQ); lexer.mark_end();
                return result;
            }
            127 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS_EQ); lexer.mark_end();
                return result;
            }
            128 => {
                result = true; lexer.set_result_symbol(anon_sym_DOTRECIPEPREFIX); lexer.mark_end();
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            129 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG_EQ); lexer.mark_end();
                return result;
            }
            130 => {
                result = true; lexer.set_result_symbol(anon_sym_endef); lexer.mark_end();
                return result;
            }
            131 => {
                result = true; lexer.set_result_symbol(anon_sym_DASHinclude); lexer.mark_end();
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            132 => {
                result = true; lexer.set_result_symbol(anon_sym_LPAREN); lexer.mark_end();
                return result;
            }
            133 => {
                result = true; lexer.set_result_symbol(anon_sym_COMMA); lexer.mark_end();
                return result;
            }
            134 => {
                result = true; lexer.set_result_symbol(anon_sym_COMMA); lexer.mark_end();
                if lookahead == 92 { state = 44; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 36 && lookahead != 40 && lookahead != 41 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            135 => {
                result = true; lexer.set_result_symbol(anon_sym_RPAREN); lexer.mark_end();
                return result;
            }
            136 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR); lexer.mark_end();
                if lookahead == 36 { state = 137; lexer.advance(false); continue; }
                return result;
            }
            137 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR_DOLLAR); lexer.mark_end();
                return result;
            }
            138 => {
                result = true; lexer.set_result_symbol(anon_sym_LPAREN2); lexer.mark_end();
                return result;
            }
            139 => {
                result = true; lexer.set_result_symbol(anon_sym_LPAREN2); lexer.mark_end();
                if lookahead == 92 { state = 102; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 36 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            140 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACE); lexer.mark_end();
                return result;
            }
            141 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACE); lexer.mark_end();
                if lookahead == 92 { state = 44; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 36 && lookahead != 40 && lookahead != 41 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            142 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACE); lexer.mark_end();
                if lookahead == 92 { state = 102; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 36 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            143 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACE); lexer.mark_end();
                return result;
            }
            144 => {
                result = true; lexer.set_result_symbol(aux_sym_variable_reference_token1); lexer.mark_end();
                return result;
            }
            145 => {
                result = true; lexer.set_result_symbol(aux_sym_variable_reference_token1); lexer.mark_end();
                if lookahead == 92 { state = 230; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 36 && lookahead != 40 && lookahead != 41 { state = 225; lexer.advance(false); continue; }
                return result;
            }
            146 => {
                result = true; lexer.set_result_symbol(aux_sym_variable_reference_token1); lexer.mark_end();
                if lookahead == 92 { state = 44; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 36 && lookahead != 40 && lookahead != 41 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            147 => {
                result = true; lexer.set_result_symbol(aux_sym_variable_reference_token1); lexer.mark_end();
                if lookahead == 92 { state = 231; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 36 { state = 212; lexer.advance(false); continue; }
                return result;
            }
            148 => {
                result = true; lexer.set_result_symbol(aux_sym_variable_reference_token1); lexer.mark_end();
                if lookahead == 92 { state = 102; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 36 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            149 => {
                result = true; lexer.set_result_symbol(anon_sym_AT2); lexer.mark_end();
                return result;
            }
            150 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT); lexer.mark_end();
                return result;
            }
            151 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                return result;
            }
            152 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK); lexer.mark_end();
                return result;
            }
            153 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET); lexer.mark_end();
                return result;
            }
            154 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS2); lexer.mark_end();
                return result;
            }
            155 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH); lexer.mark_end();
                return result;
            }
            156 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                return result;
            }
            157 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT2); lexer.mark_end();
                return result;
            }
            158 => {
                result = true; lexer.set_result_symbol(anon_sym_LT2); lexer.mark_end();
                return result;
            }
            159 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK2); lexer.mark_end();
                return result;
            }
            160 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET2); lexer.mark_end();
                return result;
            }
            161 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH2); lexer.mark_end();
                return result;
            }
            162 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR2); lexer.mark_end();
                return result;
            }
            163 => {
                result = true; lexer.set_result_symbol(aux_sym_list_token1); lexer.mark_end();
                return result;
            }
            164 => {
                result = true; lexer.set_result_symbol(aux_sym_list_token1); lexer.mark_end();
                if lookahead == 10 { state = 163; lexer.advance(false); continue; }
                return result;
            }
            165 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON2); lexer.mark_end();
                return result;
            }
            166 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON2); lexer.mark_end();
                if lookahead == 58 { state = 116; lexer.advance(false); continue; }
                if lookahead == 61 { state = 124; lexer.advance(false); continue; }
                return result;
            }
            167 => {
                result = true; lexer.set_result_symbol(anon_sym_SEMI2); lexer.mark_end();
                return result;
            }
            168 => {
                result = true; lexer.set_result_symbol(anon_sym_DQUOTE); lexer.mark_end();
                return result;
            }
            169 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTE); lexer.mark_end();
                return result;
            }
            170 => {
                result = true; lexer.set_result_symbol(aux_sym__string_token1); lexer.mark_end();
                if lookahead == 34 { state = 168; lexer.advance(false); continue; }
                if lookahead == 35 { state = 172; lexer.advance(false); continue; }
                if lookahead == 36 { state = 136; lexer.advance(false); continue; }
                if lookahead == 92 { state = 31; lexer.advance(false); continue; }
                if lookahead == 10 || lookahead == 13 { state = 68; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 170; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 39 { state = 173; lexer.advance(false); continue; }
                return result;
            }
            171 => {
                result = true; lexer.set_result_symbol(aux_sym__string_token1); lexer.mark_end();
                if lookahead == 35 { state = 172; lexer.advance(false); continue; }
                if lookahead == 36 { state = 136; lexer.advance(false); continue; }
                if lookahead == 39 { state = 169; lexer.advance(false); continue; }
                if lookahead == 92 { state = 32; lexer.advance(false); continue; }
                if lookahead == 10 || lookahead == 13 { state = 91; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 171; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 34 || 36 < lookahead) { state = 173; lexer.advance(false); continue; }
                return result;
            }
            172 => {
                result = true; lexer.set_result_symbol(aux_sym__string_token1); lexer.mark_end();
                if lookahead == 92 { state = 232; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 34 || lookahead == 36 || lookahead == 39 { state = 233; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 172; lexer.advance(false); continue; }
                return result;
            }
            173 => {
                result = true; lexer.set_result_symbol(aux_sym__string_token1); lexer.mark_end();
                if lookahead == 92 { state = 103; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 34 && lookahead != 36 && lookahead != 39 { state = 173; lexer.advance(false); continue; }
                return result;
            }
            174 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 61 { state = 127; lexer.advance(false); continue; }
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            175 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 61 { state = 126; lexer.advance(false); continue; }
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            176 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 67 { state = 181; lexer.advance(false); continue; }
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            177 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 69 { state = 176; lexer.advance(false); continue; }
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            178 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 69 { state = 180; lexer.advance(false); continue; }
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            179 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 69 { state = 184; lexer.advance(false); continue; }
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            180 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 70 { state = 182; lexer.advance(false); continue; }
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            181 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 73 { state = 183; lexer.advance(false); continue; }
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            182 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 73 { state = 187; lexer.advance(false); continue; }
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            183 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 80 { state = 179; lexer.advance(false); continue; }
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            184 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 80 { state = 186; lexer.advance(false); continue; }
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            185 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 82 { state = 177; lexer.advance(false); continue; }
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            186 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 82 { state = 178; lexer.advance(false); continue; }
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            187 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 88 { state = 128; lexer.advance(false); continue; }
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            188 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                if lookahead == 99 { state = 195; lexer.advance(false); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            189 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                if lookahead == 100 { state = 191; lexer.advance(false); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            190 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                if lookahead == 100 { state = 192; lexer.advance(false); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            191 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                if lookahead == 101 { state = 193; lexer.advance(false); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            192 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                if lookahead == 101 { state = 131; lexer.advance(false); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            193 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                if lookahead == 102 { state = 130; lexer.advance(false); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            194 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                if lookahead == 105 { state = 197; lexer.advance(false); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            195 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                if lookahead == 108 { state = 198; lexer.advance(false); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            196 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                if lookahead == 110 { state = 189; lexer.advance(false); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            197 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                if lookahead == 110 { state = 188; lexer.advance(false); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            198 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                if lookahead == 117 { state = 190; lexer.advance(false); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            199 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                if lookahead == 37 || lookahead == 42 || lookahead == 43 || 45 <= lookahead && lookahead <= 57 || 63 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            200 => {
                result = true; lexer.set_result_symbol(anon_sym_RPAREN2); lexer.mark_end();
                return result;
            }
            201 => {
                result = true; lexer.set_result_symbol(anon_sym_TAB); lexer.mark_end();
                if lookahead == 9 { state = 201; lexer.advance(false); continue; }
                if lookahead == 92 { state = 5; lexer.advance(false); continue; }
                return result;
            }
            202 => {
                result = true; lexer.set_result_symbol(anon_sym_TAB); lexer.mark_end();
                if lookahead == 9 { state = 202; lexer.advance(false); continue; }
                if lookahead == 92 { state = 28; lexer.advance(false); continue; }
                if lookahead == 11 || lookahead == 12 || lookahead == 32 { state = 207; lexer.advance(false); continue; }
                return result;
            }
            203 => {
                result = true; lexer.set_result_symbol(sym__rawline); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 203), (13, 203), (35, 229), (92, 37), (101, 41), (9, 36), (11, 36), (12, 36),
                    (32, 36),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead != 0 { state = 42; lexer.advance(false); continue; }
                return result;
            }
            204 => {
                result = true; lexer.set_result_symbol(sym__rawline); lexer.mark_end();
                if lookahead == 10 { state = 206; lexer.advance(false); continue; }
                if lookahead == 13 { state = 204; lexer.advance(false); continue; }
                if lookahead != 0 { state = 229; lexer.advance(false); continue; }
                return result;
            }
            205 => {
                result = true; lexer.set_result_symbol(sym__rawline); lexer.mark_end();
                if lookahead == 10 { state = 206; lexer.advance(false); continue; }
                if lookahead == 13 { state = 205; lexer.advance(false); continue; }
                if lookahead != 0 { state = 42; lexer.advance(false); continue; }
                return result;
            }
            206 => {
                result = true; lexer.set_result_symbol(sym__rawline); lexer.mark_end();
                if lookahead == 10 || lookahead == 13 { state = 206; lexer.advance(false); continue; }
                return result;
            }
            207 => {
                result = true; lexer.set_result_symbol(aux_sym__shell_text_without_split_token1); lexer.mark_end();
                if lookahead == 9 { state = 202; lexer.advance(false); continue; }
                if lookahead == 35 { state = 212; lexer.advance(false); continue; }
                if lookahead == 47 { state = 210; lexer.advance(false); continue; }
                if lookahead == 92 { state = 28; lexer.advance(false); continue; }
                if lookahead == 11 || lookahead == 12 || lookahead == 32 { state = 207; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 35 && lookahead != 36 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            208 => {
                result = true; lexer.set_result_symbol(aux_sym__shell_text_without_split_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (35, 212), (43, 122), (45, 121), (47, 210), (64, 120), (92, 27), (9, 208), (11, 208),
                    (12, 208), (32, 208),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 35 && lookahead != 36 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            209 => {
                result = true; lexer.set_result_symbol(aux_sym__shell_text_without_split_token1); lexer.mark_end();
                if lookahead == 35 { state = 212; lexer.advance(false); continue; }
                if lookahead == 47 { state = 210; lexer.advance(false); continue; }
                if lookahead == 92 { state = 17; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 11 || lookahead == 12 || lookahead == 32 { state = 209; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 35 && lookahead != 36 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            210 => {
                result = true; lexer.set_result_symbol(aux_sym__shell_text_without_split_token1); lexer.mark_end();
                if lookahead == 47 { state = 217; lexer.advance(false); continue; }
                if lookahead == 92 { state = 102; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 36 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            211 => {
                result = true; lexer.set_result_symbol(aux_sym__shell_text_without_split_token1); lexer.mark_end();
                if lookahead == 92 { state = 231; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 212; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 36 { state = 212; lexer.advance(false); continue; }
                return result;
            }
            212 => {
                result = true; lexer.set_result_symbol(aux_sym__shell_text_without_split_token1); lexer.mark_end();
                if lookahead == 92 { state = 231; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 36 { state = 212; lexer.advance(false); continue; }
                return result;
            }
            213 => {
                result = true; lexer.set_result_symbol(aux_sym__shell_text_without_split_token1); lexer.mark_end();
                if lookahead == 92 { state = 102; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 214; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 36 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            214 => {
                result = true; lexer.set_result_symbol(aux_sym__shell_text_without_split_token1); lexer.mark_end();
                if lookahead == 92 { state = 102; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 36 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            215 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_SLASH); lexer.mark_end();
                return result;
            }
            216 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_SLASH); lexer.mark_end();
                if lookahead == 92 { state = 44; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 36 && lookahead != 40 && lookahead != 41 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            217 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_SLASH); lexer.mark_end();
                if lookahead == 92 { state = 102; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 36 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            218 => {
                result = true; lexer.set_result_symbol(aux_sym_text_token1); lexer.mark_end();
                if lookahead == 10 { state = 227; lexer.advance(false); continue; }
                if lookahead == 92 { state = 230; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 13 && lookahead != 36 && lookahead != 40 && lookahead != 41 { state = 225; lexer.advance(false); continue; }
                return result;
            }
            219 => {
                result = true; lexer.set_result_symbol(aux_sym_text_token1); lexer.mark_end();
                if lookahead == 35 { state = 225; lexer.advance(false); continue; }
                if lookahead == 44 { state = 134; lexer.advance(false); continue; }
                if lookahead == 47 { state = 223; lexer.advance(false); continue; }
                if lookahead == 92 { state = 15; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 11 || lookahead == 12 || lookahead == 32 { state = 219; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 35 && lookahead != 36 && lookahead != 40 && lookahead != 41 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            220 => {
                result = true; lexer.set_result_symbol(aux_sym_text_token1); lexer.mark_end();
                if lookahead == 35 { state = 225; lexer.advance(false); continue; }
                if lookahead == 44 { state = 134; lexer.advance(false); continue; }
                if lookahead == 47 { state = 223; lexer.advance(false); continue; }
                if lookahead == 92 { state = 15; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 219; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 35 && lookahead != 36 && lookahead != 40 && lookahead != 41 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            221 => {
                result = true; lexer.set_result_symbol(aux_sym_text_token1); lexer.mark_end();
                if lookahead == 35 { state = 225; lexer.advance(false); continue; }
                if lookahead == 47 { state = 223; lexer.advance(false); continue; }
                if lookahead == 92 { state = 19; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 11 || lookahead == 12 || lookahead == 32 { state = 221; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 35 && lookahead != 36 && lookahead != 40 && lookahead != 41 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            222 => {
                result = true; lexer.set_result_symbol(aux_sym_text_token1); lexer.mark_end();
                if lookahead == 35 { state = 225; lexer.advance(false); continue; }
                if lookahead == 47 { state = 223; lexer.advance(false); continue; }
                if lookahead == 92 { state = 19; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 221; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 35 && lookahead != 36 && lookahead != 40 && lookahead != 41 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            223 => {
                result = true; lexer.set_result_symbol(aux_sym_text_token1); lexer.mark_end();
                if lookahead == 47 { state = 216; lexer.advance(false); continue; }
                if lookahead == 92 { state = 44; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 36 && lookahead != 40 && lookahead != 41 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            224 => {
                result = true; lexer.set_result_symbol(aux_sym_text_token1); lexer.mark_end();
                if lookahead == 92 { state = 230; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 225; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 36 && lookahead != 40 && lookahead != 41 { state = 225; lexer.advance(false); continue; }
                return result;
            }
            225 => {
                result = true; lexer.set_result_symbol(aux_sym_text_token1); lexer.mark_end();
                if lookahead == 92 { state = 230; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 36 && lookahead != 40 && lookahead != 41 { state = 225; lexer.advance(false); continue; }
                return result;
            }
            226 => {
                result = true; lexer.set_result_symbol(aux_sym_text_token1); lexer.mark_end();
                if lookahead == 92 { state = 44; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 227; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 36 && lookahead != 40 && lookahead != 41 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            227 => {
                result = true; lexer.set_result_symbol(aux_sym_text_token1); lexer.mark_end();
                if lookahead == 92 { state = 44; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 36 && lookahead != 40 && lookahead != 41 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            228 => {
                result = true; lexer.set_result_symbol(aux_sym_text_token1); lexer.mark_end();
                if lookahead == 92 { state = 44; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 13 && lookahead != 36 && lookahead != 40 && lookahead != 41 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            229 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 10 { state = 206; lexer.advance(false); continue; }
                if lookahead == 13 { state = 204; lexer.advance(false); continue; }
                if lookahead != 0 { state = 229; lexer.advance(false); continue; }
                return result;
            }
            230 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 10 { state = 227; lexer.advance(false); continue; }
                if lookahead == 13 { state = 218; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 224; lexer.advance(false); continue; }
                if lookahead != 0 { state = 225; lexer.advance(false); continue; }
                return result;
            }
            231 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 13 { state = 233; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 211; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 212; lexer.advance(false); continue; }
                return result;
            }
            232 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 13 { state = 233; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 172; lexer.advance(false); continue; }
                return result;
            }
            233 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 { state = 233; lexer.advance(false); continue; }
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
                if lookahead == 68 { state = 1; lexer.advance(false); continue; }
                if lookahead == 70 { state = 2; lexer.advance(false); continue; }
                if lookahead == 86 { state = 3; lexer.advance(false); continue; }
                if lookahead == 92 { state = 4; lexer.advance(true); continue; }
                if lookahead == 97 { state = 5; lexer.advance(false); continue; }
                if lookahead == 98 { state = 6; lexer.advance(false); continue; }
                if lookahead == 99 { state = 7; lexer.advance(false); continue; }
                if lookahead == 100 { state = 8; lexer.advance(false); continue; }
                if lookahead == 101 { state = 9; lexer.advance(false); continue; }
                if lookahead == 102 { state = 10; lexer.advance(false); continue; }
                if lookahead == 105 { state = 11; lexer.advance(false); continue; }
                if lookahead == 106 { state = 12; lexer.advance(false); continue; }
                if lookahead == 108 { state = 13; lexer.advance(false); continue; }
                if lookahead == 110 { state = 14; lexer.advance(false); continue; }
                if lookahead == 111 { state = 15; lexer.advance(false); continue; }
                if lookahead == 112 { state = 16; lexer.advance(false); continue; }
                if lookahead == 114 { state = 17; lexer.advance(false); continue; }
                if lookahead == 115 { state = 18; lexer.advance(false); continue; }
                if lookahead == 117 { state = 19; lexer.advance(false); continue; }
                if lookahead == 118 { state = 20; lexer.advance(false); continue; }
                if lookahead == 119 { state = 21; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 22; lexer.advance(true); continue; }
                return result;
            }
            1 => {
                result = true; lexer.set_result_symbol(anon_sym_D); lexer.mark_end();
                return result;
            }
            2 => {
                result = true; lexer.set_result_symbol(anon_sym_F); lexer.mark_end();
                return result;
            }
            3 => {
                if lookahead == 80 { state = 23; lexer.advance(false); continue; }
                return result;
            }
            4 => {
                if lookahead == 10 { state = 22; lexer.advance(true); continue; }
                if lookahead == 13 { state = 24; lexer.advance(true); continue; }
                return result;
            }
            5 => {
                if lookahead == 98 { state = 25; lexer.advance(false); continue; }
                if lookahead == 100 { state = 26; lexer.advance(false); continue; }
                if lookahead == 110 { state = 27; lexer.advance(false); continue; }
                return result;
            }
            6 => {
                if lookahead == 97 { state = 28; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if lookahead == 97 { state = 29; lexer.advance(false); continue; }
                return result;
            }
            8 => {
                if lookahead == 101 { state = 30; lexer.advance(false); continue; }
                if lookahead == 105 { state = 31; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if lookahead == 108 { state = 32; lexer.advance(false); continue; }
                if lookahead == 110 { state = 33; lexer.advance(false); continue; }
                if lookahead == 114 { state = 34; lexer.advance(false); continue; }
                if lookahead == 118 { state = 35; lexer.advance(false); continue; }
                if lookahead == 120 { state = 36; lexer.advance(false); continue; }
                return result;
            }
            10 => {
                if lookahead == 105 { state = 37; lexer.advance(false); continue; }
                if lookahead == 108 { state = 38; lexer.advance(false); continue; }
                if lookahead == 111 { state = 39; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if lookahead == 102 { state = 40; lexer.advance(false); continue; }
                if lookahead == 110 { state = 41; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if lookahead == 111 { state = 42; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if lookahead == 97 { state = 43; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if lookahead == 111 { state = 44; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if lookahead == 114 { state = 45; lexer.advance(false); continue; }
                if lookahead == 118 { state = 46; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if lookahead == 97 { state = 47; lexer.advance(false); continue; }
                if lookahead == 114 { state = 48; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if lookahead == 101 { state = 49; lexer.advance(false); continue; }
                return result;
            }
            18 => {
                if lookahead == 104 { state = 50; lexer.advance(false); continue; }
                if lookahead == 105 { state = 51; lexer.advance(false); continue; }
                if lookahead == 111 { state = 52; lexer.advance(false); continue; }
                if lookahead == 116 { state = 53; lexer.advance(false); continue; }
                if lookahead == 117 { state = 54; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if lookahead == 110 { state = 55; lexer.advance(false); continue; }
                return result;
            }
            20 => {
                if lookahead == 97 { state = 56; lexer.advance(false); continue; }
                if lookahead == 112 { state = 57; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                if lookahead == 97 { state = 58; lexer.advance(false); continue; }
                if lookahead == 105 { state = 59; lexer.advance(false); continue; }
                if lookahead == 111 { state = 60; lexer.advance(false); continue; }
                return result;
            }
            22 => {
                if lookahead == 86 { state = 3; lexer.advance(false); continue; }
                if lookahead == 92 { state = 4; lexer.advance(true); continue; }
                if lookahead == 100 { state = 61; lexer.advance(false); continue; }
                if lookahead == 101 { state = 62; lexer.advance(false); continue; }
                if lookahead == 105 { state = 63; lexer.advance(false); continue; }
                if lookahead == 111 { state = 64; lexer.advance(false); continue; }
                if lookahead == 112 { state = 65; lexer.advance(false); continue; }
                if lookahead == 115 { state = 66; lexer.advance(false); continue; }
                if lookahead == 117 { state = 19; lexer.advance(false); continue; }
                if lookahead == 118 { state = 67; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 22; lexer.advance(true); continue; }
                return result;
            }
            23 => {
                if lookahead == 65 { state = 68; lexer.advance(false); continue; }
                return result;
            }
            24 => {
                if lookahead == 86 { state = 3; lexer.advance(false); continue; }
                if lookahead == 92 { state = 4; lexer.advance(true); continue; }
                if lookahead == 100 { state = 61; lexer.advance(false); continue; }
                if lookahead == 101 { state = 62; lexer.advance(false); continue; }
                if lookahead == 105 { state = 63; lexer.advance(false); continue; }
                if lookahead == 111 { state = 64; lexer.advance(false); continue; }
                if lookahead == 112 { state = 65; lexer.advance(false); continue; }
                if lookahead == 115 { state = 66; lexer.advance(false); continue; }
                if lookahead == 117 { state = 19; lexer.advance(false); continue; }
                if lookahead == 118 { state = 67; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 22; lexer.advance(true); continue; }
                return result;
            }
            25 => {
                if lookahead == 115 { state = 69; lexer.advance(false); continue; }
                return result;
            }
            26 => {
                if lookahead == 100 { state = 70; lexer.advance(false); continue; }
                return result;
            }
            27 => {
                if lookahead == 100 { state = 71; lexer.advance(false); continue; }
                return result;
            }
            28 => {
                if lookahead == 115 { state = 72; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                if lookahead == 108 { state = 73; lexer.advance(false); continue; }
                return result;
            }
            30 => {
                if lookahead == 102 { state = 74; lexer.advance(false); continue; }
                return result;
            }
            31 => {
                if lookahead == 114 { state = 75; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                if lookahead == 115 { state = 76; lexer.advance(false); continue; }
                return result;
            }
            33 => {
                if lookahead == 100 { state = 77; lexer.advance(false); continue; }
                return result;
            }
            34 => {
                if lookahead == 114 { state = 78; lexer.advance(false); continue; }
                return result;
            }
            35 => {
                if lookahead == 97 { state = 79; lexer.advance(false); continue; }
                return result;
            }
            36 => {
                if lookahead == 112 { state = 80; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                if lookahead == 108 { state = 81; lexer.advance(false); continue; }
                if lookahead == 110 { state = 82; lexer.advance(false); continue; }
                if lookahead == 114 { state = 83; lexer.advance(false); continue; }
                return result;
            }
            38 => {
                if lookahead == 97 { state = 84; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                if lookahead == 114 { state = 85; lexer.advance(false); continue; }
                return result;
            }
            40 => {
                result = true; lexer.set_result_symbol(anon_sym_if); lexer.mark_end();
                if lookahead == 100 { state = 86; lexer.advance(false); continue; }
                if lookahead == 101 { state = 87; lexer.advance(false); continue; }
                if lookahead == 110 { state = 88; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                if lookahead == 99 { state = 89; lexer.advance(false); continue; }
                if lookahead == 102 { state = 90; lexer.advance(false); continue; }
                return result;
            }
            42 => {
                if lookahead == 105 { state = 91; lexer.advance(false); continue; }
                return result;
            }
            43 => {
                if lookahead == 115 { state = 92; lexer.advance(false); continue; }
                return result;
            }
            44 => {
                if lookahead == 116 { state = 93; lexer.advance(false); continue; }
                return result;
            }
            45 => {
                result = true; lexer.set_result_symbol(anon_sym_or); lexer.mark_end();
                if lookahead == 105 { state = 94; lexer.advance(false); continue; }
                return result;
            }
            46 => {
                if lookahead == 101 { state = 95; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                if lookahead == 116 { state = 96; lexer.advance(false); continue; }
                return result;
            }
            48 => {
                if lookahead == 105 { state = 97; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                if lookahead == 97 { state = 98; lexer.advance(false); continue; }
                return result;
            }
            50 => {
                if lookahead == 101 { state = 99; lexer.advance(false); continue; }
                return result;
            }
            51 => {
                if lookahead == 110 { state = 100; lexer.advance(false); continue; }
                return result;
            }
            52 => {
                if lookahead == 114 { state = 101; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                if lookahead == 114 { state = 102; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                if lookahead == 98 { state = 103; lexer.advance(false); continue; }
                if lookahead == 102 { state = 104; lexer.advance(false); continue; }
                return result;
            }
            55 => {
                if lookahead == 100 { state = 105; lexer.advance(false); continue; }
                if lookahead == 101 { state = 106; lexer.advance(false); continue; }
                return result;
            }
            56 => {
                if lookahead == 108 { state = 107; lexer.advance(false); continue; }
                return result;
            }
            57 => {
                if lookahead == 97 { state = 108; lexer.advance(false); continue; }
                return result;
            }
            58 => {
                if lookahead == 114 { state = 109; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                if lookahead == 108 { state = 110; lexer.advance(false); continue; }
                return result;
            }
            60 => {
                if lookahead == 114 { state = 111; lexer.advance(false); continue; }
                return result;
            }
            61 => {
                if lookahead == 101 { state = 30; lexer.advance(false); continue; }
                return result;
            }
            62 => {
                if lookahead == 108 { state = 32; lexer.advance(false); continue; }
                if lookahead == 110 { state = 33; lexer.advance(false); continue; }
                if lookahead == 120 { state = 36; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                if lookahead == 102 { state = 112; lexer.advance(false); continue; }
                if lookahead == 110 { state = 113; lexer.advance(false); continue; }
                return result;
            }
            64 => {
                if lookahead == 118 { state = 46; lexer.advance(false); continue; }
                return result;
            }
            65 => {
                if lookahead == 114 { state = 48; lexer.advance(false); continue; }
                return result;
            }
            66 => {
                if lookahead == 104 { state = 50; lexer.advance(false); continue; }
                if lookahead == 105 { state = 51; lexer.advance(false); continue; }
                return result;
            }
            67 => {
                if lookahead == 112 { state = 57; lexer.advance(false); continue; }
                return result;
            }
            68 => {
                if lookahead == 84 { state = 114; lexer.advance(false); continue; }
                return result;
            }
            69 => {
                if lookahead == 112 { state = 115; lexer.advance(false); continue; }
                return result;
            }
            70 => {
                if lookahead == 112 { state = 116; lexer.advance(false); continue; }
                if lookahead == 115 { state = 117; lexer.advance(false); continue; }
                return result;
            }
            71 => {
                result = true; lexer.set_result_symbol(anon_sym_and); lexer.mark_end();
                return result;
            }
            72 => {
                if lookahead == 101 { state = 118; lexer.advance(false); continue; }
                return result;
            }
            73 => {
                if lookahead == 108 { state = 119; lexer.advance(false); continue; }
                return result;
            }
            74 => {
                if lookahead == 105 { state = 120; lexer.advance(false); continue; }
                return result;
            }
            75 => {
                result = true; lexer.set_result_symbol(anon_sym_dir); lexer.mark_end();
                return result;
            }
            76 => {
                if lookahead == 101 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            77 => {
                if lookahead == 105 { state = 122; lexer.advance(false); continue; }
                return result;
            }
            78 => {
                if lookahead == 111 { state = 123; lexer.advance(false); continue; }
                return result;
            }
            79 => {
                if lookahead == 108 { state = 124; lexer.advance(false); continue; }
                return result;
            }
            80 => {
                if lookahead == 111 { state = 125; lexer.advance(false); continue; }
                return result;
            }
            81 => {
                if lookahead == 101 { state = 126; lexer.advance(false); continue; }
                if lookahead == 116 { state = 127; lexer.advance(false); continue; }
                return result;
            }
            82 => {
                if lookahead == 100 { state = 128; lexer.advance(false); continue; }
                return result;
            }
            83 => {
                if lookahead == 115 { state = 129; lexer.advance(false); continue; }
                return result;
            }
            84 => {
                if lookahead == 118 { state = 130; lexer.advance(false); continue; }
                return result;
            }
            85 => {
                if lookahead == 101 { state = 131; lexer.advance(false); continue; }
                return result;
            }
            86 => {
                if lookahead == 101 { state = 132; lexer.advance(false); continue; }
                return result;
            }
            87 => {
                if lookahead == 113 { state = 133; lexer.advance(false); continue; }
                return result;
            }
            88 => {
                if lookahead == 100 { state = 134; lexer.advance(false); continue; }
                if lookahead == 101 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            89 => {
                if lookahead == 108 { state = 136; lexer.advance(false); continue; }
                return result;
            }
            90 => {
                if lookahead == 111 { state = 137; lexer.advance(false); continue; }
                return result;
            }
            91 => {
                if lookahead == 110 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            92 => {
                if lookahead == 116 { state = 139; lexer.advance(false); continue; }
                return result;
            }
            93 => {
                if lookahead == 100 { state = 140; lexer.advance(false); continue; }
                return result;
            }
            94 => {
                if lookahead == 103 { state = 141; lexer.advance(false); continue; }
                return result;
            }
            95 => {
                if lookahead == 114 { state = 142; lexer.advance(false); continue; }
                return result;
            }
            96 => {
                if lookahead == 115 { state = 143; lexer.advance(false); continue; }
                return result;
            }
            97 => {
                if lookahead == 118 { state = 144; lexer.advance(false); continue; }
                return result;
            }
            98 => {
                if lookahead == 108 { state = 145; lexer.advance(false); continue; }
                return result;
            }
            99 => {
                if lookahead == 108 { state = 146; lexer.advance(false); continue; }
                return result;
            }
            100 => {
                if lookahead == 99 { state = 147; lexer.advance(false); continue; }
                return result;
            }
            101 => {
                if lookahead == 116 { state = 148; lexer.advance(false); continue; }
                return result;
            }
            102 => {
                if lookahead == 105 { state = 149; lexer.advance(false); continue; }
                return result;
            }
            103 => {
                if lookahead == 115 { state = 150; lexer.advance(false); continue; }
                return result;
            }
            104 => {
                if lookahead == 102 { state = 151; lexer.advance(false); continue; }
                return result;
            }
            105 => {
                if lookahead == 101 { state = 152; lexer.advance(false); continue; }
                return result;
            }
            106 => {
                if lookahead == 120 { state = 153; lexer.advance(false); continue; }
                return result;
            }
            107 => {
                if lookahead == 117 { state = 154; lexer.advance(false); continue; }
                return result;
            }
            108 => {
                if lookahead == 116 { state = 155; lexer.advance(false); continue; }
                return result;
            }
            109 => {
                if lookahead == 110 { state = 156; lexer.advance(false); continue; }
                return result;
            }
            110 => {
                if lookahead == 100 { state = 157; lexer.advance(false); continue; }
                return result;
            }
            111 => {
                if lookahead == 100 { state = 158; lexer.advance(false); continue; }
                return result;
            }
            112 => {
                if lookahead == 100 { state = 86; lexer.advance(false); continue; }
                if lookahead == 101 { state = 87; lexer.advance(false); continue; }
                if lookahead == 110 { state = 88; lexer.advance(false); continue; }
                return result;
            }
            113 => {
                if lookahead == 99 { state = 89; lexer.advance(false); continue; }
                return result;
            }
            114 => {
                if lookahead == 72 { state = 159; lexer.advance(false); continue; }
                return result;
            }
            115 => {
                if lookahead == 97 { state = 160; lexer.advance(false); continue; }
                return result;
            }
            116 => {
                if lookahead == 114 { state = 161; lexer.advance(false); continue; }
                return result;
            }
            117 => {
                if lookahead == 117 { state = 162; lexer.advance(false); continue; }
                return result;
            }
            118 => {
                if lookahead == 110 { state = 163; lexer.advance(false); continue; }
                return result;
            }
            119 => {
                result = true; lexer.set_result_symbol(anon_sym_call); lexer.mark_end();
                return result;
            }
            120 => {
                if lookahead == 110 { state = 164; lexer.advance(false); continue; }
                return result;
            }
            121 => {
                result = true; lexer.set_result_symbol(anon_sym_else); lexer.mark_end();
                return result;
            }
            122 => {
                if lookahead == 102 { state = 165; lexer.advance(false); continue; }
                return result;
            }
            123 => {
                if lookahead == 114 { state = 166; lexer.advance(false); continue; }
                return result;
            }
            124 => {
                result = true; lexer.set_result_symbol(anon_sym_eval); lexer.mark_end();
                return result;
            }
            125 => {
                if lookahead == 114 { state = 167; lexer.advance(false); continue; }
                return result;
            }
            126 => {
                result = true; lexer.set_result_symbol(anon_sym_file); lexer.mark_end();
                return result;
            }
            127 => {
                if lookahead == 101 { state = 168; lexer.advance(false); continue; }
                return result;
            }
            128 => {
                if lookahead == 115 { state = 169; lexer.advance(false); continue; }
                return result;
            }
            129 => {
                if lookahead == 116 { state = 170; lexer.advance(false); continue; }
                return result;
            }
            130 => {
                if lookahead == 111 { state = 171; lexer.advance(false); continue; }
                return result;
            }
            131 => {
                if lookahead == 97 { state = 172; lexer.advance(false); continue; }
                return result;
            }
            132 => {
                if lookahead == 102 { state = 173; lexer.advance(false); continue; }
                return result;
            }
            133 => {
                result = true; lexer.set_result_symbol(anon_sym_ifeq); lexer.mark_end();
                return result;
            }
            134 => {
                if lookahead == 101 { state = 174; lexer.advance(false); continue; }
                return result;
            }
            135 => {
                if lookahead == 113 { state = 175; lexer.advance(false); continue; }
                return result;
            }
            136 => {
                if lookahead == 117 { state = 176; lexer.advance(false); continue; }
                return result;
            }
            137 => {
                result = true; lexer.set_result_symbol(anon_sym_info); lexer.mark_end();
                return result;
            }
            138 => {
                result = true; lexer.set_result_symbol(anon_sym_join); lexer.mark_end();
                return result;
            }
            139 => {
                if lookahead == 119 { state = 177; lexer.advance(false); continue; }
                return result;
            }
            140 => {
                if lookahead == 105 { state = 178; lexer.advance(false); continue; }
                return result;
            }
            141 => {
                if lookahead == 105 { state = 179; lexer.advance(false); continue; }
                return result;
            }
            142 => {
                if lookahead == 114 { state = 180; lexer.advance(false); continue; }
                return result;
            }
            143 => {
                if lookahead == 117 { state = 181; lexer.advance(false); continue; }
                return result;
            }
            144 => {
                if lookahead == 97 { state = 182; lexer.advance(false); continue; }
                return result;
            }
            145 => {
                if lookahead == 112 { state = 183; lexer.advance(false); continue; }
                return result;
            }
            146 => {
                if lookahead == 108 { state = 184; lexer.advance(false); continue; }
                return result;
            }
            147 => {
                if lookahead == 108 { state = 185; lexer.advance(false); continue; }
                return result;
            }
            148 => {
                result = true; lexer.set_result_symbol(anon_sym_sort); lexer.mark_end();
                return result;
            }
            149 => {
                if lookahead == 112 { state = 186; lexer.advance(false); continue; }
                return result;
            }
            150 => {
                if lookahead == 116 { state = 187; lexer.advance(false); continue; }
                return result;
            }
            151 => {
                if lookahead == 105 { state = 188; lexer.advance(false); continue; }
                return result;
            }
            152 => {
                if lookahead == 102 { state = 189; lexer.advance(false); continue; }
                return result;
            }
            153 => {
                if lookahead == 112 { state = 190; lexer.advance(false); continue; }
                return result;
            }
            154 => {
                if lookahead == 101 { state = 191; lexer.advance(false); continue; }
                return result;
            }
            155 => {
                if lookahead == 104 { state = 192; lexer.advance(false); continue; }
                return result;
            }
            156 => {
                if lookahead == 105 { state = 193; lexer.advance(false); continue; }
                return result;
            }
            157 => {
                if lookahead == 99 { state = 194; lexer.advance(false); continue; }
                return result;
            }
            158 => {
                result = true; lexer.set_result_symbol(anon_sym_word); lexer.mark_end();
                if lookahead == 108 { state = 195; lexer.advance(false); continue; }
                if lookahead == 115 { state = 196; lexer.advance(false); continue; }
                return result;
            }
            159 => {
                result = true; lexer.set_result_symbol(anon_sym_VPATH); lexer.mark_end();
                return result;
            }
            160 => {
                if lookahead == 116 { state = 197; lexer.advance(false); continue; }
                return result;
            }
            161 => {
                if lookahead == 101 { state = 198; lexer.advance(false); continue; }
                return result;
            }
            162 => {
                if lookahead == 102 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            163 => {
                if lookahead == 97 { state = 200; lexer.advance(false); continue; }
                return result;
            }
            164 => {
                if lookahead == 101 { state = 201; lexer.advance(false); continue; }
                return result;
            }
            165 => {
                result = true; lexer.set_result_symbol(anon_sym_endif); lexer.mark_end();
                return result;
            }
            166 => {
                result = true; lexer.set_result_symbol(anon_sym_error); lexer.mark_end();
                return result;
            }
            167 => {
                if lookahead == 116 { state = 202; lexer.advance(false); continue; }
                return result;
            }
            168 => {
                if lookahead == 114 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            169 => {
                if lookahead == 116 { state = 204; lexer.advance(false); continue; }
                return result;
            }
            170 => {
                if lookahead == 119 { state = 205; lexer.advance(false); continue; }
                return result;
            }
            171 => {
                if lookahead == 114 { state = 206; lexer.advance(false); continue; }
                return result;
            }
            172 => {
                if lookahead == 99 { state = 207; lexer.advance(false); continue; }
                return result;
            }
            173 => {
                result = true; lexer.set_result_symbol(anon_sym_ifdef); lexer.mark_end();
                return result;
            }
            174 => {
                if lookahead == 102 { state = 208; lexer.advance(false); continue; }
                return result;
            }
            175 => {
                result = true; lexer.set_result_symbol(anon_sym_ifneq); lexer.mark_end();
                return result;
            }
            176 => {
                if lookahead == 100 { state = 209; lexer.advance(false); continue; }
                return result;
            }
            177 => {
                if lookahead == 111 { state = 210; lexer.advance(false); continue; }
                return result;
            }
            178 => {
                if lookahead == 114 { state = 211; lexer.advance(false); continue; }
                return result;
            }
            179 => {
                if lookahead == 110 { state = 212; lexer.advance(false); continue; }
                return result;
            }
            180 => {
                if lookahead == 105 { state = 213; lexer.advance(false); continue; }
                return result;
            }
            181 => {
                if lookahead == 98 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            182 => {
                if lookahead == 116 { state = 215; lexer.advance(false); continue; }
                return result;
            }
            183 => {
                if lookahead == 97 { state = 216; lexer.advance(false); continue; }
                return result;
            }
            184 => {
                result = true; lexer.set_result_symbol(anon_sym_shell); lexer.mark_end();
                return result;
            }
            185 => {
                if lookahead == 117 { state = 217; lexer.advance(false); continue; }
                return result;
            }
            186 => {
                result = true; lexer.set_result_symbol(anon_sym_strip); lexer.mark_end();
                return result;
            }
            187 => {
                result = true; lexer.set_result_symbol(anon_sym_subst); lexer.mark_end();
                return result;
            }
            188 => {
                if lookahead == 120 { state = 218; lexer.advance(false); continue; }
                return result;
            }
            189 => {
                if lookahead == 105 { state = 219; lexer.advance(false); continue; }
                return result;
            }
            190 => {
                if lookahead == 111 { state = 220; lexer.advance(false); continue; }
                return result;
            }
            191 => {
                result = true; lexer.set_result_symbol(anon_sym_value); lexer.mark_end();
                return result;
            }
            192 => {
                result = true; lexer.set_result_symbol(anon_sym_vpath); lexer.mark_end();
                return result;
            }
            193 => {
                if lookahead == 110 { state = 221; lexer.advance(false); continue; }
                return result;
            }
            194 => {
                if lookahead == 97 { state = 222; lexer.advance(false); continue; }
                return result;
            }
            195 => {
                if lookahead == 105 { state = 223; lexer.advance(false); continue; }
                return result;
            }
            196 => {
                result = true; lexer.set_result_symbol(anon_sym_words); lexer.mark_end();
                return result;
            }
            197 => {
                if lookahead == 104 { state = 224; lexer.advance(false); continue; }
                return result;
            }
            198 => {
                if lookahead == 102 { state = 225; lexer.advance(false); continue; }
                return result;
            }
            199 => {
                if lookahead == 102 { state = 226; lexer.advance(false); continue; }
                return result;
            }
            200 => {
                if lookahead == 109 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            201 => {
                result = true; lexer.set_result_symbol(anon_sym_define); lexer.mark_end();
                return result;
            }
            202 => {
                result = true; lexer.set_result_symbol(anon_sym_export); lexer.mark_end();
                return result;
            }
            203 => {
                result = true; lexer.set_result_symbol(anon_sym_filter); lexer.mark_end();
                if lookahead == 45 { state = 228; lexer.advance(false); continue; }
                return result;
            }
            204 => {
                if lookahead == 114 { state = 229; lexer.advance(false); continue; }
                return result;
            }
            205 => {
                if lookahead == 111 { state = 230; lexer.advance(false); continue; }
                return result;
            }
            206 => {
                result = true; lexer.set_result_symbol(anon_sym_flavor); lexer.mark_end();
                return result;
            }
            207 => {
                if lookahead == 104 { state = 231; lexer.advance(false); continue; }
                return result;
            }
            208 => {
                result = true; lexer.set_result_symbol(anon_sym_ifndef); lexer.mark_end();
                return result;
            }
            209 => {
                if lookahead == 101 { state = 232; lexer.advance(false); continue; }
                return result;
            }
            210 => {
                if lookahead == 114 { state = 233; lexer.advance(false); continue; }
                return result;
            }
            211 => {
                result = true; lexer.set_result_symbol(anon_sym_notdir); lexer.mark_end();
                return result;
            }
            212 => {
                result = true; lexer.set_result_symbol(anon_sym_origin); lexer.mark_end();
                return result;
            }
            213 => {
                if lookahead == 100 { state = 234; lexer.advance(false); continue; }
                return result;
            }
            214 => {
                if lookahead == 115 { state = 235; lexer.advance(false); continue; }
                return result;
            }
            215 => {
                if lookahead == 101 { state = 236; lexer.advance(false); continue; }
                return result;
            }
            216 => {
                if lookahead == 116 { state = 237; lexer.advance(false); continue; }
                return result;
            }
            217 => {
                if lookahead == 100 { state = 238; lexer.advance(false); continue; }
                return result;
            }
            218 => {
                result = true; lexer.set_result_symbol(anon_sym_suffix); lexer.mark_end();
                return result;
            }
            219 => {
                if lookahead == 110 { state = 239; lexer.advance(false); continue; }
                return result;
            }
            220 => {
                if lookahead == 114 { state = 240; lexer.advance(false); continue; }
                return result;
            }
            221 => {
                if lookahead == 103 { state = 241; lexer.advance(false); continue; }
                return result;
            }
            222 => {
                if lookahead == 114 { state = 242; lexer.advance(false); continue; }
                return result;
            }
            223 => {
                if lookahead == 115 { state = 243; lexer.advance(false); continue; }
                return result;
            }
            224 => {
                result = true; lexer.set_result_symbol(anon_sym_abspath); lexer.mark_end();
                return result;
            }
            225 => {
                if lookahead == 105 { state = 244; lexer.advance(false); continue; }
                return result;
            }
            226 => {
                if lookahead == 105 { state = 245; lexer.advance(false); continue; }
                return result;
            }
            227 => {
                if lookahead == 101 { state = 246; lexer.advance(false); continue; }
                return result;
            }
            228 => {
                if lookahead == 111 { state = 247; lexer.advance(false); continue; }
                return result;
            }
            229 => {
                if lookahead == 105 { state = 248; lexer.advance(false); continue; }
                return result;
            }
            230 => {
                if lookahead == 114 { state = 249; lexer.advance(false); continue; }
                return result;
            }
            231 => {
                result = true; lexer.set_result_symbol(anon_sym_foreach); lexer.mark_end();
                return result;
            }
            232 => {
                result = true; lexer.set_result_symbol(anon_sym_include); lexer.mark_end();
                return result;
            }
            233 => {
                if lookahead == 100 { state = 250; lexer.advance(false); continue; }
                return result;
            }
            234 => {
                if lookahead == 101 { state = 251; lexer.advance(false); continue; }
                return result;
            }
            235 => {
                if lookahead == 116 { state = 252; lexer.advance(false); continue; }
                return result;
            }
            236 => {
                result = true; lexer.set_result_symbol(anon_sym_private); lexer.mark_end();
                return result;
            }
            237 => {
                if lookahead == 104 { state = 253; lexer.advance(false); continue; }
                return result;
            }
            238 => {
                if lookahead == 101 { state = 254; lexer.advance(false); continue; }
                return result;
            }
            239 => {
                if lookahead == 101 { state = 255; lexer.advance(false); continue; }
                return result;
            }
            240 => {
                if lookahead == 116 { state = 256; lexer.advance(false); continue; }
                return result;
            }
            241 => {
                result = true; lexer.set_result_symbol(anon_sym_warning); lexer.mark_end();
                return result;
            }
            242 => {
                if lookahead == 100 { state = 257; lexer.advance(false); continue; }
                return result;
            }
            243 => {
                if lookahead == 116 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            244 => {
                if lookahead == 120 { state = 259; lexer.advance(false); continue; }
                return result;
            }
            245 => {
                if lookahead == 120 { state = 260; lexer.advance(false); continue; }
                return result;
            }
            246 => {
                result = true; lexer.set_result_symbol(anon_sym_basename); lexer.mark_end();
                return result;
            }
            247 => {
                if lookahead == 117 { state = 261; lexer.advance(false); continue; }
                return result;
            }
            248 => {
                if lookahead == 110 { state = 262; lexer.advance(false); continue; }
                return result;
            }
            249 => {
                if lookahead == 100 { state = 263; lexer.advance(false); continue; }
                return result;
            }
            250 => {
                result = true; lexer.set_result_symbol(anon_sym_lastword); lexer.mark_end();
                return result;
            }
            251 => {
                result = true; lexer.set_result_symbol(anon_sym_override); lexer.mark_end();
                return result;
            }
            252 => {
                result = true; lexer.set_result_symbol(anon_sym_patsubst); lexer.mark_end();
                return result;
            }
            253 => {
                result = true; lexer.set_result_symbol(anon_sym_realpath); lexer.mark_end();
                return result;
            }
            254 => {
                result = true; lexer.set_result_symbol(anon_sym_sinclude); lexer.mark_end();
                return result;
            }
            255 => {
                result = true; lexer.set_result_symbol(anon_sym_undefine); lexer.mark_end();
                return result;
            }
            256 => {
                result = true; lexer.set_result_symbol(anon_sym_unexport); lexer.mark_end();
                return result;
            }
            257 => {
                result = true; lexer.set_result_symbol(anon_sym_wildcard); lexer.mark_end();
                return result;
            }
            258 => {
                result = true; lexer.set_result_symbol(anon_sym_wordlist); lexer.mark_end();
                return result;
            }
            259 => {
                result = true; lexer.set_result_symbol(anon_sym_addprefix); lexer.mark_end();
                return result;
            }
            260 => {
                result = true; lexer.set_result_symbol(anon_sym_addsuffix); lexer.mark_end();
                return result;
            }
            261 => {
                if lookahead == 116 { state = 264; lexer.advance(false); continue; }
                return result;
            }
            262 => {
                if lookahead == 103 { state = 265; lexer.advance(false); continue; }
                return result;
            }
            263 => {
                result = true; lexer.set_result_symbol(anon_sym_firstword); lexer.mark_end();
                return result;
            }
            264 => {
                result = true; lexer.set_result_symbol(anon_sym_filter_DASHout); lexer.mark_end();
                return result;
            }
            265 => {
                result = true; lexer.set_result_symbol(anon_sym_findstring); lexer.mark_end();
                return result;
            }
            _ => return false,
        }
    }
}
