//! The `bash` grammar's lexer: `ts_lex` and `ts_lex_keywords`, transliterated from
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

const anon_sym_A: Symbol = 137;
const anon_sym_AMP: Symbol = 29;
const anon_sym_AMP_AMP: Symbol = 25;
const anon_sym_AMP_EQ: Symbol = 20;
const anon_sym_AMP_GT: Symbol = 78;
const anon_sym_AMP_GT_GT: Symbol = 79;
const anon_sym_AT: Symbol = 113;
const anon_sym_AT2: Symbol = 150;
const anon_sym_BANG: Symbol = 65;
const anon_sym_BANG2: Symbol = 112;
const anon_sym_BANG_EQ: Symbol = 31;
const anon_sym_BQUOTE: Symbol = 142;
const anon_sym_CARET: Symbol = 28;
const anon_sym_CARET_CARET: Symbol = 130;
const anon_sym_CARET_EQ: Symbol = 21;
const anon_sym_COLON: Symbol = 89;
const anon_sym_COLON_DASH: Symbol = 119;
const anon_sym_COLON_EQ: Symbol = 117;
const anon_sym_COLON_PLUS: Symbol = 121;
const anon_sym_COLON_QMARK: Symbol = 123;
const anon_sym_COMMA: Symbol = 8;
const anon_sym_COMMA_COMMA: Symbol = 129;
const anon_sym_DASH: Symbol = 39;
const anon_sym_DASH2: Symbol = 92;
const anon_sym_DASH3: Symbol = 118;
const anon_sym_DASH_DASH: Symbol = 11;
const anon_sym_DASH_DASH2: Symbol = 91;
const anon_sym_DASH_EQ: Symbol = 13;
const anon_sym_DASHa: Symbol = 26;
const anon_sym_DASHo: Symbol = 24;
const anon_sym_DOLLAR: Symbol = 101;
const anon_sym_DOLLAR_BQUOTE: Symbol = 143;
const anon_sym_DOLLAR_LBRACE: Symbol = 110;
const anon_sym_DOLLAR_LBRACK: Symbol = 96;
const anon_sym_DOLLAR_LPAREN: Symbol = 141;
const anon_sym_DOLLAR_LPAREN_LPAREN: Symbol = 95;
const anon_sym_DOT_DOT: Symbol = 98;
const anon_sym_DQUOTE: Symbol = 103;
const anon_sym_E: Symbol = 135;
const anon_sym_EQ: Symbol = 9;
const anon_sym_EQ2: Symbol = 116;
const anon_sym_EQ_EQ: Symbol = 30;
const anon_sym_EQ_TILDE: Symbol = 77;
const anon_sym_GT: Symbol = 33;
const anon_sym_GT_AMP: Symbol = 81;
const anon_sym_GT_AMP_DASH: Symbol = 84;
const anon_sym_GT_EQ: Symbol = 35;
const anon_sym_GT_GT: Symbol = 37;
const anon_sym_GT_GT_EQ: Symbol = 19;
const anon_sym_GT_LPAREN: Symbol = 145;
const anon_sym_GT_PIPE: Symbol = 82;
const anon_sym_K: Symbol = 138;
const anon_sym_L: Symbol = 133;
const anon_sym_LBRACE: Symbol = 62;
const anon_sym_LBRACK: Symbol = 66;
const anon_sym_LBRACK_LBRACK: Symbol = 68;
const anon_sym_LPAREN: Symbol = 44;
const anon_sym_LPAREN_LPAREN: Symbol = 5;
const anon_sym_LT: Symbol = 32;
const anon_sym_LT_AMP: Symbol = 80;
const anon_sym_LT_AMP_DASH: Symbol = 83;
const anon_sym_LT_EQ: Symbol = 34;
const anon_sym_LT_LPAREN: Symbol = 144;
const anon_sym_LT_LT: Symbol = 36;
const anon_sym_LT_LT_DASH: Symbol = 85;
const anon_sym_LT_LT_EQ: Symbol = 18;
const anon_sym_LT_LT_LT: Symbol = 87;
const anon_sym_P: Symbol = 136;
const anon_sym_PERCENT: Symbol = 42;
const anon_sym_PERCENT_EQ: Symbol = 16;
const anon_sym_PERCENT_PERCENT: Symbol = 124;
const anon_sym_PIPE: Symbol = 27;
const anon_sym_PIPE_AMP: Symbol = 64;
const anon_sym_PIPE_EQ: Symbol = 22;
const anon_sym_PIPE_PIPE: Symbol = 23;
const anon_sym_PLUS: Symbol = 38;
const anon_sym_PLUS2: Symbol = 93;
const anon_sym_PLUS3: Symbol = 120;
const anon_sym_PLUS_EQ: Symbol = 12;
const anon_sym_PLUS_PLUS: Symbol = 10;
const anon_sym_PLUS_PLUS2: Symbol = 90;
const anon_sym_POUND: Symbol = 109;
const anon_sym_POUND2: Symbol = 115;
const anon_sym_Q: Symbol = 134;
const anon_sym_QMARK: Symbol = 88;
const anon_sym_QMARK2: Symbol = 122;
const anon_sym_RBRACE: Symbol = 63;
const anon_sym_RBRACE2: Symbol = 99;
const anon_sym_RBRACE3: Symbol = 111;
const anon_sym_RBRACK: Symbol = 67;
const anon_sym_RBRACK_RBRACK: Symbol = 69;
const anon_sym_RPAREN: Symbol = 45;
const anon_sym_RPAREN_RPAREN: Symbol = 6;
const anon_sym_SEMI: Symbol = 7;
const anon_sym_SEMI_AMP: Symbol = 59;
const anon_sym_SEMI_SEMI: Symbol = 58;
const anon_sym_SEMI_SEMI_AMP: Symbol = 60;
const anon_sym_SLASH: Symbol = 41;
const anon_sym_SLASH_EQ: Symbol = 15;
const anon_sym_SLASH_PERCENT: Symbol = 128;
const anon_sym_SLASH_POUND: Symbol = 127;
const anon_sym_SLASH_SLASH: Symbol = 126;
const anon_sym_STAR: Symbol = 40;
const anon_sym_STAR2: Symbol = 114;
const anon_sym_STAR_EQ: Symbol = 14;
const anon_sym_STAR_STAR: Symbol = 43;
const anon_sym_STAR_STAR_EQ: Symbol = 17;
const anon_sym_TILDE: Symbol = 94;
const anon_sym_U: Symbol = 131;
const anon_sym__: Symbol = 151;
const anon_sym_a: Symbol = 139;
const anon_sym_case: Symbol = 56;
const anon_sym_declare: Symbol = 70;
const anon_sym_do: Symbol = 49;
const anon_sym_done: Symbol = 50;
const anon_sym_elif: Symbol = 54;
const anon_sym_else: Symbol = 55;
const anon_sym_esac: Symbol = 57;
const anon_sym_export: Symbol = 72;
const anon_sym_fi: Symbol = 53;
const anon_sym_for: Symbol = 2;
const anon_sym_function: Symbol = 61;
const anon_sym_if: Symbol = 51;
const anon_sym_in: Symbol = 4;
const anon_sym_k: Symbol = 140;
const anon_sym_local: Symbol = 74;
const anon_sym_readonly: Symbol = 73;
const anon_sym_select: Symbol = 3;
const anon_sym_then: Symbol = 52;
const anon_sym_typeset: Symbol = 71;
const anon_sym_u: Symbol = 132;
const anon_sym_unset: Symbol = 75;
const anon_sym_unsetenv: Symbol = 76;
const anon_sym_until: Symbol = 48;
const anon_sym_while: Symbol = 47;
const aux_sym__c_word_token1: Symbol = 46;
const aux_sym__expansion_regex_token1: Symbol = 125;
const aux_sym__multiline_variable_name_token1: Symbol = 149;
const aux_sym__simple_variable_name_token1: Symbol = 148;
const aux_sym_brace_expression_token1: Symbol = 97;
const aux_sym_concatenation_token1: Symbol = 100;
const aux_sym_heredoc_redirect_token1: Symbol = 86;
const aux_sym_number_token1: Symbol = 107;
const aux_sym_number_token2: Symbol = 108;
const sym__comment_word: Symbol = 147;
const sym__special_character: Symbol = 102;
const sym_ansi_c_string: Symbol = 106;
const sym_comment: Symbol = 146;
const sym_raw_string: Symbol = 105;
const sym_string_content: Symbol = 104;
const sym_word: Symbol = 1;
const ts_builtin_sym_end: Symbol = 0;

#[rustfmt::skip]
static sym__comment_word_character_set_1: [CharacterRange; 12] = [
    CharacterRange::new(0, 8), CharacterRange::new(14, 31), CharacterRange::new(33, 33), CharacterRange::new(35, 35), CharacterRange::new(37, 37), CharacterRange::new(42, 58),
    CharacterRange::new(61, 61), CharacterRange::new(63, 90), CharacterRange::new(92, 92), CharacterRange::new(94, 95), CharacterRange::new(97, 122), CharacterRange::new(126, 1114111),
];

#[rustfmt::skip]
static sym_word_character_set_1: [CharacterRange; 11] = [
    CharacterRange::new(0, 8), CharacterRange::new(14, 31), CharacterRange::new(33, 33), CharacterRange::new(37, 37), CharacterRange::new(42, 58), CharacterRange::new(61, 61),
    CharacterRange::new(63, 90), CharacterRange::new(92, 92), CharacterRange::new(94, 95), CharacterRange::new(97, 122), CharacterRange::new(126, 1114111),
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
                if eof { state = 541; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 815), (34, 783), (35, 820), (36, 776), (37, 641), (38, 595), (39, 485), (40, 648),
                    (41, 649), (42, 819), (43, 764), (44, 551), (45, 762), (46, 940), (47, 636), (58, 757),
                    (59, 548), (60, 601), (61, 822), (62, 610), (63, 832), (64, 817), (91, 668), (92, 298),
                    (93, 669), (94, 589), (95, 937), (96, 851), (101, 947), (105, 946), (123, 660), (124, 586),
                    (125, 773), (126, 766),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 539; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 769; lexer.advance(false); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            1 => {
                if lookahead == 10 { state = 397; lexer.advance(true); continue; }
                return result;
            }
            2 => {
                if lookahead == 10 { state = 412; lexer.advance(true); continue; }
                return result;
            }
            3 => {
                if lookahead == 10 { state = 413; lexer.advance(true); continue; }
                return result;
            }
            4 => {
                if lookahead == 10 { state = 414; lexer.advance(true); continue; }
                return result;
            }
            5 => {
                if lookahead == 10 { state = 6; lexer.advance(true); continue; }
                return result;
            }
            6 => {
                if let Some(next) = advance_map(&[
                    (10, 682), (33, 664), (34, 783), (35, 857), (36, 776), (37, 643), (38, 595), (39, 485),
                    (40, 647), (41, 649), (42, 632), (43, 765), (45, 763), (47, 638), (48, 797), (59, 549),
                    (60, 601), (61, 554), (62, 610), (63, 755), (92, 304), (94, 590), (96, 851), (124, 586),
                    (126, 766), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 6; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if lookahead == 10 { state = 399; lexer.advance(true); continue; }
                return result;
            }
            8 => {
                if lookahead == 10 { state = 415; lexer.advance(true); continue; }
                return result;
            }
            9 => {
                if lookahead == 10 { state = 416; lexer.advance(true); continue; }
                return result;
            }
            10 => {
                if lookahead == 10 { state = 400; lexer.advance(true); continue; }
                return result;
            }
            11 => {
                if lookahead == 10 { state = 401; lexer.advance(true); continue; }
                return result;
            }
            12 => {
                if let Some(next) = advance_map(&[
                    (10, 683), (33, 664), (34, 783), (35, 811), (36, 776), (37, 643), (38, 595), (39, 485),
                    (40, 647), (41, 649), (42, 632), (43, 624), (45, 627), (47, 638), (48, 801), (59, 549),
                    (60, 601), (61, 554), (62, 610), (63, 755), (64, 936), (92, 14), (94, 590), (95, 938),
                    (96, 850), (124, 586), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 12; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if lookahead == 10 { state = 875; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if lookahead == 10 { state = 875; lexer.advance(false); continue; }
                if lookahead == 13 { state = 13; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 12; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if lookahead == 10 { state = 934; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if lookahead == 10 { state = 934; lexer.advance(false); continue; }
                if lookahead == 13 { state = 15; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if lookahead == 10 { state = 934; lexer.advance(false); continue; }
                if lookahead == 13 { state = 15; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            18 => {
                if lookahead == 10 { state = 913; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if lookahead == 10 { state = 913; lexer.advance(false); continue; }
                if lookahead == 13 { state = 18; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 398; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            20 => {
                if lookahead == 10 { state = 156; lexer.advance(true); continue; }
                return result;
            }
            21 => {
                if lookahead == 10 { state = 420; lexer.advance(true); continue; }
                return result;
            }
            22 => {
                if lookahead == 10 { state = 417; lexer.advance(true); continue; }
                return result;
            }
            23 => {
                if lookahead == 10 { state = 236; lexer.advance(true); continue; }
                return result;
            }
            24 => {
                if lookahead == 10 { state = 239; lexer.advance(true); continue; }
                return result;
            }
            25 => {
                if lookahead == 10 { state = 242; lexer.advance(true); continue; }
                return result;
            }
            26 => {
                if lookahead == 10 { state = 245; lexer.advance(true); continue; }
                return result;
            }
            27 => {
                if lookahead == 10 { state = 877; lexer.advance(false); continue; }
                return result;
            }
            28 => {
                if lookahead == 10 { state = 877; lexer.advance(false); continue; }
                if lookahead == 13 { state = 27; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 216; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                if lookahead == 10 { state = 879; lexer.advance(false); continue; }
                return result;
            }
            30 => {
                if lookahead == 10 { state = 879; lexer.advance(false); continue; }
                if lookahead == 13 { state = 29; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 235; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            31 => {
                if lookahead == 10 { state = 881; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                if lookahead == 10 { state = 881; lexer.advance(false); continue; }
                if lookahead == 13 { state = 31; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 238; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            33 => {
                if lookahead == 10 { state = 248; lexer.advance(true); continue; }
                return result;
            }
            34 => {
                if lookahead == 10 { state = 251; lexer.advance(true); continue; }
                return result;
            }
            35 => {
                if lookahead == 10 { state = 883; lexer.advance(false); continue; }
                return result;
            }
            36 => {
                if lookahead == 10 { state = 883; lexer.advance(false); continue; }
                if lookahead == 13 { state = 35; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 241; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                if lookahead == 10 { state = 254; lexer.advance(true); continue; }
                return result;
            }
            38 => {
                if lookahead == 10 { state = 887; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                if lookahead == 10 { state = 887; lexer.advance(false); continue; }
                if lookahead == 13 { state = 38; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 247; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            40 => {
                if lookahead == 10 { state = 891; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                if lookahead == 10 { state = 891; lexer.advance(false); continue; }
                if lookahead == 13 { state = 40; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 253; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            42 => {
                if lookahead == 10 { state = 257; lexer.advance(true); continue; }
                return result;
            }
            43 => {
                if lookahead == 10 { state = 451; lexer.advance(true); continue; }
                return result;
            }
            44 => {
                if lookahead == 10 { state = 260; lexer.advance(true); continue; }
                return result;
            }
            45 => {
                if lookahead == 10 { state = 455; lexer.advance(true); continue; }
                return result;
            }
            46 => {
                if lookahead == 10 { state = 893; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                if lookahead == 10 { state = 893; lexer.advance(false); continue; }
                if lookahead == 13 { state = 46; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 256; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            48 => {
                if lookahead == 10 { state = 895; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                if lookahead == 10 { state = 895; lexer.advance(false); continue; }
                if lookahead == 13 { state = 48; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 259; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            50 => {
                if lookahead == 10 { state = 269; lexer.advance(true); continue; }
                return result;
            }
            51 => {
                if lookahead == 10 { state = 901; lexer.advance(false); continue; }
                return result;
            }
            52 => {
                if lookahead == 10 { state = 901; lexer.advance(false); continue; }
                if lookahead == 13 { state = 51; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 268; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                if lookahead == 10 { state = 904; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                if lookahead == 10 { state = 904; lexer.advance(false); continue; }
                if lookahead == 13 { state = 53; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 273; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            55 => {
                if lookahead == 10 { state = 464; lexer.advance(true); continue; }
                return result;
            }
            56 => {
                if lookahead == 10 { state = 915; lexer.advance(false); continue; }
                return result;
            }
            57 => {
                if lookahead == 10 { state = 915; lexer.advance(false); continue; }
                if lookahead == 13 { state = 56; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 405; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            58 => {
                if lookahead == 10 { state = 907; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                if lookahead == 10 { state = 907; lexer.advance(false); continue; }
                if lookahead == 13 { state = 58; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 279; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            60 => {
                if lookahead == 10 { state = 458; lexer.advance(true); continue; }
                return result;
            }
            61 => {
                if lookahead == 10 { state = 916; lexer.advance(false); continue; }
                return result;
            }
            62 => {
                if lookahead == 10 { state = 916; lexer.advance(false); continue; }
                if lookahead == 13 { state = 61; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 406; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                if lookahead == 10 { state = 454; lexer.advance(true); continue; }
                return result;
            }
            64 => {
                if lookahead == 10 { state = 917; lexer.advance(false); continue; }
                return result;
            }
            65 => {
                if lookahead == 10 { state = 917; lexer.advance(false); continue; }
                if lookahead == 13 { state = 64; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 407; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            66 => {
                if lookahead == 10 { state = 452; lexer.advance(true); continue; }
                return result;
            }
            67 => {
                if lookahead == 10 { state = 908; lexer.advance(false); continue; }
                return result;
            }
            68 => {
                if lookahead == 10 { state = 908; lexer.advance(false); continue; }
                if lookahead == 13 { state = 67; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 281; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            69 => {
                if lookahead == 10 { state = 924; lexer.advance(false); continue; }
                return result;
            }
            70 => {
                if lookahead == 10 { state = 924; lexer.advance(false); continue; }
                if lookahead == 13 { state = 69; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 424; lexer.advance(true); continue; }
                return result;
            }
            71 => {
                if lookahead == 10 { state = 923; lexer.advance(false); continue; }
                return result;
            }
            72 => {
                if lookahead == 10 { state = 923; lexer.advance(false); continue; }
                if lookahead == 13 { state = 71; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 423; lexer.advance(true); continue; }
                return result;
            }
            73 => {
                if lookahead == 10 { state = 460; lexer.advance(true); continue; }
                return result;
            }
            74 => {
                if lookahead == 10 { state = 428; lexer.advance(true); continue; }
                return result;
            }
            75 => {
                if lookahead == 10 { state = 918; lexer.advance(false); continue; }
                return result;
            }
            76 => {
                if lookahead == 10 { state = 918; lexer.advance(false); continue; }
                if lookahead == 13 { state = 75; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 408; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            77 => {
                if lookahead == 10 { state = 922; lexer.advance(false); continue; }
                return result;
            }
            78 => {
                if lookahead == 10 { state = 922; lexer.advance(false); continue; }
                if lookahead == 13 { state = 77; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 422; lexer.advance(true); continue; }
                return result;
            }
            79 => {
                if lookahead == 10 { state = 418; lexer.advance(true); continue; }
                return result;
            }
            80 => {
                if lookahead == 10 { state = 429; lexer.advance(true); continue; }
                return result;
            }
            81 => {
                if lookahead == 10 { state = 926; lexer.advance(false); continue; }
                return result;
            }
            82 => {
                if lookahead == 10 { state = 926; lexer.advance(false); continue; }
                if lookahead == 13 { state = 81; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 426; lexer.advance(true); continue; }
                return result;
            }
            83 => {
                if lookahead == 10 { state = 419; lexer.advance(true); continue; }
                return result;
            }
            84 => {
                if lookahead == 10 { state = 431; lexer.advance(true); continue; }
                return result;
            }
            85 => {
                if lookahead == 10 { state = 437; lexer.advance(true); continue; }
                return result;
            }
            86 => {
                if lookahead == 10 { state = 914; lexer.advance(false); continue; }
                return result;
            }
            87 => {
                if lookahead == 10 { state = 914; lexer.advance(false); continue; }
                if lookahead == 13 { state = 86; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 404; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            88 => {
                if lookahead == 10 { state = 432; lexer.advance(true); continue; }
                return result;
            }
            89 => {
                if lookahead == 10 { state = 294; lexer.advance(true); continue; }
                return result;
            }
            90 => {
                if lookahead == 10 { state = 434; lexer.advance(true); continue; }
                return result;
            }
            91 => {
                if lookahead == 10 { state = 92; lexer.advance(true); continue; }
                return result;
            }
            92 => {
                if let Some(next) = advance_map(&[
                    (10, 684), (33, 500), (35, 857), (37, 644), (38, 594), (42, 633), (43, 625), (44, 550),
                    (45, 629), (47, 639), (59, 547), (60, 609), (61, 555), (62, 614),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 92 { state = 361; lexer.advance(true); continue; }
                if lookahead == 94 { state = 591; lexer.advance(false); continue; }
                if lookahead == 124 { state = 588; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 92; lexer.advance(true); continue; }
                return result;
            }
            93 => {
                if let Some(next) = advance_map(&[
                    (10, 684), (34, 783), (35, 857), (36, 780), (38, 593), (40, 647), (43, 494), (44, 550),
                    (45, 496), (48, 804), (59, 547),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 92 { state = 375; lexer.advance(true); continue; }
                if lookahead == 96 { state = 850; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 93; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 806; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 { state = 650; lexer.advance(false); continue; }
                return result;
            }
            94 => {
                if let Some(next) = advance_map(&[
                    (10, 684), (35, 857), (36, 778), (38, 483), (40, 647), (45, 498), (48, 803), (58, 756),
                    (60, 605), (62, 612),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 92 { state = 385; lexer.advance(true); continue; }
                if lookahead == 96 { state = 850; lexer.advance(false); continue; }
                if lookahead == 124 { state = 510; lexer.advance(false); continue; }
                if lookahead == 125 { state = 813; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 94; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 805; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            95 => {
                if lookahead == 10 { state = 684; lexer.advance(false); continue; }
                if lookahead == 35 { state = 857; lexer.advance(false); continue; }
                if lookahead == 38 { state = 596; lexer.advance(false); continue; }
                if lookahead == 59 { state = 548; lexer.advance(false); continue; }
                if lookahead == 60 { state = 606; lexer.advance(false); continue; }
                if lookahead == 62 { state = 612; lexer.advance(false); continue; }
                if lookahead == 92 { state = 374; lexer.advance(true); continue; }
                if lookahead == 96 { state = 504; lexer.advance(false); continue; }
                if lookahead == 101 { state = 508; lexer.advance(false); continue; }
                if lookahead == 124 { state = 587; lexer.advance(false); continue; }
                if 91 <= lookahead && lookahead <= 93 || 123 <= lookahead && lookahead <= 125 { state = 781; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 95; lexer.advance(true); continue; }
                return result;
            }
            96 => {
                if lookahead == 10 { state = 684; lexer.advance(false); continue; }
                if lookahead == 35 { state = 857; lexer.advance(false); continue; }
                if lookahead == 38 { state = 596; lexer.advance(false); continue; }
                if lookahead == 59 { state = 548; lexer.advance(false); continue; }
                if lookahead == 60 { state = 607; lexer.advance(false); continue; }
                if lookahead == 62 { state = 612; lexer.advance(false); continue; }
                if lookahead == 92 { state = 376; lexer.advance(true); continue; }
                if lookahead == 96 { state = 504; lexer.advance(false); continue; }
                if lookahead == 101 { state = 508; lexer.advance(false); continue; }
                if lookahead == 124 { state = 587; lexer.advance(false); continue; }
                if 91 <= lookahead && lookahead <= 93 || 123 <= lookahead && lookahead <= 125 { state = 781; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 96; lexer.advance(true); continue; }
                return result;
            }
            97 => {
                if lookahead == 10 { state = 684; lexer.advance(false); continue; }
                if lookahead == 35 { state = 857; lexer.advance(false); continue; }
                if lookahead == 38 { state = 483; lexer.advance(false); continue; }
                if lookahead == 60 { state = 605; lexer.advance(false); continue; }
                if lookahead == 62 { state = 612; lexer.advance(false); continue; }
                if lookahead == 92 { state = 386; lexer.advance(true); continue; }
                if lookahead == 96 { state = 504; lexer.advance(false); continue; }
                if lookahead == 124 { state = 510; lexer.advance(false); continue; }
                if 91 <= lookahead && lookahead <= 93 || 123 <= lookahead && lookahead <= 125 { state = 781; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 97; lexer.advance(true); continue; }
                return result;
            }
            98 => {
                if lookahead == 10 { state = 684; lexer.advance(false); continue; }
                if lookahead == 35 { state = 857; lexer.advance(false); continue; }
                if lookahead == 38 { state = 593; lexer.advance(false); continue; }
                if lookahead == 59 { state = 549; lexer.advance(false); continue; }
                if lookahead == 92 { state = 389; lexer.advance(true); continue; }
                if lookahead == 96 { state = 504; lexer.advance(false); continue; }
                if lookahead == 105 { state = 507; lexer.advance(false); continue; }
                if 91 <= lookahead && lookahead <= 93 || lookahead == 123 || lookahead == 125 { state = 781; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 98; lexer.advance(true); continue; }
                return result;
            }
            99 => {
                if lookahead == 10 { state = 876; lexer.advance(false); continue; }
                return result;
            }
            100 => {
                if lookahead == 10 { state = 876; lexer.advance(false); continue; }
                if lookahead == 13 { state = 99; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 202; lexer.advance(true); continue; }
                return result;
            }
            101 => {
                if lookahead == 10 { state = 436; lexer.advance(true); continue; }
                return result;
            }
            102 => {
                if lookahead == 10 { state = 433; lexer.advance(true); continue; }
                return result;
            }
            103 => {
                if lookahead == 10 { state = 880; lexer.advance(false); continue; }
                return result;
            }
            104 => {
                if lookahead == 10 { state = 880; lexer.advance(false); continue; }
                if lookahead == 13 { state = 103; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 237; lexer.advance(true); continue; }
                return result;
            }
            105 => {
                if lookahead == 10 { state = 882; lexer.advance(false); continue; }
                return result;
            }
            106 => {
                if lookahead == 10 { state = 882; lexer.advance(false); continue; }
                if lookahead == 13 { state = 105; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 240; lexer.advance(true); continue; }
                return result;
            }
            107 => {
                if lookahead == 10 { state = 466; lexer.advance(true); continue; }
                return result;
            }
            108 => {
                if lookahead == 10 { state = 888; lexer.advance(false); continue; }
                return result;
            }
            109 => {
                if lookahead == 10 { state = 888; lexer.advance(false); continue; }
                if lookahead == 13 { state = 108; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 249; lexer.advance(true); continue; }
                return result;
            }
            110 => {
                if lookahead == 10 { state = 296; lexer.advance(true); continue; }
                return result;
            }
            111 => {
                if lookahead == 10 { state = 476; lexer.advance(true); continue; }
                return result;
            }
            112 => {
                if lookahead == 10 { state = 467; lexer.advance(true); continue; }
                return result;
            }
            113 => {
                if lookahead == 10 { state = 472; lexer.advance(true); continue; }
                return result;
            }
            114 => {
                if lookahead == 10 { state = 911; lexer.advance(false); continue; }
                return result;
            }
            115 => {
                if lookahead == 10 { state = 911; lexer.advance(false); continue; }
                if lookahead == 13 { state = 114; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 287; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            116 => {
                if lookahead == 10 { state = 445; lexer.advance(true); continue; }
                return result;
            }
            117 => {
                if lookahead == 10 { state = 927; lexer.advance(false); continue; }
                return result;
            }
            118 => {
                if lookahead == 10 { state = 927; lexer.advance(false); continue; }
                if lookahead == 13 { state = 117; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 438; lexer.advance(true); continue; }
                return result;
            }
            119 => {
                if lookahead == 10 { state = 896; lexer.advance(false); continue; }
                return result;
            }
            120 => {
                if lookahead == 10 { state = 896; lexer.advance(false); continue; }
                if lookahead == 13 { state = 119; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 261; lexer.advance(true); continue; }
                return result;
            }
            121 => {
                if lookahead == 10 { state = 928; lexer.advance(false); continue; }
                return result;
            }
            122 => {
                if lookahead == 10 { state = 928; lexer.advance(false); continue; }
                if lookahead == 13 { state = 121; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 439; lexer.advance(true); continue; }
                return result;
            }
            123 => {
                if lookahead == 10 { state = 921; lexer.advance(false); continue; }
                return result;
            }
            124 => {
                if lookahead == 10 { state = 921; lexer.advance(false); continue; }
                if lookahead == 13 { state = 123; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 411; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            125 => {
                if lookahead == 10 { state = 473; lexer.advance(true); continue; }
                return result;
            }
            126 => {
                if lookahead == 10 { state = 912; lexer.advance(false); continue; }
                return result;
            }
            127 => {
                if lookahead == 10 { state = 912; lexer.advance(false); continue; }
                if lookahead == 13 { state = 126; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 289; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            128 => {
                if lookahead == 10 { state = 95; lexer.advance(true); continue; }
                return result;
            }
            129 => {
                if lookahead == 10 { state = 93; lexer.advance(true); continue; }
                return result;
            }
            130 => {
                if lookahead == 10 { state = 468; lexer.advance(true); continue; }
                return result;
            }
            131 => {
                if lookahead == 10 { state = 448; lexer.advance(true); continue; }
                return result;
            }
            132 => {
                if lookahead == 10 { state = 480; lexer.advance(true); continue; }
                return result;
            }
            133 => {
                if lookahead == 10 { state = 477; lexer.advance(true); continue; }
                return result;
            }
            134 => {
                if lookahead == 10 { state = 478; lexer.advance(true); continue; }
                return result;
            }
            135 => {
                if lookahead == 10 { state = 469; lexer.advance(true); continue; }
                return result;
            }
            136 => {
                if lookahead == 10 { state = 481; lexer.advance(true); continue; }
                return result;
            }
            137 => {
                if lookahead == 10 { state = 94; lexer.advance(true); continue; }
                return result;
            }
            138 => {
                if lookahead == 10 { state = 97; lexer.advance(true); continue; }
                return result;
            }
            139 => {
                if lookahead == 10 { state = 902; lexer.advance(false); continue; }
                return result;
            }
            140 => {
                if lookahead == 10 { state = 902; lexer.advance(false); continue; }
                if lookahead == 13 { state = 139; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 270; lexer.advance(true); continue; }
                return result;
            }
            141 => {
                if lookahead == 10 { state = 931; lexer.advance(false); continue; }
                return result;
            }
            142 => {
                if lookahead == 10 { state = 931; lexer.advance(false); continue; }
                if lookahead == 13 { state = 141; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 442; lexer.advance(true); continue; }
                return result;
            }
            143 => {
                if lookahead == 10 { state = 449; lexer.advance(true); continue; }
                return result;
            }
            144 => {
                if lookahead == 10 { state = 788; lexer.advance(false); continue; }
                if lookahead == 13 { state = 785; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 788; lexer.advance(false); continue; }
                if lookahead != 0 { state = 792; lexer.advance(false); continue; }
                return result;
            }
            145 => {
                if lookahead == 10 { state = 932; lexer.advance(false); continue; }
                return result;
            }
            146 => {
                if lookahead == 10 { state = 932; lexer.advance(false); continue; }
                if lookahead == 13 { state = 145; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 444; lexer.advance(true); continue; }
                return result;
            }
            147 => {
                if lookahead == 10 { state = 789; lexer.advance(false); continue; }
                if lookahead == 13 { state = 786; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 789; lexer.advance(false); continue; }
                if lookahead != 0 { state = 792; lexer.advance(false); continue; }
                return result;
            }
            148 => {
                if lookahead == 10 { state = 479; lexer.advance(true); continue; }
                return result;
            }
            149 => {
                if lookahead == 10 { state = 933; lexer.advance(false); continue; }
                return result;
            }
            150 => {
                if lookahead == 10 { state = 933; lexer.advance(false); continue; }
                if lookahead == 13 { state = 149; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 447; lexer.advance(true); continue; }
                return result;
            }
            151 => {
                if lookahead == 10 { state = 98; lexer.advance(true); continue; }
                return result;
            }
            152 => {
                if lookahead == 10 { state = 470; lexer.advance(true); continue; }
                return result;
            }
            153 => {
                if lookahead == 10 { state = 471; lexer.advance(true); continue; }
                return result;
            }
            154 => {
                if lookahead == 10 { state = 482; lexer.advance(true); continue; }
                return result;
            }
            155 => {
                if lookahead == 10 { state = 402; lexer.advance(true); continue; }
                return result;
            }
            156 => {
                if let Some(next) = advance_map(&[
                    (10, 685), (33, 942), (34, 783), (35, 857), (36, 776), (37, 643), (38, 595), (39, 485),
                    (40, 647), (41, 649), (42, 632), (43, 624), (45, 627), (47, 638), (48, 797), (59, 549),
                    (60, 601), (61, 554), (62, 610), (63, 755), (92, 312), (94, 590), (96, 851), (124, 586),
                    (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 156; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            157 => {
                if lookahead == 10 { state = 233; lexer.advance(true); continue; }
                return result;
            }
            158 => {
                if lookahead == 10 { state = 421; lexer.advance(true); continue; }
                return result;
            }
            159 => {
                if lookahead == 10 { state = 274; lexer.advance(true); continue; }
                return result;
            }
            160 => {
                if lookahead == 10 { state = 276; lexer.advance(true); continue; }
                return result;
            }
            161 => {
                if lookahead == 10 { state = 278; lexer.advance(true); continue; }
                return result;
            }
            162 => {
                if lookahead == 10 { state = 282; lexer.advance(true); continue; }
                return result;
            }
            163 => {
                if lookahead == 10 { state = 885; lexer.advance(false); continue; }
                return result;
            }
            164 => {
                if lookahead == 10 { state = 885; lexer.advance(false); continue; }
                if lookahead == 13 { state = 163; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 244; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            165 => {
                if lookahead == 10 { state = 286; lexer.advance(true); continue; }
                return result;
            }
            166 => {
                if lookahead == 10 { state = 291; lexer.advance(true); continue; }
                return result;
            }
            167 => {
                if lookahead == 10 { state = 889; lexer.advance(false); continue; }
                return result;
            }
            168 => {
                if lookahead == 10 { state = 889; lexer.advance(false); continue; }
                if lookahead == 13 { state = 167; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 250; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            169 => {
                if lookahead == 10 { state = 280; lexer.advance(true); continue; }
                return result;
            }
            170 => {
                if lookahead == 10 { state = 284; lexer.advance(true); continue; }
                return result;
            }
            171 => {
                if lookahead == 10 { state = 290; lexer.advance(true); continue; }
                return result;
            }
            172 => {
                if lookahead == 10 { state = 456; lexer.advance(true); continue; }
                return result;
            }
            173 => {
                if lookahead == 10 { state = 899; lexer.advance(false); continue; }
                return result;
            }
            174 => {
                if lookahead == 10 { state = 899; lexer.advance(false); continue; }
                if lookahead == 13 { state = 173; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 265; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            175 => {
                if lookahead == 10 { state = 295; lexer.advance(true); continue; }
                return result;
            }
            176 => {
                if lookahead == 10 { state = 465; lexer.advance(true); continue; }
                return result;
            }
            177 => {
                if lookahead == 10 { state = 459; lexer.advance(true); continue; }
                return result;
            }
            178 => {
                if lookahead == 10 { state = 457; lexer.advance(true); continue; }
                return result;
            }
            179 => {
                if lookahead == 10 { state = 910; lexer.advance(false); continue; }
                return result;
            }
            180 => {
                if lookahead == 10 { state = 910; lexer.advance(false); continue; }
                if lookahead == 13 { state = 179; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 285; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            181 => {
                if lookahead == 10 { state = 925; lexer.advance(false); continue; }
                return result;
            }
            182 => {
                if lookahead == 10 { state = 925; lexer.advance(false); continue; }
                if lookahead == 13 { state = 181; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 425; lexer.advance(true); continue; }
                return result;
            }
            183 => {
                if lookahead == 10 { state = 462; lexer.advance(true); continue; }
                return result;
            }
            184 => {
                if lookahead == 10 { state = 878; lexer.advance(false); continue; }
                return result;
            }
            185 => {
                if lookahead == 10 { state = 878; lexer.advance(false); continue; }
                if lookahead == 13 { state = 184; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 234; lexer.advance(true); continue; }
                return result;
            }
            186 => {
                if lookahead == 10 { state = 430; lexer.advance(true); continue; }
                return result;
            }
            187 => {
                if lookahead == 10 { state = 884; lexer.advance(false); continue; }
                return result;
            }
            188 => {
                if lookahead == 10 { state = 884; lexer.advance(false); continue; }
                if lookahead == 13 { state = 187; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 243; lexer.advance(true); continue; }
                return result;
            }
            189 => {
                if lookahead == 10 { state = 894; lexer.advance(false); continue; }
                return result;
            }
            190 => {
                if lookahead == 10 { state = 894; lexer.advance(false); continue; }
                if lookahead == 13 { state = 189; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 258; lexer.advance(true); continue; }
                return result;
            }
            191 => {
                if lookahead == 10 { state = 890; lexer.advance(false); continue; }
                return result;
            }
            192 => {
                if lookahead == 10 { state = 890; lexer.advance(false); continue; }
                if lookahead == 13 { state = 191; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 252; lexer.advance(true); continue; }
                return result;
            }
            193 => {
                if lookahead == 10 { state = 297; lexer.advance(true); continue; }
                return result;
            }
            194 => {
                if lookahead == 10 { state = 929; lexer.advance(false); continue; }
                return result;
            }
            195 => {
                if lookahead == 10 { state = 929; lexer.advance(false); continue; }
                if lookahead == 13 { state = 194; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 440; lexer.advance(true); continue; }
                return result;
            }
            196 => {
                if lookahead == 10 { state = 898; lexer.advance(false); continue; }
                return result;
            }
            197 => {
                if lookahead == 10 { state = 898; lexer.advance(false); continue; }
                if lookahead == 13 { state = 196; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 264; lexer.advance(true); continue; }
                return result;
            }
            198 => {
                if lookahead == 10 { state = 930; lexer.advance(false); continue; }
                return result;
            }
            199 => {
                if lookahead == 10 { state = 930; lexer.advance(false); continue; }
                if lookahead == 13 { state = 198; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 441; lexer.advance(true); continue; }
                return result;
            }
            200 => {
                if lookahead == 10 { state = 96; lexer.advance(true); continue; }
                return result;
            }
            201 => {
                if lookahead == 10 { state = 403; lexer.advance(true); continue; }
                return result;
            }
            202 => {
                if let Some(next) = advance_map(&[
                    (10, 686), (33, 663), (34, 783), (35, 811), (36, 775), (38, 596), (42, 631), (45, 626),
                    (59, 548), (60, 606), (62, 612), (63, 754), (64, 935), (92, 100), (95, 939), (101, 872),
                    (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 202; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            203 => {
                if lookahead == 10 { state = 288; lexer.advance(true); continue; }
                return result;
            }
            204 => {
                if lookahead == 10 { state = 292; lexer.advance(true); continue; }
                return result;
            }
            205 => {
                if lookahead == 10 { state = 293; lexer.advance(true); continue; }
                return result;
            }
            206 => {
                if lookahead == 10 { state = 905; lexer.advance(false); continue; }
                return result;
            }
            207 => {
                if lookahead == 10 { state = 905; lexer.advance(false); continue; }
                if lookahead == 13 { state = 206; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 275; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            208 => {
                if lookahead == 10 { state = 461; lexer.advance(true); continue; }
                return result;
            }
            209 => {
                if lookahead == 10 { state = 453; lexer.advance(true); continue; }
                return result;
            }
            210 => {
                if lookahead == 10 { state = 886; lexer.advance(false); continue; }
                return result;
            }
            211 => {
                if lookahead == 10 { state = 886; lexer.advance(false); continue; }
                if lookahead == 13 { state = 210; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 246; lexer.advance(true); continue; }
                return result;
            }
            212 => {
                if lookahead == 10 { state = 900; lexer.advance(false); continue; }
                return result;
            }
            213 => {
                if lookahead == 10 { state = 900; lexer.advance(false); continue; }
                if lookahead == 13 { state = 212; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 267; lexer.advance(true); continue; }
                return result;
            }
            214 => {
                if lookahead == 10 { state = 892; lexer.advance(false); continue; }
                return result;
            }
            215 => {
                if lookahead == 10 { state = 892; lexer.advance(false); continue; }
                if lookahead == 13 { state = 214; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 255; lexer.advance(true); continue; }
                return result;
            }
            216 => {
                if let Some(next) = advance_map(&[
                    (10, 687), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (40, 647),
                    (42, 634), (45, 630), (48, 801), (59, 548), (60, 603), (61, 943), (62, 611), (63, 755),
                    (64, 936), (92, 28), (95, 938), (96, 850), (101, 866), (124, 587), (91, 781), (93, 781),
                    (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 216; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            217 => {
                if lookahead == 10 { state = 263; lexer.advance(true); continue; }
                return result;
            }
            218 => {
                if lookahead == 10 { state = 266; lexer.advance(true); continue; }
                return result;
            }
            219 => {
                if lookahead == 10 { state = 272; lexer.advance(true); continue; }
                return result;
            }
            220 => {
                if lookahead == 10 { state = 897; lexer.advance(false); continue; }
                return result;
            }
            221 => {
                if lookahead == 10 { state = 897; lexer.advance(false); continue; }
                if lookahead == 13 { state = 220; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 262; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            222 => {
                if lookahead == 10 { state = 903; lexer.advance(false); continue; }
                return result;
            }
            223 => {
                if lookahead == 10 { state = 903; lexer.advance(false); continue; }
                if lookahead == 13 { state = 222; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 271; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            224 => {
                if lookahead == 10 { state = 906; lexer.advance(false); continue; }
                return result;
            }
            225 => {
                if lookahead == 10 { state = 906; lexer.advance(false); continue; }
                if lookahead == 13 { state = 224; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 277; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            226 => {
                if lookahead == 10 { state = 463; lexer.advance(true); continue; }
                return result;
            }
            227 => {
                if lookahead == 10 { state = 919; lexer.advance(false); continue; }
                return result;
            }
            228 => {
                if lookahead == 10 { state = 919; lexer.advance(false); continue; }
                if lookahead == 13 { state = 227; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 409; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            229 => {
                if lookahead == 10 { state = 909; lexer.advance(false); continue; }
                return result;
            }
            230 => {
                if lookahead == 10 { state = 909; lexer.advance(false); continue; }
                if lookahead == 13 { state = 229; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 283; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            231 => {
                if lookahead == 10 { state = 920; lexer.advance(false); continue; }
                return result;
            }
            232 => {
                if lookahead == 10 { state = 920; lexer.advance(false); continue; }
                if lookahead == 13 { state = 231; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 410; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            233 => {
                if let Some(next) = advance_map(&[
                    (10, 688), (33, 942), (34, 783), (35, 857), (36, 776), (37, 643), (38, 595), (39, 485),
                    (40, 647), (41, 649), (42, 632), (43, 624), (45, 627), (47, 638), (48, 797), (59, 549),
                    (60, 601), (61, 554), (62, 610), (63, 755), (92, 313), (94, 590), (96, 850), (124, 586),
                    (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 233; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            234 => {
                if let Some(next) = advance_map(&[
                    (10, 689), (33, 663), (34, 783), (35, 811), (36, 775), (38, 596), (42, 631), (45, 626),
                    (59, 548), (60, 607), (62, 612), (63, 754), (64, 935), (92, 185), (95, 939), (101, 872),
                    (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 234; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            235 => {
                if let Some(next) = advance_map(&[
                    (10, 690), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (40, 647),
                    (42, 634), (45, 630), (48, 801), (59, 548), (60, 603), (61, 943), (62, 611), (63, 755),
                    (64, 936), (92, 30), (95, 938), (96, 850), (124, 587), (91, 781), (93, 781), (123, 781),
                    (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 235; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            236 => {
                if let Some(next) = advance_map(&[
                    (10, 691), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (40, 647), (45, 941),
                    (48, 797), (59, 548), (60, 603), (61, 943), (62, 611), (92, 317), (96, 850), (101, 947),
                    (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 236; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            237 => {
                if let Some(next) = advance_map(&[
                    (10, 692), (33, 663), (34, 783), (35, 811), (36, 775), (38, 596), (42, 631), (45, 626),
                    (59, 548), (60, 606), (62, 612), (63, 754), (64, 935), (92, 104), (95, 939), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 237; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            238 => {
                if let Some(next) = advance_map(&[
                    (10, 693), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (42, 634),
                    (45, 630), (48, 801), (59, 548), (60, 603), (61, 943), (62, 611), (63, 755), (64, 936),
                    (92, 32), (95, 938), (96, 850), (101, 866), (124, 587), (91, 781), (93, 781), (123, 781),
                    (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 238; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            239 => {
                if let Some(next) = advance_map(&[
                    (10, 694), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (40, 647), (45, 941),
                    (48, 797), (59, 548), (60, 603), (61, 943), (62, 611), (92, 318), (96, 850), (124, 587),
                    (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 239; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            240 => {
                if let Some(next) = advance_map(&[
                    (10, 695), (33, 663), (34, 783), (35, 811), (36, 775), (38, 596), (42, 631), (45, 626),
                    (59, 549), (60, 606), (62, 612), (63, 754), (64, 935), (92, 106), (95, 939), (96, 850),
                    (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 240; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            241 => {
                if let Some(next) = advance_map(&[
                    (10, 696), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (42, 634),
                    (45, 630), (48, 801), (59, 548), (60, 603), (61, 943), (62, 611), (63, 755), (64, 936),
                    (92, 36), (95, 938), (96, 850), (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 241; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            242 => {
                if let Some(next) = advance_map(&[
                    (10, 697), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (40, 647), (41, 649),
                    (45, 941), (48, 797), (59, 549), (60, 603), (61, 943), (62, 611), (92, 319), (96, 850),
                    (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 242; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            243 => {
                if let Some(next) = advance_map(&[
                    (10, 698), (33, 663), (34, 783), (35, 811), (36, 775), (38, 596), (42, 631), (45, 626),
                    (59, 548), (60, 607), (62, 612), (63, 754), (64, 935), (92, 188), (95, 939), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 243; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            244 => {
                if let Some(next) = advance_map(&[
                    (10, 699), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (40, 647),
                    (42, 634), (45, 630), (48, 801), (59, 549), (60, 603), (61, 943), (62, 611), (63, 755),
                    (64, 936), (92, 164), (95, 938), (96, 850), (124, 587), (91, 781), (93, 781), (123, 781),
                    (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 244; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            245 => {
                if let Some(next) = advance_map(&[
                    (10, 700), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (45, 941), (48, 798),
                    (59, 548), (60, 604), (62, 611), (92, 320), (96, 850), (101, 861), (124, 587), (91, 781),
                    (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 245; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 800; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 { state = 863; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            246 => {
                if let Some(next) = advance_map(&[
                    (10, 701), (33, 663), (34, 783), (35, 811), (36, 775), (38, 596), (42, 631), (45, 626),
                    (59, 549), (60, 606), (62, 612), (63, 754), (64, 935), (92, 211), (95, 939), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 246; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            247 => {
                if let Some(next) = advance_map(&[
                    (10, 702), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (40, 647),
                    (41, 649), (42, 634), (45, 630), (48, 801), (59, 549), (60, 603), (61, 943), (62, 611),
                    (63, 755), (64, 936), (92, 39), (95, 938), (96, 850), (124, 587), (91, 781), (93, 781),
                    (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 247; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            248 => {
                if let Some(next) = advance_map(&[
                    (10, 703), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (45, 941), (48, 798),
                    (59, 548), (60, 604), (62, 611), (92, 321), (96, 850), (124, 587), (91, 781), (93, 781),
                    (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 248; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 800; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 { state = 863; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            249 => {
                if let Some(next) = advance_map(&[
                    (10, 704), (33, 663), (34, 783), (35, 811), (36, 775), (38, 596), (41, 649), (42, 631),
                    (45, 626), (59, 549), (60, 606), (62, 612), (63, 754), (64, 935), (92, 109), (95, 939),
                    (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 249; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            250 => {
                if let Some(next) = advance_map(&[
                    (10, 705), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (42, 634),
                    (45, 630), (48, 801), (59, 549), (60, 603), (61, 943), (62, 611), (63, 755), (64, 936),
                    (92, 168), (95, 938), (96, 850), (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 250; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            251 => {
                if let Some(next) = advance_map(&[
                    (10, 706), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (41, 649), (45, 941),
                    (48, 798), (59, 549), (60, 604), (62, 611), (92, 322), (96, 850), (124, 587), (91, 781),
                    (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 251; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 800; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 { state = 863; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            252 => {
                if let Some(next) = advance_map(&[
                    (10, 707), (33, 663), (34, 783), (35, 811), (36, 775), (38, 596), (41, 649), (42, 631),
                    (45, 626), (59, 549), (60, 607), (62, 612), (63, 754), (64, 935), (92, 192), (95, 939),
                    (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 252; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            253 => {
                if let Some(next) = advance_map(&[
                    (10, 708), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (41, 649),
                    (42, 634), (45, 630), (48, 801), (59, 549), (60, 603), (61, 943), (62, 611), (63, 755),
                    (64, 936), (92, 41), (95, 938), (96, 850), (124, 587), (91, 781), (93, 781), (123, 781),
                    (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 253; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            254 => {
                if let Some(next) = advance_map(&[
                    (10, 709), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (45, 941), (48, 797),
                    (59, 548), (60, 603), (62, 611), (92, 323), (96, 850), (101, 947), (124, 587), (91, 781),
                    (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 254; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            255 => {
                if let Some(next) = advance_map(&[
                    (10, 710), (33, 663), (34, 783), (35, 811), (36, 775), (38, 596), (42, 631), (45, 626),
                    (59, 549), (60, 607), (62, 612), (63, 754), (64, 935), (92, 215), (95, 939), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 255; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            256 => {
                if let Some(next) = advance_map(&[
                    (10, 711), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (42, 634),
                    (45, 630), (48, 801), (59, 548), (60, 603), (62, 611), (63, 755), (64, 936), (92, 47),
                    (95, 938), (96, 850), (101, 866), (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 256; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            257 => {
                if let Some(next) = advance_map(&[
                    (10, 712), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (45, 941), (48, 797),
                    (59, 548), (60, 603), (62, 611), (92, 324), (96, 850), (124, 587), (91, 781), (93, 781),
                    (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 257; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            258 => {
                if let Some(next) = advance_map(&[
                    (10, 713), (33, 663), (34, 783), (35, 811), (36, 775), (38, 596), (42, 631), (45, 626),
                    (59, 549), (60, 607), (62, 612), (63, 754), (64, 935), (92, 190), (95, 939), (96, 850),
                    (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 258; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            259 => {
                if let Some(next) = advance_map(&[
                    (10, 714), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (42, 634),
                    (45, 630), (48, 801), (59, 548), (60, 603), (62, 611), (63, 755), (64, 936), (92, 49),
                    (95, 938), (96, 850), (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 259; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            260 => {
                if let Some(next) = advance_map(&[
                    (10, 715), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (41, 649), (45, 941),
                    (48, 797), (59, 549), (60, 603), (62, 611), (92, 326), (96, 850), (124, 587), (91, 781),
                    (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 260; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            261 => {
                if let Some(next) = advance_map(&[
                    (10, 716), (33, 663), (34, 783), (35, 811), (36, 775), (38, 483), (42, 631), (45, 626),
                    (60, 606), (62, 612), (63, 754), (64, 935), (92, 120), (95, 939), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 261; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            262 => {
                if let Some(next) = advance_map(&[
                    (10, 717), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (42, 634),
                    (45, 630), (48, 801), (59, 548), (60, 604), (62, 611), (63, 755), (64, 936), (92, 221),
                    (95, 938), (96, 850), (101, 866), (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 262; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            263 => {
                if let Some(next) = advance_map(&[
                    (10, 718), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (45, 941), (48, 797),
                    (59, 548), (60, 604), (62, 611), (92, 393), (96, 850), (101, 947), (124, 587), (91, 781),
                    (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 263; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            264 => {
                if let Some(next) = advance_map(&[
                    (10, 719), (33, 663), (34, 783), (35, 811), (36, 775), (38, 483), (42, 631), (45, 626),
                    (60, 607), (62, 612), (63, 754), (64, 935), (92, 197), (95, 939), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 264; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            265 => {
                if let Some(next) = advance_map(&[
                    (10, 720), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (42, 634),
                    (45, 630), (48, 801), (59, 549), (60, 603), (62, 611), (63, 755), (64, 936), (92, 174),
                    (95, 938), (96, 850), (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 265; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            266 => {
                if let Some(next) = advance_map(&[
                    (10, 721), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (45, 941), (48, 797),
                    (59, 548), (60, 604), (62, 611), (92, 394), (96, 850), (124, 587), (91, 781), (93, 781),
                    (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 266; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            267 => {
                if let Some(next) = advance_map(&[
                    (10, 722), (33, 663), (34, 783), (35, 811), (36, 775), (38, 483), (42, 631), (45, 626),
                    (60, 605), (62, 612), (63, 754), (64, 935), (92, 213), (95, 939), (124, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 267; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            268 => {
                if let Some(next) = advance_map(&[
                    (10, 723), (33, 666), (34, 783), (35, 811), (36, 776), (38, 483), (39, 485), (40, 647),
                    (42, 634), (45, 630), (48, 801), (60, 603), (61, 943), (62, 611), (63, 755), (64, 936),
                    (92, 52), (95, 938), (96, 850), (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 268; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && (lookahead < 59 || 93 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            269 => {
                if let Some(next) = advance_map(&[
                    (10, 724), (34, 783), (35, 857), (36, 776), (38, 483), (39, 485), (45, 941), (48, 797),
                    (60, 602), (62, 611), (92, 328), (96, 850), (124, 587), (91, 781), (93, 781), (123, 781),
                    (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 269; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && lookahead != 59 && lookahead != 60 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            270 => {
                if let Some(next) = advance_map(&[
                    (10, 725), (33, 663), (34, 783), (35, 811), (36, 775), (38, 593), (42, 631), (45, 626),
                    (59, 549), (63, 754), (64, 935), (92, 140), (95, 939), (105, 871),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 270; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            271 => {
                if let Some(next) = advance_map(&[
                    (10, 726), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (42, 634),
                    (45, 630), (48, 801), (59, 548), (60, 604), (62, 611), (63, 755), (64, 936), (92, 223),
                    (95, 938), (96, 850), (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 271; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            272 => {
                if let Some(next) = advance_map(&[
                    (10, 727), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (41, 649), (45, 941),
                    (48, 797), (59, 549), (60, 604), (62, 611), (92, 395), (96, 850), (124, 587), (91, 781),
                    (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 272; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            273 => {
                if let Some(next) = advance_map(&[
                    (10, 728), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (41, 649),
                    (42, 634), (45, 630), (48, 801), (59, 549), (60, 603), (62, 611), (63, 755), (64, 936),
                    (92, 54), (95, 938), (96, 850), (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 273; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            274 => {
                if let Some(next) = advance_map(&[
                    (10, 729), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (40, 647), (45, 941),
                    (48, 797), (59, 548), (60, 603), (61, 943), (62, 611), (92, 330), (96, 851), (101, 947),
                    (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 274; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            275 => {
                if let Some(next) = advance_map(&[
                    (10, 730), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (42, 634),
                    (45, 630), (48, 801), (59, 549), (60, 604), (62, 611), (63, 755), (64, 936), (92, 207),
                    (95, 938), (96, 850), (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 275; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            276 => {
                if let Some(next) = advance_map(&[
                    (10, 731), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (40, 647), (45, 941),
                    (48, 797), (59, 548), (60, 603), (61, 943), (62, 611), (92, 332), (96, 851), (124, 587),
                    (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 276; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            277 => {
                if let Some(next) = advance_map(&[
                    (10, 732), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (41, 649),
                    (42, 634), (45, 630), (48, 801), (59, 549), (60, 604), (62, 611), (63, 755), (64, 936),
                    (92, 225), (95, 938), (96, 850), (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 277; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            278 => {
                if let Some(next) = advance_map(&[
                    (10, 733), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (40, 647), (41, 649),
                    (45, 941), (48, 797), (59, 549), (60, 603), (61, 943), (62, 611), (92, 334), (96, 851),
                    (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 278; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            279 => {
                if let Some(next) = advance_map(&[
                    (10, 734), (33, 666), (34, 783), (35, 811), (36, 776), (38, 483), (39, 485), (42, 634),
                    (45, 630), (48, 801), (60, 603), (61, 943), (62, 611), (63, 755), (64, 936), (92, 59),
                    (95, 938), (96, 850), (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 279; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && (lookahead < 59 || 93 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            280 => {
                if let Some(next) = advance_map(&[
                    (10, 735), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (45, 941), (48, 797),
                    (59, 548), (60, 603), (62, 611), (92, 336), (96, 851), (101, 947), (124, 587), (91, 781),
                    (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 280; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            281 => {
                if let Some(next) = advance_map(&[
                    (10, 736), (33, 666), (34, 783), (35, 811), (36, 776), (38, 483), (39, 485), (42, 634),
                    (45, 630), (48, 801), (60, 603), (62, 611), (63, 755), (64, 936), (92, 68), (95, 938),
                    (96, 850), (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 281; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && lookahead != 59 && lookahead != 60 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            282 => {
                if let Some(next) = advance_map(&[
                    (10, 737), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (45, 941), (48, 798),
                    (59, 548), (60, 604), (62, 611), (92, 337), (96, 851), (101, 861), (124, 587), (91, 781),
                    (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 282; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 800; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 { state = 863; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            283 => {
                if let Some(next) = advance_map(&[
                    (10, 738), (33, 666), (34, 783), (35, 811), (36, 776), (38, 483), (39, 485), (42, 634),
                    (45, 630), (48, 801), (60, 604), (62, 611), (63, 755), (64, 936), (92, 230), (95, 938),
                    (96, 850), (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 283; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && lookahead != 59 && lookahead != 60 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            284 => {
                if let Some(next) = advance_map(&[
                    (10, 739), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (45, 941), (48, 797),
                    (59, 548), (60, 603), (62, 611), (92, 341), (96, 851), (124, 587), (91, 781), (93, 781),
                    (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 284; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            285 => {
                if let Some(next) = advance_map(&[
                    (10, 740), (33, 666), (34, 783), (35, 811), (36, 776), (38, 483), (39, 485), (42, 634),
                    (45, 630), (48, 801), (60, 602), (62, 611), (63, 755), (64, 936), (92, 180), (95, 938),
                    (96, 850), (124, 510), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 285; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && lookahead != 59 && lookahead != 60 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            286 => {
                if let Some(next) = advance_map(&[
                    (10, 741), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (45, 941), (48, 798),
                    (59, 548), (60, 604), (62, 611), (92, 342), (96, 851), (124, 587), (91, 781), (93, 781),
                    (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 286; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 800; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 { state = 863; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            287 => {
                if let Some(next) = advance_map(&[
                    (10, 742), (33, 666), (34, 783), (35, 811), (36, 776), (38, 593), (39, 485), (42, 634),
                    (45, 630), (48, 801), (59, 549), (60, 489), (62, 490), (63, 755), (64, 936), (92, 115),
                    (95, 938), (96, 850), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 287; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            288 => {
                if let Some(next) = advance_map(&[
                    (10, 743), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (45, 941), (48, 797),
                    (59, 548), (60, 604), (62, 611), (92, 344), (96, 851), (101, 947), (124, 587), (91, 781),
                    (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 288; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            289 => {
                if let Some(next) = advance_map(&[
                    (10, 744), (33, 666), (34, 783), (35, 811), (36, 776), (39, 485), (42, 634), (45, 630),
                    (48, 801), (60, 489), (62, 490), (63, 755), (64, 936), (92, 127), (95, 938), (96, 850),
                    (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 289; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && lookahead != 59 && lookahead != 60 && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            290 => {
                if let Some(next) = advance_map(&[
                    (10, 745), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (41, 649), (45, 941),
                    (48, 797), (59, 549), (60, 603), (62, 611), (92, 345), (96, 851), (124, 587), (91, 781),
                    (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 290; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            291 => {
                if let Some(next) = advance_map(&[
                    (10, 746), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (41, 649), (45, 941),
                    (48, 798), (59, 549), (60, 604), (62, 611), (92, 346), (96, 851), (124, 587), (91, 781),
                    (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 291; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 800; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 { state = 863; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            292 => {
                if let Some(next) = advance_map(&[
                    (10, 747), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (45, 941), (48, 797),
                    (59, 548), (60, 604), (62, 611), (92, 347), (96, 851), (124, 587), (91, 781), (93, 781),
                    (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 292; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            293 => {
                if let Some(next) = advance_map(&[
                    (10, 748), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (41, 649), (45, 941),
                    (48, 797), (59, 549), (60, 604), (62, 611), (92, 349), (96, 851), (124, 587), (91, 781),
                    (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 293; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            294 => {
                if let Some(next) = advance_map(&[
                    (10, 749), (34, 783), (35, 857), (36, 776), (38, 593), (39, 485), (40, 647), (45, 941),
                    (48, 797), (59, 549), (60, 489), (62, 490), (92, 355), (96, 850), (101, 947), (91, 781),
                    (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 294; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && (lookahead < 123 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            295 => {
                if let Some(next) = advance_map(&[
                    (10, 750), (34, 783), (35, 857), (36, 776), (38, 483), (39, 485), (45, 941), (48, 797),
                    (60, 602), (62, 611), (92, 365), (96, 851), (124, 510), (91, 781), (93, 781), (123, 781),
                    (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 295; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && lookahead != 59 && lookahead != 60 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            296 => {
                if let Some(next) = advance_map(&[
                    (10, 751), (34, 783), (35, 857), (36, 776), (38, 593), (39, 485), (45, 941), (48, 797),
                    (59, 549), (60, 489), (62, 490), (92, 368), (96, 850), (91, 781), (93, 781), (123, 781),
                    (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 296; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && (lookahead < 123 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            297 => {
                if let Some(next) = advance_map(&[
                    (10, 752), (34, 783), (35, 857), (36, 776), (38, 593), (39, 485), (45, 941), (48, 797),
                    (59, 549), (60, 489), (62, 490), (92, 378), (96, 851), (91, 781), (93, 781), (123, 781),
                    (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 297; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && (lookahead < 123 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            298 => {
                if lookahead == 13 { state = 1; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 397; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            299 => {
                if lookahead == 13 { state = 784; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 790; lexer.advance(false); continue; }
                if lookahead != 0 { state = 792; lexer.advance(false); continue; }
                return result;
            }
            300 => {
                if lookahead == 13 { state = 793; lexer.advance(false); continue; }
                if lookahead != 0 { state = 792; lexer.advance(false); continue; }
                return result;
            }
            301 => {
                if lookahead == 13 { state = 2; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 412; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            302 => {
                if lookahead == 13 { state = 3; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 413; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            303 => {
                if lookahead == 13 { state = 4; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 414; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            304 => {
                if lookahead == 13 { state = 5; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 6; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            305 => {
                if lookahead == 13 { state = 7; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 399; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            306 => {
                if lookahead == 13 { state = 8; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 415; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            307 => {
                if lookahead == 13 { state = 9; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 416; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            308 => {
                if lookahead == 13 { state = 155; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 402; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            309 => {
                if lookahead == 13 { state = 10; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 400; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            310 => {
                if lookahead == 13 { state = 201; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 403; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            311 => {
                if lookahead == 13 { state = 11; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 401; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            312 => {
                if lookahead == 13 { state = 20; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 156; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            313 => {
                if lookahead == 13 { state = 157; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 233; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            314 => {
                if lookahead == 13 { state = 21; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 420; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            315 => {
                if lookahead == 13 { state = 158; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 421; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            316 => {
                if lookahead == 13 { state = 22; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 417; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            317 => {
                if lookahead == 13 { state = 23; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 236; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            318 => {
                if lookahead == 13 { state = 24; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 239; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            319 => {
                if lookahead == 13 { state = 25; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 242; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            320 => {
                if lookahead == 13 { state = 26; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 245; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            321 => {
                if lookahead == 13 { state = 33; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 248; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            322 => {
                if lookahead == 13 { state = 34; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 251; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            323 => {
                if lookahead == 13 { state = 37; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 254; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            324 => {
                if lookahead == 13 { state = 42; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 257; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            325 => {
                if lookahead == 13 { state = 43; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 451; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            326 => {
                if lookahead == 13 { state = 44; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 260; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            327 => {
                if lookahead == 13 { state = 45; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 455; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            328 => {
                if lookahead == 13 { state = 50; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 269; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            329 => {
                if lookahead == 13 { state = 55; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 464; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            330 => {
                if lookahead == 13 { state = 159; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 274; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            331 => {
                if lookahead == 13 { state = 60; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 458; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            332 => {
                if lookahead == 13 { state = 160; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 276; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            333 => {
                if lookahead == 13 { state = 63; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 454; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            334 => {
                if lookahead == 13 { state = 161; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 278; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            335 => {
                if lookahead == 13 { state = 66; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 452; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            336 => {
                if lookahead == 13 { state = 169; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 280; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            337 => {
                if lookahead == 13 { state = 162; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 282; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            338 => {
                if lookahead == 13 { state = 73; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 460; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            339 => {
                if lookahead == 13 { state = 74; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 428; lexer.advance(true); continue; }
                return result;
            }
            340 => {
                if lookahead == 13 { state = 79; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 418; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            341 => {
                if lookahead == 13 { state = 170; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 284; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            342 => {
                if lookahead == 13 { state = 165; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 286; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            343 => {
                if lookahead == 13 { state = 80; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 429; lexer.advance(true); continue; }
                return result;
            }
            344 => {
                if lookahead == 13 { state = 203; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 288; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            345 => {
                if lookahead == 13 { state = 171; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 290; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            346 => {
                if lookahead == 13 { state = 166; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 291; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            347 => {
                if lookahead == 13 { state = 204; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 292; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            348 => {
                if lookahead == 13 { state = 83; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 419; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            349 => {
                if lookahead == 13 { state = 205; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 293; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            350 => {
                if lookahead == 13 { state = 172; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 456; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            351 => {
                if lookahead == 13 { state = 84; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 431; lexer.advance(true); continue; }
                return result;
            }
            352 => {
                if lookahead == 13 { state = 177; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 459; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            353 => {
                if lookahead == 13 { state = 85; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 437; lexer.advance(true); continue; }
                return result;
            }
            354 => {
                if lookahead == 13 { state = 88; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 432; lexer.advance(true); continue; }
                return result;
            }
            355 => {
                if lookahead == 13 { state = 89; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 294; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            356 => {
                if lookahead == 13 { state = 176; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 465; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            357 => {
                if lookahead == 13 { state = 178; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 457; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            358 => {
                if lookahead == 13 { state = 208; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 461; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            359 => {
                if lookahead == 13 { state = 90; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 434; lexer.advance(true); continue; }
                return result;
            }
            360 => {
                if lookahead == 13 { state = 183; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 462; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            361 => {
                if lookahead == 13 { state = 91; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 92; lexer.advance(true); continue; }
                return result;
            }
            362 => {
                if lookahead == 13 { state = 101; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 436; lexer.advance(true); continue; }
                return result;
            }
            363 => {
                if lookahead == 13 { state = 102; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 433; lexer.advance(true); continue; }
                return result;
            }
            364 => {
                if lookahead == 13 { state = 186; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 430; lexer.advance(true); continue; }
                return result;
            }
            365 => {
                if lookahead == 13 { state = 175; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 295; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            366 => {
                if lookahead == 13 { state = 107; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 466; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            367 => {
                if lookahead == 13 { state = 209; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 453; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            368 => {
                if lookahead == 13 { state = 110; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 296; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            369 => {
                if lookahead == 13 { state = 111; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 476; lexer.advance(true); continue; }
                return result;
            }
            370 => {
                if lookahead == 13 { state = 112; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 467; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            371 => {
                if lookahead == 13 { state = 113; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 472; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            372 => {
                if lookahead == 13 { state = 116; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 445; lexer.advance(true); continue; }
                return result;
            }
            373 => {
                if lookahead == 13 { state = 125; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 473; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            374 => {
                if lookahead == 13 { state = 128; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 95; lexer.advance(true); continue; }
                return result;
            }
            375 => {
                if lookahead == 13 { state = 129; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 93; lexer.advance(true); continue; }
                return result;
            }
            376 => {
                if lookahead == 13 { state = 200; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 96; lexer.advance(true); continue; }
                return result;
            }
            377 => {
                if lookahead == 13 { state = 130; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 468; lexer.advance(true); continue; }
                return result;
            }
            378 => {
                if lookahead == 13 { state = 193; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 297; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            379 => {
                if lookahead == 13 { state = 131; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 448; lexer.advance(true); continue; }
                return result;
            }
            380 => {
                if lookahead == 13 { state = 132; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 480; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            381 => {
                if lookahead == 13 { state = 133; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 477; lexer.advance(true); continue; }
                return result;
            }
            382 => {
                if lookahead == 13 { state = 134; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 478; lexer.advance(true); continue; }
                return result;
            }
            383 => {
                if lookahead == 13 { state = 135; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 469; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            384 => {
                if lookahead == 13 { state = 136; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 481; lexer.advance(true); continue; }
                return result;
            }
            385 => {
                if lookahead == 13 { state = 137; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 94; lexer.advance(true); continue; }
                return result;
            }
            386 => {
                if lookahead == 13 { state = 138; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 97; lexer.advance(true); continue; }
                return result;
            }
            387 => {
                if lookahead == 13 { state = 143; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 449; lexer.advance(true); continue; }
                return result;
            }
            388 => {
                if lookahead == 13 { state = 148; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 479; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            389 => {
                if lookahead == 13 { state = 151; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 98; lexer.advance(true); continue; }
                return result;
            }
            390 => {
                if lookahead == 13 { state = 152; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 470; lexer.advance(true); continue; }
                return result;
            }
            391 => {
                if lookahead == 13 { state = 153; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 471; lexer.advance(true); continue; }
                return result;
            }
            392 => {
                if lookahead == 13 { state = 154; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 482; lexer.advance(true); continue; }
                return result;
            }
            393 => {
                if lookahead == 13 { state = 217; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 263; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            394 => {
                if lookahead == 13 { state = 218; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 266; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            395 => {
                if lookahead == 13 { state = 219; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 272; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            396 => {
                if lookahead == 13 { state = 226; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 463; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            397 => {
                if let Some(next) = advance_map(&[
                    (33, 664), (34, 783), (35, 811), (36, 776), (37, 641), (38, 595), (39, 485), (40, 648),
                    (41, 649), (42, 632), (43, 764), (44, 551), (45, 762), (47, 636), (58, 759), (59, 548),
                    (60, 601), (61, 557), (62, 610), (63, 755), (64, 936), (91, 668), (92, 298), (93, 669),
                    (94, 589), (95, 937), (96, 851), (101, 947), (105, 946), (123, 660), (124, 586), (125, 813),
                    (126, 766),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 397; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            398 => {
                if let Some(next) = advance_map(&[
                    (33, 664), (34, 783), (35, 811), (36, 776), (37, 643), (38, 595), (39, 485), (40, 647),
                    (42, 632), (43, 624), (45, 627), (47, 638), (48, 801), (60, 601), (61, 554), (62, 610),
                    (63, 755), (64, 936), (92, 19), (93, 669), (94, 590), (95, 938), (96, 850), (124, 586),
                    (91, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 398; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 32 || 43 < lookahead) && (lookahead < 59 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            399 => {
                if let Some(next) = advance_map(&[
                    (33, 664), (34, 783), (35, 857), (36, 776), (37, 643), (38, 595), (39, 485), (40, 647),
                    (42, 632), (43, 765), (45, 763), (47, 638), (48, 797), (60, 601), (61, 554), (62, 610),
                    (63, 755), (92, 305), (93, 669), (94, 590), (96, 851), (124, 586), (126, 766), (91, 781),
                    (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 399; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 32 || 43 < lookahead) && (lookahead < 59 || 63 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            400 => {
                if let Some(next) = advance_map(&[
                    (33, 664), (34, 783), (35, 857), (36, 776), (37, 643), (38, 594), (39, 485), (40, 647),
                    (41, 649), (42, 632), (43, 765), (45, 763), (47, 638), (48, 797), (60, 608), (61, 554),
                    (62, 613), (63, 755), (92, 309), (94, 590), (96, 851), (124, 588), (126, 766), (91, 781),
                    (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 400; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 59 || 63 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            401 => {
                if let Some(next) = advance_map(&[
                    (33, 664), (34, 783), (35, 857), (36, 776), (37, 643), (38, 594), (39, 485), (40, 647),
                    (42, 632), (43, 765), (45, 763), (47, 638), (48, 797), (58, 759), (60, 608), (61, 554),
                    (62, 613), (63, 755), (92, 311), (94, 590), (96, 851), (124, 588), (126, 766), (91, 781),
                    (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 401; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 32 || 43 < lookahead) && (lookahead < 47 || 63 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            402 => {
                if let Some(next) = advance_map(&[
                    (33, 664), (34, 783), (35, 857), (36, 776), (37, 643), (38, 594), (39, 485), (40, 647),
                    (42, 632), (43, 765), (45, 763), (47, 638), (48, 797), (60, 608), (61, 554), (62, 613),
                    (63, 755), (92, 308), (93, 669), (94, 590), (96, 851), (124, 588), (126, 766), (91, 781),
                    (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 402; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 32 || 43 < lookahead) && (lookahead < 59 || 63 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            403 => {
                if let Some(next) = advance_map(&[
                    (33, 664), (34, 783), (35, 857), (36, 776), (37, 643), (38, 594), (39, 485), (40, 647),
                    (42, 632), (43, 765), (45, 763), (47, 638), (48, 797), (60, 608), (61, 554), (62, 613),
                    (63, 755), (92, 310), (93, 782), (94, 590), (96, 851), (124, 588), (126, 766), (91, 781),
                    (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 403; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 32 || 43 < lookahead) && (lookahead < 59 || 63 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            404 => {
                if let Some(next) = advance_map(&[
                    (33, 666), (34, 783), (35, 811), (36, 776), (38, 502), (39, 485), (42, 634), (45, 630),
                    (48, 801), (60, 602), (62, 611), (63, 755), (64, 936), (92, 87), (95, 938), (96, 850),
                    (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 404; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && lookahead != 59 && lookahead != 60 && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            405 => {
                if let Some(next) = advance_map(&[
                    (33, 666), (34, 783), (35, 811), (36, 776), (38, 483), (39, 485), (40, 647), (42, 634),
                    (45, 630), (48, 801), (60, 603), (61, 943), (62, 611), (63, 755), (64, 936), (92, 57),
                    (95, 938), (96, 850), (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 405; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && (lookahead < 59 || 93 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            406 => {
                if let Some(next) = advance_map(&[
                    (33, 666), (34, 783), (35, 811), (36, 776), (38, 483), (39, 485), (42, 634), (45, 630),
                    (48, 801), (60, 603), (61, 943), (62, 611), (63, 755), (64, 936), (92, 62), (95, 938),
                    (96, 850), (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 406; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && (lookahead < 59 || 93 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            407 => {
                if let Some(next) = advance_map(&[
                    (33, 666), (34, 783), (35, 811), (36, 776), (38, 483), (39, 485), (42, 634), (45, 630),
                    (48, 801), (60, 603), (62, 611), (63, 755), (64, 936), (92, 65), (93, 669), (95, 938),
                    (96, 850), (124, 587), (91, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 407; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && lookahead != 59 && lookahead != 60 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            408 => {
                if let Some(next) = advance_map(&[
                    (33, 666), (34, 783), (35, 811), (36, 776), (38, 483), (39, 485), (42, 634), (45, 630),
                    (48, 801), (60, 603), (62, 611), (63, 755), (64, 936), (92, 76), (95, 938), (96, 850),
                    (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 408; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && lookahead != 59 && lookahead != 60 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            409 => {
                if let Some(next) = advance_map(&[
                    (33, 666), (34, 783), (35, 811), (36, 776), (38, 483), (39, 485), (42, 634), (45, 630),
                    (48, 801), (60, 604), (62, 611), (63, 755), (64, 936), (92, 228), (93, 669), (95, 938),
                    (96, 850), (124, 587), (91, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 409; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && lookahead != 59 && lookahead != 60 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            410 => {
                if let Some(next) = advance_map(&[
                    (33, 666), (34, 783), (35, 811), (36, 776), (38, 483), (39, 485), (42, 634), (45, 630),
                    (48, 801), (60, 604), (62, 611), (63, 755), (64, 936), (92, 232), (95, 938), (96, 850),
                    (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 410; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && lookahead != 59 && lookahead != 60 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            411 => {
                if let Some(next) = advance_map(&[
                    (33, 666), (34, 783), (35, 811), (36, 776), (39, 485), (41, 649), (42, 634), (45, 630),
                    (48, 801), (60, 489), (62, 490), (63, 755), (64, 936), (92, 124), (95, 938), (96, 850),
                    (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 411; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && lookahead != 59 && lookahead != 60 && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            412 => {
                if let Some(next) = advance_map(&[
                    (33, 666), (34, 783), (35, 857), (36, 776), (38, 502), (39, 485), (40, 648), (41, 649),
                    (45, 941), (48, 797), (59, 484), (60, 602), (62, 611), (91, 668), (92, 301), (96, 850),
                    (123, 660), (124, 584), (93, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 412; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            413 => {
                if let Some(next) = advance_map(&[
                    (33, 666), (34, 783), (35, 857), (36, 776), (38, 502), (39, 485), (40, 648), (41, 493),
                    (43, 765), (45, 763), (48, 797), (60, 602), (62, 611), (91, 668), (92, 302), (96, 850),
                    (123, 660), (124, 585), (126, 766), (93, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 413; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 59 && lookahead != 60 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            414 => {
                if let Some(next) = advance_map(&[
                    (33, 666), (34, 783), (35, 857), (36, 776), (38, 502), (39, 485), (40, 648), (43, 765),
                    (45, 763), (48, 797), (60, 602), (62, 611), (91, 668), (92, 303), (93, 669), (96, 850),
                    (123, 660), (125, 781), (126, 766),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 414; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && lookahead != 59 && lookahead != 60 && (lookahead < 123 || 126 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            415 => {
                if let Some(next) = advance_map(&[
                    (33, 666), (34, 783), (35, 857), (36, 776), (38, 502), (39, 485), (40, 648), (45, 941),
                    (48, 797), (59, 484), (60, 602), (62, 611), (91, 668), (92, 306), (96, 850), (101, 947),
                    (123, 660), (93, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 415; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && (lookahead < 123 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            416 => {
                if let Some(next) = advance_map(&[
                    (33, 666), (34, 783), (35, 857), (36, 776), (38, 502), (39, 485), (40, 648), (45, 941),
                    (48, 797), (60, 602), (62, 611), (91, 668), (92, 307), (93, 781), (96, 850), (123, 660),
                    (125, 661),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 416; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && lookahead != 59 && lookahead != 60 && (lookahead < 123 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            417 => {
                if let Some(next) = advance_map(&[
                    (33, 666), (34, 783), (35, 857), (36, 776), (38, 502), (39, 485), (40, 647), (43, 765),
                    (45, 763), (48, 797), (60, 602), (62, 611), (91, 668), (92, 316), (96, 850), (126, 766),
                    (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 417; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && lookahead != 59 && lookahead != 60 && (lookahead < 123 || 126 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            418 => {
                if let Some(next) = advance_map(&[
                    (33, 666), (34, 783), (35, 857), (36, 776), (39, 485), (40, 648), (43, 765), (45, 763),
                    (48, 797), (60, 489), (62, 490), (92, 340), (96, 850), (123, 660), (126, 766), (91, 781),
                    (93, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 418; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && lookahead != 59 && lookahead != 60 && (lookahead < 123 || 126 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            419 => {
                if let Some(next) = advance_map(&[
                    (33, 666), (34, 783), (35, 857), (36, 776), (39, 485), (40, 647), (43, 765), (45, 763),
                    (48, 797), (60, 489), (62, 490), (92, 348), (96, 850), (126, 766), (91, 781), (93, 781),
                    (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 419; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && lookahead != 59 && lookahead != 60 && (lookahead < 123 || 126 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            420 => {
                if let Some(next) = advance_map(&[
                    (33, 942), (34, 783), (35, 857), (36, 776), (37, 643), (38, 595), (39, 485), (40, 647),
                    (42, 632), (43, 624), (45, 627), (47, 638), (48, 797), (60, 601), (61, 554), (62, 610),
                    (63, 755), (92, 314), (93, 669), (94, 590), (96, 851), (124, 586), (91, 781), (123, 781),
                    (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 420; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 32 || 43 < lookahead) && (lookahead < 59 || 63 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            421 => {
                if let Some(next) = advance_map(&[
                    (33, 942), (34, 783), (35, 857), (36, 776), (37, 643), (38, 595), (39, 485), (40, 647),
                    (42, 632), (43, 624), (45, 627), (47, 638), (48, 797), (60, 601), (61, 554), (62, 610),
                    (63, 755), (92, 315), (93, 669), (94, 590), (96, 850), (124, 586), (91, 781), (123, 781),
                    (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 421; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 32 || 43 < lookahead) && (lookahead < 59 || 63 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            422 => {
                if let Some(next) = advance_map(&[
                    (33, 665), (34, 783), (35, 811), (36, 775), (37, 644), (38, 594), (41, 649), (42, 633),
                    (43, 625), (45, 628), (47, 639), (60, 609), (61, 556), (62, 614), (63, 754), (64, 935),
                    (92, 78), (94, 591), (95, 939), (124, 588),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 422; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            423 => {
                if let Some(next) = advance_map(&[
                    (33, 665), (34, 783), (35, 811), (36, 775), (37, 644), (38, 594), (42, 633), (43, 625),
                    (45, 628), (47, 639), (58, 756), (60, 609), (61, 556), (62, 614), (63, 754), (64, 935),
                    (92, 72), (94, 591), (95, 939), (124, 588),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 423; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            424 => {
                if let Some(next) = advance_map(&[
                    (33, 665), (34, 783), (35, 811), (36, 775), (37, 644), (38, 594), (42, 633), (43, 625),
                    (45, 628), (47, 639), (60, 609), (61, 556), (62, 614), (63, 754), (64, 935), (92, 70),
                    (93, 669), (94, 591), (95, 939), (124, 588),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 424; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            425 => {
                if let Some(next) = advance_map(&[
                    (33, 665), (34, 783), (35, 811), (36, 775), (37, 644), (38, 594), (42, 633), (43, 625),
                    (45, 628), (47, 639), (60, 609), (61, 556), (62, 614), (63, 754), (64, 935), (92, 182),
                    (93, 503), (94, 591), (95, 939), (124, 588),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 425; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            426 => {
                if let Some(next) = advance_map(&[
                    (33, 665), (34, 783), (35, 811), (36, 775), (37, 644), (38, 594), (42, 633), (43, 625),
                    (45, 628), (47, 639), (60, 609), (61, 556), (62, 614), (63, 754), (64, 935), (92, 82),
                    (94, 591), (95, 939), (124, 588),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 426; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            427 => {
                if let Some(next) = advance_map(&[
                    (33, 500), (34, 783), (35, 857), (36, 780), (37, 644), (38, 594), (41, 649), (42, 633),
                    (43, 625), (44, 550), (45, 628), (46, 497), (47, 639), (58, 756), (59, 484), (60, 609),
                    (61, 556), (62, 614), (63, 754),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 92 { state = 339; lexer.advance(true); continue; }
                if lookahead == 93 { state = 782; lexer.advance(false); continue; }
                if lookahead == 94 { state = 591; lexer.advance(false); continue; }
                if lookahead == 96 { state = 850; lexer.advance(false); continue; }
                if lookahead == 101 { state = 508; lexer.advance(false); continue; }
                if lookahead == 105 { state = 507; lexer.advance(false); continue; }
                if lookahead == 124 { state = 588; lexer.advance(false); continue; }
                if lookahead == 91 || 123 <= lookahead && lookahead <= 125 { state = 781; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 428; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 770; lexer.advance(false); continue; }
                return result;
            }
            428 => {
                if let Some(next) = advance_map(&[
                    (33, 500), (34, 783), (35, 857), (36, 780), (37, 644), (38, 594), (41, 649), (42, 633),
                    (43, 625), (44, 550), (45, 628), (47, 639), (58, 756), (59, 484), (60, 609), (61, 556),
                    (62, 614), (63, 754),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 92 { state = 339; lexer.advance(true); continue; }
                if lookahead == 93 { state = 782; lexer.advance(false); continue; }
                if lookahead == 94 { state = 591; lexer.advance(false); continue; }
                if lookahead == 96 { state = 850; lexer.advance(false); continue; }
                if lookahead == 101 { state = 508; lexer.advance(false); continue; }
                if lookahead == 105 { state = 507; lexer.advance(false); continue; }
                if lookahead == 124 { state = 588; lexer.advance(false); continue; }
                if lookahead == 91 || 123 <= lookahead && lookahead <= 125 { state = 781; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 428; lexer.advance(true); continue; }
                return result;
            }
            429 => {
                if let Some(next) = advance_map(&[
                    (33, 500), (34, 783), (35, 857), (36, 492), (37, 644), (38, 594), (40, 488), (41, 493),
                    (42, 633), (43, 625), (44, 550), (45, 628), (47, 639), (58, 756), (60, 609), (61, 556),
                    (62, 614), (63, 754),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 92 { state = 343; lexer.advance(true); continue; }
                if lookahead == 93 { state = 669; lexer.advance(false); continue; }
                if lookahead == 94 { state = 591; lexer.advance(false); continue; }
                if lookahead == 96 { state = 850; lexer.advance(false); continue; }
                if lookahead == 124 { state = 588; lexer.advance(false); continue; }
                if lookahead == 91 || 123 <= lookahead && lookahead <= 125 { state = 781; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 429; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            430 => {
                if let Some(next) = advance_map(&[
                    (33, 500), (35, 857), (37, 644), (38, 594), (41, 649), (42, 633), (43, 625), (44, 550),
                    (45, 629), (47, 639), (60, 609), (61, 555), (62, 614),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 92 { state = 364; lexer.advance(true); continue; }
                if lookahead == 94 { state = 591; lexer.advance(false); continue; }
                if lookahead == 124 { state = 588; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 430; lexer.advance(true); continue; }
                return result;
            }
            431 => {
                if let Some(next) = advance_map(&[
                    (33, 500), (35, 857), (37, 644), (38, 594), (41, 649), (42, 633), (43, 625), (45, 628),
                    (47, 639), (58, 756), (60, 609), (61, 556), (62, 614), (63, 754),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 92 { state = 351; lexer.advance(true); continue; }
                if lookahead == 93 { state = 782; lexer.advance(false); continue; }
                if lookahead == 94 { state = 591; lexer.advance(false); continue; }
                if lookahead == 96 { state = 504; lexer.advance(false); continue; }
                if lookahead == 124 { state = 588; lexer.advance(false); continue; }
                if lookahead == 91 || 123 <= lookahead && lookahead <= 125 { state = 781; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 431; lexer.advance(true); continue; }
                return result;
            }
            432 => {
                if let Some(next) = advance_map(&[
                    (33, 500), (35, 857), (37, 644), (38, 594), (41, 649), (42, 633), (43, 625), (45, 628),
                    (47, 639), (58, 756), (60, 609), (61, 556), (62, 614), (63, 754),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 92 { state = 354; lexer.advance(true); continue; }
                if lookahead == 94 { state = 591; lexer.advance(false); continue; }
                if lookahead == 96 { state = 504; lexer.advance(false); continue; }
                if lookahead == 124 { state = 588; lexer.advance(false); continue; }
                if 91 <= lookahead && lookahead <= 93 || 123 <= lookahead && lookahead <= 125 { state = 781; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 432; lexer.advance(true); continue; }
                return result;
            }
            433 => {
                if let Some(next) = advance_map(&[
                    (33, 500), (35, 857), (37, 644), (38, 594), (41, 649), (42, 633), (43, 625), (45, 628),
                    (47, 639), (60, 609), (61, 556), (62, 614), (63, 754), (91, 667),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 92 { state = 363; lexer.advance(true); continue; }
                if lookahead == 94 { state = 591; lexer.advance(false); continue; }
                if lookahead == 124 { state = 588; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 433; lexer.advance(true); continue; }
                return result;
            }
            434 => {
                if let Some(next) = advance_map(&[
                    (33, 500), (35, 857), (37, 644), (38, 594), (41, 493), (42, 633), (43, 625), (44, 550),
                    (45, 628), (47, 639), (58, 756), (60, 609), (61, 556), (62, 614), (63, 754), (91, 667),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 92 { state = 359; lexer.advance(true); continue; }
                if lookahead == 93 { state = 669; lexer.advance(false); continue; }
                if lookahead == 94 { state = 591; lexer.advance(false); continue; }
                if lookahead == 96 { state = 504; lexer.advance(false); continue; }
                if lookahead == 124 { state = 588; lexer.advance(false); continue; }
                if lookahead == 125 { state = 813; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 434; lexer.advance(true); continue; }
                return result;
            }
            435 => {
                if let Some(next) = advance_map(&[
                    (33, 500), (35, 857), (37, 644), (38, 594), (41, 493), (42, 633), (43, 625), (44, 550),
                    (45, 629), (47, 639), (60, 609), (61, 555), (62, 614),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 92 { state = 362; lexer.advance(true); continue; }
                if lookahead == 94 { state = 591; lexer.advance(false); continue; }
                if lookahead == 124 { state = 588; lexer.advance(false); continue; }
                if lookahead == 125 { state = 773; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 436; lexer.advance(true); continue; }
                return result;
            }
            436 => {
                if let Some(next) = advance_map(&[
                    (33, 500), (35, 857), (37, 644), (38, 594), (41, 493), (42, 633), (43, 625), (44, 550),
                    (45, 629), (47, 639), (60, 609), (61, 555), (62, 614),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 92 { state = 362; lexer.advance(true); continue; }
                if lookahead == 94 { state = 591; lexer.advance(false); continue; }
                if lookahead == 124 { state = 588; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 436; lexer.advance(true); continue; }
                return result;
            }
            437 => {
                if let Some(next) = advance_map(&[
                    (33, 500), (35, 857), (37, 644), (38, 594), (42, 633), (43, 625), (45, 628), (47, 639),
                    (60, 609), (61, 556), (62, 614), (63, 754),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 92 { state = 353; lexer.advance(true); continue; }
                if lookahead == 93 { state = 669; lexer.advance(false); continue; }
                if lookahead == 94 { state = 591; lexer.advance(false); continue; }
                if lookahead == 96 { state = 504; lexer.advance(false); continue; }
                if lookahead == 124 { state = 588; lexer.advance(false); continue; }
                if lookahead == 91 || 123 <= lookahead && lookahead <= 125 { state = 781; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 437; lexer.advance(true); continue; }
                return result;
            }
            438 => {
                if let Some(next) = advance_map(&[
                    (33, 663), (34, 783), (35, 811), (36, 775), (38, 483), (42, 631), (45, 626), (60, 606),
                    (62, 612), (63, 754), (64, 935), (92, 118), (93, 669), (95, 939), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 438; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            439 => {
                if let Some(next) = advance_map(&[
                    (33, 663), (34, 783), (35, 811), (36, 775), (38, 483), (42, 631), (45, 626), (60, 606),
                    (62, 612), (63, 754), (64, 935), (92, 122), (95, 939), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 439; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            440 => {
                if let Some(next) = advance_map(&[
                    (33, 663), (34, 783), (35, 811), (36, 775), (38, 483), (42, 631), (45, 626), (60, 607),
                    (62, 612), (63, 754), (64, 935), (92, 195), (93, 669), (95, 939), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 440; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            441 => {
                if let Some(next) = advance_map(&[
                    (33, 663), (34, 783), (35, 811), (36, 775), (38, 483), (42, 631), (45, 626), (60, 607),
                    (62, 612), (63, 754), (64, 935), (92, 199), (95, 939), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 441; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            442 => {
                if let Some(next) = advance_map(&[
                    (33, 663), (34, 783), (35, 811), (36, 775), (41, 649), (42, 631), (45, 626), (63, 754),
                    (64, 935), (92, 142), (95, 939), (124, 584),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 442; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            443 => {
                if let Some(next) = advance_map(&[
                    (33, 663), (34, 783), (35, 811), (36, 775), (42, 631), (45, 626), (63, 754), (64, 935),
                    (92, 144), (95, 939),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 10 || lookahead == 13 { state = 443; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 788; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 95 || 122 < lookahead) { state = 792; lexer.advance(false); continue; }
                return result;
            }
            444 => {
                if let Some(next) = advance_map(&[
                    (33, 663), (34, 783), (35, 811), (36, 775), (42, 631), (45, 626), (63, 754), (64, 935),
                    (92, 146), (95, 939),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 444; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            445 => {
                if let Some(next) = advance_map(&[
                    (33, 663), (34, 783), (35, 857), (36, 780), (38, 483), (39, 485), (40, 647), (43, 765),
                    (45, 763), (48, 803), (60, 606), (62, 612),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 92 { state = 372; lexer.advance(true); continue; }
                if lookahead == 93 { state = 669; lexer.advance(false); continue; }
                if lookahead == 96 { state = 850; lexer.advance(false); continue; }
                if lookahead == 124 { state = 587; lexer.advance(false); continue; }
                if lookahead == 126 { state = 766; lexer.advance(false); continue; }
                if lookahead == 91 || 123 <= lookahead && lookahead <= 125 { state = 781; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 445; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 805; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            446 => {
                if let Some(next) = advance_map(&[
                    (33, 663), (35, 811), (36, 775), (42, 631), (45, 626), (63, 754), (64, 935), (92, 147),
                    (95, 939),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 10 || lookahead == 13 { state = 446; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 789; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 32 || 36 < lookahead) && (lookahead < 95 || 122 < lookahead) { state = 792; lexer.advance(false); continue; }
                return result;
            }
            447 => {
                if let Some(next) = advance_map(&[
                    (33, 663), (35, 811), (36, 775), (42, 631), (45, 626), (63, 754), (64, 935), (92, 150),
                    (95, 939),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 447; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            448 => {
                if lookahead == 33 { state = 663; lexer.advance(false); continue; }
                if lookahead == 35 { state = 811; lexer.advance(false); continue; }
                if lookahead == 36 { state = 779; lexer.advance(false); continue; }
                if lookahead == 42 { state = 631; lexer.advance(false); continue; }
                if lookahead == 45 { state = 626; lexer.advance(false); continue; }
                if lookahead == 63 { state = 754; lexer.advance(false); continue; }
                if lookahead == 64 { state = 935; lexer.advance(false); continue; }
                if lookahead == 92 { state = 379; lexer.advance(true); continue; }
                if lookahead == 95 { state = 939; lexer.advance(false); continue; }
                if lookahead == 96 { state = 850; lexer.advance(false); continue; }
                if lookahead == 125 { state = 813; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 448; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            449 => {
                if lookahead == 33 { state = 663; lexer.advance(false); continue; }
                if lookahead == 35 { state = 811; lexer.advance(false); continue; }
                if lookahead == 36 { state = 779; lexer.advance(false); continue; }
                if lookahead == 42 { state = 631; lexer.advance(false); continue; }
                if lookahead == 45 { state = 626; lexer.advance(false); continue; }
                if lookahead == 63 { state = 754; lexer.advance(false); continue; }
                if lookahead == 64 { state = 935; lexer.advance(false); continue; }
                if lookahead == 92 { state = 387; lexer.advance(true); continue; }
                if lookahead == 95 { state = 939; lexer.advance(false); continue; }
                if lookahead == 96 { state = 850; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 449; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            450 => {
                if let Some(next) = advance_map(&[
                    (33, 814), (35, 820), (36, 779), (42, 631), (45, 626), (61, 821), (63, 754), (64, 935),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 92 { state = 379; lexer.advance(true); continue; }
                if lookahead == 95 { state = 939; lexer.advance(false); continue; }
                if lookahead == 96 { state = 850; lexer.advance(false); continue; }
                if lookahead == 125 { state = 813; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 448; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            451 => {
                if let Some(next) = advance_map(&[
                    (34, 783), (35, 857), (36, 776), (38, 502), (39, 485), (40, 647), (45, 941), (48, 797),
                    (60, 602), (62, 611), (91, 668), (92, 325), (96, 850), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 451; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && lookahead != 59 && lookahead != 60 && (lookahead < 123 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            452 => {
                if let Some(next) = advance_map(&[
                    (34, 783), (35, 857), (36, 776), (38, 502), (39, 485), (41, 649), (45, 941), (48, 797),
                    (60, 602), (62, 611), (92, 335), (96, 850), (124, 584), (91, 781), (93, 781), (123, 781),
                    (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 452; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && lookahead != 59 && lookahead != 60 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            453 => {
                if let Some(next) = advance_map(&[
                    (34, 783), (35, 857), (36, 776), (38, 502), (39, 485), (41, 649), (45, 941), (48, 797),
                    (60, 602), (62, 611), (92, 367), (96, 851), (124, 584), (91, 781), (93, 781), (123, 781),
                    (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 453; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && lookahead != 59 && lookahead != 60 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            454 => {
                if let Some(next) = advance_map(&[
                    (34, 783), (35, 857), (36, 776), (38, 483), (39, 485), (40, 647), (41, 649), (45, 941),
                    (48, 797), (60, 603), (62, 611), (92, 333), (96, 850), (124, 587), (91, 781), (93, 781),
                    (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 454; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 59 && lookahead != 60 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            455 => {
                if let Some(next) = advance_map(&[
                    (34, 783), (35, 857), (36, 776), (38, 483), (39, 485), (40, 647), (45, 941), (48, 797),
                    (60, 603), (61, 943), (62, 611), (92, 327), (96, 850), (124, 587), (91, 781), (93, 781),
                    (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 455; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && (lookahead < 59 || 62 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            456 => {
                if let Some(next) = advance_map(&[
                    (34, 783), (35, 857), (36, 776), (38, 483), (39, 485), (40, 647), (45, 941), (48, 797),
                    (60, 603), (61, 943), (62, 611), (92, 350), (96, 851), (124, 587), (91, 781), (93, 781),
                    (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 456; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && (lookahead < 59 || 62 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            457 => {
                if let Some(next) = advance_map(&[
                    (34, 783), (35, 857), (36, 776), (38, 483), (39, 485), (41, 649), (45, 941), (48, 797),
                    (60, 603), (62, 611), (92, 357), (96, 851), (124, 587), (91, 781), (93, 781), (123, 781),
                    (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 457; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && lookahead != 59 && lookahead != 60 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            458 => {
                if let Some(next) = advance_map(&[
                    (34, 783), (35, 857), (36, 776), (38, 483), (39, 485), (45, 941), (48, 797), (60, 603),
                    (62, 611), (92, 331), (93, 669), (96, 850), (124, 587), (91, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 458; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && lookahead != 59 && lookahead != 60 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            459 => {
                if let Some(next) = advance_map(&[
                    (34, 783), (35, 857), (36, 776), (38, 483), (39, 485), (45, 941), (48, 797), (60, 603),
                    (62, 611), (92, 352), (93, 669), (96, 851), (124, 587), (91, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 459; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && lookahead != 59 && lookahead != 60 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            460 => {
                if let Some(next) = advance_map(&[
                    (34, 783), (35, 857), (36, 776), (38, 483), (39, 485), (45, 941), (48, 797), (60, 604),
                    (62, 611), (92, 338), (96, 850), (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 460; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && lookahead != 59 && lookahead != 60 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            461 => {
                if let Some(next) = advance_map(&[
                    (34, 783), (35, 857), (36, 776), (38, 483), (39, 485), (45, 941), (48, 797), (60, 604),
                    (62, 611), (92, 358), (93, 669), (96, 851), (124, 587), (91, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 461; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && lookahead != 59 && lookahead != 60 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            462 => {
                if let Some(next) = advance_map(&[
                    (34, 783), (35, 857), (36, 776), (38, 483), (39, 485), (45, 941), (48, 797), (60, 604),
                    (62, 611), (92, 360), (96, 851), (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 462; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && lookahead != 59 && lookahead != 60 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            463 => {
                if let Some(next) = advance_map(&[
                    (34, 783), (35, 857), (36, 776), (38, 483), (39, 485), (45, 941), (48, 797), (60, 604),
                    (62, 611), (92, 396), (93, 669), (96, 850), (124, 587), (91, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 463; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && lookahead != 59 && lookahead != 60 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            464 => {
                if let Some(next) = advance_map(&[
                    (34, 783), (35, 857), (36, 776), (38, 483), (39, 485), (45, 941), (48, 798), (60, 604),
                    (62, 611), (92, 329), (96, 850), (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 464; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 800; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 { state = 863; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && lookahead != 59 && lookahead != 60 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            465 => {
                if let Some(next) = advance_map(&[
                    (34, 783), (35, 857), (36, 776), (38, 483), (39, 485), (45, 941), (48, 798), (60, 604),
                    (62, 611), (92, 356), (96, 851), (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 465; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 800; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 { state = 863; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && lookahead != 59 && lookahead != 60 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            466 => {
                if let Some(next) = advance_map(&[
                    (34, 783), (35, 857), (36, 776), (39, 485), (40, 647), (45, 941), (48, 797), (60, 489),
                    (62, 490), (92, 366), (96, 850), (101, 947), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 466; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && lookahead != 59 && lookahead != 60 && (lookahead < 123 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            467 => {
                if let Some(next) = advance_map(&[
                    (34, 783), (35, 857), (36, 776), (39, 485), (40, 647), (45, 941), (48, 797), (60, 489),
                    (62, 490), (92, 370), (96, 850), (125, 813), (91, 781), (93, 781), (123, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 467; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && lookahead != 59 && lookahead != 60 && (lookahead < 123 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            468 => {
                if let Some(next) = advance_map(&[
                    (34, 783), (35, 857), (36, 780), (38, 483), (40, 647), (41, 493), (43, 495), (45, 496),
                    (48, 804), (60, 607), (61, 553), (62, 612),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 92 { state = 377; lexer.advance(true); continue; }
                if lookahead == 93 { state = 669; lexer.advance(false); continue; }
                if lookahead == 96 { state = 850; lexer.advance(false); continue; }
                if lookahead == 124 { state = 587; lexer.advance(false); continue; }
                if lookahead == 91 || 123 <= lookahead && lookahead <= 125 { state = 781; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 468; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 806; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 { state = 650; lexer.advance(false); continue; }
                return result;
            }
            469 => {
                if let Some(next) = advance_map(&[
                    (34, 783), (35, 857), (36, 777), (39, 485), (40, 647), (41, 649), (60, 489), (62, 490),
                    (92, 383), (96, 850), (124, 584), (125, 813),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 469; lexer.advance(true); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && lookahead != 59 && lookahead != 60 && (lookahead < 91 || 93 < lookahead) && (lookahead < 123 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            470 => {
                if let Some(next) = advance_map(&[
                    (34, 783), (35, 857), (36, 491), (37, 640), (42, 631), (43, 623), (45, 626), (47, 635),
                    (58, 756),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 92 { state = 390; lexer.advance(true); continue; }
                if lookahead == 96 { state = 850; lexer.advance(false); continue; }
                if lookahead == 125 { state = 813; lexer.advance(false); continue; }
                if 91 <= lookahead && lookahead <= 93 || lookahead == 123 { state = 781; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 470; lexer.advance(true); continue; }
                return result;
            }
            471 => {
                if lookahead == 34 { state = 783; lexer.advance(false); continue; }
                if lookahead == 35 { state = 857; lexer.advance(false); continue; }
                if lookahead == 39 { state = 485; lexer.advance(false); continue; }
                if lookahead == 41 { state = 649; lexer.advance(false); continue; }
                if lookahead == 92 { state = 391; lexer.advance(true); continue; }
                if lookahead == 125 { state = 813; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 837; lexer.advance(false); continue; }
                return result;
            }
            472 => {
                if let Some(next) = advance_map(&[
                    (34, 783), (35, 858), (36, 776), (39, 485), (40, 647), (45, 941), (48, 797), (60, 489),
                    (62, 490), (92, 371), (96, 850), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 472; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && lookahead != 59 && lookahead != 60 && (lookahead < 123 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            473 => {
                if let Some(next) = advance_map(&[
                    (34, 783), (35, 858), (36, 776), (39, 485), (45, 941), (48, 797), (60, 489), (62, 490),
                    (92, 373), (93, 669), (96, 850), (91, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 473; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) && lookahead != 59 && lookahead != 60 && (lookahead < 123 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            474 => {
                if lookahead == 34 { state = 783; lexer.advance(false); continue; }
                if lookahead == 35 { state = 791; lexer.advance(false); continue; }
                if lookahead == 36 { state = 778; lexer.advance(false); continue; }
                if lookahead == 92 { state = 299; lexer.advance(false); continue; }
                if lookahead == 96 { state = 850; lexer.advance(false); continue; }
                if lookahead == 10 || lookahead == 13 { state = 474; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 790; lexer.advance(false); continue; }
                if lookahead != 0 { state = 792; lexer.advance(false); continue; }
                return result;
            }
            475 => {
                if let Some(next) = advance_map(&[
                    (35, 811), (37, 642), (42, 818), (43, 828), (44, 552), (45, 825), (47, 637), (58, 758),
                    (61, 821), (63, 831), (64, 816), (91, 667),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 92 { state = 369; lexer.advance(true); continue; }
                if lookahead == 94 { state = 592; lexer.advance(false); continue; }
                if lookahead == 125 { state = 813; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 476; lexer.advance(true); continue; }
                return result;
            }
            476 => {
                if lookahead == 35 { state = 811; lexer.advance(false); continue; }
                if lookahead == 37 { state = 642; lexer.advance(false); continue; }
                if lookahead == 44 { state = 552; lexer.advance(false); continue; }
                if lookahead == 47 { state = 637; lexer.advance(false); continue; }
                if lookahead == 58 { state = 756; lexer.advance(false); continue; }
                if lookahead == 91 { state = 667; lexer.advance(false); continue; }
                if lookahead == 92 { state = 369; lexer.advance(true); continue; }
                if lookahead == 94 { state = 592; lexer.advance(false); continue; }
                if lookahead == 125 { state = 813; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 476; lexer.advance(true); continue; }
                return result;
            }
            477 => {
                if lookahead == 35 { state = 857; lexer.advance(false); continue; }
                if lookahead == 36 { state = 509; lexer.advance(false); continue; }
                if lookahead == 38 { state = 483; lexer.advance(false); continue; }
                if lookahead == 45 { state = 498; lexer.advance(false); continue; }
                if lookahead == 48 { state = 803; lexer.advance(false); continue; }
                if lookahead == 60 { state = 607; lexer.advance(false); continue; }
                if lookahead == 62 { state = 612; lexer.advance(false); continue; }
                if lookahead == 92 { state = 381; lexer.advance(true); continue; }
                if lookahead == 96 { state = 504; lexer.advance(false); continue; }
                if lookahead == 124 { state = 587; lexer.advance(false); continue; }
                if 91 <= lookahead && lookahead <= 93 || 123 <= lookahead && lookahead <= 125 { state = 781; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 477; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 805; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            478 => {
                if let Some(next) = advance_map(&[
                    (35, 857), (37, 640), (38, 483), (42, 631), (43, 623), (45, 626), (47, 635), (60, 606),
                    (62, 612),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 92 { state = 382; lexer.advance(true); continue; }
                if lookahead == 96 { state = 504; lexer.advance(false); continue; }
                if lookahead == 124 { state = 587; lexer.advance(false); continue; }
                if 91 <= lookahead && lookahead <= 93 || 123 <= lookahead && lookahead <= 125 { state = 781; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 478; lexer.advance(true); continue; }
                return result;
            }
            479 => {
                if let Some(next) = advance_map(&[
                    (35, 857), (38, 502), (40, 648), (59, 547), (60, 605), (62, 612), (91, 668), (92, 388),
                    (123, 660),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 479; lexer.advance(true); continue; }
                if lookahead != 0 && (lookahead < 34 || 36 < lookahead) && (lookahead < 38 || 41 < lookahead) && (lookahead < 91 || 93 < lookahead) && lookahead != 96 && (lookahead < 123 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            480 => {
                if let Some(next) = advance_map(&[
                    (35, 857), (38, 483), (60, 606), (62, 612), (92, 380), (93, 669), (96, 504), (124, 587),
                    (91, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 480; lexer.advance(true); continue; }
                if lookahead != 0 && (lookahead < 34 || 36 < lookahead) && (lookahead < 38 || 41 < lookahead) && lookahead != 59 && lookahead != 60 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            481 => {
                if lookahead == 35 { state = 857; lexer.advance(false); continue; }
                if lookahead == 38 { state = 483; lexer.advance(false); continue; }
                if lookahead == 60 { state = 607; lexer.advance(false); continue; }
                if lookahead == 62 { state = 612; lexer.advance(false); continue; }
                if lookahead == 92 { state = 384; lexer.advance(true); continue; }
                if lookahead == 93 { state = 669; lexer.advance(false); continue; }
                if lookahead == 96 { state = 504; lexer.advance(false); continue; }
                if lookahead == 124 { state = 587; lexer.advance(false); continue; }
                if lookahead == 91 || 123 <= lookahead && lookahead <= 125 { state = 781; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 481; lexer.advance(true); continue; }
                return result;
            }
            482 => {
                if lookahead == 35 { state = 857; lexer.advance(false); continue; }
                if lookahead == 43 { state = 501; lexer.advance(false); continue; }
                if lookahead == 47 { state = 635; lexer.advance(false); continue; }
                if lookahead == 61 { state = 553; lexer.advance(false); continue; }
                if lookahead == 91 { state = 667; lexer.advance(false); continue; }
                if lookahead == 92 { state = 392; lexer.advance(true); continue; }
                if lookahead == 96 { state = 504; lexer.advance(false); continue; }
                if lookahead == 125 { state = 813; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 482; lexer.advance(true); continue; }
                return result;
            }
            483 => {
                if lookahead == 38 { state = 582; lexer.advance(false); continue; }
                if lookahead == 62 { state = 674; lexer.advance(false); continue; }
                return result;
            }
            484 => {
                if lookahead == 38 { state = 658; lexer.advance(false); continue; }
                if lookahead == 59 { state = 657; lexer.advance(false); continue; }
                return result;
            }
            485 => {
                if lookahead == 39 { state = 794; lexer.advance(false); continue; }
                if lookahead != 0 { state = 485; lexer.advance(false); continue; }
                return result;
            }
            486 => {
                if lookahead == 39 { state = 795; lexer.advance(false); continue; }
                if lookahead == 92 { state = 487; lexer.advance(false); continue; }
                if lookahead != 0 { state = 486; lexer.advance(false); continue; }
                return result;
            }
            487 => {
                if lookahead == 39 { state = 796; lexer.advance(false); continue; }
                if lookahead == 92 { state = 487; lexer.advance(false); continue; }
                if lookahead != 0 { state = 486; lexer.advance(false); continue; }
                return result;
            }
            488 => {
                if lookahead == 40 { state = 545; lexer.advance(false); continue; }
                return result;
            }
            489 => {
                if lookahead == 40 { state = 853; lexer.advance(false); continue; }
                return result;
            }
            490 => {
                if lookahead == 40 { state = 854; lexer.advance(false); continue; }
                return result;
            }
            491 => {
                if lookahead == 40 { state = 848; lexer.advance(false); continue; }
                if lookahead == 96 { state = 852; lexer.advance(false); continue; }
                return result;
            }
            492 => {
                if lookahead == 40 { state = 848; lexer.advance(false); continue; }
                if lookahead == 96 { state = 852; lexer.advance(false); continue; }
                if lookahead == 123 { state = 812; lexer.advance(false); continue; }
                return result;
            }
            493 => {
                if lookahead == 41 { state = 546; lexer.advance(false); continue; }
                return result;
            }
            494 => {
                if lookahead == 43 { state = 558; lexer.advance(false); continue; }
                return result;
            }
            495 => {
                if lookahead == 43 { state = 558; lexer.advance(false); continue; }
                if lookahead == 61 { state = 562; lexer.advance(false); continue; }
                return result;
            }
            496 => {
                if lookahead == 45 { state = 560; lexer.advance(false); continue; }
                if lookahead == 48 { state = 804; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 806; lexer.advance(false); continue; }
                return result;
            }
            497 => {
                if lookahead == 46 { state = 771; lexer.advance(false); continue; }
                return result;
            }
            498 => {
                if lookahead == 48 { state = 804; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 806; lexer.advance(false); continue; }
                return result;
            }
            499 => {
                if lookahead == 60 { state = 753; lexer.advance(false); continue; }
                return result;
            }
            500 => {
                if lookahead == 61 { state = 599; lexer.advance(false); continue; }
                return result;
            }
            501 => {
                if lookahead == 61 { state = 562; lexer.advance(false); continue; }
                return result;
            }
            502 => {
                if lookahead == 62 { state = 674; lexer.advance(false); continue; }
                return result;
            }
            503 => {
                if lookahead == 93 { state = 671; lexer.advance(false); continue; }
                return result;
            }
            504 => {
                if lookahead == 96 { state = 774; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 504; lexer.advance(false); continue; }
                return result;
            }
            505 => {
                if lookahead == 97 { state = 506; lexer.advance(false); continue; }
                return result;
            }
            506 => {
                if lookahead == 99 { state = 651; lexer.advance(false); continue; }
                return result;
            }
            507 => {
                if lookahead == 110 { state = 542; lexer.advance(false); continue; }
                return result;
            }
            508 => {
                if lookahead == 115 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            509 => {
                if lookahead == 123 { state = 812; lexer.advance(false); continue; }
                return result;
            }
            510 => {
                if lookahead == 124 { state = 580; lexer.advance(false); continue; }
                return result;
            }
            511 => {
                if 48 <= lookahead && lookahead <= 57 { state = 806; lexer.advance(false); continue; }
                return result;
            }
            512 => {
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            513 => {
                if eof { state = 541; lexer.advance(false); continue; }
                if lookahead == 10 { state = 684; lexer.advance(false); continue; }
                if lookahead == 35 { state = 857; lexer.advance(false); continue; }
                if lookahead == 38 { state = 596; lexer.advance(false); continue; }
                if lookahead == 41 { state = 649; lexer.advance(false); continue; }
                if lookahead == 59 { state = 549; lexer.advance(false); continue; }
                if lookahead == 60 { state = 606; lexer.advance(false); continue; }
                if lookahead == 62 { state = 612; lexer.advance(false); continue; }
                if lookahead == 92 { state = 535; lexer.advance(true); continue; }
                if lookahead == 96 { state = 851; lexer.advance(false); continue; }
                if lookahead == 124 { state = 587; lexer.advance(false); continue; }
                if 91 <= lookahead && lookahead <= 93 || 123 <= lookahead && lookahead <= 125 { state = 781; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 513; lexer.advance(true); continue; }
                return result;
            }
            514 => {
                if eof { state = 541; lexer.advance(false); continue; }
                if lookahead == 10 { state = 684; lexer.advance(false); continue; }
                if lookahead == 35 { state = 857; lexer.advance(false); continue; }
                if lookahead == 38 { state = 596; lexer.advance(false); continue; }
                if lookahead == 41 { state = 649; lexer.advance(false); continue; }
                if lookahead == 59 { state = 549; lexer.advance(false); continue; }
                if lookahead == 60 { state = 606; lexer.advance(false); continue; }
                if lookahead == 62 { state = 612; lexer.advance(false); continue; }
                if lookahead == 92 { state = 537; lexer.advance(true); continue; }
                if lookahead == 96 { state = 850; lexer.advance(false); continue; }
                if lookahead == 124 { state = 587; lexer.advance(false); continue; }
                if 91 <= lookahead && lookahead <= 93 || 123 <= lookahead && lookahead <= 125 { state = 781; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 514; lexer.advance(true); continue; }
                return result;
            }
            515 => {
                if eof { state = 541; lexer.advance(false); continue; }
                if lookahead == 10 { state = 684; lexer.advance(false); continue; }
                if lookahead == 35 { state = 857; lexer.advance(false); continue; }
                if lookahead == 38 { state = 596; lexer.advance(false); continue; }
                if lookahead == 41 { state = 649; lexer.advance(false); continue; }
                if lookahead == 59 { state = 549; lexer.advance(false); continue; }
                if lookahead == 60 { state = 607; lexer.advance(false); continue; }
                if lookahead == 62 { state = 612; lexer.advance(false); continue; }
                if lookahead == 92 { state = 536; lexer.advance(true); continue; }
                if lookahead == 96 { state = 851; lexer.advance(false); continue; }
                if lookahead == 124 { state = 587; lexer.advance(false); continue; }
                if 91 <= lookahead && lookahead <= 93 || 123 <= lookahead && lookahead <= 125 { state = 781; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 515; lexer.advance(true); continue; }
                return result;
            }
            516 => {
                if eof { state = 541; lexer.advance(false); continue; }
                if lookahead == 10 { state = 684; lexer.advance(false); continue; }
                if lookahead == 35 { state = 857; lexer.advance(false); continue; }
                if lookahead == 38 { state = 596; lexer.advance(false); continue; }
                if lookahead == 41 { state = 649; lexer.advance(false); continue; }
                if lookahead == 59 { state = 549; lexer.advance(false); continue; }
                if lookahead == 60 { state = 607; lexer.advance(false); continue; }
                if lookahead == 62 { state = 612; lexer.advance(false); continue; }
                if lookahead == 92 { state = 538; lexer.advance(true); continue; }
                if lookahead == 96 { state = 850; lexer.advance(false); continue; }
                if lookahead == 124 { state = 587; lexer.advance(false); continue; }
                if 91 <= lookahead && lookahead <= 93 || 123 <= lookahead && lookahead <= 125 { state = 781; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 516; lexer.advance(true); continue; }
                return result;
            }
            517 => {
                if eof { state = 541; lexer.advance(false); continue; }
                if lookahead == 10 { state = 513; lexer.advance(true); continue; }
                return result;
            }
            518 => {
                if eof { state = 541; lexer.advance(false); continue; }
                if lookahead == 10 { state = 515; lexer.advance(true); continue; }
                return result;
            }
            519 => {
                if eof { state = 541; lexer.advance(false); continue; }
                if lookahead == 10 { state = 514; lexer.advance(true); continue; }
                return result;
            }
            520 => {
                if eof { state = 541; lexer.advance(false); continue; }
                if lookahead == 10 { state = 516; lexer.advance(true); continue; }
                return result;
            }
            521 => {
                if eof { state = 541; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (10, 697), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (40, 647), (41, 649),
                    (45, 941), (48, 797), (59, 549), (60, 603), (61, 943), (62, 611), (92, 319), (96, 850),
                    (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 521; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            522 => {
                if eof { state = 541; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (10, 699), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (40, 647),
                    (42, 634), (45, 630), (48, 801), (59, 549), (60, 603), (61, 943), (62, 611), (63, 755),
                    (64, 936), (92, 164), (95, 938), (96, 850), (124, 587), (91, 781), (93, 781), (123, 781),
                    (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 522; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            523 => {
                if eof { state = 541; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (10, 701), (33, 663), (34, 783), (35, 811), (36, 775), (38, 596), (42, 631), (45, 626),
                    (59, 549), (60, 606), (62, 612), (63, 754), (64, 935), (92, 211), (95, 939), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 523; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            524 => {
                if eof { state = 541; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (10, 705), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (42, 634),
                    (45, 630), (48, 801), (59, 549), (60, 603), (61, 943), (62, 611), (63, 755), (64, 936),
                    (92, 168), (95, 938), (96, 850), (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 524; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            525 => {
                if eof { state = 541; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (10, 706), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (41, 649), (45, 941),
                    (48, 798), (59, 549), (60, 604), (62, 611), (92, 322), (96, 850), (124, 587), (91, 781),
                    (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 525; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 800; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 { state = 863; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            526 => {
                if eof { state = 541; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (10, 710), (33, 663), (34, 783), (35, 811), (36, 775), (38, 596), (42, 631), (45, 626),
                    (59, 549), (60, 607), (62, 612), (63, 754), (64, 935), (92, 215), (95, 939), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 526; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            527 => {
                if eof { state = 541; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (10, 715), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (41, 649), (45, 941),
                    (48, 797), (59, 549), (60, 603), (62, 611), (92, 326), (96, 850), (124, 587), (91, 781),
                    (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 527; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            528 => {
                if eof { state = 541; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (10, 720), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (42, 634),
                    (45, 630), (48, 801), (59, 549), (60, 603), (62, 611), (63, 755), (64, 936), (92, 174),
                    (95, 938), (96, 850), (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 528; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            529 => {
                if eof { state = 541; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (10, 727), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (41, 649), (45, 941),
                    (48, 797), (59, 549), (60, 604), (62, 611), (92, 395), (96, 850), (124, 587), (91, 781),
                    (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 529; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            530 => {
                if eof { state = 541; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (10, 730), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (42, 634),
                    (45, 630), (48, 801), (59, 549), (60, 604), (62, 611), (63, 755), (64, 936), (92, 207),
                    (95, 938), (96, 850), (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 530; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            531 => {
                if eof { state = 541; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (10, 733), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (40, 647), (41, 649),
                    (45, 941), (48, 797), (59, 549), (60, 603), (61, 943), (62, 611), (92, 334), (96, 851),
                    (124, 587), (91, 781), (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 531; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            532 => {
                if eof { state = 541; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (10, 745), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (41, 649), (45, 941),
                    (48, 797), (59, 549), (60, 603), (62, 611), (92, 345), (96, 851), (124, 587), (91, 781),
                    (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 532; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            533 => {
                if eof { state = 541; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (10, 746), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (41, 649), (45, 941),
                    (48, 798), (59, 549), (60, 604), (62, 611), (92, 346), (96, 851), (124, 587), (91, 781),
                    (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 533; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 800; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 { state = 863; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            534 => {
                if eof { state = 541; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (10, 748), (34, 783), (35, 857), (36, 776), (38, 596), (39, 485), (41, 649), (45, 941),
                    (48, 797), (59, 549), (60, 604), (62, 611), (92, 349), (96, 851), (124, 587), (91, 781),
                    (93, 781), (123, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 534; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 41 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            535 => {
                if eof { state = 541; lexer.advance(false); continue; }
                if lookahead == 13 { state = 517; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 513; lexer.advance(true); continue; }
                return result;
            }
            536 => {
                if eof { state = 541; lexer.advance(false); continue; }
                if lookahead == 13 { state = 518; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 515; lexer.advance(true); continue; }
                return result;
            }
            537 => {
                if eof { state = 541; lexer.advance(false); continue; }
                if lookahead == 13 { state = 519; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 514; lexer.advance(true); continue; }
                return result;
            }
            538 => {
                if eof { state = 541; lexer.advance(false); continue; }
                if lookahead == 13 { state = 520; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 516; lexer.advance(true); continue; }
                return result;
            }
            539 => {
                if eof { state = 541; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 664), (34, 783), (35, 811), (36, 776), (37, 641), (38, 595), (39, 485), (40, 648),
                    (41, 649), (42, 632), (43, 764), (44, 551), (45, 762), (47, 636), (58, 759), (59, 548),
                    (60, 601), (61, 557), (62, 610), (63, 755), (64, 936), (91, 668), (92, 298), (93, 669),
                    (94, 589), (95, 937), (96, 851), (101, 947), (105, 946), (123, 660), (124, 586), (125, 813),
                    (126, 766),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 539; lexer.advance(true); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            540 => {
                if eof { state = 541; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 666), (34, 783), (35, 857), (36, 776), (38, 502), (39, 485), (40, 648), (41, 649),
                    (45, 941), (48, 797), (59, 484), (60, 602), (62, 611), (91, 668), (92, 301), (96, 850),
                    (123, 660), (124, 584), (93, 781), (125, 781),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 540; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if lookahead != 0 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            541 => {
                result = true; lexer.set_result_symbol(ts_builtin_sym_end); lexer.mark_end();
                return result;
            }
            542 => {
                result = true; lexer.set_result_symbol(anon_sym_in); lexer.mark_end();
                return result;
            }
            543 => {
                result = true; lexer.set_result_symbol(anon_sym_in); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            544 => {
                result = true; lexer.set_result_symbol(anon_sym_in); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            545 => {
                result = true; lexer.set_result_symbol(anon_sym_LPAREN_LPAREN); lexer.mark_end();
                return result;
            }
            546 => {
                result = true; lexer.set_result_symbol(anon_sym_RPAREN_RPAREN); lexer.mark_end();
                return result;
            }
            547 => {
                result = true; lexer.set_result_symbol(anon_sym_SEMI); lexer.mark_end();
                return result;
            }
            548 => {
                result = true; lexer.set_result_symbol(anon_sym_SEMI); lexer.mark_end();
                if lookahead == 38 { state = 658; lexer.advance(false); continue; }
                if lookahead == 59 { state = 657; lexer.advance(false); continue; }
                return result;
            }
            549 => {
                result = true; lexer.set_result_symbol(anon_sym_SEMI); lexer.mark_end();
                if lookahead == 59 { state = 656; lexer.advance(false); continue; }
                return result;
            }
            550 => {
                result = true; lexer.set_result_symbol(anon_sym_COMMA); lexer.mark_end();
                return result;
            }
            551 => {
                result = true; lexer.set_result_symbol(anon_sym_COMMA); lexer.mark_end();
                if lookahead == 44 { state = 845; lexer.advance(false); continue; }
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            552 => {
                result = true; lexer.set_result_symbol(anon_sym_COMMA); lexer.mark_end();
                if lookahead == 44 { state = 844; lexer.advance(false); continue; }
                return result;
            }
            553 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                return result;
            }
            554 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                if lookahead == 61 { state = 598; lexer.advance(false); continue; }
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if lookahead == 126 { state = 673; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            555 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                if lookahead == 61 { state = 597; lexer.advance(false); continue; }
                return result;
            }
            556 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                if lookahead == 61 { state = 597; lexer.advance(false); continue; }
                if lookahead == 126 { state = 672; lexer.advance(false); continue; }
                return result;
            }
            557 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            558 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS_PLUS); lexer.mark_end();
                return result;
            }
            559 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS_PLUS); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            560 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_DASH); lexer.mark_end();
                return result;
            }
            561 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_DASH); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            562 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS_EQ); lexer.mark_end();
                return result;
            }
            563 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS_EQ); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            564 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_EQ); lexer.mark_end();
                return result;
            }
            565 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_EQ); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            566 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR_EQ); lexer.mark_end();
                return result;
            }
            567 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR_EQ); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            568 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_EQ); lexer.mark_end();
                return result;
            }
            569 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_EQ); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            570 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT_EQ); lexer.mark_end();
                return result;
            }
            571 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT_EQ); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            572 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR_STAR_EQ); lexer.mark_end();
                return result;
            }
            573 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR_STAR_EQ); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            574 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT_EQ); lexer.mark_end();
                return result;
            }
            575 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT_EQ); lexer.mark_end();
                return result;
            }
            576 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP_EQ); lexer.mark_end();
                return result;
            }
            577 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET_EQ); lexer.mark_end();
                return result;
            }
            578 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET_EQ); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            579 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE_EQ); lexer.mark_end();
                return result;
            }
            580 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE_PIPE); lexer.mark_end();
                return result;
            }
            581 => {
                result = true; lexer.set_result_symbol(anon_sym_DASHo); lexer.mark_end();
                return result;
            }
            582 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP_AMP); lexer.mark_end();
                return result;
            }
            583 => {
                result = true; lexer.set_result_symbol(anon_sym_DASHa); lexer.mark_end();
                return result;
            }
            584 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                return result;
            }
            585 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                if lookahead == 38 { state = 662; lexer.advance(false); continue; }
                return result;
            }
            586 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                if lookahead == 38 { state = 662; lexer.advance(false); continue; }
                if lookahead == 61 { state = 579; lexer.advance(false); continue; }
                if lookahead == 124 { state = 580; lexer.advance(false); continue; }
                return result;
            }
            587 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                if lookahead == 38 { state = 662; lexer.advance(false); continue; }
                if lookahead == 124 { state = 580; lexer.advance(false); continue; }
                return result;
            }
            588 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                if lookahead == 61 { state = 579; lexer.advance(false); continue; }
                if lookahead == 124 { state = 580; lexer.advance(false); continue; }
                return result;
            }
            589 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET); lexer.mark_end();
                if lookahead == 61 { state = 578; lexer.advance(false); continue; }
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if lookahead == 94 { state = 847; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            590 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET); lexer.mark_end();
                if lookahead == 61 { state = 578; lexer.advance(false); continue; }
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            591 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET); lexer.mark_end();
                if lookahead == 61 { state = 577; lexer.advance(false); continue; }
                return result;
            }
            592 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET); lexer.mark_end();
                if lookahead == 94 { state = 846; lexer.advance(false); continue; }
                return result;
            }
            593 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP); lexer.mark_end();
                return result;
            }
            594 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP); lexer.mark_end();
                if lookahead == 38 { state = 582; lexer.advance(false); continue; }
                if lookahead == 61 { state = 576; lexer.advance(false); continue; }
                return result;
            }
            595 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP); lexer.mark_end();
                if lookahead == 38 { state = 582; lexer.advance(false); continue; }
                if lookahead == 61 { state = 576; lexer.advance(false); continue; }
                if lookahead == 62 { state = 674; lexer.advance(false); continue; }
                return result;
            }
            596 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP); lexer.mark_end();
                if lookahead == 38 { state = 582; lexer.advance(false); continue; }
                if lookahead == 62 { state = 674; lexer.advance(false); continue; }
                return result;
            }
            597 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ_EQ); lexer.mark_end();
                return result;
            }
            598 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ_EQ); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            599 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG_EQ); lexer.mark_end();
                return result;
            }
            600 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG_EQ); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            601 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 38 { state = 676; lexer.advance(false); continue; }
                if lookahead == 40 { state = 853; lexer.advance(false); continue; }
                if lookahead == 60 { state = 619; lexer.advance(false); continue; }
                if lookahead == 61 { state = 615; lexer.advance(false); continue; }
                return result;
            }
            602 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 38 { state = 676; lexer.advance(false); continue; }
                if lookahead == 40 { state = 853; lexer.advance(false); continue; }
                if lookahead == 60 { state = 499; lexer.advance(false); continue; }
                return result;
            }
            603 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 38 { state = 676; lexer.advance(false); continue; }
                if lookahead == 40 { state = 853; lexer.advance(false); continue; }
                if lookahead == 60 { state = 618; lexer.advance(false); continue; }
                return result;
            }
            604 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 38 { state = 676; lexer.advance(false); continue; }
                if lookahead == 40 { state = 853; lexer.advance(false); continue; }
                if lookahead == 60 { state = 617; lexer.advance(false); continue; }
                return result;
            }
            605 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 38 { state = 676; lexer.advance(false); continue; }
                if lookahead == 60 { state = 499; lexer.advance(false); continue; }
                return result;
            }
            606 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 38 { state = 676; lexer.advance(false); continue; }
                if lookahead == 60 { state = 618; lexer.advance(false); continue; }
                return result;
            }
            607 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 38 { state = 676; lexer.advance(false); continue; }
                if lookahead == 60 { state = 617; lexer.advance(false); continue; }
                return result;
            }
            608 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 40 { state = 853; lexer.advance(false); continue; }
                if lookahead == 60 { state = 620; lexer.advance(false); continue; }
                if lookahead == 61 { state = 615; lexer.advance(false); continue; }
                return result;
            }
            609 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 60 { state = 620; lexer.advance(false); continue; }
                if lookahead == 61 { state = 615; lexer.advance(false); continue; }
                return result;
            }
            610 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                if lookahead == 38 { state = 677; lexer.advance(false); continue; }
                if lookahead == 40 { state = 854; lexer.advance(false); continue; }
                if lookahead == 61 { state = 616; lexer.advance(false); continue; }
                if lookahead == 62 { state = 622; lexer.advance(false); continue; }
                if lookahead == 124 { state = 678; lexer.advance(false); continue; }
                return result;
            }
            611 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                if lookahead == 38 { state = 677; lexer.advance(false); continue; }
                if lookahead == 40 { state = 854; lexer.advance(false); continue; }
                if lookahead == 62 { state = 621; lexer.advance(false); continue; }
                if lookahead == 124 { state = 678; lexer.advance(false); continue; }
                return result;
            }
            612 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                if lookahead == 38 { state = 677; lexer.advance(false); continue; }
                if lookahead == 62 { state = 621; lexer.advance(false); continue; }
                if lookahead == 124 { state = 678; lexer.advance(false); continue; }
                return result;
            }
            613 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                if lookahead == 40 { state = 854; lexer.advance(false); continue; }
                if lookahead == 61 { state = 616; lexer.advance(false); continue; }
                if lookahead == 62 { state = 622; lexer.advance(false); continue; }
                return result;
            }
            614 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                if lookahead == 61 { state = 616; lexer.advance(false); continue; }
                if lookahead == 62 { state = 622; lexer.advance(false); continue; }
                return result;
            }
            615 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_EQ); lexer.mark_end();
                return result;
            }
            616 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_EQ); lexer.mark_end();
                return result;
            }
            617 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT); lexer.mark_end();
                if lookahead == 45 { state = 681; lexer.advance(false); continue; }
                return result;
            }
            618 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT); lexer.mark_end();
                if lookahead == 45 { state = 681; lexer.advance(false); continue; }
                if lookahead == 60 { state = 753; lexer.advance(false); continue; }
                return result;
            }
            619 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT); lexer.mark_end();
                if lookahead == 45 { state = 681; lexer.advance(false); continue; }
                if lookahead == 60 { state = 753; lexer.advance(false); continue; }
                if lookahead == 61 { state = 574; lexer.advance(false); continue; }
                return result;
            }
            620 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT); lexer.mark_end();
                if lookahead == 61 { state = 574; lexer.advance(false); continue; }
                return result;
            }
            621 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT); lexer.mark_end();
                return result;
            }
            622 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT); lexer.mark_end();
                if lookahead == 61 { state = 575; lexer.advance(false); continue; }
                return result;
            }
            623 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                return result;
            }
            624 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                if lookahead == 43 { state = 559; lexer.advance(false); continue; }
                if lookahead == 61 { state = 563; lexer.advance(false); continue; }
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            625 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                if lookahead == 43 { state = 558; lexer.advance(false); continue; }
                if lookahead == 61 { state = 562; lexer.advance(false); continue; }
                return result;
            }
            626 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                return result;
            }
            627 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 561; lexer.advance(false); continue; }
                if lookahead == 48 { state = 797; lexer.advance(false); continue; }
                if lookahead == 61 { state = 565; lexer.advance(false); continue; }
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            628 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 560; lexer.advance(false); continue; }
                if lookahead == 61 { state = 564; lexer.advance(false); continue; }
                return result;
            }
            629 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 560; lexer.advance(false); continue; }
                if lookahead == 61 { state = 564; lexer.advance(false); continue; }
                if lookahead == 97 { state = 583; lexer.advance(false); continue; }
                if lookahead == 111 { state = 581; lexer.advance(false); continue; }
                return result;
            }
            630 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 48 { state = 797; lexer.advance(false); continue; }
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            631 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                return result;
            }
            632 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                if lookahead == 42 { state = 645; lexer.advance(false); continue; }
                if lookahead == 61 { state = 567; lexer.advance(false); continue; }
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            633 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                if lookahead == 42 { state = 646; lexer.advance(false); continue; }
                if lookahead == 61 { state = 566; lexer.advance(false); continue; }
                return result;
            }
            634 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            635 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH); lexer.mark_end();
                return result;
            }
            636 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH); lexer.mark_end();
                if lookahead == 35 { state = 841; lexer.advance(false); continue; }
                if lookahead == 37 { state = 843; lexer.advance(false); continue; }
                if lookahead == 47 { state = 839; lexer.advance(false); continue; }
                if lookahead == 61 { state = 569; lexer.advance(false); continue; }
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            637 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH); lexer.mark_end();
                if lookahead == 35 { state = 840; lexer.advance(false); continue; }
                if lookahead == 37 { state = 842; lexer.advance(false); continue; }
                if lookahead == 47 { state = 838; lexer.advance(false); continue; }
                return result;
            }
            638 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH); lexer.mark_end();
                if lookahead == 61 { state = 569; lexer.advance(false); continue; }
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            639 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH); lexer.mark_end();
                if lookahead == 61 { state = 568; lexer.advance(false); continue; }
                return result;
            }
            640 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT); lexer.mark_end();
                return result;
            }
            641 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT); lexer.mark_end();
                if lookahead == 37 { state = 836; lexer.advance(false); continue; }
                if lookahead == 61 { state = 571; lexer.advance(false); continue; }
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            642 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT); lexer.mark_end();
                if lookahead == 37 { state = 835; lexer.advance(false); continue; }
                return result;
            }
            643 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT); lexer.mark_end();
                if lookahead == 61 { state = 571; lexer.advance(false); continue; }
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            644 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT); lexer.mark_end();
                if lookahead == 61 { state = 570; lexer.advance(false); continue; }
                return result;
            }
            645 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR_STAR); lexer.mark_end();
                if lookahead == 61 { state = 573; lexer.advance(false); continue; }
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            646 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR_STAR); lexer.mark_end();
                if lookahead == 61 { state = 572; lexer.advance(false); continue; }
                return result;
            }
            647 => {
                result = true; lexer.set_result_symbol(anon_sym_LPAREN); lexer.mark_end();
                return result;
            }
            648 => {
                result = true; lexer.set_result_symbol(anon_sym_LPAREN); lexer.mark_end();
                if lookahead == 40 { state = 545; lexer.advance(false); continue; }
                return result;
            }
            649 => {
                result = true; lexer.set_result_symbol(anon_sym_RPAREN); lexer.mark_end();
                return result;
            }
            650 => {
                result = true; lexer.set_result_symbol(aux_sym__c_word_token1); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 650; lexer.advance(false); continue; }
                return result;
            }
            651 => {
                result = true; lexer.set_result_symbol(anon_sym_esac); lexer.mark_end();
                return result;
            }
            652 => {
                result = true; lexer.set_result_symbol(anon_sym_esac); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 863; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            653 => {
                result = true; lexer.set_result_symbol(anon_sym_esac); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            654 => {
                result = true; lexer.set_result_symbol(anon_sym_esac); lexer.mark_end();
                if lookahead == 92 { state = 17; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            655 => {
                result = true; lexer.set_result_symbol(anon_sym_esac); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            656 => {
                result = true; lexer.set_result_symbol(anon_sym_SEMI_SEMI); lexer.mark_end();
                return result;
            }
            657 => {
                result = true; lexer.set_result_symbol(anon_sym_SEMI_SEMI); lexer.mark_end();
                if lookahead == 38 { state = 659; lexer.advance(false); continue; }
                return result;
            }
            658 => {
                result = true; lexer.set_result_symbol(anon_sym_SEMI_AMP); lexer.mark_end();
                return result;
            }
            659 => {
                result = true; lexer.set_result_symbol(anon_sym_SEMI_SEMI_AMP); lexer.mark_end();
                return result;
            }
            660 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACE); lexer.mark_end();
                return result;
            }
            661 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACE); lexer.mark_end();
                return result;
            }
            662 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE_AMP); lexer.mark_end();
                return result;
            }
            663 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG); lexer.mark_end();
                return result;
            }
            664 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG); lexer.mark_end();
                if lookahead == 61 { state = 600; lexer.advance(false); continue; }
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            665 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG); lexer.mark_end();
                if lookahead == 61 { state = 599; lexer.advance(false); continue; }
                return result;
            }
            666 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            667 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK); lexer.mark_end();
                return result;
            }
            668 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK); lexer.mark_end();
                if lookahead == 91 { state = 670; lexer.advance(false); continue; }
                return result;
            }
            669 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACK); lexer.mark_end();
                return result;
            }
            670 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK_LBRACK); lexer.mark_end();
                return result;
            }
            671 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACK_RBRACK); lexer.mark_end();
                return result;
            }
            672 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ_TILDE); lexer.mark_end();
                return result;
            }
            673 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ_TILDE); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            674 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP_GT); lexer.mark_end();
                if lookahead == 62 { state = 675; lexer.advance(false); continue; }
                return result;
            }
            675 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP_GT_GT); lexer.mark_end();
                return result;
            }
            676 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_AMP); lexer.mark_end();
                if lookahead == 45 { state = 679; lexer.advance(false); continue; }
                return result;
            }
            677 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_AMP); lexer.mark_end();
                if lookahead == 45 { state = 680; lexer.advance(false); continue; }
                return result;
            }
            678 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_PIPE); lexer.mark_end();
                return result;
            }
            679 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_AMP_DASH); lexer.mark_end();
                return result;
            }
            680 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_AMP_DASH); lexer.mark_end();
                return result;
            }
            681 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT_DASH); lexer.mark_end();
                return result;
            }
            682 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 682; lexer.advance(false); continue; }
                if lookahead == 43 { state = 765; lexer.advance(false); continue; }
                if lookahead == 45 { state = 763; lexer.advance(false); continue; }
                if lookahead == 92 { state = 304; lexer.advance(false); continue; }
                if lookahead == 126 { state = 766; lexer.advance(false); continue; }
                return result;
            }
            683 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 683; lexer.advance(false); continue; }
                if lookahead == 92 { state = 14; lexer.advance(false); continue; }
                return result;
            }
            684 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 684; lexer.advance(false); continue; }
                return result;
            }
            685 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 685; lexer.advance(false); continue; }
                if lookahead == 92 { state = 312; lexer.advance(false); continue; }
                return result;
            }
            686 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 686; lexer.advance(false); continue; }
                if lookahead == 92 { state = 100; lexer.advance(false); continue; }
                return result;
            }
            687 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 687; lexer.advance(false); continue; }
                if lookahead == 92 { state = 28; lexer.advance(false); continue; }
                return result;
            }
            688 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 688; lexer.advance(false); continue; }
                if lookahead == 92 { state = 313; lexer.advance(false); continue; }
                return result;
            }
            689 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 689; lexer.advance(false); continue; }
                if lookahead == 92 { state = 185; lexer.advance(false); continue; }
                return result;
            }
            690 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 690; lexer.advance(false); continue; }
                if lookahead == 92 { state = 30; lexer.advance(false); continue; }
                return result;
            }
            691 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 691; lexer.advance(false); continue; }
                if lookahead == 92 { state = 317; lexer.advance(false); continue; }
                return result;
            }
            692 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 692; lexer.advance(false); continue; }
                if lookahead == 92 { state = 104; lexer.advance(false); continue; }
                return result;
            }
            693 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 693; lexer.advance(false); continue; }
                if lookahead == 92 { state = 32; lexer.advance(false); continue; }
                return result;
            }
            694 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 694; lexer.advance(false); continue; }
                if lookahead == 92 { state = 318; lexer.advance(false); continue; }
                return result;
            }
            695 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 695; lexer.advance(false); continue; }
                if lookahead == 92 { state = 106; lexer.advance(false); continue; }
                return result;
            }
            696 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 696; lexer.advance(false); continue; }
                if lookahead == 92 { state = 36; lexer.advance(false); continue; }
                return result;
            }
            697 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 697; lexer.advance(false); continue; }
                if lookahead == 92 { state = 319; lexer.advance(false); continue; }
                return result;
            }
            698 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 698; lexer.advance(false); continue; }
                if lookahead == 92 { state = 188; lexer.advance(false); continue; }
                return result;
            }
            699 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 699; lexer.advance(false); continue; }
                if lookahead == 92 { state = 164; lexer.advance(false); continue; }
                return result;
            }
            700 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 700; lexer.advance(false); continue; }
                if lookahead == 92 { state = 320; lexer.advance(false); continue; }
                return result;
            }
            701 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 701; lexer.advance(false); continue; }
                if lookahead == 92 { state = 211; lexer.advance(false); continue; }
                return result;
            }
            702 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 702; lexer.advance(false); continue; }
                if lookahead == 92 { state = 39; lexer.advance(false); continue; }
                return result;
            }
            703 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 703; lexer.advance(false); continue; }
                if lookahead == 92 { state = 321; lexer.advance(false); continue; }
                return result;
            }
            704 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 704; lexer.advance(false); continue; }
                if lookahead == 92 { state = 109; lexer.advance(false); continue; }
                return result;
            }
            705 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 705; lexer.advance(false); continue; }
                if lookahead == 92 { state = 168; lexer.advance(false); continue; }
                return result;
            }
            706 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 706; lexer.advance(false); continue; }
                if lookahead == 92 { state = 322; lexer.advance(false); continue; }
                return result;
            }
            707 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 707; lexer.advance(false); continue; }
                if lookahead == 92 { state = 192; lexer.advance(false); continue; }
                return result;
            }
            708 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 708; lexer.advance(false); continue; }
                if lookahead == 92 { state = 41; lexer.advance(false); continue; }
                return result;
            }
            709 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 709; lexer.advance(false); continue; }
                if lookahead == 92 { state = 323; lexer.advance(false); continue; }
                return result;
            }
            710 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 710; lexer.advance(false); continue; }
                if lookahead == 92 { state = 215; lexer.advance(false); continue; }
                return result;
            }
            711 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 711; lexer.advance(false); continue; }
                if lookahead == 92 { state = 47; lexer.advance(false); continue; }
                return result;
            }
            712 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 712; lexer.advance(false); continue; }
                if lookahead == 92 { state = 324; lexer.advance(false); continue; }
                return result;
            }
            713 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 713; lexer.advance(false); continue; }
                if lookahead == 92 { state = 190; lexer.advance(false); continue; }
                return result;
            }
            714 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 714; lexer.advance(false); continue; }
                if lookahead == 92 { state = 49; lexer.advance(false); continue; }
                return result;
            }
            715 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 715; lexer.advance(false); continue; }
                if lookahead == 92 { state = 326; lexer.advance(false); continue; }
                return result;
            }
            716 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 716; lexer.advance(false); continue; }
                if lookahead == 92 { state = 120; lexer.advance(false); continue; }
                return result;
            }
            717 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 717; lexer.advance(false); continue; }
                if lookahead == 92 { state = 221; lexer.advance(false); continue; }
                return result;
            }
            718 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 718; lexer.advance(false); continue; }
                if lookahead == 92 { state = 393; lexer.advance(false); continue; }
                return result;
            }
            719 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 719; lexer.advance(false); continue; }
                if lookahead == 92 { state = 197; lexer.advance(false); continue; }
                return result;
            }
            720 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 720; lexer.advance(false); continue; }
                if lookahead == 92 { state = 174; lexer.advance(false); continue; }
                return result;
            }
            721 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 721; lexer.advance(false); continue; }
                if lookahead == 92 { state = 394; lexer.advance(false); continue; }
                return result;
            }
            722 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 722; lexer.advance(false); continue; }
                if lookahead == 92 { state = 213; lexer.advance(false); continue; }
                return result;
            }
            723 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 723; lexer.advance(false); continue; }
                if lookahead == 92 { state = 52; lexer.advance(false); continue; }
                return result;
            }
            724 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 724; lexer.advance(false); continue; }
                if lookahead == 92 { state = 328; lexer.advance(false); continue; }
                return result;
            }
            725 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 725; lexer.advance(false); continue; }
                if lookahead == 92 { state = 140; lexer.advance(false); continue; }
                return result;
            }
            726 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 726; lexer.advance(false); continue; }
                if lookahead == 92 { state = 223; lexer.advance(false); continue; }
                return result;
            }
            727 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 727; lexer.advance(false); continue; }
                if lookahead == 92 { state = 395; lexer.advance(false); continue; }
                return result;
            }
            728 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 728; lexer.advance(false); continue; }
                if lookahead == 92 { state = 54; lexer.advance(false); continue; }
                return result;
            }
            729 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 729; lexer.advance(false); continue; }
                if lookahead == 92 { state = 330; lexer.advance(false); continue; }
                return result;
            }
            730 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 730; lexer.advance(false); continue; }
                if lookahead == 92 { state = 207; lexer.advance(false); continue; }
                return result;
            }
            731 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 731; lexer.advance(false); continue; }
                if lookahead == 92 { state = 332; lexer.advance(false); continue; }
                return result;
            }
            732 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 732; lexer.advance(false); continue; }
                if lookahead == 92 { state = 225; lexer.advance(false); continue; }
                return result;
            }
            733 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 733; lexer.advance(false); continue; }
                if lookahead == 92 { state = 334; lexer.advance(false); continue; }
                return result;
            }
            734 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 734; lexer.advance(false); continue; }
                if lookahead == 92 { state = 59; lexer.advance(false); continue; }
                return result;
            }
            735 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 735; lexer.advance(false); continue; }
                if lookahead == 92 { state = 336; lexer.advance(false); continue; }
                return result;
            }
            736 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 736; lexer.advance(false); continue; }
                if lookahead == 92 { state = 68; lexer.advance(false); continue; }
                return result;
            }
            737 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 737; lexer.advance(false); continue; }
                if lookahead == 92 { state = 337; lexer.advance(false); continue; }
                return result;
            }
            738 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 738; lexer.advance(false); continue; }
                if lookahead == 92 { state = 230; lexer.advance(false); continue; }
                return result;
            }
            739 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 739; lexer.advance(false); continue; }
                if lookahead == 92 { state = 341; lexer.advance(false); continue; }
                return result;
            }
            740 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 740; lexer.advance(false); continue; }
                if lookahead == 92 { state = 180; lexer.advance(false); continue; }
                return result;
            }
            741 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 741; lexer.advance(false); continue; }
                if lookahead == 92 { state = 342; lexer.advance(false); continue; }
                return result;
            }
            742 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 742; lexer.advance(false); continue; }
                if lookahead == 92 { state = 115; lexer.advance(false); continue; }
                return result;
            }
            743 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 743; lexer.advance(false); continue; }
                if lookahead == 92 { state = 344; lexer.advance(false); continue; }
                return result;
            }
            744 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 744; lexer.advance(false); continue; }
                if lookahead == 92 { state = 127; lexer.advance(false); continue; }
                return result;
            }
            745 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 745; lexer.advance(false); continue; }
                if lookahead == 92 { state = 345; lexer.advance(false); continue; }
                return result;
            }
            746 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 746; lexer.advance(false); continue; }
                if lookahead == 92 { state = 346; lexer.advance(false); continue; }
                return result;
            }
            747 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 747; lexer.advance(false); continue; }
                if lookahead == 92 { state = 347; lexer.advance(false); continue; }
                return result;
            }
            748 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 748; lexer.advance(false); continue; }
                if lookahead == 92 { state = 349; lexer.advance(false); continue; }
                return result;
            }
            749 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 749; lexer.advance(false); continue; }
                if lookahead == 92 { state = 355; lexer.advance(false); continue; }
                return result;
            }
            750 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 750; lexer.advance(false); continue; }
                if lookahead == 92 { state = 365; lexer.advance(false); continue; }
                return result;
            }
            751 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 751; lexer.advance(false); continue; }
                if lookahead == 92 { state = 368; lexer.advance(false); continue; }
                return result;
            }
            752 => {
                result = true; lexer.set_result_symbol(aux_sym_heredoc_redirect_token1); lexer.mark_end();
                if lookahead == 10 { state = 752; lexer.advance(false); continue; }
                if lookahead == 92 { state = 378; lexer.advance(false); continue; }
                return result;
            }
            753 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT_LT); lexer.mark_end();
                return result;
            }
            754 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK); lexer.mark_end();
                return result;
            }
            755 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            756 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON); lexer.mark_end();
                return result;
            }
            757 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON); lexer.mark_end();
                if lookahead == 43 { state = 830; lexer.advance(false); continue; }
                if lookahead == 45 { state = 827; lexer.advance(false); continue; }
                if lookahead == 61 { state = 824; lexer.advance(false); continue; }
                if lookahead == 63 { state = 834; lexer.advance(false); continue; }
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            758 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON); lexer.mark_end();
                if lookahead == 43 { state = 829; lexer.advance(false); continue; }
                if lookahead == 45 { state = 826; lexer.advance(false); continue; }
                if lookahead == 61 { state = 823; lexer.advance(false); continue; }
                if lookahead == 63 { state = 833; lexer.advance(false); continue; }
                return result;
            }
            759 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            760 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS_PLUS2); lexer.mark_end();
                return result;
            }
            761 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_DASH2); lexer.mark_end();
                return result;
            }
            762 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH2); lexer.mark_end();
                return result;
            }
            763 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH2); lexer.mark_end();
                if lookahead == 45 { state = 761; lexer.advance(false); continue; }
                return result;
            }
            764 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS2); lexer.mark_end();
                return result;
            }
            765 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS2); lexer.mark_end();
                if lookahead == 43 { state = 760; lexer.advance(false); continue; }
                return result;
            }
            766 => {
                result = true; lexer.set_result_symbol(anon_sym_TILDE); lexer.mark_end();
                return result;
            }
            767 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR_LPAREN_LPAREN); lexer.mark_end();
                return result;
            }
            768 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR_LBRACK); lexer.mark_end();
                return result;
            }
            769 => {
                result = true; lexer.set_result_symbol(aux_sym_brace_expression_token1); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 769; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            770 => {
                result = true; lexer.set_result_symbol(aux_sym_brace_expression_token1); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 { state = 770; lexer.advance(false); continue; }
                return result;
            }
            771 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT_DOT); lexer.mark_end();
                return result;
            }
            772 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT_DOT); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            773 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACE2); lexer.mark_end();
                return result;
            }
            774 => {
                result = true; lexer.set_result_symbol(aux_sym_concatenation_token1); lexer.mark_end();
                return result;
            }
            775 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR); lexer.mark_end();
                return result;
            }
            776 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR); lexer.mark_end();
                if lookahead == 39 { state = 486; lexer.advance(false); continue; }
                if lookahead == 40 { state = 849; lexer.advance(false); continue; }
                if lookahead == 91 { state = 768; lexer.advance(false); continue; }
                if lookahead == 96 { state = 852; lexer.advance(false); continue; }
                if lookahead == 123 { state = 812; lexer.advance(false); continue; }
                return result;
            }
            777 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR); lexer.mark_end();
                if lookahead == 39 { state = 486; lexer.advance(false); continue; }
                if lookahead == 40 { state = 848; lexer.advance(false); continue; }
                if lookahead == 96 { state = 852; lexer.advance(false); continue; }
                if lookahead == 123 { state = 812; lexer.advance(false); continue; }
                return result;
            }
            778 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR); lexer.mark_end();
                if lookahead == 40 { state = 849; lexer.advance(false); continue; }
                if lookahead == 91 { state = 768; lexer.advance(false); continue; }
                if lookahead == 96 { state = 852; lexer.advance(false); continue; }
                if lookahead == 123 { state = 812; lexer.advance(false); continue; }
                return result;
            }
            779 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR); lexer.mark_end();
                if lookahead == 40 { state = 848; lexer.advance(false); continue; }
                if lookahead == 96 { state = 852; lexer.advance(false); continue; }
                return result;
            }
            780 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR); lexer.mark_end();
                if lookahead == 40 { state = 848; lexer.advance(false); continue; }
                if lookahead == 96 { state = 852; lexer.advance(false); continue; }
                if lookahead == 123 { state = 812; lexer.advance(false); continue; }
                return result;
            }
            781 => {
                result = true; lexer.set_result_symbol(sym__special_character); lexer.mark_end();
                return result;
            }
            782 => {
                result = true; lexer.set_result_symbol(sym__special_character); lexer.mark_end();
                if lookahead == 93 { state = 671; lexer.advance(false); continue; }
                return result;
            }
            783 => {
                result = true; lexer.set_result_symbol(anon_sym_DQUOTE); lexer.mark_end();
                return result;
            }
            784 => {
                result = true; lexer.set_result_symbol(sym_string_content); lexer.mark_end();
                if lookahead == 10 { state = 790; lexer.advance(false); continue; }
                if lookahead == 92 { state = 300; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 13 && lookahead != 34 && lookahead != 36 && lookahead != 96 { state = 792; lexer.advance(false); continue; }
                return result;
            }
            785 => {
                result = true; lexer.set_result_symbol(sym_string_content); lexer.mark_end();
                if lookahead == 10 { state = 788; lexer.advance(false); continue; }
                if lookahead == 92 { state = 300; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 13 && lookahead != 34 && lookahead != 36 && lookahead != 96 { state = 792; lexer.advance(false); continue; }
                return result;
            }
            786 => {
                result = true; lexer.set_result_symbol(sym_string_content); lexer.mark_end();
                if lookahead == 10 { state = 789; lexer.advance(false); continue; }
                if lookahead == 92 { state = 300; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 13 && lookahead != 34 && lookahead != 36 && lookahead != 96 { state = 792; lexer.advance(false); continue; }
                return result;
            }
            787 => {
                result = true; lexer.set_result_symbol(sym_string_content); lexer.mark_end();
                if lookahead == 10 { state = 792; lexer.advance(false); continue; }
                if lookahead == 92 { state = 855; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 13 && lookahead != 34 && lookahead != 36 && lookahead != 96 { state = 791; lexer.advance(false); continue; }
                return result;
            }
            788 => {
                result = true; lexer.set_result_symbol(sym_string_content); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (33, 663), (34, 783), (35, 811), (36, 775), (42, 631), (45, 626), (63, 754), (64, 935),
                    (92, 144), (95, 939),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 10 || lookahead == 13 { state = 443; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 788; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 95 || 122 < lookahead) { state = 792; lexer.advance(false); continue; }
                return result;
            }
            789 => {
                result = true; lexer.set_result_symbol(sym_string_content); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (33, 663), (35, 811), (36, 775), (42, 631), (45, 626), (63, 754), (64, 935), (92, 147),
                    (95, 939),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 10 || lookahead == 13 { state = 446; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 789; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 32 || 36 < lookahead) && (lookahead < 95 || 122 < lookahead) { state = 792; lexer.advance(false); continue; }
                return result;
            }
            790 => {
                result = true; lexer.set_result_symbol(sym_string_content); lexer.mark_end();
                if lookahead == 34 { state = 783; lexer.advance(false); continue; }
                if lookahead == 35 { state = 791; lexer.advance(false); continue; }
                if lookahead == 36 { state = 778; lexer.advance(false); continue; }
                if lookahead == 92 { state = 299; lexer.advance(false); continue; }
                if lookahead == 96 { state = 850; lexer.advance(false); continue; }
                if lookahead == 10 || lookahead == 13 { state = 474; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 790; lexer.advance(false); continue; }
                if lookahead != 0 { state = 792; lexer.advance(false); continue; }
                return result;
            }
            791 => {
                result = true; lexer.set_result_symbol(sym_string_content); lexer.mark_end();
                if lookahead == 92 { state = 855; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 34 && lookahead != 36 && lookahead != 96 { state = 791; lexer.advance(false); continue; }
                return result;
            }
            792 => {
                result = true; lexer.set_result_symbol(sym_string_content); lexer.mark_end();
                if lookahead == 92 { state = 300; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 13 && lookahead != 34 && lookahead != 36 && lookahead != 96 { state = 792; lexer.advance(false); continue; }
                return result;
            }
            793 => {
                result = true; lexer.set_result_symbol(sym_string_content); lexer.mark_end();
                if lookahead == 92 { state = 300; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 13 && lookahead != 34 && lookahead != 36 && lookahead != 96 { state = 792; lexer.advance(false); continue; }
                return result;
            }
            794 => {
                result = true; lexer.set_result_symbol(sym_raw_string); lexer.mark_end();
                return result;
            }
            795 => {
                result = true; lexer.set_result_symbol(sym_ansi_c_string); lexer.mark_end();
                return result;
            }
            796 => {
                result = true; lexer.set_result_symbol(sym_ansi_c_string); lexer.mark_end();
                if lookahead == 39 { state = 795; lexer.advance(false); continue; }
                if lookahead == 92 { state = 487; lexer.advance(false); continue; }
                if lookahead != 0 { state = 486; lexer.advance(false); continue; }
                return result;
            }
            797 => {
                result = true; lexer.set_result_symbol(aux_sym_number_token1); lexer.mark_end();
                if lookahead == 35 { state = 809; lexer.advance(false); continue; }
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if lookahead == 120 { state = 948; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if !eof && set_contains(&sym_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            798 => {
                result = true; lexer.set_result_symbol(aux_sym_number_token1); lexer.mark_end();
                if lookahead == 35 { state = 809; lexer.advance(false); continue; }
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if lookahead == 120 { state = 862; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 800; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 863; lexer.advance(false); continue; }
                if !eof && set_contains(&sym_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            799 => {
                result = true; lexer.set_result_symbol(aux_sym_number_token1); lexer.mark_end();
                if lookahead == 35 { state = 809; lexer.advance(false); continue; }
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if !eof && set_contains(&sym_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            800 => {
                result = true; lexer.set_result_symbol(aux_sym_number_token1); lexer.mark_end();
                if lookahead == 35 { state = 809; lexer.advance(false); continue; }
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 800; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 863; lexer.advance(false); continue; }
                if !eof && set_contains(&sym_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            801 => {
                result = true; lexer.set_result_symbol(aux_sym_number_token1); lexer.mark_end();
                if lookahead == 35 { state = 809; lexer.advance(false); continue; }
                if lookahead == 92 { state = 17; lexer.advance(false); continue; }
                if lookahead == 120 { state = 867; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if !eof && set_contains(&sym_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            802 => {
                result = true; lexer.set_result_symbol(aux_sym_number_token1); lexer.mark_end();
                if lookahead == 35 { state = 809; lexer.advance(false); continue; }
                if lookahead == 92 { state = 17; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if !eof && set_contains(&sym_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            803 => {
                result = true; lexer.set_result_symbol(aux_sym_number_token1); lexer.mark_end();
                if lookahead == 35 { state = 810; lexer.advance(false); continue; }
                if lookahead == 120 { state = 873; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 805; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            804 => {
                result = true; lexer.set_result_symbol(aux_sym_number_token1); lexer.mark_end();
                if lookahead == 35 { state = 810; lexer.advance(false); continue; }
                if lookahead == 120 { state = 511; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 806; lexer.advance(false); continue; }
                return result;
            }
            805 => {
                result = true; lexer.set_result_symbol(aux_sym_number_token1); lexer.mark_end();
                if lookahead == 35 { state = 810; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 805; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            806 => {
                result = true; lexer.set_result_symbol(aux_sym_number_token1); lexer.mark_end();
                if lookahead == 35 { state = 810; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 806; lexer.advance(false); continue; }
                return result;
            }
            807 => {
                result = true; lexer.set_result_symbol(aux_sym_number_token1); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 64 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 807; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            808 => {
                result = true; lexer.set_result_symbol(aux_sym_number_token1); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 64 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 808; lexer.advance(false); continue; }
                return result;
            }
            809 => {
                result = true; lexer.set_result_symbol(aux_sym_number_token2); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 64 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 807; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            810 => {
                result = true; lexer.set_result_symbol(aux_sym_number_token2); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 64 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 808; lexer.advance(false); continue; }
                return result;
            }
            811 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND); lexer.mark_end();
                return result;
            }
            812 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR_LBRACE); lexer.mark_end();
                return result;
            }
            813 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACE3); lexer.mark_end();
                return result;
            }
            814 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG2); lexer.mark_end();
                return result;
            }
            815 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG2); lexer.mark_end();
                if lookahead == 61 { state = 600; lexer.advance(false); continue; }
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            816 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                return result;
            }
            817 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            818 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR2); lexer.mark_end();
                return result;
            }
            819 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR2); lexer.mark_end();
                if lookahead == 42 { state = 645; lexer.advance(false); continue; }
                if lookahead == 61 { state = 567; lexer.advance(false); continue; }
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            820 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND2); lexer.mark_end();
                return result;
            }
            821 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ2); lexer.mark_end();
                return result;
            }
            822 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ2); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            823 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON_EQ); lexer.mark_end();
                return result;
            }
            824 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON_EQ); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            825 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH3); lexer.mark_end();
                return result;
            }
            826 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON_DASH); lexer.mark_end();
                return result;
            }
            827 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON_DASH); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            828 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS3); lexer.mark_end();
                return result;
            }
            829 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON_PLUS); lexer.mark_end();
                return result;
            }
            830 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON_PLUS); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            831 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK2); lexer.mark_end();
                return result;
            }
            832 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK2); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            833 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON_QMARK); lexer.mark_end();
                return result;
            }
            834 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON_QMARK); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            835 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT_PERCENT); lexer.mark_end();
                return result;
            }
            836 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT_PERCENT); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            837 => {
                result = true; lexer.set_result_symbol(aux_sym__expansion_regex_token1); lexer.mark_end();
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 837; lexer.advance(false); continue; }
                return result;
            }
            838 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_SLASH); lexer.mark_end();
                return result;
            }
            839 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_SLASH); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            840 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_POUND); lexer.mark_end();
                return result;
            }
            841 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_POUND); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            842 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_PERCENT); lexer.mark_end();
                return result;
            }
            843 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_PERCENT); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            844 => {
                result = true; lexer.set_result_symbol(anon_sym_COMMA_COMMA); lexer.mark_end();
                return result;
            }
            845 => {
                result = true; lexer.set_result_symbol(anon_sym_COMMA_COMMA); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            846 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET_CARET); lexer.mark_end();
                return result;
            }
            847 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET_CARET); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            848 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR_LPAREN); lexer.mark_end();
                return result;
            }
            849 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR_LPAREN); lexer.mark_end();
                if lookahead == 40 { state = 767; lexer.advance(false); continue; }
                return result;
            }
            850 => {
                result = true; lexer.set_result_symbol(anon_sym_BQUOTE); lexer.mark_end();
                return result;
            }
            851 => {
                result = true; lexer.set_result_symbol(anon_sym_BQUOTE); lexer.mark_end();
                if lookahead == 96 { state = 774; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 504; lexer.advance(false); continue; }
                return result;
            }
            852 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR_BQUOTE); lexer.mark_end();
                return result;
            }
            853 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LPAREN); lexer.mark_end();
                return result;
            }
            854 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_LPAREN); lexer.mark_end();
                return result;
            }
            855 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 10 { state = 792; lexer.advance(false); continue; }
                if lookahead == 13 { state = 787; lexer.advance(false); continue; }
                if lookahead != 0 { state = 791; lexer.advance(false); continue; }
                return result;
            }
            856 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 9 || 11 <= lookahead && lookahead <= 13 { state = 857; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) { state = 858; lexer.advance(false); continue; }
                return result;
            }
            857 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 { state = 857; lexer.advance(false); continue; }
                return result;
            }
            858 => {
                result = true; lexer.set_result_symbol(sym__comment_word); lexer.mark_end();
                if lookahead == 92 { state = 856; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 858; lexer.advance(false); continue; }
                return result;
            }
            859 => {
                result = true; lexer.set_result_symbol(aux_sym__simple_variable_name_token1); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if lookahead == 97 { state = 860; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 863; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            860 => {
                result = true; lexer.set_result_symbol(aux_sym__simple_variable_name_token1); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if lookahead == 99 { state = 652; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 863; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            861 => {
                result = true; lexer.set_result_symbol(aux_sym__simple_variable_name_token1); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if lookahead == 115 { state = 859; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 863; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            862 => {
                result = true; lexer.set_result_symbol(aux_sym__simple_variable_name_token1); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 800; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 863; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            863 => {
                result = true; lexer.set_result_symbol(aux_sym__simple_variable_name_token1); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 863; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            864 => {
                result = true; lexer.set_result_symbol(aux_sym__simple_variable_name_token1); lexer.mark_end();
                if lookahead == 92 { state = 17; lexer.advance(false); continue; }
                if lookahead == 97 { state = 865; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            865 => {
                result = true; lexer.set_result_symbol(aux_sym__simple_variable_name_token1); lexer.mark_end();
                if lookahead == 92 { state = 17; lexer.advance(false); continue; }
                if lookahead == 99 { state = 654; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            866 => {
                result = true; lexer.set_result_symbol(aux_sym__simple_variable_name_token1); lexer.mark_end();
                if lookahead == 92 { state = 17; lexer.advance(false); continue; }
                if lookahead == 115 { state = 864; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            867 => {
                result = true; lexer.set_result_symbol(aux_sym__simple_variable_name_token1); lexer.mark_end();
                if lookahead == 92 { state = 17; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            868 => {
                result = true; lexer.set_result_symbol(aux_sym__simple_variable_name_token1); lexer.mark_end();
                if lookahead == 92 { state = 17; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            869 => {
                result = true; lexer.set_result_symbol(aux_sym__simple_variable_name_token1); lexer.mark_end();
                if lookahead == 97 { state = 870; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 98 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            870 => {
                result = true; lexer.set_result_symbol(aux_sym__simple_variable_name_token1); lexer.mark_end();
                if lookahead == 99 { state = 655; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            871 => {
                result = true; lexer.set_result_symbol(aux_sym__simple_variable_name_token1); lexer.mark_end();
                if lookahead == 110 { state = 544; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            872 => {
                result = true; lexer.set_result_symbol(aux_sym__simple_variable_name_token1); lexer.mark_end();
                if lookahead == 115 { state = 869; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            873 => {
                result = true; lexer.set_result_symbol(aux_sym__simple_variable_name_token1); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 { state = 805; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            874 => {
                result = true; lexer.set_result_symbol(aux_sym__simple_variable_name_token1); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            875 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 683), (33, 664), (34, 783), (35, 811), (36, 776), (37, 643), (38, 595), (39, 485),
                    (40, 647), (41, 649), (42, 632), (43, 624), (45, 627), (47, 638), (48, 801), (59, 549),
                    (60, 601), (61, 554), (62, 610), (63, 755), (64, 936), (92, 14), (94, 590), (95, 938),
                    (96, 850), (124, 586),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 12; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 59 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            876 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 686), (33, 663), (34, 783), (35, 811), (36, 775), (38, 596), (42, 631), (45, 626),
                    (59, 548), (60, 606), (62, 612), (63, 754), (64, 935), (92, 100), (95, 939), (101, 872),
                    (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 202; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            877 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 687), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (40, 647),
                    (42, 634), (45, 630), (48, 801), (59, 548), (60, 603), (61, 943), (62, 611), (63, 755),
                    (64, 936), (92, 28), (95, 938), (96, 850), (101, 866), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 216; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && (lookahead < 59 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            878 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 689), (33, 663), (34, 783), (35, 811), (36, 775), (38, 596), (42, 631), (45, 626),
                    (59, 548), (60, 607), (62, 612), (63, 754), (64, 935), (92, 185), (95, 939), (101, 872),
                    (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 234; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            879 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 690), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (40, 647),
                    (42, 634), (45, 630), (48, 801), (59, 548), (60, 603), (61, 943), (62, 611), (63, 755),
                    (64, 936), (92, 30), (95, 938), (96, 850), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 235; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && (lookahead < 59 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            880 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 692), (33, 663), (34, 783), (35, 811), (36, 775), (38, 596), (42, 631), (45, 626),
                    (59, 548), (60, 606), (62, 612), (63, 754), (64, 935), (92, 104), (95, 939), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 237; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            881 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 693), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (42, 634),
                    (45, 630), (48, 801), (59, 548), (60, 603), (61, 943), (62, 611), (63, 755), (64, 936),
                    (92, 32), (95, 938), (96, 850), (101, 866), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 238; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && (lookahead < 59 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            882 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 695), (33, 663), (34, 783), (35, 811), (36, 775), (38, 596), (42, 631), (45, 626),
                    (59, 549), (60, 606), (62, 612), (63, 754), (64, 935), (92, 106), (95, 939), (96, 850),
                    (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 240; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            883 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 696), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (42, 634),
                    (45, 630), (48, 801), (59, 548), (60, 603), (61, 943), (62, 611), (63, 755), (64, 936),
                    (92, 36), (95, 938), (96, 850), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 241; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && (lookahead < 59 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            884 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 698), (33, 663), (34, 783), (35, 811), (36, 775), (38, 596), (42, 631), (45, 626),
                    (59, 548), (60, 607), (62, 612), (63, 754), (64, 935), (92, 188), (95, 939), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 243; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            885 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 699), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (40, 647),
                    (42, 634), (45, 630), (48, 801), (59, 549), (60, 603), (61, 943), (62, 611), (63, 755),
                    (64, 936), (92, 164), (95, 938), (96, 850), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 244; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && (lookahead < 59 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            886 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 701), (33, 663), (34, 783), (35, 811), (36, 775), (38, 596), (42, 631), (45, 626),
                    (59, 549), (60, 606), (62, 612), (63, 754), (64, 935), (92, 211), (95, 939), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 246; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            887 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 702), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (40, 647),
                    (41, 649), (42, 634), (45, 630), (48, 801), (59, 549), (60, 603), (61, 943), (62, 611),
                    (63, 755), (64, 936), (92, 39), (95, 938), (96, 850), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 247; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 59 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            888 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 704), (33, 663), (34, 783), (35, 811), (36, 775), (38, 596), (41, 649), (42, 631),
                    (45, 626), (59, 549), (60, 606), (62, 612), (63, 754), (64, 935), (92, 109), (95, 939),
                    (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 249; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            889 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 705), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (42, 634),
                    (45, 630), (48, 801), (59, 549), (60, 603), (61, 943), (62, 611), (63, 755), (64, 936),
                    (92, 168), (95, 938), (96, 850), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 250; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && (lookahead < 59 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            890 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 707), (33, 663), (34, 783), (35, 811), (36, 775), (38, 596), (41, 649), (42, 631),
                    (45, 626), (59, 549), (60, 607), (62, 612), (63, 754), (64, 935), (92, 192), (95, 939),
                    (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 252; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            891 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 708), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (41, 649),
                    (42, 634), (45, 630), (48, 801), (59, 549), (60, 603), (61, 943), (62, 611), (63, 755),
                    (64, 936), (92, 41), (95, 938), (96, 850), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 253; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && (lookahead < 59 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            892 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 710), (33, 663), (34, 783), (35, 811), (36, 775), (38, 596), (42, 631), (45, 626),
                    (59, 549), (60, 607), (62, 612), (63, 754), (64, 935), (92, 215), (95, 939), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 255; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            893 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 711), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (42, 634),
                    (45, 630), (48, 801), (59, 548), (60, 603), (62, 611), (63, 755), (64, 936), (92, 47),
                    (95, 938), (96, 850), (101, 866), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 256; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && (lookahead < 62 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            894 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 713), (33, 663), (34, 783), (35, 811), (36, 775), (38, 596), (42, 631), (45, 626),
                    (59, 549), (60, 607), (62, 612), (63, 754), (64, 935), (92, 190), (95, 939), (96, 850),
                    (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 258; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            895 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 714), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (42, 634),
                    (45, 630), (48, 801), (59, 548), (60, 603), (62, 611), (63, 755), (64, 936), (92, 49),
                    (95, 938), (96, 850), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 259; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && (lookahead < 62 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            896 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 716), (33, 663), (34, 783), (35, 811), (36, 775), (38, 483), (42, 631), (45, 626),
                    (60, 606), (62, 612), (63, 754), (64, 935), (92, 120), (95, 939), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 261; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            897 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 717), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (42, 634),
                    (45, 630), (48, 801), (59, 548), (60, 604), (62, 611), (63, 755), (64, 936), (92, 221),
                    (95, 938), (96, 850), (101, 866), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 262; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && (lookahead < 62 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            898 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 719), (33, 663), (34, 783), (35, 811), (36, 775), (38, 483), (42, 631), (45, 626),
                    (60, 607), (62, 612), (63, 754), (64, 935), (92, 197), (95, 939), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 264; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            899 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 720), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (42, 634),
                    (45, 630), (48, 801), (59, 549), (60, 603), (62, 611), (63, 755), (64, 936), (92, 174),
                    (95, 938), (96, 850), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 265; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && (lookahead < 62 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            900 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 722), (33, 663), (34, 783), (35, 811), (36, 775), (38, 483), (42, 631), (45, 626),
                    (60, 605), (62, 612), (63, 754), (64, 935), (92, 213), (95, 939), (124, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 267; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            901 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 723), (33, 666), (34, 783), (35, 811), (36, 776), (38, 483), (39, 485), (40, 647),
                    (42, 634), (45, 630), (48, 801), (60, 603), (61, 943), (62, 611), (63, 755), (64, 936),
                    (92, 52), (95, 938), (96, 850), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 268; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && (lookahead < 59 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            902 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 725), (33, 663), (34, 783), (35, 811), (36, 775), (38, 593), (42, 631), (45, 626),
                    (59, 549), (63, 754), (64, 935), (92, 140), (95, 939), (105, 871),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 270; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            903 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 726), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (42, 634),
                    (45, 630), (48, 801), (59, 548), (60, 604), (62, 611), (63, 755), (64, 936), (92, 223),
                    (95, 938), (96, 850), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 271; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && (lookahead < 62 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            904 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 728), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (41, 649),
                    (42, 634), (45, 630), (48, 801), (59, 549), (60, 603), (62, 611), (63, 755), (64, 936),
                    (92, 54), (95, 938), (96, 850), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 273; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && (lookahead < 62 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            905 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 730), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (42, 634),
                    (45, 630), (48, 801), (59, 549), (60, 604), (62, 611), (63, 755), (64, 936), (92, 207),
                    (95, 938), (96, 850), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 275; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && (lookahead < 62 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            906 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 732), (33, 666), (34, 783), (35, 811), (36, 776), (38, 596), (39, 485), (41, 649),
                    (42, 634), (45, 630), (48, 801), (59, 549), (60, 604), (62, 611), (63, 755), (64, 936),
                    (92, 225), (95, 938), (96, 850), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 277; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && (lookahead < 62 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            907 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 734), (33, 666), (34, 783), (35, 811), (36, 776), (38, 483), (39, 485), (42, 634),
                    (45, 630), (48, 801), (60, 603), (61, 943), (62, 611), (63, 755), (64, 936), (92, 59),
                    (95, 938), (96, 850), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 279; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && (lookahead < 59 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            908 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 736), (33, 666), (34, 783), (35, 811), (36, 776), (38, 483), (39, 485), (42, 634),
                    (45, 630), (48, 801), (60, 603), (62, 611), (63, 755), (64, 936), (92, 68), (95, 938),
                    (96, 850), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 281; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && lookahead != 59 && lookahead != 60 && (lookahead < 62 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            909 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 738), (33, 666), (34, 783), (35, 811), (36, 776), (38, 483), (39, 485), (42, 634),
                    (45, 630), (48, 801), (60, 604), (62, 611), (63, 755), (64, 936), (92, 230), (95, 938),
                    (96, 850), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 283; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && lookahead != 59 && lookahead != 60 && (lookahead < 62 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            910 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 740), (33, 666), (34, 783), (35, 811), (36, 776), (38, 483), (39, 485), (42, 634),
                    (45, 630), (48, 801), (60, 602), (62, 611), (63, 755), (64, 936), (92, 180), (95, 938),
                    (96, 850), (124, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 285; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && lookahead != 59 && lookahead != 60 && (lookahead < 62 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            911 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 742), (33, 666), (34, 783), (35, 811), (36, 776), (38, 593), (39, 485), (42, 634),
                    (45, 630), (48, 801), (59, 549), (60, 489), (62, 490), (63, 755), (64, 936), (92, 115),
                    (95, 938), (96, 850),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 287; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && (lookahead < 62 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            912 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (10, 744), (33, 666), (34, 783), (35, 811), (36, 776), (39, 485), (42, 634), (45, 630),
                    (48, 801), (60, 489), (62, 490), (63, 755), (64, 936), (92, 127), (95, 938), (96, 850),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 289; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && lookahead != 59 && lookahead != 60 && (lookahead < 62 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            913 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (33, 664), (34, 783), (35, 811), (36, 776), (37, 643), (38, 595), (39, 485), (40, 647),
                    (42, 632), (43, 624), (45, 627), (47, 638), (48, 801), (60, 601), (61, 554), (62, 610),
                    (63, 755), (64, 936), (92, 19), (93, 669), (94, 590), (95, 938), (96, 850), (124, 586),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 398; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 32 || 43 < lookahead) && (lookahead < 59 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            914 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (33, 666), (34, 783), (35, 811), (36, 776), (38, 502), (39, 485), (42, 634), (45, 630),
                    (48, 801), (60, 602), (62, 611), (63, 755), (64, 936), (92, 87), (95, 938), (96, 850),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 404; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && lookahead != 59 && lookahead != 60 && (lookahead < 62 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            915 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (33, 666), (34, 783), (35, 811), (36, 776), (38, 483), (39, 485), (40, 647), (42, 634),
                    (45, 630), (48, 801), (60, 603), (61, 943), (62, 611), (63, 755), (64, 936), (92, 57),
                    (95, 938), (96, 850), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 405; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && (lookahead < 59 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            916 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (33, 666), (34, 783), (35, 811), (36, 776), (38, 483), (39, 485), (42, 634), (45, 630),
                    (48, 801), (60, 603), (61, 943), (62, 611), (63, 755), (64, 936), (92, 62), (95, 938),
                    (96, 850), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 406; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && (lookahead < 59 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            917 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (33, 666), (34, 783), (35, 811), (36, 776), (38, 483), (39, 485), (42, 634), (45, 630),
                    (48, 801), (60, 603), (62, 611), (63, 755), (64, 936), (92, 65), (93, 669), (95, 938),
                    (96, 850), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 407; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && lookahead != 59 && lookahead != 60 && (lookahead < 62 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            918 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (33, 666), (34, 783), (35, 811), (36, 776), (38, 483), (39, 485), (42, 634), (45, 630),
                    (48, 801), (60, 603), (62, 611), (63, 755), (64, 936), (92, 76), (95, 938), (96, 850),
                    (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 408; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && lookahead != 59 && lookahead != 60 && (lookahead < 62 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            919 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (33, 666), (34, 783), (35, 811), (36, 776), (38, 483), (39, 485), (42, 634), (45, 630),
                    (48, 801), (60, 604), (62, 611), (63, 755), (64, 936), (92, 228), (93, 669), (95, 938),
                    (96, 850), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 409; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && lookahead != 59 && lookahead != 60 && (lookahead < 62 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            920 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (33, 666), (34, 783), (35, 811), (36, 776), (38, 483), (39, 485), (42, 634), (45, 630),
                    (48, 801), (60, 604), (62, 611), (63, 755), (64, 936), (92, 232), (95, 938), (96, 850),
                    (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 410; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && lookahead != 59 && lookahead != 60 && (lookahead < 62 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            921 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (33, 666), (34, 783), (35, 811), (36, 776), (39, 485), (41, 649), (42, 634), (45, 630),
                    (48, 801), (60, 489), (62, 490), (63, 755), (64, 936), (92, 124), (95, 938), (96, 850),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 411; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 802; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 38 || 42 < lookahead) && lookahead != 59 && lookahead != 60 && (lookahead < 62 || 93 < lookahead) && (lookahead < 95 || 125 < lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            922 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (33, 665), (34, 783), (35, 811), (36, 775), (37, 644), (38, 594), (41, 649), (42, 633),
                    (43, 625), (45, 628), (47, 639), (60, 609), (61, 556), (62, 614), (63, 754), (64, 935),
                    (92, 78), (94, 591), (95, 939), (124, 588),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 422; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            923 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (33, 665), (34, 783), (35, 811), (36, 775), (37, 644), (38, 594), (42, 633), (43, 625),
                    (45, 628), (47, 639), (58, 756), (60, 609), (61, 556), (62, 614), (63, 754), (64, 935),
                    (92, 72), (94, 591), (95, 939), (124, 588),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 423; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            924 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (33, 665), (34, 783), (35, 811), (36, 775), (37, 644), (38, 594), (42, 633), (43, 625),
                    (45, 628), (47, 639), (60, 609), (61, 556), (62, 614), (63, 754), (64, 935), (92, 70),
                    (93, 669), (94, 591), (95, 939), (124, 588),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 424; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            925 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (33, 665), (34, 783), (35, 811), (36, 775), (37, 644), (38, 594), (42, 633), (43, 625),
                    (45, 628), (47, 639), (60, 609), (61, 556), (62, 614), (63, 754), (64, 935), (92, 182),
                    (93, 503), (94, 591), (95, 939), (124, 588),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 425; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            926 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (33, 665), (34, 783), (35, 811), (36, 775), (37, 644), (38, 594), (42, 633), (43, 625),
                    (45, 628), (47, 639), (60, 609), (61, 556), (62, 614), (63, 754), (64, 935), (92, 82),
                    (94, 591), (95, 939), (124, 588),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 426; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            927 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (33, 663), (34, 783), (35, 811), (36, 775), (38, 483), (42, 631), (45, 626), (60, 606),
                    (62, 612), (63, 754), (64, 935), (92, 118), (93, 669), (95, 939), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 438; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            928 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (33, 663), (34, 783), (35, 811), (36, 775), (38, 483), (42, 631), (45, 626), (60, 606),
                    (62, 612), (63, 754), (64, 935), (92, 122), (95, 939), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 439; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            929 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (33, 663), (34, 783), (35, 811), (36, 775), (38, 483), (42, 631), (45, 626), (60, 607),
                    (62, 612), (63, 754), (64, 935), (92, 195), (93, 669), (95, 939), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 440; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            930 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (33, 663), (34, 783), (35, 811), (36, 775), (38, 483), (42, 631), (45, 626), (60, 607),
                    (62, 612), (63, 754), (64, 935), (92, 199), (95, 939), (124, 587),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 441; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            931 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (33, 663), (34, 783), (35, 811), (36, 775), (41, 649), (42, 631), (45, 626), (63, 754),
                    (64, 935), (92, 142), (95, 939), (124, 584),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 442; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            932 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (33, 663), (34, 783), (35, 811), (36, 775), (42, 631), (45, 626), (63, 754), (64, 935),
                    (92, 146), (95, 939),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 444; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            933 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (33, 663), (35, 811), (36, 775), (42, 631), (45, 626), (63, 754), (64, 935), (92, 150),
                    (95, 939),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 447; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            934 => {
                result = true; lexer.set_result_symbol(aux_sym__multiline_variable_name_token1); lexer.mark_end();
                if lookahead == 92 { state = 16; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 934; lexer.advance(false); continue; }
                return result;
            }
            935 => {
                result = true; lexer.set_result_symbol(anon_sym_AT2); lexer.mark_end();
                return result;
            }
            936 => {
                result = true; lexer.set_result_symbol(anon_sym_AT2); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            937 => {
                result = true; lexer.set_result_symbol(anon_sym__); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            938 => {
                result = true; lexer.set_result_symbol(anon_sym__); lexer.mark_end();
                if lookahead == 92 { state = 17; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 868; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            939 => {
                result = true; lexer.set_result_symbol(anon_sym__); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            940 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 46 { state = 772; lexer.advance(false); continue; }
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            941 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 48 { state = 797; lexer.advance(false); continue; }
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            942 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 61 { state = 600; lexer.advance(false); continue; }
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            943 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 61 { state = 598; lexer.advance(false); continue; }
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if lookahead == 126 { state = 673; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            944 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if lookahead == 97 { state = 945; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            945 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if lookahead == 99 { state = 653; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            946 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if lookahead == 110 { state = 543; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            947 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if lookahead == 115 { state = 944; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            948 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 799; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
                return result;
            }
            949 => {
                result = true; lexer.set_result_symbol(sym_word); lexer.mark_end();
                if lookahead == 92 { state = 512; lexer.advance(false); continue; }
                if !eof && set_contains(&sym__comment_word_character_set_1, lookahead) { state = 949; lexer.advance(false); continue; }
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
                if lookahead == 65 { state = 1; lexer.advance(false); continue; }
                if lookahead == 69 { state = 2; lexer.advance(false); continue; }
                if lookahead == 75 { state = 3; lexer.advance(false); continue; }
                if lookahead == 76 { state = 4; lexer.advance(false); continue; }
                if lookahead == 80 { state = 5; lexer.advance(false); continue; }
                if lookahead == 81 { state = 6; lexer.advance(false); continue; }
                if lookahead == 85 { state = 7; lexer.advance(false); continue; }
                if lookahead == 92 { state = 8; lexer.advance(true); continue; }
                if lookahead == 97 { state = 9; lexer.advance(false); continue; }
                if lookahead == 99 { state = 10; lexer.advance(false); continue; }
                if lookahead == 100 { state = 11; lexer.advance(false); continue; }
                if lookahead == 101 { state = 12; lexer.advance(false); continue; }
                if lookahead == 102 { state = 13; lexer.advance(false); continue; }
                if lookahead == 105 { state = 14; lexer.advance(false); continue; }
                if lookahead == 107 { state = 15; lexer.advance(false); continue; }
                if lookahead == 108 { state = 16; lexer.advance(false); continue; }
                if lookahead == 114 { state = 17; lexer.advance(false); continue; }
                if lookahead == 115 { state = 18; lexer.advance(false); continue; }
                if lookahead == 116 { state = 19; lexer.advance(false); continue; }
                if lookahead == 117 { state = 20; lexer.advance(false); continue; }
                if lookahead == 119 { state = 21; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 22; lexer.advance(true); continue; }
                return result;
            }
            1 => {
                result = true; lexer.set_result_symbol(anon_sym_A); lexer.mark_end();
                return result;
            }
            2 => {
                result = true; lexer.set_result_symbol(anon_sym_E); lexer.mark_end();
                return result;
            }
            3 => {
                result = true; lexer.set_result_symbol(anon_sym_K); lexer.mark_end();
                return result;
            }
            4 => {
                result = true; lexer.set_result_symbol(anon_sym_L); lexer.mark_end();
                return result;
            }
            5 => {
                result = true; lexer.set_result_symbol(anon_sym_P); lexer.mark_end();
                return result;
            }
            6 => {
                result = true; lexer.set_result_symbol(anon_sym_Q); lexer.mark_end();
                return result;
            }
            7 => {
                result = true; lexer.set_result_symbol(anon_sym_U); lexer.mark_end();
                return result;
            }
            8 => {
                if lookahead == 13 { state = 23; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 22; lexer.advance(true); continue; }
                return result;
            }
            9 => {
                result = true; lexer.set_result_symbol(anon_sym_a); lexer.mark_end();
                return result;
            }
            10 => {
                if lookahead == 97 { state = 24; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if lookahead == 101 { state = 25; lexer.advance(false); continue; }
                if lookahead == 111 { state = 26; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if lookahead == 108 { state = 27; lexer.advance(false); continue; }
                if lookahead == 120 { state = 28; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if lookahead == 105 { state = 29; lexer.advance(false); continue; }
                if lookahead == 111 { state = 30; lexer.advance(false); continue; }
                if lookahead == 117 { state = 31; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if lookahead == 102 { state = 32; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                result = true; lexer.set_result_symbol(anon_sym_k); lexer.mark_end();
                return result;
            }
            16 => {
                if lookahead == 111 { state = 33; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if lookahead == 101 { state = 34; lexer.advance(false); continue; }
                return result;
            }
            18 => {
                if lookahead == 101 { state = 35; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if lookahead == 104 { state = 36; lexer.advance(false); continue; }
                if lookahead == 121 { state = 37; lexer.advance(false); continue; }
                return result;
            }
            20 => {
                result = true; lexer.set_result_symbol(anon_sym_u); lexer.mark_end();
                if lookahead == 110 { state = 38; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                if lookahead == 104 { state = 39; lexer.advance(false); continue; }
                return result;
            }
            22 => {
                if lookahead == 92 { state = 8; lexer.advance(true); continue; }
                if lookahead == 99 { state = 10; lexer.advance(false); continue; }
                if lookahead == 100 { state = 11; lexer.advance(false); continue; }
                if lookahead == 101 { state = 12; lexer.advance(false); continue; }
                if lookahead == 102 { state = 13; lexer.advance(false); continue; }
                if lookahead == 105 { state = 14; lexer.advance(false); continue; }
                if lookahead == 108 { state = 16; lexer.advance(false); continue; }
                if lookahead == 114 { state = 17; lexer.advance(false); continue; }
                if lookahead == 115 { state = 18; lexer.advance(false); continue; }
                if lookahead == 116 { state = 19; lexer.advance(false); continue; }
                if lookahead == 117 { state = 40; lexer.advance(false); continue; }
                if lookahead == 119 { state = 21; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 22; lexer.advance(true); continue; }
                return result;
            }
            23 => {
                if lookahead == 10 { state = 22; lexer.advance(true); continue; }
                return result;
            }
            24 => {
                if lookahead == 115 { state = 41; lexer.advance(false); continue; }
                return result;
            }
            25 => {
                if lookahead == 99 { state = 42; lexer.advance(false); continue; }
                return result;
            }
            26 => {
                result = true; lexer.set_result_symbol(anon_sym_do); lexer.mark_end();
                if lookahead == 110 { state = 43; lexer.advance(false); continue; }
                return result;
            }
            27 => {
                if lookahead == 105 { state = 44; lexer.advance(false); continue; }
                if lookahead == 115 { state = 45; lexer.advance(false); continue; }
                return result;
            }
            28 => {
                if lookahead == 112 { state = 46; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                result = true; lexer.set_result_symbol(anon_sym_fi); lexer.mark_end();
                return result;
            }
            30 => {
                if lookahead == 114 { state = 47; lexer.advance(false); continue; }
                return result;
            }
            31 => {
                if lookahead == 110 { state = 48; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                result = true; lexer.set_result_symbol(anon_sym_if); lexer.mark_end();
                return result;
            }
            33 => {
                if lookahead == 99 { state = 49; lexer.advance(false); continue; }
                return result;
            }
            34 => {
                if lookahead == 97 { state = 50; lexer.advance(false); continue; }
                return result;
            }
            35 => {
                if lookahead == 108 { state = 51; lexer.advance(false); continue; }
                return result;
            }
            36 => {
                if lookahead == 101 { state = 52; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                if lookahead == 112 { state = 53; lexer.advance(false); continue; }
                return result;
            }
            38 => {
                if lookahead == 115 { state = 54; lexer.advance(false); continue; }
                if lookahead == 116 { state = 55; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                if lookahead == 105 { state = 56; lexer.advance(false); continue; }
                return result;
            }
            40 => {
                if lookahead == 110 { state = 38; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                if lookahead == 101 { state = 57; lexer.advance(false); continue; }
                return result;
            }
            42 => {
                if lookahead == 108 { state = 58; lexer.advance(false); continue; }
                return result;
            }
            43 => {
                if lookahead == 101 { state = 59; lexer.advance(false); continue; }
                return result;
            }
            44 => {
                if lookahead == 102 { state = 60; lexer.advance(false); continue; }
                return result;
            }
            45 => {
                if lookahead == 101 { state = 61; lexer.advance(false); continue; }
                return result;
            }
            46 => {
                if lookahead == 111 { state = 62; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                result = true; lexer.set_result_symbol(anon_sym_for); lexer.mark_end();
                return result;
            }
            48 => {
                if lookahead == 99 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                if lookahead == 97 { state = 64; lexer.advance(false); continue; }
                return result;
            }
            50 => {
                if lookahead == 100 { state = 65; lexer.advance(false); continue; }
                return result;
            }
            51 => {
                if lookahead == 101 { state = 66; lexer.advance(false); continue; }
                return result;
            }
            52 => {
                if lookahead == 110 { state = 67; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                if lookahead == 101 { state = 68; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                if lookahead == 101 { state = 69; lexer.advance(false); continue; }
                return result;
            }
            55 => {
                if lookahead == 105 { state = 70; lexer.advance(false); continue; }
                return result;
            }
            56 => {
                if lookahead == 108 { state = 71; lexer.advance(false); continue; }
                return result;
            }
            57 => {
                result = true; lexer.set_result_symbol(anon_sym_case); lexer.mark_end();
                return result;
            }
            58 => {
                if lookahead == 97 { state = 72; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                result = true; lexer.set_result_symbol(anon_sym_done); lexer.mark_end();
                return result;
            }
            60 => {
                result = true; lexer.set_result_symbol(anon_sym_elif); lexer.mark_end();
                return result;
            }
            61 => {
                result = true; lexer.set_result_symbol(anon_sym_else); lexer.mark_end();
                return result;
            }
            62 => {
                if lookahead == 114 { state = 73; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                if lookahead == 116 { state = 74; lexer.advance(false); continue; }
                return result;
            }
            64 => {
                if lookahead == 108 { state = 75; lexer.advance(false); continue; }
                return result;
            }
            65 => {
                if lookahead == 111 { state = 76; lexer.advance(false); continue; }
                return result;
            }
            66 => {
                if lookahead == 99 { state = 77; lexer.advance(false); continue; }
                return result;
            }
            67 => {
                result = true; lexer.set_result_symbol(anon_sym_then); lexer.mark_end();
                return result;
            }
            68 => {
                if lookahead == 115 { state = 78; lexer.advance(false); continue; }
                return result;
            }
            69 => {
                if lookahead == 116 { state = 79; lexer.advance(false); continue; }
                return result;
            }
            70 => {
                if lookahead == 108 { state = 80; lexer.advance(false); continue; }
                return result;
            }
            71 => {
                if lookahead == 101 { state = 81; lexer.advance(false); continue; }
                return result;
            }
            72 => {
                if lookahead == 114 { state = 82; lexer.advance(false); continue; }
                return result;
            }
            73 => {
                if lookahead == 116 { state = 83; lexer.advance(false); continue; }
                return result;
            }
            74 => {
                if lookahead == 105 { state = 84; lexer.advance(false); continue; }
                return result;
            }
            75 => {
                result = true; lexer.set_result_symbol(anon_sym_local); lexer.mark_end();
                return result;
            }
            76 => {
                if lookahead == 110 { state = 85; lexer.advance(false); continue; }
                return result;
            }
            77 => {
                if lookahead == 116 { state = 86; lexer.advance(false); continue; }
                return result;
            }
            78 => {
                if lookahead == 101 { state = 87; lexer.advance(false); continue; }
                return result;
            }
            79 => {
                result = true; lexer.set_result_symbol(anon_sym_unset); lexer.mark_end();
                if lookahead == 101 { state = 88; lexer.advance(false); continue; }
                return result;
            }
            80 => {
                result = true; lexer.set_result_symbol(anon_sym_until); lexer.mark_end();
                return result;
            }
            81 => {
                result = true; lexer.set_result_symbol(anon_sym_while); lexer.mark_end();
                return result;
            }
            82 => {
                if lookahead == 101 { state = 89; lexer.advance(false); continue; }
                return result;
            }
            83 => {
                result = true; lexer.set_result_symbol(anon_sym_export); lexer.mark_end();
                return result;
            }
            84 => {
                if lookahead == 111 { state = 90; lexer.advance(false); continue; }
                return result;
            }
            85 => {
                if lookahead == 108 { state = 91; lexer.advance(false); continue; }
                return result;
            }
            86 => {
                result = true; lexer.set_result_symbol(anon_sym_select); lexer.mark_end();
                return result;
            }
            87 => {
                if lookahead == 116 { state = 92; lexer.advance(false); continue; }
                return result;
            }
            88 => {
                if lookahead == 110 { state = 93; lexer.advance(false); continue; }
                return result;
            }
            89 => {
                result = true; lexer.set_result_symbol(anon_sym_declare); lexer.mark_end();
                return result;
            }
            90 => {
                if lookahead == 110 { state = 94; lexer.advance(false); continue; }
                return result;
            }
            91 => {
                if lookahead == 121 { state = 95; lexer.advance(false); continue; }
                return result;
            }
            92 => {
                result = true; lexer.set_result_symbol(anon_sym_typeset); lexer.mark_end();
                return result;
            }
            93 => {
                if lookahead == 118 { state = 96; lexer.advance(false); continue; }
                return result;
            }
            94 => {
                result = true; lexer.set_result_symbol(anon_sym_function); lexer.mark_end();
                return result;
            }
            95 => {
                result = true; lexer.set_result_symbol(anon_sym_readonly); lexer.mark_end();
                return result;
            }
            96 => {
                result = true; lexer.set_result_symbol(anon_sym_unsetenv); lexer.mark_end();
                return result;
            }
            _ => return false,
        }
    }
}
