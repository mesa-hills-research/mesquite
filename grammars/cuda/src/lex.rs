//! The `cuda` grammar's lexer: `ts_lex` and `ts_lex_keywords`, transliterated from
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

const anon_sym_AMP: Symbol = 33;
const anon_sym_AMP_AMP: Symbol = 30;
const anon_sym_AMP_EQ: Symbol = 139;
const anon_sym_BANG: Symbol = 22;
const anon_sym_BANG_EQ: Symbol = 35;
const anon_sym_CARET: Symbol = 32;
const anon_sym_CARET_CARET: Symbol = 227;
const anon_sym_CARET_EQ: Symbol = 140;
const anon_sym_COLON: Symbol = 57;
const anon_sym_COLON_COLON: Symbol = 58;
const anon_sym_COLON_RBRACK: Symbol = 229;
const anon_sym_COMMA: Symbol = 7;
const anon_sym_DASH: Symbol = 24;
const anon_sym_DASH_DASH: Symbol = 154;
const anon_sym_DASH_EQ: Symbol = 136;
const anon_sym_DASH_GT: Symbol = 171;
const anon_sym_DASH_GT_STAR: Symbol = 226;
const anon_sym_DOT: Symbol = 169;
const anon_sym_DOT_DOT_DOT: Symbol = 6;
const anon_sym_DOT_STAR: Symbol = 170;
const anon_sym_DQUOTE: Symbol = 183;
const anon_sym_DQUOTE_DQUOTE: Symbol = 232;
const anon_sym_EQ: Symbol = 83;
const anon_sym_EQ_EQ: Symbol = 34;
const anon_sym_GT: Symbol = 36;
const anon_sym_GT2: Symbol = 202;
const anon_sym_GT_EQ: Symbol = 37;
const anon_sym_GT_GT: Symbol = 41;
const anon_sym_GT_GT_EQ: Symbol = 138;
const anon_sym_LBRACE: Symbol = 74;
const anon_sym_LBRACK: Symbol = 80;
const anon_sym_LBRACK_COLON: Symbol = 228;
const anon_sym_LBRACK_LBRACK: Symbol = 59;
const anon_sym_LBRACK_RBRACK: Symbol = 231;
const anon_sym_LF: Symbol = 10;
const anon_sym_LPAREN: Symbol = 5;
const anon_sym_LPAREN2: Symbol = 20;
const anon_sym_LPAREN_RPAREN: Symbol = 230;
const anon_sym_LR_DQUOTE: Symbol = 219;
const anon_sym_LT: Symbol = 39;
const anon_sym_LT_EQ: Symbol = 38;
const anon_sym_LT_EQ_GT: Symbol = 147;
const anon_sym_LT_LT: Symbol = 40;
const anon_sym_LT_LT_EQ: Symbol = 137;
const anon_sym_L_DQUOTE: Symbol = 179;
const anon_sym_L_SQUOTE: Symbol = 173;
const anon_sym_NULL: Symbol = 189;
const anon_sym_PERCENT: Symbol = 28;
const anon_sym_PERCENT_EQ: Symbol = 134;
const anon_sym_PIPE: Symbol = 31;
const anon_sym_PIPE_EQ: Symbol = 141;
const anon_sym_PIPE_PIPE: Symbol = 29;
const anon_sym_PLUS: Symbol = 25;
const anon_sym_PLUS_EQ: Symbol = 135;
const anon_sym_PLUS_PLUS: Symbol = 155;
const anon_sym_QMARK: Symbol = 131;
const anon_sym_RBRACE: Symbol = 75;
const anon_sym_RBRACK: Symbol = 82;
const anon_sym_RBRACK_RBRACK: Symbol = 60;
const anon_sym_RPAREN: Symbol = 8;
const anon_sym_R_DQUOTE: Symbol = 218;
const anon_sym_SEMI: Symbol = 42;
const anon_sym_SLASH: Symbol = 27;
const anon_sym_SLASH_EQ: Symbol = 133;
const anon_sym_SQUOTE: Symbol = 177;
const anon_sym_STAR: Symbol = 26;
const anon_sym_STAR_EQ: Symbol = 132;
const anon_sym_TILDE: Symbol = 23;
const anon_sym_UR_DQUOTE: Symbol = 221;
const anon_sym_U_DQUOTE: Symbol = 181;
const anon_sym_U_SQUOTE: Symbol = 175;
const anon_sym__Alignas: Symbol = 109;
const anon_sym__Alignof: Symbol = 161;
const anon_sym__Atomic: Symbol = 96;
const anon_sym__Generic: Symbol = 163;
const anon_sym__Nonnull: Symbol = 99;
const anon_sym__Noreturn: Symbol = 97;
const anon_sym___alignof: Symbol = 158;
const anon_sym___alignof__: Symbol = 157;
const anon_sym___asm: Symbol = 167;
const anon_sym___asm__: Symbol = 166;
const anon_sym___attribute: Symbol = 55;
const anon_sym___attribute__: Symbol = 54;
const anon_sym___based: Symbol = 62;
const anon_sym___cdecl: Symbol = 63;
const anon_sym___clrcall: Symbol = 64;
const anon_sym___constant__: Symbol = 105;
const anon_sym___declspec: Symbol = 61;
const anon_sym___device__: Symbol = 46;
const anon_sym___except: Symbol = 128;
const anon_sym___extension__: Symbol = 43;
const anon_sym___fastcall: Symbol = 66;
const anon_sym___finally: Symbol = 129;
const anon_sym___forceinline: Symbol = 88;
const anon_sym___forceinline__: Symbol = 50;
const anon_sym___global__: Symbol = 48;
const anon_sym___grid_constant__: Symbol = 107;
const anon_sym___host__: Symbol = 45;
const anon_sym___inline: Symbol = 86;
const anon_sym___inline__: Symbol = 87;
const anon_sym___launch_bounds__: Symbol = 237;
const anon_sym___leave: Symbol = 130;
const anon_sym___local__: Symbol = 104;
const anon_sym___managed__: Symbol = 106;
const anon_sym___noinline__: Symbol = 51;
const anon_sym___restrict__: Symbol = 95;
const anon_sym___shared__: Symbol = 103;
const anon_sym___stdcall: Symbol = 65;
const anon_sym___thiscall: Symbol = 67;
const anon_sym___thread: Symbol = 90;
const anon_sym___tile__: Symbol = 47;
const anon_sym___tile_global__: Symbol = 49;
const anon_sym___try: Symbol = 127;
const anon_sym___unaligned: Symbol = 73;
const anon_sym___vectorcall: Symbol = 68;
const anon_sym___volatile__: Symbol = 168;
const anon_sym__alignof: Symbol = 159;
const anon_sym__unaligned: Symbol = 72;
const anon_sym_alignas: Symbol = 108;
const anon_sym_alignof: Symbol = 160;
const anon_sym_and: Symbol = 149;
const anon_sym_and_eq: Symbol = 142;
const anon_sym_asm: Symbol = 165;
const anon_sym_bitand: Symbol = 152;
const anon_sym_bitor: Symbol = 150;
const anon_sym_break: Symbol = 124;
const anon_sym_case: Symbol = 118;
const anon_sym_catch: Symbol = 217;
const anon_sym_class: Symbol = 112;
const anon_sym_co_await: Symbol = 223;
const anon_sym_co_return: Symbol = 215;
const anon_sym_co_yield: Symbol = 216;
const anon_sym_compl: Symbol = 146;
const anon_sym_concept: Symbol = 214;
const anon_sym_const: Symbol = 91;
const anon_sym_consteval: Symbol = 102;
const anon_sym_constexpr: Symbol = 92;
const anon_sym_constinit: Symbol = 101;
const anon_sym_continue: Symbol = 125;
const anon_sym_decltype: Symbol = 193;
const anon_sym_default: Symbol = 119;
const anon_sym_defined: Symbol = 21;
const anon_sym_delete: Symbol = 205;
const anon_sym_do: Symbol = 121;
const anon_sym_else: Symbol = 116;
const anon_sym_enum: Symbol = 111;
const anon_sym_explicit: Symbol = 196;
const anon_sym_export: Symbol = 197;
const anon_sym_extern: Symbol = 53;
const anon_sym_final: Symbol = 194;
const anon_sym_for: Symbol = 122;
const anon_sym_friend: Symbol = 207;
const anon_sym_goto: Symbol = 126;
const anon_sym_if: Symbol = 115;
const anon_sym_import: Symbol = 199;
const anon_sym_inline: Symbol = 85;
const anon_sym_long: Symbol = 78;
const anon_sym_module: Symbol = 198;
const anon_sym_mutable: Symbol = 100;
const anon_sym_namespace: Symbol = 212;
const anon_sym_new: Symbol = 224;
const anon_sym_noexcept: Symbol = 210;
const anon_sym_noreturn: Symbol = 98;
const anon_sym_not: Symbol = 145;
const anon_sym_not_eq: Symbol = 153;
const anon_sym_nullptr: Symbol = 190;
const anon_sym_offsetof: Symbol = 162;
const anon_sym_operator: Symbol = 203;
const anon_sym_or: Symbol = 148;
const anon_sym_or_eq: Symbol = 143;
const anon_sym_override: Symbol = 195;
const anon_sym_private: Symbol = 200;
const anon_sym_protected: Symbol = 209;
const anon_sym_public: Symbol = 208;
const anon_sym_register: Symbol = 84;
const anon_sym_requires: Symbol = 225;
const anon_sym_restrict: Symbol = 94;
const anon_sym_return: Symbol = 123;
const anon_sym_short: Symbol = 79;
const anon_sym_signed: Symbol = 76;
const anon_sym_sizeof: Symbol = 156;
const anon_sym_static: Symbol = 81;
const anon_sym_static_assert: Symbol = 213;
const anon_sym_struct: Symbol = 113;
const anon_sym_switch: Symbol = 117;
const anon_sym_template: Symbol = 201;
const anon_sym_thread_local: Symbol = 89;
const anon_sym_throw: Symbol = 211;
const anon_sym_try: Symbol = 204;
const anon_sym_typedef: Symbol = 44;
const anon_sym_typename: Symbol = 164;
const anon_sym_u8R_DQUOTE: Symbol = 222;
const anon_sym_u8_DQUOTE: Symbol = 182;
const anon_sym_u8_SQUOTE: Symbol = 176;
const anon_sym_uR_DQUOTE: Symbol = 220;
const anon_sym_u_DQUOTE: Symbol = 180;
const anon_sym_u_SQUOTE: Symbol = 174;
const anon_sym_union: Symbol = 114;
const anon_sym_unsigned: Symbol = 77;
const anon_sym_using: Symbol = 56;
const anon_sym_virtual: Symbol = 52;
const anon_sym_volatile: Symbol = 93;
const anon_sym_while: Symbol = 120;
const anon_sym_xor: Symbol = 151;
const anon_sym_xor_eq: Symbol = 144;
const aux_sym_char_literal_token1: Symbol = 178;
const aux_sym_kernel_call_syntax_token1: Symbol = 235;
const aux_sym_kernel_call_syntax_token2: Symbol = 236;
const aux_sym_preproc_def_token1: Symbol = 4;
const aux_sym_preproc_elif_token1: Symbol = 15;
const aux_sym_preproc_elifdef_token1: Symbol = 16;
const aux_sym_preproc_elifdef_token2: Symbol = 17;
const aux_sym_preproc_else_token1: Symbol = 14;
const aux_sym_preproc_if_token1: Symbol = 9;
const aux_sym_preproc_if_token2: Symbol = 11;
const aux_sym_preproc_ifdef_token1: Symbol = 12;
const aux_sym_preproc_ifdef_token2: Symbol = 13;
const aux_sym_preproc_include_token1: Symbol = 2;
const aux_sym_preproc_include_token2: Symbol = 3;
const aux_sym_pure_virtual_clause_token1: Symbol = 206;
const aux_sym_string_literal_token1: Symbol = 184;
const sym_auto: Symbol = 192;
const sym_comment: Symbol = 191;
const sym_escape_sequence: Symbol = 185;
const sym_false: Symbol = 188;
const sym_identifier: Symbol = 1;
const sym_literal_suffix: Symbol = 234;
const sym_ms_restrict_modifier: Symbol = 69;
const sym_ms_signed_ptr_modifier: Symbol = 71;
const sym_ms_unsigned_ptr_modifier: Symbol = 70;
const sym_number_literal: Symbol = 172;
const sym_preproc_arg: Symbol = 18;
const sym_preproc_directive: Symbol = 19;
const sym_primitive_type: Symbol = 110;
const sym_system_lib_string: Symbol = 186;
const sym_this: Symbol = 233;
const sym_true: Symbol = 187;
const ts_builtin_sym_end: Symbol = 0;

#[rustfmt::skip]
static sym_identifier_character_set_1: [CharacterRange; 687] = [
    CharacterRange::new(36, 36), CharacterRange::new(65, 90), CharacterRange::new(92, 92), CharacterRange::new(95, 95), CharacterRange::new(97, 122), CharacterRange::new(170, 170),
    CharacterRange::new(181, 181), CharacterRange::new(186, 186), CharacterRange::new(192, 214), CharacterRange::new(216, 246), CharacterRange::new(248, 705), CharacterRange::new(710, 721),
    CharacterRange::new(736, 740), CharacterRange::new(748, 748), CharacterRange::new(750, 750), CharacterRange::new(880, 884), CharacterRange::new(886, 887), CharacterRange::new(891, 893),
    CharacterRange::new(895, 895), CharacterRange::new(902, 902), CharacterRange::new(904, 906), CharacterRange::new(908, 908), CharacterRange::new(910, 929), CharacterRange::new(931, 1013),
    CharacterRange::new(1015, 1153), CharacterRange::new(1162, 1327), CharacterRange::new(1329, 1366), CharacterRange::new(1369, 1369), CharacterRange::new(1376, 1416), CharacterRange::new(1488, 1514),
    CharacterRange::new(1519, 1522), CharacterRange::new(1568, 1610), CharacterRange::new(1646, 1647), CharacterRange::new(1649, 1747), CharacterRange::new(1749, 1749), CharacterRange::new(1765, 1766),
    CharacterRange::new(1774, 1775), CharacterRange::new(1786, 1788), CharacterRange::new(1791, 1791), CharacterRange::new(1808, 1808), CharacterRange::new(1810, 1839), CharacterRange::new(1869, 1957),
    CharacterRange::new(1969, 1969), CharacterRange::new(1994, 2026), CharacterRange::new(2036, 2037), CharacterRange::new(2042, 2042), CharacterRange::new(2048, 2069), CharacterRange::new(2074, 2074),
    CharacterRange::new(2084, 2084), CharacterRange::new(2088, 2088), CharacterRange::new(2112, 2136), CharacterRange::new(2144, 2154), CharacterRange::new(2160, 2183), CharacterRange::new(2185, 2190),
    CharacterRange::new(2208, 2249), CharacterRange::new(2308, 2361), CharacterRange::new(2365, 2365), CharacterRange::new(2384, 2384), CharacterRange::new(2392, 2401), CharacterRange::new(2417, 2432),
    CharacterRange::new(2437, 2444), CharacterRange::new(2447, 2448), CharacterRange::new(2451, 2472), CharacterRange::new(2474, 2480), CharacterRange::new(2482, 2482), CharacterRange::new(2486, 2489),
    CharacterRange::new(2493, 2493), CharacterRange::new(2510, 2510), CharacterRange::new(2524, 2525), CharacterRange::new(2527, 2529), CharacterRange::new(2544, 2545), CharacterRange::new(2556, 2556),
    CharacterRange::new(2565, 2570), CharacterRange::new(2575, 2576), CharacterRange::new(2579, 2600), CharacterRange::new(2602, 2608), CharacterRange::new(2610, 2611), CharacterRange::new(2613, 2614),
    CharacterRange::new(2616, 2617), CharacterRange::new(2649, 2652), CharacterRange::new(2654, 2654), CharacterRange::new(2674, 2676), CharacterRange::new(2693, 2701), CharacterRange::new(2703, 2705),
    CharacterRange::new(2707, 2728), CharacterRange::new(2730, 2736), CharacterRange::new(2738, 2739), CharacterRange::new(2741, 2745), CharacterRange::new(2749, 2749), CharacterRange::new(2768, 2768),
    CharacterRange::new(2784, 2785), CharacterRange::new(2809, 2809), CharacterRange::new(2821, 2828), CharacterRange::new(2831, 2832), CharacterRange::new(2835, 2856), CharacterRange::new(2858, 2864),
    CharacterRange::new(2866, 2867), CharacterRange::new(2869, 2873), CharacterRange::new(2877, 2877), CharacterRange::new(2908, 2909), CharacterRange::new(2911, 2913), CharacterRange::new(2929, 2929),
    CharacterRange::new(2947, 2947), CharacterRange::new(2949, 2954), CharacterRange::new(2958, 2960), CharacterRange::new(2962, 2965), CharacterRange::new(2969, 2970), CharacterRange::new(2972, 2972),
    CharacterRange::new(2974, 2975), CharacterRange::new(2979, 2980), CharacterRange::new(2984, 2986), CharacterRange::new(2990, 3001), CharacterRange::new(3024, 3024), CharacterRange::new(3077, 3084),
    CharacterRange::new(3086, 3088), CharacterRange::new(3090, 3112), CharacterRange::new(3114, 3129), CharacterRange::new(3133, 3133), CharacterRange::new(3160, 3162), CharacterRange::new(3165, 3165),
    CharacterRange::new(3168, 3169), CharacterRange::new(3200, 3200), CharacterRange::new(3205, 3212), CharacterRange::new(3214, 3216), CharacterRange::new(3218, 3240), CharacterRange::new(3242, 3251),
    CharacterRange::new(3253, 3257), CharacterRange::new(3261, 3261), CharacterRange::new(3293, 3294), CharacterRange::new(3296, 3297), CharacterRange::new(3313, 3314), CharacterRange::new(3332, 3340),
    CharacterRange::new(3342, 3344), CharacterRange::new(3346, 3386), CharacterRange::new(3389, 3389), CharacterRange::new(3406, 3406), CharacterRange::new(3412, 3414), CharacterRange::new(3423, 3425),
    CharacterRange::new(3450, 3455), CharacterRange::new(3461, 3478), CharacterRange::new(3482, 3505), CharacterRange::new(3507, 3515), CharacterRange::new(3517, 3517), CharacterRange::new(3520, 3526),
    CharacterRange::new(3585, 3632), CharacterRange::new(3634, 3634), CharacterRange::new(3648, 3654), CharacterRange::new(3713, 3714), CharacterRange::new(3716, 3716), CharacterRange::new(3718, 3722),
    CharacterRange::new(3724, 3747), CharacterRange::new(3749, 3749), CharacterRange::new(3751, 3760), CharacterRange::new(3762, 3762), CharacterRange::new(3773, 3773), CharacterRange::new(3776, 3780),
    CharacterRange::new(3782, 3782), CharacterRange::new(3804, 3807), CharacterRange::new(3840, 3840), CharacterRange::new(3904, 3911), CharacterRange::new(3913, 3948), CharacterRange::new(3976, 3980),
    CharacterRange::new(4096, 4138), CharacterRange::new(4159, 4159), CharacterRange::new(4176, 4181), CharacterRange::new(4186, 4189), CharacterRange::new(4193, 4193), CharacterRange::new(4197, 4198),
    CharacterRange::new(4206, 4208), CharacterRange::new(4213, 4225), CharacterRange::new(4238, 4238), CharacterRange::new(4256, 4293), CharacterRange::new(4295, 4295), CharacterRange::new(4301, 4301),
    CharacterRange::new(4304, 4346), CharacterRange::new(4348, 4680), CharacterRange::new(4682, 4685), CharacterRange::new(4688, 4694), CharacterRange::new(4696, 4696), CharacterRange::new(4698, 4701),
    CharacterRange::new(4704, 4744), CharacterRange::new(4746, 4749), CharacterRange::new(4752, 4784), CharacterRange::new(4786, 4789), CharacterRange::new(4792, 4798), CharacterRange::new(4800, 4800),
    CharacterRange::new(4802, 4805), CharacterRange::new(4808, 4822), CharacterRange::new(4824, 4880), CharacterRange::new(4882, 4885), CharacterRange::new(4888, 4954), CharacterRange::new(4992, 5007),
    CharacterRange::new(5024, 5109), CharacterRange::new(5112, 5117), CharacterRange::new(5121, 5740), CharacterRange::new(5743, 5759), CharacterRange::new(5761, 5786), CharacterRange::new(5792, 5866),
    CharacterRange::new(5870, 5880), CharacterRange::new(5888, 5905), CharacterRange::new(5919, 5937), CharacterRange::new(5952, 5969), CharacterRange::new(5984, 5996), CharacterRange::new(5998, 6000),
    CharacterRange::new(6016, 6067), CharacterRange::new(6103, 6103), CharacterRange::new(6108, 6108), CharacterRange::new(6176, 6264), CharacterRange::new(6272, 6312), CharacterRange::new(6314, 6314),
    CharacterRange::new(6320, 6389), CharacterRange::new(6400, 6430), CharacterRange::new(6480, 6509), CharacterRange::new(6512, 6516), CharacterRange::new(6528, 6571), CharacterRange::new(6576, 6601),
    CharacterRange::new(6656, 6678), CharacterRange::new(6688, 6740), CharacterRange::new(6823, 6823), CharacterRange::new(6917, 6963), CharacterRange::new(6981, 6988), CharacterRange::new(7043, 7072),
    CharacterRange::new(7086, 7087), CharacterRange::new(7098, 7141), CharacterRange::new(7168, 7203), CharacterRange::new(7245, 7247), CharacterRange::new(7258, 7293), CharacterRange::new(7296, 7306),
    CharacterRange::new(7312, 7354), CharacterRange::new(7357, 7359), CharacterRange::new(7401, 7404), CharacterRange::new(7406, 7411), CharacterRange::new(7413, 7414), CharacterRange::new(7418, 7418),
    CharacterRange::new(7424, 7615), CharacterRange::new(7680, 7957), CharacterRange::new(7960, 7965), CharacterRange::new(7968, 8005), CharacterRange::new(8008, 8013), CharacterRange::new(8016, 8023),
    CharacterRange::new(8025, 8025), CharacterRange::new(8027, 8027), CharacterRange::new(8029, 8029), CharacterRange::new(8031, 8061), CharacterRange::new(8064, 8116), CharacterRange::new(8118, 8124),
    CharacterRange::new(8126, 8126), CharacterRange::new(8130, 8132), CharacterRange::new(8134, 8140), CharacterRange::new(8144, 8147), CharacterRange::new(8150, 8155), CharacterRange::new(8160, 8172),
    CharacterRange::new(8178, 8180), CharacterRange::new(8182, 8188), CharacterRange::new(8305, 8305), CharacterRange::new(8319, 8319), CharacterRange::new(8336, 8348), CharacterRange::new(8450, 8450),
    CharacterRange::new(8455, 8455), CharacterRange::new(8458, 8467), CharacterRange::new(8469, 8469), CharacterRange::new(8472, 8477), CharacterRange::new(8484, 8484), CharacterRange::new(8486, 8486),
    CharacterRange::new(8488, 8488), CharacterRange::new(8490, 8505), CharacterRange::new(8508, 8511), CharacterRange::new(8517, 8521), CharacterRange::new(8526, 8526), CharacterRange::new(8544, 8584),
    CharacterRange::new(11264, 11492), CharacterRange::new(11499, 11502), CharacterRange::new(11506, 11507), CharacterRange::new(11520, 11557), CharacterRange::new(11559, 11559), CharacterRange::new(11565, 11565),
    CharacterRange::new(11568, 11623), CharacterRange::new(11631, 11631), CharacterRange::new(11648, 11670), CharacterRange::new(11680, 11686), CharacterRange::new(11688, 11694), CharacterRange::new(11696, 11702),
    CharacterRange::new(11704, 11710), CharacterRange::new(11712, 11718), CharacterRange::new(11720, 11726), CharacterRange::new(11728, 11734), CharacterRange::new(11736, 11742), CharacterRange::new(12293, 12295),
    CharacterRange::new(12321, 12329), CharacterRange::new(12337, 12341), CharacterRange::new(12344, 12348), CharacterRange::new(12353, 12438), CharacterRange::new(12445, 12447), CharacterRange::new(12449, 12538),
    CharacterRange::new(12540, 12543), CharacterRange::new(12549, 12591), CharacterRange::new(12593, 12686), CharacterRange::new(12704, 12735), CharacterRange::new(12784, 12799), CharacterRange::new(13312, 19903),
    CharacterRange::new(19968, 42124), CharacterRange::new(42192, 42237), CharacterRange::new(42240, 42508), CharacterRange::new(42512, 42527), CharacterRange::new(42538, 42539), CharacterRange::new(42560, 42606),
    CharacterRange::new(42623, 42653), CharacterRange::new(42656, 42735), CharacterRange::new(42775, 42783), CharacterRange::new(42786, 42888), CharacterRange::new(42891, 42957), CharacterRange::new(42960, 42961),
    CharacterRange::new(42963, 42963), CharacterRange::new(42965, 42972), CharacterRange::new(42994, 43009), CharacterRange::new(43011, 43013), CharacterRange::new(43015, 43018), CharacterRange::new(43020, 43042),
    CharacterRange::new(43072, 43123), CharacterRange::new(43138, 43187), CharacterRange::new(43250, 43255), CharacterRange::new(43259, 43259), CharacterRange::new(43261, 43262), CharacterRange::new(43274, 43301),
    CharacterRange::new(43312, 43334), CharacterRange::new(43360, 43388), CharacterRange::new(43396, 43442), CharacterRange::new(43471, 43471), CharacterRange::new(43488, 43492), CharacterRange::new(43494, 43503),
    CharacterRange::new(43514, 43518), CharacterRange::new(43520, 43560), CharacterRange::new(43584, 43586), CharacterRange::new(43588, 43595), CharacterRange::new(43616, 43638), CharacterRange::new(43642, 43642),
    CharacterRange::new(43646, 43695), CharacterRange::new(43697, 43697), CharacterRange::new(43701, 43702), CharacterRange::new(43705, 43709), CharacterRange::new(43712, 43712), CharacterRange::new(43714, 43714),
    CharacterRange::new(43739, 43741), CharacterRange::new(43744, 43754), CharacterRange::new(43762, 43764), CharacterRange::new(43777, 43782), CharacterRange::new(43785, 43790), CharacterRange::new(43793, 43798),
    CharacterRange::new(43808, 43814), CharacterRange::new(43816, 43822), CharacterRange::new(43824, 43866), CharacterRange::new(43868, 43881), CharacterRange::new(43888, 44002), CharacterRange::new(44032, 55203),
    CharacterRange::new(55216, 55238), CharacterRange::new(55243, 55291), CharacterRange::new(63744, 64109), CharacterRange::new(64112, 64217), CharacterRange::new(64256, 64262), CharacterRange::new(64275, 64279),
    CharacterRange::new(64285, 64285), CharacterRange::new(64287, 64296), CharacterRange::new(64298, 64310), CharacterRange::new(64312, 64316), CharacterRange::new(64318, 64318), CharacterRange::new(64320, 64321),
    CharacterRange::new(64323, 64324), CharacterRange::new(64326, 64433), CharacterRange::new(64467, 64605), CharacterRange::new(64612, 64829), CharacterRange::new(64848, 64911), CharacterRange::new(64914, 64967),
    CharacterRange::new(65008, 65017), CharacterRange::new(65137, 65137), CharacterRange::new(65139, 65139), CharacterRange::new(65143, 65143), CharacterRange::new(65145, 65145), CharacterRange::new(65147, 65147),
    CharacterRange::new(65149, 65149), CharacterRange::new(65151, 65276), CharacterRange::new(65313, 65338), CharacterRange::new(65345, 65370), CharacterRange::new(65382, 65437), CharacterRange::new(65440, 65470),
    CharacterRange::new(65474, 65479), CharacterRange::new(65482, 65487), CharacterRange::new(65490, 65495), CharacterRange::new(65498, 65500), CharacterRange::new(65536, 65547), CharacterRange::new(65549, 65574),
    CharacterRange::new(65576, 65594), CharacterRange::new(65596, 65597), CharacterRange::new(65599, 65613), CharacterRange::new(65616, 65629), CharacterRange::new(65664, 65786), CharacterRange::new(65856, 65908),
    CharacterRange::new(66176, 66204), CharacterRange::new(66208, 66256), CharacterRange::new(66304, 66335), CharacterRange::new(66349, 66378), CharacterRange::new(66384, 66421), CharacterRange::new(66432, 66461),
    CharacterRange::new(66464, 66499), CharacterRange::new(66504, 66511), CharacterRange::new(66513, 66517), CharacterRange::new(66560, 66717), CharacterRange::new(66736, 66771), CharacterRange::new(66776, 66811),
    CharacterRange::new(66816, 66855), CharacterRange::new(66864, 66915), CharacterRange::new(66928, 66938), CharacterRange::new(66940, 66954), CharacterRange::new(66956, 66962), CharacterRange::new(66964, 66965),
    CharacterRange::new(66967, 66977), CharacterRange::new(66979, 66993), CharacterRange::new(66995, 67001), CharacterRange::new(67003, 67004), CharacterRange::new(67008, 67059), CharacterRange::new(67072, 67382),
    CharacterRange::new(67392, 67413), CharacterRange::new(67424, 67431), CharacterRange::new(67456, 67461), CharacterRange::new(67463, 67504), CharacterRange::new(67506, 67514), CharacterRange::new(67584, 67589),
    CharacterRange::new(67592, 67592), CharacterRange::new(67594, 67637), CharacterRange::new(67639, 67640), CharacterRange::new(67644, 67644), CharacterRange::new(67647, 67669), CharacterRange::new(67680, 67702),
    CharacterRange::new(67712, 67742), CharacterRange::new(67808, 67826), CharacterRange::new(67828, 67829), CharacterRange::new(67840, 67861), CharacterRange::new(67872, 67897), CharacterRange::new(67968, 68023),
    CharacterRange::new(68030, 68031), CharacterRange::new(68096, 68096), CharacterRange::new(68112, 68115), CharacterRange::new(68117, 68119), CharacterRange::new(68121, 68149), CharacterRange::new(68192, 68220),
    CharacterRange::new(68224, 68252), CharacterRange::new(68288, 68295), CharacterRange::new(68297, 68324), CharacterRange::new(68352, 68405), CharacterRange::new(68416, 68437), CharacterRange::new(68448, 68466),
    CharacterRange::new(68480, 68497), CharacterRange::new(68608, 68680), CharacterRange::new(68736, 68786), CharacterRange::new(68800, 68850), CharacterRange::new(68864, 68899), CharacterRange::new(68938, 68965),
    CharacterRange::new(68975, 68997), CharacterRange::new(69248, 69289), CharacterRange::new(69296, 69297), CharacterRange::new(69314, 69316), CharacterRange::new(69376, 69404), CharacterRange::new(69415, 69415),
    CharacterRange::new(69424, 69445), CharacterRange::new(69488, 69505), CharacterRange::new(69552, 69572), CharacterRange::new(69600, 69622), CharacterRange::new(69635, 69687), CharacterRange::new(69745, 69746),
    CharacterRange::new(69749, 69749), CharacterRange::new(69763, 69807), CharacterRange::new(69840, 69864), CharacterRange::new(69891, 69926), CharacterRange::new(69956, 69956), CharacterRange::new(69959, 69959),
    CharacterRange::new(69968, 70002), CharacterRange::new(70006, 70006), CharacterRange::new(70019, 70066), CharacterRange::new(70081, 70084), CharacterRange::new(70106, 70106), CharacterRange::new(70108, 70108),
    CharacterRange::new(70144, 70161), CharacterRange::new(70163, 70187), CharacterRange::new(70207, 70208), CharacterRange::new(70272, 70278), CharacterRange::new(70280, 70280), CharacterRange::new(70282, 70285),
    CharacterRange::new(70287, 70301), CharacterRange::new(70303, 70312), CharacterRange::new(70320, 70366), CharacterRange::new(70405, 70412), CharacterRange::new(70415, 70416), CharacterRange::new(70419, 70440),
    CharacterRange::new(70442, 70448), CharacterRange::new(70450, 70451), CharacterRange::new(70453, 70457), CharacterRange::new(70461, 70461), CharacterRange::new(70480, 70480), CharacterRange::new(70493, 70497),
    CharacterRange::new(70528, 70537), CharacterRange::new(70539, 70539), CharacterRange::new(70542, 70542), CharacterRange::new(70544, 70581), CharacterRange::new(70583, 70583), CharacterRange::new(70609, 70609),
    CharacterRange::new(70611, 70611), CharacterRange::new(70656, 70708), CharacterRange::new(70727, 70730), CharacterRange::new(70751, 70753), CharacterRange::new(70784, 70831), CharacterRange::new(70852, 70853),
    CharacterRange::new(70855, 70855), CharacterRange::new(71040, 71086), CharacterRange::new(71128, 71131), CharacterRange::new(71168, 71215), CharacterRange::new(71236, 71236), CharacterRange::new(71296, 71338),
    CharacterRange::new(71352, 71352), CharacterRange::new(71424, 71450), CharacterRange::new(71488, 71494), CharacterRange::new(71680, 71723), CharacterRange::new(71840, 71903), CharacterRange::new(71935, 71942),
    CharacterRange::new(71945, 71945), CharacterRange::new(71948, 71955), CharacterRange::new(71957, 71958), CharacterRange::new(71960, 71983), CharacterRange::new(71999, 71999), CharacterRange::new(72001, 72001),
    CharacterRange::new(72096, 72103), CharacterRange::new(72106, 72144), CharacterRange::new(72161, 72161), CharacterRange::new(72163, 72163), CharacterRange::new(72192, 72192), CharacterRange::new(72203, 72242),
    CharacterRange::new(72250, 72250), CharacterRange::new(72272, 72272), CharacterRange::new(72284, 72329), CharacterRange::new(72349, 72349), CharacterRange::new(72368, 72440), CharacterRange::new(72640, 72672),
    CharacterRange::new(72704, 72712), CharacterRange::new(72714, 72750), CharacterRange::new(72768, 72768), CharacterRange::new(72818, 72847), CharacterRange::new(72960, 72966), CharacterRange::new(72968, 72969),
    CharacterRange::new(72971, 73008), CharacterRange::new(73030, 73030), CharacterRange::new(73056, 73061), CharacterRange::new(73063, 73064), CharacterRange::new(73066, 73097), CharacterRange::new(73112, 73112),
    CharacterRange::new(73440, 73458), CharacterRange::new(73474, 73474), CharacterRange::new(73476, 73488), CharacterRange::new(73490, 73523), CharacterRange::new(73648, 73648), CharacterRange::new(73728, 74649),
    CharacterRange::new(74752, 74862), CharacterRange::new(74880, 75075), CharacterRange::new(77712, 77808), CharacterRange::new(77824, 78895), CharacterRange::new(78913, 78918), CharacterRange::new(78944, 82938),
    CharacterRange::new(82944, 83526), CharacterRange::new(90368, 90397), CharacterRange::new(92160, 92728), CharacterRange::new(92736, 92766), CharacterRange::new(92784, 92862), CharacterRange::new(92880, 92909),
    CharacterRange::new(92928, 92975), CharacterRange::new(92992, 92995), CharacterRange::new(93027, 93047), CharacterRange::new(93053, 93071), CharacterRange::new(93504, 93548), CharacterRange::new(93760, 93823),
    CharacterRange::new(93952, 94026), CharacterRange::new(94032, 94032), CharacterRange::new(94099, 94111), CharacterRange::new(94176, 94177), CharacterRange::new(94179, 94179), CharacterRange::new(94208, 100343),
    CharacterRange::new(100352, 101589), CharacterRange::new(101631, 101640), CharacterRange::new(110576, 110579), CharacterRange::new(110581, 110587), CharacterRange::new(110589, 110590), CharacterRange::new(110592, 110882),
    CharacterRange::new(110898, 110898), CharacterRange::new(110928, 110930), CharacterRange::new(110933, 110933), CharacterRange::new(110948, 110951), CharacterRange::new(110960, 111355), CharacterRange::new(113664, 113770),
    CharacterRange::new(113776, 113788), CharacterRange::new(113792, 113800), CharacterRange::new(113808, 113817), CharacterRange::new(119808, 119892), CharacterRange::new(119894, 119964), CharacterRange::new(119966, 119967),
    CharacterRange::new(119970, 119970), CharacterRange::new(119973, 119974), CharacterRange::new(119977, 119980), CharacterRange::new(119982, 119993), CharacterRange::new(119995, 119995), CharacterRange::new(119997, 120003),
    CharacterRange::new(120005, 120069), CharacterRange::new(120071, 120074), CharacterRange::new(120077, 120084), CharacterRange::new(120086, 120092), CharacterRange::new(120094, 120121), CharacterRange::new(120123, 120126),
    CharacterRange::new(120128, 120132), CharacterRange::new(120134, 120134), CharacterRange::new(120138, 120144), CharacterRange::new(120146, 120485), CharacterRange::new(120488, 120512), CharacterRange::new(120514, 120538),
    CharacterRange::new(120540, 120570), CharacterRange::new(120572, 120596), CharacterRange::new(120598, 120628), CharacterRange::new(120630, 120654), CharacterRange::new(120656, 120686), CharacterRange::new(120688, 120712),
    CharacterRange::new(120714, 120744), CharacterRange::new(120746, 120770), CharacterRange::new(120772, 120779), CharacterRange::new(122624, 122654), CharacterRange::new(122661, 122666), CharacterRange::new(122928, 122989),
    CharacterRange::new(123136, 123180), CharacterRange::new(123191, 123197), CharacterRange::new(123214, 123214), CharacterRange::new(123536, 123565), CharacterRange::new(123584, 123627), CharacterRange::new(124112, 124139),
    CharacterRange::new(124368, 124397), CharacterRange::new(124400, 124400), CharacterRange::new(124896, 124902), CharacterRange::new(124904, 124907), CharacterRange::new(124909, 124910), CharacterRange::new(124912, 124926),
    CharacterRange::new(124928, 125124), CharacterRange::new(125184, 125251), CharacterRange::new(125259, 125259), CharacterRange::new(126464, 126467), CharacterRange::new(126469, 126495), CharacterRange::new(126497, 126498),
    CharacterRange::new(126500, 126500), CharacterRange::new(126503, 126503), CharacterRange::new(126505, 126514), CharacterRange::new(126516, 126519), CharacterRange::new(126521, 126521), CharacterRange::new(126523, 126523),
    CharacterRange::new(126530, 126530), CharacterRange::new(126535, 126535), CharacterRange::new(126537, 126537), CharacterRange::new(126539, 126539), CharacterRange::new(126541, 126543), CharacterRange::new(126545, 126546),
    CharacterRange::new(126548, 126548), CharacterRange::new(126551, 126551), CharacterRange::new(126553, 126553), CharacterRange::new(126555, 126555), CharacterRange::new(126557, 126557), CharacterRange::new(126559, 126559),
    CharacterRange::new(126561, 126562), CharacterRange::new(126564, 126564), CharacterRange::new(126567, 126570), CharacterRange::new(126572, 126578), CharacterRange::new(126580, 126583), CharacterRange::new(126585, 126588),
    CharacterRange::new(126590, 126590), CharacterRange::new(126592, 126601), CharacterRange::new(126603, 126619), CharacterRange::new(126625, 126627), CharacterRange::new(126629, 126633), CharacterRange::new(126635, 126651),
    CharacterRange::new(131072, 173791), CharacterRange::new(173824, 177977), CharacterRange::new(177984, 178205), CharacterRange::new(178208, 183969), CharacterRange::new(183984, 191456), CharacterRange::new(191472, 192093),
    CharacterRange::new(194560, 195101), CharacterRange::new(196608, 201546), CharacterRange::new(201552, 205743),
];

#[rustfmt::skip]
static sym_identifier_character_set_2: [CharacterRange; 802] = [
    CharacterRange::new(36, 36), CharacterRange::new(48, 57), CharacterRange::new(65, 90), CharacterRange::new(92, 92), CharacterRange::new(95, 95), CharacterRange::new(97, 122),
    CharacterRange::new(170, 170), CharacterRange::new(181, 181), CharacterRange::new(183, 183), CharacterRange::new(186, 186), CharacterRange::new(192, 214), CharacterRange::new(216, 246),
    CharacterRange::new(248, 705), CharacterRange::new(710, 721), CharacterRange::new(736, 740), CharacterRange::new(748, 748), CharacterRange::new(750, 750), CharacterRange::new(768, 884),
    CharacterRange::new(886, 887), CharacterRange::new(891, 893), CharacterRange::new(895, 895), CharacterRange::new(902, 906), CharacterRange::new(908, 908), CharacterRange::new(910, 929),
    CharacterRange::new(931, 1013), CharacterRange::new(1015, 1153), CharacterRange::new(1155, 1159), CharacterRange::new(1162, 1327), CharacterRange::new(1329, 1366), CharacterRange::new(1369, 1369),
    CharacterRange::new(1376, 1416), CharacterRange::new(1425, 1469), CharacterRange::new(1471, 1471), CharacterRange::new(1473, 1474), CharacterRange::new(1476, 1477), CharacterRange::new(1479, 1479),
    CharacterRange::new(1488, 1514), CharacterRange::new(1519, 1522), CharacterRange::new(1552, 1562), CharacterRange::new(1568, 1641), CharacterRange::new(1646, 1747), CharacterRange::new(1749, 1756),
    CharacterRange::new(1759, 1768), CharacterRange::new(1770, 1788), CharacterRange::new(1791, 1791), CharacterRange::new(1808, 1866), CharacterRange::new(1869, 1969), CharacterRange::new(1984, 2037),
    CharacterRange::new(2042, 2042), CharacterRange::new(2045, 2045), CharacterRange::new(2048, 2093), CharacterRange::new(2112, 2139), CharacterRange::new(2144, 2154), CharacterRange::new(2160, 2183),
    CharacterRange::new(2185, 2190), CharacterRange::new(2199, 2273), CharacterRange::new(2275, 2403), CharacterRange::new(2406, 2415), CharacterRange::new(2417, 2435), CharacterRange::new(2437, 2444),
    CharacterRange::new(2447, 2448), CharacterRange::new(2451, 2472), CharacterRange::new(2474, 2480), CharacterRange::new(2482, 2482), CharacterRange::new(2486, 2489), CharacterRange::new(2492, 2500),
    CharacterRange::new(2503, 2504), CharacterRange::new(2507, 2510), CharacterRange::new(2519, 2519), CharacterRange::new(2524, 2525), CharacterRange::new(2527, 2531), CharacterRange::new(2534, 2545),
    CharacterRange::new(2556, 2556), CharacterRange::new(2558, 2558), CharacterRange::new(2561, 2563), CharacterRange::new(2565, 2570), CharacterRange::new(2575, 2576), CharacterRange::new(2579, 2600),
    CharacterRange::new(2602, 2608), CharacterRange::new(2610, 2611), CharacterRange::new(2613, 2614), CharacterRange::new(2616, 2617), CharacterRange::new(2620, 2620), CharacterRange::new(2622, 2626),
    CharacterRange::new(2631, 2632), CharacterRange::new(2635, 2637), CharacterRange::new(2641, 2641), CharacterRange::new(2649, 2652), CharacterRange::new(2654, 2654), CharacterRange::new(2662, 2677),
    CharacterRange::new(2689, 2691), CharacterRange::new(2693, 2701), CharacterRange::new(2703, 2705), CharacterRange::new(2707, 2728), CharacterRange::new(2730, 2736), CharacterRange::new(2738, 2739),
    CharacterRange::new(2741, 2745), CharacterRange::new(2748, 2757), CharacterRange::new(2759, 2761), CharacterRange::new(2763, 2765), CharacterRange::new(2768, 2768), CharacterRange::new(2784, 2787),
    CharacterRange::new(2790, 2799), CharacterRange::new(2809, 2815), CharacterRange::new(2817, 2819), CharacterRange::new(2821, 2828), CharacterRange::new(2831, 2832), CharacterRange::new(2835, 2856),
    CharacterRange::new(2858, 2864), CharacterRange::new(2866, 2867), CharacterRange::new(2869, 2873), CharacterRange::new(2876, 2884), CharacterRange::new(2887, 2888), CharacterRange::new(2891, 2893),
    CharacterRange::new(2901, 2903), CharacterRange::new(2908, 2909), CharacterRange::new(2911, 2915), CharacterRange::new(2918, 2927), CharacterRange::new(2929, 2929), CharacterRange::new(2946, 2947),
    CharacterRange::new(2949, 2954), CharacterRange::new(2958, 2960), CharacterRange::new(2962, 2965), CharacterRange::new(2969, 2970), CharacterRange::new(2972, 2972), CharacterRange::new(2974, 2975),
    CharacterRange::new(2979, 2980), CharacterRange::new(2984, 2986), CharacterRange::new(2990, 3001), CharacterRange::new(3006, 3010), CharacterRange::new(3014, 3016), CharacterRange::new(3018, 3021),
    CharacterRange::new(3024, 3024), CharacterRange::new(3031, 3031), CharacterRange::new(3046, 3055), CharacterRange::new(3072, 3084), CharacterRange::new(3086, 3088), CharacterRange::new(3090, 3112),
    CharacterRange::new(3114, 3129), CharacterRange::new(3132, 3140), CharacterRange::new(3142, 3144), CharacterRange::new(3146, 3149), CharacterRange::new(3157, 3158), CharacterRange::new(3160, 3162),
    CharacterRange::new(3165, 3165), CharacterRange::new(3168, 3171), CharacterRange::new(3174, 3183), CharacterRange::new(3200, 3203), CharacterRange::new(3205, 3212), CharacterRange::new(3214, 3216),
    CharacterRange::new(3218, 3240), CharacterRange::new(3242, 3251), CharacterRange::new(3253, 3257), CharacterRange::new(3260, 3268), CharacterRange::new(3270, 3272), CharacterRange::new(3274, 3277),
    CharacterRange::new(3285, 3286), CharacterRange::new(3293, 3294), CharacterRange::new(3296, 3299), CharacterRange::new(3302, 3311), CharacterRange::new(3313, 3315), CharacterRange::new(3328, 3340),
    CharacterRange::new(3342, 3344), CharacterRange::new(3346, 3396), CharacterRange::new(3398, 3400), CharacterRange::new(3402, 3406), CharacterRange::new(3412, 3415), CharacterRange::new(3423, 3427),
    CharacterRange::new(3430, 3439), CharacterRange::new(3450, 3455), CharacterRange::new(3457, 3459), CharacterRange::new(3461, 3478), CharacterRange::new(3482, 3505), CharacterRange::new(3507, 3515),
    CharacterRange::new(3517, 3517), CharacterRange::new(3520, 3526), CharacterRange::new(3530, 3530), CharacterRange::new(3535, 3540), CharacterRange::new(3542, 3542), CharacterRange::new(3544, 3551),
    CharacterRange::new(3558, 3567), CharacterRange::new(3570, 3571), CharacterRange::new(3585, 3642), CharacterRange::new(3648, 3662), CharacterRange::new(3664, 3673), CharacterRange::new(3713, 3714),
    CharacterRange::new(3716, 3716), CharacterRange::new(3718, 3722), CharacterRange::new(3724, 3747), CharacterRange::new(3749, 3749), CharacterRange::new(3751, 3773), CharacterRange::new(3776, 3780),
    CharacterRange::new(3782, 3782), CharacterRange::new(3784, 3790), CharacterRange::new(3792, 3801), CharacterRange::new(3804, 3807), CharacterRange::new(3840, 3840), CharacterRange::new(3864, 3865),
    CharacterRange::new(3872, 3881), CharacterRange::new(3893, 3893), CharacterRange::new(3895, 3895), CharacterRange::new(3897, 3897), CharacterRange::new(3902, 3911), CharacterRange::new(3913, 3948),
    CharacterRange::new(3953, 3972), CharacterRange::new(3974, 3991), CharacterRange::new(3993, 4028), CharacterRange::new(4038, 4038), CharacterRange::new(4096, 4169), CharacterRange::new(4176, 4253),
    CharacterRange::new(4256, 4293), CharacterRange::new(4295, 4295), CharacterRange::new(4301, 4301), CharacterRange::new(4304, 4346), CharacterRange::new(4348, 4680), CharacterRange::new(4682, 4685),
    CharacterRange::new(4688, 4694), CharacterRange::new(4696, 4696), CharacterRange::new(4698, 4701), CharacterRange::new(4704, 4744), CharacterRange::new(4746, 4749), CharacterRange::new(4752, 4784),
    CharacterRange::new(4786, 4789), CharacterRange::new(4792, 4798), CharacterRange::new(4800, 4800), CharacterRange::new(4802, 4805), CharacterRange::new(4808, 4822), CharacterRange::new(4824, 4880),
    CharacterRange::new(4882, 4885), CharacterRange::new(4888, 4954), CharacterRange::new(4957, 4959), CharacterRange::new(4969, 4977), CharacterRange::new(4992, 5007), CharacterRange::new(5024, 5109),
    CharacterRange::new(5112, 5117), CharacterRange::new(5121, 5740), CharacterRange::new(5743, 5759), CharacterRange::new(5761, 5786), CharacterRange::new(5792, 5866), CharacterRange::new(5870, 5880),
    CharacterRange::new(5888, 5909), CharacterRange::new(5919, 5940), CharacterRange::new(5952, 5971), CharacterRange::new(5984, 5996), CharacterRange::new(5998, 6000), CharacterRange::new(6002, 6003),
    CharacterRange::new(6016, 6099), CharacterRange::new(6103, 6103), CharacterRange::new(6108, 6109), CharacterRange::new(6112, 6121), CharacterRange::new(6155, 6157), CharacterRange::new(6159, 6169),
    CharacterRange::new(6176, 6264), CharacterRange::new(6272, 6314), CharacterRange::new(6320, 6389), CharacterRange::new(6400, 6430), CharacterRange::new(6432, 6443), CharacterRange::new(6448, 6459),
    CharacterRange::new(6470, 6509), CharacterRange::new(6512, 6516), CharacterRange::new(6528, 6571), CharacterRange::new(6576, 6601), CharacterRange::new(6608, 6618), CharacterRange::new(6656, 6683),
    CharacterRange::new(6688, 6750), CharacterRange::new(6752, 6780), CharacterRange::new(6783, 6793), CharacterRange::new(6800, 6809), CharacterRange::new(6823, 6823), CharacterRange::new(6832, 6845),
    CharacterRange::new(6847, 6862), CharacterRange::new(6912, 6988), CharacterRange::new(6992, 7001), CharacterRange::new(7019, 7027), CharacterRange::new(7040, 7155), CharacterRange::new(7168, 7223),
    CharacterRange::new(7232, 7241), CharacterRange::new(7245, 7293), CharacterRange::new(7296, 7306), CharacterRange::new(7312, 7354), CharacterRange::new(7357, 7359), CharacterRange::new(7376, 7378),
    CharacterRange::new(7380, 7418), CharacterRange::new(7424, 7957), CharacterRange::new(7960, 7965), CharacterRange::new(7968, 8005), CharacterRange::new(8008, 8013), CharacterRange::new(8016, 8023),
    CharacterRange::new(8025, 8025), CharacterRange::new(8027, 8027), CharacterRange::new(8029, 8029), CharacterRange::new(8031, 8061), CharacterRange::new(8064, 8116), CharacterRange::new(8118, 8124),
    CharacterRange::new(8126, 8126), CharacterRange::new(8130, 8132), CharacterRange::new(8134, 8140), CharacterRange::new(8144, 8147), CharacterRange::new(8150, 8155), CharacterRange::new(8160, 8172),
    CharacterRange::new(8178, 8180), CharacterRange::new(8182, 8188), CharacterRange::new(8204, 8205), CharacterRange::new(8255, 8256), CharacterRange::new(8276, 8276), CharacterRange::new(8305, 8305),
    CharacterRange::new(8319, 8319), CharacterRange::new(8336, 8348), CharacterRange::new(8400, 8412), CharacterRange::new(8417, 8417), CharacterRange::new(8421, 8432), CharacterRange::new(8450, 8450),
    CharacterRange::new(8455, 8455), CharacterRange::new(8458, 8467), CharacterRange::new(8469, 8469), CharacterRange::new(8472, 8477), CharacterRange::new(8484, 8484), CharacterRange::new(8486, 8486),
    CharacterRange::new(8488, 8488), CharacterRange::new(8490, 8505), CharacterRange::new(8508, 8511), CharacterRange::new(8517, 8521), CharacterRange::new(8526, 8526), CharacterRange::new(8544, 8584),
    CharacterRange::new(11264, 11492), CharacterRange::new(11499, 11507), CharacterRange::new(11520, 11557), CharacterRange::new(11559, 11559), CharacterRange::new(11565, 11565), CharacterRange::new(11568, 11623),
    CharacterRange::new(11631, 11631), CharacterRange::new(11647, 11670), CharacterRange::new(11680, 11686), CharacterRange::new(11688, 11694), CharacterRange::new(11696, 11702), CharacterRange::new(11704, 11710),
    CharacterRange::new(11712, 11718), CharacterRange::new(11720, 11726), CharacterRange::new(11728, 11734), CharacterRange::new(11736, 11742), CharacterRange::new(11744, 11775), CharacterRange::new(12293, 12295),
    CharacterRange::new(12321, 12335), CharacterRange::new(12337, 12341), CharacterRange::new(12344, 12348), CharacterRange::new(12353, 12438), CharacterRange::new(12441, 12442), CharacterRange::new(12445, 12447),
    CharacterRange::new(12449, 12543), CharacterRange::new(12549, 12591), CharacterRange::new(12593, 12686), CharacterRange::new(12704, 12735), CharacterRange::new(12784, 12799), CharacterRange::new(13312, 19903),
    CharacterRange::new(19968, 42124), CharacterRange::new(42192, 42237), CharacterRange::new(42240, 42508), CharacterRange::new(42512, 42539), CharacterRange::new(42560, 42607), CharacterRange::new(42612, 42621),
    CharacterRange::new(42623, 42737), CharacterRange::new(42775, 42783), CharacterRange::new(42786, 42888), CharacterRange::new(42891, 42957), CharacterRange::new(42960, 42961), CharacterRange::new(42963, 42963),
    CharacterRange::new(42965, 42972), CharacterRange::new(42994, 43047), CharacterRange::new(43052, 43052), CharacterRange::new(43072, 43123), CharacterRange::new(43136, 43205), CharacterRange::new(43216, 43225),
    CharacterRange::new(43232, 43255), CharacterRange::new(43259, 43259), CharacterRange::new(43261, 43309), CharacterRange::new(43312, 43347), CharacterRange::new(43360, 43388), CharacterRange::new(43392, 43456),
    CharacterRange::new(43471, 43481), CharacterRange::new(43488, 43518), CharacterRange::new(43520, 43574), CharacterRange::new(43584, 43597), CharacterRange::new(43600, 43609), CharacterRange::new(43616, 43638),
    CharacterRange::new(43642, 43714), CharacterRange::new(43739, 43741), CharacterRange::new(43744, 43759), CharacterRange::new(43762, 43766), CharacterRange::new(43777, 43782), CharacterRange::new(43785, 43790),
    CharacterRange::new(43793, 43798), CharacterRange::new(43808, 43814), CharacterRange::new(43816, 43822), CharacterRange::new(43824, 43866), CharacterRange::new(43868, 43881), CharacterRange::new(43888, 44010),
    CharacterRange::new(44012, 44013), CharacterRange::new(44016, 44025), CharacterRange::new(44032, 55203), CharacterRange::new(55216, 55238), CharacterRange::new(55243, 55291), CharacterRange::new(63744, 64109),
    CharacterRange::new(64112, 64217), CharacterRange::new(64256, 64262), CharacterRange::new(64275, 64279), CharacterRange::new(64285, 64296), CharacterRange::new(64298, 64310), CharacterRange::new(64312, 64316),
    CharacterRange::new(64318, 64318), CharacterRange::new(64320, 64321), CharacterRange::new(64323, 64324), CharacterRange::new(64326, 64433), CharacterRange::new(64467, 64605), CharacterRange::new(64612, 64829),
    CharacterRange::new(64848, 64911), CharacterRange::new(64914, 64967), CharacterRange::new(65008, 65017), CharacterRange::new(65024, 65039), CharacterRange::new(65056, 65071), CharacterRange::new(65075, 65076),
    CharacterRange::new(65101, 65103), CharacterRange::new(65137, 65137), CharacterRange::new(65139, 65139), CharacterRange::new(65143, 65143), CharacterRange::new(65145, 65145), CharacterRange::new(65147, 65147),
    CharacterRange::new(65149, 65149), CharacterRange::new(65151, 65276), CharacterRange::new(65296, 65305), CharacterRange::new(65313, 65338), CharacterRange::new(65343, 65343), CharacterRange::new(65345, 65370),
    CharacterRange::new(65381, 65470), CharacterRange::new(65474, 65479), CharacterRange::new(65482, 65487), CharacterRange::new(65490, 65495), CharacterRange::new(65498, 65500), CharacterRange::new(65536, 65547),
    CharacterRange::new(65549, 65574), CharacterRange::new(65576, 65594), CharacterRange::new(65596, 65597), CharacterRange::new(65599, 65613), CharacterRange::new(65616, 65629), CharacterRange::new(65664, 65786),
    CharacterRange::new(65856, 65908), CharacterRange::new(66045, 66045), CharacterRange::new(66176, 66204), CharacterRange::new(66208, 66256), CharacterRange::new(66272, 66272), CharacterRange::new(66304, 66335),
    CharacterRange::new(66349, 66378), CharacterRange::new(66384, 66426), CharacterRange::new(66432, 66461), CharacterRange::new(66464, 66499), CharacterRange::new(66504, 66511), CharacterRange::new(66513, 66517),
    CharacterRange::new(66560, 66717), CharacterRange::new(66720, 66729), CharacterRange::new(66736, 66771), CharacterRange::new(66776, 66811), CharacterRange::new(66816, 66855), CharacterRange::new(66864, 66915),
    CharacterRange::new(66928, 66938), CharacterRange::new(66940, 66954), CharacterRange::new(66956, 66962), CharacterRange::new(66964, 66965), CharacterRange::new(66967, 66977), CharacterRange::new(66979, 66993),
    CharacterRange::new(66995, 67001), CharacterRange::new(67003, 67004), CharacterRange::new(67008, 67059), CharacterRange::new(67072, 67382), CharacterRange::new(67392, 67413), CharacterRange::new(67424, 67431),
    CharacterRange::new(67456, 67461), CharacterRange::new(67463, 67504), CharacterRange::new(67506, 67514), CharacterRange::new(67584, 67589), CharacterRange::new(67592, 67592), CharacterRange::new(67594, 67637),
    CharacterRange::new(67639, 67640), CharacterRange::new(67644, 67644), CharacterRange::new(67647, 67669), CharacterRange::new(67680, 67702), CharacterRange::new(67712, 67742), CharacterRange::new(67808, 67826),
    CharacterRange::new(67828, 67829), CharacterRange::new(67840, 67861), CharacterRange::new(67872, 67897), CharacterRange::new(67968, 68023), CharacterRange::new(68030, 68031), CharacterRange::new(68096, 68099),
    CharacterRange::new(68101, 68102), CharacterRange::new(68108, 68115), CharacterRange::new(68117, 68119), CharacterRange::new(68121, 68149), CharacterRange::new(68152, 68154), CharacterRange::new(68159, 68159),
    CharacterRange::new(68192, 68220), CharacterRange::new(68224, 68252), CharacterRange::new(68288, 68295), CharacterRange::new(68297, 68326), CharacterRange::new(68352, 68405), CharacterRange::new(68416, 68437),
    CharacterRange::new(68448, 68466), CharacterRange::new(68480, 68497), CharacterRange::new(68608, 68680), CharacterRange::new(68736, 68786), CharacterRange::new(68800, 68850), CharacterRange::new(68864, 68903),
    CharacterRange::new(68912, 68921), CharacterRange::new(68928, 68965), CharacterRange::new(68969, 68973), CharacterRange::new(68975, 68997), CharacterRange::new(69248, 69289), CharacterRange::new(69291, 69292),
    CharacterRange::new(69296, 69297), CharacterRange::new(69314, 69316), CharacterRange::new(69372, 69404), CharacterRange::new(69415, 69415), CharacterRange::new(69424, 69456), CharacterRange::new(69488, 69509),
    CharacterRange::new(69552, 69572), CharacterRange::new(69600, 69622), CharacterRange::new(69632, 69702), CharacterRange::new(69734, 69749), CharacterRange::new(69759, 69818), CharacterRange::new(69826, 69826),
    CharacterRange::new(69840, 69864), CharacterRange::new(69872, 69881), CharacterRange::new(69888, 69940), CharacterRange::new(69942, 69951), CharacterRange::new(69956, 69959), CharacterRange::new(69968, 70003),
    CharacterRange::new(70006, 70006), CharacterRange::new(70016, 70084), CharacterRange::new(70089, 70092), CharacterRange::new(70094, 70106), CharacterRange::new(70108, 70108), CharacterRange::new(70144, 70161),
    CharacterRange::new(70163, 70199), CharacterRange::new(70206, 70209), CharacterRange::new(70272, 70278), CharacterRange::new(70280, 70280), CharacterRange::new(70282, 70285), CharacterRange::new(70287, 70301),
    CharacterRange::new(70303, 70312), CharacterRange::new(70320, 70378), CharacterRange::new(70384, 70393), CharacterRange::new(70400, 70403), CharacterRange::new(70405, 70412), CharacterRange::new(70415, 70416),
    CharacterRange::new(70419, 70440), CharacterRange::new(70442, 70448), CharacterRange::new(70450, 70451), CharacterRange::new(70453, 70457), CharacterRange::new(70459, 70468), CharacterRange::new(70471, 70472),
    CharacterRange::new(70475, 70477), CharacterRange::new(70480, 70480), CharacterRange::new(70487, 70487), CharacterRange::new(70493, 70499), CharacterRange::new(70502, 70508), CharacterRange::new(70512, 70516),
    CharacterRange::new(70528, 70537), CharacterRange::new(70539, 70539), CharacterRange::new(70542, 70542), CharacterRange::new(70544, 70581), CharacterRange::new(70583, 70592), CharacterRange::new(70594, 70594),
    CharacterRange::new(70597, 70597), CharacterRange::new(70599, 70602), CharacterRange::new(70604, 70611), CharacterRange::new(70625, 70626), CharacterRange::new(70656, 70730), CharacterRange::new(70736, 70745),
    CharacterRange::new(70750, 70753), CharacterRange::new(70784, 70853), CharacterRange::new(70855, 70855), CharacterRange::new(70864, 70873), CharacterRange::new(71040, 71093), CharacterRange::new(71096, 71104),
    CharacterRange::new(71128, 71133), CharacterRange::new(71168, 71232), CharacterRange::new(71236, 71236), CharacterRange::new(71248, 71257), CharacterRange::new(71296, 71352), CharacterRange::new(71360, 71369),
    CharacterRange::new(71376, 71395), CharacterRange::new(71424, 71450), CharacterRange::new(71453, 71467), CharacterRange::new(71472, 71481), CharacterRange::new(71488, 71494), CharacterRange::new(71680, 71738),
    CharacterRange::new(71840, 71913), CharacterRange::new(71935, 71942), CharacterRange::new(71945, 71945), CharacterRange::new(71948, 71955), CharacterRange::new(71957, 71958), CharacterRange::new(71960, 71989),
    CharacterRange::new(71991, 71992), CharacterRange::new(71995, 72003), CharacterRange::new(72016, 72025), CharacterRange::new(72096, 72103), CharacterRange::new(72106, 72151), CharacterRange::new(72154, 72161),
    CharacterRange::new(72163, 72164), CharacterRange::new(72192, 72254), CharacterRange::new(72263, 72263), CharacterRange::new(72272, 72345), CharacterRange::new(72349, 72349), CharacterRange::new(72368, 72440),
    CharacterRange::new(72640, 72672), CharacterRange::new(72688, 72697), CharacterRange::new(72704, 72712), CharacterRange::new(72714, 72758), CharacterRange::new(72760, 72768), CharacterRange::new(72784, 72793),
    CharacterRange::new(72818, 72847), CharacterRange::new(72850, 72871), CharacterRange::new(72873, 72886), CharacterRange::new(72960, 72966), CharacterRange::new(72968, 72969), CharacterRange::new(72971, 73014),
    CharacterRange::new(73018, 73018), CharacterRange::new(73020, 73021), CharacterRange::new(73023, 73031), CharacterRange::new(73040, 73049), CharacterRange::new(73056, 73061), CharacterRange::new(73063, 73064),
    CharacterRange::new(73066, 73102), CharacterRange::new(73104, 73105), CharacterRange::new(73107, 73112), CharacterRange::new(73120, 73129), CharacterRange::new(73440, 73462), CharacterRange::new(73472, 73488),
    CharacterRange::new(73490, 73530), CharacterRange::new(73534, 73538), CharacterRange::new(73552, 73562), CharacterRange::new(73648, 73648), CharacterRange::new(73728, 74649), CharacterRange::new(74752, 74862),
    CharacterRange::new(74880, 75075), CharacterRange::new(77712, 77808), CharacterRange::new(77824, 78895), CharacterRange::new(78912, 78933), CharacterRange::new(78944, 82938), CharacterRange::new(82944, 83526),
    CharacterRange::new(90368, 90425), CharacterRange::new(92160, 92728), CharacterRange::new(92736, 92766), CharacterRange::new(92768, 92777), CharacterRange::new(92784, 92862), CharacterRange::new(92864, 92873),
    CharacterRange::new(92880, 92909), CharacterRange::new(92912, 92916), CharacterRange::new(92928, 92982), CharacterRange::new(92992, 92995), CharacterRange::new(93008, 93017), CharacterRange::new(93027, 93047),
    CharacterRange::new(93053, 93071), CharacterRange::new(93504, 93548), CharacterRange::new(93552, 93561), CharacterRange::new(93760, 93823), CharacterRange::new(93952, 94026), CharacterRange::new(94031, 94087),
    CharacterRange::new(94095, 94111), CharacterRange::new(94176, 94177), CharacterRange::new(94179, 94180), CharacterRange::new(94192, 94193), CharacterRange::new(94208, 100343), CharacterRange::new(100352, 101589),
    CharacterRange::new(101631, 101640), CharacterRange::new(110576, 110579), CharacterRange::new(110581, 110587), CharacterRange::new(110589, 110590), CharacterRange::new(110592, 110882), CharacterRange::new(110898, 110898),
    CharacterRange::new(110928, 110930), CharacterRange::new(110933, 110933), CharacterRange::new(110948, 110951), CharacterRange::new(110960, 111355), CharacterRange::new(113664, 113770), CharacterRange::new(113776, 113788),
    CharacterRange::new(113792, 113800), CharacterRange::new(113808, 113817), CharacterRange::new(113821, 113822), CharacterRange::new(118000, 118009), CharacterRange::new(118528, 118573), CharacterRange::new(118576, 118598),
    CharacterRange::new(119141, 119145), CharacterRange::new(119149, 119154), CharacterRange::new(119163, 119170), CharacterRange::new(119173, 119179), CharacterRange::new(119210, 119213), CharacterRange::new(119362, 119364),
    CharacterRange::new(119808, 119892), CharacterRange::new(119894, 119964), CharacterRange::new(119966, 119967), CharacterRange::new(119970, 119970), CharacterRange::new(119973, 119974), CharacterRange::new(119977, 119980),
    CharacterRange::new(119982, 119993), CharacterRange::new(119995, 119995), CharacterRange::new(119997, 120003), CharacterRange::new(120005, 120069), CharacterRange::new(120071, 120074), CharacterRange::new(120077, 120084),
    CharacterRange::new(120086, 120092), CharacterRange::new(120094, 120121), CharacterRange::new(120123, 120126), CharacterRange::new(120128, 120132), CharacterRange::new(120134, 120134), CharacterRange::new(120138, 120144),
    CharacterRange::new(120146, 120485), CharacterRange::new(120488, 120512), CharacterRange::new(120514, 120538), CharacterRange::new(120540, 120570), CharacterRange::new(120572, 120596), CharacterRange::new(120598, 120628),
    CharacterRange::new(120630, 120654), CharacterRange::new(120656, 120686), CharacterRange::new(120688, 120712), CharacterRange::new(120714, 120744), CharacterRange::new(120746, 120770), CharacterRange::new(120772, 120779),
    CharacterRange::new(120782, 120831), CharacterRange::new(121344, 121398), CharacterRange::new(121403, 121452), CharacterRange::new(121461, 121461), CharacterRange::new(121476, 121476), CharacterRange::new(121499, 121503),
    CharacterRange::new(121505, 121519), CharacterRange::new(122624, 122654), CharacterRange::new(122661, 122666), CharacterRange::new(122880, 122886), CharacterRange::new(122888, 122904), CharacterRange::new(122907, 122913),
    CharacterRange::new(122915, 122916), CharacterRange::new(122918, 122922), CharacterRange::new(122928, 122989), CharacterRange::new(123023, 123023), CharacterRange::new(123136, 123180), CharacterRange::new(123184, 123197),
    CharacterRange::new(123200, 123209), CharacterRange::new(123214, 123214), CharacterRange::new(123536, 123566), CharacterRange::new(123584, 123641), CharacterRange::new(124112, 124153), CharacterRange::new(124368, 124410),
    CharacterRange::new(124896, 124902), CharacterRange::new(124904, 124907), CharacterRange::new(124909, 124910), CharacterRange::new(124912, 124926), CharacterRange::new(124928, 125124), CharacterRange::new(125136, 125142),
    CharacterRange::new(125184, 125259), CharacterRange::new(125264, 125273), CharacterRange::new(126464, 126467), CharacterRange::new(126469, 126495), CharacterRange::new(126497, 126498), CharacterRange::new(126500, 126500),
    CharacterRange::new(126503, 126503), CharacterRange::new(126505, 126514), CharacterRange::new(126516, 126519), CharacterRange::new(126521, 126521), CharacterRange::new(126523, 126523), CharacterRange::new(126530, 126530),
    CharacterRange::new(126535, 126535), CharacterRange::new(126537, 126537), CharacterRange::new(126539, 126539), CharacterRange::new(126541, 126543), CharacterRange::new(126545, 126546), CharacterRange::new(126548, 126548),
    CharacterRange::new(126551, 126551), CharacterRange::new(126553, 126553), CharacterRange::new(126555, 126555), CharacterRange::new(126557, 126557), CharacterRange::new(126559, 126559), CharacterRange::new(126561, 126562),
    CharacterRange::new(126564, 126564), CharacterRange::new(126567, 126570), CharacterRange::new(126572, 126578), CharacterRange::new(126580, 126583), CharacterRange::new(126585, 126588), CharacterRange::new(126590, 126590),
    CharacterRange::new(126592, 126601), CharacterRange::new(126603, 126619), CharacterRange::new(126625, 126627), CharacterRange::new(126629, 126633), CharacterRange::new(126635, 126651), CharacterRange::new(130032, 130041),
    CharacterRange::new(131072, 173791), CharacterRange::new(173824, 177977), CharacterRange::new(177984, 178205), CharacterRange::new(178208, 183969), CharacterRange::new(183984, 191456), CharacterRange::new(191472, 192093),
    CharacterRange::new(194560, 195101), CharacterRange::new(196608, 201546), CharacterRange::new(201552, 205743), CharacterRange::new(917760, 917999),
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
                if eof { state = 501; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 568), (34, 701), (35, 447), (37, 592), (38, 604), (39, 692), (40, 505), (41, 508),
                    (42, 588), (43, 582), (44, 507), (45, 571), (46, 666), (47, 590), (48, 674), (58, 632),
                    (59, 629), (60, 616), (61, 648), (62, 817), (63, 651), (70, 744), (76, 718), (82, 721),
                    (84, 748), (85, 722), (91, 642), (92, 2), (93, 646), (94, 600), (98, 790), (99, 769),
                    (100, 786), (102, 753), (105, 783), (109, 755), (110, 803), (112, 800), (115, 770), (116, 795),
                    (117, 725), (118, 787), (123, 637), (124, 596), (125, 638), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 499; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            1 => {
                if lookahead == 10 { state = 261; lexer.advance(true); continue; }
                return result;
            }
            2 => {
                if lookahead == 10 { state = 261; lexer.advance(true); continue; }
                if lookahead == 13 { state = 1; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            3 => {
                if lookahead == 10 { state = 274; lexer.advance(true); continue; }
                return result;
            }
            4 => {
                if lookahead == 10 { state = 274; lexer.advance(true); continue; }
                if lookahead == 13 { state = 3; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                if lookahead == 10 { state = 273; lexer.advance(true); continue; }
                return result;
            }
            6 => {
                if lookahead == 10 { state = 273; lexer.advance(true); continue; }
                if lookahead == 13 { state = 5; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if lookahead == 10 { state = 276; lexer.advance(true); continue; }
                return result;
            }
            8 => {
                if lookahead == 10 { state = 276; lexer.advance(true); continue; }
                if lookahead == 13 { state = 7; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if lookahead == 10 { state = 275; lexer.advance(true); continue; }
                return result;
            }
            10 => {
                if lookahead == 10 { state = 275; lexer.advance(true); continue; }
                if lookahead == 13 { state = 9; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if lookahead == 10 { state = 277; lexer.advance(true); continue; }
                return result;
            }
            12 => {
                if lookahead == 10 { state = 277; lexer.advance(true); continue; }
                if lookahead == 13 { state = 11; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if lookahead == 10 { state = 278; lexer.advance(true); continue; }
                return result;
            }
            14 => {
                if lookahead == 10 { state = 278; lexer.advance(true); continue; }
                if lookahead == 13 { state = 13; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if lookahead == 10 { state = 329; lexer.advance(true); continue; }
                return result;
            }
            16 => {
                if lookahead == 10 { state = 329; lexer.advance(true); continue; }
                if lookahead == 13 { state = 15; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if lookahead == 10 { state = 264; lexer.advance(true); continue; }
                return result;
            }
            18 => {
                if lookahead == 10 { state = 264; lexer.advance(true); continue; }
                if lookahead == 13 { state = 17; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if lookahead == 10 { state = 266; lexer.advance(true); continue; }
                return result;
            }
            20 => {
                if lookahead == 10 { state = 266; lexer.advance(true); continue; }
                if lookahead == 13 { state = 19; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                if lookahead == 10 { state = 267; lexer.advance(true); continue; }
                return result;
            }
            22 => {
                if lookahead == 10 { state = 267; lexer.advance(true); continue; }
                if lookahead == 13 { state = 21; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            23 => {
                if lookahead == 10 { state = 391; lexer.advance(true); continue; }
                return result;
            }
            24 => {
                if lookahead == 10 { state = 391; lexer.advance(true); continue; }
                if lookahead == 13 { state = 23; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            25 => {
                if lookahead == 10 { state = 330; lexer.advance(true); continue; }
                return result;
            }
            26 => {
                if lookahead == 10 { state = 330; lexer.advance(true); continue; }
                if lookahead == 13 { state = 25; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            27 => {
                if lookahead == 10 { state = 271; lexer.advance(true); continue; }
                return result;
            }
            28 => {
                if lookahead == 10 { state = 271; lexer.advance(true); continue; }
                if lookahead == 13 { state = 27; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                if lookahead == 10 { state = 291; lexer.advance(true); continue; }
                return result;
            }
            30 => {
                if lookahead == 10 { state = 291; lexer.advance(true); continue; }
                if lookahead == 13 { state = 29; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            31 => {
                if lookahead == 10 { state = 286; lexer.advance(true); continue; }
                return result;
            }
            32 => {
                if lookahead == 10 { state = 286; lexer.advance(true); continue; }
                if lookahead == 13 { state = 31; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            33 => {
                if lookahead == 10 { state = 279; lexer.advance(true); continue; }
                return result;
            }
            34 => {
                if lookahead == 10 { state = 279; lexer.advance(true); continue; }
                if lookahead == 13 { state = 33; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            35 => {
                if lookahead == 10 { state = 309; lexer.advance(true); continue; }
                return result;
            }
            36 => {
                if lookahead == 10 { state = 309; lexer.advance(true); continue; }
                if lookahead == 13 { state = 35; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                if lookahead == 10 { state = 313; lexer.advance(true); continue; }
                return result;
            }
            38 => {
                if lookahead == 10 { state = 313; lexer.advance(true); continue; }
                if lookahead == 13 { state = 37; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                if lookahead == 10 { state = 308; lexer.advance(true); continue; }
                return result;
            }
            40 => {
                if lookahead == 10 { state = 308; lexer.advance(true); continue; }
                if lookahead == 13 { state = 39; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                if lookahead == 10 { state = 331; lexer.advance(true); continue; }
                return result;
            }
            42 => {
                if lookahead == 10 { state = 331; lexer.advance(true); continue; }
                if lookahead == 13 { state = 41; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            43 => {
                if lookahead == 10 { state = 366; lexer.advance(true); continue; }
                return result;
            }
            44 => {
                if lookahead == 10 { state = 366; lexer.advance(true); continue; }
                if lookahead == 13 { state = 43; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            45 => {
                if lookahead == 10 { state = 341; lexer.advance(true); continue; }
                return result;
            }
            46 => {
                if lookahead == 10 { state = 341; lexer.advance(true); continue; }
                if lookahead == 13 { state = 45; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                if lookahead == 10 { state = 355; lexer.advance(true); continue; }
                return result;
            }
            48 => {
                if lookahead == 10 { state = 355; lexer.advance(true); continue; }
                if lookahead == 13 { state = 47; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                if lookahead == 10 { state = 281; lexer.advance(true); continue; }
                return result;
            }
            50 => {
                if lookahead == 10 { state = 281; lexer.advance(true); continue; }
                if lookahead == 13 { state = 49; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            51 => {
                if lookahead == 10 { state = 316; lexer.advance(true); continue; }
                return result;
            }
            52 => {
                if lookahead == 10 { state = 316; lexer.advance(true); continue; }
                if lookahead == 13 { state = 51; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                if lookahead == 10 { state = 287; lexer.advance(true); continue; }
                return result;
            }
            54 => {
                if lookahead == 10 { state = 287; lexer.advance(true); continue; }
                if lookahead == 13 { state = 53; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            55 => {
                if lookahead == 10 { state = 333; lexer.advance(true); continue; }
                return result;
            }
            56 => {
                if lookahead == 10 { state = 333; lexer.advance(true); continue; }
                if lookahead == 13 { state = 55; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            57 => {
                if lookahead == 10 { state = 293; lexer.advance(true); continue; }
                return result;
            }
            58 => {
                if lookahead == 10 { state = 293; lexer.advance(true); continue; }
                if lookahead == 13 { state = 57; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                if lookahead == 10 { state = 295; lexer.advance(true); continue; }
                return result;
            }
            60 => {
                if lookahead == 10 { state = 295; lexer.advance(true); continue; }
                if lookahead == 13 { state = 59; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            61 => {
                if lookahead == 10 { state = 345; lexer.advance(true); continue; }
                return result;
            }
            62 => {
                if lookahead == 10 { state = 345; lexer.advance(true); continue; }
                if lookahead == 13 { state = 61; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                if lookahead == 10 { state = 312; lexer.advance(true); continue; }
                return result;
            }
            64 => {
                if lookahead == 10 { state = 312; lexer.advance(true); continue; }
                if lookahead == 13 { state = 63; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            65 => {
                if lookahead == 10 { state = 335; lexer.advance(true); continue; }
                return result;
            }
            66 => {
                if lookahead == 10 { state = 335; lexer.advance(true); continue; }
                if lookahead == 13 { state = 65; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            67 => {
                if lookahead == 10 { state = 368; lexer.advance(true); continue; }
                return result;
            }
            68 => {
                if lookahead == 10 { state = 368; lexer.advance(true); continue; }
                if lookahead == 13 { state = 67; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            69 => {
                if lookahead == 10 { state = 378; lexer.advance(true); continue; }
                return result;
            }
            70 => {
                if lookahead == 10 { state = 378; lexer.advance(true); continue; }
                if lookahead == 13 { state = 69; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            71 => {
                if lookahead == 10 { state = 351; lexer.advance(true); continue; }
                return result;
            }
            72 => {
                if lookahead == 10 { state = 351; lexer.advance(true); continue; }
                if lookahead == 13 { state = 71; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            73 => {
                if lookahead == 10 { state = 360; lexer.advance(true); continue; }
                return result;
            }
            74 => {
                if lookahead == 10 { state = 360; lexer.advance(true); continue; }
                if lookahead == 13 { state = 73; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            75 => {
                if lookahead == 10 { state = 334; lexer.advance(true); continue; }
                return result;
            }
            76 => {
                if lookahead == 10 { state = 334; lexer.advance(true); continue; }
                if lookahead == 13 { state = 75; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            77 => {
                if lookahead == 10 { state = 349; lexer.advance(true); continue; }
                return result;
            }
            78 => {
                if lookahead == 10 { state = 349; lexer.advance(true); continue; }
                if lookahead == 13 { state = 77; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            79 => {
                if lookahead == 10 { state = 392; lexer.advance(true); continue; }
                return result;
            }
            80 => {
                if lookahead == 10 { state = 392; lexer.advance(true); continue; }
                if lookahead == 13 { state = 79; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            81 => {
                if lookahead == 10 { state = 365; lexer.advance(true); continue; }
                return result;
            }
            82 => {
                if lookahead == 10 { state = 365; lexer.advance(true); continue; }
                if lookahead == 13 { state = 81; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            83 => {
                if lookahead == 10 { state = 393; lexer.advance(true); continue; }
                return result;
            }
            84 => {
                if lookahead == 10 { state = 393; lexer.advance(true); continue; }
                if lookahead == 13 { state = 83; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            85 => {
                if lookahead == 10 { state = 394; lexer.advance(true); continue; }
                return result;
            }
            86 => {
                if lookahead == 10 { state = 394; lexer.advance(true); continue; }
                if lookahead == 13 { state = 85; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            87 => {
                if lookahead == 10 { state = 282; lexer.advance(true); continue; }
                return result;
            }
            88 => {
                if lookahead == 10 { state = 282; lexer.advance(true); continue; }
                if lookahead == 13 { state = 87; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            89 => {
                if lookahead == 10 { state = 397; lexer.advance(true); continue; }
                return result;
            }
            90 => {
                if lookahead == 10 { state = 397; lexer.advance(true); continue; }
                if lookahead == 13 { state = 89; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            91 => {
                if lookahead == 10 { state = 386; lexer.advance(true); continue; }
                return result;
            }
            92 => {
                if lookahead == 10 { state = 386; lexer.advance(true); continue; }
                if lookahead == 13 { state = 91; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            93 => {
                if lookahead == 10 { state = 398; lexer.advance(true); continue; }
                return result;
            }
            94 => {
                if lookahead == 10 { state = 398; lexer.advance(true); continue; }
                if lookahead == 13 { state = 93; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            95 => {
                if lookahead == 10 { state = 396; lexer.advance(true); continue; }
                return result;
            }
            96 => {
                if lookahead == 10 { state = 396; lexer.advance(true); continue; }
                if lookahead == 13 { state = 95; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            97 => {
                if lookahead == 10 { state = 292; lexer.advance(true); continue; }
                return result;
            }
            98 => {
                if lookahead == 10 { state = 292; lexer.advance(true); continue; }
                if lookahead == 13 { state = 97; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            99 => {
                if lookahead == 10 { state = 289; lexer.advance(true); continue; }
                return result;
            }
            100 => {
                if lookahead == 10 { state = 289; lexer.advance(true); continue; }
                if lookahead == 13 { state = 99; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            101 => {
                if lookahead == 10 { state = 300; lexer.advance(true); continue; }
                return result;
            }
            102 => {
                if lookahead == 10 { state = 300; lexer.advance(true); continue; }
                if lookahead == 13 { state = 101; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            103 => {
                if lookahead == 10 { state = 298; lexer.advance(true); continue; }
                return result;
            }
            104 => {
                if lookahead == 10 { state = 298; lexer.advance(true); continue; }
                if lookahead == 13 { state = 103; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            105 => {
                if lookahead == 10 { state = 319; lexer.advance(true); continue; }
                return result;
            }
            106 => {
                if lookahead == 10 { state = 319; lexer.advance(true); continue; }
                if lookahead == 13 { state = 105; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            107 => {
                if lookahead == 10 { state = 310; lexer.advance(true); continue; }
                return result;
            }
            108 => {
                if lookahead == 10 { state = 310; lexer.advance(true); continue; }
                if lookahead == 13 { state = 107; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            109 => {
                if lookahead == 10 { state = 340; lexer.advance(true); continue; }
                return result;
            }
            110 => {
                if lookahead == 10 { state = 340; lexer.advance(true); continue; }
                if lookahead == 13 { state = 109; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            111 => {
                if lookahead == 10 { state = 332; lexer.advance(true); continue; }
                return result;
            }
            112 => {
                if lookahead == 10 { state = 332; lexer.advance(true); continue; }
                if lookahead == 13 { state = 111; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            113 => {
                if lookahead == 10 { state = 343; lexer.advance(true); continue; }
                return result;
            }
            114 => {
                if lookahead == 10 { state = 343; lexer.advance(true); continue; }
                if lookahead == 13 { state = 113; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            115 => {
                if lookahead == 10 { state = 344; lexer.advance(true); continue; }
                return result;
            }
            116 => {
                if lookahead == 10 { state = 344; lexer.advance(true); continue; }
                if lookahead == 13 { state = 115; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            117 => {
                if lookahead == 10 { state = 399; lexer.advance(true); continue; }
                return result;
            }
            118 => {
                if lookahead == 10 { state = 399; lexer.advance(true); continue; }
                if lookahead == 13 { state = 117; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            119 => {
                if lookahead == 10 { state = 395; lexer.advance(true); continue; }
                return result;
            }
            120 => {
                if lookahead == 10 { state = 395; lexer.advance(true); continue; }
                if lookahead == 13 { state = 119; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            121 => {
                if lookahead == 10 { state = 350; lexer.advance(true); continue; }
                return result;
            }
            122 => {
                if lookahead == 10 { state = 350; lexer.advance(true); continue; }
                if lookahead == 13 { state = 121; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            123 => {
                if lookahead == 10 { state = 272; lexer.advance(true); continue; }
                return result;
            }
            124 => {
                if lookahead == 10 { state = 272; lexer.advance(true); continue; }
                if lookahead == 13 { state = 123; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            125 => {
                if lookahead == 10 { state = 339; lexer.advance(true); continue; }
                return result;
            }
            126 => {
                if lookahead == 10 { state = 339; lexer.advance(true); continue; }
                if lookahead == 13 { state = 125; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            127 => {
                if lookahead == 10 { state = 400; lexer.advance(true); continue; }
                return result;
            }
            128 => {
                if lookahead == 10 { state = 400; lexer.advance(true); continue; }
                if lookahead == 13 { state = 127; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            129 => {
                if lookahead == 10 { state = 364; lexer.advance(true); continue; }
                return result;
            }
            130 => {
                if lookahead == 10 { state = 364; lexer.advance(true); continue; }
                if lookahead == 13 { state = 129; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            131 => {
                if lookahead == 10 { state = 406; lexer.advance(true); continue; }
                return result;
            }
            132 => {
                if lookahead == 10 { state = 406; lexer.advance(true); continue; }
                if lookahead == 13 { state = 131; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            133 => {
                if lookahead == 10 { state = 401; lexer.advance(true); continue; }
                return result;
            }
            134 => {
                if lookahead == 10 { state = 401; lexer.advance(true); continue; }
                if lookahead == 13 { state = 133; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            135 => {
                if lookahead == 10 { state = 137; lexer.advance(true); continue; }
                return result;
            }
            136 => {
                if lookahead == 10 { state = 137; lexer.advance(true); continue; }
                if lookahead == 13 { state = 135; lexer.advance(true); continue; }
                return result;
            }
            137 => {
                if let Some(next) = advance_map(&[
                    (10, 510), (33, 432), (37, 591), (38, 603), (40, 566), (42, 587), (43, 580), (45, 570),
                    (47, 589), (60, 617), (61, 433), (62, 608),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 92 { state = 136; lexer.advance(true); continue; }
                if lookahead == 94 { state = 598; lexer.advance(false); continue; }
                if lookahead == 124 { state = 597; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 137; lexer.advance(true); continue; }
                return result;
            }
            138 => {
                if lookahead == 10 { state = 280; lexer.advance(true); continue; }
                return result;
            }
            139 => {
                if lookahead == 10 { state = 280; lexer.advance(true); continue; }
                if lookahead == 13 { state = 138; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            140 => {
                if lookahead == 10 { state = 405; lexer.advance(true); continue; }
                return result;
            }
            141 => {
                if lookahead == 10 { state = 405; lexer.advance(true); continue; }
                if lookahead == 13 { state = 140; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            142 => {
                if lookahead == 10 { state = 387; lexer.advance(true); continue; }
                return result;
            }
            143 => {
                if lookahead == 10 { state = 387; lexer.advance(true); continue; }
                if lookahead == 13 { state = 142; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            144 => {
                if lookahead == 10 { state = 388; lexer.advance(true); continue; }
                return result;
            }
            145 => {
                if lookahead == 10 { state = 388; lexer.advance(true); continue; }
                if lookahead == 13 { state = 144; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            146 => {
                if lookahead == 10 { state = 389; lexer.advance(true); continue; }
                if lookahead == 34 { state = 701; lexer.advance(false); continue; }
                if lookahead == 47 { state = 702; lexer.advance(false); continue; }
                if lookahead == 92 { state = 147; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 705; lexer.advance(false); continue; }
                if lookahead != 0 { state = 706; lexer.advance(false); continue; }
                return result;
            }
            147 => {
                if lookahead == 10 { state = 708; lexer.advance(false); continue; }
                if lookahead == 13 { state = 707; lexer.advance(false); continue; }
                if lookahead == 85 { state = 497; lexer.advance(false); continue; }
                if lookahead == 117 { state = 489; lexer.advance(false); continue; }
                if lookahead == 120 { state = 483; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 710; lexer.advance(false); continue; }
                if lookahead != 0 { state = 707; lexer.advance(false); continue; }
                return result;
            }
            148 => {
                if lookahead == 10 { state = 402; lexer.advance(true); continue; }
                if lookahead == 39 { state = 692; lexer.advance(false); continue; }
                if lookahead == 47 { state = 695; lexer.advance(false); continue; }
                if lookahead == 92 { state = 694; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 696; lexer.advance(false); continue; }
                if lookahead != 0 { state = 693; lexer.advance(false); continue; }
                return result;
            }
            149 => {
                if lookahead == 10 { state = 503; lexer.advance(false); continue; }
                if lookahead == 13 { state = 153; lexer.advance(false); continue; }
                if lookahead == 40 { state = 505; lexer.advance(false); continue; }
                if lookahead == 47 { state = 531; lexer.advance(false); continue; }
                if lookahead == 92 { state = 526; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 419; lexer.advance(true); continue; }
                if lookahead != 0 { state = 533; lexer.advance(false); continue; }
                return result;
            }
            150 => {
                if lookahead == 10 { state = 503; lexer.advance(false); continue; }
                if lookahead == 13 { state = 153; lexer.advance(false); continue; }
                if lookahead == 47 { state = 531; lexer.advance(false); continue; }
                if lookahead == 92 { state = 526; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 419; lexer.advance(true); continue; }
                if lookahead != 0 { state = 533; lexer.advance(false); continue; }
                return result;
            }
            151 => {
                if lookahead == 10 { state = 503; lexer.advance(false); continue; }
                if lookahead == 13 { state = 152; lexer.advance(false); continue; }
                if lookahead == 40 { state = 566; lexer.advance(false); continue; }
                if lookahead == 47 { state = 410; lexer.advance(false); continue; }
                if lookahead == 92 { state = 155; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 407; lexer.advance(true); continue; }
                return result;
            }
            152 => {
                if lookahead == 10 { state = 503; lexer.advance(false); continue; }
                if lookahead == 40 { state = 566; lexer.advance(false); continue; }
                if lookahead == 47 { state = 410; lexer.advance(false); continue; }
                if lookahead == 92 { state = 155; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 407; lexer.advance(true); continue; }
                return result;
            }
            153 => {
                if lookahead == 10 { state = 503; lexer.advance(false); continue; }
                if lookahead == 47 { state = 531; lexer.advance(false); continue; }
                if lookahead == 92 { state = 526; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 419; lexer.advance(true); continue; }
                if lookahead != 0 { state = 533; lexer.advance(false); continue; }
                return result;
            }
            154 => {
                if lookahead == 10 { state = 407; lexer.advance(true); continue; }
                return result;
            }
            155 => {
                if lookahead == 10 { state = 407; lexer.advance(true); continue; }
                if lookahead == 13 { state = 154; lexer.advance(true); continue; }
                return result;
            }
            156 => {
                if lookahead == 10 { state = 262; lexer.advance(true); continue; }
                return result;
            }
            157 => {
                if lookahead == 10 { state = 262; lexer.advance(true); continue; }
                if lookahead == 13 { state = 156; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            158 => {
                if lookahead == 10 { state = 265; lexer.advance(true); continue; }
                return result;
            }
            159 => {
                if lookahead == 10 { state = 265; lexer.advance(true); continue; }
                if lookahead == 13 { state = 158; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            160 => {
                if lookahead == 10 { state = 269; lexer.advance(true); continue; }
                return result;
            }
            161 => {
                if lookahead == 10 { state = 269; lexer.advance(true); continue; }
                if lookahead == 13 { state = 160; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            162 => {
                if lookahead == 10 { state = 314; lexer.advance(true); continue; }
                return result;
            }
            163 => {
                if lookahead == 10 { state = 314; lexer.advance(true); continue; }
                if lookahead == 13 { state = 162; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            164 => {
                if lookahead == 10 { state = 382; lexer.advance(true); continue; }
                return result;
            }
            165 => {
                if lookahead == 10 { state = 382; lexer.advance(true); continue; }
                if lookahead == 13 { state = 164; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            166 => {
                if lookahead == 10 { state = 325; lexer.advance(true); continue; }
                return result;
            }
            167 => {
                if lookahead == 10 { state = 325; lexer.advance(true); continue; }
                if lookahead == 13 { state = 166; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            168 => {
                if lookahead == 10 { state = 288; lexer.advance(true); continue; }
                return result;
            }
            169 => {
                if lookahead == 10 { state = 288; lexer.advance(true); continue; }
                if lookahead == 13 { state = 168; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            170 => {
                if lookahead == 10 { state = 294; lexer.advance(true); continue; }
                return result;
            }
            171 => {
                if lookahead == 10 { state = 294; lexer.advance(true); continue; }
                if lookahead == 13 { state = 170; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            172 => {
                if lookahead == 10 { state = 346; lexer.advance(true); continue; }
                return result;
            }
            173 => {
                if lookahead == 10 { state = 346; lexer.advance(true); continue; }
                if lookahead == 13 { state = 172; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            174 => {
                if lookahead == 10 { state = 362; lexer.advance(true); continue; }
                return result;
            }
            175 => {
                if lookahead == 10 { state = 362; lexer.advance(true); continue; }
                if lookahead == 13 { state = 174; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            176 => {
                if lookahead == 10 { state = 356; lexer.advance(true); continue; }
                return result;
            }
            177 => {
                if lookahead == 10 { state = 356; lexer.advance(true); continue; }
                if lookahead == 13 { state = 176; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            178 => {
                if lookahead == 10 { state = 379; lexer.advance(true); continue; }
                return result;
            }
            179 => {
                if lookahead == 10 { state = 379; lexer.advance(true); continue; }
                if lookahead == 13 { state = 178; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            180 => {
                if lookahead == 10 { state = 367; lexer.advance(true); continue; }
                return result;
            }
            181 => {
                if lookahead == 10 { state = 367; lexer.advance(true); continue; }
                if lookahead == 13 { state = 180; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            182 => {
                if lookahead == 10 { state = 284; lexer.advance(true); continue; }
                return result;
            }
            183 => {
                if lookahead == 10 { state = 284; lexer.advance(true); continue; }
                if lookahead == 13 { state = 182; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            184 => {
                if lookahead == 10 { state = 304; lexer.advance(true); continue; }
                return result;
            }
            185 => {
                if lookahead == 10 { state = 304; lexer.advance(true); continue; }
                if lookahead == 13 { state = 184; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            186 => {
                if lookahead == 10 { state = 296; lexer.advance(true); continue; }
                return result;
            }
            187 => {
                if lookahead == 10 { state = 296; lexer.advance(true); continue; }
                if lookahead == 13 { state = 186; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            188 => {
                if lookahead == 10 { state = 318; lexer.advance(true); continue; }
                return result;
            }
            189 => {
                if lookahead == 10 { state = 318; lexer.advance(true); continue; }
                if lookahead == 13 { state = 188; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            190 => {
                if lookahead == 10 { state = 320; lexer.advance(true); continue; }
                return result;
            }
            191 => {
                if lookahead == 10 { state = 320; lexer.advance(true); continue; }
                if lookahead == 13 { state = 190; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            192 => {
                if lookahead == 10 { state = 342; lexer.advance(true); continue; }
                return result;
            }
            193 => {
                if lookahead == 10 { state = 342; lexer.advance(true); continue; }
                if lookahead == 13 { state = 192; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            194 => {
                if lookahead == 10 { state = 370; lexer.advance(true); continue; }
                return result;
            }
            195 => {
                if lookahead == 10 { state = 370; lexer.advance(true); continue; }
                if lookahead == 13 { state = 194; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            196 => {
                if lookahead == 10 { state = 337; lexer.advance(true); continue; }
                return result;
            }
            197 => {
                if lookahead == 10 { state = 337; lexer.advance(true); continue; }
                if lookahead == 13 { state = 196; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            198 => {
                if lookahead == 10 { state = 263; lexer.advance(true); continue; }
                return result;
            }
            199 => {
                if lookahead == 10 { state = 263; lexer.advance(true); continue; }
                if lookahead == 13 { state = 198; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            200 => {
                if lookahead == 10 { state = 268; lexer.advance(true); continue; }
                return result;
            }
            201 => {
                if lookahead == 10 { state = 268; lexer.advance(true); continue; }
                if lookahead == 13 { state = 200; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            202 => {
                if lookahead == 10 { state = 315; lexer.advance(true); continue; }
                return result;
            }
            203 => {
                if lookahead == 10 { state = 315; lexer.advance(true); continue; }
                if lookahead == 13 { state = 202; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            204 => {
                if lookahead == 10 { state = 358; lexer.advance(true); continue; }
                return result;
            }
            205 => {
                if lookahead == 10 { state = 358; lexer.advance(true); continue; }
                if lookahead == 13 { state = 204; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            206 => {
                if lookahead == 10 { state = 347; lexer.advance(true); continue; }
                return result;
            }
            207 => {
                if lookahead == 10 { state = 347; lexer.advance(true); continue; }
                if lookahead == 13 { state = 206; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            208 => {
                if lookahead == 10 { state = 363; lexer.advance(true); continue; }
                return result;
            }
            209 => {
                if lookahead == 10 { state = 363; lexer.advance(true); continue; }
                if lookahead == 13 { state = 208; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            210 => {
                if lookahead == 10 { state = 383; lexer.advance(true); continue; }
                return result;
            }
            211 => {
                if lookahead == 10 { state = 383; lexer.advance(true); continue; }
                if lookahead == 13 { state = 210; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            212 => {
                if lookahead == 10 { state = 306; lexer.advance(true); continue; }
                return result;
            }
            213 => {
                if lookahead == 10 { state = 306; lexer.advance(true); continue; }
                if lookahead == 13 { state = 212; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            214 => {
                if lookahead == 10 { state = 328; lexer.advance(true); continue; }
                return result;
            }
            215 => {
                if lookahead == 10 { state = 328; lexer.advance(true); continue; }
                if lookahead == 13 { state = 214; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            216 => {
                if lookahead == 10 { state = 369; lexer.advance(true); continue; }
                return result;
            }
            217 => {
                if lookahead == 10 { state = 369; lexer.advance(true); continue; }
                if lookahead == 13 { state = 216; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            218 => {
                if lookahead == 10 { state = 338; lexer.advance(true); continue; }
                return result;
            }
            219 => {
                if lookahead == 10 { state = 338; lexer.advance(true); continue; }
                if lookahead == 13 { state = 218; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            220 => {
                if lookahead == 10 { state = 270; lexer.advance(true); continue; }
                return result;
            }
            221 => {
                if lookahead == 10 { state = 270; lexer.advance(true); continue; }
                if lookahead == 13 { state = 220; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            222 => {
                if lookahead == 10 { state = 323; lexer.advance(true); continue; }
                return result;
            }
            223 => {
                if lookahead == 10 { state = 323; lexer.advance(true); continue; }
                if lookahead == 13 { state = 222; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            224 => {
                if lookahead == 10 { state = 359; lexer.advance(true); continue; }
                return result;
            }
            225 => {
                if lookahead == 10 { state = 359; lexer.advance(true); continue; }
                if lookahead == 13 { state = 224; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            226 => {
                if lookahead == 10 { state = 348; lexer.advance(true); continue; }
                return result;
            }
            227 => {
                if lookahead == 10 { state = 348; lexer.advance(true); continue; }
                if lookahead == 13 { state = 226; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            228 => {
                if lookahead == 10 { state = 381; lexer.advance(true); continue; }
                return result;
            }
            229 => {
                if lookahead == 10 { state = 381; lexer.advance(true); continue; }
                if lookahead == 13 { state = 228; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            230 => {
                if lookahead == 10 { state = 375; lexer.advance(true); continue; }
                return result;
            }
            231 => {
                if lookahead == 10 { state = 375; lexer.advance(true); continue; }
                if lookahead == 13 { state = 230; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            232 => {
                if lookahead == 10 { state = 302; lexer.advance(true); continue; }
                return result;
            }
            233 => {
                if lookahead == 10 { state = 302; lexer.advance(true); continue; }
                if lookahead == 13 { state = 232; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            234 => {
                if lookahead == 10 { state = 371; lexer.advance(true); continue; }
                return result;
            }
            235 => {
                if lookahead == 10 { state = 371; lexer.advance(true); continue; }
                if lookahead == 13 { state = 234; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            236 => {
                if lookahead == 10 { state = 311; lexer.advance(true); continue; }
                return result;
            }
            237 => {
                if lookahead == 10 { state = 311; lexer.advance(true); continue; }
                if lookahead == 13 { state = 236; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            238 => {
                if lookahead == 10 { state = 354; lexer.advance(true); continue; }
                return result;
            }
            239 => {
                if lookahead == 10 { state = 354; lexer.advance(true); continue; }
                if lookahead == 13 { state = 238; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            240 => {
                if lookahead == 10 { state = 372; lexer.advance(true); continue; }
                return result;
            }
            241 => {
                if lookahead == 10 { state = 372; lexer.advance(true); continue; }
                if lookahead == 13 { state = 240; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            242 => {
                if lookahead == 10 { state = 324; lexer.advance(true); continue; }
                return result;
            }
            243 => {
                if lookahead == 10 { state = 324; lexer.advance(true); continue; }
                if lookahead == 13 { state = 242; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            244 => {
                if lookahead == 10 { state = 353; lexer.advance(true); continue; }
                return result;
            }
            245 => {
                if lookahead == 10 { state = 353; lexer.advance(true); continue; }
                if lookahead == 13 { state = 244; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            246 => {
                if lookahead == 10 { state = 374; lexer.advance(true); continue; }
                return result;
            }
            247 => {
                if lookahead == 10 { state = 374; lexer.advance(true); continue; }
                if lookahead == 13 { state = 246; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            248 => {
                if lookahead == 10 { state = 321; lexer.advance(true); continue; }
                return result;
            }
            249 => {
                if lookahead == 10 { state = 321; lexer.advance(true); continue; }
                if lookahead == 13 { state = 248; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            250 => {
                if lookahead == 10 { state = 385; lexer.advance(true); continue; }
                return result;
            }
            251 => {
                if lookahead == 10 { state = 385; lexer.advance(true); continue; }
                if lookahead == 13 { state = 250; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            252 => {
                if lookahead == 10 { state = 373; lexer.advance(true); continue; }
                return result;
            }
            253 => {
                if lookahead == 10 { state = 373; lexer.advance(true); continue; }
                if lookahead == 13 { state = 252; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            254 => {
                if lookahead == 10 { state = 322; lexer.advance(true); continue; }
                return result;
            }
            255 => {
                if lookahead == 10 { state = 322; lexer.advance(true); continue; }
                if lookahead == 13 { state = 254; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            256 => {
                if lookahead == 10 { state = 377; lexer.advance(true); continue; }
                return result;
            }
            257 => {
                if lookahead == 10 { state = 377; lexer.advance(true); continue; }
                if lookahead == 13 { state = 256; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            258 => {
                if lookahead == 10 { state = 326; lexer.advance(true); continue; }
                return result;
            }
            259 => {
                if lookahead == 10 { state = 326; lexer.advance(true); continue; }
                if lookahead == 13 { state = 258; lexer.advance(true); continue; }
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            260 => {
                if lookahead == 13 { state = 816; lexer.advance(false); continue; }
                if lookahead == 92 { state = 810; lexer.advance(false); continue; }
                if lookahead != 0 { state = 815; lexer.advance(false); continue; }
                return result;
            }
            261 => {
                if let Some(next) = advance_map(&[
                    (33, 568), (34, 701), (35, 447), (37, 592), (38, 604), (39, 692), (40, 566), (41, 508),
                    (42, 588), (43, 582), (44, 507), (45, 571), (46, 666), (47, 590), (48, 674), (58, 632),
                    (59, 629), (60, 616), (61, 648), (62, 817), (63, 651), (70, 744), (76, 718), (82, 721),
                    (84, 748), (85, 722), (91, 642), (92, 2), (93, 646), (94, 600), (98, 790), (99, 769),
                    (100, 786), (102, 753), (105, 783), (109, 755), (110, 803), (112, 800), (115, 770), (116, 795),
                    (117, 725), (118, 787), (123, 637), (124, 596), (125, 638), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 261; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            262 => {
                if let Some(next) = advance_map(&[
                    (33, 568), (34, 701), (35, 456), (37, 592), (38, 604), (39, 692), (40, 566), (41, 508),
                    (42, 588), (43, 582), (44, 507), (45, 572), (46, 666), (47, 590), (48, 674), (58, 632),
                    (59, 629), (60, 616), (61, 648), (62, 609), (63, 651), (70, 744), (76, 718), (82, 721),
                    (84, 748), (85, 722), (91, 640), (92, 157), (93, 646), (94, 600), (98, 790), (99, 769),
                    (100, 786), (102, 753), (105, 783), (109, 755), (110, 803), (112, 800), (115, 770), (116, 795),
                    (117, 725), (118, 787), (123, 637), (124, 596), (125, 638), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 262; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            263 => {
                if let Some(next) = advance_map(&[
                    (33, 568), (34, 701), (35, 456), (37, 591), (38, 603), (39, 692), (40, 566), (41, 508),
                    (42, 587), (43, 583), (44, 507), (45, 573), (46, 666), (47, 589), (48, 674), (58, 632),
                    (59, 629), (60, 618), (61, 433), (62, 608), (63, 651), (70, 744), (76, 718), (82, 721),
                    (84, 748), (85, 722), (91, 640), (92, 199), (93, 445), (94, 601), (98, 790), (99, 769),
                    (100, 786), (102, 753), (105, 783), (109, 755), (110, 803), (112, 800), (115, 770), (116, 795),
                    (117, 725), (118, 787), (123, 637), (124, 597), (125, 638), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 263; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            264 => {
                if let Some(next) = advance_map(&[
                    (33, 568), (34, 701), (37, 592), (38, 604), (39, 692), (40, 566), (41, 508), (42, 588),
                    (43, 582), (44, 507), (45, 571), (46, 666), (47, 590), (48, 674), (58, 426), (60, 616),
                    (61, 648), (62, 609), (63, 651), (70, 744), (76, 718), (82, 721), (84, 748), (85, 722),
                    (91, 640), (92, 18), (94, 600), (98, 790), (99, 769), (100, 786), (102, 753), (105, 783),
                    (109, 755), (110, 803), (112, 800), (115, 770), (116, 795), (117, 725), (118, 787), (123, 637),
                    (124, 596), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 264; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            265 => {
                if let Some(next) = advance_map(&[
                    (33, 568), (34, 701), (37, 592), (38, 604), (39, 692), (40, 566), (42, 588), (43, 582),
                    (44, 507), (45, 572), (46, 666), (47, 590), (48, 674), (58, 426), (60, 616), (61, 648),
                    (62, 817), (63, 651), (70, 744), (76, 718), (82, 721), (84, 748), (85, 722), (91, 640),
                    (92, 159), (94, 600), (98, 790), (99, 769), (100, 786), (102, 753), (105, 783), (109, 755),
                    (110, 803), (112, 800), (115, 770), (116, 795), (117, 725), (118, 787), (123, 637), (124, 596),
                    (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 265; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            266 => {
                if let Some(next) = advance_map(&[
                    (33, 568), (34, 701), (37, 592), (38, 604), (39, 692), (40, 566), (42, 588), (43, 582),
                    (44, 507), (45, 572), (46, 666), (47, 590), (48, 674), (58, 426), (60, 616), (61, 648),
                    (62, 610), (63, 651), (70, 744), (76, 718), (82, 721), (84, 748), (85, 722), (91, 640),
                    (92, 20), (94, 600), (98, 790), (99, 769), (100, 786), (102, 753), (105, 783), (109, 755),
                    (110, 803), (112, 800), (115, 770), (116, 795), (117, 725), (118, 787), (123, 637), (124, 596),
                    (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 266; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            267 => {
                if let Some(next) = advance_map(&[
                    (33, 568), (34, 701), (37, 592), (38, 604), (39, 692), (40, 566), (42, 588), (43, 582),
                    (44, 507), (45, 572), (46, 666), (47, 590), (48, 674), (58, 426), (60, 616), (61, 648),
                    (62, 609), (63, 651), (70, 744), (76, 718), (82, 721), (84, 748), (85, 722), (91, 640),
                    (92, 22), (93, 445), (94, 600), (98, 790), (99, 769), (100, 786), (102, 753), (105, 783),
                    (109, 755), (110, 803), (112, 800), (115, 770), (116, 795), (117, 725), (118, 787), (123, 637),
                    (124, 596), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 267; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            268 => {
                if let Some(next) = advance_map(&[
                    (33, 568), (34, 701), (37, 591), (38, 603), (39, 692), (40, 566), (42, 587), (43, 583),
                    (44, 507), (45, 573), (46, 666), (47, 589), (48, 674), (58, 426), (60, 618), (61, 433),
                    (62, 817), (63, 651), (70, 744), (76, 718), (82, 721), (84, 748), (85, 722), (91, 640),
                    (92, 201), (94, 601), (98, 790), (99, 769), (100, 786), (102, 753), (105, 783), (109, 755),
                    (110, 803), (112, 800), (115, 770), (116, 795), (117, 725), (118, 787), (123, 637), (124, 597),
                    (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 268; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            269 => {
                if let Some(next) = advance_map(&[
                    (33, 568), (34, 701), (37, 591), (38, 603), (39, 692), (40, 566), (42, 587), (43, 583),
                    (44, 507), (45, 573), (46, 666), (47, 589), (48, 674), (58, 426), (60, 618), (61, 433),
                    (62, 608), (63, 651), (70, 744), (76, 718), (82, 721), (84, 748), (85, 722), (91, 640),
                    (92, 161), (93, 646), (94, 601), (98, 790), (99, 769), (100, 786), (102, 753), (105, 783),
                    (109, 755), (110, 803), (112, 800), (115, 770), (116, 795), (117, 725), (118, 787), (123, 637),
                    (124, 597), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 269; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            270 => {
                if let Some(next) = advance_map(&[
                    (33, 568), (34, 701), (37, 591), (38, 603), (39, 692), (40, 566), (42, 587), (43, 583),
                    (44, 507), (45, 573), (46, 666), (47, 589), (48, 674), (58, 426), (60, 618), (61, 433),
                    (62, 611), (63, 651), (70, 744), (76, 718), (82, 721), (84, 748), (85, 722), (91, 640),
                    (92, 221), (94, 601), (98, 790), (99, 769), (100, 786), (102, 753), (105, 783), (109, 755),
                    (110, 803), (112, 800), (115, 770), (116, 795), (117, 725), (118, 787), (123, 637), (124, 597),
                    (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 270; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            271 => {
                if let Some(next) = advance_map(&[
                    (33, 568), (34, 390), (37, 592), (38, 604), (40, 408), (42, 588), (43, 584), (44, 507),
                    (45, 575), (47, 590), (58, 426), (60, 619), (61, 648), (62, 609), (91, 428), (92, 28),
                    (94, 599), (98, 790), (99, 769), (100, 786), (102, 782), (105, 783), (109, 755), (110, 803),
                    (112, 800), (115, 770), (117, 775), (118, 787), (124, 596), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 271; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            272 => {
                if let Some(next) = advance_map(&[
                    (33, 568), (34, 390), (37, 592), (38, 604), (40, 408), (42, 588), (43, 584), (44, 507),
                    (45, 575), (47, 590), (60, 619), (61, 648), (62, 609), (91, 444), (92, 124), (94, 599),
                    (124, 596), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 272; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            273 => {
                if let Some(next) = advance_map(&[
                    (33, 567), (34, 701), (35, 447), (38, 603), (39, 692), (40, 566), (42, 587), (43, 583),
                    (44, 507), (45, 574), (46, 476), (47, 410), (48, 674), (58, 426), (59, 629), (60, 430),
                    (62, 434), (70, 744), (76, 718), (82, 721), (84, 748), (85, 722), (91, 641), (92, 6),
                    (93, 445), (94, 446), (98, 790), (99, 769), (100, 786), (102, 753), (105, 783), (109, 755),
                    (110, 803), (112, 800), (115, 770), (116, 795), (117, 725), (118, 787), (123, 637), (124, 595),
                    (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 273; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            274 => {
                if let Some(next) = advance_map(&[
                    (33, 567), (34, 701), (35, 451), (37, 591), (38, 603), (39, 692), (40, 566), (41, 508),
                    (42, 587), (43, 583), (44, 507), (45, 574), (46, 668), (47, 589), (48, 674), (58, 426),
                    (59, 629), (60, 615), (61, 647), (62, 817), (70, 744), (76, 718), (82, 721), (84, 748),
                    (85, 722), (91, 641), (92, 4), (93, 646), (94, 446), (98, 790), (99, 769), (100, 786),
                    (102, 753), (105, 783), (109, 755), (110, 803), (112, 800), (115, 770), (116, 795), (117, 725),
                    (118, 787), (123, 637), (124, 470), (125, 638), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 274; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            275 => {
                if let Some(next) = advance_map(&[
                    (33, 567), (34, 701), (35, 455), (38, 602), (39, 692), (40, 566), (41, 508), (42, 587),
                    (43, 583), (45, 574), (46, 416), (47, 410), (48, 674), (58, 426), (59, 629), (62, 437),
                    (70, 744), (76, 718), (82, 721), (84, 748), (85, 722), (91, 641), (92, 10), (94, 446),
                    (98, 790), (99, 769), (100, 786), (102, 753), (105, 783), (109, 755), (110, 803), (112, 800),
                    (115, 770), (116, 795), (117, 725), (118, 787), (123, 637), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 275; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            276 => {
                if let Some(next) = advance_map(&[
                    (33, 567), (34, 701), (35, 449), (38, 603), (39, 692), (40, 566), (42, 587), (43, 583),
                    (44, 507), (45, 574), (46, 476), (47, 410), (48, 674), (58, 426), (59, 629), (62, 607),
                    (70, 744), (76, 718), (82, 721), (84, 748), (85, 722), (91, 641), (92, 8), (94, 446),
                    (98, 790), (99, 769), (100, 786), (102, 753), (105, 783), (109, 755), (110, 803), (112, 800),
                    (115, 770), (116, 795), (117, 725), (118, 787), (123, 637), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 276; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            277 => {
                if let Some(next) = advance_map(&[
                    (33, 567), (34, 701), (38, 603), (39, 692), (40, 566), (42, 587), (43, 583), (45, 574),
                    (46, 416), (47, 410), (48, 674), (58, 426), (70, 744), (76, 718), (82, 721), (84, 748),
                    (85, 722), (91, 640), (92, 12), (94, 446), (98, 790), (99, 769), (100, 786), (102, 753),
                    (105, 783), (109, 755), (110, 803), (112, 800), (115, 770), (116, 795), (117, 725), (118, 787),
                    (123, 637), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 277; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            278 => {
                if let Some(next) = advance_map(&[
                    (33, 567), (34, 701), (38, 602), (39, 692), (40, 566), (41, 508), (42, 587), (43, 583),
                    (44, 507), (45, 574), (46, 668), (47, 410), (48, 674), (58, 631), (59, 629), (60, 615),
                    (61, 647), (62, 817), (70, 744), (76, 718), (82, 721), (84, 748), (85, 722), (91, 640),
                    (92, 14), (93, 646), (94, 446), (98, 790), (99, 769), (100, 786), (102, 753), (105, 783),
                    (109, 755), (110, 803), (112, 800), (115, 770), (116, 795), (117, 725), (118, 787), (123, 637),
                    (125, 638), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 278; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            279 => {
                if let Some(next) = advance_map(&[
                    (33, 567), (34, 701), (38, 602), (39, 692), (40, 566), (42, 587), (43, 583), (45, 573),
                    (46, 476), (47, 410), (48, 674), (58, 426), (60, 615), (70, 744), (76, 718), (82, 721),
                    (84, 748), (85, 722), (91, 641), (92, 34), (94, 446), (98, 790), (99, 769), (100, 786),
                    (102, 753), (105, 783), (109, 755), (110, 803), (112, 800), (115, 770), (116, 795), (117, 725),
                    (118, 787), (123, 637), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 279; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            280 => {
                if let Some(next) = advance_map(&[
                    (33, 567), (39, 692), (40, 566), (41, 508), (43, 585), (45, 578), (46, 476), (47, 410),
                    (48, 674), (76, 736), (85, 737), (92, 139), (117, 738), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 280; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            281 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (34, 701), (35, 456), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588),
                    (43, 584), (44, 507), (45, 576), (46, 665), (47, 590), (58, 632), (59, 629), (60, 616),
                    (61, 648), (62, 609), (63, 651), (76, 719), (82, 721), (85, 723), (91, 639), (92, 50),
                    (93, 445), (94, 599), (117, 726), (123, 637), (124, 596), (125, 638),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 281; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            282 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (34, 701), (35, 456), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588),
                    (43, 584), (44, 507), (45, 576), (46, 665), (47, 590), (58, 633), (59, 629), (60, 616),
                    (61, 648), (62, 609), (63, 651), (76, 719), (82, 721), (85, 723), (91, 639), (92, 88),
                    (93, 445), (94, 599), (117, 726), (124, 596), (125, 638),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 282; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            283 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (34, 701), (35, 456), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588),
                    (43, 584), (44, 507), (45, 576), (46, 665), (47, 590), (58, 633), (59, 629), (60, 616),
                    (61, 648), (62, 609), (63, 651), (76, 831), (82, 832), (85, 833), (91, 639), (92, 88),
                    (93, 445), (94, 599), (117, 834), (124, 596), (125, 638),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 282; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 95 || 97 <= lookahead && lookahead <= 122 { state = 840; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            284 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (34, 701), (35, 456), (37, 591), (38, 603), (40, 566), (41, 508), (42, 587),
                    (43, 581), (44, 507), (45, 577), (46, 665), (47, 589), (58, 633), (59, 629), (60, 618),
                    (61, 433), (62, 608), (63, 651), (76, 719), (82, 721), (85, 723), (91, 639), (92, 183),
                    (93, 445), (94, 598), (117, 726), (124, 597), (125, 638),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 284; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            285 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (34, 701), (35, 456), (37, 591), (38, 603), (40, 566), (41, 508), (42, 587),
                    (43, 581), (44, 507), (45, 577), (46, 665), (47, 589), (58, 633), (59, 629), (60, 618),
                    (61, 433), (62, 608), (63, 651), (76, 831), (82, 832), (85, 833), (91, 639), (92, 183),
                    (93, 445), (94, 598), (117, 834), (124, 597), (125, 638),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 284; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 95 || 97 <= lookahead && lookahead <= 122 { state = 840; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            286 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (34, 701), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584),
                    (44, 507), (45, 575), (46, 665), (47, 590), (58, 426), (60, 616), (61, 648), (62, 609),
                    (63, 651), (76, 719), (82, 721), (85, 723), (91, 641), (92, 32), (94, 599), (117, 726),
                    (123, 637), (124, 596), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 286; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            287 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (34, 701), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584),
                    (44, 507), (45, 575), (46, 665), (47, 590), (58, 426), (60, 616), (61, 648), (62, 609),
                    (63, 651), (76, 719), (82, 721), (85, 723), (91, 643), (92, 54), (94, 599), (117, 726),
                    (123, 637), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 287; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            288 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (34, 701), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584),
                    (44, 507), (45, 575), (46, 665), (47, 590), (58, 426), (60, 616), (61, 648), (62, 609),
                    (63, 651), (76, 719), (82, 721), (85, 723), (91, 639), (92, 169), (94, 599), (117, 726),
                    (123, 637), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 288; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            289 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (34, 701), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584),
                    (44, 507), (45, 575), (46, 665), (47, 590), (60, 616), (61, 648), (62, 609), (63, 651),
                    (76, 719), (82, 721), (85, 723), (91, 639), (92, 100), (94, 599), (117, 726), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 289; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            290 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (34, 701), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584),
                    (44, 507), (45, 575), (46, 665), (47, 590), (60, 616), (61, 648), (62, 609), (63, 651),
                    (76, 831), (82, 832), (85, 833), (91, 639), (92, 100), (94, 599), (117, 834), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 289; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 840; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            291 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (34, 701), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584),
                    (44, 507), (45, 576), (46, 665), (47, 590), (58, 631), (59, 629), (60, 616), (61, 648),
                    (62, 609), (63, 651), (76, 719), (82, 721), (85, 723), (91, 641), (92, 30), (94, 599),
                    (117, 726), (123, 637), (124, 596), (125, 638), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 291; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            292 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (34, 701), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507),
                    (45, 576), (46, 665), (47, 590), (58, 426), (59, 629), (60, 616), (61, 648), (62, 609),
                    (63, 651), (76, 719), (82, 721), (85, 723), (91, 643), (92, 98), (94, 599), (117, 726),
                    (123, 637), (124, 596), (125, 638),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 292; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            293 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (34, 701), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507),
                    (45, 576), (46, 665), (47, 590), (58, 426), (60, 616), (61, 648), (62, 817), (63, 651),
                    (76, 719), (82, 721), (85, 723), (91, 639), (92, 58), (94, 599), (117, 726), (123, 637),
                    (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 293; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            294 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (34, 701), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507),
                    (45, 576), (46, 665), (47, 590), (58, 426), (60, 616), (61, 648), (62, 610), (63, 651),
                    (76, 719), (82, 721), (85, 723), (91, 639), (92, 171), (94, 599), (117, 726), (123, 637),
                    (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 294; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            295 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (34, 701), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507),
                    (45, 576), (46, 665), (47, 590), (58, 426), (60, 616), (61, 648), (62, 609), (63, 651),
                    (76, 719), (82, 721), (85, 723), (91, 639), (92, 60), (93, 646), (94, 599), (117, 726),
                    (123, 637), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 295; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            296 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (34, 701), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507),
                    (45, 576), (46, 665), (47, 590), (60, 616), (61, 648), (62, 817), (63, 651), (76, 719),
                    (82, 721), (85, 723), (91, 639), (92, 187), (94, 599), (117, 726), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 296; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            297 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (34, 701), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507),
                    (45, 576), (46, 665), (47, 590), (60, 616), (61, 648), (62, 817), (63, 651), (76, 831),
                    (82, 832), (85, 833), (91, 639), (92, 187), (94, 599), (117, 834), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 296; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 840; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            298 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (34, 701), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507),
                    (45, 576), (46, 665), (47, 590), (60, 616), (61, 648), (62, 610), (63, 651), (76, 719),
                    (82, 721), (85, 723), (91, 639), (92, 104), (94, 599), (117, 726), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 298; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            299 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (34, 701), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507),
                    (45, 576), (46, 665), (47, 590), (60, 616), (61, 648), (62, 610), (63, 651), (76, 831),
                    (82, 832), (85, 833), (91, 639), (92, 104), (94, 599), (117, 834), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 298; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 840; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            300 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (34, 701), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507),
                    (45, 576), (46, 665), (47, 590), (60, 616), (61, 648), (62, 609), (63, 651), (76, 719),
                    (82, 721), (85, 723), (91, 639), (92, 102), (93, 646), (94, 599), (117, 726), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 300; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            301 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (34, 701), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507),
                    (45, 576), (46, 665), (47, 590), (60, 616), (61, 648), (62, 609), (63, 651), (76, 831),
                    (82, 832), (85, 833), (91, 639), (92, 102), (93, 646), (94, 599), (117, 834), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 300; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 95 || 97 <= lookahead && lookahead <= 122 { state = 840; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            302 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (34, 701), (37, 591), (38, 603), (40, 566), (42, 587), (43, 581), (44, 507),
                    (45, 577), (46, 665), (47, 589), (60, 618), (61, 433), (62, 817), (63, 651), (76, 719),
                    (82, 721), (85, 723), (91, 639), (92, 233), (94, 598), (117, 726), (124, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 302; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            303 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (34, 701), (37, 591), (38, 603), (40, 566), (42, 587), (43, 581), (44, 507),
                    (45, 577), (46, 665), (47, 589), (60, 618), (61, 433), (62, 817), (63, 651), (76, 831),
                    (82, 832), (85, 833), (91, 639), (92, 233), (94, 598), (117, 834), (124, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 302; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 840; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            304 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (34, 701), (37, 591), (38, 603), (40, 566), (42, 587), (43, 581), (44, 507),
                    (45, 577), (46, 665), (47, 589), (60, 618), (61, 433), (62, 608), (63, 651), (76, 719),
                    (82, 721), (85, 723), (91, 639), (92, 185), (93, 646), (94, 598), (117, 726), (124, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 304; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            305 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (34, 701), (37, 591), (38, 603), (40, 566), (42, 587), (43, 581), (44, 507),
                    (45, 577), (46, 665), (47, 589), (60, 618), (61, 433), (62, 608), (63, 651), (76, 831),
                    (82, 832), (85, 833), (91, 639), (92, 185), (93, 646), (94, 598), (117, 834), (124, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 304; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 95 || 97 <= lookahead && lookahead <= 122 { state = 840; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            306 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (34, 701), (37, 591), (38, 603), (40, 566), (42, 587), (43, 581), (44, 507),
                    (45, 577), (46, 665), (47, 589), (60, 618), (61, 433), (62, 611), (63, 651), (76, 719),
                    (82, 721), (85, 723), (91, 639), (92, 213), (94, 598), (117, 726), (124, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 306; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            307 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (34, 701), (37, 591), (38, 603), (40, 566), (42, 587), (43, 581), (44, 507),
                    (45, 577), (46, 665), (47, 589), (60, 618), (61, 433), (62, 611), (63, 651), (76, 831),
                    (82, 832), (85, 833), (91, 639), (92, 213), (94, 598), (117, 834), (124, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 306; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 840; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            308 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (35, 467), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584),
                    (44, 507), (45, 575), (46, 665), (47, 590), (58, 631), (59, 629), (60, 616), (61, 648),
                    (62, 609), (63, 651), (91, 643), (92, 40), (94, 599), (123, 637), (124, 596), (125, 638),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 308; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            309 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (35, 456), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584),
                    (44, 507), (45, 576), (46, 665), (47, 590), (48, 818), (58, 631), (59, 629), (60, 616),
                    (61, 648), (62, 609), (63, 651), (91, 641), (92, 36), (93, 646), (94, 599), (123, 637),
                    (124, 596), (125, 638), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 309; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            310 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (35, 456), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584),
                    (44, 507), (45, 576), (46, 665), (47, 590), (58, 632), (59, 629), (60, 616), (61, 648),
                    (62, 609), (63, 651), (91, 640), (92, 108), (93, 445), (94, 599), (124, 596), (125, 638),
                    (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 310; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            311 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (35, 456), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584),
                    (44, 507), (45, 576), (46, 665), (47, 590), (58, 632), (59, 629), (60, 616), (61, 648),
                    (62, 609), (63, 651), (91, 639), (92, 237), (93, 445), (94, 599), (123, 637), (124, 596),
                    (125, 638),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 311; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            312 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (35, 456), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584),
                    (44, 507), (45, 576), (46, 665), (47, 590), (58, 631), (60, 616), (61, 648), (62, 609),
                    (63, 651), (91, 639), (92, 64), (93, 646), (94, 599), (123, 637), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 312; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            313 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (35, 456), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584),
                    (44, 507), (45, 576), (46, 665), (47, 590), (58, 633), (59, 629), (60, 616), (61, 648),
                    (62, 609), (63, 651), (91, 643), (92, 38), (93, 445), (94, 599), (123, 637), (124, 596),
                    (125, 638),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 313; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            314 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (35, 456), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584),
                    (44, 507), (45, 576), (46, 665), (47, 590), (58, 633), (59, 629), (60, 616), (61, 648),
                    (62, 609), (63, 651), (91, 643), (92, 163), (93, 646), (94, 599), (123, 637), (124, 596),
                    (125, 638),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 314; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            315 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (35, 456), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584),
                    (44, 507), (45, 576), (46, 665), (47, 590), (58, 633), (59, 629), (60, 616), (61, 648),
                    (62, 609), (63, 651), (91, 639), (92, 203), (93, 445), (94, 599), (123, 637), (124, 596),
                    (125, 638),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 315; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            316 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (35, 456), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584),
                    (44, 507), (45, 576), (46, 665), (47, 590), (58, 633), (59, 629), (60, 616), (61, 648),
                    (62, 609), (63, 651), (91, 639), (92, 52), (93, 445), (94, 599), (98, 790), (99, 769),
                    (100, 786), (102, 782), (105, 783), (109, 755), (110, 803), (112, 800), (115, 770), (117, 775),
                    (118, 787), (123, 637), (124, 596), (125, 638),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 316; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            317 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (35, 456), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584),
                    (44, 507), (45, 576), (46, 665), (47, 590), (58, 633), (59, 629), (60, 616), (61, 648),
                    (62, 609), (63, 651), (91, 639), (92, 189), (93, 445), (94, 599), (124, 596), (125, 638),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 318; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 95 || 97 <= lookahead && lookahead <= 122 { state = 840; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            318 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (35, 456), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584),
                    (44, 507), (45, 576), (46, 665), (47, 590), (58, 633), (59, 629), (60, 616), (61, 648),
                    (62, 609), (63, 651), (91, 639), (92, 189), (93, 445), (94, 599), (124, 596), (125, 638),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 318; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            319 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (35, 456), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584),
                    (44, 507), (45, 576), (46, 665), (47, 590), (58, 633), (59, 629), (60, 616), (61, 648),
                    (62, 609), (63, 651), (91, 645), (92, 106), (93, 445), (94, 599), (124, 596), (125, 638),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 319; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            320 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (35, 456), (37, 591), (38, 603), (40, 566), (41, 508), (42, 587), (43, 581),
                    (44, 507), (45, 577), (46, 665), (47, 589), (58, 632), (59, 629), (60, 618), (61, 433),
                    (62, 608), (63, 651), (91, 640), (92, 191), (93, 445), (94, 598), (124, 597), (125, 638),
                    (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 320; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            321 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (35, 456), (37, 591), (38, 603), (40, 566), (41, 508), (42, 587), (43, 581),
                    (44, 507), (45, 577), (46, 665), (47, 589), (58, 632), (59, 629), (60, 618), (61, 433),
                    (62, 608), (63, 651), (91, 639), (92, 249), (93, 646), (94, 598), (123, 637), (124, 597),
                    (125, 638),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 321; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            322 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (35, 456), (37, 591), (38, 603), (40, 566), (41, 508), (42, 587), (43, 581),
                    (44, 507), (45, 577), (46, 665), (47, 589), (58, 632), (59, 629), (60, 618), (61, 433),
                    (62, 608), (63, 651), (91, 639), (92, 255), (93, 445), (94, 598), (123, 637), (124, 597),
                    (125, 638),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 322; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            323 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (35, 456), (37, 591), (38, 603), (40, 566), (41, 508), (42, 587), (43, 581),
                    (44, 507), (45, 577), (46, 665), (47, 589), (58, 633), (59, 629), (60, 618), (61, 433),
                    (62, 608), (63, 651), (91, 643), (92, 223), (93, 445), (94, 598), (123, 637), (124, 597),
                    (125, 638),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 323; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            324 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (35, 456), (37, 591), (38, 603), (40, 566), (41, 508), (42, 587), (43, 581),
                    (44, 507), (45, 577), (46, 665), (47, 589), (58, 633), (59, 629), (60, 618), (61, 433),
                    (62, 608), (63, 651), (91, 639), (92, 243), (93, 445), (94, 598), (123, 637), (124, 597),
                    (125, 638),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 324; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            325 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (35, 456), (37, 591), (38, 603), (40, 566), (41, 508), (42, 587), (43, 581),
                    (44, 507), (45, 577), (46, 665), (47, 589), (58, 633), (59, 629), (60, 618), (61, 433),
                    (62, 608), (63, 651), (91, 639), (92, 167), (93, 445), (94, 598), (98, 790), (99, 769),
                    (100, 786), (102, 782), (105, 783), (109, 755), (110, 803), (112, 800), (115, 770), (117, 775),
                    (118, 787), (123, 637), (124, 597), (125, 638),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 325; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            326 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (35, 456), (37, 591), (38, 603), (40, 566), (41, 508), (42, 587), (43, 581),
                    (44, 507), (45, 577), (46, 665), (47, 589), (58, 633), (59, 629), (60, 618), (61, 433),
                    (62, 608), (63, 651), (91, 639), (92, 259), (93, 646), (94, 598), (123, 637), (124, 597),
                    (125, 638),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 326; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            327 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (35, 456), (37, 591), (38, 603), (40, 566), (41, 508), (42, 587), (43, 581),
                    (44, 507), (45, 577), (46, 665), (47, 589), (58, 633), (59, 629), (60, 618), (61, 433),
                    (62, 608), (63, 651), (91, 639), (92, 215), (93, 445), (94, 598), (124, 597), (125, 638),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 328; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 95 || 97 <= lookahead && lookahead <= 122 { state = 840; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            328 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (35, 456), (37, 591), (38, 603), (40, 566), (41, 508), (42, 587), (43, 581),
                    (44, 507), (45, 577), (46, 665), (47, 589), (58, 633), (59, 629), (60, 618), (61, 433),
                    (62, 608), (63, 651), (91, 639), (92, 215), (93, 445), (94, 598), (124, 597), (125, 638),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 328; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            329 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (35, 448), (37, 591), (38, 603), (40, 566), (41, 508), (42, 587), (43, 581),
                    (44, 507), (45, 577), (46, 665), (47, 589), (58, 426), (59, 629), (60, 618), (61, 433),
                    (62, 608), (63, 651), (91, 641), (92, 16), (93, 445), (94, 598), (98, 790), (99, 769),
                    (100, 786), (102, 782), (105, 783), (109, 755), (110, 803), (112, 800), (115, 770), (117, 775),
                    (118, 787), (123, 637), (124, 597), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 329; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            330 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (35, 450), (37, 591), (38, 603), (40, 566), (41, 508), (42, 587), (43, 580),
                    (44, 507), (45, 570), (47, 589), (58, 426), (59, 629), (60, 617), (61, 433), (62, 608),
                    (91, 641), (92, 26), (94, 598), (98, 790), (99, 769), (100, 786), (102, 782), (105, 783),
                    (109, 755), (110, 803), (112, 800), (115, 770), (117, 775), (118, 787), (124, 597), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 330; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            331 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584), (44, 507),
                    (45, 575), (46, 665), (47, 590), (58, 426), (60, 616), (61, 648), (62, 609), (63, 651),
                    (91, 641), (92, 42), (94, 599), (123, 637), (124, 596), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 331; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            332 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584), (44, 507),
                    (45, 575), (46, 665), (47, 590), (58, 426), (60, 616), (61, 648), (62, 609), (63, 651),
                    (91, 640), (92, 112), (94, 599), (124, 596), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 332; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            333 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584), (44, 507),
                    (45, 575), (46, 665), (47, 590), (58, 631), (59, 629), (60, 616), (61, 648), (62, 609),
                    (63, 651), (91, 639), (92, 56), (94, 599), (123, 637), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 333; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            334 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584), (44, 507),
                    (45, 575), (46, 665), (47, 590), (58, 630), (60, 616), (61, 648), (62, 609), (63, 651),
                    (91, 639), (92, 76), (94, 599), (123, 637), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 334; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            335 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584), (44, 507),
                    (45, 575), (46, 665), (47, 590), (60, 616), (61, 648), (62, 609), (63, 651), (91, 639),
                    (92, 66), (94, 599), (98, 790), (99, 769), (100, 786), (102, 782), (105, 783), (109, 755),
                    (110, 803), (112, 800), (115, 770), (117, 775), (118, 787), (123, 637), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 335; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            336 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584), (44, 507),
                    (45, 575), (46, 665), (47, 590), (60, 616), (61, 648), (62, 609), (63, 651), (91, 639),
                    (92, 197), (94, 599), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 337; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 840; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            337 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584), (44, 507),
                    (45, 575), (46, 665), (47, 590), (60, 616), (61, 648), (62, 609), (63, 651), (91, 639),
                    (92, 197), (94, 599), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 337; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            338 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584), (44, 507),
                    (45, 575), (46, 665), (47, 590), (60, 616), (61, 648), (62, 609), (63, 651), (91, 645),
                    (92, 219), (94, 599), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 338; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            339 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584), (44, 507),
                    (45, 575), (46, 665), (47, 590), (60, 616), (61, 648), (62, 609), (63, 651), (91, 644),
                    (92, 126), (94, 599), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 339; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            340 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584), (44, 507),
                    (45, 576), (46, 665), (47, 590), (58, 631), (59, 629), (60, 616), (61, 648), (62, 609),
                    (63, 651), (91, 643), (92, 110), (94, 599), (123, 637), (124, 596), (125, 638),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 340; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            341 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (41, 508), (42, 588), (43, 584), (44, 507),
                    (45, 576), (46, 665), (47, 590), (58, 630), (59, 629), (60, 616), (61, 648), (62, 817),
                    (63, 651), (91, 643), (92, 46), (93, 646), (94, 599), (123, 637), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 341; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            342 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507), (45, 576),
                    (46, 665), (47, 590), (58, 426), (60, 616), (61, 648), (62, 817), (63, 651), (91, 640),
                    (92, 193), (94, 599), (124, 596), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 342; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            343 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507), (45, 576),
                    (46, 665), (47, 590), (58, 426), (60, 616), (61, 648), (62, 610), (63, 651), (91, 640),
                    (92, 114), (94, 599), (124, 596), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 343; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            344 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507), (45, 576),
                    (46, 665), (47, 590), (58, 426), (60, 616), (61, 648), (62, 609), (63, 651), (91, 640),
                    (92, 116), (93, 646), (94, 599), (124, 596), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 344; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            345 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507), (45, 576),
                    (46, 665), (47, 590), (58, 631), (60, 616), (61, 648), (62, 817), (63, 651), (91, 639),
                    (92, 62), (94, 599), (123, 637), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 345; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            346 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507), (45, 576),
                    (46, 665), (47, 590), (58, 631), (60, 616), (61, 648), (62, 610), (63, 651), (91, 639),
                    (92, 173), (94, 599), (123, 637), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 346; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            347 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507), (45, 576),
                    (46, 665), (47, 590), (58, 630), (60, 616), (61, 648), (62, 817), (63, 651), (91, 639),
                    (92, 207), (94, 599), (123, 637), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 347; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            348 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507), (45, 576),
                    (46, 665), (47, 590), (58, 630), (60, 616), (61, 648), (62, 610), (63, 651), (91, 639),
                    (92, 227), (94, 599), (123, 637), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 348; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            349 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507), (45, 576),
                    (46, 665), (47, 590), (58, 630), (60, 616), (61, 648), (62, 609), (63, 651), (91, 639),
                    (92, 78), (93, 646), (94, 599), (123, 637), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 349; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            350 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507), (45, 576),
                    (46, 665), (47, 590), (59, 629), (60, 616), (61, 648), (62, 609), (63, 651), (91, 644),
                    (92, 122), (94, 599), (124, 596), (125, 638),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 350; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            351 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507), (45, 576),
                    (46, 665), (47, 590), (60, 616), (61, 648), (62, 817), (63, 651), (91, 639), (92, 72),
                    (94, 599), (98, 790), (99, 769), (100, 786), (102, 782), (105, 783), (109, 755), (110, 803),
                    (112, 800), (115, 770), (117, 775), (118, 787), (123, 637), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 351; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            352 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507), (45, 576),
                    (46, 665), (47, 590), (60, 616), (61, 648), (62, 817), (63, 651), (91, 639), (92, 245),
                    (94, 599), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 353; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 840; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            353 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507), (45, 576),
                    (46, 665), (47, 590), (60, 616), (61, 648), (62, 817), (63, 651), (91, 639), (92, 245),
                    (94, 599), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 353; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            354 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507), (45, 576),
                    (46, 665), (47, 590), (60, 616), (61, 648), (62, 817), (63, 651), (91, 645), (92, 239),
                    (94, 599), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 354; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            355 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507), (45, 576),
                    (46, 665), (47, 590), (60, 616), (61, 648), (62, 610), (63, 651), (91, 643), (92, 48),
                    (94, 599), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 355; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            356 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507), (45, 576),
                    (46, 665), (47, 590), (60, 616), (61, 648), (62, 610), (63, 651), (91, 639), (92, 177),
                    (94, 599), (98, 790), (99, 769), (100, 786), (102, 782), (105, 783), (109, 755), (110, 803),
                    (112, 800), (115, 770), (117, 775), (118, 787), (123, 637), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 356; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            357 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507), (45, 576),
                    (46, 665), (47, 590), (60, 616), (61, 648), (62, 610), (63, 651), (91, 639), (92, 205),
                    (94, 599), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 358; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 840; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            358 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507), (45, 576),
                    (46, 665), (47, 590), (60, 616), (61, 648), (62, 610), (63, 651), (91, 639), (92, 205),
                    (94, 599), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 358; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            359 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507), (45, 576),
                    (46, 665), (47, 590), (60, 616), (61, 648), (62, 610), (63, 651), (91, 645), (92, 225),
                    (94, 599), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 359; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            360 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507), (45, 576),
                    (46, 665), (47, 590), (60, 616), (61, 648), (62, 609), (63, 651), (91, 639), (92, 74),
                    (93, 646), (94, 599), (98, 790), (99, 769), (100, 786), (102, 782), (105, 783), (109, 755),
                    (110, 803), (112, 800), (115, 770), (117, 775), (118, 787), (123, 637), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 360; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            361 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507), (45, 576),
                    (46, 665), (47, 590), (60, 616), (61, 648), (62, 609), (63, 651), (91, 639), (92, 175),
                    (93, 646), (94, 599), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 362; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 95 || 97 <= lookahead && lookahead <= 122 { state = 840; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            362 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507), (45, 576),
                    (46, 665), (47, 590), (60, 616), (61, 648), (62, 609), (63, 651), (91, 639), (92, 175),
                    (93, 646), (94, 599), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 362; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            363 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (40, 566), (42, 588), (43, 584), (44, 507), (45, 576),
                    (46, 665), (47, 590), (60, 616), (61, 648), (62, 609), (63, 651), (91, 645), (92, 209),
                    (93, 646), (94, 599), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 363; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            364 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 592), (38, 604), (42, 588), (43, 586), (44, 507), (45, 579), (46, 409),
                    (47, 590), (60, 620), (61, 648), (62, 609), (92, 130), (94, 599), (124, 596),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 364; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            365 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 591), (38, 603), (40, 566), (41, 508), (42, 587), (43, 581), (44, 507),
                    (45, 577), (46, 665), (47, 589), (58, 632), (59, 629), (60, 618), (61, 433), (62, 608),
                    (63, 651), (91, 639), (92, 82), (93, 445), (94, 598), (98, 790), (99, 769), (100, 786),
                    (102, 782), (105, 783), (109, 755), (110, 803), (112, 800), (115, 770), (117, 775), (118, 787),
                    (123, 637), (124, 597), (125, 638),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 365; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            366 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 591), (38, 603), (40, 566), (41, 508), (42, 587), (43, 581), (44, 507),
                    (45, 577), (46, 665), (47, 589), (58, 631), (59, 629), (60, 618), (61, 648), (62, 608),
                    (63, 651), (91, 641), (92, 44), (93, 646), (94, 598), (123, 637), (124, 597), (125, 638),
                    (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 366; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            367 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 591), (38, 603), (40, 566), (41, 508), (42, 587), (43, 581), (44, 507),
                    (45, 577), (46, 665), (47, 589), (58, 631), (60, 618), (61, 433), (62, 817), (63, 651),
                    (91, 639), (92, 181), (94, 598), (123, 637), (124, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 367; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            368 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 591), (38, 603), (40, 566), (42, 587), (43, 581), (44, 507), (45, 577),
                    (46, 665), (47, 589), (58, 426), (59, 629), (60, 618), (61, 433), (62, 817), (63, 651),
                    (91, 643), (92, 68), (94, 598), (124, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 368; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            369 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 591), (38, 603), (40, 566), (42, 587), (43, 581), (44, 507), (45, 577),
                    (46, 665), (47, 589), (58, 426), (60, 618), (61, 433), (62, 817), (63, 651), (91, 640),
                    (92, 217), (94, 598), (124, 597), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 369; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            370 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 591), (38, 603), (40, 566), (42, 587), (43, 581), (44, 507), (45, 577),
                    (46, 665), (47, 589), (58, 426), (60, 618), (61, 433), (62, 608), (63, 651), (91, 640),
                    (92, 195), (93, 646), (94, 598), (124, 597), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 370; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            371 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 591), (38, 603), (40, 566), (42, 587), (43, 581), (44, 507), (45, 577),
                    (46, 665), (47, 589), (58, 426), (60, 618), (61, 433), (62, 611), (63, 651), (91, 640),
                    (92, 235), (94, 598), (124, 597), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 371; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            372 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 591), (38, 603), (40, 566), (42, 587), (43, 581), (44, 507), (45, 577),
                    (46, 665), (47, 589), (58, 631), (60, 618), (61, 433), (62, 611), (63, 651), (91, 639),
                    (92, 241), (94, 598), (123, 637), (124, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 372; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            373 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 591), (38, 603), (40, 566), (42, 587), (43, 581), (44, 507), (45, 577),
                    (46, 665), (47, 589), (58, 630), (60, 618), (61, 433), (62, 817), (63, 651), (91, 639),
                    (92, 253), (94, 598), (123, 637), (124, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 373; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            374 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 591), (38, 603), (40, 566), (42, 587), (43, 581), (44, 507), (45, 577),
                    (46, 665), (47, 589), (58, 630), (60, 618), (61, 433), (62, 611), (63, 651), (91, 639),
                    (92, 247), (94, 598), (123, 637), (124, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 374; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            375 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 591), (38, 603), (40, 566), (42, 587), (43, 581), (44, 507), (45, 577),
                    (46, 665), (47, 589), (60, 618), (61, 433), (62, 817), (63, 651), (91, 639), (92, 231),
                    (94, 598), (98, 790), (99, 769), (100, 786), (102, 782), (105, 783), (109, 755), (110, 803),
                    (112, 800), (115, 770), (117, 775), (118, 787), (123, 637), (124, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 375; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            376 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 591), (38, 603), (40, 566), (42, 587), (43, 581), (44, 507), (45, 577),
                    (46, 665), (47, 589), (60, 618), (61, 433), (62, 817), (63, 651), (91, 639), (92, 257),
                    (94, 598), (124, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 377; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 840; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            377 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 591), (38, 603), (40, 566), (42, 587), (43, 581), (44, 507), (45, 577),
                    (46, 665), (47, 589), (60, 618), (61, 433), (62, 817), (63, 651), (91, 639), (92, 257),
                    (94, 598), (124, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 377; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            378 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 591), (38, 603), (40, 566), (42, 587), (43, 581), (44, 507), (45, 577),
                    (46, 665), (47, 589), (60, 618), (61, 433), (62, 608), (63, 651), (91, 643), (92, 70),
                    (93, 646), (94, 598), (124, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 378; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            379 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 591), (38, 603), (40, 566), (42, 587), (43, 581), (44, 507), (45, 577),
                    (46, 665), (47, 589), (60, 618), (61, 433), (62, 608), (63, 651), (91, 639), (92, 179),
                    (93, 646), (94, 598), (98, 790), (99, 769), (100, 786), (102, 782), (105, 783), (109, 755),
                    (110, 803), (112, 800), (115, 770), (117, 775), (118, 787), (123, 637), (124, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 379; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            380 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 591), (38, 603), (40, 566), (42, 587), (43, 581), (44, 507), (45, 577),
                    (46, 665), (47, 589), (60, 618), (61, 433), (62, 608), (63, 651), (91, 639), (92, 229),
                    (93, 646), (94, 598), (124, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 381; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 95 || 97 <= lookahead && lookahead <= 122 { state = 840; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            381 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 591), (38, 603), (40, 566), (42, 587), (43, 581), (44, 507), (45, 577),
                    (46, 665), (47, 589), (60, 618), (61, 433), (62, 608), (63, 651), (91, 639), (92, 229),
                    (93, 646), (94, 598), (124, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 381; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            382 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 591), (38, 603), (40, 566), (42, 587), (43, 581), (44, 507), (45, 577),
                    (46, 665), (47, 589), (60, 618), (61, 433), (62, 611), (63, 651), (91, 643), (92, 165),
                    (94, 598), (124, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 382; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            383 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 591), (38, 603), (40, 566), (42, 587), (43, 581), (44, 507), (45, 577),
                    (46, 665), (47, 589), (60, 618), (61, 433), (62, 611), (63, 651), (91, 639), (92, 211),
                    (94, 598), (98, 790), (99, 769), (100, 786), (102, 782), (105, 783), (109, 755), (110, 803),
                    (112, 800), (115, 770), (117, 775), (118, 787), (123, 637), (124, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 383; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            384 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 591), (38, 603), (40, 566), (42, 587), (43, 581), (44, 507), (45, 577),
                    (46, 665), (47, 589), (60, 618), (61, 433), (62, 611), (63, 651), (91, 639), (92, 251),
                    (94, 598), (124, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 385; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 840; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            385 => {
                if let Some(next) = advance_map(&[
                    (33, 432), (37, 591), (38, 603), (40, 566), (42, 587), (43, 581), (44, 507), (45, 577),
                    (46, 665), (47, 589), (60, 618), (61, 433), (62, 611), (63, 651), (91, 639), (92, 251),
                    (94, 598), (124, 597),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 385; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            386 => {
                if let Some(next) = advance_map(&[
                    (34, 701), (38, 603), (40, 566), (42, 587), (47, 410), (58, 426), (76, 720), (85, 724),
                    (91, 641), (92, 92), (98, 790), (99, 769), (100, 786), (102, 782), (105, 783), (109, 755),
                    (110, 803), (112, 800), (115, 770), (117, 727), (118, 787), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 386; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            387 => {
                if let Some(next) = advance_map(&[
                    (34, 701), (41, 508), (44, 507), (47, 410), (58, 630), (76, 719), (82, 721), (85, 723),
                    (92, 143), (117, 726),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 387; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            388 => {
                if let Some(next) = advance_map(&[
                    (34, 701), (47, 410), (58, 630), (60, 439), (76, 720), (85, 724), (92, 145), (117, 728),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 388; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            389 => {
                if lookahead == 34 { state = 701; lexer.advance(false); continue; }
                if lookahead == 47 { state = 410; lexer.advance(false); continue; }
                if lookahead == 92 { state = 147; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 389; lexer.advance(true); continue; }
                return result;
            }
            390 => {
                if lookahead == 34 { state = 830; lexer.advance(false); continue; }
                return result;
            }
            391 => {
                if let Some(next) = advance_map(&[
                    (35, 452), (38, 603), (40, 566), (41, 508), (42, 587), (43, 580), (44, 507), (45, 570),
                    (46, 415), (47, 410), (58, 631), (59, 629), (60, 615), (61, 647), (62, 817), (91, 641),
                    (92, 24), (98, 790), (99, 769), (100, 786), (102, 782), (105, 783), (109, 755), (110, 803),
                    (112, 800), (115, 770), (117, 775), (118, 787), (123, 637), (124, 470), (125, 638), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 391; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            392 => {
                if let Some(next) = advance_map(&[
                    (35, 456), (38, 603), (40, 566), (41, 508), (42, 587), (44, 507), (46, 415), (47, 410),
                    (58, 631), (59, 629), (60, 615), (61, 647), (62, 817), (91, 640), (92, 80), (123, 637),
                    (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 392; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            393 => {
                if let Some(next) = advance_map(&[
                    (38, 603), (40, 566), (41, 508), (42, 587), (44, 507), (45, 438), (46, 415), (47, 410),
                    (58, 631), (59, 629), (60, 615), (61, 647), (62, 817), (91, 641), (92, 84), (123, 637),
                    (124, 470), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 393; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            394 => {
                if let Some(next) = advance_map(&[
                    (38, 603), (40, 566), (41, 508), (42, 587), (44, 507), (45, 438), (46, 415), (47, 410),
                    (58, 631), (59, 629), (60, 615), (61, 647), (62, 817), (91, 643), (92, 86), (98, 790),
                    (99, 769), (100, 786), (102, 782), (105, 783), (109, 755), (110, 803), (112, 800), (115, 770),
                    (117, 775), (118, 787), (123, 637), (124, 470),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 394; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            395 => {
                if let Some(next) = advance_map(&[
                    (38, 603), (40, 566), (41, 508), (42, 587), (44, 507), (45, 438), (46, 415), (47, 410),
                    (58, 631), (59, 629), (60, 615), (61, 647), (62, 817), (91, 639), (92, 120), (123, 637),
                    (124, 470),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 395; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            396 => {
                if let Some(next) = advance_map(&[
                    (38, 603), (40, 566), (41, 508), (42, 587), (44, 507), (45, 438), (46, 415), (47, 410),
                    (58, 630), (59, 629), (60, 615), (61, 647), (62, 817), (91, 643), (92, 96), (93, 646),
                    (123, 637), (124, 470),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 396; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            397 => {
                if let Some(next) = advance_map(&[
                    (38, 603), (40, 566), (41, 508), (42, 587), (44, 507), (45, 438), (46, 415), (47, 410),
                    (58, 630), (59, 629), (61, 647), (62, 817), (91, 643), (92, 90), (98, 790), (99, 769),
                    (100, 786), (102, 782), (105, 783), (109, 755), (110, 803), (112, 800), (115, 770), (117, 775),
                    (118, 787), (123, 637),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 397; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            398 => {
                if let Some(next) = advance_map(&[
                    (38, 603), (40, 566), (41, 508), (42, 587), (44, 507), (45, 438), (47, 410), (58, 631),
                    (59, 629), (60, 615), (61, 647), (62, 817), (91, 643), (92, 94), (123, 637), (124, 470),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 398; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            399 => {
                if let Some(next) = advance_map(&[
                    (38, 603), (40, 566), (41, 508), (42, 587), (44, 507), (46, 667), (47, 410), (58, 630),
                    (59, 629), (61, 647), (62, 817), (91, 639), (92, 118), (98, 790), (99, 769), (100, 786),
                    (102, 782), (105, 783), (109, 755), (110, 803), (112, 800), (115, 770), (117, 775), (118, 787),
                    (123, 637),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 399; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            400 => {
                if let Some(next) = advance_map(&[
                    (38, 603), (40, 566), (41, 508), (42, 587), (44, 507), (46, 415), (47, 410), (58, 630),
                    (59, 629), (61, 647), (62, 817), (91, 639), (92, 128), (123, 637), (124, 470),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 400; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            401 => {
                if let Some(next) = advance_map(&[
                    (38, 602), (42, 587), (46, 415), (47, 410), (58, 426), (61, 647), (91, 427), (92, 134),
                    (93, 646),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 401; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            402 => {
                if lookahead == 39 { state = 692; lexer.advance(false); continue; }
                if lookahead == 47 { state = 410; lexer.advance(false); continue; }
                if lookahead == 92 { state = 147; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 402; lexer.advance(true); continue; }
                return result;
            }
            403 => {
                if lookahead == 39 { state = 481; lexer.advance(false); continue; }
                if lookahead == 80 || lookahead == 112 { state = 471; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 403; lexer.advance(false); continue; }
                return result;
            }
            404 => {
                if lookahead == 39 { state = 477; lexer.advance(false); continue; }
                if lookahead == 46 { state = 681; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 471; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            405 => {
                if let Some(next) = advance_map(&[
                    (40, 566), (41, 508), (44, 507), (47, 410), (58, 630), (59, 629), (60, 615), (61, 647),
                    (62, 817), (91, 644), (92, 141), (93, 646), (123, 637),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 405; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            406 => {
                if let Some(next) = advance_map(&[
                    (40, 566), (47, 410), (58, 426), (70, 744), (84, 748), (91, 640), (92, 132), (102, 754),
                    (116, 795), (123, 637),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 406; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            407 => {
                if lookahead == 40 { state = 566; lexer.advance(false); continue; }
                if lookahead == 47 { state = 410; lexer.advance(false); continue; }
                if lookahead == 92 { state = 155; lexer.advance(true); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 407; lexer.advance(true); continue; }
                return result;
            }
            408 => {
                if lookahead == 41 { state = 828; lexer.advance(false); continue; }
                return result;
            }
            409 => {
                if lookahead == 42 { state = 669; lexer.advance(false); continue; }
                return result;
            }
            410 => {
                if lookahead == 42 { state = 413; lexer.advance(false); continue; }
                if lookahead == 47 { state = 815; lexer.advance(false); continue; }
                return result;
            }
            411 => {
                if lookahead == 42 { state = 824; lexer.advance(false); continue; }
                return result;
            }
            412 => {
                if lookahead == 42 { state = 412; lexer.advance(false); continue; }
                if lookahead == 47 { state = 808; lexer.advance(false); continue; }
                if lookahead != 0 { state = 413; lexer.advance(false); continue; }
                return result;
            }
            413 => {
                if lookahead == 42 { state = 412; lexer.advance(false); continue; }
                if lookahead != 0 { state = 413; lexer.advance(false); continue; }
                return result;
            }
            414 => {
                if lookahead == 42 { state = 412; lexer.advance(false); continue; }
                if lookahead != 0 { state = 524; lexer.advance(false); continue; }
                return result;
            }
            415 => {
                if lookahead == 46 { state = 417; lexer.advance(false); continue; }
                return result;
            }
            416 => {
                if lookahead == 46 { state = 417; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 673; lexer.advance(false); continue; }
                return result;
            }
            417 => {
                if lookahead == 46 { state = 506; lexer.advance(false); continue; }
                return result;
            }
            418 => {
                if lookahead == 46 { state = 481; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 679; lexer.advance(false); continue; }
                return result;
            }
            419 => {
                if lookahead == 47 { state = 531; lexer.advance(false); continue; }
                if lookahead == 92 { state = 526; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 419; lexer.advance(true); continue; }
                if lookahead != 0 { state = 533; lexer.advance(false); continue; }
                return result;
            }
            420 => {
                if lookahead == 49 { state = 424; lexer.advance(false); continue; }
                return result;
            }
            421 => {
                if lookahead == 50 { state = 672; lexer.advance(false); continue; }
                return result;
            }
            422 => {
                if lookahead == 50 { state = 425; lexer.advance(false); continue; }
                if lookahead == 54 { state = 672; lexer.advance(false); continue; }
                return result;
            }
            423 => {
                if lookahead == 52 { state = 672; lexer.advance(false); continue; }
                return result;
            }
            424 => {
                if lookahead == 54 { state = 672; lexer.advance(false); continue; }
                return result;
            }
            425 => {
                if lookahead == 56 { state = 672; lexer.advance(false); continue; }
                return result;
            }
            426 => {
                if lookahead == 58 { state = 634; lexer.advance(false); continue; }
                return result;
            }
            427 => {
                if lookahead == 58 { state = 826; lexer.advance(false); continue; }
                return result;
            }
            428 => {
                if lookahead == 58 { state = 826; lexer.advance(false); continue; }
                if lookahead == 91 { state = 635; lexer.advance(false); continue; }
                if lookahead == 93 { state = 829; lexer.advance(false); continue; }
                return result;
            }
            429 => {
                if lookahead == 60 { state = 841; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 429; lexer.advance(false); continue; }
                return result;
            }
            430 => {
                if lookahead == 60 { state = 621; lexer.advance(false); continue; }
                if lookahead == 61 { state = 613; lexer.advance(false); continue; }
                return result;
            }
            431 => {
                if lookahead == 60 { state = 429; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 431; lexer.advance(false); continue; }
                return result;
            }
            432 => {
                if lookahead == 61 { state = 606; lexer.advance(false); continue; }
                return result;
            }
            433 => {
                if lookahead == 61 { state = 605; lexer.advance(false); continue; }
                return result;
            }
            434 => {
                if lookahead == 61 { state = 612; lexer.advance(false); continue; }
                if lookahead == 62 { state = 435; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 441; lexer.advance(false); continue; }
                return result;
            }
            435 => {
                if lookahead == 61 { state = 658; lexer.advance(false); continue; }
                if lookahead == 62 { state = 842; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 436; lexer.advance(false); continue; }
                return result;
            }
            436 => {
                if lookahead == 62 { state = 842; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 436; lexer.advance(false); continue; }
                return result;
            }
            437 => {
                if lookahead == 62 { state = 625; lexer.advance(false); continue; }
                return result;
            }
            438 => {
                if lookahead == 62 { state = 670; lexer.advance(false); continue; }
                return result;
            }
            439 => {
                if lookahead == 62 { state = 714; lexer.advance(false); continue; }
                if lookahead == 92 { state = 440; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 439; lexer.advance(false); continue; }
                return result;
            }
            440 => {
                if lookahead == 62 { state = 715; lexer.advance(false); continue; }
                if lookahead == 92 { state = 440; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 439; lexer.advance(false); continue; }
                return result;
            }
            441 => {
                if lookahead == 62 { state = 436; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 441; lexer.advance(false); continue; }
                return result;
            }
            442 => {
                if lookahead == 70 { state = 420; lexer.advance(false); continue; }
                return result;
            }
            443 => {
                if lookahead == 85 { state = 496; lexer.advance(false); continue; }
                if lookahead == 117 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            444 => {
                if lookahead == 93 { state = 829; lexer.advance(false); continue; }
                return result;
            }
            445 => {
                if lookahead == 93 { state = 636; lexer.advance(false); continue; }
                return result;
            }
            446 => {
                if lookahead == 94 { state = 825; lexer.advance(false); continue; }
                return result;
            }
            447 => {
                if lookahead == 100 { state = 540; lexer.advance(false); continue; }
                if lookahead == 101 { state = 560; lexer.advance(false); continue; }
                if lookahead == 105 { state = 548; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 447; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            448 => {
                if lookahead == 100 { state = 540; lexer.advance(false); continue; }
                if lookahead == 101 { state = 560; lexer.advance(false); continue; }
                if lookahead == 105 { state = 549; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 448; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            449 => {
                if lookahead == 100 { state = 540; lexer.advance(false); continue; }
                if lookahead == 101 { state = 562; lexer.advance(false); continue; }
                if lookahead == 105 { state = 548; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 449; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            450 => {
                if lookahead == 100 { state = 540; lexer.advance(false); continue; }
                if lookahead == 101 { state = 562; lexer.advance(false); continue; }
                if lookahead == 105 { state = 549; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 450; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            451 => {
                if lookahead == 100 { state = 540; lexer.advance(false); continue; }
                if lookahead == 105 { state = 548; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 451; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            452 => {
                if lookahead == 100 { state = 540; lexer.advance(false); continue; }
                if lookahead == 105 { state = 549; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 452; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            453 => {
                if lookahead == 100 { state = 465; lexer.advance(false); continue; }
                return result;
            }
            454 => {
                if lookahead == 100 { state = 459; lexer.advance(false); continue; }
                return result;
            }
            455 => {
                if lookahead == 101 { state = 469; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 455; lexer.advance(false); continue; }
                return result;
            }
            456 => {
                if lookahead == 101 { state = 468; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 456; lexer.advance(false); continue; }
                return result;
            }
            457 => {
                if lookahead == 101 { state = 515; lexer.advance(false); continue; }
                return result;
            }
            458 => {
                if lookahead == 101 { state = 463; lexer.advance(false); continue; }
                return result;
            }
            459 => {
                if lookahead == 101 { state = 464; lexer.advance(false); continue; }
                return result;
            }
            460 => {
                if lookahead == 102 { state = 420; lexer.advance(false); continue; }
                return result;
            }
            461 => {
                if lookahead == 102 { state = 511; lexer.advance(false); continue; }
                return result;
            }
            462 => {
                if lookahead == 102 { state = 517; lexer.advance(false); continue; }
                return result;
            }
            463 => {
                if lookahead == 102 { state = 519; lexer.advance(false); continue; }
                return result;
            }
            464 => {
                if lookahead == 102 { state = 521; lexer.advance(false); continue; }
                return result;
            }
            465 => {
                if lookahead == 105 { state = 461; lexer.advance(false); continue; }
                return result;
            }
            466 => {
                if lookahead == 105 { state = 462; lexer.advance(false); continue; }
                if lookahead == 115 { state = 457; lexer.advance(false); continue; }
                return result;
            }
            467 => {
                if lookahead == 105 { state = 549; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 467; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            468 => {
                if lookahead == 108 { state = 466; lexer.advance(false); continue; }
                if lookahead == 110 { state = 453; lexer.advance(false); continue; }
                return result;
            }
            469 => {
                if lookahead == 110 { state = 453; lexer.advance(false); continue; }
                return result;
            }
            470 => {
                if lookahead == 124 { state = 593; lexer.advance(false); continue; }
                return result;
            }
            471 => {
                if lookahead == 43 || lookahead == 45 { state = 478; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 678; lexer.advance(false); continue; }
                return result;
            }
            472 => {
                if lookahead == 80 || lookahead == 112 { state = 471; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 403; lexer.advance(false); continue; }
                return result;
            }
            473 => {
                if lookahead == 48 || lookahead == 49 { state = 676; lexer.advance(false); continue; }
                return result;
            }
            474 => {
                if lookahead == 56 || lookahead == 57 { state = 404; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 55 { state = 675; lexer.advance(false); continue; }
                return result;
            }
            475 => {
                if 48 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                return result;
            }
            476 => {
                if 48 <= lookahead && lookahead <= 57 { state = 673; lexer.advance(false); continue; }
                return result;
            }
            477 => {
                if 48 <= lookahead && lookahead <= 57 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            478 => {
                if 48 <= lookahead && lookahead <= 57 { state = 678; lexer.advance(false); continue; }
                return result;
            }
            479 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 807; lexer.advance(false); continue; }
                return result;
            }
            480 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 679; lexer.advance(false); continue; }
                return result;
            }
            481 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 403; lexer.advance(false); continue; }
                return result;
            }
            482 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 707; lexer.advance(false); continue; }
                return result;
            }
            483 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 713; lexer.advance(false); continue; }
                return result;
            }
            484 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 479; lexer.advance(false); continue; }
                return result;
            }
            485 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 482; lexer.advance(false); continue; }
                return result;
            }
            486 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 484; lexer.advance(false); continue; }
                return result;
            }
            487 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 485; lexer.advance(false); continue; }
                return result;
            }
            488 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 486; lexer.advance(false); continue; }
                return result;
            }
            489 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 487; lexer.advance(false); continue; }
                return result;
            }
            490 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            491 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 489; lexer.advance(false); continue; }
                return result;
            }
            492 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 490; lexer.advance(false); continue; }
                return result;
            }
            493 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 491; lexer.advance(false); continue; }
                return result;
            }
            494 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 492; lexer.advance(false); continue; }
                return result;
            }
            495 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 493; lexer.advance(false); continue; }
                return result;
            }
            496 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 494; lexer.advance(false); continue; }
                return result;
            }
            497 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 495; lexer.advance(false); continue; }
                return result;
            }
            498 => {
                if lookahead != 0 && lookahead != 42 { state = 533; lexer.advance(false); continue; }
                return result;
            }
            499 => {
                if eof { state = 501; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 568), (34, 701), (35, 447), (37, 592), (38, 604), (39, 692), (40, 566), (41, 508),
                    (42, 588), (43, 582), (44, 507), (45, 571), (46, 666), (47, 590), (48, 674), (58, 632),
                    (59, 629), (60, 616), (61, 648), (62, 817), (63, 651), (70, 744), (76, 718), (82, 721),
                    (84, 748), (85, 722), (91, 642), (92, 2), (93, 646), (94, 600), (98, 790), (99, 769),
                    (100, 786), (102, 753), (105, 783), (109, 755), (110, 803), (112, 800), (115, 770), (116, 795),
                    (117, 725), (118, 787), (123, 637), (124, 596), (125, 638), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 499; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            500 => {
                if eof { state = 501; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 567), (34, 701), (35, 451), (37, 591), (38, 603), (39, 692), (40, 566), (41, 508),
                    (42, 587), (43, 583), (44, 507), (45, 574), (46, 668), (47, 589), (48, 674), (58, 426),
                    (59, 629), (60, 615), (61, 647), (62, 817), (70, 744), (76, 718), (82, 721), (84, 748),
                    (85, 722), (91, 641), (92, 4), (93, 646), (94, 446), (98, 790), (99, 769), (100, 786),
                    (102, 753), (105, 783), (109, 755), (110, 803), (112, 800), (115, 770), (116, 795), (117, 725),
                    (118, 787), (123, 637), (124, 470), (125, 638), (126, 569),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 500; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            501 => {
                result = true; lexer.set_result_symbol(ts_builtin_sym_end); lexer.mark_end();
                return result;
            }
            502 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_include_token1); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            503 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_include_token2); lexer.mark_end();
                return result;
            }
            504 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_def_token1); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            505 => {
                result = true; lexer.set_result_symbol(anon_sym_LPAREN); lexer.mark_end();
                return result;
            }
            506 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT_DOT_DOT); lexer.mark_end();
                return result;
            }
            507 => {
                result = true; lexer.set_result_symbol(anon_sym_COMMA); lexer.mark_end();
                return result;
            }
            508 => {
                result = true; lexer.set_result_symbol(anon_sym_RPAREN); lexer.mark_end();
                return result;
            }
            509 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_if_token1); lexer.mark_end();
                if lookahead == 100 { state = 544; lexer.advance(false); continue; }
                if lookahead == 110 { state = 538; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            510 => {
                result = true; lexer.set_result_symbol(anon_sym_LF); lexer.mark_end();
                if lookahead == 10 { state = 510; lexer.advance(false); continue; }
                return result;
            }
            511 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_if_token2); lexer.mark_end();
                return result;
            }
            512 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_if_token2); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            513 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_ifdef_token1); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            514 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_ifdef_token2); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            515 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_else_token1); lexer.mark_end();
                return result;
            }
            516 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_else_token1); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            517 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_elif_token1); lexer.mark_end();
                if lookahead == 100 { state = 458; lexer.advance(false); continue; }
                if lookahead == 110 { state = 454; lexer.advance(false); continue; }
                return result;
            }
            518 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_elif_token1); lexer.mark_end();
                if lookahead == 100 { state = 546; lexer.advance(false); continue; }
                if lookahead == 110 { state = 539; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            519 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_elifdef_token1); lexer.mark_end();
                return result;
            }
            520 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_elifdef_token1); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            521 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_elifdef_token2); lexer.mark_end();
                return result;
            }
            522 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_elifdef_token2); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            523 => {
                result = true; lexer.set_result_symbol(sym_preproc_arg); lexer.mark_end();
                if lookahead == 10 { state = 413; lexer.advance(false); continue; }
                if lookahead == 42 { state = 523; lexer.advance(false); continue; }
                if lookahead == 47 { state = 808; lexer.advance(false); continue; }
                if lookahead == 92 { state = 529; lexer.advance(false); continue; }
                if lookahead != 0 { state = 524; lexer.advance(false); continue; }
                return result;
            }
            524 => {
                result = true; lexer.set_result_symbol(sym_preproc_arg); lexer.mark_end();
                if lookahead == 10 { state = 413; lexer.advance(false); continue; }
                if lookahead == 42 { state = 523; lexer.advance(false); continue; }
                if lookahead == 47 { state = 414; lexer.advance(false); continue; }
                if lookahead == 92 { state = 529; lexer.advance(false); continue; }
                if lookahead != 0 { state = 524; lexer.advance(false); continue; }
                return result;
            }
            525 => {
                result = true; lexer.set_result_symbol(sym_preproc_arg); lexer.mark_end();
                if lookahead == 10 { state = 815; lexer.advance(false); continue; }
                if lookahead == 13 { state = 809; lexer.advance(false); continue; }
                if lookahead == 47 { state = 812; lexer.advance(false); continue; }
                if lookahead == 92 { state = 811; lexer.advance(false); continue; }
                if lookahead != 0 { state = 813; lexer.advance(false); continue; }
                return result;
            }
            526 => {
                result = true; lexer.set_result_symbol(sym_preproc_arg); lexer.mark_end();
                if lookahead == 10 { state = 419; lexer.advance(true); continue; }
                if lookahead == 13 { state = 527; lexer.advance(false); continue; }
                if lookahead == 47 { state = 498; lexer.advance(false); continue; }
                if lookahead == 92 { state = 528; lexer.advance(false); continue; }
                if lookahead != 0 { state = 533; lexer.advance(false); continue; }
                return result;
            }
            527 => {
                result = true; lexer.set_result_symbol(sym_preproc_arg); lexer.mark_end();
                if lookahead == 10 { state = 419; lexer.advance(true); continue; }
                if lookahead == 47 { state = 498; lexer.advance(false); continue; }
                if lookahead == 92 { state = 528; lexer.advance(false); continue; }
                if lookahead != 0 { state = 533; lexer.advance(false); continue; }
                return result;
            }
            528 => {
                result = true; lexer.set_result_symbol(sym_preproc_arg); lexer.mark_end();
                if lookahead == 13 { state = 534; lexer.advance(false); continue; }
                if lookahead == 47 { state = 498; lexer.advance(false); continue; }
                if lookahead == 92 { state = 528; lexer.advance(false); continue; }
                if lookahead != 0 { state = 533; lexer.advance(false); continue; }
                return result;
            }
            529 => {
                result = true; lexer.set_result_symbol(sym_preproc_arg); lexer.mark_end();
                if lookahead == 13 { state = 532; lexer.advance(false); continue; }
                if lookahead == 42 { state = 523; lexer.advance(false); continue; }
                if lookahead == 47 { state = 414; lexer.advance(false); continue; }
                if lookahead == 92 { state = 529; lexer.advance(false); continue; }
                if lookahead != 0 { state = 524; lexer.advance(false); continue; }
                return result;
            }
            530 => {
                result = true; lexer.set_result_symbol(sym_preproc_arg); lexer.mark_end();
                if lookahead == 13 { state = 814; lexer.advance(false); continue; }
                if lookahead == 47 { state = 812; lexer.advance(false); continue; }
                if lookahead == 92 { state = 811; lexer.advance(false); continue; }
                if lookahead != 0 { state = 813; lexer.advance(false); continue; }
                return result;
            }
            531 => {
                result = true; lexer.set_result_symbol(sym_preproc_arg); lexer.mark_end();
                if lookahead == 42 { state = 524; lexer.advance(false); continue; }
                if lookahead == 47 { state = 812; lexer.advance(false); continue; }
                if lookahead == 92 { state = 528; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 533; lexer.advance(false); continue; }
                return result;
            }
            532 => {
                result = true; lexer.set_result_symbol(sym_preproc_arg); lexer.mark_end();
                if lookahead == 42 { state = 523; lexer.advance(false); continue; }
                if lookahead == 47 { state = 414; lexer.advance(false); continue; }
                if lookahead == 92 { state = 529; lexer.advance(false); continue; }
                if lookahead != 0 { state = 524; lexer.advance(false); continue; }
                return result;
            }
            533 => {
                result = true; lexer.set_result_symbol(sym_preproc_arg); lexer.mark_end();
                if lookahead == 47 { state = 498; lexer.advance(false); continue; }
                if lookahead == 92 { state = 528; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 533; lexer.advance(false); continue; }
                return result;
            }
            534 => {
                result = true; lexer.set_result_symbol(sym_preproc_arg); lexer.mark_end();
                if lookahead == 47 { state = 498; lexer.advance(false); continue; }
                if lookahead == 92 { state = 528; lexer.advance(false); continue; }
                if lookahead != 0 { state = 533; lexer.advance(false); continue; }
                return result;
            }
            535 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 99 { state = 561; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            536 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 100 { state = 559; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            537 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 100 { state = 543; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            538 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 100 { state = 545; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            539 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 100 { state = 547; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            540 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 101 { state = 550; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            541 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 101 { state = 516; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            542 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 101 { state = 504; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            543 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 101 { state = 502; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            544 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 101 { state = 553; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            545 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 101 { state = 554; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            546 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 101 { state = 555; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            547 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 101 { state = 556; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            548 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 102 { state = 509; lexer.advance(false); continue; }
                if lookahead == 110 { state = 535; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            549 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 102 { state = 509; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            550 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 102 { state = 557; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            551 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 102 { state = 518; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            552 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 102 { state = 512; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            553 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 102 { state = 513; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            554 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 102 { state = 514; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            555 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 102 { state = 520; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            556 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 102 { state = 522; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            557 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 105 { state = 563; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            558 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 105 { state = 551; lexer.advance(false); continue; }
                if lookahead == 115 { state = 541; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            559 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 105 { state = 552; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            560 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 108 { state = 558; lexer.advance(false); continue; }
                if lookahead == 110 { state = 536; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            561 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 108 { state = 564; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            562 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 110 { state = 536; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            563 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 110 { state = 542; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            564 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 117 { state = 537; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            565 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            566 => {
                result = true; lexer.set_result_symbol(anon_sym_LPAREN2); lexer.mark_end();
                return result;
            }
            567 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG); lexer.mark_end();
                return result;
            }
            568 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG); lexer.mark_end();
                if lookahead == 61 { state = 606; lexer.advance(false); continue; }
                return result;
            }
            569 => {
                result = true; lexer.set_result_symbol(anon_sym_TILDE); lexer.mark_end();
                return result;
            }
            570 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                return result;
            }
            571 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 663; lexer.advance(false); continue; }
                if lookahead == 46 { state = 476; lexer.advance(false); continue; }
                if lookahead == 48 { state = 674; lexer.advance(false); continue; }
                if lookahead == 61 { state = 656; lexer.advance(false); continue; }
                if lookahead == 62 { state = 671; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                return result;
            }
            572 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 663; lexer.advance(false); continue; }
                if lookahead == 46 { state = 476; lexer.advance(false); continue; }
                if lookahead == 48 { state = 674; lexer.advance(false); continue; }
                if lookahead == 61 { state = 656; lexer.advance(false); continue; }
                if lookahead == 62 { state = 670; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                return result;
            }
            573 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 663; lexer.advance(false); continue; }
                if lookahead == 46 { state = 476; lexer.advance(false); continue; }
                if lookahead == 48 { state = 674; lexer.advance(false); continue; }
                if lookahead == 62 { state = 670; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                return result;
            }
            574 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 663; lexer.advance(false); continue; }
                if lookahead == 46 { state = 476; lexer.advance(false); continue; }
                if lookahead == 48 { state = 674; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                return result;
            }
            575 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 663; lexer.advance(false); continue; }
                if lookahead == 61 { state = 656; lexer.advance(false); continue; }
                if lookahead == 62 { state = 671; lexer.advance(false); continue; }
                return result;
            }
            576 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 663; lexer.advance(false); continue; }
                if lookahead == 61 { state = 656; lexer.advance(false); continue; }
                if lookahead == 62 { state = 670; lexer.advance(false); continue; }
                return result;
            }
            577 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 663; lexer.advance(false); continue; }
                if lookahead == 62 { state = 670; lexer.advance(false); continue; }
                return result;
            }
            578 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 46 { state = 476; lexer.advance(false); continue; }
                if lookahead == 48 { state = 674; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                return result;
            }
            579 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 61 { state = 656; lexer.advance(false); continue; }
                if lookahead == 62 { state = 411; lexer.advance(false); continue; }
                return result;
            }
            580 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                return result;
            }
            581 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                if lookahead == 43 { state = 664; lexer.advance(false); continue; }
                return result;
            }
            582 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                if lookahead == 43 { state = 664; lexer.advance(false); continue; }
                if lookahead == 46 { state = 476; lexer.advance(false); continue; }
                if lookahead == 48 { state = 674; lexer.advance(false); continue; }
                if lookahead == 61 { state = 655; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                return result;
            }
            583 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                if lookahead == 43 { state = 664; lexer.advance(false); continue; }
                if lookahead == 46 { state = 476; lexer.advance(false); continue; }
                if lookahead == 48 { state = 674; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                return result;
            }
            584 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                if lookahead == 43 { state = 664; lexer.advance(false); continue; }
                if lookahead == 61 { state = 655; lexer.advance(false); continue; }
                return result;
            }
            585 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                if lookahead == 46 { state = 476; lexer.advance(false); continue; }
                if lookahead == 48 { state = 674; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                return result;
            }
            586 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                if lookahead == 61 { state = 655; lexer.advance(false); continue; }
                return result;
            }
            587 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                return result;
            }
            588 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                if lookahead == 61 { state = 652; lexer.advance(false); continue; }
                return result;
            }
            589 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH); lexer.mark_end();
                if lookahead == 42 { state = 413; lexer.advance(false); continue; }
                if lookahead == 47 { state = 815; lexer.advance(false); continue; }
                return result;
            }
            590 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH); lexer.mark_end();
                if lookahead == 42 { state = 413; lexer.advance(false); continue; }
                if lookahead == 47 { state = 815; lexer.advance(false); continue; }
                if lookahead == 61 { state = 653; lexer.advance(false); continue; }
                return result;
            }
            591 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT); lexer.mark_end();
                return result;
            }
            592 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT); lexer.mark_end();
                if lookahead == 61 { state = 654; lexer.advance(false); continue; }
                return result;
            }
            593 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE_PIPE); lexer.mark_end();
                return result;
            }
            594 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP_AMP); lexer.mark_end();
                return result;
            }
            595 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                return result;
            }
            596 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                if lookahead == 61 { state = 661; lexer.advance(false); continue; }
                if lookahead == 124 { state = 593; lexer.advance(false); continue; }
                return result;
            }
            597 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                if lookahead == 124 { state = 593; lexer.advance(false); continue; }
                return result;
            }
            598 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET); lexer.mark_end();
                return result;
            }
            599 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET); lexer.mark_end();
                if lookahead == 61 { state = 660; lexer.advance(false); continue; }
                return result;
            }
            600 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET); lexer.mark_end();
                if lookahead == 61 { state = 660; lexer.advance(false); continue; }
                if lookahead == 94 { state = 825; lexer.advance(false); continue; }
                return result;
            }
            601 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET); lexer.mark_end();
                if lookahead == 94 { state = 825; lexer.advance(false); continue; }
                return result;
            }
            602 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP); lexer.mark_end();
                return result;
            }
            603 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP); lexer.mark_end();
                if lookahead == 38 { state = 594; lexer.advance(false); continue; }
                return result;
            }
            604 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP); lexer.mark_end();
                if lookahead == 38 { state = 594; lexer.advance(false); continue; }
                if lookahead == 61 { state = 659; lexer.advance(false); continue; }
                return result;
            }
            605 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ_EQ); lexer.mark_end();
                return result;
            }
            606 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG_EQ); lexer.mark_end();
                return result;
            }
            607 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                return result;
            }
            608 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                if lookahead == 61 { state = 612; lexer.advance(false); continue; }
                if lookahead == 62 { state = 625; lexer.advance(false); continue; }
                return result;
            }
            609 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                if lookahead == 61 { state = 612; lexer.advance(false); continue; }
                if lookahead == 62 { state = 626; lexer.advance(false); continue; }
                return result;
            }
            610 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                if lookahead == 61 { state = 612; lexer.advance(false); continue; }
                if lookahead == 62 { state = 627; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 441; lexer.advance(false); continue; }
                return result;
            }
            611 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                if lookahead == 61 { state = 612; lexer.advance(false); continue; }
                if lookahead == 62 { state = 628; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 441; lexer.advance(false); continue; }
                return result;
            }
            612 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_EQ); lexer.mark_end();
                return result;
            }
            613 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_EQ); lexer.mark_end();
                return result;
            }
            614 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_EQ); lexer.mark_end();
                if lookahead == 62 { state = 662; lexer.advance(false); continue; }
                return result;
            }
            615 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                return result;
            }
            616 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 60 { state = 622; lexer.advance(false); continue; }
                if lookahead == 61 { state = 614; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 431; lexer.advance(false); continue; }
                return result;
            }
            617 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 60 { state = 621; lexer.advance(false); continue; }
                if lookahead == 61 { state = 613; lexer.advance(false); continue; }
                return result;
            }
            618 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 60 { state = 623; lexer.advance(false); continue; }
                if lookahead == 61 { state = 614; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 431; lexer.advance(false); continue; }
                return result;
            }
            619 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 60 { state = 624; lexer.advance(false); continue; }
                if lookahead == 61 { state = 614; lexer.advance(false); continue; }
                return result;
            }
            620 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 60 { state = 624; lexer.advance(false); continue; }
                if lookahead == 61 { state = 613; lexer.advance(false); continue; }
                return result;
            }
            621 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT); lexer.mark_end();
                return result;
            }
            622 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT); lexer.mark_end();
                if lookahead == 60 { state = 841; lexer.advance(false); continue; }
                if lookahead == 61 { state = 657; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 429; lexer.advance(false); continue; }
                return result;
            }
            623 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT); lexer.mark_end();
                if lookahead == 60 { state = 841; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 429; lexer.advance(false); continue; }
                return result;
            }
            624 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT); lexer.mark_end();
                if lookahead == 61 { state = 657; lexer.advance(false); continue; }
                return result;
            }
            625 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT); lexer.mark_end();
                return result;
            }
            626 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT); lexer.mark_end();
                if lookahead == 61 { state = 658; lexer.advance(false); continue; }
                return result;
            }
            627 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT); lexer.mark_end();
                if lookahead == 61 { state = 658; lexer.advance(false); continue; }
                if lookahead == 62 { state = 842; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 436; lexer.advance(false); continue; }
                return result;
            }
            628 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT); lexer.mark_end();
                if lookahead == 62 { state = 842; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 436; lexer.advance(false); continue; }
                return result;
            }
            629 => {
                result = true; lexer.set_result_symbol(anon_sym_SEMI); lexer.mark_end();
                return result;
            }
            630 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON); lexer.mark_end();
                return result;
            }
            631 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON); lexer.mark_end();
                if lookahead == 58 { state = 634; lexer.advance(false); continue; }
                return result;
            }
            632 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON); lexer.mark_end();
                if lookahead == 58 { state = 634; lexer.advance(false); continue; }
                if lookahead == 93 { state = 827; lexer.advance(false); continue; }
                return result;
            }
            633 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON); lexer.mark_end();
                if lookahead == 93 { state = 827; lexer.advance(false); continue; }
                return result;
            }
            634 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON_COLON); lexer.mark_end();
                return result;
            }
            635 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK_LBRACK); lexer.mark_end();
                return result;
            }
            636 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACK_RBRACK); lexer.mark_end();
                return result;
            }
            637 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACE); lexer.mark_end();
                return result;
            }
            638 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACE); lexer.mark_end();
                return result;
            }
            639 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK); lexer.mark_end();
                return result;
            }
            640 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK); lexer.mark_end();
                if lookahead == 58 { state = 826; lexer.advance(false); continue; }
                return result;
            }
            641 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK); lexer.mark_end();
                if lookahead == 58 { state = 826; lexer.advance(false); continue; }
                if lookahead == 91 { state = 635; lexer.advance(false); continue; }
                return result;
            }
            642 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK); lexer.mark_end();
                if lookahead == 58 { state = 826; lexer.advance(false); continue; }
                if lookahead == 91 { state = 635; lexer.advance(false); continue; }
                if lookahead == 93 { state = 829; lexer.advance(false); continue; }
                return result;
            }
            643 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK); lexer.mark_end();
                if lookahead == 91 { state = 635; lexer.advance(false); continue; }
                return result;
            }
            644 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK); lexer.mark_end();
                if lookahead == 91 { state = 635; lexer.advance(false); continue; }
                if lookahead == 93 { state = 829; lexer.advance(false); continue; }
                return result;
            }
            645 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK); lexer.mark_end();
                if lookahead == 93 { state = 829; lexer.advance(false); continue; }
                return result;
            }
            646 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACK); lexer.mark_end();
                return result;
            }
            647 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                return result;
            }
            648 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                if lookahead == 61 { state = 605; lexer.advance(false); continue; }
                return result;
            }
            649 => {
                result = true; lexer.set_result_symbol(sym_primitive_type); lexer.mark_end();
                if lookahead == 49 { state = 743; lexer.advance(false); continue; }
                if lookahead == 51 { state = 741; lexer.advance(false); continue; }
                if lookahead == 54 { state = 742; lexer.advance(false); continue; }
                if lookahead == 56 { state = 752; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 112 { state = 801; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            650 => {
                result = true; lexer.set_result_symbol(sym_primitive_type); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            651 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK); lexer.mark_end();
                return result;
            }
            652 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR_EQ); lexer.mark_end();
                return result;
            }
            653 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_EQ); lexer.mark_end();
                return result;
            }
            654 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT_EQ); lexer.mark_end();
                return result;
            }
            655 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS_EQ); lexer.mark_end();
                return result;
            }
            656 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_EQ); lexer.mark_end();
                return result;
            }
            657 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT_EQ); lexer.mark_end();
                return result;
            }
            658 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT_EQ); lexer.mark_end();
                return result;
            }
            659 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP_EQ); lexer.mark_end();
                return result;
            }
            660 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET_EQ); lexer.mark_end();
                return result;
            }
            661 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE_EQ); lexer.mark_end();
                return result;
            }
            662 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_EQ_GT); lexer.mark_end();
                return result;
            }
            663 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_DASH); lexer.mark_end();
                return result;
            }
            664 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS_PLUS); lexer.mark_end();
                return result;
            }
            665 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                if lookahead == 42 { state = 669; lexer.advance(false); continue; }
                if lookahead == 46 { state = 417; lexer.advance(false); continue; }
                return result;
            }
            666 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                if lookahead == 42 { state = 669; lexer.advance(false); continue; }
                if lookahead == 46 { state = 417; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 673; lexer.advance(false); continue; }
                return result;
            }
            667 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                if lookahead == 46 { state = 417; lexer.advance(false); continue; }
                return result;
            }
            668 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                if lookahead == 46 { state = 417; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 673; lexer.advance(false); continue; }
                return result;
            }
            669 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT_STAR); lexer.mark_end();
                return result;
            }
            670 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_GT); lexer.mark_end();
                return result;
            }
            671 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_GT); lexer.mark_end();
                if lookahead == 42 { state = 824; lexer.advance(false); continue; }
                return result;
            }
            672 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                return result;
            }
            673 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (39, 476), (66, 442), (98, 460), (69, 471), (101, 471), (70, 680), (102, 680), (76, 672),
                    (108, 672),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 673; lexer.advance(false); continue; }
                return result;
            }
            674 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (39, 474), (46, 681), (76, 682), (108, 685), (66, 473), (98, 473), (69, 471), (101, 471),
                    (85, 684), (117, 684), (88, 418), (120, 418), (90, 687), (122, 687), (56, 404), (57, 404),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 55 { state = 675; lexer.advance(false); continue; }
                return result;
            }
            675 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (39, 474), (46, 681), (76, 682), (108, 685), (69, 471), (101, 471), (85, 684), (117, 684),
                    (90, 687), (122, 687), (56, 404), (57, 404),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 55 { state = 675; lexer.advance(false); continue; }
                return result;
            }
            676 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (39, 473), (76, 682), (108, 685), (85, 684), (117, 684), (90, 687), (122, 687), (48, 676),
                    (49, 676),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            677 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (39, 475), (46, 681), (76, 682), (108, 685), (69, 471), (101, 471), (85, 684), (117, 684),
                    (90, 687), (122, 687),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 677; lexer.advance(false); continue; }
                return result;
            }
            678 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if lookahead == 39 { state = 478; lexer.advance(false); continue; }
                if lookahead == 66 { state = 442; lexer.advance(false); continue; }
                if lookahead == 98 { state = 460; lexer.advance(false); continue; }
                if lookahead == 70 || lookahead == 102 { state = 680; lexer.advance(false); continue; }
                if lookahead == 76 || lookahead == 108 { state = 672; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 678; lexer.advance(false); continue; }
                return result;
            }
            679 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (39, 480), (46, 472), (76, 682), (108, 685), (80, 471), (112, 471), (85, 684), (117, 684),
                    (90, 687), (122, 687),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 679; lexer.advance(false); continue; }
                return result;
            }
            680 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if lookahead == 49 { state = 422; lexer.advance(false); continue; }
                if lookahead == 51 { state = 421; lexer.advance(false); continue; }
                if lookahead == 54 { state = 423; lexer.advance(false); continue; }
                return result;
            }
            681 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (66, 442), (98, 460), (69, 471), (101, 471), (70, 680), (102, 680), (76, 672), (108, 672),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 673; lexer.advance(false); continue; }
                return result;
            }
            682 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if lookahead == 76 { state = 687; lexer.advance(false); continue; }
                if lookahead == 85 || lookahead == 117 { state = 672; lexer.advance(false); continue; }
                return result;
            }
            683 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if lookahead == 76 { state = 672; lexer.advance(false); continue; }
                return result;
            }
            684 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if lookahead == 76 { state = 683; lexer.advance(false); continue; }
                if lookahead == 108 { state = 686; lexer.advance(false); continue; }
                if lookahead == 90 || lookahead == 122 { state = 672; lexer.advance(false); continue; }
                return result;
            }
            685 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if lookahead == 108 { state = 687; lexer.advance(false); continue; }
                if lookahead == 85 || lookahead == 117 { state = 672; lexer.advance(false); continue; }
                return result;
            }
            686 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if lookahead == 108 { state = 672; lexer.advance(false); continue; }
                return result;
            }
            687 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if lookahead == 85 || lookahead == 117 { state = 672; lexer.advance(false); continue; }
                return result;
            }
            688 => {
                result = true; lexer.set_result_symbol(anon_sym_L_SQUOTE); lexer.mark_end();
                return result;
            }
            689 => {
                result = true; lexer.set_result_symbol(anon_sym_u_SQUOTE); lexer.mark_end();
                return result;
            }
            690 => {
                result = true; lexer.set_result_symbol(anon_sym_U_SQUOTE); lexer.mark_end();
                return result;
            }
            691 => {
                result = true; lexer.set_result_symbol(anon_sym_u8_SQUOTE); lexer.mark_end();
                return result;
            }
            692 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTE); lexer.mark_end();
                return result;
            }
            693 => {
                result = true; lexer.set_result_symbol(aux_sym_char_literal_token1); lexer.mark_end();
                return result;
            }
            694 => {
                result = true; lexer.set_result_symbol(aux_sym_char_literal_token1); lexer.mark_end();
                if lookahead == 10 { state = 708; lexer.advance(false); continue; }
                if lookahead == 13 { state = 707; lexer.advance(false); continue; }
                if lookahead == 85 { state = 497; lexer.advance(false); continue; }
                if lookahead == 117 { state = 489; lexer.advance(false); continue; }
                if lookahead == 120 { state = 483; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 710; lexer.advance(false); continue; }
                if lookahead != 0 { state = 707; lexer.advance(false); continue; }
                return result;
            }
            695 => {
                result = true; lexer.set_result_symbol(aux_sym_char_literal_token1); lexer.mark_end();
                if lookahead == 42 { state = 413; lexer.advance(false); continue; }
                if lookahead == 47 { state = 815; lexer.advance(false); continue; }
                return result;
            }
            696 => {
                result = true; lexer.set_result_symbol(aux_sym_char_literal_token1); lexer.mark_end();
                if lookahead == 92 { state = 147; lexer.advance(false); continue; }
                return result;
            }
            697 => {
                result = true; lexer.set_result_symbol(anon_sym_L_DQUOTE); lexer.mark_end();
                return result;
            }
            698 => {
                result = true; lexer.set_result_symbol(anon_sym_u_DQUOTE); lexer.mark_end();
                return result;
            }
            699 => {
                result = true; lexer.set_result_symbol(anon_sym_U_DQUOTE); lexer.mark_end();
                return result;
            }
            700 => {
                result = true; lexer.set_result_symbol(anon_sym_u8_DQUOTE); lexer.mark_end();
                return result;
            }
            701 => {
                result = true; lexer.set_result_symbol(anon_sym_DQUOTE); lexer.mark_end();
                return result;
            }
            702 => {
                result = true; lexer.set_result_symbol(aux_sym_string_literal_token1); lexer.mark_end();
                if lookahead == 42 { state = 704; lexer.advance(false); continue; }
                if lookahead == 47 { state = 706; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 34 && lookahead != 92 { state = 706; lexer.advance(false); continue; }
                return result;
            }
            703 => {
                result = true; lexer.set_result_symbol(aux_sym_string_literal_token1); lexer.mark_end();
                if lookahead == 42 { state = 703; lexer.advance(false); continue; }
                if lookahead == 47 { state = 706; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 34 && lookahead != 92 { state = 704; lexer.advance(false); continue; }
                return result;
            }
            704 => {
                result = true; lexer.set_result_symbol(aux_sym_string_literal_token1); lexer.mark_end();
                if lookahead == 42 { state = 703; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 34 && lookahead != 92 { state = 704; lexer.advance(false); continue; }
                return result;
            }
            705 => {
                result = true; lexer.set_result_symbol(aux_sym_string_literal_token1); lexer.mark_end();
                if lookahead == 47 { state = 702; lexer.advance(false); continue; }
                if lookahead == 9 || 11 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 705; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 34 && lookahead != 92 { state = 706; lexer.advance(false); continue; }
                return result;
            }
            706 => {
                result = true; lexer.set_result_symbol(aux_sym_string_literal_token1); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 && lookahead != 34 && lookahead != 92 { state = 706; lexer.advance(false); continue; }
                return result;
            }
            707 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                return result;
            }
            708 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                if lookahead == 92 { state = 147; lexer.advance(false); continue; }
                return result;
            }
            709 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 { state = 707; lexer.advance(false); continue; }
                return result;
            }
            710 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 { state = 709; lexer.advance(false); continue; }
                return result;
            }
            711 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 707; lexer.advance(false); continue; }
                return result;
            }
            712 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 711; lexer.advance(false); continue; }
                return result;
            }
            713 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 712; lexer.advance(false); continue; }
                return result;
            }
            714 => {
                result = true; lexer.set_result_symbol(sym_system_lib_string); lexer.mark_end();
                return result;
            }
            715 => {
                result = true; lexer.set_result_symbol(sym_system_lib_string); lexer.mark_end();
                if lookahead == 62 { state = 714; lexer.advance(false); continue; }
                if lookahead == 92 { state = 440; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 439; lexer.advance(false); continue; }
                return result;
            }
            716 => {
                result = true; lexer.set_result_symbol(sym_true); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            717 => {
                result = true; lexer.set_result_symbol(sym_false); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            718 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 34 { state = 697; lexer.advance(false); continue; }
                if lookahead == 39 { state = 688; lexer.advance(false); continue; }
                if lookahead == 82 { state = 729; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            719 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 34 { state = 697; lexer.advance(false); continue; }
                if lookahead == 82 { state = 729; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            720 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 34 { state = 697; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            721 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 34 { state = 819; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            722 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 34 { state = 699; lexer.advance(false); continue; }
                if lookahead == 39 { state = 690; lexer.advance(false); continue; }
                if lookahead == 82 { state = 730; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            723 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 34 { state = 699; lexer.advance(false); continue; }
                if lookahead == 82 { state = 730; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            724 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 34 { state = 699; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            725 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 34 { state = 698; lexer.advance(false); continue; }
                if lookahead == 39 { state = 689; lexer.advance(false); continue; }
                if lookahead == 56 { state = 731; lexer.advance(false); continue; }
                if lookahead == 82 { state = 734; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 105 { state = 785; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            726 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 34 { state = 698; lexer.advance(false); continue; }
                if lookahead == 56 { state = 732; lexer.advance(false); continue; }
                if lookahead == 82 { state = 734; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            727 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 34 { state = 698; lexer.advance(false); continue; }
                if lookahead == 56 { state = 733; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 105 { state = 785; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            728 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 34 { state = 698; lexer.advance(false); continue; }
                if lookahead == 56 { state = 733; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            729 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 34 { state = 820; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            730 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 34 { state = 822; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            731 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 34 { state = 700; lexer.advance(false); continue; }
                if lookahead == 39 { state = 691; lexer.advance(false); continue; }
                if lookahead == 82 { state = 735; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            732 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 34 { state = 700; lexer.advance(false); continue; }
                if lookahead == 82 { state = 735; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            733 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 34 { state = 700; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            734 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 34 { state = 821; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            735 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 34 { state = 823; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            736 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 39 { state = 688; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            737 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 39 { state = 690; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            738 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 39 { state = 689; lexer.advance(false); continue; }
                if lookahead == 56 { state = 739; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            739 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 39 { state = 691; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            740 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 49 { state = 743; lexer.advance(false); continue; }
                if lookahead == 51 { state = 741; lexer.advance(false); continue; }
                if lookahead == 54 { state = 742; lexer.advance(false); continue; }
                if lookahead == 56 { state = 752; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 112 { state = 801; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            741 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 50 { state = 752; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            742 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 52 { state = 752; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            743 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 54 { state = 752; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            744 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 65 { state = 747; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            745 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 69 { state = 716; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            746 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 69 { state = 717; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            747 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 76 { state = 749; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            748 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 82 { state = 750; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            749 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 83 { state = 746; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            750 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 85 { state = 745; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            751 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 95 { state = 758; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            752 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 95 { state = 798; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            753 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 97 { state = 776; lexer.advance(false); continue; }
                if lookahead == 108 { state = 788; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            754 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 97 { state = 776; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            755 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 97 { state = 805; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            756 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 97 { state = 792; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            757 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 97 { state = 798; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            758 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 97 { state = 780; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            759 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 98 { state = 781; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            760 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 100 { state = 650; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            761 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 100 { state = 772; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            762 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 101 { state = 716; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            763 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 101 { state = 650; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            764 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 101 { state = 717; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            765 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 101 { state = 752; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            766 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 102 { state = 752; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            767 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 102 { state = 766; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            768 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 103 { state = 784; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            769 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 104 { state = 756; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            770 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 105 { state = 806; lexer.advance(false); continue; }
                if lookahead == 115 { state = 771; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            771 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 105 { state = 806; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            772 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 105 { state = 767; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            773 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 105 { state = 768; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            774 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 105 { state = 760; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            775 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 105 { state = 785; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            776 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 108 { state = 796; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            777 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 108 { state = 650; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            778 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 108 { state = 791; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            779 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 108 { state = 778; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            780 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 108 { state = 773; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            781 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 108 { state = 763; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            782 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 108 { state = 788; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            783 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 110 { state = 797; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            784 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 110 { state = 752; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            785 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 110 { state = 799; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            786 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 111 { state = 802; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            787 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 111 { state = 774; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            788 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 111 { state = 757; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            789 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 111 { state = 777; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            790 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 111 { state = 789; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            791 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 112 { state = 801; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            792 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 114 { state = 649; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            793 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 114 { state = 761; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            794 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 114 { state = 752; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            795 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 114 { state = 804; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            796 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 115 { state = 764; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            797 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 116 { state = 649; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            798 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 116 { state = 650; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            799 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 116 { state = 740; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            800 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 116 { state = 793; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            801 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 116 { state = 794; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            802 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 117 { state = 759; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            803 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 117 { state = 779; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            804 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 117 { state = 762; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            805 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 120 { state = 751; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            806 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if lookahead == 122 { state = 765; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            807 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            808 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                return result;
            }
            809 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 10 { state = 815; lexer.advance(false); continue; }
                if lookahead == 47 { state = 812; lexer.advance(false); continue; }
                if lookahead == 92 { state = 530; lexer.advance(false); continue; }
                if lookahead != 0 { state = 813; lexer.advance(false); continue; }
                return result;
            }
            810 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 13 { state = 816; lexer.advance(false); continue; }
                if lookahead == 92 { state = 810; lexer.advance(false); continue; }
                if lookahead != 0 { state = 815; lexer.advance(false); continue; }
                return result;
            }
            811 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 13 { state = 814; lexer.advance(false); continue; }
                if lookahead == 47 { state = 812; lexer.advance(false); continue; }
                if lookahead == 92 { state = 811; lexer.advance(false); continue; }
                if lookahead != 0 { state = 813; lexer.advance(false); continue; }
                return result;
            }
            812 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 42 { state = 815; lexer.advance(false); continue; }
                if lookahead == 92 { state = 525; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 813; lexer.advance(false); continue; }
                return result;
            }
            813 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 47 { state = 812; lexer.advance(false); continue; }
                if lookahead == 92 { state = 530; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 813; lexer.advance(false); continue; }
                return result;
            }
            814 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 47 { state = 812; lexer.advance(false); continue; }
                if lookahead == 92 { state = 530; lexer.advance(false); continue; }
                if lookahead != 0 { state = 813; lexer.advance(false); continue; }
                return result;
            }
            815 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 92 { state = 260; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 815; lexer.advance(false); continue; }
                return result;
            }
            816 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 92 { state = 260; lexer.advance(false); continue; }
                if lookahead != 0 { state = 815; lexer.advance(false); continue; }
                return result;
            }
            817 => {
                result = true; lexer.set_result_symbol(anon_sym_GT2); lexer.mark_end();
                return result;
            }
            818 => {
                result = true; lexer.set_result_symbol(aux_sym_pure_virtual_clause_token1); lexer.mark_end();
                return result;
            }
            819 => {
                result = true; lexer.set_result_symbol(anon_sym_R_DQUOTE); lexer.mark_end();
                return result;
            }
            820 => {
                result = true; lexer.set_result_symbol(anon_sym_LR_DQUOTE); lexer.mark_end();
                return result;
            }
            821 => {
                result = true; lexer.set_result_symbol(anon_sym_uR_DQUOTE); lexer.mark_end();
                return result;
            }
            822 => {
                result = true; lexer.set_result_symbol(anon_sym_UR_DQUOTE); lexer.mark_end();
                return result;
            }
            823 => {
                result = true; lexer.set_result_symbol(anon_sym_u8R_DQUOTE); lexer.mark_end();
                return result;
            }
            824 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_GT_STAR); lexer.mark_end();
                return result;
            }
            825 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET_CARET); lexer.mark_end();
                return result;
            }
            826 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK_COLON); lexer.mark_end();
                return result;
            }
            827 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON_RBRACK); lexer.mark_end();
                return result;
            }
            828 => {
                result = true; lexer.set_result_symbol(anon_sym_LPAREN_RPAREN); lexer.mark_end();
                return result;
            }
            829 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK_RBRACK); lexer.mark_end();
                return result;
            }
            830 => {
                result = true; lexer.set_result_symbol(anon_sym_DQUOTE_DQUOTE); lexer.mark_end();
                return result;
            }
            831 => {
                result = true; lexer.set_result_symbol(sym_literal_suffix); lexer.mark_end();
                if lookahead == 34 { state = 697; lexer.advance(false); continue; }
                if lookahead == 82 { state = 835; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 840; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            832 => {
                result = true; lexer.set_result_symbol(sym_literal_suffix); lexer.mark_end();
                if lookahead == 34 { state = 819; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 840; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            833 => {
                result = true; lexer.set_result_symbol(sym_literal_suffix); lexer.mark_end();
                if lookahead == 34 { state = 699; lexer.advance(false); continue; }
                if lookahead == 82 { state = 836; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 840; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            834 => {
                result = true; lexer.set_result_symbol(sym_literal_suffix); lexer.mark_end();
                if lookahead == 34 { state = 698; lexer.advance(false); continue; }
                if lookahead == 56 { state = 837; lexer.advance(false); continue; }
                if lookahead == 82 { state = 838; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 840; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            835 => {
                result = true; lexer.set_result_symbol(sym_literal_suffix); lexer.mark_end();
                if lookahead == 34 { state = 820; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 840; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            836 => {
                result = true; lexer.set_result_symbol(sym_literal_suffix); lexer.mark_end();
                if lookahead == 34 { state = 822; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 840; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            837 => {
                result = true; lexer.set_result_symbol(sym_literal_suffix); lexer.mark_end();
                if lookahead == 34 { state = 700; lexer.advance(false); continue; }
                if lookahead == 82 { state = 839; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 840; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            838 => {
                result = true; lexer.set_result_symbol(sym_literal_suffix); lexer.mark_end();
                if lookahead == 34 { state = 821; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 840; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            839 => {
                result = true; lexer.set_result_symbol(sym_literal_suffix); lexer.mark_end();
                if lookahead == 34 { state = 823; lexer.advance(false); continue; }
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 840; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            840 => {
                result = true; lexer.set_result_symbol(sym_literal_suffix); lexer.mark_end();
                if lookahead == 92 { state = 443; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 840; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 807; lexer.advance(false); continue; }
                return result;
            }
            841 => {
                result = true; lexer.set_result_symbol(aux_sym_kernel_call_syntax_token1); lexer.mark_end();
                return result;
            }
            842 => {
                result = true; lexer.set_result_symbol(aux_sym_kernel_call_syntax_token2); lexer.mark_end();
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
                if lookahead == 78 { state = 1; lexer.advance(false); continue; }
                if lookahead == 92 { state = 2; lexer.advance(true); continue; }
                if lookahead == 95 { state = 3; lexer.advance(false); continue; }
                if lookahead == 97 { state = 4; lexer.advance(false); continue; }
                if lookahead == 98 { state = 5; lexer.advance(false); continue; }
                if lookahead == 99 { state = 6; lexer.advance(false); continue; }
                if lookahead == 100 { state = 7; lexer.advance(false); continue; }
                if lookahead == 101 { state = 8; lexer.advance(false); continue; }
                if lookahead == 102 { state = 9; lexer.advance(false); continue; }
                if lookahead == 103 { state = 10; lexer.advance(false); continue; }
                if lookahead == 105 { state = 11; lexer.advance(false); continue; }
                if lookahead == 108 { state = 12; lexer.advance(false); continue; }
                if lookahead == 109 { state = 13; lexer.advance(false); continue; }
                if lookahead == 110 { state = 14; lexer.advance(false); continue; }
                if lookahead == 111 { state = 15; lexer.advance(false); continue; }
                if lookahead == 112 { state = 16; lexer.advance(false); continue; }
                if lookahead == 114 { state = 17; lexer.advance(false); continue; }
                if lookahead == 115 { state = 18; lexer.advance(false); continue; }
                if lookahead == 116 { state = 19; lexer.advance(false); continue; }
                if lookahead == 117 { state = 20; lexer.advance(false); continue; }
                if lookahead == 118 { state = 21; lexer.advance(false); continue; }
                if lookahead == 119 { state = 22; lexer.advance(false); continue; }
                if lookahead == 120 { state = 23; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 0; lexer.advance(true); continue; }
                return result;
            }
            1 => {
                if lookahead == 85 { state = 24; lexer.advance(false); continue; }
                return result;
            }
            2 => {
                if lookahead == 10 { state = 0; lexer.advance(true); continue; }
                if lookahead == 13 { state = 25; lexer.advance(true); continue; }
                return result;
            }
            3 => {
                if lookahead == 65 { state = 26; lexer.advance(false); continue; }
                if lookahead == 71 { state = 27; lexer.advance(false); continue; }
                if lookahead == 78 { state = 28; lexer.advance(false); continue; }
                if lookahead == 95 { state = 29; lexer.advance(false); continue; }
                if lookahead == 97 { state = 30; lexer.advance(false); continue; }
                if lookahead == 117 { state = 31; lexer.advance(false); continue; }
                return result;
            }
            4 => {
                if lookahead == 108 { state = 32; lexer.advance(false); continue; }
                if lookahead == 110 { state = 33; lexer.advance(false); continue; }
                if lookahead == 115 { state = 34; lexer.advance(false); continue; }
                if lookahead == 117 { state = 35; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                if lookahead == 105 { state = 36; lexer.advance(false); continue; }
                if lookahead == 114 { state = 37; lexer.advance(false); continue; }
                return result;
            }
            6 => {
                if lookahead == 97 { state = 38; lexer.advance(false); continue; }
                if lookahead == 108 { state = 39; lexer.advance(false); continue; }
                if lookahead == 111 { state = 40; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if lookahead == 101 { state = 41; lexer.advance(false); continue; }
                if lookahead == 111 { state = 42; lexer.advance(false); continue; }
                return result;
            }
            8 => {
                if lookahead == 108 { state = 43; lexer.advance(false); continue; }
                if lookahead == 110 { state = 44; lexer.advance(false); continue; }
                if lookahead == 120 { state = 45; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if lookahead == 105 { state = 46; lexer.advance(false); continue; }
                if lookahead == 111 { state = 47; lexer.advance(false); continue; }
                if lookahead == 114 { state = 48; lexer.advance(false); continue; }
                return result;
            }
            10 => {
                if lookahead == 111 { state = 49; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if lookahead == 102 { state = 50; lexer.advance(false); continue; }
                if lookahead == 109 { state = 51; lexer.advance(false); continue; }
                if lookahead == 110 { state = 52; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if lookahead == 111 { state = 53; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if lookahead == 111 { state = 54; lexer.advance(false); continue; }
                if lookahead == 117 { state = 55; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if lookahead == 97 { state = 56; lexer.advance(false); continue; }
                if lookahead == 101 { state = 57; lexer.advance(false); continue; }
                if lookahead == 111 { state = 58; lexer.advance(false); continue; }
                if lookahead == 117 { state = 59; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if lookahead == 102 { state = 60; lexer.advance(false); continue; }
                if lookahead == 112 { state = 61; lexer.advance(false); continue; }
                if lookahead == 114 { state = 62; lexer.advance(false); continue; }
                if lookahead == 118 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if lookahead == 114 { state = 64; lexer.advance(false); continue; }
                if lookahead == 117 { state = 65; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if lookahead == 101 { state = 66; lexer.advance(false); continue; }
                return result;
            }
            18 => {
                if lookahead == 104 { state = 67; lexer.advance(false); continue; }
                if lookahead == 105 { state = 68; lexer.advance(false); continue; }
                if lookahead == 116 { state = 69; lexer.advance(false); continue; }
                if lookahead == 119 { state = 70; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if lookahead == 101 { state = 71; lexer.advance(false); continue; }
                if lookahead == 104 { state = 72; lexer.advance(false); continue; }
                if lookahead == 114 { state = 73; lexer.advance(false); continue; }
                if lookahead == 121 { state = 74; lexer.advance(false); continue; }
                return result;
            }
            20 => {
                if lookahead == 110 { state = 75; lexer.advance(false); continue; }
                if lookahead == 115 { state = 76; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                if lookahead == 105 { state = 77; lexer.advance(false); continue; }
                if lookahead == 111 { state = 78; lexer.advance(false); continue; }
                return result;
            }
            22 => {
                if lookahead == 104 { state = 79; lexer.advance(false); continue; }
                return result;
            }
            23 => {
                if lookahead == 111 { state = 80; lexer.advance(false); continue; }
                return result;
            }
            24 => {
                if lookahead == 76 { state = 81; lexer.advance(false); continue; }
                return result;
            }
            25 => {
                if lookahead == 10 { state = 0; lexer.advance(true); continue; }
                return result;
            }
            26 => {
                if lookahead == 108 { state = 82; lexer.advance(false); continue; }
                if lookahead == 116 { state = 83; lexer.advance(false); continue; }
                return result;
            }
            27 => {
                if lookahead == 101 { state = 84; lexer.advance(false); continue; }
                return result;
            }
            28 => {
                if lookahead == 111 { state = 85; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                if let Some(next) = advance_map(&[
                    (97, 86), (98, 87), (99, 88), (100, 89), (101, 90), (102, 91), (103, 92), (104, 93),
                    (105, 94), (108, 95), (109, 96), (110, 97), (114, 98), (115, 99), (116, 100), (117, 101),
                    (118, 102),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            30 => {
                if lookahead == 108 { state = 103; lexer.advance(false); continue; }
                return result;
            }
            31 => {
                if lookahead == 110 { state = 104; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                if lookahead == 105 { state = 105; lexer.advance(false); continue; }
                return result;
            }
            33 => {
                if lookahead == 100 { state = 106; lexer.advance(false); continue; }
                return result;
            }
            34 => {
                if lookahead == 109 { state = 107; lexer.advance(false); continue; }
                return result;
            }
            35 => {
                if lookahead == 116 { state = 108; lexer.advance(false); continue; }
                return result;
            }
            36 => {
                if lookahead == 116 { state = 109; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                if lookahead == 101 { state = 110; lexer.advance(false); continue; }
                return result;
            }
            38 => {
                if lookahead == 115 { state = 111; lexer.advance(false); continue; }
                if lookahead == 116 { state = 112; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                if lookahead == 97 { state = 113; lexer.advance(false); continue; }
                return result;
            }
            40 => {
                if lookahead == 95 { state = 114; lexer.advance(false); continue; }
                if lookahead == 109 { state = 115; lexer.advance(false); continue; }
                if lookahead == 110 { state = 116; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                if lookahead == 99 { state = 117; lexer.advance(false); continue; }
                if lookahead == 102 { state = 118; lexer.advance(false); continue; }
                if lookahead == 108 { state = 119; lexer.advance(false); continue; }
                return result;
            }
            42 => {
                result = true; lexer.set_result_symbol(anon_sym_do); lexer.mark_end();
                return result;
            }
            43 => {
                if lookahead == 115 { state = 120; lexer.advance(false); continue; }
                return result;
            }
            44 => {
                if lookahead == 117 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            45 => {
                if lookahead == 112 { state = 122; lexer.advance(false); continue; }
                if lookahead == 116 { state = 123; lexer.advance(false); continue; }
                return result;
            }
            46 => {
                if lookahead == 110 { state = 124; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                if lookahead == 114 { state = 125; lexer.advance(false); continue; }
                return result;
            }
            48 => {
                if lookahead == 105 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                if lookahead == 116 { state = 127; lexer.advance(false); continue; }
                return result;
            }
            50 => {
                result = true; lexer.set_result_symbol(anon_sym_if); lexer.mark_end();
                return result;
            }
            51 => {
                if lookahead == 112 { state = 128; lexer.advance(false); continue; }
                return result;
            }
            52 => {
                if lookahead == 108 { state = 129; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                if lookahead == 110 { state = 130; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                if lookahead == 100 { state = 131; lexer.advance(false); continue; }
                return result;
            }
            55 => {
                if lookahead == 116 { state = 132; lexer.advance(false); continue; }
                return result;
            }
            56 => {
                if lookahead == 109 { state = 133; lexer.advance(false); continue; }
                return result;
            }
            57 => {
                if lookahead == 119 { state = 134; lexer.advance(false); continue; }
                return result;
            }
            58 => {
                if lookahead == 101 { state = 135; lexer.advance(false); continue; }
                if lookahead == 114 { state = 136; lexer.advance(false); continue; }
                if lookahead == 116 { state = 137; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                if lookahead == 108 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            60 => {
                if lookahead == 102 { state = 139; lexer.advance(false); continue; }
                return result;
            }
            61 => {
                if lookahead == 101 { state = 140; lexer.advance(false); continue; }
                return result;
            }
            62 => {
                result = true; lexer.set_result_symbol(anon_sym_or); lexer.mark_end();
                if lookahead == 95 { state = 141; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                if lookahead == 101 { state = 142; lexer.advance(false); continue; }
                return result;
            }
            64 => {
                if lookahead == 105 { state = 143; lexer.advance(false); continue; }
                if lookahead == 111 { state = 144; lexer.advance(false); continue; }
                return result;
            }
            65 => {
                if lookahead == 98 { state = 145; lexer.advance(false); continue; }
                return result;
            }
            66 => {
                if lookahead == 103 { state = 146; lexer.advance(false); continue; }
                if lookahead == 113 { state = 147; lexer.advance(false); continue; }
                if lookahead == 115 { state = 148; lexer.advance(false); continue; }
                if lookahead == 116 { state = 149; lexer.advance(false); continue; }
                return result;
            }
            67 => {
                if lookahead == 111 { state = 150; lexer.advance(false); continue; }
                return result;
            }
            68 => {
                if lookahead == 103 { state = 151; lexer.advance(false); continue; }
                if lookahead == 122 { state = 152; lexer.advance(false); continue; }
                return result;
            }
            69 => {
                if lookahead == 97 { state = 153; lexer.advance(false); continue; }
                if lookahead == 114 { state = 154; lexer.advance(false); continue; }
                return result;
            }
            70 => {
                if lookahead == 105 { state = 155; lexer.advance(false); continue; }
                return result;
            }
            71 => {
                if lookahead == 109 { state = 156; lexer.advance(false); continue; }
                return result;
            }
            72 => {
                if lookahead == 105 { state = 157; lexer.advance(false); continue; }
                if lookahead == 114 { state = 158; lexer.advance(false); continue; }
                return result;
            }
            73 => {
                if lookahead == 121 { state = 159; lexer.advance(false); continue; }
                return result;
            }
            74 => {
                if lookahead == 112 { state = 160; lexer.advance(false); continue; }
                return result;
            }
            75 => {
                if lookahead == 105 { state = 161; lexer.advance(false); continue; }
                if lookahead == 115 { state = 162; lexer.advance(false); continue; }
                return result;
            }
            76 => {
                if lookahead == 105 { state = 163; lexer.advance(false); continue; }
                return result;
            }
            77 => {
                if lookahead == 114 { state = 164; lexer.advance(false); continue; }
                return result;
            }
            78 => {
                if lookahead == 108 { state = 165; lexer.advance(false); continue; }
                return result;
            }
            79 => {
                if lookahead == 105 { state = 166; lexer.advance(false); continue; }
                return result;
            }
            80 => {
                if lookahead == 114 { state = 167; lexer.advance(false); continue; }
                return result;
            }
            81 => {
                if lookahead == 76 { state = 168; lexer.advance(false); continue; }
                return result;
            }
            82 => {
                if lookahead == 105 { state = 169; lexer.advance(false); continue; }
                return result;
            }
            83 => {
                if lookahead == 111 { state = 170; lexer.advance(false); continue; }
                return result;
            }
            84 => {
                if lookahead == 110 { state = 171; lexer.advance(false); continue; }
                return result;
            }
            85 => {
                if lookahead == 110 { state = 172; lexer.advance(false); continue; }
                if lookahead == 114 { state = 173; lexer.advance(false); continue; }
                return result;
            }
            86 => {
                if lookahead == 108 { state = 174; lexer.advance(false); continue; }
                if lookahead == 115 { state = 175; lexer.advance(false); continue; }
                if lookahead == 116 { state = 176; lexer.advance(false); continue; }
                return result;
            }
            87 => {
                if lookahead == 97 { state = 177; lexer.advance(false); continue; }
                return result;
            }
            88 => {
                if lookahead == 100 { state = 178; lexer.advance(false); continue; }
                if lookahead == 108 { state = 179; lexer.advance(false); continue; }
                if lookahead == 111 { state = 180; lexer.advance(false); continue; }
                return result;
            }
            89 => {
                if lookahead == 101 { state = 181; lexer.advance(false); continue; }
                return result;
            }
            90 => {
                if lookahead == 120 { state = 182; lexer.advance(false); continue; }
                return result;
            }
            91 => {
                if lookahead == 97 { state = 183; lexer.advance(false); continue; }
                if lookahead == 105 { state = 184; lexer.advance(false); continue; }
                if lookahead == 111 { state = 185; lexer.advance(false); continue; }
                return result;
            }
            92 => {
                if lookahead == 108 { state = 186; lexer.advance(false); continue; }
                if lookahead == 114 { state = 187; lexer.advance(false); continue; }
                return result;
            }
            93 => {
                if lookahead == 111 { state = 188; lexer.advance(false); continue; }
                return result;
            }
            94 => {
                if lookahead == 110 { state = 189; lexer.advance(false); continue; }
                return result;
            }
            95 => {
                if lookahead == 97 { state = 190; lexer.advance(false); continue; }
                if lookahead == 101 { state = 191; lexer.advance(false); continue; }
                if lookahead == 111 { state = 192; lexer.advance(false); continue; }
                return result;
            }
            96 => {
                if lookahead == 97 { state = 193; lexer.advance(false); continue; }
                return result;
            }
            97 => {
                if lookahead == 111 { state = 194; lexer.advance(false); continue; }
                return result;
            }
            98 => {
                if lookahead == 101 { state = 195; lexer.advance(false); continue; }
                return result;
            }
            99 => {
                if lookahead == 104 { state = 196; lexer.advance(false); continue; }
                if lookahead == 112 { state = 197; lexer.advance(false); continue; }
                if lookahead == 116 { state = 198; lexer.advance(false); continue; }
                return result;
            }
            100 => {
                if lookahead == 104 { state = 199; lexer.advance(false); continue; }
                if lookahead == 105 { state = 200; lexer.advance(false); continue; }
                if lookahead == 114 { state = 201; lexer.advance(false); continue; }
                return result;
            }
            101 => {
                if lookahead == 110 { state = 202; lexer.advance(false); continue; }
                if lookahead == 112 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            102 => {
                if lookahead == 101 { state = 204; lexer.advance(false); continue; }
                if lookahead == 111 { state = 205; lexer.advance(false); continue; }
                return result;
            }
            103 => {
                if lookahead == 105 { state = 206; lexer.advance(false); continue; }
                return result;
            }
            104 => {
                if lookahead == 97 { state = 207; lexer.advance(false); continue; }
                return result;
            }
            105 => {
                if lookahead == 103 { state = 208; lexer.advance(false); continue; }
                return result;
            }
            106 => {
                result = true; lexer.set_result_symbol(anon_sym_and); lexer.mark_end();
                if lookahead == 95 { state = 209; lexer.advance(false); continue; }
                return result;
            }
            107 => {
                result = true; lexer.set_result_symbol(anon_sym_asm); lexer.mark_end();
                return result;
            }
            108 => {
                if lookahead == 111 { state = 210; lexer.advance(false); continue; }
                return result;
            }
            109 => {
                if lookahead == 97 { state = 211; lexer.advance(false); continue; }
                if lookahead == 111 { state = 212; lexer.advance(false); continue; }
                return result;
            }
            110 => {
                if lookahead == 97 { state = 213; lexer.advance(false); continue; }
                return result;
            }
            111 => {
                if lookahead == 101 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            112 => {
                if lookahead == 99 { state = 215; lexer.advance(false); continue; }
                return result;
            }
            113 => {
                if lookahead == 115 { state = 216; lexer.advance(false); continue; }
                return result;
            }
            114 => {
                if lookahead == 97 { state = 217; lexer.advance(false); continue; }
                if lookahead == 114 { state = 218; lexer.advance(false); continue; }
                if lookahead == 121 { state = 219; lexer.advance(false); continue; }
                return result;
            }
            115 => {
                if lookahead == 112 { state = 220; lexer.advance(false); continue; }
                return result;
            }
            116 => {
                if lookahead == 99 { state = 221; lexer.advance(false); continue; }
                if lookahead == 115 { state = 222; lexer.advance(false); continue; }
                if lookahead == 116 { state = 223; lexer.advance(false); continue; }
                return result;
            }
            117 => {
                if lookahead == 108 { state = 224; lexer.advance(false); continue; }
                return result;
            }
            118 => {
                if lookahead == 97 { state = 225; lexer.advance(false); continue; }
                if lookahead == 105 { state = 226; lexer.advance(false); continue; }
                return result;
            }
            119 => {
                if lookahead == 101 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            120 => {
                if lookahead == 101 { state = 228; lexer.advance(false); continue; }
                return result;
            }
            121 => {
                if lookahead == 109 { state = 229; lexer.advance(false); continue; }
                return result;
            }
            122 => {
                if lookahead == 108 { state = 230; lexer.advance(false); continue; }
                if lookahead == 111 { state = 231; lexer.advance(false); continue; }
                return result;
            }
            123 => {
                if lookahead == 101 { state = 232; lexer.advance(false); continue; }
                return result;
            }
            124 => {
                if lookahead == 97 { state = 233; lexer.advance(false); continue; }
                return result;
            }
            125 => {
                result = true; lexer.set_result_symbol(anon_sym_for); lexer.mark_end();
                return result;
            }
            126 => {
                if lookahead == 101 { state = 234; lexer.advance(false); continue; }
                return result;
            }
            127 => {
                if lookahead == 111 { state = 235; lexer.advance(false); continue; }
                return result;
            }
            128 => {
                if lookahead == 111 { state = 236; lexer.advance(false); continue; }
                return result;
            }
            129 => {
                if lookahead == 105 { state = 237; lexer.advance(false); continue; }
                return result;
            }
            130 => {
                if lookahead == 103 { state = 238; lexer.advance(false); continue; }
                return result;
            }
            131 => {
                if lookahead == 117 { state = 239; lexer.advance(false); continue; }
                return result;
            }
            132 => {
                if lookahead == 97 { state = 240; lexer.advance(false); continue; }
                return result;
            }
            133 => {
                if lookahead == 101 { state = 241; lexer.advance(false); continue; }
                return result;
            }
            134 => {
                result = true; lexer.set_result_symbol(anon_sym_new); lexer.mark_end();
                return result;
            }
            135 => {
                if lookahead == 120 { state = 242; lexer.advance(false); continue; }
                return result;
            }
            136 => {
                if lookahead == 101 { state = 243; lexer.advance(false); continue; }
                return result;
            }
            137 => {
                result = true; lexer.set_result_symbol(anon_sym_not); lexer.mark_end();
                if lookahead == 95 { state = 244; lexer.advance(false); continue; }
                return result;
            }
            138 => {
                if lookahead == 108 { state = 245; lexer.advance(false); continue; }
                return result;
            }
            139 => {
                if lookahead == 115 { state = 246; lexer.advance(false); continue; }
                return result;
            }
            140 => {
                if lookahead == 114 { state = 247; lexer.advance(false); continue; }
                return result;
            }
            141 => {
                if lookahead == 101 { state = 248; lexer.advance(false); continue; }
                return result;
            }
            142 => {
                if lookahead == 114 { state = 249; lexer.advance(false); continue; }
                return result;
            }
            143 => {
                if lookahead == 118 { state = 250; lexer.advance(false); continue; }
                return result;
            }
            144 => {
                if lookahead == 116 { state = 251; lexer.advance(false); continue; }
                return result;
            }
            145 => {
                if lookahead == 108 { state = 252; lexer.advance(false); continue; }
                return result;
            }
            146 => {
                if lookahead == 105 { state = 253; lexer.advance(false); continue; }
                return result;
            }
            147 => {
                if lookahead == 117 { state = 254; lexer.advance(false); continue; }
                return result;
            }
            148 => {
                if lookahead == 116 { state = 255; lexer.advance(false); continue; }
                return result;
            }
            149 => {
                if lookahead == 117 { state = 256; lexer.advance(false); continue; }
                return result;
            }
            150 => {
                if lookahead == 114 { state = 257; lexer.advance(false); continue; }
                return result;
            }
            151 => {
                if lookahead == 110 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            152 => {
                if lookahead == 101 { state = 259; lexer.advance(false); continue; }
                return result;
            }
            153 => {
                if lookahead == 116 { state = 260; lexer.advance(false); continue; }
                return result;
            }
            154 => {
                if lookahead == 117 { state = 261; lexer.advance(false); continue; }
                return result;
            }
            155 => {
                if lookahead == 116 { state = 262; lexer.advance(false); continue; }
                return result;
            }
            156 => {
                if lookahead == 112 { state = 263; lexer.advance(false); continue; }
                return result;
            }
            157 => {
                if lookahead == 115 { state = 264; lexer.advance(false); continue; }
                return result;
            }
            158 => {
                if lookahead == 101 { state = 265; lexer.advance(false); continue; }
                if lookahead == 111 { state = 266; lexer.advance(false); continue; }
                return result;
            }
            159 => {
                result = true; lexer.set_result_symbol(anon_sym_try); lexer.mark_end();
                return result;
            }
            160 => {
                if lookahead == 101 { state = 267; lexer.advance(false); continue; }
                return result;
            }
            161 => {
                if lookahead == 111 { state = 268; lexer.advance(false); continue; }
                return result;
            }
            162 => {
                if lookahead == 105 { state = 269; lexer.advance(false); continue; }
                return result;
            }
            163 => {
                if lookahead == 110 { state = 270; lexer.advance(false); continue; }
                return result;
            }
            164 => {
                if lookahead == 116 { state = 271; lexer.advance(false); continue; }
                return result;
            }
            165 => {
                if lookahead == 97 { state = 272; lexer.advance(false); continue; }
                return result;
            }
            166 => {
                if lookahead == 108 { state = 273; lexer.advance(false); continue; }
                return result;
            }
            167 => {
                result = true; lexer.set_result_symbol(anon_sym_xor); lexer.mark_end();
                if lookahead == 95 { state = 274; lexer.advance(false); continue; }
                return result;
            }
            168 => {
                result = true; lexer.set_result_symbol(anon_sym_NULL); lexer.mark_end();
                return result;
            }
            169 => {
                if lookahead == 103 { state = 275; lexer.advance(false); continue; }
                return result;
            }
            170 => {
                if lookahead == 109 { state = 276; lexer.advance(false); continue; }
                return result;
            }
            171 => {
                if lookahead == 101 { state = 277; lexer.advance(false); continue; }
                return result;
            }
            172 => {
                if lookahead == 110 { state = 278; lexer.advance(false); continue; }
                return result;
            }
            173 => {
                if lookahead == 101 { state = 279; lexer.advance(false); continue; }
                return result;
            }
            174 => {
                if lookahead == 105 { state = 280; lexer.advance(false); continue; }
                return result;
            }
            175 => {
                if lookahead == 109 { state = 281; lexer.advance(false); continue; }
                return result;
            }
            176 => {
                if lookahead == 116 { state = 282; lexer.advance(false); continue; }
                return result;
            }
            177 => {
                if lookahead == 115 { state = 283; lexer.advance(false); continue; }
                return result;
            }
            178 => {
                if lookahead == 101 { state = 284; lexer.advance(false); continue; }
                return result;
            }
            179 => {
                if lookahead == 114 { state = 285; lexer.advance(false); continue; }
                return result;
            }
            180 => {
                if lookahead == 110 { state = 286; lexer.advance(false); continue; }
                return result;
            }
            181 => {
                if lookahead == 99 { state = 287; lexer.advance(false); continue; }
                if lookahead == 118 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            182 => {
                if lookahead == 99 { state = 289; lexer.advance(false); continue; }
                if lookahead == 116 { state = 290; lexer.advance(false); continue; }
                return result;
            }
            183 => {
                if lookahead == 115 { state = 291; lexer.advance(false); continue; }
                return result;
            }
            184 => {
                if lookahead == 110 { state = 292; lexer.advance(false); continue; }
                return result;
            }
            185 => {
                if lookahead == 114 { state = 293; lexer.advance(false); continue; }
                return result;
            }
            186 => {
                if lookahead == 111 { state = 294; lexer.advance(false); continue; }
                return result;
            }
            187 => {
                if lookahead == 105 { state = 295; lexer.advance(false); continue; }
                return result;
            }
            188 => {
                if lookahead == 115 { state = 296; lexer.advance(false); continue; }
                return result;
            }
            189 => {
                if lookahead == 108 { state = 297; lexer.advance(false); continue; }
                return result;
            }
            190 => {
                if lookahead == 117 { state = 298; lexer.advance(false); continue; }
                return result;
            }
            191 => {
                if lookahead == 97 { state = 299; lexer.advance(false); continue; }
                return result;
            }
            192 => {
                if lookahead == 99 { state = 300; lexer.advance(false); continue; }
                return result;
            }
            193 => {
                if lookahead == 110 { state = 301; lexer.advance(false); continue; }
                return result;
            }
            194 => {
                if lookahead == 105 { state = 302; lexer.advance(false); continue; }
                return result;
            }
            195 => {
                if lookahead == 115 { state = 303; lexer.advance(false); continue; }
                return result;
            }
            196 => {
                if lookahead == 97 { state = 304; lexer.advance(false); continue; }
                return result;
            }
            197 => {
                if lookahead == 116 { state = 305; lexer.advance(false); continue; }
                return result;
            }
            198 => {
                if lookahead == 100 { state = 306; lexer.advance(false); continue; }
                return result;
            }
            199 => {
                if lookahead == 105 { state = 307; lexer.advance(false); continue; }
                if lookahead == 114 { state = 308; lexer.advance(false); continue; }
                return result;
            }
            200 => {
                if lookahead == 108 { state = 309; lexer.advance(false); continue; }
                return result;
            }
            201 => {
                if lookahead == 121 { state = 310; lexer.advance(false); continue; }
                return result;
            }
            202 => {
                if lookahead == 97 { state = 311; lexer.advance(false); continue; }
                return result;
            }
            203 => {
                if lookahead == 116 { state = 312; lexer.advance(false); continue; }
                return result;
            }
            204 => {
                if lookahead == 99 { state = 313; lexer.advance(false); continue; }
                return result;
            }
            205 => {
                if lookahead == 108 { state = 314; lexer.advance(false); continue; }
                return result;
            }
            206 => {
                if lookahead == 103 { state = 315; lexer.advance(false); continue; }
                return result;
            }
            207 => {
                if lookahead == 108 { state = 316; lexer.advance(false); continue; }
                return result;
            }
            208 => {
                if lookahead == 110 { state = 317; lexer.advance(false); continue; }
                return result;
            }
            209 => {
                if lookahead == 101 { state = 318; lexer.advance(false); continue; }
                return result;
            }
            210 => {
                result = true; lexer.set_result_symbol(sym_auto); lexer.mark_end();
                return result;
            }
            211 => {
                if lookahead == 110 { state = 319; lexer.advance(false); continue; }
                return result;
            }
            212 => {
                if lookahead == 114 { state = 320; lexer.advance(false); continue; }
                return result;
            }
            213 => {
                if lookahead == 107 { state = 321; lexer.advance(false); continue; }
                return result;
            }
            214 => {
                result = true; lexer.set_result_symbol(anon_sym_case); lexer.mark_end();
                return result;
            }
            215 => {
                if lookahead == 104 { state = 322; lexer.advance(false); continue; }
                return result;
            }
            216 => {
                if lookahead == 115 { state = 323; lexer.advance(false); continue; }
                return result;
            }
            217 => {
                if lookahead == 119 { state = 324; lexer.advance(false); continue; }
                return result;
            }
            218 => {
                if lookahead == 101 { state = 325; lexer.advance(false); continue; }
                return result;
            }
            219 => {
                if lookahead == 105 { state = 326; lexer.advance(false); continue; }
                return result;
            }
            220 => {
                if lookahead == 108 { state = 327; lexer.advance(false); continue; }
                return result;
            }
            221 => {
                if lookahead == 101 { state = 328; lexer.advance(false); continue; }
                return result;
            }
            222 => {
                if lookahead == 116 { state = 329; lexer.advance(false); continue; }
                return result;
            }
            223 => {
                if lookahead == 105 { state = 330; lexer.advance(false); continue; }
                return result;
            }
            224 => {
                if lookahead == 116 { state = 331; lexer.advance(false); continue; }
                return result;
            }
            225 => {
                if lookahead == 117 { state = 332; lexer.advance(false); continue; }
                return result;
            }
            226 => {
                if lookahead == 110 { state = 333; lexer.advance(false); continue; }
                return result;
            }
            227 => {
                if lookahead == 116 { state = 334; lexer.advance(false); continue; }
                return result;
            }
            228 => {
                result = true; lexer.set_result_symbol(anon_sym_else); lexer.mark_end();
                return result;
            }
            229 => {
                result = true; lexer.set_result_symbol(anon_sym_enum); lexer.mark_end();
                return result;
            }
            230 => {
                if lookahead == 105 { state = 335; lexer.advance(false); continue; }
                return result;
            }
            231 => {
                if lookahead == 114 { state = 336; lexer.advance(false); continue; }
                return result;
            }
            232 => {
                if lookahead == 114 { state = 337; lexer.advance(false); continue; }
                return result;
            }
            233 => {
                if lookahead == 108 { state = 338; lexer.advance(false); continue; }
                return result;
            }
            234 => {
                if lookahead == 110 { state = 339; lexer.advance(false); continue; }
                return result;
            }
            235 => {
                result = true; lexer.set_result_symbol(anon_sym_goto); lexer.mark_end();
                return result;
            }
            236 => {
                if lookahead == 114 { state = 340; lexer.advance(false); continue; }
                return result;
            }
            237 => {
                if lookahead == 110 { state = 341; lexer.advance(false); continue; }
                return result;
            }
            238 => {
                result = true; lexer.set_result_symbol(anon_sym_long); lexer.mark_end();
                return result;
            }
            239 => {
                if lookahead == 108 { state = 342; lexer.advance(false); continue; }
                return result;
            }
            240 => {
                if lookahead == 98 { state = 343; lexer.advance(false); continue; }
                return result;
            }
            241 => {
                if lookahead == 115 { state = 344; lexer.advance(false); continue; }
                return result;
            }
            242 => {
                if lookahead == 99 { state = 345; lexer.advance(false); continue; }
                return result;
            }
            243 => {
                if lookahead == 116 { state = 346; lexer.advance(false); continue; }
                return result;
            }
            244 => {
                if lookahead == 101 { state = 347; lexer.advance(false); continue; }
                return result;
            }
            245 => {
                if lookahead == 112 { state = 348; lexer.advance(false); continue; }
                return result;
            }
            246 => {
                if lookahead == 101 { state = 349; lexer.advance(false); continue; }
                return result;
            }
            247 => {
                if lookahead == 97 { state = 350; lexer.advance(false); continue; }
                return result;
            }
            248 => {
                if lookahead == 113 { state = 351; lexer.advance(false); continue; }
                return result;
            }
            249 => {
                if lookahead == 114 { state = 352; lexer.advance(false); continue; }
                return result;
            }
            250 => {
                if lookahead == 97 { state = 353; lexer.advance(false); continue; }
                return result;
            }
            251 => {
                if lookahead == 101 { state = 354; lexer.advance(false); continue; }
                return result;
            }
            252 => {
                if lookahead == 105 { state = 355; lexer.advance(false); continue; }
                return result;
            }
            253 => {
                if lookahead == 115 { state = 356; lexer.advance(false); continue; }
                return result;
            }
            254 => {
                if lookahead == 105 { state = 357; lexer.advance(false); continue; }
                return result;
            }
            255 => {
                if lookahead == 114 { state = 358; lexer.advance(false); continue; }
                return result;
            }
            256 => {
                if lookahead == 114 { state = 359; lexer.advance(false); continue; }
                return result;
            }
            257 => {
                if lookahead == 116 { state = 360; lexer.advance(false); continue; }
                return result;
            }
            258 => {
                if lookahead == 101 { state = 361; lexer.advance(false); continue; }
                return result;
            }
            259 => {
                if lookahead == 111 { state = 362; lexer.advance(false); continue; }
                return result;
            }
            260 => {
                if lookahead == 105 { state = 363; lexer.advance(false); continue; }
                return result;
            }
            261 => {
                if lookahead == 99 { state = 364; lexer.advance(false); continue; }
                return result;
            }
            262 => {
                if lookahead == 99 { state = 365; lexer.advance(false); continue; }
                return result;
            }
            263 => {
                if lookahead == 108 { state = 366; lexer.advance(false); continue; }
                return result;
            }
            264 => {
                result = true; lexer.set_result_symbol(sym_this); lexer.mark_end();
                return result;
            }
            265 => {
                if lookahead == 97 { state = 367; lexer.advance(false); continue; }
                return result;
            }
            266 => {
                if lookahead == 119 { state = 368; lexer.advance(false); continue; }
                return result;
            }
            267 => {
                if lookahead == 100 { state = 369; lexer.advance(false); continue; }
                if lookahead == 110 { state = 370; lexer.advance(false); continue; }
                return result;
            }
            268 => {
                if lookahead == 110 { state = 371; lexer.advance(false); continue; }
                return result;
            }
            269 => {
                if lookahead == 103 { state = 372; lexer.advance(false); continue; }
                return result;
            }
            270 => {
                if lookahead == 103 { state = 373; lexer.advance(false); continue; }
                return result;
            }
            271 => {
                if lookahead == 117 { state = 374; lexer.advance(false); continue; }
                return result;
            }
            272 => {
                if lookahead == 116 { state = 375; lexer.advance(false); continue; }
                return result;
            }
            273 => {
                if lookahead == 101 { state = 376; lexer.advance(false); continue; }
                return result;
            }
            274 => {
                if lookahead == 101 { state = 377; lexer.advance(false); continue; }
                return result;
            }
            275 => {
                if lookahead == 110 { state = 378; lexer.advance(false); continue; }
                return result;
            }
            276 => {
                if lookahead == 105 { state = 379; lexer.advance(false); continue; }
                return result;
            }
            277 => {
                if lookahead == 114 { state = 380; lexer.advance(false); continue; }
                return result;
            }
            278 => {
                if lookahead == 117 { state = 381; lexer.advance(false); continue; }
                return result;
            }
            279 => {
                if lookahead == 116 { state = 382; lexer.advance(false); continue; }
                return result;
            }
            280 => {
                if lookahead == 103 { state = 383; lexer.advance(false); continue; }
                return result;
            }
            281 => {
                result = true; lexer.set_result_symbol(anon_sym___asm); lexer.mark_end();
                if lookahead == 95 { state = 384; lexer.advance(false); continue; }
                return result;
            }
            282 => {
                if lookahead == 114 { state = 385; lexer.advance(false); continue; }
                return result;
            }
            283 => {
                if lookahead == 101 { state = 386; lexer.advance(false); continue; }
                return result;
            }
            284 => {
                if lookahead == 99 { state = 387; lexer.advance(false); continue; }
                return result;
            }
            285 => {
                if lookahead == 99 { state = 388; lexer.advance(false); continue; }
                return result;
            }
            286 => {
                if lookahead == 115 { state = 389; lexer.advance(false); continue; }
                return result;
            }
            287 => {
                if lookahead == 108 { state = 390; lexer.advance(false); continue; }
                return result;
            }
            288 => {
                if lookahead == 105 { state = 391; lexer.advance(false); continue; }
                return result;
            }
            289 => {
                if lookahead == 101 { state = 392; lexer.advance(false); continue; }
                return result;
            }
            290 => {
                if lookahead == 101 { state = 393; lexer.advance(false); continue; }
                return result;
            }
            291 => {
                if lookahead == 116 { state = 394; lexer.advance(false); continue; }
                return result;
            }
            292 => {
                if lookahead == 97 { state = 395; lexer.advance(false); continue; }
                return result;
            }
            293 => {
                if lookahead == 99 { state = 396; lexer.advance(false); continue; }
                return result;
            }
            294 => {
                if lookahead == 98 { state = 397; lexer.advance(false); continue; }
                return result;
            }
            295 => {
                if lookahead == 100 { state = 398; lexer.advance(false); continue; }
                return result;
            }
            296 => {
                if lookahead == 116 { state = 399; lexer.advance(false); continue; }
                return result;
            }
            297 => {
                if lookahead == 105 { state = 400; lexer.advance(false); continue; }
                return result;
            }
            298 => {
                if lookahead == 110 { state = 401; lexer.advance(false); continue; }
                return result;
            }
            299 => {
                if lookahead == 118 { state = 402; lexer.advance(false); continue; }
                return result;
            }
            300 => {
                if lookahead == 97 { state = 403; lexer.advance(false); continue; }
                return result;
            }
            301 => {
                if lookahead == 97 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            302 => {
                if lookahead == 110 { state = 405; lexer.advance(false); continue; }
                return result;
            }
            303 => {
                if lookahead == 116 { state = 406; lexer.advance(false); continue; }
                return result;
            }
            304 => {
                if lookahead == 114 { state = 407; lexer.advance(false); continue; }
                return result;
            }
            305 => {
                if lookahead == 114 { state = 408; lexer.advance(false); continue; }
                return result;
            }
            306 => {
                if lookahead == 99 { state = 409; lexer.advance(false); continue; }
                return result;
            }
            307 => {
                if lookahead == 115 { state = 410; lexer.advance(false); continue; }
                return result;
            }
            308 => {
                if lookahead == 101 { state = 411; lexer.advance(false); continue; }
                return result;
            }
            309 => {
                if lookahead == 101 { state = 412; lexer.advance(false); continue; }
                return result;
            }
            310 => {
                result = true; lexer.set_result_symbol(anon_sym___try); lexer.mark_end();
                return result;
            }
            311 => {
                if lookahead == 108 { state = 413; lexer.advance(false); continue; }
                return result;
            }
            312 => {
                if lookahead == 114 { state = 414; lexer.advance(false); continue; }
                return result;
            }
            313 => {
                if lookahead == 116 { state = 415; lexer.advance(false); continue; }
                return result;
            }
            314 => {
                if lookahead == 97 { state = 416; lexer.advance(false); continue; }
                return result;
            }
            315 => {
                if lookahead == 110 { state = 417; lexer.advance(false); continue; }
                return result;
            }
            316 => {
                if lookahead == 105 { state = 418; lexer.advance(false); continue; }
                return result;
            }
            317 => {
                if lookahead == 97 { state = 419; lexer.advance(false); continue; }
                if lookahead == 111 { state = 420; lexer.advance(false); continue; }
                return result;
            }
            318 => {
                if lookahead == 113 { state = 421; lexer.advance(false); continue; }
                return result;
            }
            319 => {
                if lookahead == 100 { state = 422; lexer.advance(false); continue; }
                return result;
            }
            320 => {
                result = true; lexer.set_result_symbol(anon_sym_bitor); lexer.mark_end();
                return result;
            }
            321 => {
                result = true; lexer.set_result_symbol(anon_sym_break); lexer.mark_end();
                return result;
            }
            322 => {
                result = true; lexer.set_result_symbol(anon_sym_catch); lexer.mark_end();
                return result;
            }
            323 => {
                result = true; lexer.set_result_symbol(anon_sym_class); lexer.mark_end();
                return result;
            }
            324 => {
                if lookahead == 97 { state = 423; lexer.advance(false); continue; }
                return result;
            }
            325 => {
                if lookahead == 116 { state = 424; lexer.advance(false); continue; }
                return result;
            }
            326 => {
                if lookahead == 101 { state = 425; lexer.advance(false); continue; }
                return result;
            }
            327 => {
                result = true; lexer.set_result_symbol(anon_sym_compl); lexer.mark_end();
                return result;
            }
            328 => {
                if lookahead == 112 { state = 426; lexer.advance(false); continue; }
                return result;
            }
            329 => {
                result = true; lexer.set_result_symbol(anon_sym_const); lexer.mark_end();
                if lookahead == 101 { state = 427; lexer.advance(false); continue; }
                if lookahead == 105 { state = 428; lexer.advance(false); continue; }
                return result;
            }
            330 => {
                if lookahead == 110 { state = 429; lexer.advance(false); continue; }
                return result;
            }
            331 => {
                if lookahead == 121 { state = 430; lexer.advance(false); continue; }
                return result;
            }
            332 => {
                if lookahead == 108 { state = 431; lexer.advance(false); continue; }
                return result;
            }
            333 => {
                if lookahead == 101 { state = 432; lexer.advance(false); continue; }
                return result;
            }
            334 => {
                if lookahead == 101 { state = 433; lexer.advance(false); continue; }
                return result;
            }
            335 => {
                if lookahead == 99 { state = 434; lexer.advance(false); continue; }
                return result;
            }
            336 => {
                if lookahead == 116 { state = 435; lexer.advance(false); continue; }
                return result;
            }
            337 => {
                if lookahead == 110 { state = 436; lexer.advance(false); continue; }
                return result;
            }
            338 => {
                result = true; lexer.set_result_symbol(anon_sym_final); lexer.mark_end();
                return result;
            }
            339 => {
                if lookahead == 100 { state = 437; lexer.advance(false); continue; }
                return result;
            }
            340 => {
                if lookahead == 116 { state = 438; lexer.advance(false); continue; }
                return result;
            }
            341 => {
                if lookahead == 101 { state = 439; lexer.advance(false); continue; }
                return result;
            }
            342 => {
                if lookahead == 101 { state = 440; lexer.advance(false); continue; }
                return result;
            }
            343 => {
                if lookahead == 108 { state = 441; lexer.advance(false); continue; }
                return result;
            }
            344 => {
                if lookahead == 112 { state = 442; lexer.advance(false); continue; }
                return result;
            }
            345 => {
                if lookahead == 101 { state = 443; lexer.advance(false); continue; }
                return result;
            }
            346 => {
                if lookahead == 117 { state = 444; lexer.advance(false); continue; }
                return result;
            }
            347 => {
                if lookahead == 113 { state = 445; lexer.advance(false); continue; }
                return result;
            }
            348 => {
                if lookahead == 116 { state = 446; lexer.advance(false); continue; }
                return result;
            }
            349 => {
                if lookahead == 116 { state = 447; lexer.advance(false); continue; }
                return result;
            }
            350 => {
                if lookahead == 116 { state = 448; lexer.advance(false); continue; }
                return result;
            }
            351 => {
                result = true; lexer.set_result_symbol(anon_sym_or_eq); lexer.mark_end();
                return result;
            }
            352 => {
                if lookahead == 105 { state = 449; lexer.advance(false); continue; }
                return result;
            }
            353 => {
                if lookahead == 116 { state = 450; lexer.advance(false); continue; }
                return result;
            }
            354 => {
                if lookahead == 99 { state = 451; lexer.advance(false); continue; }
                return result;
            }
            355 => {
                if lookahead == 99 { state = 452; lexer.advance(false); continue; }
                return result;
            }
            356 => {
                if lookahead == 116 { state = 453; lexer.advance(false); continue; }
                return result;
            }
            357 => {
                if lookahead == 114 { state = 454; lexer.advance(false); continue; }
                return result;
            }
            358 => {
                if lookahead == 105 { state = 455; lexer.advance(false); continue; }
                return result;
            }
            359 => {
                if lookahead == 110 { state = 456; lexer.advance(false); continue; }
                return result;
            }
            360 => {
                result = true; lexer.set_result_symbol(anon_sym_short); lexer.mark_end();
                return result;
            }
            361 => {
                if lookahead == 100 { state = 457; lexer.advance(false); continue; }
                return result;
            }
            362 => {
                if lookahead == 102 { state = 458; lexer.advance(false); continue; }
                return result;
            }
            363 => {
                if lookahead == 99 { state = 459; lexer.advance(false); continue; }
                return result;
            }
            364 => {
                if lookahead == 116 { state = 460; lexer.advance(false); continue; }
                return result;
            }
            365 => {
                if lookahead == 104 { state = 461; lexer.advance(false); continue; }
                return result;
            }
            366 => {
                if lookahead == 97 { state = 462; lexer.advance(false); continue; }
                return result;
            }
            367 => {
                if lookahead == 100 { state = 463; lexer.advance(false); continue; }
                return result;
            }
            368 => {
                result = true; lexer.set_result_symbol(anon_sym_throw); lexer.mark_end();
                return result;
            }
            369 => {
                if lookahead == 101 { state = 464; lexer.advance(false); continue; }
                return result;
            }
            370 => {
                if lookahead == 97 { state = 465; lexer.advance(false); continue; }
                return result;
            }
            371 => {
                result = true; lexer.set_result_symbol(anon_sym_union); lexer.mark_end();
                return result;
            }
            372 => {
                if lookahead == 110 { state = 466; lexer.advance(false); continue; }
                return result;
            }
            373 => {
                result = true; lexer.set_result_symbol(anon_sym_using); lexer.mark_end();
                return result;
            }
            374 => {
                if lookahead == 97 { state = 467; lexer.advance(false); continue; }
                return result;
            }
            375 => {
                if lookahead == 105 { state = 468; lexer.advance(false); continue; }
                return result;
            }
            376 => {
                result = true; lexer.set_result_symbol(anon_sym_while); lexer.mark_end();
                return result;
            }
            377 => {
                if lookahead == 113 { state = 469; lexer.advance(false); continue; }
                return result;
            }
            378 => {
                if lookahead == 97 { state = 470; lexer.advance(false); continue; }
                if lookahead == 111 { state = 471; lexer.advance(false); continue; }
                return result;
            }
            379 => {
                if lookahead == 99 { state = 472; lexer.advance(false); continue; }
                return result;
            }
            380 => {
                if lookahead == 105 { state = 473; lexer.advance(false); continue; }
                return result;
            }
            381 => {
                if lookahead == 108 { state = 474; lexer.advance(false); continue; }
                return result;
            }
            382 => {
                if lookahead == 117 { state = 475; lexer.advance(false); continue; }
                return result;
            }
            383 => {
                if lookahead == 110 { state = 476; lexer.advance(false); continue; }
                return result;
            }
            384 => {
                if lookahead == 95 { state = 477; lexer.advance(false); continue; }
                return result;
            }
            385 => {
                if lookahead == 105 { state = 478; lexer.advance(false); continue; }
                return result;
            }
            386 => {
                if lookahead == 100 { state = 479; lexer.advance(false); continue; }
                return result;
            }
            387 => {
                if lookahead == 108 { state = 480; lexer.advance(false); continue; }
                return result;
            }
            388 => {
                if lookahead == 97 { state = 481; lexer.advance(false); continue; }
                return result;
            }
            389 => {
                if lookahead == 116 { state = 482; lexer.advance(false); continue; }
                return result;
            }
            390 => {
                if lookahead == 115 { state = 483; lexer.advance(false); continue; }
                return result;
            }
            391 => {
                if lookahead == 99 { state = 484; lexer.advance(false); continue; }
                return result;
            }
            392 => {
                if lookahead == 112 { state = 485; lexer.advance(false); continue; }
                return result;
            }
            393 => {
                if lookahead == 110 { state = 486; lexer.advance(false); continue; }
                return result;
            }
            394 => {
                if lookahead == 99 { state = 487; lexer.advance(false); continue; }
                return result;
            }
            395 => {
                if lookahead == 108 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            396 => {
                if lookahead == 101 { state = 489; lexer.advance(false); continue; }
                return result;
            }
            397 => {
                if lookahead == 97 { state = 490; lexer.advance(false); continue; }
                return result;
            }
            398 => {
                if lookahead == 95 { state = 491; lexer.advance(false); continue; }
                return result;
            }
            399 => {
                if lookahead == 95 { state = 492; lexer.advance(false); continue; }
                return result;
            }
            400 => {
                if lookahead == 110 { state = 493; lexer.advance(false); continue; }
                return result;
            }
            401 => {
                if lookahead == 99 { state = 494; lexer.advance(false); continue; }
                return result;
            }
            402 => {
                if lookahead == 101 { state = 495; lexer.advance(false); continue; }
                return result;
            }
            403 => {
                if lookahead == 108 { state = 496; lexer.advance(false); continue; }
                return result;
            }
            404 => {
                if lookahead == 103 { state = 497; lexer.advance(false); continue; }
                return result;
            }
            405 => {
                if lookahead == 108 { state = 498; lexer.advance(false); continue; }
                return result;
            }
            406 => {
                if lookahead == 114 { state = 499; lexer.advance(false); continue; }
                return result;
            }
            407 => {
                if lookahead == 101 { state = 500; lexer.advance(false); continue; }
                return result;
            }
            408 => {
                result = true; lexer.set_result_symbol(sym_ms_signed_ptr_modifier); lexer.mark_end();
                return result;
            }
            409 => {
                if lookahead == 97 { state = 501; lexer.advance(false); continue; }
                return result;
            }
            410 => {
                if lookahead == 99 { state = 502; lexer.advance(false); continue; }
                return result;
            }
            411 => {
                if lookahead == 97 { state = 503; lexer.advance(false); continue; }
                return result;
            }
            412 => {
                if lookahead == 95 { state = 504; lexer.advance(false); continue; }
                return result;
            }
            413 => {
                if lookahead == 105 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            414 => {
                result = true; lexer.set_result_symbol(sym_ms_unsigned_ptr_modifier); lexer.mark_end();
                return result;
            }
            415 => {
                if lookahead == 111 { state = 506; lexer.advance(false); continue; }
                return result;
            }
            416 => {
                if lookahead == 116 { state = 507; lexer.advance(false); continue; }
                return result;
            }
            417 => {
                if lookahead == 111 { state = 508; lexer.advance(false); continue; }
                return result;
            }
            418 => {
                if lookahead == 103 { state = 509; lexer.advance(false); continue; }
                return result;
            }
            419 => {
                if lookahead == 115 { state = 510; lexer.advance(false); continue; }
                return result;
            }
            420 => {
                if lookahead == 102 { state = 511; lexer.advance(false); continue; }
                return result;
            }
            421 => {
                result = true; lexer.set_result_symbol(anon_sym_and_eq); lexer.mark_end();
                return result;
            }
            422 => {
                result = true; lexer.set_result_symbol(anon_sym_bitand); lexer.mark_end();
                return result;
            }
            423 => {
                if lookahead == 105 { state = 512; lexer.advance(false); continue; }
                return result;
            }
            424 => {
                if lookahead == 117 { state = 513; lexer.advance(false); continue; }
                return result;
            }
            425 => {
                if lookahead == 108 { state = 514; lexer.advance(false); continue; }
                return result;
            }
            426 => {
                if lookahead == 116 { state = 515; lexer.advance(false); continue; }
                return result;
            }
            427 => {
                if lookahead == 118 { state = 516; lexer.advance(false); continue; }
                if lookahead == 120 { state = 517; lexer.advance(false); continue; }
                return result;
            }
            428 => {
                if lookahead == 110 { state = 518; lexer.advance(false); continue; }
                return result;
            }
            429 => {
                if lookahead == 117 { state = 519; lexer.advance(false); continue; }
                return result;
            }
            430 => {
                if lookahead == 112 { state = 520; lexer.advance(false); continue; }
                return result;
            }
            431 => {
                if lookahead == 116 { state = 521; lexer.advance(false); continue; }
                return result;
            }
            432 => {
                if lookahead == 100 { state = 522; lexer.advance(false); continue; }
                return result;
            }
            433 => {
                result = true; lexer.set_result_symbol(anon_sym_delete); lexer.mark_end();
                return result;
            }
            434 => {
                if lookahead == 105 { state = 523; lexer.advance(false); continue; }
                return result;
            }
            435 => {
                result = true; lexer.set_result_symbol(anon_sym_export); lexer.mark_end();
                return result;
            }
            436 => {
                result = true; lexer.set_result_symbol(anon_sym_extern); lexer.mark_end();
                return result;
            }
            437 => {
                result = true; lexer.set_result_symbol(anon_sym_friend); lexer.mark_end();
                return result;
            }
            438 => {
                result = true; lexer.set_result_symbol(anon_sym_import); lexer.mark_end();
                return result;
            }
            439 => {
                result = true; lexer.set_result_symbol(anon_sym_inline); lexer.mark_end();
                return result;
            }
            440 => {
                result = true; lexer.set_result_symbol(anon_sym_module); lexer.mark_end();
                return result;
            }
            441 => {
                if lookahead == 101 { state = 524; lexer.advance(false); continue; }
                return result;
            }
            442 => {
                if lookahead == 97 { state = 525; lexer.advance(false); continue; }
                return result;
            }
            443 => {
                if lookahead == 112 { state = 526; lexer.advance(false); continue; }
                return result;
            }
            444 => {
                if lookahead == 114 { state = 527; lexer.advance(false); continue; }
                return result;
            }
            445 => {
                result = true; lexer.set_result_symbol(anon_sym_not_eq); lexer.mark_end();
                return result;
            }
            446 => {
                if lookahead == 114 { state = 528; lexer.advance(false); continue; }
                return result;
            }
            447 => {
                if lookahead == 111 { state = 529; lexer.advance(false); continue; }
                return result;
            }
            448 => {
                if lookahead == 111 { state = 530; lexer.advance(false); continue; }
                return result;
            }
            449 => {
                if lookahead == 100 { state = 531; lexer.advance(false); continue; }
                return result;
            }
            450 => {
                if lookahead == 101 { state = 532; lexer.advance(false); continue; }
                return result;
            }
            451 => {
                if lookahead == 116 { state = 533; lexer.advance(false); continue; }
                return result;
            }
            452 => {
                result = true; lexer.set_result_symbol(anon_sym_public); lexer.mark_end();
                return result;
            }
            453 => {
                if lookahead == 101 { state = 534; lexer.advance(false); continue; }
                return result;
            }
            454 => {
                if lookahead == 101 { state = 535; lexer.advance(false); continue; }
                return result;
            }
            455 => {
                if lookahead == 99 { state = 536; lexer.advance(false); continue; }
                return result;
            }
            456 => {
                result = true; lexer.set_result_symbol(anon_sym_return); lexer.mark_end();
                return result;
            }
            457 => {
                result = true; lexer.set_result_symbol(anon_sym_signed); lexer.mark_end();
                return result;
            }
            458 => {
                result = true; lexer.set_result_symbol(anon_sym_sizeof); lexer.mark_end();
                return result;
            }
            459 => {
                result = true; lexer.set_result_symbol(anon_sym_static); lexer.mark_end();
                if lookahead == 95 { state = 537; lexer.advance(false); continue; }
                return result;
            }
            460 => {
                result = true; lexer.set_result_symbol(anon_sym_struct); lexer.mark_end();
                return result;
            }
            461 => {
                result = true; lexer.set_result_symbol(anon_sym_switch); lexer.mark_end();
                return result;
            }
            462 => {
                if lookahead == 116 { state = 538; lexer.advance(false); continue; }
                return result;
            }
            463 => {
                if lookahead == 95 { state = 539; lexer.advance(false); continue; }
                return result;
            }
            464 => {
                if lookahead == 102 { state = 540; lexer.advance(false); continue; }
                return result;
            }
            465 => {
                if lookahead == 109 { state = 541; lexer.advance(false); continue; }
                return result;
            }
            466 => {
                if lookahead == 101 { state = 542; lexer.advance(false); continue; }
                return result;
            }
            467 => {
                if lookahead == 108 { state = 543; lexer.advance(false); continue; }
                return result;
            }
            468 => {
                if lookahead == 108 { state = 544; lexer.advance(false); continue; }
                return result;
            }
            469 => {
                result = true; lexer.set_result_symbol(anon_sym_xor_eq); lexer.mark_end();
                return result;
            }
            470 => {
                if lookahead == 115 { state = 545; lexer.advance(false); continue; }
                return result;
            }
            471 => {
                if lookahead == 102 { state = 546; lexer.advance(false); continue; }
                return result;
            }
            472 => {
                result = true; lexer.set_result_symbol(anon_sym__Atomic); lexer.mark_end();
                return result;
            }
            473 => {
                if lookahead == 99 { state = 547; lexer.advance(false); continue; }
                return result;
            }
            474 => {
                if lookahead == 108 { state = 548; lexer.advance(false); continue; }
                return result;
            }
            475 => {
                if lookahead == 114 { state = 549; lexer.advance(false); continue; }
                return result;
            }
            476 => {
                if lookahead == 111 { state = 550; lexer.advance(false); continue; }
                return result;
            }
            477 => {
                result = true; lexer.set_result_symbol(anon_sym___asm__); lexer.mark_end();
                return result;
            }
            478 => {
                if lookahead == 98 { state = 551; lexer.advance(false); continue; }
                return result;
            }
            479 => {
                result = true; lexer.set_result_symbol(anon_sym___based); lexer.mark_end();
                return result;
            }
            480 => {
                result = true; lexer.set_result_symbol(anon_sym___cdecl); lexer.mark_end();
                return result;
            }
            481 => {
                if lookahead == 108 { state = 552; lexer.advance(false); continue; }
                return result;
            }
            482 => {
                if lookahead == 97 { state = 553; lexer.advance(false); continue; }
                return result;
            }
            483 => {
                if lookahead == 112 { state = 554; lexer.advance(false); continue; }
                return result;
            }
            484 => {
                if lookahead == 101 { state = 555; lexer.advance(false); continue; }
                return result;
            }
            485 => {
                if lookahead == 116 { state = 556; lexer.advance(false); continue; }
                return result;
            }
            486 => {
                if lookahead == 115 { state = 557; lexer.advance(false); continue; }
                return result;
            }
            487 => {
                if lookahead == 97 { state = 558; lexer.advance(false); continue; }
                return result;
            }
            488 => {
                if lookahead == 108 { state = 559; lexer.advance(false); continue; }
                return result;
            }
            489 => {
                if lookahead == 105 { state = 560; lexer.advance(false); continue; }
                return result;
            }
            490 => {
                if lookahead == 108 { state = 561; lexer.advance(false); continue; }
                return result;
            }
            491 => {
                if lookahead == 99 { state = 562; lexer.advance(false); continue; }
                return result;
            }
            492 => {
                if lookahead == 95 { state = 563; lexer.advance(false); continue; }
                return result;
            }
            493 => {
                if lookahead == 101 { state = 564; lexer.advance(false); continue; }
                return result;
            }
            494 => {
                if lookahead == 104 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            495 => {
                result = true; lexer.set_result_symbol(anon_sym___leave); lexer.mark_end();
                return result;
            }
            496 => {
                if lookahead == 95 { state = 566; lexer.advance(false); continue; }
                return result;
            }
            497 => {
                if lookahead == 101 { state = 567; lexer.advance(false); continue; }
                return result;
            }
            498 => {
                if lookahead == 105 { state = 568; lexer.advance(false); continue; }
                return result;
            }
            499 => {
                if lookahead == 105 { state = 569; lexer.advance(false); continue; }
                return result;
            }
            500 => {
                if lookahead == 100 { state = 570; lexer.advance(false); continue; }
                return result;
            }
            501 => {
                if lookahead == 108 { state = 571; lexer.advance(false); continue; }
                return result;
            }
            502 => {
                if lookahead == 97 { state = 572; lexer.advance(false); continue; }
                return result;
            }
            503 => {
                if lookahead == 100 { state = 573; lexer.advance(false); continue; }
                return result;
            }
            504 => {
                if lookahead == 95 { state = 574; lexer.advance(false); continue; }
                if lookahead == 103 { state = 575; lexer.advance(false); continue; }
                return result;
            }
            505 => {
                if lookahead == 103 { state = 576; lexer.advance(false); continue; }
                return result;
            }
            506 => {
                if lookahead == 114 { state = 577; lexer.advance(false); continue; }
                return result;
            }
            507 => {
                if lookahead == 105 { state = 578; lexer.advance(false); continue; }
                return result;
            }
            508 => {
                if lookahead == 102 { state = 579; lexer.advance(false); continue; }
                return result;
            }
            509 => {
                if lookahead == 110 { state = 580; lexer.advance(false); continue; }
                return result;
            }
            510 => {
                result = true; lexer.set_result_symbol(anon_sym_alignas); lexer.mark_end();
                return result;
            }
            511 => {
                result = true; lexer.set_result_symbol(anon_sym_alignof); lexer.mark_end();
                return result;
            }
            512 => {
                if lookahead == 116 { state = 581; lexer.advance(false); continue; }
                return result;
            }
            513 => {
                if lookahead == 114 { state = 582; lexer.advance(false); continue; }
                return result;
            }
            514 => {
                if lookahead == 100 { state = 583; lexer.advance(false); continue; }
                return result;
            }
            515 => {
                result = true; lexer.set_result_symbol(anon_sym_concept); lexer.mark_end();
                return result;
            }
            516 => {
                if lookahead == 97 { state = 584; lexer.advance(false); continue; }
                return result;
            }
            517 => {
                if lookahead == 112 { state = 585; lexer.advance(false); continue; }
                return result;
            }
            518 => {
                if lookahead == 105 { state = 586; lexer.advance(false); continue; }
                return result;
            }
            519 => {
                if lookahead == 101 { state = 587; lexer.advance(false); continue; }
                return result;
            }
            520 => {
                if lookahead == 101 { state = 588; lexer.advance(false); continue; }
                return result;
            }
            521 => {
                result = true; lexer.set_result_symbol(anon_sym_default); lexer.mark_end();
                return result;
            }
            522 => {
                result = true; lexer.set_result_symbol(anon_sym_defined); lexer.mark_end();
                return result;
            }
            523 => {
                if lookahead == 116 { state = 589; lexer.advance(false); continue; }
                return result;
            }
            524 => {
                result = true; lexer.set_result_symbol(anon_sym_mutable); lexer.mark_end();
                return result;
            }
            525 => {
                if lookahead == 99 { state = 590; lexer.advance(false); continue; }
                return result;
            }
            526 => {
                if lookahead == 116 { state = 591; lexer.advance(false); continue; }
                return result;
            }
            527 => {
                if lookahead == 110 { state = 592; lexer.advance(false); continue; }
                return result;
            }
            528 => {
                result = true; lexer.set_result_symbol(anon_sym_nullptr); lexer.mark_end();
                return result;
            }
            529 => {
                if lookahead == 102 { state = 593; lexer.advance(false); continue; }
                return result;
            }
            530 => {
                if lookahead == 114 { state = 594; lexer.advance(false); continue; }
                return result;
            }
            531 => {
                if lookahead == 101 { state = 595; lexer.advance(false); continue; }
                return result;
            }
            532 => {
                result = true; lexer.set_result_symbol(anon_sym_private); lexer.mark_end();
                return result;
            }
            533 => {
                if lookahead == 101 { state = 596; lexer.advance(false); continue; }
                return result;
            }
            534 => {
                if lookahead == 114 { state = 597; lexer.advance(false); continue; }
                return result;
            }
            535 => {
                if lookahead == 115 { state = 598; lexer.advance(false); continue; }
                return result;
            }
            536 => {
                if lookahead == 116 { state = 599; lexer.advance(false); continue; }
                return result;
            }
            537 => {
                if lookahead == 97 { state = 600; lexer.advance(false); continue; }
                return result;
            }
            538 => {
                if lookahead == 101 { state = 601; lexer.advance(false); continue; }
                return result;
            }
            539 => {
                if lookahead == 108 { state = 602; lexer.advance(false); continue; }
                return result;
            }
            540 => {
                result = true; lexer.set_result_symbol(anon_sym_typedef); lexer.mark_end();
                return result;
            }
            541 => {
                if lookahead == 101 { state = 603; lexer.advance(false); continue; }
                return result;
            }
            542 => {
                if lookahead == 100 { state = 604; lexer.advance(false); continue; }
                return result;
            }
            543 => {
                result = true; lexer.set_result_symbol(anon_sym_virtual); lexer.mark_end();
                return result;
            }
            544 => {
                if lookahead == 101 { state = 605; lexer.advance(false); continue; }
                return result;
            }
            545 => {
                result = true; lexer.set_result_symbol(anon_sym__Alignas); lexer.mark_end();
                return result;
            }
            546 => {
                result = true; lexer.set_result_symbol(anon_sym__Alignof); lexer.mark_end();
                return result;
            }
            547 => {
                result = true; lexer.set_result_symbol(anon_sym__Generic); lexer.mark_end();
                return result;
            }
            548 => {
                result = true; lexer.set_result_symbol(anon_sym__Nonnull); lexer.mark_end();
                return result;
            }
            549 => {
                if lookahead == 110 { state = 606; lexer.advance(false); continue; }
                return result;
            }
            550 => {
                if lookahead == 102 { state = 607; lexer.advance(false); continue; }
                return result;
            }
            551 => {
                if lookahead == 117 { state = 608; lexer.advance(false); continue; }
                return result;
            }
            552 => {
                if lookahead == 108 { state = 609; lexer.advance(false); continue; }
                return result;
            }
            553 => {
                if lookahead == 110 { state = 610; lexer.advance(false); continue; }
                return result;
            }
            554 => {
                if lookahead == 101 { state = 611; lexer.advance(false); continue; }
                return result;
            }
            555 => {
                if lookahead == 95 { state = 612; lexer.advance(false); continue; }
                return result;
            }
            556 => {
                result = true; lexer.set_result_symbol(anon_sym___except); lexer.mark_end();
                return result;
            }
            557 => {
                if lookahead == 105 { state = 613; lexer.advance(false); continue; }
                return result;
            }
            558 => {
                if lookahead == 108 { state = 614; lexer.advance(false); continue; }
                return result;
            }
            559 => {
                if lookahead == 121 { state = 615; lexer.advance(false); continue; }
                return result;
            }
            560 => {
                if lookahead == 110 { state = 616; lexer.advance(false); continue; }
                return result;
            }
            561 => {
                if lookahead == 95 { state = 617; lexer.advance(false); continue; }
                return result;
            }
            562 => {
                if lookahead == 111 { state = 618; lexer.advance(false); continue; }
                return result;
            }
            563 => {
                result = true; lexer.set_result_symbol(anon_sym___host__); lexer.mark_end();
                return result;
            }
            564 => {
                result = true; lexer.set_result_symbol(anon_sym___inline); lexer.mark_end();
                if lookahead == 95 { state = 619; lexer.advance(false); continue; }
                return result;
            }
            565 => {
                if lookahead == 95 { state = 620; lexer.advance(false); continue; }
                return result;
            }
            566 => {
                if lookahead == 95 { state = 621; lexer.advance(false); continue; }
                return result;
            }
            567 => {
                if lookahead == 100 { state = 622; lexer.advance(false); continue; }
                return result;
            }
            568 => {
                if lookahead == 110 { state = 623; lexer.advance(false); continue; }
                return result;
            }
            569 => {
                if lookahead == 99 { state = 624; lexer.advance(false); continue; }
                return result;
            }
            570 => {
                if lookahead == 95 { state = 625; lexer.advance(false); continue; }
                return result;
            }
            571 => {
                if lookahead == 108 { state = 626; lexer.advance(false); continue; }
                return result;
            }
            572 => {
                if lookahead == 108 { state = 627; lexer.advance(false); continue; }
                return result;
            }
            573 => {
                result = true; lexer.set_result_symbol(anon_sym___thread); lexer.mark_end();
                return result;
            }
            574 => {
                result = true; lexer.set_result_symbol(anon_sym___tile__); lexer.mark_end();
                return result;
            }
            575 => {
                if lookahead == 108 { state = 628; lexer.advance(false); continue; }
                return result;
            }
            576 => {
                if lookahead == 110 { state = 629; lexer.advance(false); continue; }
                return result;
            }
            577 => {
                if lookahead == 99 { state = 630; lexer.advance(false); continue; }
                return result;
            }
            578 => {
                if lookahead == 108 { state = 631; lexer.advance(false); continue; }
                return result;
            }
            579 => {
                result = true; lexer.set_result_symbol(anon_sym__alignof); lexer.mark_end();
                return result;
            }
            580 => {
                if lookahead == 101 { state = 632; lexer.advance(false); continue; }
                return result;
            }
            581 => {
                result = true; lexer.set_result_symbol(anon_sym_co_await); lexer.mark_end();
                return result;
            }
            582 => {
                if lookahead == 110 { state = 633; lexer.advance(false); continue; }
                return result;
            }
            583 => {
                result = true; lexer.set_result_symbol(anon_sym_co_yield); lexer.mark_end();
                return result;
            }
            584 => {
                if lookahead == 108 { state = 634; lexer.advance(false); continue; }
                return result;
            }
            585 => {
                if lookahead == 114 { state = 635; lexer.advance(false); continue; }
                return result;
            }
            586 => {
                if lookahead == 116 { state = 636; lexer.advance(false); continue; }
                return result;
            }
            587 => {
                result = true; lexer.set_result_symbol(anon_sym_continue); lexer.mark_end();
                return result;
            }
            588 => {
                result = true; lexer.set_result_symbol(anon_sym_decltype); lexer.mark_end();
                return result;
            }
            589 => {
                result = true; lexer.set_result_symbol(anon_sym_explicit); lexer.mark_end();
                return result;
            }
            590 => {
                if lookahead == 101 { state = 637; lexer.advance(false); continue; }
                return result;
            }
            591 => {
                result = true; lexer.set_result_symbol(anon_sym_noexcept); lexer.mark_end();
                return result;
            }
            592 => {
                result = true; lexer.set_result_symbol(anon_sym_noreturn); lexer.mark_end();
                return result;
            }
            593 => {
                result = true; lexer.set_result_symbol(anon_sym_offsetof); lexer.mark_end();
                return result;
            }
            594 => {
                result = true; lexer.set_result_symbol(anon_sym_operator); lexer.mark_end();
                return result;
            }
            595 => {
                result = true; lexer.set_result_symbol(anon_sym_override); lexer.mark_end();
                return result;
            }
            596 => {
                if lookahead == 100 { state = 638; lexer.advance(false); continue; }
                return result;
            }
            597 => {
                result = true; lexer.set_result_symbol(anon_sym_register); lexer.mark_end();
                return result;
            }
            598 => {
                result = true; lexer.set_result_symbol(anon_sym_requires); lexer.mark_end();
                return result;
            }
            599 => {
                result = true; lexer.set_result_symbol(anon_sym_restrict); lexer.mark_end();
                return result;
            }
            600 => {
                if lookahead == 115 { state = 639; lexer.advance(false); continue; }
                return result;
            }
            601 => {
                result = true; lexer.set_result_symbol(anon_sym_template); lexer.mark_end();
                return result;
            }
            602 => {
                if lookahead == 111 { state = 640; lexer.advance(false); continue; }
                return result;
            }
            603 => {
                result = true; lexer.set_result_symbol(anon_sym_typename); lexer.mark_end();
                return result;
            }
            604 => {
                result = true; lexer.set_result_symbol(anon_sym_unsigned); lexer.mark_end();
                return result;
            }
            605 => {
                result = true; lexer.set_result_symbol(anon_sym_volatile); lexer.mark_end();
                return result;
            }
            606 => {
                result = true; lexer.set_result_symbol(anon_sym__Noreturn); lexer.mark_end();
                return result;
            }
            607 => {
                result = true; lexer.set_result_symbol(anon_sym___alignof); lexer.mark_end();
                if lookahead == 95 { state = 641; lexer.advance(false); continue; }
                return result;
            }
            608 => {
                if lookahead == 116 { state = 642; lexer.advance(false); continue; }
                return result;
            }
            609 => {
                result = true; lexer.set_result_symbol(anon_sym___clrcall); lexer.mark_end();
                return result;
            }
            610 => {
                if lookahead == 116 { state = 643; lexer.advance(false); continue; }
                return result;
            }
            611 => {
                if lookahead == 99 { state = 644; lexer.advance(false); continue; }
                return result;
            }
            612 => {
                if lookahead == 95 { state = 645; lexer.advance(false); continue; }
                return result;
            }
            613 => {
                if lookahead == 111 { state = 646; lexer.advance(false); continue; }
                return result;
            }
            614 => {
                if lookahead == 108 { state = 647; lexer.advance(false); continue; }
                return result;
            }
            615 => {
                result = true; lexer.set_result_symbol(anon_sym___finally); lexer.mark_end();
                return result;
            }
            616 => {
                if lookahead == 108 { state = 648; lexer.advance(false); continue; }
                return result;
            }
            617 => {
                if lookahead == 95 { state = 649; lexer.advance(false); continue; }
                return result;
            }
            618 => {
                if lookahead == 110 { state = 650; lexer.advance(false); continue; }
                return result;
            }
            619 => {
                if lookahead == 95 { state = 651; lexer.advance(false); continue; }
                return result;
            }
            620 => {
                if lookahead == 98 { state = 652; lexer.advance(false); continue; }
                return result;
            }
            621 => {
                result = true; lexer.set_result_symbol(anon_sym___local__); lexer.mark_end();
                return result;
            }
            622 => {
                if lookahead == 95 { state = 653; lexer.advance(false); continue; }
                return result;
            }
            623 => {
                if lookahead == 101 { state = 654; lexer.advance(false); continue; }
                return result;
            }
            624 => {
                if lookahead == 116 { state = 655; lexer.advance(false); continue; }
                return result;
            }
            625 => {
                if lookahead == 95 { state = 656; lexer.advance(false); continue; }
                return result;
            }
            626 => {
                result = true; lexer.set_result_symbol(anon_sym___stdcall); lexer.mark_end();
                return result;
            }
            627 => {
                if lookahead == 108 { state = 657; lexer.advance(false); continue; }
                return result;
            }
            628 => {
                if lookahead == 111 { state = 658; lexer.advance(false); continue; }
                return result;
            }
            629 => {
                if lookahead == 101 { state = 659; lexer.advance(false); continue; }
                return result;
            }
            630 => {
                if lookahead == 97 { state = 660; lexer.advance(false); continue; }
                return result;
            }
            631 => {
                if lookahead == 101 { state = 661; lexer.advance(false); continue; }
                return result;
            }
            632 => {
                if lookahead == 100 { state = 662; lexer.advance(false); continue; }
                return result;
            }
            633 => {
                result = true; lexer.set_result_symbol(anon_sym_co_return); lexer.mark_end();
                return result;
            }
            634 => {
                result = true; lexer.set_result_symbol(anon_sym_consteval); lexer.mark_end();
                return result;
            }
            635 => {
                result = true; lexer.set_result_symbol(anon_sym_constexpr); lexer.mark_end();
                return result;
            }
            636 => {
                result = true; lexer.set_result_symbol(anon_sym_constinit); lexer.mark_end();
                return result;
            }
            637 => {
                result = true; lexer.set_result_symbol(anon_sym_namespace); lexer.mark_end();
                return result;
            }
            638 => {
                result = true; lexer.set_result_symbol(anon_sym_protected); lexer.mark_end();
                return result;
            }
            639 => {
                if lookahead == 115 { state = 663; lexer.advance(false); continue; }
                return result;
            }
            640 => {
                if lookahead == 99 { state = 664; lexer.advance(false); continue; }
                return result;
            }
            641 => {
                if lookahead == 95 { state = 665; lexer.advance(false); continue; }
                return result;
            }
            642 => {
                if lookahead == 101 { state = 666; lexer.advance(false); continue; }
                return result;
            }
            643 => {
                if lookahead == 95 { state = 667; lexer.advance(false); continue; }
                return result;
            }
            644 => {
                result = true; lexer.set_result_symbol(anon_sym___declspec); lexer.mark_end();
                return result;
            }
            645 => {
                result = true; lexer.set_result_symbol(anon_sym___device__); lexer.mark_end();
                return result;
            }
            646 => {
                if lookahead == 110 { state = 668; lexer.advance(false); continue; }
                return result;
            }
            647 => {
                result = true; lexer.set_result_symbol(anon_sym___fastcall); lexer.mark_end();
                return result;
            }
            648 => {
                if lookahead == 105 { state = 669; lexer.advance(false); continue; }
                return result;
            }
            649 => {
                result = true; lexer.set_result_symbol(anon_sym___global__); lexer.mark_end();
                return result;
            }
            650 => {
                if lookahead == 115 { state = 670; lexer.advance(false); continue; }
                return result;
            }
            651 => {
                result = true; lexer.set_result_symbol(anon_sym___inline__); lexer.mark_end();
                return result;
            }
            652 => {
                if lookahead == 111 { state = 671; lexer.advance(false); continue; }
                return result;
            }
            653 => {
                if lookahead == 95 { state = 672; lexer.advance(false); continue; }
                return result;
            }
            654 => {
                if lookahead == 95 { state = 673; lexer.advance(false); continue; }
                return result;
            }
            655 => {
                result = true; lexer.set_result_symbol(sym_ms_restrict_modifier); lexer.mark_end();
                if lookahead == 95 { state = 674; lexer.advance(false); continue; }
                return result;
            }
            656 => {
                result = true; lexer.set_result_symbol(anon_sym___shared__); lexer.mark_end();
                return result;
            }
            657 => {
                result = true; lexer.set_result_symbol(anon_sym___thiscall); lexer.mark_end();
                return result;
            }
            658 => {
                if lookahead == 98 { state = 675; lexer.advance(false); continue; }
                return result;
            }
            659 => {
                if lookahead == 100 { state = 676; lexer.advance(false); continue; }
                return result;
            }
            660 => {
                if lookahead == 108 { state = 677; lexer.advance(false); continue; }
                return result;
            }
            661 => {
                if lookahead == 95 { state = 678; lexer.advance(false); continue; }
                return result;
            }
            662 => {
                result = true; lexer.set_result_symbol(anon_sym__unaligned); lexer.mark_end();
                return result;
            }
            663 => {
                if lookahead == 101 { state = 679; lexer.advance(false); continue; }
                return result;
            }
            664 => {
                if lookahead == 97 { state = 680; lexer.advance(false); continue; }
                return result;
            }
            665 => {
                result = true; lexer.set_result_symbol(anon_sym___alignof__); lexer.mark_end();
                return result;
            }
            666 => {
                result = true; lexer.set_result_symbol(anon_sym___attribute); lexer.mark_end();
                if lookahead == 95 { state = 681; lexer.advance(false); continue; }
                return result;
            }
            667 => {
                if lookahead == 95 { state = 682; lexer.advance(false); continue; }
                return result;
            }
            668 => {
                if lookahead == 95 { state = 683; lexer.advance(false); continue; }
                return result;
            }
            669 => {
                if lookahead == 110 { state = 684; lexer.advance(false); continue; }
                return result;
            }
            670 => {
                if lookahead == 116 { state = 685; lexer.advance(false); continue; }
                return result;
            }
            671 => {
                if lookahead == 117 { state = 686; lexer.advance(false); continue; }
                return result;
            }
            672 => {
                result = true; lexer.set_result_symbol(anon_sym___managed__); lexer.mark_end();
                return result;
            }
            673 => {
                if lookahead == 95 { state = 687; lexer.advance(false); continue; }
                return result;
            }
            674 => {
                if lookahead == 95 { state = 688; lexer.advance(false); continue; }
                return result;
            }
            675 => {
                if lookahead == 97 { state = 689; lexer.advance(false); continue; }
                return result;
            }
            676 => {
                result = true; lexer.set_result_symbol(anon_sym___unaligned); lexer.mark_end();
                return result;
            }
            677 => {
                if lookahead == 108 { state = 690; lexer.advance(false); continue; }
                return result;
            }
            678 => {
                if lookahead == 95 { state = 691; lexer.advance(false); continue; }
                return result;
            }
            679 => {
                if lookahead == 114 { state = 692; lexer.advance(false); continue; }
                return result;
            }
            680 => {
                if lookahead == 108 { state = 693; lexer.advance(false); continue; }
                return result;
            }
            681 => {
                if lookahead == 95 { state = 694; lexer.advance(false); continue; }
                return result;
            }
            682 => {
                result = true; lexer.set_result_symbol(anon_sym___constant__); lexer.mark_end();
                return result;
            }
            683 => {
                if lookahead == 95 { state = 695; lexer.advance(false); continue; }
                return result;
            }
            684 => {
                if lookahead == 101 { state = 696; lexer.advance(false); continue; }
                return result;
            }
            685 => {
                if lookahead == 97 { state = 697; lexer.advance(false); continue; }
                return result;
            }
            686 => {
                if lookahead == 110 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            687 => {
                result = true; lexer.set_result_symbol(anon_sym___noinline__); lexer.mark_end();
                return result;
            }
            688 => {
                result = true; lexer.set_result_symbol(anon_sym___restrict__); lexer.mark_end();
                return result;
            }
            689 => {
                if lookahead == 108 { state = 699; lexer.advance(false); continue; }
                return result;
            }
            690 => {
                result = true; lexer.set_result_symbol(anon_sym___vectorcall); lexer.mark_end();
                return result;
            }
            691 => {
                result = true; lexer.set_result_symbol(anon_sym___volatile__); lexer.mark_end();
                return result;
            }
            692 => {
                if lookahead == 116 { state = 700; lexer.advance(false); continue; }
                return result;
            }
            693 => {
                result = true; lexer.set_result_symbol(anon_sym_thread_local); lexer.mark_end();
                return result;
            }
            694 => {
                result = true; lexer.set_result_symbol(anon_sym___attribute__); lexer.mark_end();
                return result;
            }
            695 => {
                result = true; lexer.set_result_symbol(anon_sym___extension__); lexer.mark_end();
                return result;
            }
            696 => {
                result = true; lexer.set_result_symbol(anon_sym___forceinline); lexer.mark_end();
                if lookahead == 95 { state = 701; lexer.advance(false); continue; }
                return result;
            }
            697 => {
                if lookahead == 110 { state = 702; lexer.advance(false); continue; }
                return result;
            }
            698 => {
                if lookahead == 100 { state = 703; lexer.advance(false); continue; }
                return result;
            }
            699 => {
                if lookahead == 95 { state = 704; lexer.advance(false); continue; }
                return result;
            }
            700 => {
                result = true; lexer.set_result_symbol(anon_sym_static_assert); lexer.mark_end();
                return result;
            }
            701 => {
                if lookahead == 95 { state = 705; lexer.advance(false); continue; }
                return result;
            }
            702 => {
                if lookahead == 116 { state = 706; lexer.advance(false); continue; }
                return result;
            }
            703 => {
                if lookahead == 115 { state = 707; lexer.advance(false); continue; }
                return result;
            }
            704 => {
                if lookahead == 95 { state = 708; lexer.advance(false); continue; }
                return result;
            }
            705 => {
                result = true; lexer.set_result_symbol(anon_sym___forceinline__); lexer.mark_end();
                return result;
            }
            706 => {
                if lookahead == 95 { state = 709; lexer.advance(false); continue; }
                return result;
            }
            707 => {
                if lookahead == 95 { state = 710; lexer.advance(false); continue; }
                return result;
            }
            708 => {
                result = true; lexer.set_result_symbol(anon_sym___tile_global__); lexer.mark_end();
                return result;
            }
            709 => {
                if lookahead == 95 { state = 711; lexer.advance(false); continue; }
                return result;
            }
            710 => {
                if lookahead == 95 { state = 712; lexer.advance(false); continue; }
                return result;
            }
            711 => {
                result = true; lexer.set_result_symbol(anon_sym___grid_constant__); lexer.mark_end();
                return result;
            }
            712 => {
                result = true; lexer.set_result_symbol(anon_sym___launch_bounds__); lexer.mark_end();
                return result;
            }
            _ => return false,
        }
    }
}
