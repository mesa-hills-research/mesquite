//! The `verilog` grammar's lexer: `ts_lex` and `ts_lex_keywords`, transliterated from
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

const anon_sym_0: Symbol = 84;
const anon_sym_01: Symbol = 373;
const anon_sym_1: Symbol = 85;
const anon_sym_10: Symbol = 374;
const anon_sym_1_SQUOTEB0: Symbol = 293;
const anon_sym_1_SQUOTEB1: Symbol = 294;
const anon_sym_1_SQUOTEBX: Symbol = 296;
const anon_sym_1_SQUOTEBx: Symbol = 295;
const anon_sym_1_SQUOTEb0: Symbol = 289;
const anon_sym_1_SQUOTEb1: Symbol = 290;
const anon_sym_1_SQUOTEbX: Symbol = 292;
const anon_sym_1_SQUOTEbx: Symbol = 291;
const anon_sym_1step: Symbol = 168;
const anon_sym_2: Symbol = 86;
const anon_sym_AMP: Symbol = 394;
const anon_sym_AMP_AMP: Symbol = 253;
const anon_sym_AMP_AMP_AMP: Symbol = 338;
const anon_sym_AMP_EQ: Symbol = 312;
const anon_sym_AT: Symbol = 327;
const anon_sym_AT_AT: Symbol = 241;
const anon_sym_AT_STAR: Symbol = 328;
const anon_sym_BANG: Symbol = 252;
const anon_sym_BANG_EQ: Symbol = 183;
const anon_sym_BANG_EQ_EQ: Symbol = 379;
const anon_sym_BANG_EQ_QMARK: Symbol = 393;
const anon_sym_BQUOTE: Symbol = 16;
const anon_sym_BSLASH: Symbol = 431;
const anon_sym_CARET: Symbol = 396;
const anon_sym_CARET_EQ: Symbol = 314;
const anon_sym_CARET_TILDE: Symbol = 397;
const anon_sym_COLON: Symbol = 51;
const anon_sym_COLON_COLON: Symbol = 122;
const anon_sym_COLON_EQ: Symbol = 115;
const anon_sym_COLON_SLASH: Symbol = 116;
const anon_sym_COMMA: Symbol = 12;
const anon_sym_DASH: Symbol = 297;
const anon_sym_DASH_COLON: Symbol = 387;
const anon_sym_DASH_DASH: Symbol = 415;
const anon_sym_DASH_EQ: Symbol = 308;
const anon_sym_DASH_GT: Symbol = 334;
const anon_sym_DASH_GT_GT: Symbol = 335;
const anon_sym_DOLLAR: Symbol = 170;
const anon_sym_DOLLARerror: Symbol = 81;
const anon_sym_DOLLARfatal: Symbol = 80;
const anon_sym_DOLLARfullskew: Symbol = 369;
const anon_sym_DOLLARhold: Symbol = 362;
const anon_sym_DOLLARinfo: Symbol = 83;
const anon_sym_DOLLARnochange: Symbol = 372;
const anon_sym_DOLLARperiod: Symbol = 370;
const anon_sym_DOLLARrecovery: Symbol = 364;
const anon_sym_DOLLARrecrem: Symbol = 366;
const anon_sym_DOLLARremoval: Symbol = 365;
const anon_sym_DOLLARroot: Symbol = 433;
const anon_sym_DOLLARsetup: Symbol = 361;
const anon_sym_DOLLARsetuphold: Symbol = 363;
const anon_sym_DOLLARskew: Symbol = 367;
const anon_sym_DOLLARtimeskew: Symbol = 368;
const anon_sym_DOLLARunit: Symbol = 434;
const anon_sym_DOLLARwarning: Symbol = 82;
const anon_sym_DOLLARwidth: Symbol = 371;
const anon_sym_DOT: Symbol = 73;
const anon_sym_DOT_STAR: Symbol = 48;
const anon_sym_DQUOTE: Symbol = 2;
const anon_sym_DQUOTEDPI_DASHC_DQUOTE: Symbol = 172;
const anon_sym_DQUOTEDPI_DQUOTE: Symbol = 173;
const anon_sym_EQ: Symbol = 13;
const anon_sym_EQ_EQ: Symbol = 182;
const anon_sym_EQ_EQ_EQ: Symbol = 378;
const anon_sym_EQ_EQ_QMARK: Symbol = 392;
const anon_sym_EQ_GT: Symbol = 249;
const anon_sym_GT: Symbol = 6;
const anon_sym_GT_EQ: Symbol = 185;
const anon_sym_GT_GT: Symbol = 384;
const anon_sym_GT_GT_EQ: Symbol = 316;
const anon_sym_GT_GT_GT: Symbol = 399;
const anon_sym_GT_GT_GT_EQ: Symbol = 318;
const anon_sym_LBRACE: Symbol = 74;
const anon_sym_LBRACK: Symbol = 112;
const anon_sym_LBRACK_DASH_GT: Symbol = 234;
const anon_sym_LBRACK_EQ: Symbol = 233;
const anon_sym_LBRACK_PLUS_RBRACK: Symbol = 232;
const anon_sym_LBRACK_STAR: Symbol = 230;
const anon_sym_LBRACK_STAR_RBRACK: Symbol = 231;
const anon_sym_LBRACKu2013_GT: Symbol = 250;
const anon_sym_LF: Symbol = 15;
const anon_sym_LPAREN: Symbol = 10;
const anon_sym_LPAREN_STAR: Symbol = 427;
const anon_sym_LT: Symbol = 4;
const anon_sym_LT_DASH_GT: Symbol = 401;
const anon_sym_LT_EQ: Symbol = 184;
const anon_sym_LT_LT: Symbol = 385;
const anon_sym_LT_LT_EQ: Symbol = 315;
const anon_sym_LT_LT_LT: Symbol = 400;
const anon_sym_LT_LT_LT_EQ: Symbol = 317;
const anon_sym_PATHPULSE_DOLLAR_EQ: Symbol = 169;
const anon_sym_PERCENT: Symbol = 181;
const anon_sym_PERCENT_EQ: Symbol = 311;
const anon_sym_PIPE: Symbol = 395;
const anon_sym_PIPE_DASH_GT: Symbol = 200;
const anon_sym_PIPE_EQ: Symbol = 313;
const anon_sym_PIPE_EQ_GT: Symbol = 201;
const anon_sym_PIPE_PIPE: Symbol = 254;
const anon_sym_PLUS: Symbol = 176;
const anon_sym_PLUS_COLON: Symbol = 386;
const anon_sym_PLUS_EQ: Symbol = 307;
const anon_sym_PLUS_PLUS: Symbol = 177;
const anon_sym_POUND: Symbol = 71;
const anon_sym_POUND0: Symbol = 347;
const anon_sym_POUND_DASH_POUND: Symbol = 204;
const anon_sym_POUND_EQ_POUND: Symbol = 205;
const anon_sym_POUND_POUND: Symbol = 227;
const anon_sym_POUND_POUND_LBRACK_PLUS_RBRACK: Symbol = 229;
const anon_sym_POUND_POUND_LBRACK_STAR_RBRACK: Symbol = 228;
const anon_sym_QMARK: Symbol = 391;
const anon_sym_RBRACE: Symbol = 75;
const anon_sym_RBRACK: Symbol = 113;
const anon_sym_RPAREN: Symbol = 11;
const anon_sym_SEMI: Symbol = 49;
const anon_sym_SLASH: Symbol = 30;
const anon_sym_SLASH_EQ: Symbol = 310;
const anon_sym_SQUOTE: Symbol = 412;
const anon_sym_SQUOTE0: Symbol = 424;
const anon_sym_SQUOTE1: Symbol = 425;
const anon_sym_SQUOTEB0: Symbol = 382;
const anon_sym_SQUOTEB1: Symbol = 383;
const anon_sym_SQUOTE_LBRACE: Symbol = 343;
const anon_sym_SQUOTEb0: Symbol = 380;
const anon_sym_SQUOTEb1: Symbol = 381;
const anon_sym_STAR: Symbol = 123;
const anon_sym_STAR_COLON_COLON_STAR: Symbol = 125;
const anon_sym_STAR_EQ: Symbol = 309;
const anon_sym_STAR_GT: Symbol = 356;
const anon_sym_STAR_RPAREN: Symbol = 428;
const anon_sym_STAR_STAR: Symbol = 180;
const anon_sym_TILDE: Symbol = 377;
const anon_sym_TILDE_AMP: Symbol = 413;
const anon_sym_TILDE_CARET: Symbol = 398;
const anon_sym_TILDE_PIPE: Symbol = 414;
const anon_sym_accept_on: Symbol = 217;
const anon_sym_alias: Symbol = 302;
const anon_sym_always: Symbol = 208;
const anon_sym_always_comb: Symbol = 303;
const anon_sym_always_ff: Symbol = 305;
const anon_sym_always_latch: Symbol = 304;
const anon_sym_and: Symbol = 199;
const anon_sym_assert: Symbol = 187;
const anon_sym_assign: Symbol = 301;
const anon_sym_assume: Symbol = 189;
const anon_sym_automatic: Symbol = 136;
const anon_sym_before: Symbol = 106;
const anon_sym_begin: Symbol = 242;
const anon_sym_bind: Symbol = 92;
const anon_sym_bins: Symbol = 246;
const anon_sym_binsof: Symbol = 256;
const anon_sym_bit: Symbol = 147;
const anon_sym_break: Symbol = 330;
const anon_sym_buf: Symbol = 274;
const anon_sym_bufif0: Symbol = 262;
const anon_sym_bufif1: Symbol = 263;
const anon_sym_byte: Symbol = 141;
const anon_sym_case: Symbol = 202;
const anon_sym_casex: Symbol = 341;
const anon_sym_casez: Symbol = 340;
const anon_sym_chandle: Symbol = 139;
const anon_sym_checker: Symbol = 59;
const anon_sym_class: Symbol = 62;
const anon_sym_clocking: Symbol = 88;
const anon_sym_cmos: Symbol = 260;
const anon_sym_const: Symbol = 95;
const anon_sym_constraint: Symbol = 104;
const anon_sym_context: Symbol = 174;
const anon_sym_continue: Symbol = 331;
const anon_sym_cover: Symbol = 190;
const anon_sym_covergroup: Symbol = 236;
const anon_sym_coverpoint: Symbol = 244;
const anon_sym_cross: Symbol = 251;
const anon_sym_deassign: Symbol = 319;
const anon_sym_default: Symbol = 87;
const anon_sym_defparam: Symbol = 91;
const anon_sym_disable: Symbol = 89;
const anon_sym_dist: Symbol = 235;
const anon_sym_do: Symbol = 346;
const anon_sym_edge: Symbol = 359;
const anon_sym_else: Symbol = 110;
const anon_sym_end: Symbol = 243;
const anon_sym_endcase: Symbol = 203;
const anon_sym_endchecker: Symbol = 60;
const anon_sym_endclass: Symbol = 65;
const anon_sym_endclocking: Symbol = 348;
const anon_sym_endfunction: Symbol = 103;
const anon_sym_endgenerate: Symbol = 282;
const anon_sym_endgroup: Symbol = 237;
const anon_sym_endinterface: Symbol = 55;
const anon_sym_endmodule: Symbol = 50;
const anon_sym_endpackage: Symbol = 68;
const anon_sym_endprimitive: Symbol = 285;
const anon_sym_endprogram: Symbol = 57;
const anon_sym_endproperty: Symbol = 194;
const anon_sym_endsequence: Symbol = 221;
const anon_sym_endspecify: Symbol = 351;
const anon_sym_endtable: Symbol = 287;
const anon_sym_endtask: Symbol = 175;
const anon_sym_enum: Symbol = 131;
const anon_sym_event: Symbol = 140;
const anon_sym_eventually: Symbol = 211;
const anon_sym_expect: Symbol = 191;
const anon_sym_export: Symbol = 124;
const anon_sym_extends: Symbol = 63;
const anon_sym_extern: Symbol = 52;
const anon_sym_final: Symbol = 306;
const anon_sym_first_match: Symbol = 224;
const anon_sym_for: Symbol = 283;
const anon_sym_force: Symbol = 320;
const anon_sym_foreach: Symbol = 111;
const anon_sym_forever: Symbol = 344;
const anon_sym_fork: Symbol = 322;
const anon_sym_forkjoin: Symbol = 93;
const anon_sym_fs: Symbol = 408;
const anon_sym_function: Symbol = 96;
const anon_sym_generate: Symbol = 281;
const anon_sym_genvar: Symbol = 126;
const anon_sym_global: Symbol = 349;
const anon_sym_highz0: Symbol = 160;
const anon_sym_highz1: Symbol = 159;
const anon_sym_if: Symbol = 109;
const anon_sym_iff: Symbol = 90;
const anon_sym_ifnone: Symbol = 360;
const anon_sym_ignore_bins: Symbol = 248;
const anon_sym_illegal_bins: Symbol = 247;
const anon_sym_implements: Symbol = 64;
const anon_sym_implies: Symbol = 216;
const anon_sym_import: Symbol = 121;
const anon_sym_initial: Symbol = 288;
const anon_sym_inout: Symbol = 78;
const anon_sym_input: Symbol = 76;
const anon_sym_inside: Symbol = 339;
const anon_sym_int: Symbol = 143;
const anon_sym_integer: Symbol = 145;
const anon_sym_interconnect: Symbol = 129;
const anon_sym_interface: Symbol = 56;
const anon_sym_intersect: Symbol = 223;
const anon_sym_join: Symbol = 323;
const anon_sym_join_any: Symbol = 324;
const anon_sym_join_none: Symbol = 325;
const anon_sym_large: Symbol = 167;
const anon_sym_let: Symbol = 257;
const anon_sym_local: Symbol = 100;
const anon_sym_localparam: Symbol = 117;
const anon_sym_logic: Symbol = 148;
const anon_sym_longint: Symbol = 144;
const anon_sym_macromodule: Symbol = 54;
const anon_sym_matches: Symbol = 255;
const anon_sym_medium: Symbol = 166;
const anon_sym_modport: Symbol = 186;
const anon_sym_module: Symbol = 53;
const anon_sym_ms: Symbol = 404;
const anon_sym_nand: Symbol = 270;
const anon_sym_negedge: Symbol = 358;
const anon_sym_nettype: Symbol = 134;
const anon_sym_new: Symbol = 97;
const anon_sym_nexttime: Symbol = 206;
const anon_sym_nmos: Symbol = 266;
const anon_sym_none: Symbol = 42;
const anon_sym_nor: Symbol = 271;
const anon_sym_noshowcancelled: Symbol = 355;
const anon_sym_not: Symbol = 197;
const anon_sym_notif0: Symbol = 264;
const anon_sym_notif1: Symbol = 265;
const anon_sym_ns: Symbol = 406;
const anon_sym_null: Symbol = 390;
const anon_sym_option: Symbol = 238;
const anon_sym_or: Symbol = 198;
const anon_sym_output: Symbol = 77;
const anon_sym_package: Symbol = 67;
const anon_sym_packed: Symbol = 138;
const anon_sym_parameter: Symbol = 118;
const anon_sym_pmos: Symbol = 267;
const anon_sym_posedge: Symbol = 357;
const anon_sym_primitive: Symbol = 284;
const anon_sym_priority: Symbol = 337;
const anon_sym_program: Symbol = 58;
const anon_sym_property: Symbol = 188;
const anon_sym_protected: Symbol = 99;
const anon_sym_ps: Symbol = 407;
const anon_sym_pull0: Symbol = 44;
const anon_sym_pull1: Symbol = 45;
const anon_sym_pulldown: Symbol = 258;
const anon_sym_pullup: Symbol = 259;
const anon_sym_pulsestyle_ondetect: Symbol = 353;
const anon_sym_pulsestyle_onevent: Symbol = 352;
const anon_sym_pure: Symbol = 66;
const anon_sym_rand: Symbol = 94;
const anon_sym_randc: Symbol = 101;
const anon_sym_randcase: Symbol = 342;
const anon_sym_randomize: Symbol = 389;
const anon_sym_rcmos: Symbol = 261;
const anon_sym_real: Symbol = 151;
const anon_sym_realtime: Symbol = 152;
const anon_sym_ref: Symbol = 79;
const anon_sym_reg: Symbol = 149;
const anon_sym_reject_on: Symbol = 218;
const anon_sym_release: Symbol = 321;
const anon_sym_repeat: Symbol = 326;
const anon_sym_restrict: Symbol = 193;
const anon_sym_return: Symbol = 329;
const anon_sym_rnmos: Symbol = 268;
const anon_sym_rpmos: Symbol = 269;
const anon_sym_rtran: Symbol = 280;
const anon_sym_rtranif0: Symbol = 278;
const anon_sym_rtranif1: Symbol = 277;
const anon_sym_s: Symbol = 403;
const anon_sym_s_always: Symbol = 209;
const anon_sym_s_eventually: Symbol = 210;
const anon_sym_s_nexttime: Symbol = 207;
const anon_sym_s_until: Symbol = 213;
const anon_sym_s_until_with: Symbol = 215;
const anon_sym_sample: Symbol = 240;
const anon_sym_scalared: Symbol = 128;
const anon_sym_sequence: Symbol = 192;
const anon_sym_shortint: Symbol = 142;
const anon_sym_shortreal: Symbol = 150;
const anon_sym_showcancelled: Symbol = 354;
const anon_sym_signed: Symbol = 155;
const anon_sym_small: Symbol = 165;
const anon_sym_soft: Symbol = 107;
const anon_sym_solve: Symbol = 105;
const anon_sym_specify: Symbol = 350;
const anon_sym_specparam: Symbol = 119;
const anon_sym_static: Symbol = 98;
const anon_sym_std: Symbol = 388;
const anon_sym_string: Symbol = 137;
const anon_sym_strong: Symbol = 195;
const anon_sym_strong0: Symbol = 161;
const anon_sym_strong1: Symbol = 163;
const anon_sym_struct: Symbol = 132;
const anon_sym_super: Symbol = 102;
const anon_sym_supply0: Symbol = 153;
const anon_sym_supply1: Symbol = 154;
const anon_sym_sync_accept_on: Symbol = 219;
const anon_sym_sync_reject_on: Symbol = 220;
const anon_sym_table: Symbol = 286;
const anon_sym_tagged: Symbol = 158;
const anon_sym_task: Symbol = 171;
const anon_sym_this: Symbol = 402;
const anon_sym_throughout: Symbol = 225;
const anon_sym_time: Symbol = 146;
const anon_sym_timeprecision: Symbol = 70;
const anon_sym_timeunit: Symbol = 69;
const anon_sym_tran: Symbol = 279;
const anon_sym_tranif0: Symbol = 275;
const anon_sym_tranif1: Symbol = 276;
const anon_sym_tri: Symbol = 33;
const anon_sym_tri0: Symbol = 34;
const anon_sym_tri1: Symbol = 35;
const anon_sym_triand: Symbol = 37;
const anon_sym_trior: Symbol = 39;
const anon_sym_trireg: Symbol = 40;
const anon_sym_type: Symbol = 72;
const anon_sym_type_option: Symbol = 239;
const anon_sym_typedef: Symbol = 130;
const anon_sym_u2013: Symbol = 178;
const anon_sym_u2013_GT: Symbol = 108;
const anon_sym_u2013u2013: Symbol = 179;
const anon_sym_union: Symbol = 133;
const anon_sym_unique: Symbol = 114;
const anon_sym_unique0: Symbol = 336;
const anon_sym_unsigned: Symbol = 156;
const anon_sym_until: Symbol = 212;
const anon_sym_until_with: Symbol = 214;
const anon_sym_untyped: Symbol = 222;
const anon_sym_us: Symbol = 405;
const anon_sym_uwire: Symbol = 41;
const anon_sym_var: Symbol = 120;
const anon_sym_vectored: Symbol = 127;
const anon_sym_virtual: Symbol = 61;
const anon_sym_void: Symbol = 157;
const anon_sym_wait: Symbol = 332;
const anon_sym_wait_order: Symbol = 333;
const anon_sym_wand: Symbol = 36;
const anon_sym_weak: Symbol = 196;
const anon_sym_weak0: Symbol = 162;
const anon_sym_weak1: Symbol = 164;
const anon_sym_while: Symbol = 345;
const anon_sym_wildcard: Symbol = 245;
const anon_sym_wire: Symbol = 32;
const anon_sym_with: Symbol = 135;
const anon_sym_within: Symbol = 226;
const anon_sym_wor: Symbol = 38;
const anon_sym_xnor: Symbol = 273;
const anon_sym_xor: Symbol = 272;
const aux_sym_begin_keywords_token1: Symbol = 47;
const aux_sym_decimal_number_token1: Symbol = 416;
const aux_sym_decimal_number_token2: Symbol = 417;
const aux_sym_default_nettype_compiler_directive_token1: Symbol = 31;
const aux_sym_double_quoted_string_token1: Symbol = 3;
const aux_sym_edge_descriptor_token1: Symbol = 375;
const aux_sym_edge_descriptor_token2: Symbol = 376;
const aux_sym_escaped_identifier_token1: Symbol = 432;
const aux_sym_id_directive_token1: Symbol = 17;
const aux_sym_id_directive_token2: Symbol = 18;
const aux_sym_id_directive_token3: Symbol = 19;
const aux_sym_id_directive_token4: Symbol = 20;
const aux_sym_include_compiler_directive_standard_token1: Symbol = 5;
const aux_sym_include_compiler_directive_token1: Symbol = 7;
const aux_sym_line_compiler_directive_token1: Symbol = 46;
const aux_sym_real_number_token1: Symbol = 421;
const aux_sym_string_literal_token1: Symbol = 409;
const aux_sym_string_literal_token2: Symbol = 410;
const aux_sym_string_literal_token3: Symbol = 411;
const aux_sym_text_macro_definition_token1: Symbol = 14;
const aux_sym_timescale_compiler_directive_token1: Symbol = 29;
const aux_sym_unbased_unsized_literal_token1: Symbol = 426;
const aux_sym_unconnected_drive_token1: Symbol = 43;
const aux_sym_zero_directive_token1: Symbol = 21;
const aux_sym_zero_directive_token2: Symbol = 22;
const aux_sym_zero_directive_token3: Symbol = 23;
const aux_sym_zero_directive_token4: Symbol = 24;
const aux_sym_zero_directive_token5: Symbol = 25;
const aux_sym_zero_directive_token6: Symbol = 26;
const aux_sym_zero_directive_token7: Symbol = 27;
const aux_sym_zero_directive_token8: Symbol = 28;
const sym_binary_number: Symbol = 418;
const sym_c_identifier: Symbol = 430;
const sym_comment: Symbol = 429;
const sym_default_text: Symbol = 8;
const sym_edge_symbol: Symbol = 300;
const sym_fixed_point_number: Symbol = 422;
const sym_hex_number: Symbol = 420;
const sym_level_symbol: Symbol = 299;
const sym_macro_text: Symbol = 9;
const sym_octal_number: Symbol = 419;
const sym_output_symbol: Symbol = 298;
const sym_simple_identifier: Symbol = 1;
const sym_system_tf_identifier: Symbol = 435;
const sym_unsigned_number: Symbol = 423;
const ts_builtin_sym_end: Symbol = 0;

#[rustfmt::skip]
static sym_edge_symbol_character_set_1: [CharacterRange; 9] = [
    CharacterRange::new(42, 42), CharacterRange::new(70, 70), CharacterRange::new(78, 78), CharacterRange::new(80, 80), CharacterRange::new(82, 82), CharacterRange::new(102, 102),
    CharacterRange::new(110, 110), CharacterRange::new(112, 112), CharacterRange::new(114, 114),
];

#[rustfmt::skip]
static sym_hex_number_character_set_1: [CharacterRange; 10] = [
    CharacterRange::new(9, 13), CharacterRange::new(32, 32), CharacterRange::new(48, 57), CharacterRange::new(63, 63), CharacterRange::new(65, 70), CharacterRange::new(88, 88),
    CharacterRange::new(90, 90), CharacterRange::new(97, 102), CharacterRange::new(120, 120), CharacterRange::new(122, 122),
];

#[rustfmt::skip]
static sym_hex_number_character_set_2: [CharacterRange; 9] = [
    CharacterRange::new(48, 57), CharacterRange::new(63, 63), CharacterRange::new(65, 70), CharacterRange::new(88, 88), CharacterRange::new(90, 90), CharacterRange::new(95, 95),
    CharacterRange::new(97, 102), CharacterRange::new(120, 120), CharacterRange::new(122, 122),
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
                if eof { state = 374; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 522), (34, 375), (35, 447), (36, 481), (37, 497), (38, 621), (39, 646), (40, 406),
                    (41, 408), (42, 474), (43, 488), (44, 409), (45, 541), (46, 450), (47, 435), (48, 457),
                    (49, 460), (50, 462), (58, 444), (59, 441), (60, 382), (61, 412), (62, 391), (63, 616),
                    (64, 572), (80, 555), (91, 466), (92, 678), (93, 468), (94, 629), (96, 420), (123, 451),
                    (124, 625), (125, 452), (126, 597), (8211, 492), (66, 553), (98, 553), (88, 551), (120, 551),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 0; lexer.advance(true); continue; }
                if lookahead == 70 || lookahead == 78 || lookahead == 82 || lookahead == 102 || lookahead == 110 || lookahead == 112 || lookahead == 114 { state = 556; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            1 => {
                if lookahead == 10 { state = 645; lexer.advance(false); continue; }
                if lookahead != 0 { state = 644; lexer.advance(false); continue; }
                return result;
            }
            2 => {
                if lookahead == 10 { state = 417; lexer.advance(false); continue; }
                if lookahead == 47 { state = 66; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 2; lexer.advance(true); continue; }
                return result;
            }
            3 => {
                if lookahead == 10 { state = 83; lexer.advance(true); continue; }
                if lookahead == 47 { state = 376; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 379; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 34 && lookahead != 92 { state = 380; lexer.advance(false); continue; }
                return result;
            }
            4 => {
                if lookahead == 10 { state = 83; lexer.advance(true); continue; }
                if lookahead == 47 { state = 385; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 388; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 62 && lookahead != 92 { state = 389; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                if lookahead == 13 { state = 403; lexer.advance(false); continue; }
                if lookahead != 0 { state = 402; lexer.advance(false); continue; }
                return result;
            }
            6 => {
                if lookahead == 13 { state = 401; lexer.advance(false); continue; }
                if lookahead == 42 { state = 395; lexer.advance(false); continue; }
                if lookahead != 0 { state = 396; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if let Some(next) = advance_map(&[
                    (33, 522), (34, 375), (35, 447), (36, 481), (37, 496), (38, 620), (39, 88), (40, 405),
                    (41, 408), (42, 475), (43, 486), (44, 409), (45, 547), (46, 449), (47, 434), (48, 664),
                    (58, 443), (59, 441), (60, 383), (61, 107), (62, 392), (63, 616), (64, 573), (91, 68),
                    (92, 678), (93, 468), (94, 630), (96, 419), (123, 451), (124, 626), (126, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 7; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 661; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            8 => {
                if let Some(next) = advance_map(&[
                    (33, 522), (34, 375), (35, 56), (36, 481), (37, 496), (38, 620), (39, 647), (40, 405),
                    (41, 408), (42, 475), (43, 486), (44, 409), (45, 547), (46, 449), (47, 434), (48, 664),
                    (58, 442), (59, 441), (60, 383), (61, 108), (62, 392), (63, 616), (91, 69), (92, 678),
                    (93, 468), (94, 630), (96, 419), (123, 451), (124, 628), (125, 452), (126, 597), (8211, 116),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 8; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 661; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if let Some(next) = advance_map(&[
                    (33, 522), (34, 375), (35, 445), (36, 481), (37, 496), (38, 620), (39, 647), (40, 406),
                    (41, 408), (42, 475), (43, 486), (44, 409), (45, 547), (46, 449), (47, 434), (48, 664),
                    (58, 443), (59, 441), (60, 383), (61, 107), (62, 392), (63, 616), (91, 464), (92, 678),
                    (93, 468), (94, 630), (96, 419), (123, 451), (124, 628), (125, 452), (126, 597), (8211, 116),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 9; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 661; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            10 => {
                if let Some(next) = advance_map(&[
                    (33, 522), (34, 375), (35, 445), (36, 481), (37, 496), (38, 620), (39, 647), (40, 405),
                    (41, 408), (42, 475), (43, 486), (44, 409), (45, 547), (46, 449), (47, 434), (48, 664),
                    (58, 443), (59, 441), (60, 383), (61, 107), (62, 392), (63, 616), (64, 573), (91, 464),
                    (92, 678), (93, 468), (94, 630), (96, 419), (123, 451), (124, 628), (125, 452), (126, 597),
                    (8211, 116),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 10; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 661; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if let Some(next) = advance_map(&[
                    (33, 522), (34, 375), (35, 448), (36, 481), (37, 496), (38, 620), (39, 647), (40, 406),
                    (41, 408), (42, 475), (43, 486), (44, 409), (45, 546), (46, 449), (47, 434), (48, 664),
                    (58, 443), (59, 441), (60, 383), (61, 107), (62, 392), (63, 616), (64, 571), (91, 464),
                    (92, 678), (93, 468), (94, 630), (96, 419), (123, 451), (124, 628), (125, 452), (126, 597),
                    (8211, 116),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 11; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 661; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if let Some(next) = advance_map(&[
                    (33, 522), (34, 375), (36, 481), (37, 496), (38, 620), (39, 88), (40, 405), (41, 408),
                    (42, 475), (43, 486), (44, 409), (45, 547), (47, 434), (48, 664), (58, 86), (60, 383),
                    (61, 108), (62, 392), (63, 616), (91, 73), (92, 678), (94, 630), (96, 419), (123, 451),
                    (124, 628), (125, 452), (126, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 12; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 661; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if let Some(next) = advance_map(&[
                    (33, 522), (34, 375), (36, 481), (37, 496), (38, 620), (39, 88), (40, 405), (42, 475),
                    (43, 487), (45, 543), (47, 434), (48, 664), (58, 442), (60, 383), (61, 107), (62, 392),
                    (63, 616), (92, 678), (93, 468), (94, 630), (96, 419), (123, 451), (124, 628), (126, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 13; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 661; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if let Some(next) = advance_map(&[
                    (33, 105), (35, 447), (36, 226), (37, 496), (38, 620), (39, 648), (40, 405), (41, 408),
                    (42, 475), (43, 485), (44, 409), (45, 549), (46, 449), (47, 434), (49, 663), (58, 443),
                    (59, 441), (60, 383), (61, 107), (62, 392), (63, 616), (91, 465), (92, 678), (94, 630),
                    (124, 626), (126, 133),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 14; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 664; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if let Some(next) = advance_map(&[
                    (33, 105), (35, 447), (37, 497), (38, 621), (39, 648), (40, 406), (41, 408), (42, 476),
                    (43, 489), (44, 409), (45, 545), (46, 449), (47, 435), (58, 443), (60, 382), (61, 411),
                    (62, 391), (63, 616), (91, 465), (94, 629), (124, 625), (126, 133),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 15; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if let Some(next) = advance_map(&[
                    (33, 105), (35, 447), (37, 496), (38, 620), (39, 648), (40, 406), (41, 408), (42, 475),
                    (43, 485), (44, 409), (45, 549), (46, 449), (47, 434), (58, 443), (59, 441), (60, 383),
                    (61, 107), (62, 392), (63, 616), (91, 465), (94, 630), (124, 626), (126, 133),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 16; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if let Some(next) = advance_map(&[
                    (33, 105), (35, 447), (37, 496), (38, 620), (39, 648), (40, 406), (41, 408), (42, 475),
                    (43, 486), (44, 409), (45, 547), (46, 449), (47, 434), (58, 443), (59, 441), (60, 383),
                    (61, 107), (62, 392), (63, 616), (91, 465), (92, 678), (94, 630), (124, 626), (126, 133),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 17; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            18 => {
                if let Some(next) = advance_map(&[
                    (33, 105), (35, 446), (36, 315), (37, 496), (38, 620), (39, 648), (40, 406), (41, 408),
                    (42, 475), (43, 485), (44, 409), (45, 549), (46, 449), (47, 434), (58, 443), (59, 441),
                    (60, 383), (61, 108), (62, 392), (63, 616), (91, 466), (92, 678), (93, 468), (94, 630),
                    (123, 451), (124, 628), (125, 452), (126, 133), (8211, 116),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 18; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if let Some(next) = advance_map(&[
                    (33, 105), (35, 446), (37, 497), (38, 621), (39, 648), (40, 406), (41, 408), (42, 476),
                    (43, 489), (44, 409), (45, 545), (46, 449), (47, 435), (58, 443), (60, 382), (61, 412),
                    (62, 391), (63, 616), (91, 466), (94, 629), (124, 627), (126, 133),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 19; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            20 => {
                if let Some(next) = advance_map(&[
                    (33, 105), (35, 446), (37, 496), (38, 620), (39, 648), (40, 406), (41, 408), (42, 475),
                    (43, 486), (44, 409), (45, 547), (46, 449), (47, 434), (58, 443), (59, 441), (60, 383),
                    (61, 108), (62, 392), (63, 616), (91, 466), (92, 678), (93, 468), (94, 630), (123, 451),
                    (124, 628), (125, 452), (126, 133), (8211, 116),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 20; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                if let Some(next) = advance_map(&[
                    (33, 105), (35, 446), (37, 496), (38, 620), (39, 649), (40, 405), (41, 408), (42, 475),
                    (43, 485), (44, 409), (45, 549), (46, 449), (47, 434), (48, 666), (58, 443), (59, 441),
                    (60, 383), (61, 108), (62, 392), (63, 616), (91, 466), (92, 678), (93, 468), (94, 630),
                    (123, 451), (124, 628), (125, 452), (126, 133), (8211, 116),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 21; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 662; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            22 => {
                if let Some(next) = advance_map(&[
                    (33, 105), (35, 446), (37, 496), (38, 623), (39, 648), (41, 408), (42, 475), (43, 490),
                    (44, 409), (45, 548), (46, 449), (47, 434), (58, 443), (59, 441), (60, 383), (61, 107),
                    (62, 392), (63, 616), (91, 464), (93, 468), (94, 630), (124, 628), (126, 133),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 22; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            23 => {
                if let Some(next) = advance_map(&[
                    (33, 105), (35, 445), (36, 200), (37, 496), (38, 620), (39, 648), (40, 406), (41, 408),
                    (42, 475), (43, 485), (44, 409), (45, 549), (46, 449), (47, 434), (48, 99), (49, 94),
                    (58, 85), (59, 441), (60, 383), (61, 107), (62, 392), (63, 616), (91, 464), (92, 678),
                    (94, 630), (123, 451), (124, 628), (125, 452), (126, 133),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 23; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            24 => {
                if let Some(next) = advance_map(&[
                    (33, 105), (35, 445), (36, 200), (37, 496), (38, 620), (39, 648), (40, 406), (41, 408),
                    (42, 475), (43, 490), (44, 409), (45, 548), (46, 449), (47, 434), (58, 443), (59, 441),
                    (60, 383), (61, 107), (62, 392), (63, 616), (91, 464), (92, 678), (93, 468), (94, 630),
                    (96, 420), (124, 628), (125, 452), (126, 133),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 24; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            25 => {
                if let Some(next) = advance_map(&[
                    (33, 105), (35, 445), (36, 200), (37, 496), (38, 620), (39, 649), (40, 406), (41, 408),
                    (42, 475), (43, 486), (44, 409), (45, 547), (46, 449), (47, 434), (48, 666), (58, 85),
                    (59, 441), (60, 383), (61, 107), (62, 392), (63, 616), (91, 464), (92, 678), (94, 630),
                    (123, 451), (124, 628), (125, 452), (126, 133), (8211, 116),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 25; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 662; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            26 => {
                if let Some(next) = advance_map(&[
                    (33, 105), (35, 445), (36, 200), (37, 496), (38, 623), (40, 65), (42, 473), (43, 485),
                    (44, 409), (45, 549), (46, 449), (47, 434), (58, 100), (59, 441), (60, 383), (61, 107),
                    (62, 392), (63, 616), (91, 464), (92, 678), (94, 630), (124, 628), (126, 133),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 26; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 665; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            27 => {
                if let Some(next) = advance_map(&[
                    (33, 105), (35, 445), (36, 315), (37, 496), (38, 623), (39, 648), (40, 406), (41, 408),
                    (42, 477), (43, 485), (44, 409), (45, 549), (46, 449), (47, 434), (58, 443), (59, 441),
                    (60, 383), (61, 108), (62, 392), (63, 616), (91, 464), (92, 678), (93, 468), (94, 630),
                    (123, 451), (124, 628), (125, 452), (126, 133),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 27; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            28 => {
                if let Some(next) = advance_map(&[
                    (33, 105), (35, 445), (36, 315), (37, 496), (38, 623), (39, 648), (40, 405), (41, 408),
                    (42, 475), (43, 485), (44, 409), (45, 549), (46, 449), (47, 434), (58, 443), (59, 441),
                    (60, 383), (61, 107), (62, 392), (63, 616), (91, 464), (92, 678), (93, 468), (94, 630),
                    (123, 451), (124, 628), (125, 452), (126, 133),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 28; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                if let Some(next) = advance_map(&[
                    (33, 105), (35, 445), (37, 497), (38, 621), (39, 648), (40, 406), (41, 408), (42, 476),
                    (43, 489), (44, 409), (45, 545), (46, 449), (47, 435), (58, 443), (60, 382), (61, 412),
                    (62, 391), (63, 616), (91, 467), (94, 629), (124, 627), (126, 133),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 29; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            30 => {
                if let Some(next) = advance_map(&[
                    (33, 105), (35, 445), (37, 497), (38, 621), (39, 648), (40, 406), (41, 408), (42, 476),
                    (43, 489), (44, 409), (45, 545), (46, 449), (47, 435), (58, 443), (60, 382), (61, 411),
                    (62, 391), (63, 616), (91, 464), (94, 629), (124, 627), (126, 133),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 30; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            31 => {
                if let Some(next) = advance_map(&[
                    (33, 105), (35, 445), (37, 496), (38, 620), (39, 648), (40, 406), (41, 408), (42, 475),
                    (43, 485), (44, 409), (45, 549), (46, 449), (47, 434), (58, 443), (60, 383), (61, 108),
                    (62, 392), (63, 616), (91, 467), (94, 630), (124, 628), (126, 133),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 31; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                if let Some(next) = advance_map(&[
                    (33, 105), (35, 445), (37, 496), (38, 620), (39, 648), (40, 406), (41, 408), (42, 475),
                    (43, 486), (44, 409), (45, 547), (46, 449), (47, 434), (58, 443), (60, 383), (61, 108),
                    (62, 392), (63, 616), (91, 467), (94, 630), (124, 628), (126, 133),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 32; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            33 => {
                if let Some(next) = advance_map(&[
                    (33, 105), (35, 445), (37, 496), (38, 620), (39, 648), (40, 406), (42, 475), (43, 487),
                    (45, 543), (46, 449), (47, 434), (58, 443), (60, 383), (61, 107), (62, 392), (63, 616),
                    (91, 464), (93, 468), (94, 630), (124, 628), (126, 133),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 33; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            34 => {
                if let Some(next) = advance_map(&[
                    (33, 105), (35, 445), (37, 496), (38, 620), (39, 648), (40, 405), (41, 408), (42, 475),
                    (43, 485), (44, 409), (45, 549), (46, 449), (47, 434), (58, 443), (59, 441), (60, 383),
                    (61, 107), (62, 392), (63, 616), (91, 464), (93, 468), (94, 630), (123, 451), (124, 628),
                    (125, 452), (126, 133), (8211, 116),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 34; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            35 => {
                if let Some(next) = advance_map(&[
                    (33, 105), (35, 445), (37, 496), (38, 620), (39, 648), (40, 405), (41, 408), (42, 475),
                    (43, 485), (44, 409), (45, 549), (46, 449), (47, 434), (58, 443), (60, 383), (61, 108),
                    (62, 392), (63, 616), (91, 467), (94, 630), (124, 628), (126, 133),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 35; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            36 => {
                if let Some(next) = advance_map(&[
                    (33, 105), (35, 445), (37, 496), (38, 620), (39, 648), (40, 405), (42, 475), (43, 485),
                    (44, 409), (45, 549), (46, 449), (47, 434), (58, 85), (60, 383), (61, 107), (62, 392),
                    (63, 616), (91, 464), (94, 630), (124, 628), (125, 452), (126, 133),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 36; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                if let Some(next) = advance_map(&[
                    (33, 105), (35, 445), (37, 496), (38, 620), (39, 648), (40, 405), (42, 475), (43, 490),
                    (45, 548), (46, 449), (47, 434), (58, 443), (60, 383), (61, 107), (62, 392), (63, 616),
                    (91, 464), (93, 468), (94, 630), (124, 628), (126, 133),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 37; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            38 => {
                if let Some(next) = advance_map(&[
                    (33, 105), (35, 448), (36, 310), (37, 497), (38, 621), (39, 648), (40, 406), (41, 408),
                    (42, 476), (43, 489), (44, 409), (45, 544), (46, 449), (47, 435), (58, 443), (59, 441),
                    (60, 382), (61, 411), (62, 391), (63, 616), (64, 571), (91, 464), (92, 678), (94, 629),
                    (123, 451), (124, 627), (126, 133),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 38; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                if let Some(next) = advance_map(&[
                    (33, 105), (35, 448), (36, 310), (37, 496), (38, 620), (39, 648), (40, 406), (42, 475),
                    (43, 487), (45, 542), (46, 449), (47, 434), (58, 443), (59, 441), (60, 383), (61, 107),
                    (62, 392), (63, 616), (64, 571), (91, 464), (92, 678), (93, 468), (94, 630), (123, 451),
                    (124, 628), (126, 133),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 39; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            40 => {
                if let Some(next) = advance_map(&[
                    (33, 105), (35, 55), (37, 496), (38, 620), (40, 405), (41, 408), (42, 475), (43, 485),
                    (44, 409), (45, 549), (46, 449), (47, 434), (58, 86), (59, 441), (60, 383), (61, 108),
                    (62, 392), (63, 616), (91, 69), (93, 468), (94, 630), (123, 451), (124, 628), (125, 452),
                    (126, 133), (8211, 116),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 40; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                if let Some(next) = advance_map(&[
                    (33, 521), (34, 375), (35, 446), (36, 481), (38, 619), (39, 647), (40, 405), (41, 408),
                    (42, 472), (43, 486), (44, 409), (45, 540), (46, 449), (47, 66), (48, 664), (58, 443),
                    (59, 441), (60, 102), (61, 410), (62, 123), (64, 571), (91, 464), (92, 678), (93, 468),
                    (94, 630), (96, 419), (123, 451), (124, 624), (125, 452), (126, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 41; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 661; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            42 => {
                if let Some(next) = advance_map(&[
                    (33, 521), (34, 375), (35, 56), (36, 481), (38, 619), (39, 88), (40, 406), (41, 408),
                    (43, 486), (44, 409), (45, 540), (46, 450), (47, 66), (48, 664), (58, 86), (60, 381),
                    (64, 570), (92, 678), (94, 630), (96, 419), (123, 451), (124, 624), (125, 452), (126, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 42; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 661; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            43 => {
                if let Some(next) = advance_map(&[
                    (33, 521), (34, 375), (35, 445), (36, 481), (38, 619), (39, 88), (40, 405), (41, 408),
                    (42, 114), (43, 485), (44, 409), (45, 539), (46, 450), (47, 66), (48, 664), (58, 442),
                    (59, 441), (61, 115), (91, 464), (92, 678), (93, 468), (94, 630), (96, 419), (123, 451),
                    (124, 624), (125, 452), (126, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 43; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 661; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            44 => {
                if let Some(next) = advance_map(&[
                    (33, 521), (34, 375), (35, 448), (36, 481), (38, 619), (39, 88), (40, 406), (43, 486),
                    (45, 546), (46, 450), (47, 66), (48, 664), (59, 441), (64, 571), (91, 464), (92, 678),
                    (94, 630), (96, 419), (123, 451), (124, 624), (126, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 44; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 661; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            45 => {
                if let Some(next) = advance_map(&[
                    (33, 521), (34, 375), (35, 59), (36, 481), (38, 619), (39, 88), (40, 405), (43, 486),
                    (45, 540), (47, 66), (48, 664), (92, 678), (94, 630), (96, 419), (123, 451), (124, 624),
                    (126, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 45; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 661; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            46 => {
                if let Some(next) = advance_map(&[
                    (33, 521), (34, 375), (36, 481), (38, 619), (39, 87), (40, 406), (43, 486), (45, 540),
                    (47, 66), (48, 458), (49, 461), (92, 678), (94, 630), (96, 419), (123, 451), (124, 624),
                    (126, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 46; lexer.advance(true); continue; }
                if 50 <= lookahead && lookahead <= 57 { state = 661; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                if let Some(next) = advance_map(&[
                    (33, 521), (34, 375), (36, 482), (38, 619), (39, 89), (40, 405), (42, 472), (43, 485),
                    (45, 539), (47, 66), (48, 664), (92, 678), (93, 468), (94, 630), (96, 419), (123, 451),
                    (124, 624), (126, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 47; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 661; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            48 => {
                if let Some(next) = advance_map(&[
                    (33, 521), (34, 375), (36, 345), (38, 619), (39, 89), (40, 406), (43, 485), (45, 539),
                    (46, 449), (47, 66), (48, 664), (91, 464), (92, 678), (94, 630), (96, 419), (123, 451),
                    (124, 624), (126, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 48; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 661; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                if let Some(next) = advance_map(&[
                    (33, 521), (34, 375), (36, 345), (38, 622), (39, 88), (40, 405), (41, 408), (42, 114),
                    (43, 485), (44, 409), (45, 539), (46, 450), (47, 66), (48, 664), (58, 442), (61, 115),
                    (63, 616), (91, 464), (92, 678), (94, 630), (96, 419), (123, 451), (124, 624), (126, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 49; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 661; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            50 => {
                if let Some(next) = advance_map(&[
                    (33, 113), (34, 375), (35, 448), (36, 481), (37, 496), (39, 88), (40, 406), (42, 475),
                    (43, 486), (45, 77), (46, 449), (47, 434), (48, 664), (58, 443), (59, 441), (60, 384),
                    (61, 413), (62, 390), (64, 571), (91, 464), (92, 678), (96, 419), (123, 451), (8211, 493),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 50; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 661; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            51 => {
                if lookahead == 34 { state = 375; lexer.advance(false); continue; }
                if lookahead == 47 { state = 639; lexer.advance(false); continue; }
                if lookahead == 92 { state = 1; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 642; lexer.advance(false); continue; }
                if lookahead != 0 { state = 643; lexer.advance(false); continue; }
                return result;
            }
            52 => {
                if lookahead == 34 { state = 484; lexer.advance(false); continue; }
                if lookahead == 45 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                if lookahead == 34 { state = 483; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                if let Some(next) = advance_map(&[
                    (35, 447), (40, 405), (41, 408), (44, 409), (46, 449), (47, 66), (58, 443), (59, 441),
                    (61, 410), (91, 464), (92, 678), (124, 80),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 54; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            55 => {
                if lookahead == 35 { state = 509; lexer.advance(false); continue; }
                return result;
            }
            56 => {
                if lookahead == 35 { state = 509; lexer.advance(false); continue; }
                if lookahead == 48 { state = 580; lexer.advance(false); continue; }
                return result;
            }
            57 => {
                if lookahead == 35 { state = 506; lexer.advance(false); continue; }
                return result;
            }
            58 => {
                if lookahead == 35 { state = 507; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                if lookahead == 35 { state = 508; lexer.advance(false); continue; }
                return result;
            }
            60 => {
                if lookahead == 38 { state = 578; lexer.advance(false); continue; }
                return result;
            }
            61 => {
                if lookahead == 38 { state = 60; lexer.advance(false); continue; }
                if lookahead == 61 { state = 562; lexer.advance(false); continue; }
                return result;
            }
            62 => {
                if lookahead == 39 { state = 363; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 62; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                if lookahead == 40 { state = 405; lexer.advance(false); continue; }
                if lookahead == 47 { state = 66; lexer.advance(false); continue; }
                if lookahead == 58 { state = 442; lexer.advance(false); continue; }
                if lookahead == 101 { state = 287; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 63; lexer.advance(true); continue; }
                if lookahead == 48 || lookahead == 49 || lookahead == 63 || lookahead == 66 || lookahead == 88 || lookahead == 98 || lookahead == 120 { state = 552; lexer.advance(false); continue; }
                if set_contains(&sym_edge_symbol_character_set_1, lookahead) { state = 554; lexer.advance(false); continue; }
                return result;
            }
            64 => {
                if lookahead == 41 { state = 671; lexer.advance(false); continue; }
                if lookahead == 58 { state = 101; lexer.advance(false); continue; }
                if lookahead == 61 { state = 559; lexer.advance(false); continue; }
                if lookahead == 62 { state = 581; lexer.advance(false); continue; }
                return result;
            }
            65 => {
                if lookahead == 42 { state = 670; lexer.advance(false); continue; }
                return result;
            }
            66 => {
                if lookahead == 42 { state = 71; lexer.advance(false); continue; }
                if lookahead == 47 { state = 676; lexer.advance(false); continue; }
                return result;
            }
            67 => {
                if lookahead == 42 { state = 71; lexer.advance(false); continue; }
                if lookahead == 47 { state = 676; lexer.advance(false); continue; }
                if lookahead == 61 { state = 560; lexer.advance(false); continue; }
                return result;
            }
            68 => {
                if lookahead == 42 { state = 513; lexer.advance(false); continue; }
                if lookahead == 43 { state = 130; lexer.advance(false); continue; }
                if lookahead == 45 { state = 118; lexer.advance(false); continue; }
                if lookahead == 61 { state = 516; lexer.advance(false); continue; }
                return result;
            }
            69 => {
                if lookahead == 42 { state = 513; lexer.advance(false); continue; }
                if lookahead == 43 { state = 130; lexer.advance(false); continue; }
                if lookahead == 45 { state = 118; lexer.advance(false); continue; }
                if lookahead == 61 { state = 516; lexer.advance(false); continue; }
                if lookahead == 8211 { state = 119; lexer.advance(false); continue; }
                return result;
            }
            70 => {
                if lookahead == 42 { state = 70; lexer.advance(false); continue; }
                if lookahead == 47 { state = 672; lexer.advance(false); continue; }
                if lookahead != 0 { state = 71; lexer.advance(false); continue; }
                return result;
            }
            71 => {
                if lookahead == 42 { state = 70; lexer.advance(false); continue; }
                if lookahead != 0 { state = 71; lexer.advance(false); continue; }
                return result;
            }
            72 => {
                if lookahead == 42 { state = 478; lexer.advance(false); continue; }
                return result;
            }
            73 => {
                if lookahead == 42 { state = 512; lexer.advance(false); continue; }
                if lookahead == 61 { state = 516; lexer.advance(false); continue; }
                if lookahead == 8211 { state = 119; lexer.advance(false); continue; }
                return result;
            }
            74 => {
                if lookahead == 42 { state = 131; lexer.advance(false); continue; }
                if lookahead == 43 { state = 132; lexer.advance(false); continue; }
                return result;
            }
            75 => {
                if lookahead == 43 { state = 491; lexer.advance(false); continue; }
                if lookahead == 61 { state = 557; lexer.advance(false); continue; }
                return result;
            }
            76 => {
                if lookahead == 45 { state = 652; lexer.advance(false); continue; }
                if lookahead == 61 { state = 558; lexer.advance(false); continue; }
                return result;
            }
            77 => {
                if lookahead == 45 { state = 652; lexer.advance(false); continue; }
                if lookahead == 62 { state = 576; lexer.advance(false); continue; }
                return result;
            }
            78 => {
                if lookahead == 45 { state = 539; lexer.advance(false); continue; }
                if lookahead == 47 { state = 66; lexer.advance(false); continue; }
                if lookahead == 48 || lookahead == 49 || lookahead == 88 || lookahead == 120 { state = 550; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 78; lexer.advance(true); continue; }
                return result;
            }
            79 => {
                if lookahead == 45 { state = 120; lexer.advance(false); continue; }
                if lookahead == 61 { state = 564; lexer.advance(false); continue; }
                return result;
            }
            80 => {
                if lookahead == 45 { state = 120; lexer.advance(false); continue; }
                if lookahead == 61 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            81 => {
                if lookahead == 47 { state = 66; lexer.advance(false); continue; }
                if lookahead == 80 { state = 690; lexer.advance(false); continue; }
                if lookahead == 91 { state = 464; lexer.advance(false); continue; }
                if lookahead == 92 { state = 678; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 81; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            82 => {
                if lookahead == 47 { state = 66; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 82; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 677; lexer.advance(false); continue; }
                return result;
            }
            83 => {
                if lookahead == 47 { state = 66; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 83; lexer.advance(true); continue; }
                return result;
            }
            84 => {
                if lookahead == 47 { state = 66; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 84; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 394; lexer.advance(false); continue; }
                return result;
            }
            85 => {
                if lookahead == 47 { state = 470; lexer.advance(false); continue; }
                if lookahead == 58 { state = 471; lexer.advance(false); continue; }
                if lookahead == 61 { state = 469; lexer.advance(false); continue; }
                return result;
            }
            86 => {
                if lookahead == 47 { state = 470; lexer.advance(false); continue; }
                if lookahead == 61 { state = 469; lexer.advance(false); continue; }
                return result;
            }
            87 => {
                if let Some(next) = advance_map(&[
                    (48, 667), (49, 668), (66, 95), (98, 96), (123, 579), (68, 364), (100, 364), (72, 366),
                    (104, 366), (79, 367), (111, 367), (83, 362), (115, 362), (88, 669), (90, 669), (120, 669),
                    (122, 669),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            88 => {
                if let Some(next) = advance_map(&[
                    (48, 667), (49, 668), (123, 579), (66, 365), (98, 365), (68, 364), (100, 364), (72, 366),
                    (104, 366), (79, 367), (111, 367), (83, 362), (115, 362), (88, 669), (90, 669), (120, 669),
                    (122, 669),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            89 => {
                if let Some(next) = advance_map(&[
                    (48, 667), (49, 668), (66, 365), (98, 365), (68, 364), (100, 364), (72, 366), (104, 366),
                    (79, 367), (111, 367), (83, 362), (115, 362), (88, 669), (90, 669), (120, 669), (122, 669),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            90 => {
                if lookahead == 48 { state = 604; lexer.advance(false); continue; }
                if lookahead == 49 { state = 606; lexer.advance(false); continue; }
                return result;
            }
            91 => {
                if lookahead == 48 { state = 600; lexer.advance(false); continue; }
                if lookahead == 49 { state = 602; lexer.advance(false); continue; }
                return result;
            }
            92 => {
                if lookahead == 48 { state = 533; lexer.advance(false); continue; }
                if lookahead == 49 { state = 535; lexer.advance(false); continue; }
                if lookahead == 88 { state = 538; lexer.advance(false); continue; }
                if lookahead == 120 { state = 537; lexer.advance(false); continue; }
                return result;
            }
            93 => {
                if lookahead == 48 { state = 527; lexer.advance(false); continue; }
                if lookahead == 49 { state = 529; lexer.advance(false); continue; }
                if lookahead == 88 { state = 532; lexer.advance(false); continue; }
                if lookahead == 120 { state = 531; lexer.advance(false); continue; }
                return result;
            }
            94 => {
                if lookahead == 48 { state = 595; lexer.advance(false); continue; }
                if lookahead == 88 || lookahead == 90 || lookahead == 120 || lookahead == 122 { state = 596; lexer.advance(false); continue; }
                return result;
            }
            95 => {
                if lookahead == 48 { state = 605; lexer.advance(false); continue; }
                if lookahead == 49 { state = 607; lexer.advance(false); continue; }
                if lookahead == 63 || lookahead == 88 || lookahead == 90 || lookahead == 120 || lookahead == 122 { state = 655; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 365; lexer.advance(false); continue; }
                return result;
            }
            96 => {
                if lookahead == 48 { state = 601; lexer.advance(false); continue; }
                if lookahead == 49 { state = 603; lexer.advance(false); continue; }
                if lookahead == 63 || lookahead == 88 || lookahead == 90 || lookahead == 120 || lookahead == 122 { state = 655; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 365; lexer.advance(false); continue; }
                return result;
            }
            97 => {
                if lookahead == 48 { state = 534; lexer.advance(false); continue; }
                if lookahead == 49 { state = 536; lexer.advance(false); continue; }
                if lookahead == 63 || lookahead == 88 || lookahead == 90 || lookahead == 120 || lookahead == 122 { state = 655; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 365; lexer.advance(false); continue; }
                return result;
            }
            98 => {
                if lookahead == 48 { state = 528; lexer.advance(false); continue; }
                if lookahead == 49 { state = 530; lexer.advance(false); continue; }
                if lookahead == 63 || lookahead == 88 || lookahead == 90 || lookahead == 120 || lookahead == 122 { state = 655; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 365; lexer.advance(false); continue; }
                return result;
            }
            99 => {
                if lookahead == 49 { state = 594; lexer.advance(false); continue; }
                if lookahead == 88 || lookahead == 90 || lookahead == 120 || lookahead == 122 { state = 596; lexer.advance(false); continue; }
                return result;
            }
            100 => {
                if lookahead == 58 { state = 471; lexer.advance(false); continue; }
                return result;
            }
            101 => {
                if lookahead == 58 { state = 72; lexer.advance(false); continue; }
                return result;
            }
            102 => {
                if lookahead == 60 { state = 611; lexer.advance(false); continue; }
                return result;
            }
            103 => {
                if lookahead == 60 { state = 104; lexer.advance(false); continue; }
                if lookahead == 61 { state = 502; lexer.advance(false); continue; }
                return result;
            }
            104 => {
                if lookahead == 60 { state = 111; lexer.advance(false); continue; }
                if lookahead == 61 { state = 566; lexer.advance(false); continue; }
                return result;
            }
            105 => {
                if lookahead == 61 { state = 501; lexer.advance(false); continue; }
                return result;
            }
            106 => {
                if lookahead == 61 { state = 561; lexer.advance(false); continue; }
                return result;
            }
            107 => {
                if lookahead == 61 { state = 499; lexer.advance(false); continue; }
                return result;
            }
            108 => {
                if lookahead == 61 { state = 499; lexer.advance(false); continue; }
                if lookahead == 62 { state = 519; lexer.advance(false); continue; }
                return result;
            }
            109 => {
                if lookahead == 61 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            110 => {
                if lookahead == 61 { state = 567; lexer.advance(false); continue; }
                if lookahead == 62 { state = 112; lexer.advance(false); continue; }
                return result;
            }
            111 => {
                if lookahead == 61 { state = 568; lexer.advance(false); continue; }
                return result;
            }
            112 => {
                if lookahead == 61 { state = 569; lexer.advance(false); continue; }
                return result;
            }
            113 => {
                if lookahead == 61 { state = 500; lexer.advance(false); continue; }
                return result;
            }
            114 => {
                if lookahead == 62 { state = 581; lexer.advance(false); continue; }
                return result;
            }
            115 => {
                if lookahead == 62 { state = 519; lexer.advance(false); continue; }
                return result;
            }
            116 => {
                if lookahead == 62 { state = 463; lexer.advance(false); continue; }
                return result;
            }
            117 => {
                if lookahead == 62 { state = 637; lexer.advance(false); continue; }
                return result;
            }
            118 => {
                if lookahead == 62 { state = 517; lexer.advance(false); continue; }
                return result;
            }
            119 => {
                if lookahead == 62 { state = 520; lexer.advance(false); continue; }
                return result;
            }
            120 => {
                if lookahead == 62 { state = 504; lexer.advance(false); continue; }
                return result;
            }
            121 => {
                if lookahead == 62 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            122 => {
                if lookahead == 62 { state = 110; lexer.advance(false); continue; }
                return result;
            }
            123 => {
                if lookahead == 62 { state = 608; lexer.advance(false); continue; }
                return result;
            }
            124 => {
                if lookahead == 66 { state = 92; lexer.advance(false); continue; }
                if lookahead == 98 { state = 93; lexer.advance(false); continue; }
                return result;
            }
            125 => {
                if let Some(next) = advance_map(&[
                    (66, 97), (98, 98), (68, 364), (100, 364), (72, 366), (104, 366), (79, 367), (111, 367),
                    (83, 362), (115, 362),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            126 => {
                if lookahead == 67 { state = 53; lexer.advance(false); continue; }
                return result;
            }
            127 => {
                if lookahead == 68 { state = 129; lexer.advance(false); continue; }
                return result;
            }
            128 => {
                if lookahead == 73 { state = 52; lexer.advance(false); continue; }
                return result;
            }
            129 => {
                if lookahead == 80 { state = 128; lexer.advance(false); continue; }
                return result;
            }
            130 => {
                if lookahead == 93 { state = 515; lexer.advance(false); continue; }
                return result;
            }
            131 => {
                if lookahead == 93 { state = 510; lexer.advance(false); continue; }
                return result;
            }
            132 => {
                if lookahead == 93 { state = 511; lexer.advance(false); continue; }
                return result;
            }
            133 => {
                if lookahead == 94 { state = 632; lexer.advance(false); continue; }
                return result;
            }
            134 => {
                if lookahead == 95 { state = 247; lexer.advance(false); continue; }
                if lookahead == 99 { state = 218; lexer.advance(false); continue; }
                if lookahead == 105 { state = 222; lexer.advance(false); continue; }
                return result;
            }
            135 => {
                if lookahead == 95 { state = 162; lexer.advance(false); continue; }
                return result;
            }
            136 => {
                if lookahead == 95 { state = 285; lexer.advance(false); continue; }
                return result;
            }
            137 => {
                if lookahead == 95 { state = 172; lexer.advance(false); continue; }
                return result;
            }
            138 => {
                if lookahead == 95 { state = 250; lexer.advance(false); continue; }
                return result;
            }
            139 => {
                if lookahead == 97 { state = 149; lexer.advance(false); continue; }
                return result;
            }
            140 => {
                if lookahead == 97 { state = 344; lexer.advance(false); continue; }
                if lookahead == 105 { state = 281; lexer.advance(false); continue; }
                return result;
            }
            141 => {
                if lookahead == 97 { state = 319; lexer.advance(false); continue; }
                return result;
            }
            142 => {
                if lookahead == 97 { state = 340; lexer.advance(false); continue; }
                return result;
            }
            143 => {
                if lookahead == 97 { state = 265; lexer.advance(false); continue; }
                return result;
            }
            144 => {
                if lookahead == 97 { state = 254; lexer.advance(false); continue; }
                return result;
            }
            145 => {
                if lookahead == 97 { state = 277; lexer.advance(false); continue; }
                return result;
            }
            146 => {
                if lookahead == 97 { state = 255; lexer.advance(false); continue; }
                return result;
            }
            147 => {
                if lookahead == 97 { state = 259; lexer.advance(false); continue; }
                return result;
            }
            148 => {
                if lookahead == 97 { state = 261; lexer.advance(false); continue; }
                return result;
            }
            149 => {
                if lookahead == 98 { state = 266; lexer.advance(false); continue; }
                return result;
            }
            150 => {
                if lookahead == 99 { state = 234; lexer.advance(false); continue; }
                return result;
            }
            151 => {
                if lookahead == 99 { state = 303; lexer.advance(false); continue; }
                if lookahead == 109 { state = 295; lexer.advance(false); continue; }
                return result;
            }
            152 => {
                if lookahead == 99 { state = 297; lexer.advance(false); continue; }
                if lookahead == 100 { state = 206; lexer.advance(false); continue; }
                return result;
            }
            153 => {
                if lookahead == 99 { state = 143; lexer.advance(false); continue; }
                return result;
            }
            154 => {
                if lookahead == 99 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            155 => {
                if lookahead == 99 { state = 339; lexer.advance(false); continue; }
                return result;
            }
            156 => {
                if lookahead == 99 { state = 341; lexer.advance(false); continue; }
                return result;
            }
            157 => {
                if lookahead == 99 { state = 306; lexer.advance(false); continue; }
                return result;
            }
            158 => {
                if lookahead == 100 { state = 134; lexer.advance(false); continue; }
                return result;
            }
            159 => {
                if lookahead == 100 { state = 583; lexer.advance(false); continue; }
                return result;
            }
            160 => {
                if lookahead == 100 { state = 591; lexer.advance(false); continue; }
                return result;
            }
            161 => {
                if lookahead == 100 { state = 584; lexer.advance(false); continue; }
                return result;
            }
            162 => {
                if lookahead == 100 { state = 312; lexer.advance(false); continue; }
                return result;
            }
            163 => {
                if lookahead == 100 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            164 => {
                if lookahead == 100 { state = 323; lexer.advance(false); continue; }
                return result;
            }
            165 => {
                if lookahead == 100 { state = 324; lexer.advance(false); continue; }
                return result;
            }
            166 => {
                if lookahead == 100 { state = 202; lexer.advance(false); continue; }
                if lookahead == 110 { state = 169; lexer.advance(false); continue; }
                return result;
            }
            167 => {
                if lookahead == 100 { state = 334; lexer.advance(false); continue; }
                return result;
            }
            168 => {
                if lookahead == 100 { state = 338; lexer.advance(false); continue; }
                return result;
            }
            169 => {
                if lookahead == 100 { state = 207; lexer.advance(false); continue; }
                return result;
            }
            170 => {
                if lookahead == 100 { state = 208; lexer.advance(false); continue; }
                return result;
            }
            171 => {
                if lookahead == 100 { state = 183; lexer.advance(false); continue; }
                return result;
            }
            172 => {
                if lookahead == 100 { state = 320; lexer.advance(false); continue; }
                return result;
            }
            173 => {
                if lookahead == 100 { state = 137; lexer.advance(false); continue; }
                return result;
            }
            174 => {
                if lookahead == 100 { state = 215; lexer.advance(false); continue; }
                return result;
            }
            175 => {
                if lookahead == 101 { state = 232; lexer.advance(false); continue; }
                return result;
            }
            176 => {
                if lookahead == 101 { state = 256; lexer.advance(false); continue; }
                return result;
            }
            177 => {
                if lookahead == 101 { state = 220; lexer.advance(false); continue; }
                return result;
            }
            178 => {
                if lookahead == 101 { state = 307; lexer.advance(false); continue; }
                return result;
            }
            179 => {
                if lookahead == 101 { state = 428; lexer.advance(false); continue; }
                if lookahead == 105 { state = 221; lexer.advance(false); continue; }
                return result;
            }
            180 => {
                if lookahead == 101 { state = 438; lexer.advance(false); continue; }
                return result;
            }
            181 => {
                if lookahead == 101 { state = 415; lexer.advance(false); continue; }
                return result;
            }
            182 => {
                if lookahead == 101 { state = 357; lexer.advance(false); continue; }
                return result;
            }
            183 => {
                if lookahead == 101 { state = 393; lexer.advance(false); continue; }
                return result;
            }
            184 => {
                if lookahead == 101 { state = 433; lexer.advance(false); continue; }
                return result;
            }
            185 => {
                if lookahead == 101 { state = 430; lexer.advance(false); continue; }
                return result;
            }
            186 => {
                if lookahead == 101 { state = 431; lexer.advance(false); continue; }
                return result;
            }
            187 => {
                if lookahead == 101 { state = 436; lexer.advance(false); continue; }
                return result;
            }
            188 => {
                if lookahead == 101 { state = 437; lexer.advance(false); continue; }
                return result;
            }
            189 => {
                if lookahead == 101 { state = 429; lexer.advance(false); continue; }
                return result;
            }
            190 => {
                if lookahead == 101 { state = 151; lexer.advance(false); continue; }
                return result;
            }
            191 => {
                if lookahead == 101 { state = 593; lexer.advance(false); continue; }
                return result;
            }
            192 => {
                if lookahead == 101 { state = 526; lexer.advance(false); continue; }
                return result;
            }
            193 => {
                if lookahead == 101 { state = 329; lexer.advance(false); continue; }
                return result;
            }
            194 => {
                if lookahead == 101 { state = 326; lexer.advance(false); continue; }
                return result;
            }
            195 => {
                if lookahead == 101 { state = 270; lexer.advance(false); continue; }
                return result;
            }
            196 => {
                if lookahead == 101 { state = 352; lexer.advance(false); continue; }
                return result;
            }
            197 => {
                if lookahead == 101 { state = 325; lexer.advance(false); continue; }
                return result;
            }
            198 => {
                if lookahead == 101 { state = 353; lexer.advance(false); continue; }
                return result;
            }
            199 => {
                if lookahead == 101 { state = 316; lexer.advance(false); continue; }
                if lookahead == 102 { state = 142; lexer.advance(false); continue; }
                if lookahead == 105 { state = 280; lexer.advance(false); continue; }
                if lookahead == 114 { state = 296; lexer.advance(false); continue; }
                if lookahead == 117 { state = 278; lexer.advance(false); continue; }
                if lookahead == 119 { state = 141; lexer.advance(false); continue; }
                return result;
            }
            200 => {
                if lookahead == 101 { state = 316; lexer.advance(false); continue; }
                if lookahead == 102 { state = 142; lexer.advance(false); continue; }
                if lookahead == 105 { state = 280; lexer.advance(false); continue; }
                if lookahead == 117 { state = 278; lexer.advance(false); continue; }
                if lookahead == 119 { state = 141; lexer.advance(false); continue; }
                return result;
            }
            201 => {
                if lookahead == 101 { state = 335; lexer.advance(false); continue; }
                return result;
            }
            202 => {
                if lookahead == 101 { state = 223; lexer.advance(false); continue; }
                return result;
            }
            203 => {
                if lookahead == 101 { state = 163; lexer.advance(false); continue; }
                return result;
            }
            204 => {
                if lookahead == 101 { state = 155; lexer.advance(false); continue; }
                return result;
            }
            205 => {
                if lookahead == 101 { state = 354; lexer.advance(false); continue; }
                return result;
            }
            206 => {
                if lookahead == 101 { state = 224; lexer.advance(false); continue; }
                return result;
            }
            207 => {
                if lookahead == 101 { state = 225; lexer.advance(false); continue; }
                return result;
            }
            208 => {
                if lookahead == 101 { state = 228; lexer.advance(false); continue; }
                return result;
            }
            209 => {
                if lookahead == 101 { state = 314; lexer.advance(false); continue; }
                return result;
            }
            210 => {
                if lookahead == 101 { state = 337; lexer.advance(false); continue; }
                if lookahead == 107 { state = 196; lexer.advance(false); continue; }
                return result;
            }
            211 => {
                if lookahead == 101 { state = 317; lexer.advance(false); continue; }
                return result;
            }
            212 => {
                if lookahead == 101 { state = 148; lexer.advance(false); continue; }
                return result;
            }
            213 => {
                if lookahead == 101 { state = 328; lexer.advance(false); continue; }
                return result;
            }
            214 => {
                if lookahead == 101 { state = 173; lexer.advance(false); continue; }
                return result;
            }
            215 => {
                if lookahead == 101 { state = 229; lexer.advance(false); continue; }
                return result;
            }
            216 => {
                if lookahead == 101 { state = 360; lexer.advance(false); continue; }
                return result;
            }
            217 => {
                if lookahead == 101 { state = 156; lexer.advance(false); continue; }
                return result;
            }
            218 => {
                if lookahead == 101 { state = 269; lexer.advance(false); continue; }
                return result;
            }
            219 => {
                if lookahead == 102 { state = 166; lexer.advance(false); continue; }
                if lookahead == 110 { state = 154; lexer.advance(false); continue; }
                return result;
            }
            220 => {
                if lookahead == 102 { state = 140; lexer.advance(false); continue; }
                return result;
            }
            221 => {
                if lookahead == 102 { state = 423; lexer.advance(false); continue; }
                return result;
            }
            222 => {
                if lookahead == 102 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            223 => {
                if lookahead == 102 { state = 421; lexer.advance(false); continue; }
                return result;
            }
            224 => {
                if lookahead == 102 { state = 424; lexer.advance(false); continue; }
                return result;
            }
            225 => {
                if lookahead == 102 { state = 422; lexer.advance(false); continue; }
                return result;
            }
            226 => {
                if let Some(next) = advance_map(&[
                    (102, 347), (104, 302), (110, 298), (112, 211), (114, 190), (115, 210), (116, 243), (117, 278),
                    (119, 239),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            227 => {
                if lookahead == 102 { state = 294; lexer.advance(false); continue; }
                return result;
            }
            228 => {
                if lookahead == 102 { state = 245; lexer.advance(false); continue; }
                return result;
            }
            229 => {
                if lookahead == 102 { state = 246; lexer.advance(false); continue; }
                return result;
            }
            230 => {
                if lookahead == 103 { state = 455; lexer.advance(false); continue; }
                return result;
            }
            231 => {
                if lookahead == 103 { state = 191; lexer.advance(false); continue; }
                return result;
            }
            232 => {
                if lookahead == 103 { state = 238; lexer.advance(false); continue; }
                return result;
            }
            233 => {
                if lookahead == 104 { state = 592; lexer.advance(false); continue; }
                return result;
            }
            234 => {
                if lookahead == 104 { state = 145; lexer.advance(false); continue; }
                return result;
            }
            235 => {
                if lookahead == 105 { state = 271; lexer.advance(false); continue; }
                return result;
            }
            236 => {
                if lookahead == 105 { state = 348; lexer.advance(false); continue; }
                return result;
            }
            237 => {
                if lookahead == 105 { state = 279; lexer.advance(false); continue; }
                return result;
            }
            238 => {
                if lookahead == 105 { state = 274; lexer.advance(false); continue; }
                return result;
            }
            239 => {
                if lookahead == 105 { state = 167; lexer.advance(false); continue; }
                return result;
            }
            240 => {
                if lookahead == 105 { state = 331; lexer.advance(false); continue; }
                return result;
            }
            241 => {
                if lookahead == 105 { state = 275; lexer.advance(false); continue; }
                return result;
            }
            242 => {
                if lookahead == 105 { state = 301; lexer.advance(false); continue; }
                return result;
            }
            243 => {
                if lookahead == 105 { state = 272; lexer.advance(false); continue; }
                return result;
            }
            244 => {
                if lookahead == 105 { state = 349; lexer.advance(false); continue; }
                return result;
            }
            245 => {
                if lookahead == 105 { state = 284; lexer.advance(false); continue; }
                return result;
            }
            246 => {
                if lookahead == 105 { state = 286; lexer.advance(false); continue; }
                return result;
            }
            247 => {
                if lookahead == 107 { state = 182; lexer.advance(false); continue; }
                return result;
            }
            248 => {
                if lookahead == 107 { state = 198; lexer.advance(false); continue; }
                return result;
            }
            249 => {
                if lookahead == 107 { state = 205; lexer.advance(false); continue; }
                return result;
            }
            250 => {
                if lookahead == 107 { state = 216; lexer.advance(false); continue; }
                return result;
            }
            251 => {
                if lookahead == 108 { state = 322; lexer.advance(false); continue; }
                if lookahead == 110 { state = 158; lexer.advance(false); continue; }
                return result;
            }
            252 => {
                if lookahead == 108 { state = 425; lexer.advance(false); continue; }
                return result;
            }
            253 => {
                if lookahead == 108 { state = 426; lexer.advance(false); continue; }
                return result;
            }
            254 => {
                if lookahead == 108 { state = 453; lexer.advance(false); continue; }
                return result;
            }
            255 => {
                if lookahead == 108 { state = 586; lexer.advance(false); continue; }
                return result;
            }
            256 => {
                if lookahead == 108 { state = 267; lexer.advance(false); continue; }
                return result;
            }
            257 => {
                if lookahead == 108 { state = 332; lexer.advance(false); continue; }
                return result;
            }
            258 => {
                if lookahead == 108 { state = 346; lexer.advance(false); continue; }
                return result;
            }
            259 => {
                if lookahead == 108 { state = 252; lexer.advance(false); continue; }
                return result;
            }
            260 => {
                if lookahead == 108 { state = 327; lexer.advance(false); continue; }
                return result;
            }
            261 => {
                if lookahead == 108 { state = 253; lexer.advance(false); continue; }
                return result;
            }
            262 => {
                if lookahead == 108 { state = 159; lexer.advance(false); continue; }
                return result;
            }
            263 => {
                if lookahead == 108 { state = 260; lexer.advance(false); continue; }
                return result;
            }
            264 => {
                if lookahead == 108 { state = 161; lexer.advance(false); continue; }
                return result;
            }
            265 => {
                if lookahead == 108 { state = 184; lexer.advance(false); continue; }
                return result;
            }
            266 => {
                if lookahead == 108 { state = 192; lexer.advance(false); continue; }
                return result;
            }
            267 => {
                if lookahead == 108 { state = 170; lexer.advance(false); continue; }
                return result;
            }
            268 => {
                if lookahead == 108 { state = 174; lexer.advance(false); continue; }
                return result;
            }
            269 => {
                if lookahead == 108 { state = 268; lexer.advance(false); continue; }
                return result;
            }
            270 => {
                if lookahead == 109 { state = 587; lexer.advance(false); continue; }
                return result;
            }
            271 => {
                if lookahead == 109 { state = 197; lexer.advance(false); continue; }
                return result;
            }
            272 => {
                if lookahead == 109 { state = 213; lexer.advance(false); continue; }
                return result;
            }
            273 => {
                if lookahead == 110 { state = 152; lexer.advance(false); continue; }
                return result;
            }
            274 => {
                if lookahead == 110 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            275 => {
                if lookahead == 110 { state = 230; lexer.advance(false); continue; }
                return result;
            }
            276 => {
                if lookahead == 110 { state = 157; lexer.advance(false); continue; }
                return result;
            }
            277 => {
                if lookahead == 110 { state = 231; lexer.advance(false); continue; }
                return result;
            }
            278 => {
                if lookahead == 110 { state = 240; lexer.advance(false); continue; }
                return result;
            }
            279 => {
                if lookahead == 110 { state = 180; lexer.advance(false); continue; }
                return result;
            }
            280 => {
                if lookahead == 110 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            281 => {
                if lookahead == 110 { state = 181; lexer.advance(false); continue; }
                return result;
            }
            282 => {
                if lookahead == 110 { state = 204; lexer.advance(false); continue; }
                return result;
            }
            283 => {
                if lookahead == 110 { state = 212; lexer.advance(false); continue; }
                return result;
            }
            284 => {
                if lookahead == 110 { state = 185; lexer.advance(false); continue; }
                return result;
            }
            285 => {
                if lookahead == 110 { state = 201; lexer.advance(false); continue; }
                return result;
            }
            286 => {
                if lookahead == 110 { state = 186; lexer.advance(false); continue; }
                return result;
            }
            287 => {
                if lookahead == 110 { state = 168; lexer.advance(false); continue; }
                return result;
            }
            288 => {
                if lookahead == 110 { state = 241; lexer.advance(false); continue; }
                return result;
            }
            289 => {
                if lookahead == 110 { state = 282; lexer.advance(false); continue; }
                return result;
            }
            290 => {
                if lookahead == 110 { state = 217; lexer.advance(false); continue; }
                return result;
            }
            291 => {
                if lookahead == 110 { state = 290; lexer.advance(false); continue; }
                return result;
            }
            292 => {
                if lookahead == 111 { state = 342; lexer.advance(false); continue; }
                return result;
            }
            293 => {
                if lookahead == 111 { state = 313; lexer.advance(false); continue; }
                return result;
            }
            294 => {
                if lookahead == 111 { state = 456; lexer.advance(false); continue; }
                return result;
            }
            295 => {
                if lookahead == 111 { state = 351; lexer.advance(false); continue; }
                return result;
            }
            296 => {
                if lookahead == 111 { state = 300; lexer.advance(false); continue; }
                return result;
            }
            297 => {
                if lookahead == 111 { state = 289; lexer.advance(false); continue; }
                return result;
            }
            298 => {
                if lookahead == 111 { state = 150; lexer.advance(false); continue; }
                return result;
            }
            299 => {
                if lookahead == 111 { state = 311; lexer.advance(false); continue; }
                return result;
            }
            300 => {
                if lookahead == 111 { state = 330; lexer.advance(false); continue; }
                return result;
            }
            301 => {
                if lookahead == 111 { state = 160; lexer.advance(false); continue; }
                return result;
            }
            302 => {
                if lookahead == 111 { state = 262; lexer.advance(false); continue; }
                return result;
            }
            303 => {
                if lookahead == 111 { state = 350; lexer.advance(false); continue; }
                if lookahead == 114 { state = 195; lexer.advance(false); continue; }
                return result;
            }
            304 => {
                if lookahead == 111 { state = 264; lexer.advance(false); continue; }
                return result;
            }
            305 => {
                if lookahead == 111 { state = 321; lexer.advance(false); continue; }
                return result;
            }
            306 => {
                if lookahead == 111 { state = 291; lexer.advance(false); continue; }
                return result;
            }
            307 => {
                if lookahead == 112 { state = 479; lexer.advance(false); continue; }
                return result;
            }
            308 => {
                if lookahead == 112 { state = 582; lexer.advance(false); continue; }
                return result;
            }
            309 => {
                if lookahead == 112 { state = 187; lexer.advance(false); continue; }
                return result;
            }
            310 => {
                if lookahead == 114 { state = 702; lexer.advance(false); continue; }
                if lookahead == 117 { state = 700; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 705; lexer.advance(false); continue; }
                return result;
            }
            311 => {
                if lookahead == 114 { state = 454; lexer.advance(false); continue; }
                return result;
            }
            312 => {
                if lookahead == 114 { state = 236; lexer.advance(false); continue; }
                return result;
            }
            313 => {
                if lookahead == 114 { state = 164; lexer.advance(false); continue; }
                return result;
            }
            314 => {
                if lookahead == 114 { state = 358; lexer.advance(false); continue; }
                return result;
            }
            315 => {
                if lookahead == 114 { state = 296; lexer.advance(false); continue; }
                if lookahead == 117 { state = 278; lexer.advance(false); continue; }
                return result;
            }
            316 => {
                if lookahead == 114 { state = 318; lexer.advance(false); continue; }
                return result;
            }
            317 => {
                if lookahead == 114 { state = 242; lexer.advance(false); continue; }
                return result;
            }
            318 => {
                if lookahead == 114 { state = 299; lexer.advance(false); continue; }
                return result;
            }
            319 => {
                if lookahead == 114 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            320 => {
                if lookahead == 114 { state = 244; lexer.advance(false); continue; }
                return result;
            }
            321 => {
                if lookahead == 114 { state = 165; lexer.advance(false); continue; }
                return result;
            }
            322 => {
                if lookahead == 115 { state = 179; lexer.advance(false); continue; }
                return result;
            }
            323 => {
                if lookahead == 115 { state = 432; lexer.advance(false); continue; }
                return result;
            }
            324 => {
                if lookahead == 115 { state = 439; lexer.advance(false); continue; }
                return result;
            }
            325 => {
                if lookahead == 115 { state = 153; lexer.advance(false); continue; }
                return result;
            }
            326 => {
                if lookahead == 115 { state = 193; lexer.advance(false); continue; }
                return result;
            }
            327 => {
                if lookahead == 115 { state = 248; lexer.advance(false); continue; }
                return result;
            }
            328 => {
                if lookahead == 115 { state = 249; lexer.advance(false); continue; }
                return result;
            }
            329 => {
                if lookahead == 116 { state = 147; lexer.advance(false); continue; }
                return result;
            }
            330 => {
                if lookahead == 116 { state = 684; lexer.advance(false); continue; }
                return result;
            }
            331 => {
                if lookahead == 116 { state = 686; lexer.advance(false); continue; }
                return result;
            }
            332 => {
                if lookahead == 116 { state = 136; lexer.advance(false); continue; }
                return result;
            }
            333 => {
                if lookahead == 116 { state = 359; lexer.advance(false); continue; }
                return result;
            }
            334 => {
                if lookahead == 116 { state = 233; lexer.advance(false); continue; }
                return result;
            }
            335 => {
                if lookahead == 116 { state = 333; lexer.advance(false); continue; }
                return result;
            }
            336 => {
                if lookahead == 116 { state = 178; lexer.advance(false); continue; }
                return result;
            }
            337 => {
                if lookahead == 116 { state = 343; lexer.advance(false); continue; }
                return result;
            }
            338 => {
                if lookahead == 116 { state = 139; lexer.advance(false); continue; }
                return result;
            }
            339 => {
                if lookahead == 116 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            340 => {
                if lookahead == 116 { state = 144; lexer.advance(false); continue; }
                return result;
            }
            341 => {
                if lookahead == 116 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            342 => {
                if lookahead == 117 { state = 276; lexer.advance(false); continue; }
                return result;
            }
            343 => {
                if lookahead == 117 { state = 308; lexer.advance(false); continue; }
                return result;
            }
            344 => {
                if lookahead == 117 { state = 257; lexer.advance(false); continue; }
                return result;
            }
            345 => {
                if lookahead == 117 { state = 278; lexer.advance(false); continue; }
                return result;
            }
            346 => {
                if lookahead == 117 { state = 171; lexer.advance(false); continue; }
                return result;
            }
            347 => {
                if lookahead == 117 { state = 263; lexer.advance(false); continue; }
                return result;
            }
            348 => {
                if lookahead == 118 { state = 188; lexer.advance(false); continue; }
                return result;
            }
            349 => {
                if lookahead == 118 { state = 189; lexer.advance(false); continue; }
                return result;
            }
            350 => {
                if lookahead == 118 { state = 209; lexer.advance(false); continue; }
                return result;
            }
            351 => {
                if lookahead == 118 { state = 146; lexer.advance(false); continue; }
                return result;
            }
            352 => {
                if lookahead == 119 { state = 588; lexer.advance(false); continue; }
                return result;
            }
            353 => {
                if lookahead == 119 { state = 590; lexer.advance(false); continue; }
                return result;
            }
            354 => {
                if lookahead == 119 { state = 589; lexer.advance(false); continue; }
                return result;
            }
            355 => {
                if lookahead == 119 { state = 293; lexer.advance(false); continue; }
                return result;
            }
            356 => {
                if lookahead == 119 { state = 305; lexer.advance(false); continue; }
                return result;
            }
            357 => {
                if lookahead == 121 { state = 355; lexer.advance(false); continue; }
                return result;
            }
            358 => {
                if lookahead == 121 { state = 585; lexer.advance(false); continue; }
                return result;
            }
            359 => {
                if lookahead == 121 { state = 309; lexer.advance(false); continue; }
                return result;
            }
            360 => {
                if lookahead == 121 { state = 356; lexer.advance(false); continue; }
                return result;
            }
            361 => {
                if lookahead == 43 || lookahead == 45 { state = 369; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 658; lexer.advance(false); continue; }
                return result;
            }
            362 => {
                if let Some(next) = advance_map(&[
                    (66, 365), (98, 365), (68, 364), (100, 364), (72, 366), (104, 366), (79, 367), (111, 367),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            363 => {
                if let Some(next) = advance_map(&[
                    (66, 365), (98, 365), (68, 364), (100, 364), (72, 366), (104, 366), (79, 367), (111, 367),
                    (83, 362), (115, 362),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            364 => {
                if lookahead == 63 || lookahead == 88 || lookahead == 90 || lookahead == 120 || lookahead == 122 { state = 654; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 364; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 653; lexer.advance(false); continue; }
                return result;
            }
            365 => {
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 365; lexer.advance(false); continue; }
                if lookahead == 48 || lookahead == 49 || lookahead == 63 || lookahead == 88 || lookahead == 90 || lookahead == 120 || lookahead == 122 { state = 655; lexer.advance(false); continue; }
                return result;
            }
            366 => {
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 366; lexer.advance(false); continue; }
                if set_contains(&sym_hex_number_character_set_1, lookahead) { state = 657; lexer.advance(false); continue; }
                return result;
            }
            367 => {
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 367; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 55 || lookahead == 63 || lookahead == 88 || lookahead == 90 || lookahead == 120 || lookahead == 122 { state = 656; lexer.advance(false); continue; }
                return result;
            }
            368 => {
                if 48 <= lookahead && lookahead <= 57 { state = 659; lexer.advance(false); continue; }
                return result;
            }
            369 => {
                if 48 <= lookahead && lookahead <= 57 { state = 658; lexer.advance(false); continue; }
                return result;
            }
            370 => {
                if 48 <= lookahead && lookahead <= 57 { state = 660; lexer.advance(false); continue; }
                return result;
            }
            371 => {
                if eof { state = 374; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 105), (34, 127), (35, 446), (36, 199), (37, 496), (38, 620), (39, 648), (40, 406),
                    (41, 408), (42, 475), (43, 485), (44, 409), (45, 549), (46, 450), (47, 434), (58, 443),
                    (59, 441), (60, 383), (61, 107), (62, 392), (63, 616), (64, 572), (91, 464), (92, 678),
                    (93, 468), (94, 630), (96, 420), (123, 451), (124, 628), (125, 452), (126, 133), (8211, 116),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 371; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 666; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            372 => {
                if eof { state = 374; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 105), (35, 446), (36, 200), (37, 496), (38, 623), (39, 648), (40, 406), (41, 408),
                    (42, 475), (43, 485), (44, 409), (45, 549), (46, 449), (47, 434), (48, 457), (49, 459),
                    (50, 462), (58, 443), (59, 441), (60, 383), (61, 107), (62, 392), (63, 616), (91, 464),
                    (92, 678), (93, 468), (94, 630), (96, 420), (123, 451), (124, 628), (125, 452), (126, 133),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 372; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            373 => {
                if eof { state = 374; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (34, 127), (35, 447), (36, 199), (37, 106), (38, 61), (39, 648), (40, 406), (41, 408),
                    (42, 64), (43, 75), (44, 409), (45, 76), (46, 449), (47, 67), (49, 663), (58, 443),
                    (59, 441), (60, 103), (61, 414), (62, 122), (63, 616), (64, 570), (91, 464), (92, 678),
                    (93, 468), (94, 109), (96, 420), (123, 451), (124, 79), (125, 452),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 373; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 664; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            374 => {
                result = true; lexer.set_result_symbol(ts_builtin_sym_end); lexer.mark_end();
                return result;
            }
            375 => {
                result = true; lexer.set_result_symbol(anon_sym_DQUOTE); lexer.mark_end();
                return result;
            }
            376 => {
                result = true; lexer.set_result_symbol(aux_sym_double_quoted_string_token1); lexer.mark_end();
                if lookahead == 42 { state = 378; lexer.advance(false); continue; }
                if lookahead == 47 { state = 380; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 34 && lookahead != 92 { state = 380; lexer.advance(false); continue; }
                return result;
            }
            377 => {
                result = true; lexer.set_result_symbol(aux_sym_double_quoted_string_token1); lexer.mark_end();
                if lookahead == 42 { state = 377; lexer.advance(false); continue; }
                if lookahead == 47 { state = 380; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 34 && lookahead != 92 { state = 378; lexer.advance(false); continue; }
                return result;
            }
            378 => {
                result = true; lexer.set_result_symbol(aux_sym_double_quoted_string_token1); lexer.mark_end();
                if lookahead == 42 { state = 377; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 34 && lookahead != 92 { state = 378; lexer.advance(false); continue; }
                return result;
            }
            379 => {
                result = true; lexer.set_result_symbol(aux_sym_double_quoted_string_token1); lexer.mark_end();
                if lookahead == 47 { state = 376; lexer.advance(false); continue; }
                if lookahead == 9 || 11 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 379; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 34 && lookahead != 92 { state = 380; lexer.advance(false); continue; }
                return result;
            }
            380 => {
                result = true; lexer.set_result_symbol(aux_sym_double_quoted_string_token1); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 && lookahead != 34 && lookahead != 92 { state = 380; lexer.advance(false); continue; }
                return result;
            }
            381 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                return result;
            }
            382 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 45 { state = 117; lexer.advance(false); continue; }
                if lookahead == 60 { state = 612; lexer.advance(false); continue; }
                if lookahead == 61 { state = 502; lexer.advance(false); continue; }
                return result;
            }
            383 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 45 { state = 117; lexer.advance(false); continue; }
                if lookahead == 60 { state = 613; lexer.advance(false); continue; }
                if lookahead == 61 { state = 502; lexer.advance(false); continue; }
                return result;
            }
            384 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 61 { state = 502; lexer.advance(false); continue; }
                return result;
            }
            385 => {
                result = true; lexer.set_result_symbol(aux_sym_include_compiler_directive_standard_token1); lexer.mark_end();
                if lookahead == 42 { state = 387; lexer.advance(false); continue; }
                if lookahead == 47 { state = 389; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 62 && lookahead != 92 { state = 389; lexer.advance(false); continue; }
                return result;
            }
            386 => {
                result = true; lexer.set_result_symbol(aux_sym_include_compiler_directive_standard_token1); lexer.mark_end();
                if lookahead == 42 { state = 386; lexer.advance(false); continue; }
                if lookahead == 47 { state = 389; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 62 && lookahead != 92 { state = 387; lexer.advance(false); continue; }
                return result;
            }
            387 => {
                result = true; lexer.set_result_symbol(aux_sym_include_compiler_directive_standard_token1); lexer.mark_end();
                if lookahead == 42 { state = 386; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 62 && lookahead != 92 { state = 387; lexer.advance(false); continue; }
                return result;
            }
            388 => {
                result = true; lexer.set_result_symbol(aux_sym_include_compiler_directive_standard_token1); lexer.mark_end();
                if lookahead == 47 { state = 385; lexer.advance(false); continue; }
                if lookahead == 9 || 11 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 388; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 62 && lookahead != 92 { state = 389; lexer.advance(false); continue; }
                return result;
            }
            389 => {
                result = true; lexer.set_result_symbol(aux_sym_include_compiler_directive_standard_token1); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 && lookahead != 62 && lookahead != 92 { state = 389; lexer.advance(false); continue; }
                return result;
            }
            390 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                if lookahead == 61 { state = 503; lexer.advance(false); continue; }
                return result;
            }
            391 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                if lookahead == 61 { state = 503; lexer.advance(false); continue; }
                if lookahead == 62 { state = 609; lexer.advance(false); continue; }
                return result;
            }
            392 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                if lookahead == 61 { state = 503; lexer.advance(false); continue; }
                if lookahead == 62 { state = 610; lexer.advance(false); continue; }
                return result;
            }
            393 => {
                result = true; lexer.set_result_symbol(aux_sym_include_compiler_directive_token1); lexer.mark_end();
                return result;
            }
            394 => {
                result = true; lexer.set_result_symbol(sym_default_text); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 394; lexer.advance(false); continue; }
                return result;
            }
            395 => {
                result = true; lexer.set_result_symbol(sym_macro_text); lexer.mark_end();
                if lookahead == 10 { state = 71; lexer.advance(false); continue; }
                if lookahead == 42 { state = 395; lexer.advance(false); continue; }
                if lookahead == 47 { state = 402; lexer.advance(false); continue; }
                if lookahead == 92 { state = 6; lexer.advance(false); continue; }
                if lookahead != 0 { state = 396; lexer.advance(false); continue; }
                return result;
            }
            396 => {
                result = true; lexer.set_result_symbol(sym_macro_text); lexer.mark_end();
                if lookahead == 10 { state = 71; lexer.advance(false); continue; }
                if lookahead == 42 { state = 395; lexer.advance(false); continue; }
                if lookahead == 92 { state = 6; lexer.advance(false); continue; }
                if lookahead != 0 { state = 396; lexer.advance(false); continue; }
                return result;
            }
            397 => {
                result = true; lexer.set_result_symbol(sym_macro_text); lexer.mark_end();
                if lookahead == 10 { state = 416; lexer.advance(false); continue; }
                if lookahead == 40 { state = 407; lexer.advance(false); continue; }
                if lookahead == 47 { state = 400; lexer.advance(false); continue; }
                if lookahead == 92 { state = 5; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 397; lexer.advance(false); continue; }
                if lookahead != 0 { state = 402; lexer.advance(false); continue; }
                return result;
            }
            398 => {
                result = true; lexer.set_result_symbol(sym_macro_text); lexer.mark_end();
                if lookahead == 10 { state = 402; lexer.advance(false); continue; }
                if lookahead == 92 { state = 673; lexer.advance(false); continue; }
                if lookahead != 0 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            399 => {
                result = true; lexer.set_result_symbol(sym_macro_text); lexer.mark_end();
                if lookahead == 10 { state = 418; lexer.advance(false); continue; }
                if lookahead == 47 { state = 400; lexer.advance(false); continue; }
                if lookahead == 92 { state = 5; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 399; lexer.advance(false); continue; }
                if lookahead != 0 { state = 402; lexer.advance(false); continue; }
                return result;
            }
            400 => {
                result = true; lexer.set_result_symbol(sym_macro_text); lexer.mark_end();
                if lookahead == 42 { state = 396; lexer.advance(false); continue; }
                if lookahead == 47 { state = 404; lexer.advance(false); continue; }
                if lookahead == 92 { state = 5; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 402; lexer.advance(false); continue; }
                return result;
            }
            401 => {
                result = true; lexer.set_result_symbol(sym_macro_text); lexer.mark_end();
                if lookahead == 42 { state = 395; lexer.advance(false); continue; }
                if lookahead == 92 { state = 6; lexer.advance(false); continue; }
                if lookahead != 0 { state = 396; lexer.advance(false); continue; }
                return result;
            }
            402 => {
                result = true; lexer.set_result_symbol(sym_macro_text); lexer.mark_end();
                if lookahead == 92 { state = 5; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 402; lexer.advance(false); continue; }
                return result;
            }
            403 => {
                result = true; lexer.set_result_symbol(sym_macro_text); lexer.mark_end();
                if lookahead == 92 { state = 5; lexer.advance(false); continue; }
                if lookahead != 0 { state = 402; lexer.advance(false); continue; }
                return result;
            }
            404 => {
                result = true; lexer.set_result_symbol(sym_macro_text); lexer.mark_end();
                if lookahead == 92 { state = 673; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            405 => {
                result = true; lexer.set_result_symbol(anon_sym_LPAREN); lexer.mark_end();
                return result;
            }
            406 => {
                result = true; lexer.set_result_symbol(anon_sym_LPAREN); lexer.mark_end();
                if lookahead == 42 { state = 670; lexer.advance(false); continue; }
                return result;
            }
            407 => {
                result = true; lexer.set_result_symbol(anon_sym_LPAREN); lexer.mark_end();
                if lookahead == 92 { state = 5; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 402; lexer.advance(false); continue; }
                return result;
            }
            408 => {
                result = true; lexer.set_result_symbol(anon_sym_RPAREN); lexer.mark_end();
                return result;
            }
            409 => {
                result = true; lexer.set_result_symbol(anon_sym_COMMA); lexer.mark_end();
                return result;
            }
            410 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                return result;
            }
            411 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                if lookahead == 61 { state = 499; lexer.advance(false); continue; }
                return result;
            }
            412 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                if lookahead == 61 { state = 499; lexer.advance(false); continue; }
                if lookahead == 62 { state = 519; lexer.advance(false); continue; }
                return result;
            }
            413 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                if lookahead == 61 { state = 498; lexer.advance(false); continue; }
                return result;
            }
            414 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                if lookahead == 62 { state = 519; lexer.advance(false); continue; }
                return result;
            }
            415 => {
                result = true; lexer.set_result_symbol(aux_sym_text_macro_definition_token1); lexer.mark_end();
                return result;
            }
            416 => {
                result = true; lexer.set_result_symbol(anon_sym_LF); lexer.mark_end();
                if lookahead == 10 { state = 416; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 397; lexer.advance(false); continue; }
                return result;
            }
            417 => {
                result = true; lexer.set_result_symbol(anon_sym_LF); lexer.mark_end();
                if lookahead == 10 { state = 417; lexer.advance(false); continue; }
                return result;
            }
            418 => {
                result = true; lexer.set_result_symbol(anon_sym_LF); lexer.mark_end();
                if lookahead == 10 { state = 418; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 399; lexer.advance(false); continue; }
                return result;
            }
            419 => {
                result = true; lexer.set_result_symbol(anon_sym_BQUOTE); lexer.mark_end();
                return result;
            }
            420 => {
                result = true; lexer.set_result_symbol(anon_sym_BQUOTE); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (98, 175), (99, 176), (100, 177), (101, 251), (105, 219), (108, 237), (110, 292), (114, 194),
                    (116, 235), (117, 273),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            421 => {
                result = true; lexer.set_result_symbol(aux_sym_id_directive_token1); lexer.mark_end();
                return result;
            }
            422 => {
                result = true; lexer.set_result_symbol(aux_sym_id_directive_token2); lexer.mark_end();
                return result;
            }
            423 => {
                result = true; lexer.set_result_symbol(aux_sym_id_directive_token3); lexer.mark_end();
                return result;
            }
            424 => {
                result = true; lexer.set_result_symbol(aux_sym_id_directive_token4); lexer.mark_end();
                if lookahead == 105 { state = 283; lexer.advance(false); continue; }
                return result;
            }
            425 => {
                result = true; lexer.set_result_symbol(aux_sym_zero_directive_token1); lexer.mark_end();
                return result;
            }
            426 => {
                result = true; lexer.set_result_symbol(aux_sym_zero_directive_token2); lexer.mark_end();
                return result;
            }
            427 => {
                result = true; lexer.set_result_symbol(aux_sym_zero_directive_token3); lexer.mark_end();
                return result;
            }
            428 => {
                result = true; lexer.set_result_symbol(aux_sym_zero_directive_token4); lexer.mark_end();
                return result;
            }
            429 => {
                result = true; lexer.set_result_symbol(aux_sym_zero_directive_token5); lexer.mark_end();
                return result;
            }
            430 => {
                result = true; lexer.set_result_symbol(aux_sym_zero_directive_token6); lexer.mark_end();
                return result;
            }
            431 => {
                result = true; lexer.set_result_symbol(aux_sym_zero_directive_token7); lexer.mark_end();
                return result;
            }
            432 => {
                result = true; lexer.set_result_symbol(aux_sym_zero_directive_token8); lexer.mark_end();
                return result;
            }
            433 => {
                result = true; lexer.set_result_symbol(aux_sym_timescale_compiler_directive_token1); lexer.mark_end();
                return result;
            }
            434 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH); lexer.mark_end();
                if lookahead == 42 { state = 71; lexer.advance(false); continue; }
                if lookahead == 47 { state = 676; lexer.advance(false); continue; }
                return result;
            }
            435 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH); lexer.mark_end();
                if lookahead == 42 { state = 71; lexer.advance(false); continue; }
                if lookahead == 47 { state = 676; lexer.advance(false); continue; }
                if lookahead == 61 { state = 560; lexer.advance(false); continue; }
                return result;
            }
            436 => {
                result = true; lexer.set_result_symbol(aux_sym_default_nettype_compiler_directive_token1); lexer.mark_end();
                return result;
            }
            437 => {
                result = true; lexer.set_result_symbol(aux_sym_unconnected_drive_token1); lexer.mark_end();
                return result;
            }
            438 => {
                result = true; lexer.set_result_symbol(aux_sym_line_compiler_directive_token1); lexer.mark_end();
                return result;
            }
            439 => {
                result = true; lexer.set_result_symbol(aux_sym_begin_keywords_token1); lexer.mark_end();
                return result;
            }
            440 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT_STAR); lexer.mark_end();
                return result;
            }
            441 => {
                result = true; lexer.set_result_symbol(anon_sym_SEMI); lexer.mark_end();
                return result;
            }
            442 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON); lexer.mark_end();
                return result;
            }
            443 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON); lexer.mark_end();
                if lookahead == 58 { state = 471; lexer.advance(false); continue; }
                return result;
            }
            444 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON); lexer.mark_end();
                if lookahead == 58 { state = 471; lexer.advance(false); continue; }
                if lookahead == 61 { state = 469; lexer.advance(false); continue; }
                return result;
            }
            445 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND); lexer.mark_end();
                return result;
            }
            446 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND); lexer.mark_end();
                if lookahead == 35 { state = 509; lexer.advance(false); continue; }
                return result;
            }
            447 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND); lexer.mark_end();
                if lookahead == 35 { state = 509; lexer.advance(false); continue; }
                if lookahead == 45 { state = 57; lexer.advance(false); continue; }
                if lookahead == 61 { state = 58; lexer.advance(false); continue; }
                return result;
            }
            448 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND); lexer.mark_end();
                if lookahead == 35 { state = 508; lexer.advance(false); continue; }
                return result;
            }
            449 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                return result;
            }
            450 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                if lookahead == 42 { state = 440; lexer.advance(false); continue; }
                return result;
            }
            451 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACE); lexer.mark_end();
                return result;
            }
            452 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACE); lexer.mark_end();
                return result;
            }
            453 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLARfatal); lexer.mark_end();
                return result;
            }
            454 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLARerror); lexer.mark_end();
                return result;
            }
            455 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLARwarning); lexer.mark_end();
                return result;
            }
            456 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLARinfo); lexer.mark_end();
                return result;
            }
            457 => {
                result = true; lexer.set_result_symbol(anon_sym_0); lexer.mark_end();
                return result;
            }
            458 => {
                result = true; lexer.set_result_symbol(anon_sym_0); lexer.mark_end();
                if lookahead == 46 { state = 368; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 361; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || lookahead == 95 { state = 664; lexer.advance(false); continue; }
                return result;
            }
            459 => {
                result = true; lexer.set_result_symbol(anon_sym_1); lexer.mark_end();
                if lookahead == 39 { state = 124; lexer.advance(false); continue; }
                return result;
            }
            460 => {
                result = true; lexer.set_result_symbol(anon_sym_1); lexer.mark_end();
                if lookahead == 39 { state = 124; lexer.advance(false); continue; }
                if lookahead == 115 { state = 336; lexer.advance(false); continue; }
                return result;
            }
            461 => {
                result = true; lexer.set_result_symbol(anon_sym_1); lexer.mark_end();
                if lookahead == 39 { state = 125; lexer.advance(false); continue; }
                if lookahead == 46 { state = 368; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 361; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 62; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || lookahead == 95 { state = 661; lexer.advance(false); continue; }
                return result;
            }
            462 => {
                result = true; lexer.set_result_symbol(anon_sym_2); lexer.mark_end();
                return result;
            }
            463 => {
                result = true; lexer.set_result_symbol(anon_sym_u2013_GT); lexer.mark_end();
                return result;
            }
            464 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK); lexer.mark_end();
                return result;
            }
            465 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK); lexer.mark_end();
                if lookahead == 42 { state = 513; lexer.advance(false); continue; }
                if lookahead == 43 { state = 130; lexer.advance(false); continue; }
                if lookahead == 45 { state = 118; lexer.advance(false); continue; }
                if lookahead == 61 { state = 516; lexer.advance(false); continue; }
                return result;
            }
            466 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK); lexer.mark_end();
                if lookahead == 42 { state = 513; lexer.advance(false); continue; }
                if lookahead == 43 { state = 130; lexer.advance(false); continue; }
                if lookahead == 45 { state = 118; lexer.advance(false); continue; }
                if lookahead == 61 { state = 516; lexer.advance(false); continue; }
                if lookahead == 8211 { state = 119; lexer.advance(false); continue; }
                return result;
            }
            467 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK); lexer.mark_end();
                if lookahead == 42 { state = 512; lexer.advance(false); continue; }
                if lookahead == 61 { state = 516; lexer.advance(false); continue; }
                if lookahead == 8211 { state = 119; lexer.advance(false); continue; }
                return result;
            }
            468 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACK); lexer.mark_end();
                return result;
            }
            469 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON_EQ); lexer.mark_end();
                return result;
            }
            470 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON_SLASH); lexer.mark_end();
                return result;
            }
            471 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON_COLON); lexer.mark_end();
                return result;
            }
            472 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                return result;
            }
            473 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                if lookahead == 41 { state = 671; lexer.advance(false); continue; }
                if lookahead == 42 { state = 495; lexer.advance(false); continue; }
                return result;
            }
            474 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                if lookahead == 41 { state = 671; lexer.advance(false); continue; }
                if lookahead == 42 { state = 495; lexer.advance(false); continue; }
                if lookahead == 61 { state = 559; lexer.advance(false); continue; }
                if lookahead == 62 { state = 581; lexer.advance(false); continue; }
                return result;
            }
            475 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                if lookahead == 42 { state = 495; lexer.advance(false); continue; }
                return result;
            }
            476 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                if lookahead == 42 { state = 495; lexer.advance(false); continue; }
                if lookahead == 61 { state = 559; lexer.advance(false); continue; }
                return result;
            }
            477 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                if lookahead == 42 { state = 495; lexer.advance(false); continue; }
                if lookahead == 62 { state = 581; lexer.advance(false); continue; }
                return result;
            }
            478 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR_COLON_COLON_STAR); lexer.mark_end();
                return result;
            }
            479 => {
                result = true; lexer.set_result_symbol(anon_sym_1step); lexer.mark_end();
                return result;
            }
            480 => {
                result = true; lexer.set_result_symbol(anon_sym_PATHPULSE_DOLLAR_EQ); lexer.mark_end();
                return result;
            }
            481 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR); lexer.mark_end();
                if lookahead == 114 { state = 702; lexer.advance(false); continue; }
                if lookahead == 117 { state = 700; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 705; lexer.advance(false); continue; }
                return result;
            }
            482 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR); lexer.mark_end();
                if lookahead == 117 { state = 278; lexer.advance(false); continue; }
                return result;
            }
            483 => {
                result = true; lexer.set_result_symbol(anon_sym_DQUOTEDPI_DASHC_DQUOTE); lexer.mark_end();
                return result;
            }
            484 => {
                result = true; lexer.set_result_symbol(anon_sym_DQUOTEDPI_DQUOTE); lexer.mark_end();
                return result;
            }
            485 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                return result;
            }
            486 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                if lookahead == 43 { state = 491; lexer.advance(false); continue; }
                return result;
            }
            487 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                if lookahead == 43 { state = 491; lexer.advance(false); continue; }
                if lookahead == 58 { state = 614; lexer.advance(false); continue; }
                return result;
            }
            488 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                if lookahead == 43 { state = 491; lexer.advance(false); continue; }
                if lookahead == 58 { state = 614; lexer.advance(false); continue; }
                if lookahead == 61 { state = 557; lexer.advance(false); continue; }
                return result;
            }
            489 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                if lookahead == 43 { state = 491; lexer.advance(false); continue; }
                if lookahead == 61 { state = 557; lexer.advance(false); continue; }
                return result;
            }
            490 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                if lookahead == 58 { state = 614; lexer.advance(false); continue; }
                return result;
            }
            491 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS_PLUS); lexer.mark_end();
                return result;
            }
            492 => {
                result = true; lexer.set_result_symbol(anon_sym_u2013); lexer.mark_end();
                if lookahead == 62 { state = 463; lexer.advance(false); continue; }
                if lookahead == 8211 { state = 494; lexer.advance(false); continue; }
                return result;
            }
            493 => {
                result = true; lexer.set_result_symbol(anon_sym_u2013); lexer.mark_end();
                if lookahead == 8211 { state = 494; lexer.advance(false); continue; }
                return result;
            }
            494 => {
                result = true; lexer.set_result_symbol(anon_sym_u2013u2013); lexer.mark_end();
                return result;
            }
            495 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR_STAR); lexer.mark_end();
                return result;
            }
            496 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT); lexer.mark_end();
                return result;
            }
            497 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT); lexer.mark_end();
                if lookahead == 61 { state = 561; lexer.advance(false); continue; }
                return result;
            }
            498 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ_EQ); lexer.mark_end();
                return result;
            }
            499 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ_EQ); lexer.mark_end();
                if lookahead == 61 { state = 598; lexer.advance(false); continue; }
                if lookahead == 63 { state = 617; lexer.advance(false); continue; }
                return result;
            }
            500 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG_EQ); lexer.mark_end();
                return result;
            }
            501 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG_EQ); lexer.mark_end();
                if lookahead == 61 { state = 599; lexer.advance(false); continue; }
                if lookahead == 63 { state = 618; lexer.advance(false); continue; }
                return result;
            }
            502 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_EQ); lexer.mark_end();
                return result;
            }
            503 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_EQ); lexer.mark_end();
                return result;
            }
            504 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE_DASH_GT); lexer.mark_end();
                return result;
            }
            505 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE_EQ_GT); lexer.mark_end();
                return result;
            }
            506 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND_DASH_POUND); lexer.mark_end();
                return result;
            }
            507 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND_EQ_POUND); lexer.mark_end();
                return result;
            }
            508 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND_POUND); lexer.mark_end();
                return result;
            }
            509 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND_POUND); lexer.mark_end();
                if lookahead == 91 { state = 74; lexer.advance(false); continue; }
                return result;
            }
            510 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND_POUND_LBRACK_STAR_RBRACK); lexer.mark_end();
                return result;
            }
            511 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND_POUND_LBRACK_PLUS_RBRACK); lexer.mark_end();
                return result;
            }
            512 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK_STAR); lexer.mark_end();
                return result;
            }
            513 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK_STAR); lexer.mark_end();
                if lookahead == 93 { state = 514; lexer.advance(false); continue; }
                return result;
            }
            514 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK_STAR_RBRACK); lexer.mark_end();
                return result;
            }
            515 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK_PLUS_RBRACK); lexer.mark_end();
                return result;
            }
            516 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK_EQ); lexer.mark_end();
                return result;
            }
            517 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK_DASH_GT); lexer.mark_end();
                return result;
            }
            518 => {
                result = true; lexer.set_result_symbol(anon_sym_AT_AT); lexer.mark_end();
                return result;
            }
            519 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ_GT); lexer.mark_end();
                return result;
            }
            520 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACKu2013_GT); lexer.mark_end();
                return result;
            }
            521 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG); lexer.mark_end();
                return result;
            }
            522 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG); lexer.mark_end();
                if lookahead == 61 { state = 501; lexer.advance(false); continue; }
                return result;
            }
            523 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP_AMP); lexer.mark_end();
                return result;
            }
            524 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP_AMP); lexer.mark_end();
                if lookahead == 38 { state = 578; lexer.advance(false); continue; }
                return result;
            }
            525 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE_PIPE); lexer.mark_end();
                return result;
            }
            526 => {
                result = true; lexer.set_result_symbol(anon_sym_endtable); lexer.mark_end();
                return result;
            }
            527 => {
                result = true; lexer.set_result_symbol(anon_sym_1_SQUOTEb0); lexer.mark_end();
                return result;
            }
            528 => {
                result = true; lexer.set_result_symbol(anon_sym_1_SQUOTEb0); lexer.mark_end();
                if lookahead == 48 || lookahead == 49 || lookahead == 63 || lookahead == 88 || lookahead == 90 || lookahead == 95 || lookahead == 120 || lookahead == 122 { state = 655; lexer.advance(false); continue; }
                return result;
            }
            529 => {
                result = true; lexer.set_result_symbol(anon_sym_1_SQUOTEb1); lexer.mark_end();
                return result;
            }
            530 => {
                result = true; lexer.set_result_symbol(anon_sym_1_SQUOTEb1); lexer.mark_end();
                if lookahead == 48 || lookahead == 49 || lookahead == 63 || lookahead == 88 || lookahead == 90 || lookahead == 95 || lookahead == 120 || lookahead == 122 { state = 655; lexer.advance(false); continue; }
                return result;
            }
            531 => {
                result = true; lexer.set_result_symbol(anon_sym_1_SQUOTEbx); lexer.mark_end();
                return result;
            }
            532 => {
                result = true; lexer.set_result_symbol(anon_sym_1_SQUOTEbX); lexer.mark_end();
                return result;
            }
            533 => {
                result = true; lexer.set_result_symbol(anon_sym_1_SQUOTEB0); lexer.mark_end();
                return result;
            }
            534 => {
                result = true; lexer.set_result_symbol(anon_sym_1_SQUOTEB0); lexer.mark_end();
                if lookahead == 48 || lookahead == 49 || lookahead == 63 || lookahead == 88 || lookahead == 90 || lookahead == 95 || lookahead == 120 || lookahead == 122 { state = 655; lexer.advance(false); continue; }
                return result;
            }
            535 => {
                result = true; lexer.set_result_symbol(anon_sym_1_SQUOTEB1); lexer.mark_end();
                return result;
            }
            536 => {
                result = true; lexer.set_result_symbol(anon_sym_1_SQUOTEB1); lexer.mark_end();
                if lookahead == 48 || lookahead == 49 || lookahead == 63 || lookahead == 88 || lookahead == 90 || lookahead == 95 || lookahead == 120 || lookahead == 122 { state = 655; lexer.advance(false); continue; }
                return result;
            }
            537 => {
                result = true; lexer.set_result_symbol(anon_sym_1_SQUOTEBx); lexer.mark_end();
                return result;
            }
            538 => {
                result = true; lexer.set_result_symbol(anon_sym_1_SQUOTEBX); lexer.mark_end();
                return result;
            }
            539 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                return result;
            }
            540 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 652; lexer.advance(false); continue; }
                return result;
            }
            541 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 652; lexer.advance(false); continue; }
                if lookahead == 58 { state = 615; lexer.advance(false); continue; }
                if lookahead == 61 { state = 558; lexer.advance(false); continue; }
                if lookahead == 62 { state = 576; lexer.advance(false); continue; }
                return result;
            }
            542 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 652; lexer.advance(false); continue; }
                if lookahead == 58 { state = 615; lexer.advance(false); continue; }
                if lookahead == 62 { state = 576; lexer.advance(false); continue; }
                return result;
            }
            543 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 652; lexer.advance(false); continue; }
                if lookahead == 58 { state = 615; lexer.advance(false); continue; }
                if lookahead == 62 { state = 575; lexer.advance(false); continue; }
                return result;
            }
            544 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 652; lexer.advance(false); continue; }
                if lookahead == 61 { state = 558; lexer.advance(false); continue; }
                if lookahead == 62 { state = 576; lexer.advance(false); continue; }
                return result;
            }
            545 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 652; lexer.advance(false); continue; }
                if lookahead == 61 { state = 558; lexer.advance(false); continue; }
                if lookahead == 62 { state = 575; lexer.advance(false); continue; }
                return result;
            }
            546 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 652; lexer.advance(false); continue; }
                if lookahead == 62 { state = 576; lexer.advance(false); continue; }
                return result;
            }
            547 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 652; lexer.advance(false); continue; }
                if lookahead == 62 { state = 575; lexer.advance(false); continue; }
                return result;
            }
            548 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 58 { state = 615; lexer.advance(false); continue; }
                if lookahead == 62 { state = 575; lexer.advance(false); continue; }
                return result;
            }
            549 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 62 { state = 575; lexer.advance(false); continue; }
                return result;
            }
            550 => {
                result = true; lexer.set_result_symbol(sym_output_symbol); lexer.mark_end();
                return result;
            }
            551 => {
                result = true; lexer.set_result_symbol(sym_output_symbol); lexer.mark_end();
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            552 => {
                result = true; lexer.set_result_symbol(sym_level_symbol); lexer.mark_end();
                return result;
            }
            553 => {
                result = true; lexer.set_result_symbol(sym_level_symbol); lexer.mark_end();
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            554 => {
                result = true; lexer.set_result_symbol(sym_edge_symbol); lexer.mark_end();
                return result;
            }
            555 => {
                result = true; lexer.set_result_symbol(sym_edge_symbol); lexer.mark_end();
                if lookahead == 65 { state = 696; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 66 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            556 => {
                result = true; lexer.set_result_symbol(sym_edge_symbol); lexer.mark_end();
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            557 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS_EQ); lexer.mark_end();
                return result;
            }
            558 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_EQ); lexer.mark_end();
                return result;
            }
            559 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR_EQ); lexer.mark_end();
                return result;
            }
            560 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_EQ); lexer.mark_end();
                return result;
            }
            561 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT_EQ); lexer.mark_end();
                return result;
            }
            562 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP_EQ); lexer.mark_end();
                return result;
            }
            563 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE_EQ); lexer.mark_end();
                return result;
            }
            564 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE_EQ); lexer.mark_end();
                if lookahead == 62 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            565 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET_EQ); lexer.mark_end();
                return result;
            }
            566 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT_EQ); lexer.mark_end();
                return result;
            }
            567 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT_EQ); lexer.mark_end();
                return result;
            }
            568 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT_LT_EQ); lexer.mark_end();
                return result;
            }
            569 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT_GT_EQ); lexer.mark_end();
                return result;
            }
            570 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                return result;
            }
            571 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                if lookahead == 42 { state = 574; lexer.advance(false); continue; }
                return result;
            }
            572 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                if lookahead == 42 { state = 574; lexer.advance(false); continue; }
                if lookahead == 64 { state = 518; lexer.advance(false); continue; }
                return result;
            }
            573 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                if lookahead == 64 { state = 518; lexer.advance(false); continue; }
                return result;
            }
            574 => {
                result = true; lexer.set_result_symbol(anon_sym_AT_STAR); lexer.mark_end();
                return result;
            }
            575 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_GT); lexer.mark_end();
                return result;
            }
            576 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_GT); lexer.mark_end();
                if lookahead == 62 { state = 577; lexer.advance(false); continue; }
                return result;
            }
            577 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_GT_GT); lexer.mark_end();
                return result;
            }
            578 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP_AMP_AMP); lexer.mark_end();
                return result;
            }
            579 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTE_LBRACE); lexer.mark_end();
                return result;
            }
            580 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND0); lexer.mark_end();
                return result;
            }
            581 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR_GT); lexer.mark_end();
                return result;
            }
            582 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLARsetup); lexer.mark_end();
                if lookahead == 104 { state = 304; lexer.advance(false); continue; }
                return result;
            }
            583 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLARhold); lexer.mark_end();
                return result;
            }
            584 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLARsetuphold); lexer.mark_end();
                return result;
            }
            585 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLARrecovery); lexer.mark_end();
                return result;
            }
            586 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLARremoval); lexer.mark_end();
                return result;
            }
            587 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLARrecrem); lexer.mark_end();
                return result;
            }
            588 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLARskew); lexer.mark_end();
                return result;
            }
            589 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLARtimeskew); lexer.mark_end();
                return result;
            }
            590 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLARfullskew); lexer.mark_end();
                return result;
            }
            591 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLARperiod); lexer.mark_end();
                return result;
            }
            592 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLARwidth); lexer.mark_end();
                return result;
            }
            593 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLARnochange); lexer.mark_end();
                return result;
            }
            594 => {
                result = true; lexer.set_result_symbol(anon_sym_01); lexer.mark_end();
                return result;
            }
            595 => {
                result = true; lexer.set_result_symbol(anon_sym_10); lexer.mark_end();
                return result;
            }
            596 => {
                result = true; lexer.set_result_symbol(aux_sym_edge_descriptor_token2); lexer.mark_end();
                return result;
            }
            597 => {
                result = true; lexer.set_result_symbol(anon_sym_TILDE); lexer.mark_end();
                if lookahead == 38 { state = 650; lexer.advance(false); continue; }
                if lookahead == 94 { state = 632; lexer.advance(false); continue; }
                if lookahead == 124 { state = 651; lexer.advance(false); continue; }
                return result;
            }
            598 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ_EQ_EQ); lexer.mark_end();
                return result;
            }
            599 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG_EQ_EQ); lexer.mark_end();
                return result;
            }
            600 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTEb0); lexer.mark_end();
                return result;
            }
            601 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTEb0); lexer.mark_end();
                if lookahead == 48 || lookahead == 49 || lookahead == 63 || lookahead == 88 || lookahead == 90 || lookahead == 95 || lookahead == 120 || lookahead == 122 { state = 655; lexer.advance(false); continue; }
                return result;
            }
            602 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTEb1); lexer.mark_end();
                return result;
            }
            603 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTEb1); lexer.mark_end();
                if lookahead == 48 || lookahead == 49 || lookahead == 63 || lookahead == 88 || lookahead == 90 || lookahead == 95 || lookahead == 120 || lookahead == 122 { state = 655; lexer.advance(false); continue; }
                return result;
            }
            604 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTEB0); lexer.mark_end();
                return result;
            }
            605 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTEB0); lexer.mark_end();
                if lookahead == 48 || lookahead == 49 || lookahead == 63 || lookahead == 88 || lookahead == 90 || lookahead == 95 || lookahead == 120 || lookahead == 122 { state = 655; lexer.advance(false); continue; }
                return result;
            }
            606 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTEB1); lexer.mark_end();
                return result;
            }
            607 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTEB1); lexer.mark_end();
                if lookahead == 48 || lookahead == 49 || lookahead == 63 || lookahead == 88 || lookahead == 90 || lookahead == 95 || lookahead == 120 || lookahead == 122 { state = 655; lexer.advance(false); continue; }
                return result;
            }
            608 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT); lexer.mark_end();
                return result;
            }
            609 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT); lexer.mark_end();
                if lookahead == 61 { state = 567; lexer.advance(false); continue; }
                if lookahead == 62 { state = 634; lexer.advance(false); continue; }
                return result;
            }
            610 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT); lexer.mark_end();
                if lookahead == 62 { state = 633; lexer.advance(false); continue; }
                return result;
            }
            611 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT); lexer.mark_end();
                return result;
            }
            612 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT); lexer.mark_end();
                if lookahead == 60 { state = 636; lexer.advance(false); continue; }
                if lookahead == 61 { state = 566; lexer.advance(false); continue; }
                return result;
            }
            613 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT); lexer.mark_end();
                if lookahead == 60 { state = 635; lexer.advance(false); continue; }
                return result;
            }
            614 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS_COLON); lexer.mark_end();
                return result;
            }
            615 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_COLON); lexer.mark_end();
                return result;
            }
            616 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK); lexer.mark_end();
                return result;
            }
            617 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ_EQ_QMARK); lexer.mark_end();
                return result;
            }
            618 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG_EQ_QMARK); lexer.mark_end();
                return result;
            }
            619 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP); lexer.mark_end();
                return result;
            }
            620 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP); lexer.mark_end();
                if lookahead == 38 { state = 524; lexer.advance(false); continue; }
                return result;
            }
            621 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP); lexer.mark_end();
                if lookahead == 38 { state = 524; lexer.advance(false); continue; }
                if lookahead == 61 { state = 562; lexer.advance(false); continue; }
                return result;
            }
            622 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP); lexer.mark_end();
                if lookahead == 38 { state = 60; lexer.advance(false); continue; }
                return result;
            }
            623 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP); lexer.mark_end();
                if lookahead == 38 { state = 523; lexer.advance(false); continue; }
                return result;
            }
            624 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                return result;
            }
            625 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                if lookahead == 45 { state = 120; lexer.advance(false); continue; }
                if lookahead == 61 { state = 564; lexer.advance(false); continue; }
                if lookahead == 124 { state = 525; lexer.advance(false); continue; }
                return result;
            }
            626 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                if lookahead == 45 { state = 120; lexer.advance(false); continue; }
                if lookahead == 61 { state = 121; lexer.advance(false); continue; }
                if lookahead == 124 { state = 525; lexer.advance(false); continue; }
                return result;
            }
            627 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                if lookahead == 61 { state = 563; lexer.advance(false); continue; }
                if lookahead == 124 { state = 525; lexer.advance(false); continue; }
                return result;
            }
            628 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                if lookahead == 124 { state = 525; lexer.advance(false); continue; }
                return result;
            }
            629 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET); lexer.mark_end();
                if lookahead == 61 { state = 565; lexer.advance(false); continue; }
                if lookahead == 126 { state = 631; lexer.advance(false); continue; }
                return result;
            }
            630 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET); lexer.mark_end();
                if lookahead == 126 { state = 631; lexer.advance(false); continue; }
                return result;
            }
            631 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET_TILDE); lexer.mark_end();
                return result;
            }
            632 => {
                result = true; lexer.set_result_symbol(anon_sym_TILDE_CARET); lexer.mark_end();
                return result;
            }
            633 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT_GT); lexer.mark_end();
                return result;
            }
            634 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT_GT); lexer.mark_end();
                if lookahead == 61 { state = 569; lexer.advance(false); continue; }
                return result;
            }
            635 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT_LT); lexer.mark_end();
                return result;
            }
            636 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT_LT); lexer.mark_end();
                if lookahead == 61 { state = 568; lexer.advance(false); continue; }
                return result;
            }
            637 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_DASH_GT); lexer.mark_end();
                return result;
            }
            638 => {
                result = true; lexer.set_result_symbol(aux_sym_string_literal_token1); lexer.mark_end();
                if lookahead == 10 { state = 643; lexer.advance(false); continue; }
                if lookahead == 34 || lookahead == 92 { state = 676; lexer.advance(false); continue; }
                if lookahead != 0 { state = 638; lexer.advance(false); continue; }
                return result;
            }
            639 => {
                result = true; lexer.set_result_symbol(aux_sym_string_literal_token1); lexer.mark_end();
                if lookahead == 42 { state = 641; lexer.advance(false); continue; }
                if lookahead == 47 { state = 638; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 34 && lookahead != 92 { state = 643; lexer.advance(false); continue; }
                return result;
            }
            640 => {
                result = true; lexer.set_result_symbol(aux_sym_string_literal_token1); lexer.mark_end();
                if lookahead == 42 { state = 640; lexer.advance(false); continue; }
                if lookahead == 47 { state = 643; lexer.advance(false); continue; }
                if lookahead == 34 || lookahead == 92 { state = 71; lexer.advance(false); continue; }
                if lookahead != 0 { state = 641; lexer.advance(false); continue; }
                return result;
            }
            641 => {
                result = true; lexer.set_result_symbol(aux_sym_string_literal_token1); lexer.mark_end();
                if lookahead == 42 { state = 640; lexer.advance(false); continue; }
                if lookahead == 34 || lookahead == 92 { state = 71; lexer.advance(false); continue; }
                if lookahead != 0 { state = 641; lexer.advance(false); continue; }
                return result;
            }
            642 => {
                result = true; lexer.set_result_symbol(aux_sym_string_literal_token1); lexer.mark_end();
                if lookahead == 47 { state = 639; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 642; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 34 && lookahead != 92 { state = 643; lexer.advance(false); continue; }
                return result;
            }
            643 => {
                result = true; lexer.set_result_symbol(aux_sym_string_literal_token1); lexer.mark_end();
                if lookahead != 0 && lookahead != 34 && lookahead != 92 { state = 643; lexer.advance(false); continue; }
                return result;
            }
            644 => {
                result = true; lexer.set_result_symbol(aux_sym_string_literal_token2); lexer.mark_end();
                return result;
            }
            645 => {
                result = true; lexer.set_result_symbol(aux_sym_string_literal_token3); lexer.mark_end();
                return result;
            }
            646 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTE); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (48, 667), (49, 668), (66, 90), (98, 91), (123, 579), (88, 669), (90, 669), (120, 669),
                    (122, 669),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            647 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTE); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (48, 667), (49, 668), (123, 579), (66, 365), (98, 365), (68, 364), (100, 364), (72, 366),
                    (104, 366), (79, 367), (111, 367), (83, 362), (115, 362), (88, 669), (90, 669), (120, 669),
                    (122, 669),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            648 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTE); lexer.mark_end();
                if lookahead == 123 { state = 579; lexer.advance(false); continue; }
                return result;
            }
            649 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTE); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (123, 579), (66, 365), (98, 365), (68, 364), (100, 364), (72, 366), (104, 366), (79, 367),
                    (111, 367), (83, 362), (115, 362),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            650 => {
                result = true; lexer.set_result_symbol(anon_sym_TILDE_AMP); lexer.mark_end();
                return result;
            }
            651 => {
                result = true; lexer.set_result_symbol(anon_sym_TILDE_PIPE); lexer.mark_end();
                return result;
            }
            652 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_DASH); lexer.mark_end();
                return result;
            }
            653 => {
                result = true; lexer.set_result_symbol(aux_sym_decimal_number_token1); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || lookahead == 95 { state = 653; lexer.advance(false); continue; }
                return result;
            }
            654 => {
                result = true; lexer.set_result_symbol(aux_sym_decimal_number_token2); lexer.mark_end();
                if lookahead == 95 { state = 654; lexer.advance(false); continue; }
                return result;
            }
            655 => {
                result = true; lexer.set_result_symbol(sym_binary_number); lexer.mark_end();
                if lookahead == 48 || lookahead == 49 || lookahead == 63 || lookahead == 88 || lookahead == 90 || lookahead == 95 || lookahead == 120 || lookahead == 122 { state = 655; lexer.advance(false); continue; }
                return result;
            }
            656 => {
                result = true; lexer.set_result_symbol(sym_octal_number); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 55 || lookahead == 63 || lookahead == 88 || lookahead == 90 || lookahead == 95 || lookahead == 120 || lookahead == 122 { state = 656; lexer.advance(false); continue; }
                return result;
            }
            657 => {
                result = true; lexer.set_result_symbol(sym_hex_number); lexer.mark_end();
                if set_contains(&sym_hex_number_character_set_2, lookahead) { state = 657; lexer.advance(false); continue; }
                return result;
            }
            658 => {
                result = true; lexer.set_result_symbol(aux_sym_real_number_token1); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || lookahead == 95 { state = 658; lexer.advance(false); continue; }
                return result;
            }
            659 => {
                result = true; lexer.set_result_symbol(sym_fixed_point_number); lexer.mark_end();
                if lookahead == 69 || lookahead == 101 { state = 361; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || lookahead == 95 { state = 659; lexer.advance(false); continue; }
                return result;
            }
            660 => {
                result = true; lexer.set_result_symbol(sym_fixed_point_number); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || lookahead == 95 { state = 660; lexer.advance(false); continue; }
                return result;
            }
            661 => {
                result = true; lexer.set_result_symbol(sym_unsigned_number); lexer.mark_end();
                if lookahead == 39 { state = 363; lexer.advance(false); continue; }
                if lookahead == 46 { state = 368; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 361; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 62; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || lookahead == 95 { state = 661; lexer.advance(false); continue; }
                return result;
            }
            662 => {
                result = true; lexer.set_result_symbol(sym_unsigned_number); lexer.mark_end();
                if lookahead == 39 { state = 363; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 62; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || lookahead == 95 { state = 662; lexer.advance(false); continue; }
                return result;
            }
            663 => {
                result = true; lexer.set_result_symbol(sym_unsigned_number); lexer.mark_end();
                if lookahead == 46 { state = 368; lexer.advance(false); continue; }
                if lookahead == 115 { state = 336; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 361; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || lookahead == 95 { state = 664; lexer.advance(false); continue; }
                return result;
            }
            664 => {
                result = true; lexer.set_result_symbol(sym_unsigned_number); lexer.mark_end();
                if lookahead == 46 { state = 368; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 361; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || lookahead == 95 { state = 664; lexer.advance(false); continue; }
                return result;
            }
            665 => {
                result = true; lexer.set_result_symbol(sym_unsigned_number); lexer.mark_end();
                if lookahead == 46 { state = 370; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || lookahead == 95 { state = 665; lexer.advance(false); continue; }
                return result;
            }
            666 => {
                result = true; lexer.set_result_symbol(sym_unsigned_number); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || lookahead == 95 { state = 666; lexer.advance(false); continue; }
                return result;
            }
            667 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTE0); lexer.mark_end();
                return result;
            }
            668 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTE1); lexer.mark_end();
                return result;
            }
            669 => {
                result = true; lexer.set_result_symbol(aux_sym_unbased_unsized_literal_token1); lexer.mark_end();
                return result;
            }
            670 => {
                result = true; lexer.set_result_symbol(anon_sym_LPAREN_STAR); lexer.mark_end();
                return result;
            }
            671 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR_RPAREN); lexer.mark_end();
                return result;
            }
            672 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                return result;
            }
            673 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 10 { state = 402; lexer.advance(false); continue; }
                if lookahead == 13 { state = 398; lexer.advance(false); continue; }
                if lookahead != 0 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            674 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 9 || 11 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 676; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) { state = 674; lexer.advance(false); continue; }
                return result;
            }
            675 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 32 { state = 683; lexer.advance(false); continue; }
                return result;
            }
            676 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 { state = 676; lexer.advance(false); continue; }
                return result;
            }
            677 => {
                result = true; lexer.set_result_symbol(sym_c_identifier); lexer.mark_end();
                if lookahead == 36 { state = 698; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 677; lexer.advance(false); continue; }
                return result;
            }
            678 => {
                result = true; lexer.set_result_symbol(anon_sym_BSLASH); lexer.mark_end();
                return result;
            }
            679 => {
                result = true; lexer.set_result_symbol(aux_sym_escaped_identifier_token1); lexer.mark_end();
                if lookahead == 42 { state = 681; lexer.advance(false); continue; }
                if lookahead == 47 { state = 674; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 32 { state = 683; lexer.advance(false); continue; }
                return result;
            }
            680 => {
                result = true; lexer.set_result_symbol(aux_sym_escaped_identifier_token1); lexer.mark_end();
                if lookahead == 42 { state = 680; lexer.advance(false); continue; }
                if lookahead == 47 { state = 675; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 71; lexer.advance(false); continue; }
                if lookahead != 0 { state = 681; lexer.advance(false); continue; }
                return result;
            }
            681 => {
                result = true; lexer.set_result_symbol(aux_sym_escaped_identifier_token1); lexer.mark_end();
                if lookahead == 42 { state = 680; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 71; lexer.advance(false); continue; }
                if lookahead != 0 { state = 681; lexer.advance(false); continue; }
                return result;
            }
            682 => {
                result = true; lexer.set_result_symbol(aux_sym_escaped_identifier_token1); lexer.mark_end();
                if lookahead == 47 { state = 679; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 32 { state = 683; lexer.advance(false); continue; }
                return result;
            }
            683 => {
                result = true; lexer.set_result_symbol(aux_sym_escaped_identifier_token1); lexer.mark_end();
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 32 { state = 683; lexer.advance(false); continue; }
                return result;
            }
            684 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLARroot); lexer.mark_end();
                return result;
            }
            685 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLARroot); lexer.mark_end();
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 705; lexer.advance(false); continue; }
                return result;
            }
            686 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLARunit); lexer.mark_end();
                return result;
            }
            687 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLARunit); lexer.mark_end();
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 705; lexer.advance(false); continue; }
                return result;
            }
            688 => {
                result = true; lexer.set_result_symbol(sym_simple_identifier); lexer.mark_end();
                if lookahead == 36 { state = 689; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            689 => {
                result = true; lexer.set_result_symbol(sym_simple_identifier); lexer.mark_end();
                if lookahead == 61 { state = 480; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            690 => {
                result = true; lexer.set_result_symbol(sym_simple_identifier); lexer.mark_end();
                if lookahead == 65 { state = 696; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 66 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            691 => {
                result = true; lexer.set_result_symbol(sym_simple_identifier); lexer.mark_end();
                if lookahead == 69 { state = 688; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            692 => {
                result = true; lexer.set_result_symbol(sym_simple_identifier); lexer.mark_end();
                if lookahead == 72 { state = 694; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            693 => {
                result = true; lexer.set_result_symbol(sym_simple_identifier); lexer.mark_end();
                if lookahead == 76 { state = 695; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            694 => {
                result = true; lexer.set_result_symbol(sym_simple_identifier); lexer.mark_end();
                if lookahead == 80 { state = 697; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            695 => {
                result = true; lexer.set_result_symbol(sym_simple_identifier); lexer.mark_end();
                if lookahead == 83 { state = 691; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            696 => {
                result = true; lexer.set_result_symbol(sym_simple_identifier); lexer.mark_end();
                if lookahead == 84 { state = 692; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            697 => {
                result = true; lexer.set_result_symbol(sym_simple_identifier); lexer.mark_end();
                if lookahead == 85 { state = 693; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            698 => {
                result = true; lexer.set_result_symbol(sym_simple_identifier); lexer.mark_end();
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            699 => {
                result = true; lexer.set_result_symbol(sym_system_tf_identifier); lexer.mark_end();
                if lookahead == 105 { state = 704; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 705; lexer.advance(false); continue; }
                return result;
            }
            700 => {
                result = true; lexer.set_result_symbol(sym_system_tf_identifier); lexer.mark_end();
                if lookahead == 110 { state = 699; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 705; lexer.advance(false); continue; }
                return result;
            }
            701 => {
                result = true; lexer.set_result_symbol(sym_system_tf_identifier); lexer.mark_end();
                if lookahead == 111 { state = 703; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 705; lexer.advance(false); continue; }
                return result;
            }
            702 => {
                result = true; lexer.set_result_symbol(sym_system_tf_identifier); lexer.mark_end();
                if lookahead == 111 { state = 701; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 705; lexer.advance(false); continue; }
                return result;
            }
            703 => {
                result = true; lexer.set_result_symbol(sym_system_tf_identifier); lexer.mark_end();
                if lookahead == 116 { state = 685; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 705; lexer.advance(false); continue; }
                return result;
            }
            704 => {
                result = true; lexer.set_result_symbol(sym_system_tf_identifier); lexer.mark_end();
                if lookahead == 116 { state = 687; lexer.advance(false); continue; }
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 705; lexer.advance(false); continue; }
                return result;
            }
            705 => {
                result = true; lexer.set_result_symbol(sym_system_tf_identifier); lexer.mark_end();
                if lookahead == 36 || 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 705; lexer.advance(false); continue; }
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
                    (97, 1), (98, 2), (99, 3), (100, 4), (101, 5), (102, 6), (103, 7), (104, 8),
                    (105, 9), (106, 10), (108, 11), (109, 12), (110, 13), (111, 14), (112, 15), (114, 16),
                    (115, 17), (116, 18), (117, 19), (118, 20), (119, 21), (120, 22), (88, 23), (90, 23),
                    (122, 23),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 0; lexer.advance(true); continue; }
                return result;
            }
            1 => {
                if lookahead == 99 { state = 24; lexer.advance(false); continue; }
                if lookahead == 108 { state = 25; lexer.advance(false); continue; }
                if lookahead == 110 { state = 26; lexer.advance(false); continue; }
                if lookahead == 115 { state = 27; lexer.advance(false); continue; }
                if lookahead == 117 { state = 28; lexer.advance(false); continue; }
                return result;
            }
            2 => {
                if lookahead == 101 { state = 29; lexer.advance(false); continue; }
                if lookahead == 105 { state = 30; lexer.advance(false); continue; }
                if lookahead == 114 { state = 31; lexer.advance(false); continue; }
                if lookahead == 117 { state = 32; lexer.advance(false); continue; }
                if lookahead == 121 { state = 33; lexer.advance(false); continue; }
                return result;
            }
            3 => {
                if lookahead == 97 { state = 34; lexer.advance(false); continue; }
                if lookahead == 104 { state = 35; lexer.advance(false); continue; }
                if lookahead == 108 { state = 36; lexer.advance(false); continue; }
                if lookahead == 109 { state = 37; lexer.advance(false); continue; }
                if lookahead == 111 { state = 38; lexer.advance(false); continue; }
                if lookahead == 114 { state = 39; lexer.advance(false); continue; }
                return result;
            }
            4 => {
                if lookahead == 101 { state = 40; lexer.advance(false); continue; }
                if lookahead == 105 { state = 41; lexer.advance(false); continue; }
                if lookahead == 111 { state = 42; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                if lookahead == 100 { state = 43; lexer.advance(false); continue; }
                if lookahead == 108 { state = 44; lexer.advance(false); continue; }
                if lookahead == 110 { state = 45; lexer.advance(false); continue; }
                if lookahead == 118 { state = 46; lexer.advance(false); continue; }
                if lookahead == 120 { state = 47; lexer.advance(false); continue; }
                return result;
            }
            6 => {
                if lookahead == 105 { state = 48; lexer.advance(false); continue; }
                if lookahead == 111 { state = 49; lexer.advance(false); continue; }
                if lookahead == 115 { state = 50; lexer.advance(false); continue; }
                if lookahead == 117 { state = 51; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if lookahead == 101 { state = 52; lexer.advance(false); continue; }
                if lookahead == 108 { state = 53; lexer.advance(false); continue; }
                return result;
            }
            8 => {
                if lookahead == 105 { state = 54; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if lookahead == 102 { state = 55; lexer.advance(false); continue; }
                if lookahead == 103 { state = 56; lexer.advance(false); continue; }
                if lookahead == 108 { state = 57; lexer.advance(false); continue; }
                if lookahead == 109 { state = 58; lexer.advance(false); continue; }
                if lookahead == 110 { state = 59; lexer.advance(false); continue; }
                return result;
            }
            10 => {
                if lookahead == 111 { state = 60; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if lookahead == 97 { state = 61; lexer.advance(false); continue; }
                if lookahead == 101 { state = 62; lexer.advance(false); continue; }
                if lookahead == 111 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if lookahead == 97 { state = 64; lexer.advance(false); continue; }
                if lookahead == 101 { state = 65; lexer.advance(false); continue; }
                if lookahead == 111 { state = 66; lexer.advance(false); continue; }
                if lookahead == 115 { state = 67; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if lookahead == 97 { state = 68; lexer.advance(false); continue; }
                if lookahead == 101 { state = 69; lexer.advance(false); continue; }
                if lookahead == 109 { state = 70; lexer.advance(false); continue; }
                if lookahead == 111 { state = 71; lexer.advance(false); continue; }
                if lookahead == 115 { state = 72; lexer.advance(false); continue; }
                if lookahead == 117 { state = 73; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if lookahead == 112 { state = 74; lexer.advance(false); continue; }
                if lookahead == 114 { state = 75; lexer.advance(false); continue; }
                if lookahead == 117 { state = 76; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if lookahead == 97 { state = 77; lexer.advance(false); continue; }
                if lookahead == 109 { state = 78; lexer.advance(false); continue; }
                if lookahead == 111 { state = 79; lexer.advance(false); continue; }
                if lookahead == 114 { state = 80; lexer.advance(false); continue; }
                if lookahead == 115 { state = 81; lexer.advance(false); continue; }
                if lookahead == 117 { state = 82; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if lookahead == 97 { state = 83; lexer.advance(false); continue; }
                if lookahead == 99 { state = 84; lexer.advance(false); continue; }
                if lookahead == 101 { state = 85; lexer.advance(false); continue; }
                if lookahead == 110 { state = 86; lexer.advance(false); continue; }
                if lookahead == 112 { state = 87; lexer.advance(false); continue; }
                if lookahead == 116 { state = 88; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                result = true; lexer.set_result_symbol(anon_sym_s); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (95, 89), (97, 90), (99, 91), (101, 92), (104, 93), (105, 94), (109, 95), (111, 96),
                    (112, 97), (116, 98), (117, 99), (121, 100),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            18 => {
                if lookahead == 97 { state = 101; lexer.advance(false); continue; }
                if lookahead == 104 { state = 102; lexer.advance(false); continue; }
                if lookahead == 105 { state = 103; lexer.advance(false); continue; }
                if lookahead == 114 { state = 104; lexer.advance(false); continue; }
                if lookahead == 121 { state = 105; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if lookahead == 110 { state = 106; lexer.advance(false); continue; }
                if lookahead == 115 { state = 107; lexer.advance(false); continue; }
                if lookahead == 119 { state = 108; lexer.advance(false); continue; }
                return result;
            }
            20 => {
                if lookahead == 97 { state = 109; lexer.advance(false); continue; }
                if lookahead == 101 { state = 110; lexer.advance(false); continue; }
                if lookahead == 105 { state = 111; lexer.advance(false); continue; }
                if lookahead == 111 { state = 112; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                if lookahead == 97 { state = 113; lexer.advance(false); continue; }
                if lookahead == 101 { state = 114; lexer.advance(false); continue; }
                if lookahead == 104 { state = 115; lexer.advance(false); continue; }
                if lookahead == 105 { state = 116; lexer.advance(false); continue; }
                if lookahead == 111 { state = 117; lexer.advance(false); continue; }
                return result;
            }
            22 => {
                if lookahead == 110 { state = 118; lexer.advance(false); continue; }
                if lookahead == 111 { state = 119; lexer.advance(false); continue; }
                if lookahead == 48 || lookahead == 49 { state = 120; lexer.advance(false); continue; }
                return result;
            }
            23 => {
                if lookahead == 48 || lookahead == 49 { state = 120; lexer.advance(false); continue; }
                return result;
            }
            24 => {
                if lookahead == 99 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            25 => {
                if lookahead == 105 { state = 122; lexer.advance(false); continue; }
                if lookahead == 119 { state = 123; lexer.advance(false); continue; }
                return result;
            }
            26 => {
                if lookahead == 100 { state = 124; lexer.advance(false); continue; }
                return result;
            }
            27 => {
                if lookahead == 115 { state = 125; lexer.advance(false); continue; }
                return result;
            }
            28 => {
                if lookahead == 116 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                if lookahead == 102 { state = 127; lexer.advance(false); continue; }
                if lookahead == 103 { state = 128; lexer.advance(false); continue; }
                return result;
            }
            30 => {
                if lookahead == 110 { state = 129; lexer.advance(false); continue; }
                if lookahead == 116 { state = 130; lexer.advance(false); continue; }
                return result;
            }
            31 => {
                if lookahead == 101 { state = 131; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                if lookahead == 102 { state = 132; lexer.advance(false); continue; }
                return result;
            }
            33 => {
                if lookahead == 116 { state = 133; lexer.advance(false); continue; }
                return result;
            }
            34 => {
                if lookahead == 115 { state = 134; lexer.advance(false); continue; }
                return result;
            }
            35 => {
                if lookahead == 97 { state = 135; lexer.advance(false); continue; }
                if lookahead == 101 { state = 136; lexer.advance(false); continue; }
                return result;
            }
            36 => {
                if lookahead == 97 { state = 137; lexer.advance(false); continue; }
                if lookahead == 111 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                if lookahead == 111 { state = 139; lexer.advance(false); continue; }
                return result;
            }
            38 => {
                if lookahead == 110 { state = 140; lexer.advance(false); continue; }
                if lookahead == 118 { state = 141; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                if lookahead == 111 { state = 142; lexer.advance(false); continue; }
                return result;
            }
            40 => {
                if lookahead == 97 { state = 143; lexer.advance(false); continue; }
                if lookahead == 102 { state = 144; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                if lookahead == 115 { state = 145; lexer.advance(false); continue; }
                return result;
            }
            42 => {
                result = true; lexer.set_result_symbol(anon_sym_do); lexer.mark_end();
                return result;
            }
            43 => {
                if lookahead == 103 { state = 146; lexer.advance(false); continue; }
                return result;
            }
            44 => {
                if lookahead == 115 { state = 147; lexer.advance(false); continue; }
                return result;
            }
            45 => {
                if lookahead == 100 { state = 148; lexer.advance(false); continue; }
                if lookahead == 117 { state = 149; lexer.advance(false); continue; }
                return result;
            }
            46 => {
                if lookahead == 101 { state = 150; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                if lookahead == 112 { state = 151; lexer.advance(false); continue; }
                if lookahead == 116 { state = 152; lexer.advance(false); continue; }
                return result;
            }
            48 => {
                if lookahead == 110 { state = 153; lexer.advance(false); continue; }
                if lookahead == 114 { state = 154; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                if lookahead == 114 { state = 155; lexer.advance(false); continue; }
                return result;
            }
            50 => {
                result = true; lexer.set_result_symbol(anon_sym_fs); lexer.mark_end();
                return result;
            }
            51 => {
                if lookahead == 110 { state = 156; lexer.advance(false); continue; }
                return result;
            }
            52 => {
                if lookahead == 110 { state = 157; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                if lookahead == 111 { state = 158; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                if lookahead == 103 { state = 159; lexer.advance(false); continue; }
                return result;
            }
            55 => {
                result = true; lexer.set_result_symbol(anon_sym_if); lexer.mark_end();
                if lookahead == 102 { state = 160; lexer.advance(false); continue; }
                if lookahead == 110 { state = 161; lexer.advance(false); continue; }
                return result;
            }
            56 => {
                if lookahead == 110 { state = 162; lexer.advance(false); continue; }
                return result;
            }
            57 => {
                if lookahead == 108 { state = 163; lexer.advance(false); continue; }
                return result;
            }
            58 => {
                if lookahead == 112 { state = 164; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                if lookahead == 105 { state = 165; lexer.advance(false); continue; }
                if lookahead == 111 { state = 166; lexer.advance(false); continue; }
                if lookahead == 112 { state = 167; lexer.advance(false); continue; }
                if lookahead == 115 { state = 168; lexer.advance(false); continue; }
                if lookahead == 116 { state = 169; lexer.advance(false); continue; }
                return result;
            }
            60 => {
                if lookahead == 105 { state = 170; lexer.advance(false); continue; }
                return result;
            }
            61 => {
                if lookahead == 114 { state = 171; lexer.advance(false); continue; }
                return result;
            }
            62 => {
                if lookahead == 116 { state = 172; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                if lookahead == 99 { state = 173; lexer.advance(false); continue; }
                if lookahead == 103 { state = 174; lexer.advance(false); continue; }
                if lookahead == 110 { state = 175; lexer.advance(false); continue; }
                return result;
            }
            64 => {
                if lookahead == 99 { state = 176; lexer.advance(false); continue; }
                if lookahead == 116 { state = 177; lexer.advance(false); continue; }
                return result;
            }
            65 => {
                if lookahead == 100 { state = 178; lexer.advance(false); continue; }
                return result;
            }
            66 => {
                if lookahead == 100 { state = 179; lexer.advance(false); continue; }
                return result;
            }
            67 => {
                result = true; lexer.set_result_symbol(anon_sym_ms); lexer.mark_end();
                return result;
            }
            68 => {
                if lookahead == 110 { state = 180; lexer.advance(false); continue; }
                return result;
            }
            69 => {
                if lookahead == 103 { state = 181; lexer.advance(false); continue; }
                if lookahead == 116 { state = 182; lexer.advance(false); continue; }
                if lookahead == 119 { state = 183; lexer.advance(false); continue; }
                if lookahead == 120 { state = 184; lexer.advance(false); continue; }
                return result;
            }
            70 => {
                if lookahead == 111 { state = 185; lexer.advance(false); continue; }
                return result;
            }
            71 => {
                if lookahead == 110 { state = 186; lexer.advance(false); continue; }
                if lookahead == 114 { state = 187; lexer.advance(false); continue; }
                if lookahead == 115 { state = 188; lexer.advance(false); continue; }
                if lookahead == 116 { state = 189; lexer.advance(false); continue; }
                return result;
            }
            72 => {
                result = true; lexer.set_result_symbol(anon_sym_ns); lexer.mark_end();
                return result;
            }
            73 => {
                if lookahead == 108 { state = 190; lexer.advance(false); continue; }
                return result;
            }
            74 => {
                if lookahead == 116 { state = 191; lexer.advance(false); continue; }
                return result;
            }
            75 => {
                result = true; lexer.set_result_symbol(anon_sym_or); lexer.mark_end();
                return result;
            }
            76 => {
                if lookahead == 116 { state = 192; lexer.advance(false); continue; }
                return result;
            }
            77 => {
                if lookahead == 99 { state = 193; lexer.advance(false); continue; }
                if lookahead == 114 { state = 194; lexer.advance(false); continue; }
                return result;
            }
            78 => {
                if lookahead == 111 { state = 195; lexer.advance(false); continue; }
                return result;
            }
            79 => {
                if lookahead == 115 { state = 196; lexer.advance(false); continue; }
                return result;
            }
            80 => {
                if lookahead == 105 { state = 197; lexer.advance(false); continue; }
                if lookahead == 111 { state = 198; lexer.advance(false); continue; }
                return result;
            }
            81 => {
                result = true; lexer.set_result_symbol(anon_sym_ps); lexer.mark_end();
                return result;
            }
            82 => {
                if lookahead == 108 { state = 199; lexer.advance(false); continue; }
                if lookahead == 114 { state = 200; lexer.advance(false); continue; }
                return result;
            }
            83 => {
                if lookahead == 110 { state = 201; lexer.advance(false); continue; }
                return result;
            }
            84 => {
                if lookahead == 109 { state = 202; lexer.advance(false); continue; }
                return result;
            }
            85 => {
                if let Some(next) = advance_map(&[
                    (97, 203), (102, 204), (103, 205), (106, 206), (108, 207), (112, 208), (115, 209), (116, 210),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            86 => {
                if lookahead == 109 { state = 211; lexer.advance(false); continue; }
                return result;
            }
            87 => {
                if lookahead == 109 { state = 212; lexer.advance(false); continue; }
                return result;
            }
            88 => {
                if lookahead == 114 { state = 213; lexer.advance(false); continue; }
                return result;
            }
            89 => {
                if lookahead == 97 { state = 214; lexer.advance(false); continue; }
                if lookahead == 101 { state = 215; lexer.advance(false); continue; }
                if lookahead == 110 { state = 216; lexer.advance(false); continue; }
                if lookahead == 117 { state = 217; lexer.advance(false); continue; }
                return result;
            }
            90 => {
                if lookahead == 109 { state = 218; lexer.advance(false); continue; }
                return result;
            }
            91 => {
                if lookahead == 97 { state = 219; lexer.advance(false); continue; }
                return result;
            }
            92 => {
                if lookahead == 113 { state = 220; lexer.advance(false); continue; }
                return result;
            }
            93 => {
                if lookahead == 111 { state = 221; lexer.advance(false); continue; }
                return result;
            }
            94 => {
                if lookahead == 103 { state = 222; lexer.advance(false); continue; }
                return result;
            }
            95 => {
                if lookahead == 97 { state = 223; lexer.advance(false); continue; }
                return result;
            }
            96 => {
                if lookahead == 102 { state = 224; lexer.advance(false); continue; }
                if lookahead == 108 { state = 225; lexer.advance(false); continue; }
                return result;
            }
            97 => {
                if lookahead == 101 { state = 226; lexer.advance(false); continue; }
                return result;
            }
            98 => {
                if lookahead == 97 { state = 227; lexer.advance(false); continue; }
                if lookahead == 100 { state = 228; lexer.advance(false); continue; }
                if lookahead == 114 { state = 229; lexer.advance(false); continue; }
                return result;
            }
            99 => {
                if lookahead == 112 { state = 230; lexer.advance(false); continue; }
                return result;
            }
            100 => {
                if lookahead == 110 { state = 231; lexer.advance(false); continue; }
                return result;
            }
            101 => {
                if lookahead == 98 { state = 232; lexer.advance(false); continue; }
                if lookahead == 103 { state = 233; lexer.advance(false); continue; }
                if lookahead == 115 { state = 234; lexer.advance(false); continue; }
                return result;
            }
            102 => {
                if lookahead == 105 { state = 235; lexer.advance(false); continue; }
                if lookahead == 114 { state = 236; lexer.advance(false); continue; }
                return result;
            }
            103 => {
                if lookahead == 109 { state = 237; lexer.advance(false); continue; }
                return result;
            }
            104 => {
                if lookahead == 97 { state = 238; lexer.advance(false); continue; }
                if lookahead == 105 { state = 239; lexer.advance(false); continue; }
                return result;
            }
            105 => {
                if lookahead == 112 { state = 240; lexer.advance(false); continue; }
                return result;
            }
            106 => {
                if lookahead == 105 { state = 241; lexer.advance(false); continue; }
                if lookahead == 115 { state = 242; lexer.advance(false); continue; }
                if lookahead == 116 { state = 243; lexer.advance(false); continue; }
                return result;
            }
            107 => {
                result = true; lexer.set_result_symbol(anon_sym_us); lexer.mark_end();
                return result;
            }
            108 => {
                if lookahead == 105 { state = 244; lexer.advance(false); continue; }
                return result;
            }
            109 => {
                if lookahead == 114 { state = 245; lexer.advance(false); continue; }
                return result;
            }
            110 => {
                if lookahead == 99 { state = 246; lexer.advance(false); continue; }
                return result;
            }
            111 => {
                if lookahead == 114 { state = 247; lexer.advance(false); continue; }
                return result;
            }
            112 => {
                if lookahead == 105 { state = 248; lexer.advance(false); continue; }
                return result;
            }
            113 => {
                if lookahead == 105 { state = 249; lexer.advance(false); continue; }
                if lookahead == 110 { state = 250; lexer.advance(false); continue; }
                return result;
            }
            114 => {
                if lookahead == 97 { state = 251; lexer.advance(false); continue; }
                return result;
            }
            115 => {
                if lookahead == 105 { state = 252; lexer.advance(false); continue; }
                return result;
            }
            116 => {
                if lookahead == 108 { state = 253; lexer.advance(false); continue; }
                if lookahead == 114 { state = 254; lexer.advance(false); continue; }
                if lookahead == 116 { state = 255; lexer.advance(false); continue; }
                return result;
            }
            117 => {
                if lookahead == 114 { state = 256; lexer.advance(false); continue; }
                return result;
            }
            118 => {
                if lookahead == 111 { state = 257; lexer.advance(false); continue; }
                return result;
            }
            119 => {
                if lookahead == 114 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            120 => {
                result = true; lexer.set_result_symbol(aux_sym_edge_descriptor_token1); lexer.mark_end();
                return result;
            }
            121 => {
                if lookahead == 101 { state = 259; lexer.advance(false); continue; }
                return result;
            }
            122 => {
                if lookahead == 97 { state = 260; lexer.advance(false); continue; }
                return result;
            }
            123 => {
                if lookahead == 97 { state = 261; lexer.advance(false); continue; }
                return result;
            }
            124 => {
                result = true; lexer.set_result_symbol(anon_sym_and); lexer.mark_end();
                return result;
            }
            125 => {
                if lookahead == 101 { state = 262; lexer.advance(false); continue; }
                if lookahead == 105 { state = 263; lexer.advance(false); continue; }
                if lookahead == 117 { state = 264; lexer.advance(false); continue; }
                return result;
            }
            126 => {
                if lookahead == 111 { state = 265; lexer.advance(false); continue; }
                return result;
            }
            127 => {
                if lookahead == 111 { state = 266; lexer.advance(false); continue; }
                return result;
            }
            128 => {
                if lookahead == 105 { state = 267; lexer.advance(false); continue; }
                return result;
            }
            129 => {
                if lookahead == 100 { state = 268; lexer.advance(false); continue; }
                if lookahead == 115 { state = 269; lexer.advance(false); continue; }
                return result;
            }
            130 => {
                result = true; lexer.set_result_symbol(anon_sym_bit); lexer.mark_end();
                return result;
            }
            131 => {
                if lookahead == 97 { state = 270; lexer.advance(false); continue; }
                return result;
            }
            132 => {
                result = true; lexer.set_result_symbol(anon_sym_buf); lexer.mark_end();
                if lookahead == 105 { state = 271; lexer.advance(false); continue; }
                return result;
            }
            133 => {
                if lookahead == 101 { state = 272; lexer.advance(false); continue; }
                return result;
            }
            134 => {
                if lookahead == 101 { state = 273; lexer.advance(false); continue; }
                return result;
            }
            135 => {
                if lookahead == 110 { state = 274; lexer.advance(false); continue; }
                return result;
            }
            136 => {
                if lookahead == 99 { state = 275; lexer.advance(false); continue; }
                return result;
            }
            137 => {
                if lookahead == 115 { state = 276; lexer.advance(false); continue; }
                return result;
            }
            138 => {
                if lookahead == 99 { state = 277; lexer.advance(false); continue; }
                return result;
            }
            139 => {
                if lookahead == 115 { state = 278; lexer.advance(false); continue; }
                return result;
            }
            140 => {
                if lookahead == 115 { state = 279; lexer.advance(false); continue; }
                if lookahead == 116 { state = 280; lexer.advance(false); continue; }
                return result;
            }
            141 => {
                if lookahead == 101 { state = 281; lexer.advance(false); continue; }
                return result;
            }
            142 => {
                if lookahead == 115 { state = 282; lexer.advance(false); continue; }
                return result;
            }
            143 => {
                if lookahead == 115 { state = 283; lexer.advance(false); continue; }
                return result;
            }
            144 => {
                if lookahead == 97 { state = 284; lexer.advance(false); continue; }
                if lookahead == 112 { state = 285; lexer.advance(false); continue; }
                return result;
            }
            145 => {
                if lookahead == 97 { state = 286; lexer.advance(false); continue; }
                if lookahead == 116 { state = 287; lexer.advance(false); continue; }
                return result;
            }
            146 => {
                if lookahead == 101 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            147 => {
                if lookahead == 101 { state = 289; lexer.advance(false); continue; }
                return result;
            }
            148 => {
                result = true; lexer.set_result_symbol(anon_sym_end); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (99, 290), (102, 291), (103, 292), (105, 293), (109, 294), (112, 295), (115, 296), (116, 297),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            149 => {
                if lookahead == 109 { state = 298; lexer.advance(false); continue; }
                return result;
            }
            150 => {
                if lookahead == 110 { state = 299; lexer.advance(false); continue; }
                return result;
            }
            151 => {
                if lookahead == 101 { state = 300; lexer.advance(false); continue; }
                if lookahead == 111 { state = 301; lexer.advance(false); continue; }
                return result;
            }
            152 => {
                if lookahead == 101 { state = 302; lexer.advance(false); continue; }
                return result;
            }
            153 => {
                if lookahead == 97 { state = 303; lexer.advance(false); continue; }
                return result;
            }
            154 => {
                if lookahead == 115 { state = 304; lexer.advance(false); continue; }
                return result;
            }
            155 => {
                result = true; lexer.set_result_symbol(anon_sym_for); lexer.mark_end();
                if lookahead == 99 { state = 305; lexer.advance(false); continue; }
                if lookahead == 101 { state = 306; lexer.advance(false); continue; }
                if lookahead == 107 { state = 307; lexer.advance(false); continue; }
                return result;
            }
            156 => {
                if lookahead == 99 { state = 308; lexer.advance(false); continue; }
                return result;
            }
            157 => {
                if lookahead == 101 { state = 309; lexer.advance(false); continue; }
                if lookahead == 118 { state = 310; lexer.advance(false); continue; }
                return result;
            }
            158 => {
                if lookahead == 98 { state = 311; lexer.advance(false); continue; }
                return result;
            }
            159 => {
                if lookahead == 104 { state = 312; lexer.advance(false); continue; }
                return result;
            }
            160 => {
                result = true; lexer.set_result_symbol(anon_sym_iff); lexer.mark_end();
                return result;
            }
            161 => {
                if lookahead == 111 { state = 313; lexer.advance(false); continue; }
                return result;
            }
            162 => {
                if lookahead == 111 { state = 314; lexer.advance(false); continue; }
                return result;
            }
            163 => {
                if lookahead == 101 { state = 315; lexer.advance(false); continue; }
                return result;
            }
            164 => {
                if lookahead == 108 { state = 316; lexer.advance(false); continue; }
                if lookahead == 111 { state = 317; lexer.advance(false); continue; }
                return result;
            }
            165 => {
                if lookahead == 116 { state = 318; lexer.advance(false); continue; }
                return result;
            }
            166 => {
                if lookahead == 117 { state = 319; lexer.advance(false); continue; }
                return result;
            }
            167 => {
                if lookahead == 117 { state = 320; lexer.advance(false); continue; }
                return result;
            }
            168 => {
                if lookahead == 105 { state = 321; lexer.advance(false); continue; }
                return result;
            }
            169 => {
                result = true; lexer.set_result_symbol(anon_sym_int); lexer.mark_end();
                if lookahead == 101 { state = 322; lexer.advance(false); continue; }
                return result;
            }
            170 => {
                if lookahead == 110 { state = 323; lexer.advance(false); continue; }
                return result;
            }
            171 => {
                if lookahead == 103 { state = 324; lexer.advance(false); continue; }
                return result;
            }
            172 => {
                result = true; lexer.set_result_symbol(anon_sym_let); lexer.mark_end();
                return result;
            }
            173 => {
                if lookahead == 97 { state = 325; lexer.advance(false); continue; }
                return result;
            }
            174 => {
                if lookahead == 105 { state = 326; lexer.advance(false); continue; }
                return result;
            }
            175 => {
                if lookahead == 103 { state = 327; lexer.advance(false); continue; }
                return result;
            }
            176 => {
                if lookahead == 114 { state = 328; lexer.advance(false); continue; }
                return result;
            }
            177 => {
                if lookahead == 99 { state = 329; lexer.advance(false); continue; }
                return result;
            }
            178 => {
                if lookahead == 105 { state = 330; lexer.advance(false); continue; }
                return result;
            }
            179 => {
                if lookahead == 112 { state = 331; lexer.advance(false); continue; }
                if lookahead == 117 { state = 332; lexer.advance(false); continue; }
                return result;
            }
            180 => {
                if lookahead == 100 { state = 333; lexer.advance(false); continue; }
                return result;
            }
            181 => {
                if lookahead == 101 { state = 334; lexer.advance(false); continue; }
                return result;
            }
            182 => {
                if lookahead == 116 { state = 335; lexer.advance(false); continue; }
                return result;
            }
            183 => {
                result = true; lexer.set_result_symbol(anon_sym_new); lexer.mark_end();
                return result;
            }
            184 => {
                if lookahead == 116 { state = 336; lexer.advance(false); continue; }
                return result;
            }
            185 => {
                if lookahead == 115 { state = 337; lexer.advance(false); continue; }
                return result;
            }
            186 => {
                if lookahead == 101 { state = 338; lexer.advance(false); continue; }
                return result;
            }
            187 => {
                result = true; lexer.set_result_symbol(anon_sym_nor); lexer.mark_end();
                return result;
            }
            188 => {
                if lookahead == 104 { state = 339; lexer.advance(false); continue; }
                return result;
            }
            189 => {
                result = true; lexer.set_result_symbol(anon_sym_not); lexer.mark_end();
                if lookahead == 105 { state = 340; lexer.advance(false); continue; }
                return result;
            }
            190 => {
                if lookahead == 108 { state = 341; lexer.advance(false); continue; }
                return result;
            }
            191 => {
                if lookahead == 105 { state = 342; lexer.advance(false); continue; }
                return result;
            }
            192 => {
                if lookahead == 112 { state = 343; lexer.advance(false); continue; }
                return result;
            }
            193 => {
                if lookahead == 107 { state = 344; lexer.advance(false); continue; }
                return result;
            }
            194 => {
                if lookahead == 97 { state = 345; lexer.advance(false); continue; }
                return result;
            }
            195 => {
                if lookahead == 115 { state = 346; lexer.advance(false); continue; }
                return result;
            }
            196 => {
                if lookahead == 101 { state = 347; lexer.advance(false); continue; }
                return result;
            }
            197 => {
                if lookahead == 109 { state = 348; lexer.advance(false); continue; }
                if lookahead == 111 { state = 349; lexer.advance(false); continue; }
                return result;
            }
            198 => {
                if lookahead == 103 { state = 350; lexer.advance(false); continue; }
                if lookahead == 112 { state = 351; lexer.advance(false); continue; }
                if lookahead == 116 { state = 352; lexer.advance(false); continue; }
                return result;
            }
            199 => {
                if lookahead == 108 { state = 353; lexer.advance(false); continue; }
                if lookahead == 115 { state = 354; lexer.advance(false); continue; }
                return result;
            }
            200 => {
                if lookahead == 101 { state = 355; lexer.advance(false); continue; }
                return result;
            }
            201 => {
                if lookahead == 100 { state = 356; lexer.advance(false); continue; }
                return result;
            }
            202 => {
                if lookahead == 111 { state = 357; lexer.advance(false); continue; }
                return result;
            }
            203 => {
                if lookahead == 108 { state = 358; lexer.advance(false); continue; }
                return result;
            }
            204 => {
                result = true; lexer.set_result_symbol(anon_sym_ref); lexer.mark_end();
                return result;
            }
            205 => {
                result = true; lexer.set_result_symbol(anon_sym_reg); lexer.mark_end();
                return result;
            }
            206 => {
                if lookahead == 101 { state = 359; lexer.advance(false); continue; }
                return result;
            }
            207 => {
                if lookahead == 101 { state = 360; lexer.advance(false); continue; }
                return result;
            }
            208 => {
                if lookahead == 101 { state = 361; lexer.advance(false); continue; }
                return result;
            }
            209 => {
                if lookahead == 116 { state = 362; lexer.advance(false); continue; }
                return result;
            }
            210 => {
                if lookahead == 117 { state = 363; lexer.advance(false); continue; }
                return result;
            }
            211 => {
                if lookahead == 111 { state = 364; lexer.advance(false); continue; }
                return result;
            }
            212 => {
                if lookahead == 111 { state = 365; lexer.advance(false); continue; }
                return result;
            }
            213 => {
                if lookahead == 97 { state = 366; lexer.advance(false); continue; }
                return result;
            }
            214 => {
                if lookahead == 108 { state = 367; lexer.advance(false); continue; }
                return result;
            }
            215 => {
                if lookahead == 118 { state = 368; lexer.advance(false); continue; }
                return result;
            }
            216 => {
                if lookahead == 101 { state = 369; lexer.advance(false); continue; }
                return result;
            }
            217 => {
                if lookahead == 110 { state = 370; lexer.advance(false); continue; }
                return result;
            }
            218 => {
                if lookahead == 112 { state = 371; lexer.advance(false); continue; }
                return result;
            }
            219 => {
                if lookahead == 108 { state = 372; lexer.advance(false); continue; }
                return result;
            }
            220 => {
                if lookahead == 117 { state = 373; lexer.advance(false); continue; }
                return result;
            }
            221 => {
                if lookahead == 114 { state = 374; lexer.advance(false); continue; }
                if lookahead == 119 { state = 375; lexer.advance(false); continue; }
                return result;
            }
            222 => {
                if lookahead == 110 { state = 376; lexer.advance(false); continue; }
                return result;
            }
            223 => {
                if lookahead == 108 { state = 377; lexer.advance(false); continue; }
                return result;
            }
            224 => {
                if lookahead == 116 { state = 378; lexer.advance(false); continue; }
                return result;
            }
            225 => {
                if lookahead == 118 { state = 379; lexer.advance(false); continue; }
                return result;
            }
            226 => {
                if lookahead == 99 { state = 380; lexer.advance(false); continue; }
                return result;
            }
            227 => {
                if lookahead == 116 { state = 381; lexer.advance(false); continue; }
                return result;
            }
            228 => {
                result = true; lexer.set_result_symbol(anon_sym_std); lexer.mark_end();
                return result;
            }
            229 => {
                if lookahead == 105 { state = 382; lexer.advance(false); continue; }
                if lookahead == 111 { state = 383; lexer.advance(false); continue; }
                if lookahead == 117 { state = 384; lexer.advance(false); continue; }
                return result;
            }
            230 => {
                if lookahead == 101 { state = 385; lexer.advance(false); continue; }
                if lookahead == 112 { state = 386; lexer.advance(false); continue; }
                return result;
            }
            231 => {
                if lookahead == 99 { state = 387; lexer.advance(false); continue; }
                return result;
            }
            232 => {
                if lookahead == 108 { state = 388; lexer.advance(false); continue; }
                return result;
            }
            233 => {
                if lookahead == 103 { state = 389; lexer.advance(false); continue; }
                return result;
            }
            234 => {
                if lookahead == 107 { state = 390; lexer.advance(false); continue; }
                return result;
            }
            235 => {
                if lookahead == 115 { state = 391; lexer.advance(false); continue; }
                return result;
            }
            236 => {
                if lookahead == 111 { state = 392; lexer.advance(false); continue; }
                return result;
            }
            237 => {
                if lookahead == 101 { state = 393; lexer.advance(false); continue; }
                return result;
            }
            238 => {
                if lookahead == 110 { state = 394; lexer.advance(false); continue; }
                return result;
            }
            239 => {
                result = true; lexer.set_result_symbol(anon_sym_tri); lexer.mark_end();
                if lookahead == 48 { state = 395; lexer.advance(false); continue; }
                if lookahead == 49 { state = 396; lexer.advance(false); continue; }
                if lookahead == 97 { state = 397; lexer.advance(false); continue; }
                if lookahead == 111 { state = 398; lexer.advance(false); continue; }
                if lookahead == 114 { state = 399; lexer.advance(false); continue; }
                return result;
            }
            240 => {
                if lookahead == 101 { state = 400; lexer.advance(false); continue; }
                return result;
            }
            241 => {
                if lookahead == 111 { state = 401; lexer.advance(false); continue; }
                if lookahead == 113 { state = 402; lexer.advance(false); continue; }
                return result;
            }
            242 => {
                if lookahead == 105 { state = 403; lexer.advance(false); continue; }
                return result;
            }
            243 => {
                if lookahead == 105 { state = 404; lexer.advance(false); continue; }
                if lookahead == 121 { state = 405; lexer.advance(false); continue; }
                return result;
            }
            244 => {
                if lookahead == 114 { state = 406; lexer.advance(false); continue; }
                return result;
            }
            245 => {
                result = true; lexer.set_result_symbol(anon_sym_var); lexer.mark_end();
                return result;
            }
            246 => {
                if lookahead == 116 { state = 407; lexer.advance(false); continue; }
                return result;
            }
            247 => {
                if lookahead == 116 { state = 408; lexer.advance(false); continue; }
                return result;
            }
            248 => {
                if lookahead == 100 { state = 409; lexer.advance(false); continue; }
                return result;
            }
            249 => {
                if lookahead == 116 { state = 410; lexer.advance(false); continue; }
                return result;
            }
            250 => {
                if lookahead == 100 { state = 411; lexer.advance(false); continue; }
                return result;
            }
            251 => {
                if lookahead == 107 { state = 412; lexer.advance(false); continue; }
                return result;
            }
            252 => {
                if lookahead == 108 { state = 413; lexer.advance(false); continue; }
                return result;
            }
            253 => {
                if lookahead == 100 { state = 414; lexer.advance(false); continue; }
                return result;
            }
            254 => {
                if lookahead == 101 { state = 415; lexer.advance(false); continue; }
                return result;
            }
            255 => {
                if lookahead == 104 { state = 416; lexer.advance(false); continue; }
                return result;
            }
            256 => {
                result = true; lexer.set_result_symbol(anon_sym_wor); lexer.mark_end();
                return result;
            }
            257 => {
                if lookahead == 114 { state = 417; lexer.advance(false); continue; }
                return result;
            }
            258 => {
                result = true; lexer.set_result_symbol(anon_sym_xor); lexer.mark_end();
                return result;
            }
            259 => {
                if lookahead == 112 { state = 418; lexer.advance(false); continue; }
                return result;
            }
            260 => {
                if lookahead == 115 { state = 419; lexer.advance(false); continue; }
                return result;
            }
            261 => {
                if lookahead == 121 { state = 420; lexer.advance(false); continue; }
                return result;
            }
            262 => {
                if lookahead == 114 { state = 421; lexer.advance(false); continue; }
                return result;
            }
            263 => {
                if lookahead == 103 { state = 422; lexer.advance(false); continue; }
                return result;
            }
            264 => {
                if lookahead == 109 { state = 423; lexer.advance(false); continue; }
                return result;
            }
            265 => {
                if lookahead == 109 { state = 424; lexer.advance(false); continue; }
                return result;
            }
            266 => {
                if lookahead == 114 { state = 425; lexer.advance(false); continue; }
                return result;
            }
            267 => {
                if lookahead == 110 { state = 426; lexer.advance(false); continue; }
                return result;
            }
            268 => {
                result = true; lexer.set_result_symbol(anon_sym_bind); lexer.mark_end();
                return result;
            }
            269 => {
                result = true; lexer.set_result_symbol(anon_sym_bins); lexer.mark_end();
                if lookahead == 111 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            270 => {
                if lookahead == 107 { state = 428; lexer.advance(false); continue; }
                return result;
            }
            271 => {
                if lookahead == 102 { state = 429; lexer.advance(false); continue; }
                return result;
            }
            272 => {
                result = true; lexer.set_result_symbol(anon_sym_byte); lexer.mark_end();
                return result;
            }
            273 => {
                result = true; lexer.set_result_symbol(anon_sym_case); lexer.mark_end();
                if lookahead == 120 { state = 430; lexer.advance(false); continue; }
                if lookahead == 122 { state = 431; lexer.advance(false); continue; }
                return result;
            }
            274 => {
                if lookahead == 100 { state = 432; lexer.advance(false); continue; }
                return result;
            }
            275 => {
                if lookahead == 107 { state = 433; lexer.advance(false); continue; }
                return result;
            }
            276 => {
                if lookahead == 115 { state = 434; lexer.advance(false); continue; }
                return result;
            }
            277 => {
                if lookahead == 107 { state = 435; lexer.advance(false); continue; }
                return result;
            }
            278 => {
                result = true; lexer.set_result_symbol(anon_sym_cmos); lexer.mark_end();
                return result;
            }
            279 => {
                if lookahead == 116 { state = 436; lexer.advance(false); continue; }
                return result;
            }
            280 => {
                if lookahead == 101 { state = 437; lexer.advance(false); continue; }
                if lookahead == 105 { state = 438; lexer.advance(false); continue; }
                return result;
            }
            281 => {
                if lookahead == 114 { state = 439; lexer.advance(false); continue; }
                return result;
            }
            282 => {
                if lookahead == 115 { state = 440; lexer.advance(false); continue; }
                return result;
            }
            283 => {
                if lookahead == 115 { state = 441; lexer.advance(false); continue; }
                return result;
            }
            284 => {
                if lookahead == 117 { state = 442; lexer.advance(false); continue; }
                return result;
            }
            285 => {
                if lookahead == 97 { state = 443; lexer.advance(false); continue; }
                return result;
            }
            286 => {
                if lookahead == 98 { state = 444; lexer.advance(false); continue; }
                return result;
            }
            287 => {
                result = true; lexer.set_result_symbol(anon_sym_dist); lexer.mark_end();
                return result;
            }
            288 => {
                result = true; lexer.set_result_symbol(anon_sym_edge); lexer.mark_end();
                return result;
            }
            289 => {
                result = true; lexer.set_result_symbol(anon_sym_else); lexer.mark_end();
                return result;
            }
            290 => {
                if lookahead == 97 { state = 445; lexer.advance(false); continue; }
                if lookahead == 104 { state = 446; lexer.advance(false); continue; }
                if lookahead == 108 { state = 447; lexer.advance(false); continue; }
                return result;
            }
            291 => {
                if lookahead == 117 { state = 448; lexer.advance(false); continue; }
                return result;
            }
            292 => {
                if lookahead == 101 { state = 449; lexer.advance(false); continue; }
                if lookahead == 114 { state = 450; lexer.advance(false); continue; }
                return result;
            }
            293 => {
                if lookahead == 110 { state = 451; lexer.advance(false); continue; }
                return result;
            }
            294 => {
                if lookahead == 111 { state = 452; lexer.advance(false); continue; }
                return result;
            }
            295 => {
                if lookahead == 97 { state = 453; lexer.advance(false); continue; }
                if lookahead == 114 { state = 454; lexer.advance(false); continue; }
                return result;
            }
            296 => {
                if lookahead == 101 { state = 455; lexer.advance(false); continue; }
                if lookahead == 112 { state = 456; lexer.advance(false); continue; }
                return result;
            }
            297 => {
                if lookahead == 97 { state = 457; lexer.advance(false); continue; }
                return result;
            }
            298 => {
                result = true; lexer.set_result_symbol(anon_sym_enum); lexer.mark_end();
                return result;
            }
            299 => {
                if lookahead == 116 { state = 458; lexer.advance(false); continue; }
                return result;
            }
            300 => {
                if lookahead == 99 { state = 459; lexer.advance(false); continue; }
                return result;
            }
            301 => {
                if lookahead == 114 { state = 460; lexer.advance(false); continue; }
                return result;
            }
            302 => {
                if lookahead == 110 { state = 461; lexer.advance(false); continue; }
                if lookahead == 114 { state = 462; lexer.advance(false); continue; }
                return result;
            }
            303 => {
                if lookahead == 108 { state = 463; lexer.advance(false); continue; }
                return result;
            }
            304 => {
                if lookahead == 116 { state = 464; lexer.advance(false); continue; }
                return result;
            }
            305 => {
                if lookahead == 101 { state = 465; lexer.advance(false); continue; }
                return result;
            }
            306 => {
                if lookahead == 97 { state = 466; lexer.advance(false); continue; }
                if lookahead == 118 { state = 467; lexer.advance(false); continue; }
                return result;
            }
            307 => {
                result = true; lexer.set_result_symbol(anon_sym_fork); lexer.mark_end();
                if lookahead == 106 { state = 468; lexer.advance(false); continue; }
                return result;
            }
            308 => {
                if lookahead == 116 { state = 469; lexer.advance(false); continue; }
                return result;
            }
            309 => {
                if lookahead == 114 { state = 470; lexer.advance(false); continue; }
                return result;
            }
            310 => {
                if lookahead == 97 { state = 471; lexer.advance(false); continue; }
                return result;
            }
            311 => {
                if lookahead == 97 { state = 472; lexer.advance(false); continue; }
                return result;
            }
            312 => {
                if lookahead == 122 { state = 473; lexer.advance(false); continue; }
                return result;
            }
            313 => {
                if lookahead == 110 { state = 474; lexer.advance(false); continue; }
                return result;
            }
            314 => {
                if lookahead == 114 { state = 475; lexer.advance(false); continue; }
                return result;
            }
            315 => {
                if lookahead == 103 { state = 476; lexer.advance(false); continue; }
                return result;
            }
            316 => {
                if lookahead == 101 { state = 477; lexer.advance(false); continue; }
                if lookahead == 105 { state = 478; lexer.advance(false); continue; }
                return result;
            }
            317 => {
                if lookahead == 114 { state = 479; lexer.advance(false); continue; }
                return result;
            }
            318 => {
                if lookahead == 105 { state = 480; lexer.advance(false); continue; }
                return result;
            }
            319 => {
                if lookahead == 116 { state = 481; lexer.advance(false); continue; }
                return result;
            }
            320 => {
                if lookahead == 116 { state = 482; lexer.advance(false); continue; }
                return result;
            }
            321 => {
                if lookahead == 100 { state = 483; lexer.advance(false); continue; }
                return result;
            }
            322 => {
                if lookahead == 103 { state = 484; lexer.advance(false); continue; }
                if lookahead == 114 { state = 485; lexer.advance(false); continue; }
                return result;
            }
            323 => {
                result = true; lexer.set_result_symbol(anon_sym_join); lexer.mark_end();
                if lookahead == 95 { state = 486; lexer.advance(false); continue; }
                return result;
            }
            324 => {
                if lookahead == 101 { state = 487; lexer.advance(false); continue; }
                return result;
            }
            325 => {
                if lookahead == 108 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            326 => {
                if lookahead == 99 { state = 489; lexer.advance(false); continue; }
                return result;
            }
            327 => {
                if lookahead == 105 { state = 490; lexer.advance(false); continue; }
                return result;
            }
            328 => {
                if lookahead == 111 { state = 491; lexer.advance(false); continue; }
                return result;
            }
            329 => {
                if lookahead == 104 { state = 492; lexer.advance(false); continue; }
                return result;
            }
            330 => {
                if lookahead == 117 { state = 493; lexer.advance(false); continue; }
                return result;
            }
            331 => {
                if lookahead == 111 { state = 494; lexer.advance(false); continue; }
                return result;
            }
            332 => {
                if lookahead == 108 { state = 495; lexer.advance(false); continue; }
                return result;
            }
            333 => {
                result = true; lexer.set_result_symbol(anon_sym_nand); lexer.mark_end();
                return result;
            }
            334 => {
                if lookahead == 100 { state = 496; lexer.advance(false); continue; }
                return result;
            }
            335 => {
                if lookahead == 121 { state = 497; lexer.advance(false); continue; }
                return result;
            }
            336 => {
                if lookahead == 116 { state = 498; lexer.advance(false); continue; }
                return result;
            }
            337 => {
                result = true; lexer.set_result_symbol(anon_sym_nmos); lexer.mark_end();
                return result;
            }
            338 => {
                result = true; lexer.set_result_symbol(anon_sym_none); lexer.mark_end();
                return result;
            }
            339 => {
                if lookahead == 111 { state = 499; lexer.advance(false); continue; }
                return result;
            }
            340 => {
                if lookahead == 102 { state = 500; lexer.advance(false); continue; }
                return result;
            }
            341 => {
                result = true; lexer.set_result_symbol(anon_sym_null); lexer.mark_end();
                return result;
            }
            342 => {
                if lookahead == 111 { state = 501; lexer.advance(false); continue; }
                return result;
            }
            343 => {
                if lookahead == 117 { state = 502; lexer.advance(false); continue; }
                return result;
            }
            344 => {
                if lookahead == 97 { state = 503; lexer.advance(false); continue; }
                if lookahead == 101 { state = 504; lexer.advance(false); continue; }
                return result;
            }
            345 => {
                if lookahead == 109 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            346 => {
                result = true; lexer.set_result_symbol(anon_sym_pmos); lexer.mark_end();
                return result;
            }
            347 => {
                if lookahead == 100 { state = 506; lexer.advance(false); continue; }
                return result;
            }
            348 => {
                if lookahead == 105 { state = 507; lexer.advance(false); continue; }
                return result;
            }
            349 => {
                if lookahead == 114 { state = 508; lexer.advance(false); continue; }
                return result;
            }
            350 => {
                if lookahead == 114 { state = 509; lexer.advance(false); continue; }
                return result;
            }
            351 => {
                if lookahead == 101 { state = 510; lexer.advance(false); continue; }
                return result;
            }
            352 => {
                if lookahead == 101 { state = 511; lexer.advance(false); continue; }
                return result;
            }
            353 => {
                if lookahead == 48 { state = 512; lexer.advance(false); continue; }
                if lookahead == 49 { state = 513; lexer.advance(false); continue; }
                if lookahead == 100 { state = 514; lexer.advance(false); continue; }
                if lookahead == 117 { state = 515; lexer.advance(false); continue; }
                return result;
            }
            354 => {
                if lookahead == 101 { state = 516; lexer.advance(false); continue; }
                return result;
            }
            355 => {
                result = true; lexer.set_result_symbol(anon_sym_pure); lexer.mark_end();
                return result;
            }
            356 => {
                result = true; lexer.set_result_symbol(anon_sym_rand); lexer.mark_end();
                if lookahead == 99 { state = 517; lexer.advance(false); continue; }
                if lookahead == 111 { state = 518; lexer.advance(false); continue; }
                return result;
            }
            357 => {
                if lookahead == 115 { state = 519; lexer.advance(false); continue; }
                return result;
            }
            358 => {
                result = true; lexer.set_result_symbol(anon_sym_real); lexer.mark_end();
                if lookahead == 116 { state = 520; lexer.advance(false); continue; }
                return result;
            }
            359 => {
                if lookahead == 99 { state = 521; lexer.advance(false); continue; }
                return result;
            }
            360 => {
                if lookahead == 97 { state = 522; lexer.advance(false); continue; }
                return result;
            }
            361 => {
                if lookahead == 97 { state = 523; lexer.advance(false); continue; }
                return result;
            }
            362 => {
                if lookahead == 114 { state = 524; lexer.advance(false); continue; }
                return result;
            }
            363 => {
                if lookahead == 114 { state = 525; lexer.advance(false); continue; }
                return result;
            }
            364 => {
                if lookahead == 115 { state = 526; lexer.advance(false); continue; }
                return result;
            }
            365 => {
                if lookahead == 115 { state = 527; lexer.advance(false); continue; }
                return result;
            }
            366 => {
                if lookahead == 110 { state = 528; lexer.advance(false); continue; }
                return result;
            }
            367 => {
                if lookahead == 119 { state = 529; lexer.advance(false); continue; }
                return result;
            }
            368 => {
                if lookahead == 101 { state = 530; lexer.advance(false); continue; }
                return result;
            }
            369 => {
                if lookahead == 120 { state = 531; lexer.advance(false); continue; }
                return result;
            }
            370 => {
                if lookahead == 116 { state = 532; lexer.advance(false); continue; }
                return result;
            }
            371 => {
                if lookahead == 108 { state = 533; lexer.advance(false); continue; }
                return result;
            }
            372 => {
                if lookahead == 97 { state = 534; lexer.advance(false); continue; }
                return result;
            }
            373 => {
                if lookahead == 101 { state = 535; lexer.advance(false); continue; }
                return result;
            }
            374 => {
                if lookahead == 116 { state = 536; lexer.advance(false); continue; }
                return result;
            }
            375 => {
                if lookahead == 99 { state = 537; lexer.advance(false); continue; }
                return result;
            }
            376 => {
                if lookahead == 101 { state = 538; lexer.advance(false); continue; }
                return result;
            }
            377 => {
                if lookahead == 108 { state = 539; lexer.advance(false); continue; }
                return result;
            }
            378 => {
                result = true; lexer.set_result_symbol(anon_sym_soft); lexer.mark_end();
                return result;
            }
            379 => {
                if lookahead == 101 { state = 540; lexer.advance(false); continue; }
                return result;
            }
            380 => {
                if lookahead == 105 { state = 541; lexer.advance(false); continue; }
                if lookahead == 112 { state = 542; lexer.advance(false); continue; }
                return result;
            }
            381 => {
                if lookahead == 105 { state = 543; lexer.advance(false); continue; }
                return result;
            }
            382 => {
                if lookahead == 110 { state = 544; lexer.advance(false); continue; }
                return result;
            }
            383 => {
                if lookahead == 110 { state = 545; lexer.advance(false); continue; }
                return result;
            }
            384 => {
                if lookahead == 99 { state = 546; lexer.advance(false); continue; }
                return result;
            }
            385 => {
                if lookahead == 114 { state = 547; lexer.advance(false); continue; }
                return result;
            }
            386 => {
                if lookahead == 108 { state = 548; lexer.advance(false); continue; }
                return result;
            }
            387 => {
                if lookahead == 95 { state = 549; lexer.advance(false); continue; }
                return result;
            }
            388 => {
                if lookahead == 101 { state = 550; lexer.advance(false); continue; }
                return result;
            }
            389 => {
                if lookahead == 101 { state = 551; lexer.advance(false); continue; }
                return result;
            }
            390 => {
                result = true; lexer.set_result_symbol(anon_sym_task); lexer.mark_end();
                return result;
            }
            391 => {
                result = true; lexer.set_result_symbol(anon_sym_this); lexer.mark_end();
                return result;
            }
            392 => {
                if lookahead == 117 { state = 552; lexer.advance(false); continue; }
                return result;
            }
            393 => {
                result = true; lexer.set_result_symbol(anon_sym_time); lexer.mark_end();
                if lookahead == 112 { state = 553; lexer.advance(false); continue; }
                if lookahead == 117 { state = 554; lexer.advance(false); continue; }
                return result;
            }
            394 => {
                result = true; lexer.set_result_symbol(anon_sym_tran); lexer.mark_end();
                if lookahead == 105 { state = 555; lexer.advance(false); continue; }
                return result;
            }
            395 => {
                result = true; lexer.set_result_symbol(anon_sym_tri0); lexer.mark_end();
                return result;
            }
            396 => {
                result = true; lexer.set_result_symbol(anon_sym_tri1); lexer.mark_end();
                return result;
            }
            397 => {
                if lookahead == 110 { state = 556; lexer.advance(false); continue; }
                return result;
            }
            398 => {
                if lookahead == 114 { state = 557; lexer.advance(false); continue; }
                return result;
            }
            399 => {
                if lookahead == 101 { state = 558; lexer.advance(false); continue; }
                return result;
            }
            400 => {
                result = true; lexer.set_result_symbol(anon_sym_type); lexer.mark_end();
                if lookahead == 95 { state = 559; lexer.advance(false); continue; }
                if lookahead == 100 { state = 560; lexer.advance(false); continue; }
                return result;
            }
            401 => {
                if lookahead == 110 { state = 561; lexer.advance(false); continue; }
                return result;
            }
            402 => {
                if lookahead == 117 { state = 562; lexer.advance(false); continue; }
                return result;
            }
            403 => {
                if lookahead == 103 { state = 563; lexer.advance(false); continue; }
                return result;
            }
            404 => {
                if lookahead == 108 { state = 564; lexer.advance(false); continue; }
                return result;
            }
            405 => {
                if lookahead == 112 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            406 => {
                if lookahead == 101 { state = 566; lexer.advance(false); continue; }
                return result;
            }
            407 => {
                if lookahead == 111 { state = 567; lexer.advance(false); continue; }
                return result;
            }
            408 => {
                if lookahead == 117 { state = 568; lexer.advance(false); continue; }
                return result;
            }
            409 => {
                result = true; lexer.set_result_symbol(anon_sym_void); lexer.mark_end();
                return result;
            }
            410 => {
                result = true; lexer.set_result_symbol(anon_sym_wait); lexer.mark_end();
                if lookahead == 95 { state = 569; lexer.advance(false); continue; }
                return result;
            }
            411 => {
                result = true; lexer.set_result_symbol(anon_sym_wand); lexer.mark_end();
                return result;
            }
            412 => {
                result = true; lexer.set_result_symbol(anon_sym_weak); lexer.mark_end();
                if lookahead == 48 { state = 570; lexer.advance(false); continue; }
                if lookahead == 49 { state = 571; lexer.advance(false); continue; }
                return result;
            }
            413 => {
                if lookahead == 101 { state = 572; lexer.advance(false); continue; }
                return result;
            }
            414 => {
                if lookahead == 99 { state = 573; lexer.advance(false); continue; }
                return result;
            }
            415 => {
                result = true; lexer.set_result_symbol(anon_sym_wire); lexer.mark_end();
                return result;
            }
            416 => {
                result = true; lexer.set_result_symbol(anon_sym_with); lexer.mark_end();
                if lookahead == 105 { state = 574; lexer.advance(false); continue; }
                return result;
            }
            417 => {
                result = true; lexer.set_result_symbol(anon_sym_xnor); lexer.mark_end();
                return result;
            }
            418 => {
                if lookahead == 116 { state = 575; lexer.advance(false); continue; }
                return result;
            }
            419 => {
                result = true; lexer.set_result_symbol(anon_sym_alias); lexer.mark_end();
                return result;
            }
            420 => {
                if lookahead == 115 { state = 576; lexer.advance(false); continue; }
                return result;
            }
            421 => {
                if lookahead == 116 { state = 577; lexer.advance(false); continue; }
                return result;
            }
            422 => {
                if lookahead == 110 { state = 578; lexer.advance(false); continue; }
                return result;
            }
            423 => {
                if lookahead == 101 { state = 579; lexer.advance(false); continue; }
                return result;
            }
            424 => {
                if lookahead == 97 { state = 580; lexer.advance(false); continue; }
                return result;
            }
            425 => {
                if lookahead == 101 { state = 581; lexer.advance(false); continue; }
                return result;
            }
            426 => {
                result = true; lexer.set_result_symbol(anon_sym_begin); lexer.mark_end();
                return result;
            }
            427 => {
                if lookahead == 102 { state = 582; lexer.advance(false); continue; }
                return result;
            }
            428 => {
                result = true; lexer.set_result_symbol(anon_sym_break); lexer.mark_end();
                return result;
            }
            429 => {
                if lookahead == 48 { state = 583; lexer.advance(false); continue; }
                if lookahead == 49 { state = 584; lexer.advance(false); continue; }
                return result;
            }
            430 => {
                result = true; lexer.set_result_symbol(anon_sym_casex); lexer.mark_end();
                return result;
            }
            431 => {
                result = true; lexer.set_result_symbol(anon_sym_casez); lexer.mark_end();
                return result;
            }
            432 => {
                if lookahead == 108 { state = 585; lexer.advance(false); continue; }
                return result;
            }
            433 => {
                if lookahead == 101 { state = 586; lexer.advance(false); continue; }
                return result;
            }
            434 => {
                result = true; lexer.set_result_symbol(anon_sym_class); lexer.mark_end();
                return result;
            }
            435 => {
                if lookahead == 105 { state = 587; lexer.advance(false); continue; }
                return result;
            }
            436 => {
                result = true; lexer.set_result_symbol(anon_sym_const); lexer.mark_end();
                if lookahead == 114 { state = 588; lexer.advance(false); continue; }
                return result;
            }
            437 => {
                if lookahead == 120 { state = 589; lexer.advance(false); continue; }
                return result;
            }
            438 => {
                if lookahead == 110 { state = 590; lexer.advance(false); continue; }
                return result;
            }
            439 => {
                result = true; lexer.set_result_symbol(anon_sym_cover); lexer.mark_end();
                if lookahead == 103 { state = 591; lexer.advance(false); continue; }
                if lookahead == 112 { state = 592; lexer.advance(false); continue; }
                return result;
            }
            440 => {
                result = true; lexer.set_result_symbol(anon_sym_cross); lexer.mark_end();
                return result;
            }
            441 => {
                if lookahead == 105 { state = 593; lexer.advance(false); continue; }
                return result;
            }
            442 => {
                if lookahead == 108 { state = 594; lexer.advance(false); continue; }
                return result;
            }
            443 => {
                if lookahead == 114 { state = 595; lexer.advance(false); continue; }
                return result;
            }
            444 => {
                if lookahead == 108 { state = 596; lexer.advance(false); continue; }
                return result;
            }
            445 => {
                if lookahead == 115 { state = 597; lexer.advance(false); continue; }
                return result;
            }
            446 => {
                if lookahead == 101 { state = 598; lexer.advance(false); continue; }
                return result;
            }
            447 => {
                if lookahead == 97 { state = 599; lexer.advance(false); continue; }
                if lookahead == 111 { state = 600; lexer.advance(false); continue; }
                return result;
            }
            448 => {
                if lookahead == 110 { state = 601; lexer.advance(false); continue; }
                return result;
            }
            449 => {
                if lookahead == 110 { state = 602; lexer.advance(false); continue; }
                return result;
            }
            450 => {
                if lookahead == 111 { state = 603; lexer.advance(false); continue; }
                return result;
            }
            451 => {
                if lookahead == 116 { state = 604; lexer.advance(false); continue; }
                return result;
            }
            452 => {
                if lookahead == 100 { state = 605; lexer.advance(false); continue; }
                return result;
            }
            453 => {
                if lookahead == 99 { state = 606; lexer.advance(false); continue; }
                return result;
            }
            454 => {
                if lookahead == 105 { state = 607; lexer.advance(false); continue; }
                if lookahead == 111 { state = 608; lexer.advance(false); continue; }
                return result;
            }
            455 => {
                if lookahead == 113 { state = 609; lexer.advance(false); continue; }
                return result;
            }
            456 => {
                if lookahead == 101 { state = 610; lexer.advance(false); continue; }
                return result;
            }
            457 => {
                if lookahead == 115 { state = 611; lexer.advance(false); continue; }
                return result;
            }
            458 => {
                result = true; lexer.set_result_symbol(anon_sym_event); lexer.mark_end();
                if lookahead == 117 { state = 612; lexer.advance(false); continue; }
                return result;
            }
            459 => {
                if lookahead == 116 { state = 613; lexer.advance(false); continue; }
                return result;
            }
            460 => {
                if lookahead == 116 { state = 614; lexer.advance(false); continue; }
                return result;
            }
            461 => {
                if lookahead == 100 { state = 615; lexer.advance(false); continue; }
                return result;
            }
            462 => {
                if lookahead == 110 { state = 616; lexer.advance(false); continue; }
                return result;
            }
            463 => {
                result = true; lexer.set_result_symbol(anon_sym_final); lexer.mark_end();
                return result;
            }
            464 => {
                if lookahead == 95 { state = 617; lexer.advance(false); continue; }
                return result;
            }
            465 => {
                result = true; lexer.set_result_symbol(anon_sym_force); lexer.mark_end();
                return result;
            }
            466 => {
                if lookahead == 99 { state = 618; lexer.advance(false); continue; }
                return result;
            }
            467 => {
                if lookahead == 101 { state = 619; lexer.advance(false); continue; }
                return result;
            }
            468 => {
                if lookahead == 111 { state = 620; lexer.advance(false); continue; }
                return result;
            }
            469 => {
                if lookahead == 105 { state = 621; lexer.advance(false); continue; }
                return result;
            }
            470 => {
                if lookahead == 97 { state = 622; lexer.advance(false); continue; }
                return result;
            }
            471 => {
                if lookahead == 114 { state = 623; lexer.advance(false); continue; }
                return result;
            }
            472 => {
                if lookahead == 108 { state = 624; lexer.advance(false); continue; }
                return result;
            }
            473 => {
                if lookahead == 48 { state = 625; lexer.advance(false); continue; }
                if lookahead == 49 { state = 626; lexer.advance(false); continue; }
                return result;
            }
            474 => {
                if lookahead == 101 { state = 627; lexer.advance(false); continue; }
                return result;
            }
            475 => {
                if lookahead == 101 { state = 628; lexer.advance(false); continue; }
                return result;
            }
            476 => {
                if lookahead == 97 { state = 629; lexer.advance(false); continue; }
                return result;
            }
            477 => {
                if lookahead == 109 { state = 630; lexer.advance(false); continue; }
                return result;
            }
            478 => {
                if lookahead == 101 { state = 631; lexer.advance(false); continue; }
                return result;
            }
            479 => {
                if lookahead == 116 { state = 632; lexer.advance(false); continue; }
                return result;
            }
            480 => {
                if lookahead == 97 { state = 633; lexer.advance(false); continue; }
                return result;
            }
            481 => {
                result = true; lexer.set_result_symbol(anon_sym_inout); lexer.mark_end();
                return result;
            }
            482 => {
                result = true; lexer.set_result_symbol(anon_sym_input); lexer.mark_end();
                return result;
            }
            483 => {
                if lookahead == 101 { state = 634; lexer.advance(false); continue; }
                return result;
            }
            484 => {
                if lookahead == 101 { state = 635; lexer.advance(false); continue; }
                return result;
            }
            485 => {
                if lookahead == 99 { state = 636; lexer.advance(false); continue; }
                if lookahead == 102 { state = 637; lexer.advance(false); continue; }
                if lookahead == 115 { state = 638; lexer.advance(false); continue; }
                return result;
            }
            486 => {
                if lookahead == 97 { state = 639; lexer.advance(false); continue; }
                if lookahead == 110 { state = 640; lexer.advance(false); continue; }
                return result;
            }
            487 => {
                result = true; lexer.set_result_symbol(anon_sym_large); lexer.mark_end();
                return result;
            }
            488 => {
                result = true; lexer.set_result_symbol(anon_sym_local); lexer.mark_end();
                if lookahead == 112 { state = 641; lexer.advance(false); continue; }
                return result;
            }
            489 => {
                result = true; lexer.set_result_symbol(anon_sym_logic); lexer.mark_end();
                return result;
            }
            490 => {
                if lookahead == 110 { state = 642; lexer.advance(false); continue; }
                return result;
            }
            491 => {
                if lookahead == 109 { state = 643; lexer.advance(false); continue; }
                return result;
            }
            492 => {
                if lookahead == 101 { state = 644; lexer.advance(false); continue; }
                return result;
            }
            493 => {
                if lookahead == 109 { state = 645; lexer.advance(false); continue; }
                return result;
            }
            494 => {
                if lookahead == 114 { state = 646; lexer.advance(false); continue; }
                return result;
            }
            495 => {
                if lookahead == 101 { state = 647; lexer.advance(false); continue; }
                return result;
            }
            496 => {
                if lookahead == 103 { state = 648; lexer.advance(false); continue; }
                return result;
            }
            497 => {
                if lookahead == 112 { state = 649; lexer.advance(false); continue; }
                return result;
            }
            498 => {
                if lookahead == 105 { state = 650; lexer.advance(false); continue; }
                return result;
            }
            499 => {
                if lookahead == 119 { state = 651; lexer.advance(false); continue; }
                return result;
            }
            500 => {
                if lookahead == 48 { state = 652; lexer.advance(false); continue; }
                if lookahead == 49 { state = 653; lexer.advance(false); continue; }
                return result;
            }
            501 => {
                if lookahead == 110 { state = 654; lexer.advance(false); continue; }
                return result;
            }
            502 => {
                if lookahead == 116 { state = 655; lexer.advance(false); continue; }
                return result;
            }
            503 => {
                if lookahead == 103 { state = 656; lexer.advance(false); continue; }
                return result;
            }
            504 => {
                if lookahead == 100 { state = 657; lexer.advance(false); continue; }
                return result;
            }
            505 => {
                if lookahead == 101 { state = 658; lexer.advance(false); continue; }
                return result;
            }
            506 => {
                if lookahead == 103 { state = 659; lexer.advance(false); continue; }
                return result;
            }
            507 => {
                if lookahead == 116 { state = 660; lexer.advance(false); continue; }
                return result;
            }
            508 => {
                if lookahead == 105 { state = 661; lexer.advance(false); continue; }
                return result;
            }
            509 => {
                if lookahead == 97 { state = 662; lexer.advance(false); continue; }
                return result;
            }
            510 => {
                if lookahead == 114 { state = 663; lexer.advance(false); continue; }
                return result;
            }
            511 => {
                if lookahead == 99 { state = 664; lexer.advance(false); continue; }
                return result;
            }
            512 => {
                result = true; lexer.set_result_symbol(anon_sym_pull0); lexer.mark_end();
                return result;
            }
            513 => {
                result = true; lexer.set_result_symbol(anon_sym_pull1); lexer.mark_end();
                return result;
            }
            514 => {
                if lookahead == 111 { state = 665; lexer.advance(false); continue; }
                return result;
            }
            515 => {
                if lookahead == 112 { state = 666; lexer.advance(false); continue; }
                return result;
            }
            516 => {
                if lookahead == 115 { state = 667; lexer.advance(false); continue; }
                return result;
            }
            517 => {
                result = true; lexer.set_result_symbol(anon_sym_randc); lexer.mark_end();
                if lookahead == 97 { state = 668; lexer.advance(false); continue; }
                return result;
            }
            518 => {
                if lookahead == 109 { state = 669; lexer.advance(false); continue; }
                return result;
            }
            519 => {
                result = true; lexer.set_result_symbol(anon_sym_rcmos); lexer.mark_end();
                return result;
            }
            520 => {
                if lookahead == 105 { state = 670; lexer.advance(false); continue; }
                return result;
            }
            521 => {
                if lookahead == 116 { state = 671; lexer.advance(false); continue; }
                return result;
            }
            522 => {
                if lookahead == 115 { state = 672; lexer.advance(false); continue; }
                return result;
            }
            523 => {
                if lookahead == 116 { state = 673; lexer.advance(false); continue; }
                return result;
            }
            524 => {
                if lookahead == 105 { state = 674; lexer.advance(false); continue; }
                return result;
            }
            525 => {
                if lookahead == 110 { state = 675; lexer.advance(false); continue; }
                return result;
            }
            526 => {
                result = true; lexer.set_result_symbol(anon_sym_rnmos); lexer.mark_end();
                return result;
            }
            527 => {
                result = true; lexer.set_result_symbol(anon_sym_rpmos); lexer.mark_end();
                return result;
            }
            528 => {
                result = true; lexer.set_result_symbol(anon_sym_rtran); lexer.mark_end();
                if lookahead == 105 { state = 676; lexer.advance(false); continue; }
                return result;
            }
            529 => {
                if lookahead == 97 { state = 677; lexer.advance(false); continue; }
                return result;
            }
            530 => {
                if lookahead == 110 { state = 678; lexer.advance(false); continue; }
                return result;
            }
            531 => {
                if lookahead == 116 { state = 679; lexer.advance(false); continue; }
                return result;
            }
            532 => {
                if lookahead == 105 { state = 680; lexer.advance(false); continue; }
                return result;
            }
            533 => {
                if lookahead == 101 { state = 681; lexer.advance(false); continue; }
                return result;
            }
            534 => {
                if lookahead == 114 { state = 682; lexer.advance(false); continue; }
                return result;
            }
            535 => {
                if lookahead == 110 { state = 683; lexer.advance(false); continue; }
                return result;
            }
            536 => {
                if lookahead == 105 { state = 684; lexer.advance(false); continue; }
                if lookahead == 114 { state = 685; lexer.advance(false); continue; }
                return result;
            }
            537 => {
                if lookahead == 97 { state = 686; lexer.advance(false); continue; }
                return result;
            }
            538 => {
                if lookahead == 100 { state = 687; lexer.advance(false); continue; }
                return result;
            }
            539 => {
                result = true; lexer.set_result_symbol(anon_sym_small); lexer.mark_end();
                return result;
            }
            540 => {
                result = true; lexer.set_result_symbol(anon_sym_solve); lexer.mark_end();
                return result;
            }
            541 => {
                if lookahead == 102 { state = 688; lexer.advance(false); continue; }
                return result;
            }
            542 => {
                if lookahead == 97 { state = 689; lexer.advance(false); continue; }
                return result;
            }
            543 => {
                if lookahead == 99 { state = 690; lexer.advance(false); continue; }
                return result;
            }
            544 => {
                if lookahead == 103 { state = 691; lexer.advance(false); continue; }
                return result;
            }
            545 => {
                if lookahead == 103 { state = 692; lexer.advance(false); continue; }
                return result;
            }
            546 => {
                if lookahead == 116 { state = 693; lexer.advance(false); continue; }
                return result;
            }
            547 => {
                result = true; lexer.set_result_symbol(anon_sym_super); lexer.mark_end();
                return result;
            }
            548 => {
                if lookahead == 121 { state = 694; lexer.advance(false); continue; }
                return result;
            }
            549 => {
                if lookahead == 97 { state = 695; lexer.advance(false); continue; }
                if lookahead == 114 { state = 696; lexer.advance(false); continue; }
                return result;
            }
            550 => {
                result = true; lexer.set_result_symbol(anon_sym_table); lexer.mark_end();
                return result;
            }
            551 => {
                if lookahead == 100 { state = 697; lexer.advance(false); continue; }
                return result;
            }
            552 => {
                if lookahead == 103 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            553 => {
                if lookahead == 114 { state = 699; lexer.advance(false); continue; }
                return result;
            }
            554 => {
                if lookahead == 110 { state = 700; lexer.advance(false); continue; }
                return result;
            }
            555 => {
                if lookahead == 102 { state = 701; lexer.advance(false); continue; }
                return result;
            }
            556 => {
                if lookahead == 100 { state = 702; lexer.advance(false); continue; }
                return result;
            }
            557 => {
                result = true; lexer.set_result_symbol(anon_sym_trior); lexer.mark_end();
                return result;
            }
            558 => {
                if lookahead == 103 { state = 703; lexer.advance(false); continue; }
                return result;
            }
            559 => {
                if lookahead == 111 { state = 704; lexer.advance(false); continue; }
                return result;
            }
            560 => {
                if lookahead == 101 { state = 705; lexer.advance(false); continue; }
                return result;
            }
            561 => {
                result = true; lexer.set_result_symbol(anon_sym_union); lexer.mark_end();
                return result;
            }
            562 => {
                if lookahead == 101 { state = 706; lexer.advance(false); continue; }
                return result;
            }
            563 => {
                if lookahead == 110 { state = 707; lexer.advance(false); continue; }
                return result;
            }
            564 => {
                result = true; lexer.set_result_symbol(anon_sym_until); lexer.mark_end();
                if lookahead == 95 { state = 708; lexer.advance(false); continue; }
                return result;
            }
            565 => {
                if lookahead == 101 { state = 709; lexer.advance(false); continue; }
                return result;
            }
            566 => {
                result = true; lexer.set_result_symbol(anon_sym_uwire); lexer.mark_end();
                return result;
            }
            567 => {
                if lookahead == 114 { state = 710; lexer.advance(false); continue; }
                return result;
            }
            568 => {
                if lookahead == 97 { state = 711; lexer.advance(false); continue; }
                return result;
            }
            569 => {
                if lookahead == 111 { state = 712; lexer.advance(false); continue; }
                return result;
            }
            570 => {
                result = true; lexer.set_result_symbol(anon_sym_weak0); lexer.mark_end();
                return result;
            }
            571 => {
                result = true; lexer.set_result_symbol(anon_sym_weak1); lexer.mark_end();
                return result;
            }
            572 => {
                result = true; lexer.set_result_symbol(anon_sym_while); lexer.mark_end();
                return result;
            }
            573 => {
                if lookahead == 97 { state = 713; lexer.advance(false); continue; }
                return result;
            }
            574 => {
                if lookahead == 110 { state = 714; lexer.advance(false); continue; }
                return result;
            }
            575 => {
                if lookahead == 95 { state = 715; lexer.advance(false); continue; }
                return result;
            }
            576 => {
                result = true; lexer.set_result_symbol(anon_sym_always); lexer.mark_end();
                if lookahead == 95 { state = 716; lexer.advance(false); continue; }
                return result;
            }
            577 => {
                result = true; lexer.set_result_symbol(anon_sym_assert); lexer.mark_end();
                return result;
            }
            578 => {
                result = true; lexer.set_result_symbol(anon_sym_assign); lexer.mark_end();
                return result;
            }
            579 => {
                result = true; lexer.set_result_symbol(anon_sym_assume); lexer.mark_end();
                return result;
            }
            580 => {
                if lookahead == 116 { state = 717; lexer.advance(false); continue; }
                return result;
            }
            581 => {
                result = true; lexer.set_result_symbol(anon_sym_before); lexer.mark_end();
                return result;
            }
            582 => {
                result = true; lexer.set_result_symbol(anon_sym_binsof); lexer.mark_end();
                return result;
            }
            583 => {
                result = true; lexer.set_result_symbol(anon_sym_bufif0); lexer.mark_end();
                return result;
            }
            584 => {
                result = true; lexer.set_result_symbol(anon_sym_bufif1); lexer.mark_end();
                return result;
            }
            585 => {
                if lookahead == 101 { state = 718; lexer.advance(false); continue; }
                return result;
            }
            586 => {
                if lookahead == 114 { state = 719; lexer.advance(false); continue; }
                return result;
            }
            587 => {
                if lookahead == 110 { state = 720; lexer.advance(false); continue; }
                return result;
            }
            588 => {
                if lookahead == 97 { state = 721; lexer.advance(false); continue; }
                return result;
            }
            589 => {
                if lookahead == 116 { state = 722; lexer.advance(false); continue; }
                return result;
            }
            590 => {
                if lookahead == 117 { state = 723; lexer.advance(false); continue; }
                return result;
            }
            591 => {
                if lookahead == 114 { state = 724; lexer.advance(false); continue; }
                return result;
            }
            592 => {
                if lookahead == 111 { state = 725; lexer.advance(false); continue; }
                return result;
            }
            593 => {
                if lookahead == 103 { state = 726; lexer.advance(false); continue; }
                return result;
            }
            594 => {
                if lookahead == 116 { state = 727; lexer.advance(false); continue; }
                return result;
            }
            595 => {
                if lookahead == 97 { state = 728; lexer.advance(false); continue; }
                return result;
            }
            596 => {
                if lookahead == 101 { state = 729; lexer.advance(false); continue; }
                return result;
            }
            597 => {
                if lookahead == 101 { state = 730; lexer.advance(false); continue; }
                return result;
            }
            598 => {
                if lookahead == 99 { state = 731; lexer.advance(false); continue; }
                return result;
            }
            599 => {
                if lookahead == 115 { state = 732; lexer.advance(false); continue; }
                return result;
            }
            600 => {
                if lookahead == 99 { state = 733; lexer.advance(false); continue; }
                return result;
            }
            601 => {
                if lookahead == 99 { state = 734; lexer.advance(false); continue; }
                return result;
            }
            602 => {
                if lookahead == 101 { state = 735; lexer.advance(false); continue; }
                return result;
            }
            603 => {
                if lookahead == 117 { state = 736; lexer.advance(false); continue; }
                return result;
            }
            604 => {
                if lookahead == 101 { state = 737; lexer.advance(false); continue; }
                return result;
            }
            605 => {
                if lookahead == 117 { state = 738; lexer.advance(false); continue; }
                return result;
            }
            606 => {
                if lookahead == 107 { state = 739; lexer.advance(false); continue; }
                return result;
            }
            607 => {
                if lookahead == 109 { state = 740; lexer.advance(false); continue; }
                return result;
            }
            608 => {
                if lookahead == 103 { state = 741; lexer.advance(false); continue; }
                if lookahead == 112 { state = 742; lexer.advance(false); continue; }
                return result;
            }
            609 => {
                if lookahead == 117 { state = 743; lexer.advance(false); continue; }
                return result;
            }
            610 => {
                if lookahead == 99 { state = 744; lexer.advance(false); continue; }
                return result;
            }
            611 => {
                if lookahead == 107 { state = 745; lexer.advance(false); continue; }
                return result;
            }
            612 => {
                if lookahead == 97 { state = 746; lexer.advance(false); continue; }
                return result;
            }
            613 => {
                result = true; lexer.set_result_symbol(anon_sym_expect); lexer.mark_end();
                return result;
            }
            614 => {
                result = true; lexer.set_result_symbol(anon_sym_export); lexer.mark_end();
                return result;
            }
            615 => {
                if lookahead == 115 { state = 747; lexer.advance(false); continue; }
                return result;
            }
            616 => {
                result = true; lexer.set_result_symbol(anon_sym_extern); lexer.mark_end();
                return result;
            }
            617 => {
                if lookahead == 109 { state = 748; lexer.advance(false); continue; }
                return result;
            }
            618 => {
                if lookahead == 104 { state = 749; lexer.advance(false); continue; }
                return result;
            }
            619 => {
                if lookahead == 114 { state = 750; lexer.advance(false); continue; }
                return result;
            }
            620 => {
                if lookahead == 105 { state = 751; lexer.advance(false); continue; }
                return result;
            }
            621 => {
                if lookahead == 111 { state = 752; lexer.advance(false); continue; }
                return result;
            }
            622 => {
                if lookahead == 116 { state = 753; lexer.advance(false); continue; }
                return result;
            }
            623 => {
                result = true; lexer.set_result_symbol(anon_sym_genvar); lexer.mark_end();
                return result;
            }
            624 => {
                result = true; lexer.set_result_symbol(anon_sym_global); lexer.mark_end();
                return result;
            }
            625 => {
                result = true; lexer.set_result_symbol(anon_sym_highz0); lexer.mark_end();
                return result;
            }
            626 => {
                result = true; lexer.set_result_symbol(anon_sym_highz1); lexer.mark_end();
                return result;
            }
            627 => {
                result = true; lexer.set_result_symbol(anon_sym_ifnone); lexer.mark_end();
                return result;
            }
            628 => {
                if lookahead == 95 { state = 754; lexer.advance(false); continue; }
                return result;
            }
            629 => {
                if lookahead == 108 { state = 755; lexer.advance(false); continue; }
                return result;
            }
            630 => {
                if lookahead == 101 { state = 756; lexer.advance(false); continue; }
                return result;
            }
            631 => {
                if lookahead == 115 { state = 757; lexer.advance(false); continue; }
                return result;
            }
            632 => {
                result = true; lexer.set_result_symbol(anon_sym_import); lexer.mark_end();
                return result;
            }
            633 => {
                if lookahead == 108 { state = 758; lexer.advance(false); continue; }
                return result;
            }
            634 => {
                result = true; lexer.set_result_symbol(anon_sym_inside); lexer.mark_end();
                return result;
            }
            635 => {
                if lookahead == 114 { state = 759; lexer.advance(false); continue; }
                return result;
            }
            636 => {
                if lookahead == 111 { state = 760; lexer.advance(false); continue; }
                return result;
            }
            637 => {
                if lookahead == 97 { state = 761; lexer.advance(false); continue; }
                return result;
            }
            638 => {
                if lookahead == 101 { state = 762; lexer.advance(false); continue; }
                return result;
            }
            639 => {
                if lookahead == 110 { state = 763; lexer.advance(false); continue; }
                return result;
            }
            640 => {
                if lookahead == 111 { state = 764; lexer.advance(false); continue; }
                return result;
            }
            641 => {
                if lookahead == 97 { state = 765; lexer.advance(false); continue; }
                return result;
            }
            642 => {
                if lookahead == 116 { state = 766; lexer.advance(false); continue; }
                return result;
            }
            643 => {
                if lookahead == 111 { state = 767; lexer.advance(false); continue; }
                return result;
            }
            644 => {
                if lookahead == 115 { state = 768; lexer.advance(false); continue; }
                return result;
            }
            645 => {
                result = true; lexer.set_result_symbol(anon_sym_medium); lexer.mark_end();
                return result;
            }
            646 => {
                if lookahead == 116 { state = 769; lexer.advance(false); continue; }
                return result;
            }
            647 => {
                result = true; lexer.set_result_symbol(anon_sym_module); lexer.mark_end();
                return result;
            }
            648 => {
                if lookahead == 101 { state = 770; lexer.advance(false); continue; }
                return result;
            }
            649 => {
                if lookahead == 101 { state = 771; lexer.advance(false); continue; }
                return result;
            }
            650 => {
                if lookahead == 109 { state = 772; lexer.advance(false); continue; }
                return result;
            }
            651 => {
                if lookahead == 99 { state = 773; lexer.advance(false); continue; }
                return result;
            }
            652 => {
                result = true; lexer.set_result_symbol(anon_sym_notif0); lexer.mark_end();
                return result;
            }
            653 => {
                result = true; lexer.set_result_symbol(anon_sym_notif1); lexer.mark_end();
                return result;
            }
            654 => {
                result = true; lexer.set_result_symbol(anon_sym_option); lexer.mark_end();
                return result;
            }
            655 => {
                result = true; lexer.set_result_symbol(anon_sym_output); lexer.mark_end();
                return result;
            }
            656 => {
                if lookahead == 101 { state = 774; lexer.advance(false); continue; }
                return result;
            }
            657 => {
                result = true; lexer.set_result_symbol(anon_sym_packed); lexer.mark_end();
                return result;
            }
            658 => {
                if lookahead == 116 { state = 775; lexer.advance(false); continue; }
                return result;
            }
            659 => {
                if lookahead == 101 { state = 776; lexer.advance(false); continue; }
                return result;
            }
            660 => {
                if lookahead == 105 { state = 777; lexer.advance(false); continue; }
                return result;
            }
            661 => {
                if lookahead == 116 { state = 778; lexer.advance(false); continue; }
                return result;
            }
            662 => {
                if lookahead == 109 { state = 779; lexer.advance(false); continue; }
                return result;
            }
            663 => {
                if lookahead == 116 { state = 780; lexer.advance(false); continue; }
                return result;
            }
            664 => {
                if lookahead == 116 { state = 781; lexer.advance(false); continue; }
                return result;
            }
            665 => {
                if lookahead == 119 { state = 782; lexer.advance(false); continue; }
                return result;
            }
            666 => {
                result = true; lexer.set_result_symbol(anon_sym_pullup); lexer.mark_end();
                return result;
            }
            667 => {
                if lookahead == 116 { state = 783; lexer.advance(false); continue; }
                return result;
            }
            668 => {
                if lookahead == 115 { state = 784; lexer.advance(false); continue; }
                return result;
            }
            669 => {
                if lookahead == 105 { state = 785; lexer.advance(false); continue; }
                return result;
            }
            670 => {
                if lookahead == 109 { state = 786; lexer.advance(false); continue; }
                return result;
            }
            671 => {
                if lookahead == 95 { state = 787; lexer.advance(false); continue; }
                return result;
            }
            672 => {
                if lookahead == 101 { state = 788; lexer.advance(false); continue; }
                return result;
            }
            673 => {
                result = true; lexer.set_result_symbol(anon_sym_repeat); lexer.mark_end();
                return result;
            }
            674 => {
                if lookahead == 99 { state = 789; lexer.advance(false); continue; }
                return result;
            }
            675 => {
                result = true; lexer.set_result_symbol(anon_sym_return); lexer.mark_end();
                return result;
            }
            676 => {
                if lookahead == 102 { state = 790; lexer.advance(false); continue; }
                return result;
            }
            677 => {
                if lookahead == 121 { state = 791; lexer.advance(false); continue; }
                return result;
            }
            678 => {
                if lookahead == 116 { state = 792; lexer.advance(false); continue; }
                return result;
            }
            679 => {
                if lookahead == 116 { state = 793; lexer.advance(false); continue; }
                return result;
            }
            680 => {
                if lookahead == 108 { state = 794; lexer.advance(false); continue; }
                return result;
            }
            681 => {
                result = true; lexer.set_result_symbol(anon_sym_sample); lexer.mark_end();
                return result;
            }
            682 => {
                if lookahead == 101 { state = 795; lexer.advance(false); continue; }
                return result;
            }
            683 => {
                if lookahead == 99 { state = 796; lexer.advance(false); continue; }
                return result;
            }
            684 => {
                if lookahead == 110 { state = 797; lexer.advance(false); continue; }
                return result;
            }
            685 => {
                if lookahead == 101 { state = 798; lexer.advance(false); continue; }
                return result;
            }
            686 => {
                if lookahead == 110 { state = 799; lexer.advance(false); continue; }
                return result;
            }
            687 => {
                result = true; lexer.set_result_symbol(anon_sym_signed); lexer.mark_end();
                return result;
            }
            688 => {
                if lookahead == 121 { state = 800; lexer.advance(false); continue; }
                return result;
            }
            689 => {
                if lookahead == 114 { state = 801; lexer.advance(false); continue; }
                return result;
            }
            690 => {
                result = true; lexer.set_result_symbol(anon_sym_static); lexer.mark_end();
                return result;
            }
            691 => {
                result = true; lexer.set_result_symbol(anon_sym_string); lexer.mark_end();
                return result;
            }
            692 => {
                result = true; lexer.set_result_symbol(anon_sym_strong); lexer.mark_end();
                if lookahead == 48 { state = 802; lexer.advance(false); continue; }
                if lookahead == 49 { state = 803; lexer.advance(false); continue; }
                return result;
            }
            693 => {
                result = true; lexer.set_result_symbol(anon_sym_struct); lexer.mark_end();
                return result;
            }
            694 => {
                if lookahead == 48 { state = 804; lexer.advance(false); continue; }
                if lookahead == 49 { state = 805; lexer.advance(false); continue; }
                return result;
            }
            695 => {
                if lookahead == 99 { state = 806; lexer.advance(false); continue; }
                return result;
            }
            696 => {
                if lookahead == 101 { state = 807; lexer.advance(false); continue; }
                return result;
            }
            697 => {
                result = true; lexer.set_result_symbol(anon_sym_tagged); lexer.mark_end();
                return result;
            }
            698 => {
                if lookahead == 104 { state = 808; lexer.advance(false); continue; }
                return result;
            }
            699 => {
                if lookahead == 101 { state = 809; lexer.advance(false); continue; }
                return result;
            }
            700 => {
                if lookahead == 105 { state = 810; lexer.advance(false); continue; }
                return result;
            }
            701 => {
                if lookahead == 48 { state = 811; lexer.advance(false); continue; }
                if lookahead == 49 { state = 812; lexer.advance(false); continue; }
                return result;
            }
            702 => {
                result = true; lexer.set_result_symbol(anon_sym_triand); lexer.mark_end();
                return result;
            }
            703 => {
                result = true; lexer.set_result_symbol(anon_sym_trireg); lexer.mark_end();
                return result;
            }
            704 => {
                if lookahead == 112 { state = 813; lexer.advance(false); continue; }
                return result;
            }
            705 => {
                if lookahead == 102 { state = 814; lexer.advance(false); continue; }
                return result;
            }
            706 => {
                result = true; lexer.set_result_symbol(anon_sym_unique); lexer.mark_end();
                if lookahead == 48 { state = 815; lexer.advance(false); continue; }
                return result;
            }
            707 => {
                if lookahead == 101 { state = 816; lexer.advance(false); continue; }
                return result;
            }
            708 => {
                if lookahead == 119 { state = 817; lexer.advance(false); continue; }
                return result;
            }
            709 => {
                if lookahead == 100 { state = 818; lexer.advance(false); continue; }
                return result;
            }
            710 => {
                if lookahead == 101 { state = 819; lexer.advance(false); continue; }
                return result;
            }
            711 => {
                if lookahead == 108 { state = 820; lexer.advance(false); continue; }
                return result;
            }
            712 => {
                if lookahead == 114 { state = 821; lexer.advance(false); continue; }
                return result;
            }
            713 => {
                if lookahead == 114 { state = 822; lexer.advance(false); continue; }
                return result;
            }
            714 => {
                result = true; lexer.set_result_symbol(anon_sym_within); lexer.mark_end();
                return result;
            }
            715 => {
                if lookahead == 111 { state = 823; lexer.advance(false); continue; }
                return result;
            }
            716 => {
                if lookahead == 99 { state = 824; lexer.advance(false); continue; }
                if lookahead == 102 { state = 825; lexer.advance(false); continue; }
                if lookahead == 108 { state = 826; lexer.advance(false); continue; }
                return result;
            }
            717 => {
                if lookahead == 105 { state = 827; lexer.advance(false); continue; }
                return result;
            }
            718 => {
                result = true; lexer.set_result_symbol(anon_sym_chandle); lexer.mark_end();
                return result;
            }
            719 => {
                result = true; lexer.set_result_symbol(anon_sym_checker); lexer.mark_end();
                return result;
            }
            720 => {
                if lookahead == 103 { state = 828; lexer.advance(false); continue; }
                return result;
            }
            721 => {
                if lookahead == 105 { state = 829; lexer.advance(false); continue; }
                return result;
            }
            722 => {
                result = true; lexer.set_result_symbol(anon_sym_context); lexer.mark_end();
                return result;
            }
            723 => {
                if lookahead == 101 { state = 830; lexer.advance(false); continue; }
                return result;
            }
            724 => {
                if lookahead == 111 { state = 831; lexer.advance(false); continue; }
                return result;
            }
            725 => {
                if lookahead == 105 { state = 832; lexer.advance(false); continue; }
                return result;
            }
            726 => {
                if lookahead == 110 { state = 833; lexer.advance(false); continue; }
                return result;
            }
            727 => {
                result = true; lexer.set_result_symbol(anon_sym_default); lexer.mark_end();
                return result;
            }
            728 => {
                if lookahead == 109 { state = 834; lexer.advance(false); continue; }
                return result;
            }
            729 => {
                result = true; lexer.set_result_symbol(anon_sym_disable); lexer.mark_end();
                return result;
            }
            730 => {
                result = true; lexer.set_result_symbol(anon_sym_endcase); lexer.mark_end();
                return result;
            }
            731 => {
                if lookahead == 107 { state = 835; lexer.advance(false); continue; }
                return result;
            }
            732 => {
                if lookahead == 115 { state = 836; lexer.advance(false); continue; }
                return result;
            }
            733 => {
                if lookahead == 107 { state = 837; lexer.advance(false); continue; }
                return result;
            }
            734 => {
                if lookahead == 116 { state = 838; lexer.advance(false); continue; }
                return result;
            }
            735 => {
                if lookahead == 114 { state = 839; lexer.advance(false); continue; }
                return result;
            }
            736 => {
                if lookahead == 112 { state = 840; lexer.advance(false); continue; }
                return result;
            }
            737 => {
                if lookahead == 114 { state = 841; lexer.advance(false); continue; }
                return result;
            }
            738 => {
                if lookahead == 108 { state = 842; lexer.advance(false); continue; }
                return result;
            }
            739 => {
                if lookahead == 97 { state = 843; lexer.advance(false); continue; }
                return result;
            }
            740 => {
                if lookahead == 105 { state = 844; lexer.advance(false); continue; }
                return result;
            }
            741 => {
                if lookahead == 114 { state = 845; lexer.advance(false); continue; }
                return result;
            }
            742 => {
                if lookahead == 101 { state = 846; lexer.advance(false); continue; }
                return result;
            }
            743 => {
                if lookahead == 101 { state = 847; lexer.advance(false); continue; }
                return result;
            }
            744 => {
                if lookahead == 105 { state = 848; lexer.advance(false); continue; }
                return result;
            }
            745 => {
                result = true; lexer.set_result_symbol(anon_sym_endtask); lexer.mark_end();
                return result;
            }
            746 => {
                if lookahead == 108 { state = 849; lexer.advance(false); continue; }
                return result;
            }
            747 => {
                result = true; lexer.set_result_symbol(anon_sym_extends); lexer.mark_end();
                return result;
            }
            748 => {
                if lookahead == 97 { state = 850; lexer.advance(false); continue; }
                return result;
            }
            749 => {
                result = true; lexer.set_result_symbol(anon_sym_foreach); lexer.mark_end();
                return result;
            }
            750 => {
                result = true; lexer.set_result_symbol(anon_sym_forever); lexer.mark_end();
                return result;
            }
            751 => {
                if lookahead == 110 { state = 851; lexer.advance(false); continue; }
                return result;
            }
            752 => {
                if lookahead == 110 { state = 852; lexer.advance(false); continue; }
                return result;
            }
            753 => {
                if lookahead == 101 { state = 853; lexer.advance(false); continue; }
                return result;
            }
            754 => {
                if lookahead == 98 { state = 854; lexer.advance(false); continue; }
                return result;
            }
            755 => {
                if lookahead == 95 { state = 855; lexer.advance(false); continue; }
                return result;
            }
            756 => {
                if lookahead == 110 { state = 856; lexer.advance(false); continue; }
                return result;
            }
            757 => {
                result = true; lexer.set_result_symbol(anon_sym_implies); lexer.mark_end();
                return result;
            }
            758 => {
                result = true; lexer.set_result_symbol(anon_sym_initial); lexer.mark_end();
                return result;
            }
            759 => {
                result = true; lexer.set_result_symbol(anon_sym_integer); lexer.mark_end();
                return result;
            }
            760 => {
                if lookahead == 110 { state = 857; lexer.advance(false); continue; }
                return result;
            }
            761 => {
                if lookahead == 99 { state = 858; lexer.advance(false); continue; }
                return result;
            }
            762 => {
                if lookahead == 99 { state = 859; lexer.advance(false); continue; }
                return result;
            }
            763 => {
                if lookahead == 121 { state = 860; lexer.advance(false); continue; }
                return result;
            }
            764 => {
                if lookahead == 110 { state = 861; lexer.advance(false); continue; }
                return result;
            }
            765 => {
                if lookahead == 114 { state = 862; lexer.advance(false); continue; }
                return result;
            }
            766 => {
                result = true; lexer.set_result_symbol(anon_sym_longint); lexer.mark_end();
                return result;
            }
            767 => {
                if lookahead == 100 { state = 863; lexer.advance(false); continue; }
                return result;
            }
            768 => {
                result = true; lexer.set_result_symbol(anon_sym_matches); lexer.mark_end();
                return result;
            }
            769 => {
                result = true; lexer.set_result_symbol(anon_sym_modport); lexer.mark_end();
                return result;
            }
            770 => {
                result = true; lexer.set_result_symbol(anon_sym_negedge); lexer.mark_end();
                return result;
            }
            771 => {
                result = true; lexer.set_result_symbol(anon_sym_nettype); lexer.mark_end();
                return result;
            }
            772 => {
                if lookahead == 101 { state = 864; lexer.advance(false); continue; }
                return result;
            }
            773 => {
                if lookahead == 97 { state = 865; lexer.advance(false); continue; }
                return result;
            }
            774 => {
                result = true; lexer.set_result_symbol(anon_sym_package); lexer.mark_end();
                return result;
            }
            775 => {
                if lookahead == 101 { state = 866; lexer.advance(false); continue; }
                return result;
            }
            776 => {
                result = true; lexer.set_result_symbol(anon_sym_posedge); lexer.mark_end();
                return result;
            }
            777 => {
                if lookahead == 118 { state = 867; lexer.advance(false); continue; }
                return result;
            }
            778 => {
                if lookahead == 121 { state = 868; lexer.advance(false); continue; }
                return result;
            }
            779 => {
                result = true; lexer.set_result_symbol(anon_sym_program); lexer.mark_end();
                return result;
            }
            780 => {
                if lookahead == 121 { state = 869; lexer.advance(false); continue; }
                return result;
            }
            781 => {
                if lookahead == 101 { state = 870; lexer.advance(false); continue; }
                return result;
            }
            782 => {
                if lookahead == 110 { state = 871; lexer.advance(false); continue; }
                return result;
            }
            783 => {
                if lookahead == 121 { state = 872; lexer.advance(false); continue; }
                return result;
            }
            784 => {
                if lookahead == 101 { state = 873; lexer.advance(false); continue; }
                return result;
            }
            785 => {
                if lookahead == 122 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            786 => {
                if lookahead == 101 { state = 875; lexer.advance(false); continue; }
                return result;
            }
            787 => {
                if lookahead == 111 { state = 876; lexer.advance(false); continue; }
                return result;
            }
            788 => {
                result = true; lexer.set_result_symbol(anon_sym_release); lexer.mark_end();
                return result;
            }
            789 => {
                if lookahead == 116 { state = 877; lexer.advance(false); continue; }
                return result;
            }
            790 => {
                if lookahead == 48 { state = 878; lexer.advance(false); continue; }
                if lookahead == 49 { state = 879; lexer.advance(false); continue; }
                return result;
            }
            791 => {
                if lookahead == 115 { state = 880; lexer.advance(false); continue; }
                return result;
            }
            792 => {
                if lookahead == 117 { state = 881; lexer.advance(false); continue; }
                return result;
            }
            793 => {
                if lookahead == 105 { state = 882; lexer.advance(false); continue; }
                return result;
            }
            794 => {
                result = true; lexer.set_result_symbol(anon_sym_s_until); lexer.mark_end();
                if lookahead == 95 { state = 883; lexer.advance(false); continue; }
                return result;
            }
            795 => {
                if lookahead == 100 { state = 884; lexer.advance(false); continue; }
                return result;
            }
            796 => {
                if lookahead == 101 { state = 885; lexer.advance(false); continue; }
                return result;
            }
            797 => {
                if lookahead == 116 { state = 886; lexer.advance(false); continue; }
                return result;
            }
            798 => {
                if lookahead == 97 { state = 887; lexer.advance(false); continue; }
                return result;
            }
            799 => {
                if lookahead == 99 { state = 888; lexer.advance(false); continue; }
                return result;
            }
            800 => {
                result = true; lexer.set_result_symbol(anon_sym_specify); lexer.mark_end();
                return result;
            }
            801 => {
                if lookahead == 97 { state = 889; lexer.advance(false); continue; }
                return result;
            }
            802 => {
                result = true; lexer.set_result_symbol(anon_sym_strong0); lexer.mark_end();
                return result;
            }
            803 => {
                result = true; lexer.set_result_symbol(anon_sym_strong1); lexer.mark_end();
                return result;
            }
            804 => {
                result = true; lexer.set_result_symbol(anon_sym_supply0); lexer.mark_end();
                return result;
            }
            805 => {
                result = true; lexer.set_result_symbol(anon_sym_supply1); lexer.mark_end();
                return result;
            }
            806 => {
                if lookahead == 99 { state = 890; lexer.advance(false); continue; }
                return result;
            }
            807 => {
                if lookahead == 106 { state = 891; lexer.advance(false); continue; }
                return result;
            }
            808 => {
                if lookahead == 111 { state = 892; lexer.advance(false); continue; }
                return result;
            }
            809 => {
                if lookahead == 99 { state = 893; lexer.advance(false); continue; }
                return result;
            }
            810 => {
                if lookahead == 116 { state = 894; lexer.advance(false); continue; }
                return result;
            }
            811 => {
                result = true; lexer.set_result_symbol(anon_sym_tranif0); lexer.mark_end();
                return result;
            }
            812 => {
                result = true; lexer.set_result_symbol(anon_sym_tranif1); lexer.mark_end();
                return result;
            }
            813 => {
                if lookahead == 116 { state = 895; lexer.advance(false); continue; }
                return result;
            }
            814 => {
                result = true; lexer.set_result_symbol(anon_sym_typedef); lexer.mark_end();
                return result;
            }
            815 => {
                result = true; lexer.set_result_symbol(anon_sym_unique0); lexer.mark_end();
                return result;
            }
            816 => {
                if lookahead == 100 { state = 896; lexer.advance(false); continue; }
                return result;
            }
            817 => {
                if lookahead == 105 { state = 897; lexer.advance(false); continue; }
                return result;
            }
            818 => {
                result = true; lexer.set_result_symbol(anon_sym_untyped); lexer.mark_end();
                return result;
            }
            819 => {
                if lookahead == 100 { state = 898; lexer.advance(false); continue; }
                return result;
            }
            820 => {
                result = true; lexer.set_result_symbol(anon_sym_virtual); lexer.mark_end();
                return result;
            }
            821 => {
                if lookahead == 100 { state = 899; lexer.advance(false); continue; }
                return result;
            }
            822 => {
                if lookahead == 100 { state = 900; lexer.advance(false); continue; }
                return result;
            }
            823 => {
                if lookahead == 110 { state = 901; lexer.advance(false); continue; }
                return result;
            }
            824 => {
                if lookahead == 111 { state = 902; lexer.advance(false); continue; }
                return result;
            }
            825 => {
                if lookahead == 102 { state = 903; lexer.advance(false); continue; }
                return result;
            }
            826 => {
                if lookahead == 97 { state = 904; lexer.advance(false); continue; }
                return result;
            }
            827 => {
                if lookahead == 99 { state = 905; lexer.advance(false); continue; }
                return result;
            }
            828 => {
                result = true; lexer.set_result_symbol(anon_sym_clocking); lexer.mark_end();
                return result;
            }
            829 => {
                if lookahead == 110 { state = 906; lexer.advance(false); continue; }
                return result;
            }
            830 => {
                result = true; lexer.set_result_symbol(anon_sym_continue); lexer.mark_end();
                return result;
            }
            831 => {
                if lookahead == 117 { state = 907; lexer.advance(false); continue; }
                return result;
            }
            832 => {
                if lookahead == 110 { state = 908; lexer.advance(false); continue; }
                return result;
            }
            833 => {
                result = true; lexer.set_result_symbol(anon_sym_deassign); lexer.mark_end();
                return result;
            }
            834 => {
                result = true; lexer.set_result_symbol(anon_sym_defparam); lexer.mark_end();
                return result;
            }
            835 => {
                if lookahead == 101 { state = 909; lexer.advance(false); continue; }
                return result;
            }
            836 => {
                result = true; lexer.set_result_symbol(anon_sym_endclass); lexer.mark_end();
                return result;
            }
            837 => {
                if lookahead == 105 { state = 910; lexer.advance(false); continue; }
                return result;
            }
            838 => {
                if lookahead == 105 { state = 911; lexer.advance(false); continue; }
                return result;
            }
            839 => {
                if lookahead == 97 { state = 912; lexer.advance(false); continue; }
                return result;
            }
            840 => {
                result = true; lexer.set_result_symbol(anon_sym_endgroup); lexer.mark_end();
                return result;
            }
            841 => {
                if lookahead == 102 { state = 913; lexer.advance(false); continue; }
                return result;
            }
            842 => {
                if lookahead == 101 { state = 914; lexer.advance(false); continue; }
                return result;
            }
            843 => {
                if lookahead == 103 { state = 915; lexer.advance(false); continue; }
                return result;
            }
            844 => {
                if lookahead == 116 { state = 916; lexer.advance(false); continue; }
                return result;
            }
            845 => {
                if lookahead == 97 { state = 917; lexer.advance(false); continue; }
                return result;
            }
            846 => {
                if lookahead == 114 { state = 918; lexer.advance(false); continue; }
                return result;
            }
            847 => {
                if lookahead == 110 { state = 919; lexer.advance(false); continue; }
                return result;
            }
            848 => {
                if lookahead == 102 { state = 920; lexer.advance(false); continue; }
                return result;
            }
            849 => {
                if lookahead == 108 { state = 921; lexer.advance(false); continue; }
                return result;
            }
            850 => {
                if lookahead == 116 { state = 922; lexer.advance(false); continue; }
                return result;
            }
            851 => {
                result = true; lexer.set_result_symbol(anon_sym_forkjoin); lexer.mark_end();
                return result;
            }
            852 => {
                result = true; lexer.set_result_symbol(anon_sym_function); lexer.mark_end();
                return result;
            }
            853 => {
                result = true; lexer.set_result_symbol(anon_sym_generate); lexer.mark_end();
                return result;
            }
            854 => {
                if lookahead == 105 { state = 923; lexer.advance(false); continue; }
                return result;
            }
            855 => {
                if lookahead == 98 { state = 924; lexer.advance(false); continue; }
                return result;
            }
            856 => {
                if lookahead == 116 { state = 925; lexer.advance(false); continue; }
                return result;
            }
            857 => {
                if lookahead == 110 { state = 926; lexer.advance(false); continue; }
                return result;
            }
            858 => {
                if lookahead == 101 { state = 927; lexer.advance(false); continue; }
                return result;
            }
            859 => {
                if lookahead == 116 { state = 928; lexer.advance(false); continue; }
                return result;
            }
            860 => {
                result = true; lexer.set_result_symbol(anon_sym_join_any); lexer.mark_end();
                return result;
            }
            861 => {
                if lookahead == 101 { state = 929; lexer.advance(false); continue; }
                return result;
            }
            862 => {
                if lookahead == 97 { state = 930; lexer.advance(false); continue; }
                return result;
            }
            863 => {
                if lookahead == 117 { state = 931; lexer.advance(false); continue; }
                return result;
            }
            864 => {
                result = true; lexer.set_result_symbol(anon_sym_nexttime); lexer.mark_end();
                return result;
            }
            865 => {
                if lookahead == 110 { state = 932; lexer.advance(false); continue; }
                return result;
            }
            866 => {
                if lookahead == 114 { state = 933; lexer.advance(false); continue; }
                return result;
            }
            867 => {
                if lookahead == 101 { state = 934; lexer.advance(false); continue; }
                return result;
            }
            868 => {
                result = true; lexer.set_result_symbol(anon_sym_priority); lexer.mark_end();
                return result;
            }
            869 => {
                result = true; lexer.set_result_symbol(anon_sym_property); lexer.mark_end();
                return result;
            }
            870 => {
                if lookahead == 100 { state = 935; lexer.advance(false); continue; }
                return result;
            }
            871 => {
                result = true; lexer.set_result_symbol(anon_sym_pulldown); lexer.mark_end();
                return result;
            }
            872 => {
                if lookahead == 108 { state = 936; lexer.advance(false); continue; }
                return result;
            }
            873 => {
                result = true; lexer.set_result_symbol(anon_sym_randcase); lexer.mark_end();
                return result;
            }
            874 => {
                if lookahead == 101 { state = 937; lexer.advance(false); continue; }
                return result;
            }
            875 => {
                result = true; lexer.set_result_symbol(anon_sym_realtime); lexer.mark_end();
                return result;
            }
            876 => {
                if lookahead == 110 { state = 938; lexer.advance(false); continue; }
                return result;
            }
            877 => {
                result = true; lexer.set_result_symbol(anon_sym_restrict); lexer.mark_end();
                return result;
            }
            878 => {
                result = true; lexer.set_result_symbol(anon_sym_rtranif0); lexer.mark_end();
                return result;
            }
            879 => {
                result = true; lexer.set_result_symbol(anon_sym_rtranif1); lexer.mark_end();
                return result;
            }
            880 => {
                result = true; lexer.set_result_symbol(anon_sym_s_always); lexer.mark_end();
                return result;
            }
            881 => {
                if lookahead == 97 { state = 939; lexer.advance(false); continue; }
                return result;
            }
            882 => {
                if lookahead == 109 { state = 940; lexer.advance(false); continue; }
                return result;
            }
            883 => {
                if lookahead == 119 { state = 941; lexer.advance(false); continue; }
                return result;
            }
            884 => {
                result = true; lexer.set_result_symbol(anon_sym_scalared); lexer.mark_end();
                return result;
            }
            885 => {
                result = true; lexer.set_result_symbol(anon_sym_sequence); lexer.mark_end();
                return result;
            }
            886 => {
                result = true; lexer.set_result_symbol(anon_sym_shortint); lexer.mark_end();
                return result;
            }
            887 => {
                if lookahead == 108 { state = 942; lexer.advance(false); continue; }
                return result;
            }
            888 => {
                if lookahead == 101 { state = 943; lexer.advance(false); continue; }
                return result;
            }
            889 => {
                if lookahead == 109 { state = 944; lexer.advance(false); continue; }
                return result;
            }
            890 => {
                if lookahead == 101 { state = 945; lexer.advance(false); continue; }
                return result;
            }
            891 => {
                if lookahead == 101 { state = 946; lexer.advance(false); continue; }
                return result;
            }
            892 => {
                if lookahead == 117 { state = 947; lexer.advance(false); continue; }
                return result;
            }
            893 => {
                if lookahead == 105 { state = 948; lexer.advance(false); continue; }
                return result;
            }
            894 => {
                result = true; lexer.set_result_symbol(anon_sym_timeunit); lexer.mark_end();
                return result;
            }
            895 => {
                if lookahead == 105 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            896 => {
                result = true; lexer.set_result_symbol(anon_sym_unsigned); lexer.mark_end();
                return result;
            }
            897 => {
                if lookahead == 116 { state = 950; lexer.advance(false); continue; }
                return result;
            }
            898 => {
                result = true; lexer.set_result_symbol(anon_sym_vectored); lexer.mark_end();
                return result;
            }
            899 => {
                if lookahead == 101 { state = 951; lexer.advance(false); continue; }
                return result;
            }
            900 => {
                result = true; lexer.set_result_symbol(anon_sym_wildcard); lexer.mark_end();
                return result;
            }
            901 => {
                result = true; lexer.set_result_symbol(anon_sym_accept_on); lexer.mark_end();
                return result;
            }
            902 => {
                if lookahead == 109 { state = 952; lexer.advance(false); continue; }
                return result;
            }
            903 => {
                result = true; lexer.set_result_symbol(anon_sym_always_ff); lexer.mark_end();
                return result;
            }
            904 => {
                if lookahead == 116 { state = 953; lexer.advance(false); continue; }
                return result;
            }
            905 => {
                result = true; lexer.set_result_symbol(anon_sym_automatic); lexer.mark_end();
                return result;
            }
            906 => {
                if lookahead == 116 { state = 954; lexer.advance(false); continue; }
                return result;
            }
            907 => {
                if lookahead == 112 { state = 955; lexer.advance(false); continue; }
                return result;
            }
            908 => {
                if lookahead == 116 { state = 956; lexer.advance(false); continue; }
                return result;
            }
            909 => {
                if lookahead == 114 { state = 957; lexer.advance(false); continue; }
                return result;
            }
            910 => {
                if lookahead == 110 { state = 958; lexer.advance(false); continue; }
                return result;
            }
            911 => {
                if lookahead == 111 { state = 959; lexer.advance(false); continue; }
                return result;
            }
            912 => {
                if lookahead == 116 { state = 960; lexer.advance(false); continue; }
                return result;
            }
            913 => {
                if lookahead == 97 { state = 961; lexer.advance(false); continue; }
                return result;
            }
            914 => {
                result = true; lexer.set_result_symbol(anon_sym_endmodule); lexer.mark_end();
                return result;
            }
            915 => {
                if lookahead == 101 { state = 962; lexer.advance(false); continue; }
                return result;
            }
            916 => {
                if lookahead == 105 { state = 963; lexer.advance(false); continue; }
                return result;
            }
            917 => {
                if lookahead == 109 { state = 964; lexer.advance(false); continue; }
                return result;
            }
            918 => {
                if lookahead == 116 { state = 965; lexer.advance(false); continue; }
                return result;
            }
            919 => {
                if lookahead == 99 { state = 966; lexer.advance(false); continue; }
                return result;
            }
            920 => {
                if lookahead == 121 { state = 967; lexer.advance(false); continue; }
                return result;
            }
            921 => {
                if lookahead == 121 { state = 968; lexer.advance(false); continue; }
                return result;
            }
            922 => {
                if lookahead == 99 { state = 969; lexer.advance(false); continue; }
                return result;
            }
            923 => {
                if lookahead == 110 { state = 970; lexer.advance(false); continue; }
                return result;
            }
            924 => {
                if lookahead == 105 { state = 971; lexer.advance(false); continue; }
                return result;
            }
            925 => {
                if lookahead == 115 { state = 972; lexer.advance(false); continue; }
                return result;
            }
            926 => {
                if lookahead == 101 { state = 973; lexer.advance(false); continue; }
                return result;
            }
            927 => {
                result = true; lexer.set_result_symbol(anon_sym_interface); lexer.mark_end();
                return result;
            }
            928 => {
                result = true; lexer.set_result_symbol(anon_sym_intersect); lexer.mark_end();
                return result;
            }
            929 => {
                result = true; lexer.set_result_symbol(anon_sym_join_none); lexer.mark_end();
                return result;
            }
            930 => {
                if lookahead == 109 { state = 974; lexer.advance(false); continue; }
                return result;
            }
            931 => {
                if lookahead == 108 { state = 975; lexer.advance(false); continue; }
                return result;
            }
            932 => {
                if lookahead == 99 { state = 976; lexer.advance(false); continue; }
                return result;
            }
            933 => {
                result = true; lexer.set_result_symbol(anon_sym_parameter); lexer.mark_end();
                return result;
            }
            934 => {
                result = true; lexer.set_result_symbol(anon_sym_primitive); lexer.mark_end();
                return result;
            }
            935 => {
                result = true; lexer.set_result_symbol(anon_sym_protected); lexer.mark_end();
                return result;
            }
            936 => {
                if lookahead == 101 { state = 977; lexer.advance(false); continue; }
                return result;
            }
            937 => {
                result = true; lexer.set_result_symbol(anon_sym_randomize); lexer.mark_end();
                return result;
            }
            938 => {
                result = true; lexer.set_result_symbol(anon_sym_reject_on); lexer.mark_end();
                return result;
            }
            939 => {
                if lookahead == 108 { state = 978; lexer.advance(false); continue; }
                return result;
            }
            940 => {
                if lookahead == 101 { state = 979; lexer.advance(false); continue; }
                return result;
            }
            941 => {
                if lookahead == 105 { state = 980; lexer.advance(false); continue; }
                return result;
            }
            942 => {
                result = true; lexer.set_result_symbol(anon_sym_shortreal); lexer.mark_end();
                return result;
            }
            943 => {
                if lookahead == 108 { state = 981; lexer.advance(false); continue; }
                return result;
            }
            944 => {
                result = true; lexer.set_result_symbol(anon_sym_specparam); lexer.mark_end();
                return result;
            }
            945 => {
                if lookahead == 112 { state = 982; lexer.advance(false); continue; }
                return result;
            }
            946 => {
                if lookahead == 99 { state = 983; lexer.advance(false); continue; }
                return result;
            }
            947 => {
                if lookahead == 116 { state = 984; lexer.advance(false); continue; }
                return result;
            }
            948 => {
                if lookahead == 115 { state = 985; lexer.advance(false); continue; }
                return result;
            }
            949 => {
                if lookahead == 111 { state = 986; lexer.advance(false); continue; }
                return result;
            }
            950 => {
                if lookahead == 104 { state = 987; lexer.advance(false); continue; }
                return result;
            }
            951 => {
                if lookahead == 114 { state = 988; lexer.advance(false); continue; }
                return result;
            }
            952 => {
                if lookahead == 98 { state = 989; lexer.advance(false); continue; }
                return result;
            }
            953 => {
                if lookahead == 99 { state = 990; lexer.advance(false); continue; }
                return result;
            }
            954 => {
                result = true; lexer.set_result_symbol(anon_sym_constraint); lexer.mark_end();
                return result;
            }
            955 => {
                result = true; lexer.set_result_symbol(anon_sym_covergroup); lexer.mark_end();
                return result;
            }
            956 => {
                result = true; lexer.set_result_symbol(anon_sym_coverpoint); lexer.mark_end();
                return result;
            }
            957 => {
                result = true; lexer.set_result_symbol(anon_sym_endchecker); lexer.mark_end();
                return result;
            }
            958 => {
                if lookahead == 103 { state = 991; lexer.advance(false); continue; }
                return result;
            }
            959 => {
                if lookahead == 110 { state = 992; lexer.advance(false); continue; }
                return result;
            }
            960 => {
                if lookahead == 101 { state = 993; lexer.advance(false); continue; }
                return result;
            }
            961 => {
                if lookahead == 99 { state = 994; lexer.advance(false); continue; }
                return result;
            }
            962 => {
                result = true; lexer.set_result_symbol(anon_sym_endpackage); lexer.mark_end();
                return result;
            }
            963 => {
                if lookahead == 118 { state = 995; lexer.advance(false); continue; }
                return result;
            }
            964 => {
                result = true; lexer.set_result_symbol(anon_sym_endprogram); lexer.mark_end();
                return result;
            }
            965 => {
                if lookahead == 121 { state = 996; lexer.advance(false); continue; }
                return result;
            }
            966 => {
                if lookahead == 101 { state = 997; lexer.advance(false); continue; }
                return result;
            }
            967 => {
                result = true; lexer.set_result_symbol(anon_sym_endspecify); lexer.mark_end();
                return result;
            }
            968 => {
                result = true; lexer.set_result_symbol(anon_sym_eventually); lexer.mark_end();
                return result;
            }
            969 => {
                if lookahead == 104 { state = 998; lexer.advance(false); continue; }
                return result;
            }
            970 => {
                if lookahead == 115 { state = 999; lexer.advance(false); continue; }
                return result;
            }
            971 => {
                if lookahead == 110 { state = 1000; lexer.advance(false); continue; }
                return result;
            }
            972 => {
                result = true; lexer.set_result_symbol(anon_sym_implements); lexer.mark_end();
                return result;
            }
            973 => {
                if lookahead == 99 { state = 1001; lexer.advance(false); continue; }
                return result;
            }
            974 => {
                result = true; lexer.set_result_symbol(anon_sym_localparam); lexer.mark_end();
                return result;
            }
            975 => {
                if lookahead == 101 { state = 1002; lexer.advance(false); continue; }
                return result;
            }
            976 => {
                if lookahead == 101 { state = 1003; lexer.advance(false); continue; }
                return result;
            }
            977 => {
                if lookahead == 95 { state = 1004; lexer.advance(false); continue; }
                return result;
            }
            978 => {
                if lookahead == 108 { state = 1005; lexer.advance(false); continue; }
                return result;
            }
            979 => {
                result = true; lexer.set_result_symbol(anon_sym_s_nexttime); lexer.mark_end();
                return result;
            }
            980 => {
                if lookahead == 116 { state = 1006; lexer.advance(false); continue; }
                return result;
            }
            981 => {
                if lookahead == 108 { state = 1007; lexer.advance(false); continue; }
                return result;
            }
            982 => {
                if lookahead == 116 { state = 1008; lexer.advance(false); continue; }
                return result;
            }
            983 => {
                if lookahead == 116 { state = 1009; lexer.advance(false); continue; }
                return result;
            }
            984 => {
                result = true; lexer.set_result_symbol(anon_sym_throughout); lexer.mark_end();
                return result;
            }
            985 => {
                if lookahead == 105 { state = 1010; lexer.advance(false); continue; }
                return result;
            }
            986 => {
                if lookahead == 110 { state = 1011; lexer.advance(false); continue; }
                return result;
            }
            987 => {
                result = true; lexer.set_result_symbol(anon_sym_until_with); lexer.mark_end();
                return result;
            }
            988 => {
                result = true; lexer.set_result_symbol(anon_sym_wait_order); lexer.mark_end();
                return result;
            }
            989 => {
                result = true; lexer.set_result_symbol(anon_sym_always_comb); lexer.mark_end();
                return result;
            }
            990 => {
                if lookahead == 104 { state = 1012; lexer.advance(false); continue; }
                return result;
            }
            991 => {
                result = true; lexer.set_result_symbol(anon_sym_endclocking); lexer.mark_end();
                return result;
            }
            992 => {
                result = true; lexer.set_result_symbol(anon_sym_endfunction); lexer.mark_end();
                return result;
            }
            993 => {
                result = true; lexer.set_result_symbol(anon_sym_endgenerate); lexer.mark_end();
                return result;
            }
            994 => {
                if lookahead == 101 { state = 1013; lexer.advance(false); continue; }
                return result;
            }
            995 => {
                if lookahead == 101 { state = 1014; lexer.advance(false); continue; }
                return result;
            }
            996 => {
                result = true; lexer.set_result_symbol(anon_sym_endproperty); lexer.mark_end();
                return result;
            }
            997 => {
                result = true; lexer.set_result_symbol(anon_sym_endsequence); lexer.mark_end();
                return result;
            }
            998 => {
                result = true; lexer.set_result_symbol(anon_sym_first_match); lexer.mark_end();
                return result;
            }
            999 => {
                result = true; lexer.set_result_symbol(anon_sym_ignore_bins); lexer.mark_end();
                return result;
            }
            1000 => {
                if lookahead == 115 { state = 1015; lexer.advance(false); continue; }
                return result;
            }
            1001 => {
                if lookahead == 116 { state = 1016; lexer.advance(false); continue; }
                return result;
            }
            1002 => {
                result = true; lexer.set_result_symbol(anon_sym_macromodule); lexer.mark_end();
                return result;
            }
            1003 => {
                if lookahead == 108 { state = 1017; lexer.advance(false); continue; }
                return result;
            }
            1004 => {
                if lookahead == 111 { state = 1018; lexer.advance(false); continue; }
                return result;
            }
            1005 => {
                if lookahead == 121 { state = 1019; lexer.advance(false); continue; }
                return result;
            }
            1006 => {
                if lookahead == 104 { state = 1020; lexer.advance(false); continue; }
                return result;
            }
            1007 => {
                if lookahead == 101 { state = 1021; lexer.advance(false); continue; }
                return result;
            }
            1008 => {
                if lookahead == 95 { state = 1022; lexer.advance(false); continue; }
                return result;
            }
            1009 => {
                if lookahead == 95 { state = 1023; lexer.advance(false); continue; }
                return result;
            }
            1010 => {
                if lookahead == 111 { state = 1024; lexer.advance(false); continue; }
                return result;
            }
            1011 => {
                result = true; lexer.set_result_symbol(anon_sym_type_option); lexer.mark_end();
                return result;
            }
            1012 => {
                result = true; lexer.set_result_symbol(anon_sym_always_latch); lexer.mark_end();
                return result;
            }
            1013 => {
                result = true; lexer.set_result_symbol(anon_sym_endinterface); lexer.mark_end();
                return result;
            }
            1014 => {
                result = true; lexer.set_result_symbol(anon_sym_endprimitive); lexer.mark_end();
                return result;
            }
            1015 => {
                result = true; lexer.set_result_symbol(anon_sym_illegal_bins); lexer.mark_end();
                return result;
            }
            1016 => {
                result = true; lexer.set_result_symbol(anon_sym_interconnect); lexer.mark_end();
                return result;
            }
            1017 => {
                if lookahead == 108 { state = 1025; lexer.advance(false); continue; }
                return result;
            }
            1018 => {
                if lookahead == 110 { state = 1026; lexer.advance(false); continue; }
                return result;
            }
            1019 => {
                result = true; lexer.set_result_symbol(anon_sym_s_eventually); lexer.mark_end();
                return result;
            }
            1020 => {
                result = true; lexer.set_result_symbol(anon_sym_s_until_with); lexer.mark_end();
                return result;
            }
            1021 => {
                if lookahead == 100 { state = 1027; lexer.advance(false); continue; }
                return result;
            }
            1022 => {
                if lookahead == 111 { state = 1028; lexer.advance(false); continue; }
                return result;
            }
            1023 => {
                if lookahead == 111 { state = 1029; lexer.advance(false); continue; }
                return result;
            }
            1024 => {
                if lookahead == 110 { state = 1030; lexer.advance(false); continue; }
                return result;
            }
            1025 => {
                if lookahead == 101 { state = 1031; lexer.advance(false); continue; }
                return result;
            }
            1026 => {
                if lookahead == 100 { state = 1032; lexer.advance(false); continue; }
                if lookahead == 101 { state = 1033; lexer.advance(false); continue; }
                return result;
            }
            1027 => {
                result = true; lexer.set_result_symbol(anon_sym_showcancelled); lexer.mark_end();
                return result;
            }
            1028 => {
                if lookahead == 110 { state = 1034; lexer.advance(false); continue; }
                return result;
            }
            1029 => {
                if lookahead == 110 { state = 1035; lexer.advance(false); continue; }
                return result;
            }
            1030 => {
                result = true; lexer.set_result_symbol(anon_sym_timeprecision); lexer.mark_end();
                return result;
            }
            1031 => {
                if lookahead == 100 { state = 1036; lexer.advance(false); continue; }
                return result;
            }
            1032 => {
                if lookahead == 101 { state = 1037; lexer.advance(false); continue; }
                return result;
            }
            1033 => {
                if lookahead == 118 { state = 1038; lexer.advance(false); continue; }
                return result;
            }
            1034 => {
                result = true; lexer.set_result_symbol(anon_sym_sync_accept_on); lexer.mark_end();
                return result;
            }
            1035 => {
                result = true; lexer.set_result_symbol(anon_sym_sync_reject_on); lexer.mark_end();
                return result;
            }
            1036 => {
                result = true; lexer.set_result_symbol(anon_sym_noshowcancelled); lexer.mark_end();
                return result;
            }
            1037 => {
                if lookahead == 116 { state = 1039; lexer.advance(false); continue; }
                return result;
            }
            1038 => {
                if lookahead == 101 { state = 1040; lexer.advance(false); continue; }
                return result;
            }
            1039 => {
                if lookahead == 101 { state = 1041; lexer.advance(false); continue; }
                return result;
            }
            1040 => {
                if lookahead == 110 { state = 1042; lexer.advance(false); continue; }
                return result;
            }
            1041 => {
                if lookahead == 99 { state = 1043; lexer.advance(false); continue; }
                return result;
            }
            1042 => {
                if lookahead == 116 { state = 1044; lexer.advance(false); continue; }
                return result;
            }
            1043 => {
                if lookahead == 116 { state = 1045; lexer.advance(false); continue; }
                return result;
            }
            1044 => {
                result = true; lexer.set_result_symbol(anon_sym_pulsestyle_onevent); lexer.mark_end();
                return result;
            }
            1045 => {
                result = true; lexer.set_result_symbol(anon_sym_pulsestyle_ondetect); lexer.mark_end();
                return result;
            }
            _ => return false,
        }
    }
}
