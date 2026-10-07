//! The `haskell` grammar's lexer: `ts_lex` and `ts_lex_keywords`, transliterated from
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

const anon_sym_1: Symbol = 54;
const anon_sym_AT: Symbol = 98;
const anon_sym_BANG: Symbol = 99;
const anon_sym_BQUOTE: Symbol = 82;
const anon_sym_BSLASH: Symbol = 29;
const anon_sym_COLON_COLON: Symbol = 96;
const anon_sym_COMMA: Symbol = 4;
const anon_sym_DASH: Symbol = 39;
const anon_sym_DASH_GT: Symbol = 56;
const anon_sym_DASH_GT_DOT: Symbol = 92;
const anon_sym_DOLLAR: Symbol = 63;
const anon_sym_DOLLAR_DOLLAR: Symbol = 64;
const anon_sym_DOT: Symbol = 13;
const anon_sym_DOT_DOT: Symbol = 17;
const anon_sym_EQ: Symbol = 15;
const anon_sym_EQ_GT: Symbol = 89;
const anon_sym_LBRACE: Symbol = 5;
const anon_sym_LBRACK: Symbol = 65;
const anon_sym_LT_DASH: Symbol = 94;
const anon_sym_PERCENT: Symbol = 101;
const anon_sym_PIPE: Symbol = 28;
const anon_sym_PIPE2: Symbol = 85;
const anon_sym_PIPE_PIPE: Symbol = 73;
const anon_sym_PIPE_PIPE_RBRACK: Symbol = 74;
const anon_sym_PIPE_RBRACK: Symbol = 66;
const anon_sym_POUND: Symbol = 83;
const anon_sym_POUND2: Symbol = 84;
const anon_sym_POUND_RPAREN: Symbol = 105;
const anon_sym_RBRACE: Symbol = 6;
const anon_sym_RBRACK: Symbol = 104;
const anon_sym_SEMI: Symbol = 2;
const anon_sym_SQUOTE: Symbol = 8;
const anon_sym_SQUOTE_SQUOTE: Symbol = 23;
const anon_sym_STAR: Symbol = 9;
const anon_sym_TILDE: Symbol = 100;
const anon_sym__: Symbol = 7;
const anon_sym_anyclass: Symbol = 49;
const anon_sym_as: Symbol = 43;
const anon_sym_by: Symbol = 26;
const anon_sym_case: Symbol = 36;
const anon_sym_cases: Symbol = 38;
const anon_sym_class: Symbol = 53;
const anon_sym_d: Symbol = 72;
const anon_sym_data: Symbol = 51;
const anon_sym_default: Symbol = 52;
const anon_sym_deriving: Symbol = 50;
const anon_sym_do: Symbol = 35;
const anon_sym_e: Symbol = 69;
const anon_sym_else: Symbol = 32;
const anon_sym_export: Symbol = 62;
const anon_sym_family: Symbol = 18;
const anon_sym_forall: Symbol = 11;
const anon_sym_foreign: Symbol = 61;
const anon_sym_group: Symbol = 25;
const anon_sym_hiding: Symbol = 44;
const anon_sym_if: Symbol = 31;
const anon_sym_import: Symbol = 41;
const anon_sym_in: Symbol = 30;
const anon_sym_infix: Symbol = 58;
const anon_sym_infixl: Symbol = 57;
const anon_sym_infixr: Symbol = 55;
const anon_sym_instance: Symbol = 16;
const anon_sym_let: Symbol = 3;
const anon_sym_mdo: Symbol = 34;
const anon_sym_module: Symbol = 45;
const anon_sym_newtype: Symbol = 48;
const anon_sym_nominal: Symbol = 20;
const anon_sym_of: Symbol = 37;
const anon_sym_p: Symbol = 71;
const anon_sym_pattern: Symbol = 40;
const anon_sym_phantom: Symbol = 21;
const anon_sym_qualified: Symbol = 42;
const anon_sym_rec: Symbol = 33;
const anon_sym_representational: Symbol = 19;
const anon_sym_role: Symbol = 22;
const anon_sym_stock: Symbol = 47;
const anon_sym_t: Symbol = 70;
const anon_sym_then: Symbol = 24;
const anon_sym_type: Symbol = 14;
const anon_sym_u2190: Symbol = 95;
const anon_sym_u2192: Symbol = 91;
const anon_sym_u21d2: Symbol = 90;
const anon_sym_u2200: Symbol = 12;
const anon_sym_u2237: Symbol = 97;
const anon_sym_u22b8: Symbol = 93;
const anon_sym_u2605: Symbol = 10;
const anon_sym_u27e6: Symbol = 68;
const anon_sym_u27e7: Symbol = 67;
const anon_sym_using: Symbol = 27;
const anon_sym_via: Symbol = 46;
const anon_sym_where: Symbol = 106;
const aux_sym__paren_close_token1: Symbol = 103;
const aux_sym__paren_open_token1: Symbol = 102;
const aux_sym__token1: Symbol = 107;
const sym__binary_literal: Symbol = 79;
const sym__hex_literal: Symbol = 81;
const sym__integer_literal: Symbol = 78;
const sym__octal_literal: Symbol = 80;
const sym_calling_convention: Symbol = 59;
const sym_char: Symbol = 76;
const sym_float: Symbol = 75;
const sym_implicit_variable: Symbol = 86;
const sym_label: Symbol = 88;
const sym_name: Symbol = 87;
const sym_safety: Symbol = 60;
const sym_string: Symbol = 77;
const sym_variable: Symbol = 1;
const ts_builtin_sym_end: Symbol = 0;

#[rustfmt::skip]
static sym_implicit_variable_character_set_1: [CharacterRange; 926] = [
    CharacterRange::new(39, 39), CharacterRange::new(48, 57), CharacterRange::new(65, 90), CharacterRange::new(95, 95), CharacterRange::new(97, 122), CharacterRange::new(170, 170),
    CharacterRange::new(178, 179), CharacterRange::new(181, 181), CharacterRange::new(185, 186), CharacterRange::new(188, 190), CharacterRange::new(192, 214), CharacterRange::new(216, 246),
    CharacterRange::new(248, 705), CharacterRange::new(710, 721), CharacterRange::new(736, 740), CharacterRange::new(748, 748), CharacterRange::new(750, 750), CharacterRange::new(768, 884),
    CharacterRange::new(886, 887), CharacterRange::new(890, 893), CharacterRange::new(895, 895), CharacterRange::new(902, 902), CharacterRange::new(904, 906), CharacterRange::new(908, 908),
    CharacterRange::new(910, 929), CharacterRange::new(931, 1013), CharacterRange::new(1015, 1153), CharacterRange::new(1155, 1159), CharacterRange::new(1162, 1327), CharacterRange::new(1329, 1366),
    CharacterRange::new(1369, 1369), CharacterRange::new(1376, 1416), CharacterRange::new(1425, 1469), CharacterRange::new(1471, 1471), CharacterRange::new(1473, 1474), CharacterRange::new(1476, 1477),
    CharacterRange::new(1479, 1479), CharacterRange::new(1488, 1514), CharacterRange::new(1519, 1522), CharacterRange::new(1552, 1562), CharacterRange::new(1568, 1641), CharacterRange::new(1646, 1747),
    CharacterRange::new(1749, 1756), CharacterRange::new(1759, 1768), CharacterRange::new(1770, 1788), CharacterRange::new(1791, 1791), CharacterRange::new(1808, 1866), CharacterRange::new(1869, 1969),
    CharacterRange::new(1984, 2037), CharacterRange::new(2042, 2042), CharacterRange::new(2045, 2045), CharacterRange::new(2048, 2093), CharacterRange::new(2112, 2139), CharacterRange::new(2144, 2154),
    CharacterRange::new(2160, 2183), CharacterRange::new(2185, 2190), CharacterRange::new(2199, 2273), CharacterRange::new(2275, 2306), CharacterRange::new(2308, 2362), CharacterRange::new(2364, 2365),
    CharacterRange::new(2369, 2376), CharacterRange::new(2381, 2381), CharacterRange::new(2384, 2403), CharacterRange::new(2406, 2415), CharacterRange::new(2417, 2433), CharacterRange::new(2437, 2444),
    CharacterRange::new(2447, 2448), CharacterRange::new(2451, 2472), CharacterRange::new(2474, 2480), CharacterRange::new(2482, 2482), CharacterRange::new(2486, 2489), CharacterRange::new(2492, 2493),
    CharacterRange::new(2497, 2500), CharacterRange::new(2509, 2510), CharacterRange::new(2524, 2525), CharacterRange::new(2527, 2531), CharacterRange::new(2534, 2545), CharacterRange::new(2548, 2553),
    CharacterRange::new(2556, 2556), CharacterRange::new(2558, 2558), CharacterRange::new(2561, 2562), CharacterRange::new(2565, 2570), CharacterRange::new(2575, 2576), CharacterRange::new(2579, 2600),
    CharacterRange::new(2602, 2608), CharacterRange::new(2610, 2611), CharacterRange::new(2613, 2614), CharacterRange::new(2616, 2617), CharacterRange::new(2620, 2620), CharacterRange::new(2625, 2626),
    CharacterRange::new(2631, 2632), CharacterRange::new(2635, 2637), CharacterRange::new(2641, 2641), CharacterRange::new(2649, 2652), CharacterRange::new(2654, 2654), CharacterRange::new(2662, 2677),
    CharacterRange::new(2689, 2690), CharacterRange::new(2693, 2701), CharacterRange::new(2703, 2705), CharacterRange::new(2707, 2728), CharacterRange::new(2730, 2736), CharacterRange::new(2738, 2739),
    CharacterRange::new(2741, 2745), CharacterRange::new(2748, 2749), CharacterRange::new(2753, 2757), CharacterRange::new(2759, 2760), CharacterRange::new(2765, 2765), CharacterRange::new(2768, 2768),
    CharacterRange::new(2784, 2787), CharacterRange::new(2790, 2799), CharacterRange::new(2809, 2815), CharacterRange::new(2817, 2817), CharacterRange::new(2821, 2828), CharacterRange::new(2831, 2832),
    CharacterRange::new(2835, 2856), CharacterRange::new(2858, 2864), CharacterRange::new(2866, 2867), CharacterRange::new(2869, 2873), CharacterRange::new(2876, 2877), CharacterRange::new(2879, 2879),
    CharacterRange::new(2881, 2884), CharacterRange::new(2893, 2893), CharacterRange::new(2901, 2902), CharacterRange::new(2908, 2909), CharacterRange::new(2911, 2915), CharacterRange::new(2918, 2927),
    CharacterRange::new(2929, 2935), CharacterRange::new(2946, 2947), CharacterRange::new(2949, 2954), CharacterRange::new(2958, 2960), CharacterRange::new(2962, 2965), CharacterRange::new(2969, 2970),
    CharacterRange::new(2972, 2972), CharacterRange::new(2974, 2975), CharacterRange::new(2979, 2980), CharacterRange::new(2984, 2986), CharacterRange::new(2990, 3001), CharacterRange::new(3008, 3008),
    CharacterRange::new(3021, 3021), CharacterRange::new(3024, 3024), CharacterRange::new(3046, 3058), CharacterRange::new(3072, 3072), CharacterRange::new(3076, 3084), CharacterRange::new(3086, 3088),
    CharacterRange::new(3090, 3112), CharacterRange::new(3114, 3129), CharacterRange::new(3132, 3136), CharacterRange::new(3142, 3144), CharacterRange::new(3146, 3149), CharacterRange::new(3157, 3158),
    CharacterRange::new(3160, 3162), CharacterRange::new(3165, 3165), CharacterRange::new(3168, 3171), CharacterRange::new(3174, 3183), CharacterRange::new(3192, 3198), CharacterRange::new(3200, 3201),
    CharacterRange::new(3205, 3212), CharacterRange::new(3214, 3216), CharacterRange::new(3218, 3240), CharacterRange::new(3242, 3251), CharacterRange::new(3253, 3257), CharacterRange::new(3260, 3261),
    CharacterRange::new(3263, 3263), CharacterRange::new(3270, 3270), CharacterRange::new(3276, 3277), CharacterRange::new(3293, 3294), CharacterRange::new(3296, 3299), CharacterRange::new(3302, 3311),
    CharacterRange::new(3313, 3314), CharacterRange::new(3328, 3329), CharacterRange::new(3332, 3340), CharacterRange::new(3342, 3344), CharacterRange::new(3346, 3389), CharacterRange::new(3393, 3396),
    CharacterRange::new(3405, 3406), CharacterRange::new(3412, 3414), CharacterRange::new(3416, 3427), CharacterRange::new(3430, 3448), CharacterRange::new(3450, 3455), CharacterRange::new(3457, 3457),
    CharacterRange::new(3461, 3478), CharacterRange::new(3482, 3505), CharacterRange::new(3507, 3515), CharacterRange::new(3517, 3517), CharacterRange::new(3520, 3526), CharacterRange::new(3530, 3530),
    CharacterRange::new(3538, 3540), CharacterRange::new(3542, 3542), CharacterRange::new(3558, 3567), CharacterRange::new(3585, 3642), CharacterRange::new(3648, 3662), CharacterRange::new(3664, 3673),
    CharacterRange::new(3713, 3714), CharacterRange::new(3716, 3716), CharacterRange::new(3718, 3722), CharacterRange::new(3724, 3747), CharacterRange::new(3749, 3749), CharacterRange::new(3751, 3773),
    CharacterRange::new(3776, 3780), CharacterRange::new(3782, 3782), CharacterRange::new(3784, 3790), CharacterRange::new(3792, 3801), CharacterRange::new(3804, 3807), CharacterRange::new(3840, 3840),
    CharacterRange::new(3864, 3865), CharacterRange::new(3872, 3891), CharacterRange::new(3893, 3893), CharacterRange::new(3895, 3895), CharacterRange::new(3897, 3897), CharacterRange::new(3904, 3911),
    CharacterRange::new(3913, 3948), CharacterRange::new(3953, 3966), CharacterRange::new(3968, 3972), CharacterRange::new(3974, 3991), CharacterRange::new(3993, 4028), CharacterRange::new(4038, 4038),
    CharacterRange::new(4096, 4138), CharacterRange::new(4141, 4144), CharacterRange::new(4146, 4151), CharacterRange::new(4153, 4154), CharacterRange::new(4157, 4169), CharacterRange::new(4176, 4181),
    CharacterRange::new(4184, 4193), CharacterRange::new(4197, 4198), CharacterRange::new(4206, 4226), CharacterRange::new(4229, 4230), CharacterRange::new(4237, 4238), CharacterRange::new(4240, 4249),
    CharacterRange::new(4253, 4253), CharacterRange::new(4256, 4293), CharacterRange::new(4295, 4295), CharacterRange::new(4301, 4301), CharacterRange::new(4304, 4346), CharacterRange::new(4348, 4680),
    CharacterRange::new(4682, 4685), CharacterRange::new(4688, 4694), CharacterRange::new(4696, 4696), CharacterRange::new(4698, 4701), CharacterRange::new(4704, 4744), CharacterRange::new(4746, 4749),
    CharacterRange::new(4752, 4784), CharacterRange::new(4786, 4789), CharacterRange::new(4792, 4798), CharacterRange::new(4800, 4800), CharacterRange::new(4802, 4805), CharacterRange::new(4808, 4822),
    CharacterRange::new(4824, 4880), CharacterRange::new(4882, 4885), CharacterRange::new(4888, 4954), CharacterRange::new(4957, 4959), CharacterRange::new(4969, 4988), CharacterRange::new(4992, 5007),
    CharacterRange::new(5024, 5109), CharacterRange::new(5112, 5117), CharacterRange::new(5121, 5740), CharacterRange::new(5743, 5759), CharacterRange::new(5761, 5786), CharacterRange::new(5792, 5866),
    CharacterRange::new(5870, 5880), CharacterRange::new(5888, 5908), CharacterRange::new(5919, 5939), CharacterRange::new(5952, 5971), CharacterRange::new(5984, 5996), CharacterRange::new(5998, 6000),
    CharacterRange::new(6002, 6003), CharacterRange::new(6016, 6069), CharacterRange::new(6071, 6077), CharacterRange::new(6086, 6086), CharacterRange::new(6089, 6099), CharacterRange::new(6103, 6103),
    CharacterRange::new(6108, 6109), CharacterRange::new(6112, 6121), CharacterRange::new(6128, 6137), CharacterRange::new(6155, 6157), CharacterRange::new(6159, 6169), CharacterRange::new(6176, 6264),
    CharacterRange::new(6272, 6314), CharacterRange::new(6320, 6389), CharacterRange::new(6400, 6430), CharacterRange::new(6432, 6434), CharacterRange::new(6439, 6440), CharacterRange::new(6450, 6450),
    CharacterRange::new(6457, 6459), CharacterRange::new(6470, 6509), CharacterRange::new(6512, 6516), CharacterRange::new(6528, 6571), CharacterRange::new(6576, 6601), CharacterRange::new(6608, 6618),
    CharacterRange::new(6656, 6680), CharacterRange::new(6683, 6683), CharacterRange::new(6688, 6740), CharacterRange::new(6742, 6742), CharacterRange::new(6744, 6750), CharacterRange::new(6752, 6752),
    CharacterRange::new(6754, 6754), CharacterRange::new(6757, 6764), CharacterRange::new(6771, 6780), CharacterRange::new(6783, 6793), CharacterRange::new(6800, 6809), CharacterRange::new(6823, 6823),
    CharacterRange::new(6832, 6845), CharacterRange::new(6847, 6862), CharacterRange::new(6912, 6915), CharacterRange::new(6917, 6964), CharacterRange::new(6966, 6970), CharacterRange::new(6972, 6972),
    CharacterRange::new(6978, 6978), CharacterRange::new(6981, 6988), CharacterRange::new(6992, 7001), CharacterRange::new(7019, 7027), CharacterRange::new(7040, 7041), CharacterRange::new(7043, 7072),
    CharacterRange::new(7074, 7077), CharacterRange::new(7080, 7081), CharacterRange::new(7083, 7142), CharacterRange::new(7144, 7145), CharacterRange::new(7149, 7149), CharacterRange::new(7151, 7153),
    CharacterRange::new(7168, 7203), CharacterRange::new(7212, 7219), CharacterRange::new(7222, 7223), CharacterRange::new(7232, 7241), CharacterRange::new(7245, 7293), CharacterRange::new(7296, 7306),
    CharacterRange::new(7312, 7354), CharacterRange::new(7357, 7359), CharacterRange::new(7376, 7378), CharacterRange::new(7380, 7392), CharacterRange::new(7394, 7414), CharacterRange::new(7416, 7418),
    CharacterRange::new(7424, 7957), CharacterRange::new(7960, 7965), CharacterRange::new(7968, 8005), CharacterRange::new(8008, 8013), CharacterRange::new(8016, 8023), CharacterRange::new(8025, 8025),
    CharacterRange::new(8027, 8027), CharacterRange::new(8029, 8029), CharacterRange::new(8031, 8061), CharacterRange::new(8064, 8116), CharacterRange::new(8118, 8124), CharacterRange::new(8126, 8126),
    CharacterRange::new(8130, 8132), CharacterRange::new(8134, 8140), CharacterRange::new(8144, 8147), CharacterRange::new(8150, 8155), CharacterRange::new(8160, 8172), CharacterRange::new(8178, 8180),
    CharacterRange::new(8182, 8188), CharacterRange::new(8304, 8305), CharacterRange::new(8308, 8313), CharacterRange::new(8319, 8329), CharacterRange::new(8336, 8348), CharacterRange::new(8400, 8412),
    CharacterRange::new(8417, 8417), CharacterRange::new(8421, 8432), CharacterRange::new(8450, 8450), CharacterRange::new(8455, 8455), CharacterRange::new(8458, 8467), CharacterRange::new(8469, 8469),
    CharacterRange::new(8473, 8477), CharacterRange::new(8484, 8484), CharacterRange::new(8486, 8486), CharacterRange::new(8488, 8488), CharacterRange::new(8490, 8493), CharacterRange::new(8495, 8505),
    CharacterRange::new(8508, 8511), CharacterRange::new(8517, 8521), CharacterRange::new(8526, 8526), CharacterRange::new(8528, 8585), CharacterRange::new(9312, 9371), CharacterRange::new(9450, 9471),
    CharacterRange::new(10102, 10131), CharacterRange::new(11264, 11492), CharacterRange::new(11499, 11507), CharacterRange::new(11517, 11517), CharacterRange::new(11520, 11557), CharacterRange::new(11559, 11559),
    CharacterRange::new(11565, 11565), CharacterRange::new(11568, 11623), CharacterRange::new(11631, 11631), CharacterRange::new(11647, 11670), CharacterRange::new(11680, 11686), CharacterRange::new(11688, 11694),
    CharacterRange::new(11696, 11702), CharacterRange::new(11704, 11710), CharacterRange::new(11712, 11718), CharacterRange::new(11720, 11726), CharacterRange::new(11728, 11734), CharacterRange::new(11736, 11742),
    CharacterRange::new(11744, 11775), CharacterRange::new(11823, 11823), CharacterRange::new(12293, 12295), CharacterRange::new(12321, 12333), CharacterRange::new(12337, 12341), CharacterRange::new(12344, 12348),
    CharacterRange::new(12353, 12438), CharacterRange::new(12441, 12442), CharacterRange::new(12445, 12447), CharacterRange::new(12449, 12538), CharacterRange::new(12540, 12543), CharacterRange::new(12549, 12591),
    CharacterRange::new(12593, 12686), CharacterRange::new(12690, 12693), CharacterRange::new(12704, 12735), CharacterRange::new(12784, 12799), CharacterRange::new(12832, 12841), CharacterRange::new(12872, 12879),
    CharacterRange::new(12881, 12895), CharacterRange::new(12928, 12937), CharacterRange::new(12977, 12991), CharacterRange::new(13312, 19903), CharacterRange::new(19968, 42124), CharacterRange::new(42192, 42237),
    CharacterRange::new(42240, 42508), CharacterRange::new(42512, 42539), CharacterRange::new(42560, 42607), CharacterRange::new(42612, 42621), CharacterRange::new(42623, 42737), CharacterRange::new(42775, 42783),
    CharacterRange::new(42786, 42888), CharacterRange::new(42891, 42957), CharacterRange::new(42960, 42961), CharacterRange::new(42963, 42963), CharacterRange::new(42965, 42972), CharacterRange::new(42994, 43042),
    CharacterRange::new(43045, 43046), CharacterRange::new(43052, 43052), CharacterRange::new(43056, 43061), CharacterRange::new(43072, 43123), CharacterRange::new(43138, 43187), CharacterRange::new(43204, 43205),
    CharacterRange::new(43216, 43225), CharacterRange::new(43232, 43255), CharacterRange::new(43259, 43259), CharacterRange::new(43261, 43309), CharacterRange::new(43312, 43345), CharacterRange::new(43360, 43388),
    CharacterRange::new(43392, 43394), CharacterRange::new(43396, 43443), CharacterRange::new(43446, 43449), CharacterRange::new(43452, 43453), CharacterRange::new(43471, 43481), CharacterRange::new(43488, 43518),
    CharacterRange::new(43520, 43566), CharacterRange::new(43569, 43570), CharacterRange::new(43573, 43574), CharacterRange::new(43584, 43596), CharacterRange::new(43600, 43609), CharacterRange::new(43616, 43638),
    CharacterRange::new(43642, 43642), CharacterRange::new(43644, 43644), CharacterRange::new(43646, 43714), CharacterRange::new(43739, 43741), CharacterRange::new(43744, 43754), CharacterRange::new(43756, 43757),
    CharacterRange::new(43762, 43764), CharacterRange::new(43766, 43766), CharacterRange::new(43777, 43782), CharacterRange::new(43785, 43790), CharacterRange::new(43793, 43798), CharacterRange::new(43808, 43814),
    CharacterRange::new(43816, 43822), CharacterRange::new(43824, 43866), CharacterRange::new(43868, 43881), CharacterRange::new(43888, 44002), CharacterRange::new(44005, 44005), CharacterRange::new(44008, 44008),
    CharacterRange::new(44013, 44013), CharacterRange::new(44016, 44025), CharacterRange::new(44032, 55203), CharacterRange::new(55216, 55238), CharacterRange::new(55243, 55291), CharacterRange::new(63744, 64109),
    CharacterRange::new(64112, 64217), CharacterRange::new(64256, 64262), CharacterRange::new(64275, 64279), CharacterRange::new(64285, 64296), CharacterRange::new(64298, 64310), CharacterRange::new(64312, 64316),
    CharacterRange::new(64318, 64318), CharacterRange::new(64320, 64321), CharacterRange::new(64323, 64324), CharacterRange::new(64326, 64433), CharacterRange::new(64467, 64829), CharacterRange::new(64848, 64911),
    CharacterRange::new(64914, 64967), CharacterRange::new(65008, 65019), CharacterRange::new(65024, 65039), CharacterRange::new(65056, 65071), CharacterRange::new(65136, 65140), CharacterRange::new(65142, 65276),
    CharacterRange::new(65296, 65305), CharacterRange::new(65313, 65338), CharacterRange::new(65345, 65370), CharacterRange::new(65382, 65470), CharacterRange::new(65474, 65479), CharacterRange::new(65482, 65487),
    CharacterRange::new(65490, 65495), CharacterRange::new(65498, 65500), CharacterRange::new(65536, 65547), CharacterRange::new(65549, 65574), CharacterRange::new(65576, 65594), CharacterRange::new(65596, 65597),
    CharacterRange::new(65599, 65613), CharacterRange::new(65616, 65629), CharacterRange::new(65664, 65786), CharacterRange::new(65799, 65843), CharacterRange::new(65856, 65912), CharacterRange::new(65930, 65931),
    CharacterRange::new(66045, 66045), CharacterRange::new(66176, 66204), CharacterRange::new(66208, 66256), CharacterRange::new(66272, 66299), CharacterRange::new(66304, 66339), CharacterRange::new(66349, 66378),
    CharacterRange::new(66384, 66426), CharacterRange::new(66432, 66461), CharacterRange::new(66464, 66499), CharacterRange::new(66504, 66511), CharacterRange::new(66513, 66517), CharacterRange::new(66560, 66717),
    CharacterRange::new(66720, 66729), CharacterRange::new(66736, 66771), CharacterRange::new(66776, 66811), CharacterRange::new(66816, 66855), CharacterRange::new(66864, 66915), CharacterRange::new(66928, 66938),
    CharacterRange::new(66940, 66954), CharacterRange::new(66956, 66962), CharacterRange::new(66964, 66965), CharacterRange::new(66967, 66977), CharacterRange::new(66979, 66993), CharacterRange::new(66995, 67001),
    CharacterRange::new(67003, 67004), CharacterRange::new(67008, 67059), CharacterRange::new(67072, 67382), CharacterRange::new(67392, 67413), CharacterRange::new(67424, 67431), CharacterRange::new(67456, 67461),
    CharacterRange::new(67463, 67504), CharacterRange::new(67506, 67514), CharacterRange::new(67584, 67589), CharacterRange::new(67592, 67592), CharacterRange::new(67594, 67637), CharacterRange::new(67639, 67640),
    CharacterRange::new(67644, 67644), CharacterRange::new(67647, 67669), CharacterRange::new(67672, 67702), CharacterRange::new(67705, 67742), CharacterRange::new(67751, 67759), CharacterRange::new(67808, 67826),
    CharacterRange::new(67828, 67829), CharacterRange::new(67835, 67867), CharacterRange::new(67872, 67897), CharacterRange::new(67968, 68023), CharacterRange::new(68028, 68047), CharacterRange::new(68050, 68099),
    CharacterRange::new(68101, 68102), CharacterRange::new(68108, 68115), CharacterRange::new(68117, 68119), CharacterRange::new(68121, 68149), CharacterRange::new(68152, 68154), CharacterRange::new(68159, 68168),
    CharacterRange::new(68192, 68222), CharacterRange::new(68224, 68255), CharacterRange::new(68288, 68295), CharacterRange::new(68297, 68326), CharacterRange::new(68331, 68335), CharacterRange::new(68352, 68405),
    CharacterRange::new(68416, 68437), CharacterRange::new(68440, 68466), CharacterRange::new(68472, 68497), CharacterRange::new(68521, 68527), CharacterRange::new(68608, 68680), CharacterRange::new(68736, 68786),
    CharacterRange::new(68800, 68850), CharacterRange::new(68858, 68903), CharacterRange::new(68912, 68921), CharacterRange::new(68928, 68965), CharacterRange::new(68969, 68973), CharacterRange::new(68975, 68997),
    CharacterRange::new(69216, 69246), CharacterRange::new(69248, 69289), CharacterRange::new(69291, 69292), CharacterRange::new(69296, 69297), CharacterRange::new(69314, 69316), CharacterRange::new(69372, 69415),
    CharacterRange::new(69424, 69460), CharacterRange::new(69488, 69509), CharacterRange::new(69552, 69579), CharacterRange::new(69600, 69622), CharacterRange::new(69633, 69633), CharacterRange::new(69635, 69702),
    CharacterRange::new(69714, 69749), CharacterRange::new(69759, 69761), CharacterRange::new(69763, 69807), CharacterRange::new(69811, 69814), CharacterRange::new(69817, 69818), CharacterRange::new(69826, 69826),
    CharacterRange::new(69840, 69864), CharacterRange::new(69872, 69881), CharacterRange::new(69888, 69931), CharacterRange::new(69933, 69940), CharacterRange::new(69942, 69951), CharacterRange::new(69956, 69956),
    CharacterRange::new(69959, 69959), CharacterRange::new(69968, 70003), CharacterRange::new(70006, 70006), CharacterRange::new(70016, 70017), CharacterRange::new(70019, 70066), CharacterRange::new(70070, 70078),
    CharacterRange::new(70081, 70084), CharacterRange::new(70089, 70092), CharacterRange::new(70095, 70106), CharacterRange::new(70108, 70108), CharacterRange::new(70113, 70132), CharacterRange::new(70144, 70161),
    CharacterRange::new(70163, 70187), CharacterRange::new(70191, 70193), CharacterRange::new(70196, 70196), CharacterRange::new(70198, 70199), CharacterRange::new(70206, 70209), CharacterRange::new(70272, 70278),
    CharacterRange::new(70280, 70280), CharacterRange::new(70282, 70285), CharacterRange::new(70287, 70301), CharacterRange::new(70303, 70312), CharacterRange::new(70320, 70367), CharacterRange::new(70371, 70378),
    CharacterRange::new(70384, 70393), CharacterRange::new(70400, 70401), CharacterRange::new(70405, 70412), CharacterRange::new(70415, 70416), CharacterRange::new(70419, 70440), CharacterRange::new(70442, 70448),
    CharacterRange::new(70450, 70451), CharacterRange::new(70453, 70457), CharacterRange::new(70459, 70461), CharacterRange::new(70464, 70464), CharacterRange::new(70480, 70480), CharacterRange::new(70493, 70497),
    CharacterRange::new(70502, 70508), CharacterRange::new(70512, 70516), CharacterRange::new(70528, 70537), CharacterRange::new(70539, 70539), CharacterRange::new(70542, 70542), CharacterRange::new(70544, 70581),
    CharacterRange::new(70583, 70583), CharacterRange::new(70587, 70592), CharacterRange::new(70606, 70606), CharacterRange::new(70608, 70611), CharacterRange::new(70625, 70626), CharacterRange::new(70656, 70708),
    CharacterRange::new(70712, 70719), CharacterRange::new(70722, 70724), CharacterRange::new(70726, 70730), CharacterRange::new(70736, 70745), CharacterRange::new(70750, 70753), CharacterRange::new(70784, 70831),
    CharacterRange::new(70835, 70840), CharacterRange::new(70842, 70842), CharacterRange::new(70847, 70848), CharacterRange::new(70850, 70853), CharacterRange::new(70855, 70855), CharacterRange::new(70864, 70873),
    CharacterRange::new(71040, 71086), CharacterRange::new(71090, 71093), CharacterRange::new(71100, 71101), CharacterRange::new(71103, 71104), CharacterRange::new(71128, 71133), CharacterRange::new(71168, 71215),
    CharacterRange::new(71219, 71226), CharacterRange::new(71229, 71229), CharacterRange::new(71231, 71232), CharacterRange::new(71236, 71236), CharacterRange::new(71248, 71257), CharacterRange::new(71296, 71339),
    CharacterRange::new(71341, 71341), CharacterRange::new(71344, 71349), CharacterRange::new(71351, 71352), CharacterRange::new(71360, 71369), CharacterRange::new(71376, 71395), CharacterRange::new(71424, 71450),
    CharacterRange::new(71453, 71453), CharacterRange::new(71455, 71455), CharacterRange::new(71458, 71461), CharacterRange::new(71463, 71467), CharacterRange::new(71472, 71483), CharacterRange::new(71488, 71494),
    CharacterRange::new(71680, 71723), CharacterRange::new(71727, 71735), CharacterRange::new(71737, 71738), CharacterRange::new(71840, 71922), CharacterRange::new(71935, 71942), CharacterRange::new(71945, 71945),
    CharacterRange::new(71948, 71955), CharacterRange::new(71957, 71958), CharacterRange::new(71960, 71983), CharacterRange::new(71995, 71996), CharacterRange::new(71998, 71999), CharacterRange::new(72001, 72001),
    CharacterRange::new(72003, 72003), CharacterRange::new(72016, 72025), CharacterRange::new(72096, 72103), CharacterRange::new(72106, 72144), CharacterRange::new(72148, 72151), CharacterRange::new(72154, 72155),
    CharacterRange::new(72160, 72161), CharacterRange::new(72163, 72163), CharacterRange::new(72192, 72248), CharacterRange::new(72250, 72254), CharacterRange::new(72263, 72263), CharacterRange::new(72272, 72278),
    CharacterRange::new(72281, 72342), CharacterRange::new(72344, 72345), CharacterRange::new(72349, 72349), CharacterRange::new(72368, 72440), CharacterRange::new(72640, 72672), CharacterRange::new(72688, 72697),
    CharacterRange::new(72704, 72712), CharacterRange::new(72714, 72750), CharacterRange::new(72752, 72758), CharacterRange::new(72760, 72765), CharacterRange::new(72767, 72768), CharacterRange::new(72784, 72812),
    CharacterRange::new(72818, 72847), CharacterRange::new(72850, 72871), CharacterRange::new(72874, 72880), CharacterRange::new(72882, 72883), CharacterRange::new(72885, 72886), CharacterRange::new(72960, 72966),
    CharacterRange::new(72968, 72969), CharacterRange::new(72971, 73014), CharacterRange::new(73018, 73018), CharacterRange::new(73020, 73021), CharacterRange::new(73023, 73031), CharacterRange::new(73040, 73049),
    CharacterRange::new(73056, 73061), CharacterRange::new(73063, 73064), CharacterRange::new(73066, 73097), CharacterRange::new(73104, 73105), CharacterRange::new(73109, 73109), CharacterRange::new(73111, 73112),
    CharacterRange::new(73120, 73129), CharacterRange::new(73440, 73460), CharacterRange::new(73472, 73474), CharacterRange::new(73476, 73488), CharacterRange::new(73490, 73523), CharacterRange::new(73526, 73530),
    CharacterRange::new(73536, 73536), CharacterRange::new(73538, 73538), CharacterRange::new(73552, 73562), CharacterRange::new(73648, 73648), CharacterRange::new(73664, 73684), CharacterRange::new(73728, 74649),
    CharacterRange::new(74752, 74862), CharacterRange::new(74880, 75075), CharacterRange::new(77712, 77808), CharacterRange::new(77824, 78895), CharacterRange::new(78912, 78933), CharacterRange::new(78944, 82938),
    CharacterRange::new(82944, 83526), CharacterRange::new(90368, 90409), CharacterRange::new(90413, 90425), CharacterRange::new(92160, 92728), CharacterRange::new(92736, 92766), CharacterRange::new(92768, 92777),
    CharacterRange::new(92784, 92862), CharacterRange::new(92864, 92873), CharacterRange::new(92880, 92909), CharacterRange::new(92912, 92916), CharacterRange::new(92928, 92982), CharacterRange::new(92992, 92995),
    CharacterRange::new(93008, 93017), CharacterRange::new(93019, 93025), CharacterRange::new(93027, 93047), CharacterRange::new(93053, 93071), CharacterRange::new(93504, 93548), CharacterRange::new(93552, 93561),
    CharacterRange::new(93760, 93846), CharacterRange::new(93952, 94026), CharacterRange::new(94031, 94032), CharacterRange::new(94095, 94111), CharacterRange::new(94176, 94177), CharacterRange::new(94179, 94180),
    CharacterRange::new(94208, 100343), CharacterRange::new(100352, 101589), CharacterRange::new(101631, 101640), CharacterRange::new(110576, 110579), CharacterRange::new(110581, 110587), CharacterRange::new(110589, 110590),
    CharacterRange::new(110592, 110882), CharacterRange::new(110898, 110898), CharacterRange::new(110928, 110930), CharacterRange::new(110933, 110933), CharacterRange::new(110948, 110951), CharacterRange::new(110960, 111355),
    CharacterRange::new(113664, 113770), CharacterRange::new(113776, 113788), CharacterRange::new(113792, 113800), CharacterRange::new(113808, 113817), CharacterRange::new(113821, 113822), CharacterRange::new(118000, 118009),
    CharacterRange::new(118528, 118573), CharacterRange::new(118576, 118598), CharacterRange::new(119143, 119145), CharacterRange::new(119163, 119170), CharacterRange::new(119173, 119179), CharacterRange::new(119210, 119213),
    CharacterRange::new(119362, 119364), CharacterRange::new(119488, 119507), CharacterRange::new(119520, 119539), CharacterRange::new(119648, 119672), CharacterRange::new(119808, 119892), CharacterRange::new(119894, 119964),
    CharacterRange::new(119966, 119967), CharacterRange::new(119970, 119970), CharacterRange::new(119973, 119974), CharacterRange::new(119977, 119980), CharacterRange::new(119982, 119993), CharacterRange::new(119995, 119995),
    CharacterRange::new(119997, 120003), CharacterRange::new(120005, 120069), CharacterRange::new(120071, 120074), CharacterRange::new(120077, 120084), CharacterRange::new(120086, 120092), CharacterRange::new(120094, 120121),
    CharacterRange::new(120123, 120126), CharacterRange::new(120128, 120132), CharacterRange::new(120134, 120134), CharacterRange::new(120138, 120144), CharacterRange::new(120146, 120485), CharacterRange::new(120488, 120512),
    CharacterRange::new(120514, 120538), CharacterRange::new(120540, 120570), CharacterRange::new(120572, 120596), CharacterRange::new(120598, 120628), CharacterRange::new(120630, 120654), CharacterRange::new(120656, 120686),
    CharacterRange::new(120688, 120712), CharacterRange::new(120714, 120744), CharacterRange::new(120746, 120770), CharacterRange::new(120772, 120779), CharacterRange::new(120782, 120831), CharacterRange::new(121344, 121398),
    CharacterRange::new(121403, 121452), CharacterRange::new(121461, 121461), CharacterRange::new(121476, 121476), CharacterRange::new(121499, 121503), CharacterRange::new(121505, 121519), CharacterRange::new(122624, 122654),
    CharacterRange::new(122661, 122666), CharacterRange::new(122880, 122886), CharacterRange::new(122888, 122904), CharacterRange::new(122907, 122913), CharacterRange::new(122915, 122916), CharacterRange::new(122918, 122922),
    CharacterRange::new(122928, 122989), CharacterRange::new(123023, 123023), CharacterRange::new(123136, 123180), CharacterRange::new(123184, 123197), CharacterRange::new(123200, 123209), CharacterRange::new(123214, 123214),
    CharacterRange::new(123536, 123566), CharacterRange::new(123584, 123641), CharacterRange::new(124112, 124153), CharacterRange::new(124368, 124410), CharacterRange::new(124896, 124902), CharacterRange::new(124904, 124907),
    CharacterRange::new(124909, 124910), CharacterRange::new(124912, 124926), CharacterRange::new(124928, 125124), CharacterRange::new(125127, 125142), CharacterRange::new(125184, 125259), CharacterRange::new(125264, 125273),
    CharacterRange::new(126065, 126123), CharacterRange::new(126125, 126127), CharacterRange::new(126129, 126132), CharacterRange::new(126209, 126253), CharacterRange::new(126255, 126269), CharacterRange::new(126464, 126467),
    CharacterRange::new(126469, 126495), CharacterRange::new(126497, 126498), CharacterRange::new(126500, 126500), CharacterRange::new(126503, 126503), CharacterRange::new(126505, 126514), CharacterRange::new(126516, 126519),
    CharacterRange::new(126521, 126521), CharacterRange::new(126523, 126523), CharacterRange::new(126530, 126530), CharacterRange::new(126535, 126535), CharacterRange::new(126537, 126537), CharacterRange::new(126539, 126539),
    CharacterRange::new(126541, 126543), CharacterRange::new(126545, 126546), CharacterRange::new(126548, 126548), CharacterRange::new(126551, 126551), CharacterRange::new(126553, 126553), CharacterRange::new(126555, 126555),
    CharacterRange::new(126557, 126557), CharacterRange::new(126559, 126559), CharacterRange::new(126561, 126562), CharacterRange::new(126564, 126564), CharacterRange::new(126567, 126570), CharacterRange::new(126572, 126578),
    CharacterRange::new(126580, 126583), CharacterRange::new(126585, 126588), CharacterRange::new(126590, 126590), CharacterRange::new(126592, 126601), CharacterRange::new(126603, 126619), CharacterRange::new(126625, 126627),
    CharacterRange::new(126629, 126633), CharacterRange::new(126635, 126651), CharacterRange::new(127232, 127244), CharacterRange::new(130032, 130041), CharacterRange::new(131072, 173791), CharacterRange::new(173824, 177977),
    CharacterRange::new(177984, 178205), CharacterRange::new(178208, 183969), CharacterRange::new(183984, 191456), CharacterRange::new(191472, 192093), CharacterRange::new(194560, 195101), CharacterRange::new(196608, 201546),
    CharacterRange::new(201552, 205743), CharacterRange::new(917760, 917999),
];

#[rustfmt::skip]
static sym_name_character_set_1: [CharacterRange; 654] = [
    CharacterRange::new(65, 90), CharacterRange::new(192, 214), CharacterRange::new(216, 222), CharacterRange::new(256, 256), CharacterRange::new(258, 258), CharacterRange::new(260, 260),
    CharacterRange::new(262, 262), CharacterRange::new(264, 264), CharacterRange::new(266, 266), CharacterRange::new(268, 268), CharacterRange::new(270, 270), CharacterRange::new(272, 272),
    CharacterRange::new(274, 274), CharacterRange::new(276, 276), CharacterRange::new(278, 278), CharacterRange::new(280, 280), CharacterRange::new(282, 282), CharacterRange::new(284, 284),
    CharacterRange::new(286, 286), CharacterRange::new(288, 288), CharacterRange::new(290, 290), CharacterRange::new(292, 292), CharacterRange::new(294, 294), CharacterRange::new(296, 296),
    CharacterRange::new(298, 298), CharacterRange::new(300, 300), CharacterRange::new(302, 302), CharacterRange::new(304, 304), CharacterRange::new(306, 306), CharacterRange::new(308, 308),
    CharacterRange::new(310, 310), CharacterRange::new(313, 313), CharacterRange::new(315, 315), CharacterRange::new(317, 317), CharacterRange::new(319, 319), CharacterRange::new(321, 321),
    CharacterRange::new(323, 323), CharacterRange::new(325, 325), CharacterRange::new(327, 327), CharacterRange::new(330, 330), CharacterRange::new(332, 332), CharacterRange::new(334, 334),
    CharacterRange::new(336, 336), CharacterRange::new(338, 338), CharacterRange::new(340, 340), CharacterRange::new(342, 342), CharacterRange::new(344, 344), CharacterRange::new(346, 346),
    CharacterRange::new(348, 348), CharacterRange::new(350, 350), CharacterRange::new(352, 352), CharacterRange::new(354, 354), CharacterRange::new(356, 356), CharacterRange::new(358, 358),
    CharacterRange::new(360, 360), CharacterRange::new(362, 362), CharacterRange::new(364, 364), CharacterRange::new(366, 366), CharacterRange::new(368, 368), CharacterRange::new(370, 370),
    CharacterRange::new(372, 372), CharacterRange::new(374, 374), CharacterRange::new(376, 377), CharacterRange::new(379, 379), CharacterRange::new(381, 381), CharacterRange::new(385, 386),
    CharacterRange::new(388, 388), CharacterRange::new(390, 391), CharacterRange::new(393, 395), CharacterRange::new(398, 401), CharacterRange::new(403, 404), CharacterRange::new(406, 408),
    CharacterRange::new(412, 413), CharacterRange::new(415, 416), CharacterRange::new(418, 418), CharacterRange::new(420, 420), CharacterRange::new(422, 423), CharacterRange::new(425, 425),
    CharacterRange::new(428, 428), CharacterRange::new(430, 431), CharacterRange::new(433, 435), CharacterRange::new(437, 437), CharacterRange::new(439, 440), CharacterRange::new(444, 444),
    CharacterRange::new(452, 453), CharacterRange::new(455, 456), CharacterRange::new(458, 459), CharacterRange::new(461, 461), CharacterRange::new(463, 463), CharacterRange::new(465, 465),
    CharacterRange::new(467, 467), CharacterRange::new(469, 469), CharacterRange::new(471, 471), CharacterRange::new(473, 473), CharacterRange::new(475, 475), CharacterRange::new(478, 478),
    CharacterRange::new(480, 480), CharacterRange::new(482, 482), CharacterRange::new(484, 484), CharacterRange::new(486, 486), CharacterRange::new(488, 488), CharacterRange::new(490, 490),
    CharacterRange::new(492, 492), CharacterRange::new(494, 494), CharacterRange::new(497, 498), CharacterRange::new(500, 500), CharacterRange::new(502, 504), CharacterRange::new(506, 506),
    CharacterRange::new(508, 508), CharacterRange::new(510, 510), CharacterRange::new(512, 512), CharacterRange::new(514, 514), CharacterRange::new(516, 516), CharacterRange::new(518, 518),
    CharacterRange::new(520, 520), CharacterRange::new(522, 522), CharacterRange::new(524, 524), CharacterRange::new(526, 526), CharacterRange::new(528, 528), CharacterRange::new(530, 530),
    CharacterRange::new(532, 532), CharacterRange::new(534, 534), CharacterRange::new(536, 536), CharacterRange::new(538, 538), CharacterRange::new(540, 540), CharacterRange::new(542, 542),
    CharacterRange::new(544, 544), CharacterRange::new(546, 546), CharacterRange::new(548, 548), CharacterRange::new(550, 550), CharacterRange::new(552, 552), CharacterRange::new(554, 554),
    CharacterRange::new(556, 556), CharacterRange::new(558, 558), CharacterRange::new(560, 560), CharacterRange::new(562, 562), CharacterRange::new(570, 571), CharacterRange::new(573, 574),
    CharacterRange::new(577, 577), CharacterRange::new(579, 582), CharacterRange::new(584, 584), CharacterRange::new(586, 586), CharacterRange::new(588, 588), CharacterRange::new(590, 590),
    CharacterRange::new(880, 880), CharacterRange::new(882, 882), CharacterRange::new(886, 886), CharacterRange::new(895, 895), CharacterRange::new(902, 902), CharacterRange::new(904, 906),
    CharacterRange::new(908, 908), CharacterRange::new(910, 911), CharacterRange::new(913, 929), CharacterRange::new(931, 939), CharacterRange::new(975, 975), CharacterRange::new(978, 980),
    CharacterRange::new(984, 984), CharacterRange::new(986, 986), CharacterRange::new(988, 988), CharacterRange::new(990, 990), CharacterRange::new(992, 992), CharacterRange::new(994, 994),
    CharacterRange::new(996, 996), CharacterRange::new(998, 998), CharacterRange::new(1000, 1000), CharacterRange::new(1002, 1002), CharacterRange::new(1004, 1004), CharacterRange::new(1006, 1006),
    CharacterRange::new(1012, 1012), CharacterRange::new(1015, 1015), CharacterRange::new(1017, 1018), CharacterRange::new(1021, 1071), CharacterRange::new(1120, 1120), CharacterRange::new(1122, 1122),
    CharacterRange::new(1124, 1124), CharacterRange::new(1126, 1126), CharacterRange::new(1128, 1128), CharacterRange::new(1130, 1130), CharacterRange::new(1132, 1132), CharacterRange::new(1134, 1134),
    CharacterRange::new(1136, 1136), CharacterRange::new(1138, 1138), CharacterRange::new(1140, 1140), CharacterRange::new(1142, 1142), CharacterRange::new(1144, 1144), CharacterRange::new(1146, 1146),
    CharacterRange::new(1148, 1148), CharacterRange::new(1150, 1150), CharacterRange::new(1152, 1152), CharacterRange::new(1162, 1162), CharacterRange::new(1164, 1164), CharacterRange::new(1166, 1166),
    CharacterRange::new(1168, 1168), CharacterRange::new(1170, 1170), CharacterRange::new(1172, 1172), CharacterRange::new(1174, 1174), CharacterRange::new(1176, 1176), CharacterRange::new(1178, 1178),
    CharacterRange::new(1180, 1180), CharacterRange::new(1182, 1182), CharacterRange::new(1184, 1184), CharacterRange::new(1186, 1186), CharacterRange::new(1188, 1188), CharacterRange::new(1190, 1190),
    CharacterRange::new(1192, 1192), CharacterRange::new(1194, 1194), CharacterRange::new(1196, 1196), CharacterRange::new(1198, 1198), CharacterRange::new(1200, 1200), CharacterRange::new(1202, 1202),
    CharacterRange::new(1204, 1204), CharacterRange::new(1206, 1206), CharacterRange::new(1208, 1208), CharacterRange::new(1210, 1210), CharacterRange::new(1212, 1212), CharacterRange::new(1214, 1214),
    CharacterRange::new(1216, 1217), CharacterRange::new(1219, 1219), CharacterRange::new(1221, 1221), CharacterRange::new(1223, 1223), CharacterRange::new(1225, 1225), CharacterRange::new(1227, 1227),
    CharacterRange::new(1229, 1229), CharacterRange::new(1232, 1232), CharacterRange::new(1234, 1234), CharacterRange::new(1236, 1236), CharacterRange::new(1238, 1238), CharacterRange::new(1240, 1240),
    CharacterRange::new(1242, 1242), CharacterRange::new(1244, 1244), CharacterRange::new(1246, 1246), CharacterRange::new(1248, 1248), CharacterRange::new(1250, 1250), CharacterRange::new(1252, 1252),
    CharacterRange::new(1254, 1254), CharacterRange::new(1256, 1256), CharacterRange::new(1258, 1258), CharacterRange::new(1260, 1260), CharacterRange::new(1262, 1262), CharacterRange::new(1264, 1264),
    CharacterRange::new(1266, 1266), CharacterRange::new(1268, 1268), CharacterRange::new(1270, 1270), CharacterRange::new(1272, 1272), CharacterRange::new(1274, 1274), CharacterRange::new(1276, 1276),
    CharacterRange::new(1278, 1278), CharacterRange::new(1280, 1280), CharacterRange::new(1282, 1282), CharacterRange::new(1284, 1284), CharacterRange::new(1286, 1286), CharacterRange::new(1288, 1288),
    CharacterRange::new(1290, 1290), CharacterRange::new(1292, 1292), CharacterRange::new(1294, 1294), CharacterRange::new(1296, 1296), CharacterRange::new(1298, 1298), CharacterRange::new(1300, 1300),
    CharacterRange::new(1302, 1302), CharacterRange::new(1304, 1304), CharacterRange::new(1306, 1306), CharacterRange::new(1308, 1308), CharacterRange::new(1310, 1310), CharacterRange::new(1312, 1312),
    CharacterRange::new(1314, 1314), CharacterRange::new(1316, 1316), CharacterRange::new(1318, 1318), CharacterRange::new(1320, 1320), CharacterRange::new(1322, 1322), CharacterRange::new(1324, 1324),
    CharacterRange::new(1326, 1326), CharacterRange::new(1329, 1366), CharacterRange::new(4256, 4293), CharacterRange::new(4295, 4295), CharacterRange::new(4301, 4301), CharacterRange::new(5024, 5109),
    CharacterRange::new(7305, 7305), CharacterRange::new(7312, 7354), CharacterRange::new(7357, 7359), CharacterRange::new(7680, 7680), CharacterRange::new(7682, 7682), CharacterRange::new(7684, 7684),
    CharacterRange::new(7686, 7686), CharacterRange::new(7688, 7688), CharacterRange::new(7690, 7690), CharacterRange::new(7692, 7692), CharacterRange::new(7694, 7694), CharacterRange::new(7696, 7696),
    CharacterRange::new(7698, 7698), CharacterRange::new(7700, 7700), CharacterRange::new(7702, 7702), CharacterRange::new(7704, 7704), CharacterRange::new(7706, 7706), CharacterRange::new(7708, 7708),
    CharacterRange::new(7710, 7710), CharacterRange::new(7712, 7712), CharacterRange::new(7714, 7714), CharacterRange::new(7716, 7716), CharacterRange::new(7718, 7718), CharacterRange::new(7720, 7720),
    CharacterRange::new(7722, 7722), CharacterRange::new(7724, 7724), CharacterRange::new(7726, 7726), CharacterRange::new(7728, 7728), CharacterRange::new(7730, 7730), CharacterRange::new(7732, 7732),
    CharacterRange::new(7734, 7734), CharacterRange::new(7736, 7736), CharacterRange::new(7738, 7738), CharacterRange::new(7740, 7740), CharacterRange::new(7742, 7742), CharacterRange::new(7744, 7744),
    CharacterRange::new(7746, 7746), CharacterRange::new(7748, 7748), CharacterRange::new(7750, 7750), CharacterRange::new(7752, 7752), CharacterRange::new(7754, 7754), CharacterRange::new(7756, 7756),
    CharacterRange::new(7758, 7758), CharacterRange::new(7760, 7760), CharacterRange::new(7762, 7762), CharacterRange::new(7764, 7764), CharacterRange::new(7766, 7766), CharacterRange::new(7768, 7768),
    CharacterRange::new(7770, 7770), CharacterRange::new(7772, 7772), CharacterRange::new(7774, 7774), CharacterRange::new(7776, 7776), CharacterRange::new(7778, 7778), CharacterRange::new(7780, 7780),
    CharacterRange::new(7782, 7782), CharacterRange::new(7784, 7784), CharacterRange::new(7786, 7786), CharacterRange::new(7788, 7788), CharacterRange::new(7790, 7790), CharacterRange::new(7792, 7792),
    CharacterRange::new(7794, 7794), CharacterRange::new(7796, 7796), CharacterRange::new(7798, 7798), CharacterRange::new(7800, 7800), CharacterRange::new(7802, 7802), CharacterRange::new(7804, 7804),
    CharacterRange::new(7806, 7806), CharacterRange::new(7808, 7808), CharacterRange::new(7810, 7810), CharacterRange::new(7812, 7812), CharacterRange::new(7814, 7814), CharacterRange::new(7816, 7816),
    CharacterRange::new(7818, 7818), CharacterRange::new(7820, 7820), CharacterRange::new(7822, 7822), CharacterRange::new(7824, 7824), CharacterRange::new(7826, 7826), CharacterRange::new(7828, 7828),
    CharacterRange::new(7838, 7838), CharacterRange::new(7840, 7840), CharacterRange::new(7842, 7842), CharacterRange::new(7844, 7844), CharacterRange::new(7846, 7846), CharacterRange::new(7848, 7848),
    CharacterRange::new(7850, 7850), CharacterRange::new(7852, 7852), CharacterRange::new(7854, 7854), CharacterRange::new(7856, 7856), CharacterRange::new(7858, 7858), CharacterRange::new(7860, 7860),
    CharacterRange::new(7862, 7862), CharacterRange::new(7864, 7864), CharacterRange::new(7866, 7866), CharacterRange::new(7868, 7868), CharacterRange::new(7870, 7870), CharacterRange::new(7872, 7872),
    CharacterRange::new(7874, 7874), CharacterRange::new(7876, 7876), CharacterRange::new(7878, 7878), CharacterRange::new(7880, 7880), CharacterRange::new(7882, 7882), CharacterRange::new(7884, 7884),
    CharacterRange::new(7886, 7886), CharacterRange::new(7888, 7888), CharacterRange::new(7890, 7890), CharacterRange::new(7892, 7892), CharacterRange::new(7894, 7894), CharacterRange::new(7896, 7896),
    CharacterRange::new(7898, 7898), CharacterRange::new(7900, 7900), CharacterRange::new(7902, 7902), CharacterRange::new(7904, 7904), CharacterRange::new(7906, 7906), CharacterRange::new(7908, 7908),
    CharacterRange::new(7910, 7910), CharacterRange::new(7912, 7912), CharacterRange::new(7914, 7914), CharacterRange::new(7916, 7916), CharacterRange::new(7918, 7918), CharacterRange::new(7920, 7920),
    CharacterRange::new(7922, 7922), CharacterRange::new(7924, 7924), CharacterRange::new(7926, 7926), CharacterRange::new(7928, 7928), CharacterRange::new(7930, 7930), CharacterRange::new(7932, 7932),
    CharacterRange::new(7934, 7934), CharacterRange::new(7944, 7951), CharacterRange::new(7960, 7965), CharacterRange::new(7976, 7983), CharacterRange::new(7992, 7999), CharacterRange::new(8008, 8013),
    CharacterRange::new(8025, 8025), CharacterRange::new(8027, 8027), CharacterRange::new(8029, 8029), CharacterRange::new(8031, 8031), CharacterRange::new(8040, 8047), CharacterRange::new(8072, 8079),
    CharacterRange::new(8088, 8095), CharacterRange::new(8104, 8111), CharacterRange::new(8120, 8124), CharacterRange::new(8136, 8140), CharacterRange::new(8152, 8155), CharacterRange::new(8168, 8172),
    CharacterRange::new(8184, 8188), CharacterRange::new(8450, 8450), CharacterRange::new(8455, 8455), CharacterRange::new(8459, 8461), CharacterRange::new(8464, 8466), CharacterRange::new(8469, 8469),
    CharacterRange::new(8473, 8477), CharacterRange::new(8484, 8484), CharacterRange::new(8486, 8486), CharacterRange::new(8488, 8488), CharacterRange::new(8490, 8493), CharacterRange::new(8496, 8499),
    CharacterRange::new(8510, 8511), CharacterRange::new(8517, 8517), CharacterRange::new(8579, 8579), CharacterRange::new(11264, 11311), CharacterRange::new(11360, 11360), CharacterRange::new(11362, 11364),
    CharacterRange::new(11367, 11367), CharacterRange::new(11369, 11369), CharacterRange::new(11371, 11371), CharacterRange::new(11373, 11376), CharacterRange::new(11378, 11378), CharacterRange::new(11381, 11381),
    CharacterRange::new(11390, 11392), CharacterRange::new(11394, 11394), CharacterRange::new(11396, 11396), CharacterRange::new(11398, 11398), CharacterRange::new(11400, 11400), CharacterRange::new(11402, 11402),
    CharacterRange::new(11404, 11404), CharacterRange::new(11406, 11406), CharacterRange::new(11408, 11408), CharacterRange::new(11410, 11410), CharacterRange::new(11412, 11412), CharacterRange::new(11414, 11414),
    CharacterRange::new(11416, 11416), CharacterRange::new(11418, 11418), CharacterRange::new(11420, 11420), CharacterRange::new(11422, 11422), CharacterRange::new(11424, 11424), CharacterRange::new(11426, 11426),
    CharacterRange::new(11428, 11428), CharacterRange::new(11430, 11430), CharacterRange::new(11432, 11432), CharacterRange::new(11434, 11434), CharacterRange::new(11436, 11436), CharacterRange::new(11438, 11438),
    CharacterRange::new(11440, 11440), CharacterRange::new(11442, 11442), CharacterRange::new(11444, 11444), CharacterRange::new(11446, 11446), CharacterRange::new(11448, 11448), CharacterRange::new(11450, 11450),
    CharacterRange::new(11452, 11452), CharacterRange::new(11454, 11454), CharacterRange::new(11456, 11456), CharacterRange::new(11458, 11458), CharacterRange::new(11460, 11460), CharacterRange::new(11462, 11462),
    CharacterRange::new(11464, 11464), CharacterRange::new(11466, 11466), CharacterRange::new(11468, 11468), CharacterRange::new(11470, 11470), CharacterRange::new(11472, 11472), CharacterRange::new(11474, 11474),
    CharacterRange::new(11476, 11476), CharacterRange::new(11478, 11478), CharacterRange::new(11480, 11480), CharacterRange::new(11482, 11482), CharacterRange::new(11484, 11484), CharacterRange::new(11486, 11486),
    CharacterRange::new(11488, 11488), CharacterRange::new(11490, 11490), CharacterRange::new(11499, 11499), CharacterRange::new(11501, 11501), CharacterRange::new(11506, 11506), CharacterRange::new(42560, 42560),
    CharacterRange::new(42562, 42562), CharacterRange::new(42564, 42564), CharacterRange::new(42566, 42566), CharacterRange::new(42568, 42568), CharacterRange::new(42570, 42570), CharacterRange::new(42572, 42572),
    CharacterRange::new(42574, 42574), CharacterRange::new(42576, 42576), CharacterRange::new(42578, 42578), CharacterRange::new(42580, 42580), CharacterRange::new(42582, 42582), CharacterRange::new(42584, 42584),
    CharacterRange::new(42586, 42586), CharacterRange::new(42588, 42588), CharacterRange::new(42590, 42590), CharacterRange::new(42592, 42592), CharacterRange::new(42594, 42594), CharacterRange::new(42596, 42596),
    CharacterRange::new(42598, 42598), CharacterRange::new(42600, 42600), CharacterRange::new(42602, 42602), CharacterRange::new(42604, 42604), CharacterRange::new(42624, 42624), CharacterRange::new(42626, 42626),
    CharacterRange::new(42628, 42628), CharacterRange::new(42630, 42630), CharacterRange::new(42632, 42632), CharacterRange::new(42634, 42634), CharacterRange::new(42636, 42636), CharacterRange::new(42638, 42638),
    CharacterRange::new(42640, 42640), CharacterRange::new(42642, 42642), CharacterRange::new(42644, 42644), CharacterRange::new(42646, 42646), CharacterRange::new(42648, 42648), CharacterRange::new(42650, 42650),
    CharacterRange::new(42786, 42786), CharacterRange::new(42788, 42788), CharacterRange::new(42790, 42790), CharacterRange::new(42792, 42792), CharacterRange::new(42794, 42794), CharacterRange::new(42796, 42796),
    CharacterRange::new(42798, 42798), CharacterRange::new(42802, 42802), CharacterRange::new(42804, 42804), CharacterRange::new(42806, 42806), CharacterRange::new(42808, 42808), CharacterRange::new(42810, 42810),
    CharacterRange::new(42812, 42812), CharacterRange::new(42814, 42814), CharacterRange::new(42816, 42816), CharacterRange::new(42818, 42818), CharacterRange::new(42820, 42820), CharacterRange::new(42822, 42822),
    CharacterRange::new(42824, 42824), CharacterRange::new(42826, 42826), CharacterRange::new(42828, 42828), CharacterRange::new(42830, 42830), CharacterRange::new(42832, 42832), CharacterRange::new(42834, 42834),
    CharacterRange::new(42836, 42836), CharacterRange::new(42838, 42838), CharacterRange::new(42840, 42840), CharacterRange::new(42842, 42842), CharacterRange::new(42844, 42844), CharacterRange::new(42846, 42846),
    CharacterRange::new(42848, 42848), CharacterRange::new(42850, 42850), CharacterRange::new(42852, 42852), CharacterRange::new(42854, 42854), CharacterRange::new(42856, 42856), CharacterRange::new(42858, 42858),
    CharacterRange::new(42860, 42860), CharacterRange::new(42862, 42862), CharacterRange::new(42873, 42873), CharacterRange::new(42875, 42875), CharacterRange::new(42877, 42878), CharacterRange::new(42880, 42880),
    CharacterRange::new(42882, 42882), CharacterRange::new(42884, 42884), CharacterRange::new(42886, 42886), CharacterRange::new(42891, 42891), CharacterRange::new(42893, 42893), CharacterRange::new(42896, 42896),
    CharacterRange::new(42898, 42898), CharacterRange::new(42902, 42902), CharacterRange::new(42904, 42904), CharacterRange::new(42906, 42906), CharacterRange::new(42908, 42908), CharacterRange::new(42910, 42910),
    CharacterRange::new(42912, 42912), CharacterRange::new(42914, 42914), CharacterRange::new(42916, 42916), CharacterRange::new(42918, 42918), CharacterRange::new(42920, 42920), CharacterRange::new(42922, 42926),
    CharacterRange::new(42928, 42932), CharacterRange::new(42934, 42934), CharacterRange::new(42936, 42936), CharacterRange::new(42938, 42938), CharacterRange::new(42940, 42940), CharacterRange::new(42942, 42942),
    CharacterRange::new(42944, 42944), CharacterRange::new(42946, 42946), CharacterRange::new(42948, 42951), CharacterRange::new(42953, 42953), CharacterRange::new(42955, 42956), CharacterRange::new(42960, 42960),
    CharacterRange::new(42966, 42966), CharacterRange::new(42968, 42968), CharacterRange::new(42970, 42970), CharacterRange::new(42972, 42972), CharacterRange::new(42997, 42997), CharacterRange::new(65313, 65338),
    CharacterRange::new(66560, 66599), CharacterRange::new(66736, 66771), CharacterRange::new(66928, 66938), CharacterRange::new(66940, 66954), CharacterRange::new(66956, 66962), CharacterRange::new(66964, 66965),
    CharacterRange::new(68736, 68786), CharacterRange::new(68944, 68965), CharacterRange::new(71840, 71871), CharacterRange::new(93760, 93791), CharacterRange::new(119808, 119833), CharacterRange::new(119860, 119885),
    CharacterRange::new(119912, 119937), CharacterRange::new(119964, 119964), CharacterRange::new(119966, 119967), CharacterRange::new(119970, 119970), CharacterRange::new(119973, 119974), CharacterRange::new(119977, 119980),
    CharacterRange::new(119982, 119989), CharacterRange::new(120016, 120041), CharacterRange::new(120068, 120069), CharacterRange::new(120071, 120074), CharacterRange::new(120077, 120084), CharacterRange::new(120086, 120092),
    CharacterRange::new(120120, 120121), CharacterRange::new(120123, 120126), CharacterRange::new(120128, 120132), CharacterRange::new(120134, 120134), CharacterRange::new(120138, 120144), CharacterRange::new(120172, 120197),
    CharacterRange::new(120224, 120249), CharacterRange::new(120276, 120301), CharacterRange::new(120328, 120353), CharacterRange::new(120380, 120405), CharacterRange::new(120432, 120457), CharacterRange::new(120488, 120512),
    CharacterRange::new(120546, 120570), CharacterRange::new(120604, 120628), CharacterRange::new(120662, 120686), CharacterRange::new(120720, 120744), CharacterRange::new(120778, 120778), CharacterRange::new(125184, 125217),
];

#[rustfmt::skip]
static sym_variable_character_set_1: [CharacterRange; 1176] = [
    CharacterRange::new(95, 95), CharacterRange::new(97, 122), CharacterRange::new(170, 170), CharacterRange::new(181, 181), CharacterRange::new(186, 186), CharacterRange::new(223, 246),
    CharacterRange::new(248, 255), CharacterRange::new(257, 257), CharacterRange::new(259, 259), CharacterRange::new(261, 261), CharacterRange::new(263, 263), CharacterRange::new(265, 265),
    CharacterRange::new(267, 267), CharacterRange::new(269, 269), CharacterRange::new(271, 271), CharacterRange::new(273, 273), CharacterRange::new(275, 275), CharacterRange::new(277, 277),
    CharacterRange::new(279, 279), CharacterRange::new(281, 281), CharacterRange::new(283, 283), CharacterRange::new(285, 285), CharacterRange::new(287, 287), CharacterRange::new(289, 289),
    CharacterRange::new(291, 291), CharacterRange::new(293, 293), CharacterRange::new(295, 295), CharacterRange::new(297, 297), CharacterRange::new(299, 299), CharacterRange::new(301, 301),
    CharacterRange::new(303, 303), CharacterRange::new(305, 305), CharacterRange::new(307, 307), CharacterRange::new(309, 309), CharacterRange::new(311, 312), CharacterRange::new(314, 314),
    CharacterRange::new(316, 316), CharacterRange::new(318, 318), CharacterRange::new(320, 320), CharacterRange::new(322, 322), CharacterRange::new(324, 324), CharacterRange::new(326, 326),
    CharacterRange::new(328, 329), CharacterRange::new(331, 331), CharacterRange::new(333, 333), CharacterRange::new(335, 335), CharacterRange::new(337, 337), CharacterRange::new(339, 339),
    CharacterRange::new(341, 341), CharacterRange::new(343, 343), CharacterRange::new(345, 345), CharacterRange::new(347, 347), CharacterRange::new(349, 349), CharacterRange::new(351, 351),
    CharacterRange::new(353, 353), CharacterRange::new(355, 355), CharacterRange::new(357, 357), CharacterRange::new(359, 359), CharacterRange::new(361, 361), CharacterRange::new(363, 363),
    CharacterRange::new(365, 365), CharacterRange::new(367, 367), CharacterRange::new(369, 369), CharacterRange::new(371, 371), CharacterRange::new(373, 373), CharacterRange::new(375, 375),
    CharacterRange::new(378, 378), CharacterRange::new(380, 380), CharacterRange::new(382, 384), CharacterRange::new(387, 387), CharacterRange::new(389, 389), CharacterRange::new(392, 392),
    CharacterRange::new(396, 397), CharacterRange::new(402, 402), CharacterRange::new(405, 405), CharacterRange::new(409, 411), CharacterRange::new(414, 414), CharacterRange::new(417, 417),
    CharacterRange::new(419, 419), CharacterRange::new(421, 421), CharacterRange::new(424, 424), CharacterRange::new(426, 427), CharacterRange::new(429, 429), CharacterRange::new(432, 432),
    CharacterRange::new(436, 436), CharacterRange::new(438, 438), CharacterRange::new(441, 443), CharacterRange::new(445, 451), CharacterRange::new(454, 454), CharacterRange::new(457, 457),
    CharacterRange::new(460, 460), CharacterRange::new(462, 462), CharacterRange::new(464, 464), CharacterRange::new(466, 466), CharacterRange::new(468, 468), CharacterRange::new(470, 470),
    CharacterRange::new(472, 472), CharacterRange::new(474, 474), CharacterRange::new(476, 477), CharacterRange::new(479, 479), CharacterRange::new(481, 481), CharacterRange::new(483, 483),
    CharacterRange::new(485, 485), CharacterRange::new(487, 487), CharacterRange::new(489, 489), CharacterRange::new(491, 491), CharacterRange::new(493, 493), CharacterRange::new(495, 496),
    CharacterRange::new(499, 499), CharacterRange::new(501, 501), CharacterRange::new(505, 505), CharacterRange::new(507, 507), CharacterRange::new(509, 509), CharacterRange::new(511, 511),
    CharacterRange::new(513, 513), CharacterRange::new(515, 515), CharacterRange::new(517, 517), CharacterRange::new(519, 519), CharacterRange::new(521, 521), CharacterRange::new(523, 523),
    CharacterRange::new(525, 525), CharacterRange::new(527, 527), CharacterRange::new(529, 529), CharacterRange::new(531, 531), CharacterRange::new(533, 533), CharacterRange::new(535, 535),
    CharacterRange::new(537, 537), CharacterRange::new(539, 539), CharacterRange::new(541, 541), CharacterRange::new(543, 543), CharacterRange::new(545, 545), CharacterRange::new(547, 547),
    CharacterRange::new(549, 549), CharacterRange::new(551, 551), CharacterRange::new(553, 553), CharacterRange::new(555, 555), CharacterRange::new(557, 557), CharacterRange::new(559, 559),
    CharacterRange::new(561, 561), CharacterRange::new(563, 569), CharacterRange::new(572, 572), CharacterRange::new(575, 576), CharacterRange::new(578, 578), CharacterRange::new(583, 583),
    CharacterRange::new(585, 585), CharacterRange::new(587, 587), CharacterRange::new(589, 589), CharacterRange::new(591, 687), CharacterRange::new(881, 881), CharacterRange::new(883, 883),
    CharacterRange::new(887, 887), CharacterRange::new(891, 893), CharacterRange::new(912, 912), CharacterRange::new(940, 974), CharacterRange::new(976, 977), CharacterRange::new(981, 983),
    CharacterRange::new(985, 985), CharacterRange::new(987, 987), CharacterRange::new(989, 989), CharacterRange::new(991, 991), CharacterRange::new(993, 993), CharacterRange::new(995, 995),
    CharacterRange::new(997, 997), CharacterRange::new(999, 999), CharacterRange::new(1001, 1001), CharacterRange::new(1003, 1003), CharacterRange::new(1005, 1005), CharacterRange::new(1007, 1011),
    CharacterRange::new(1013, 1013), CharacterRange::new(1016, 1016), CharacterRange::new(1019, 1020), CharacterRange::new(1072, 1119), CharacterRange::new(1121, 1121), CharacterRange::new(1123, 1123),
    CharacterRange::new(1125, 1125), CharacterRange::new(1127, 1127), CharacterRange::new(1129, 1129), CharacterRange::new(1131, 1131), CharacterRange::new(1133, 1133), CharacterRange::new(1135, 1135),
    CharacterRange::new(1137, 1137), CharacterRange::new(1139, 1139), CharacterRange::new(1141, 1141), CharacterRange::new(1143, 1143), CharacterRange::new(1145, 1145), CharacterRange::new(1147, 1147),
    CharacterRange::new(1149, 1149), CharacterRange::new(1151, 1151), CharacterRange::new(1153, 1153), CharacterRange::new(1163, 1163), CharacterRange::new(1165, 1165), CharacterRange::new(1167, 1167),
    CharacterRange::new(1169, 1169), CharacterRange::new(1171, 1171), CharacterRange::new(1173, 1173), CharacterRange::new(1175, 1175), CharacterRange::new(1177, 1177), CharacterRange::new(1179, 1179),
    CharacterRange::new(1181, 1181), CharacterRange::new(1183, 1183), CharacterRange::new(1185, 1185), CharacterRange::new(1187, 1187), CharacterRange::new(1189, 1189), CharacterRange::new(1191, 1191),
    CharacterRange::new(1193, 1193), CharacterRange::new(1195, 1195), CharacterRange::new(1197, 1197), CharacterRange::new(1199, 1199), CharacterRange::new(1201, 1201), CharacterRange::new(1203, 1203),
    CharacterRange::new(1205, 1205), CharacterRange::new(1207, 1207), CharacterRange::new(1209, 1209), CharacterRange::new(1211, 1211), CharacterRange::new(1213, 1213), CharacterRange::new(1215, 1215),
    CharacterRange::new(1218, 1218), CharacterRange::new(1220, 1220), CharacterRange::new(1222, 1222), CharacterRange::new(1224, 1224), CharacterRange::new(1226, 1226), CharacterRange::new(1228, 1228),
    CharacterRange::new(1230, 1231), CharacterRange::new(1233, 1233), CharacterRange::new(1235, 1235), CharacterRange::new(1237, 1237), CharacterRange::new(1239, 1239), CharacterRange::new(1241, 1241),
    CharacterRange::new(1243, 1243), CharacterRange::new(1245, 1245), CharacterRange::new(1247, 1247), CharacterRange::new(1249, 1249), CharacterRange::new(1251, 1251), CharacterRange::new(1253, 1253),
    CharacterRange::new(1255, 1255), CharacterRange::new(1257, 1257), CharacterRange::new(1259, 1259), CharacterRange::new(1261, 1261), CharacterRange::new(1263, 1263), CharacterRange::new(1265, 1265),
    CharacterRange::new(1267, 1267), CharacterRange::new(1269, 1269), CharacterRange::new(1271, 1271), CharacterRange::new(1273, 1273), CharacterRange::new(1275, 1275), CharacterRange::new(1277, 1277),
    CharacterRange::new(1279, 1279), CharacterRange::new(1281, 1281), CharacterRange::new(1283, 1283), CharacterRange::new(1285, 1285), CharacterRange::new(1287, 1287), CharacterRange::new(1289, 1289),
    CharacterRange::new(1291, 1291), CharacterRange::new(1293, 1293), CharacterRange::new(1295, 1295), CharacterRange::new(1297, 1297), CharacterRange::new(1299, 1299), CharacterRange::new(1301, 1301),
    CharacterRange::new(1303, 1303), CharacterRange::new(1305, 1305), CharacterRange::new(1307, 1307), CharacterRange::new(1309, 1309), CharacterRange::new(1311, 1311), CharacterRange::new(1313, 1313),
    CharacterRange::new(1315, 1315), CharacterRange::new(1317, 1317), CharacterRange::new(1319, 1319), CharacterRange::new(1321, 1321), CharacterRange::new(1323, 1323), CharacterRange::new(1325, 1325),
    CharacterRange::new(1327, 1327), CharacterRange::new(1376, 1416), CharacterRange::new(1488, 1514), CharacterRange::new(1519, 1522), CharacterRange::new(1568, 1599), CharacterRange::new(1601, 1610),
    CharacterRange::new(1646, 1647), CharacterRange::new(1649, 1747), CharacterRange::new(1749, 1749), CharacterRange::new(1774, 1775), CharacterRange::new(1786, 1788), CharacterRange::new(1791, 1791),
    CharacterRange::new(1808, 1808), CharacterRange::new(1810, 1839), CharacterRange::new(1869, 1957), CharacterRange::new(1969, 1969), CharacterRange::new(1994, 2026), CharacterRange::new(2048, 2069),
    CharacterRange::new(2112, 2136), CharacterRange::new(2144, 2154), CharacterRange::new(2160, 2183), CharacterRange::new(2185, 2190), CharacterRange::new(2208, 2248), CharacterRange::new(2308, 2361),
    CharacterRange::new(2365, 2365), CharacterRange::new(2384, 2384), CharacterRange::new(2392, 2401), CharacterRange::new(2418, 2432), CharacterRange::new(2437, 2444), CharacterRange::new(2447, 2448),
    CharacterRange::new(2451, 2472), CharacterRange::new(2474, 2480), CharacterRange::new(2482, 2482), CharacterRange::new(2486, 2489), CharacterRange::new(2493, 2493), CharacterRange::new(2510, 2510),
    CharacterRange::new(2524, 2525), CharacterRange::new(2527, 2529), CharacterRange::new(2544, 2545), CharacterRange::new(2556, 2556), CharacterRange::new(2565, 2570), CharacterRange::new(2575, 2576),
    CharacterRange::new(2579, 2600), CharacterRange::new(2602, 2608), CharacterRange::new(2610, 2611), CharacterRange::new(2613, 2614), CharacterRange::new(2616, 2617), CharacterRange::new(2649, 2652),
    CharacterRange::new(2654, 2654), CharacterRange::new(2674, 2676), CharacterRange::new(2693, 2701), CharacterRange::new(2703, 2705), CharacterRange::new(2707, 2728), CharacterRange::new(2730, 2736),
    CharacterRange::new(2738, 2739), CharacterRange::new(2741, 2745), CharacterRange::new(2749, 2749), CharacterRange::new(2768, 2768), CharacterRange::new(2784, 2785), CharacterRange::new(2809, 2809),
    CharacterRange::new(2821, 2828), CharacterRange::new(2831, 2832), CharacterRange::new(2835, 2856), CharacterRange::new(2858, 2864), CharacterRange::new(2866, 2867), CharacterRange::new(2869, 2873),
    CharacterRange::new(2877, 2877), CharacterRange::new(2908, 2909), CharacterRange::new(2911, 2913), CharacterRange::new(2929, 2929), CharacterRange::new(2947, 2947), CharacterRange::new(2949, 2954),
    CharacterRange::new(2958, 2960), CharacterRange::new(2962, 2965), CharacterRange::new(2969, 2970), CharacterRange::new(2972, 2972), CharacterRange::new(2974, 2975), CharacterRange::new(2979, 2980),
    CharacterRange::new(2984, 2986), CharacterRange::new(2990, 3001), CharacterRange::new(3024, 3024), CharacterRange::new(3077, 3084), CharacterRange::new(3086, 3088), CharacterRange::new(3090, 3112),
    CharacterRange::new(3114, 3129), CharacterRange::new(3133, 3133), CharacterRange::new(3160, 3162), CharacterRange::new(3165, 3165), CharacterRange::new(3168, 3169), CharacterRange::new(3200, 3200),
    CharacterRange::new(3205, 3212), CharacterRange::new(3214, 3216), CharacterRange::new(3218, 3240), CharacterRange::new(3242, 3251), CharacterRange::new(3253, 3257), CharacterRange::new(3261, 3261),
    CharacterRange::new(3293, 3294), CharacterRange::new(3296, 3297), CharacterRange::new(3313, 3314), CharacterRange::new(3332, 3340), CharacterRange::new(3342, 3344), CharacterRange::new(3346, 3386),
    CharacterRange::new(3389, 3389), CharacterRange::new(3406, 3406), CharacterRange::new(3412, 3414), CharacterRange::new(3423, 3425), CharacterRange::new(3450, 3455), CharacterRange::new(3461, 3478),
    CharacterRange::new(3482, 3505), CharacterRange::new(3507, 3515), CharacterRange::new(3517, 3517), CharacterRange::new(3520, 3526), CharacterRange::new(3585, 3632), CharacterRange::new(3634, 3635),
    CharacterRange::new(3648, 3653), CharacterRange::new(3713, 3714), CharacterRange::new(3716, 3716), CharacterRange::new(3718, 3722), CharacterRange::new(3724, 3747), CharacterRange::new(3749, 3749),
    CharacterRange::new(3751, 3760), CharacterRange::new(3762, 3763), CharacterRange::new(3773, 3773), CharacterRange::new(3776, 3780), CharacterRange::new(3804, 3807), CharacterRange::new(3840, 3840),
    CharacterRange::new(3904, 3911), CharacterRange::new(3913, 3948), CharacterRange::new(3976, 3980), CharacterRange::new(4096, 4138), CharacterRange::new(4159, 4159), CharacterRange::new(4176, 4181),
    CharacterRange::new(4186, 4189), CharacterRange::new(4193, 4193), CharacterRange::new(4197, 4198), CharacterRange::new(4206, 4208), CharacterRange::new(4213, 4225), CharacterRange::new(4238, 4238),
    CharacterRange::new(4304, 4346), CharacterRange::new(4349, 4680), CharacterRange::new(4682, 4685), CharacterRange::new(4688, 4694), CharacterRange::new(4696, 4696), CharacterRange::new(4698, 4701),
    CharacterRange::new(4704, 4744), CharacterRange::new(4746, 4749), CharacterRange::new(4752, 4784), CharacterRange::new(4786, 4789), CharacterRange::new(4792, 4798), CharacterRange::new(4800, 4800),
    CharacterRange::new(4802, 4805), CharacterRange::new(4808, 4822), CharacterRange::new(4824, 4880), CharacterRange::new(4882, 4885), CharacterRange::new(4888, 4954), CharacterRange::new(4992, 5007),
    CharacterRange::new(5112, 5117), CharacterRange::new(5121, 5740), CharacterRange::new(5743, 5759), CharacterRange::new(5761, 5786), CharacterRange::new(5792, 5866), CharacterRange::new(5873, 5880),
    CharacterRange::new(5888, 5905), CharacterRange::new(5919, 5937), CharacterRange::new(5952, 5969), CharacterRange::new(5984, 5996), CharacterRange::new(5998, 6000), CharacterRange::new(6016, 6067),
    CharacterRange::new(6108, 6108), CharacterRange::new(6176, 6210), CharacterRange::new(6212, 6264), CharacterRange::new(6272, 6276), CharacterRange::new(6279, 6312), CharacterRange::new(6314, 6314),
    CharacterRange::new(6320, 6389), CharacterRange::new(6400, 6430), CharacterRange::new(6480, 6509), CharacterRange::new(6512, 6516), CharacterRange::new(6528, 6571), CharacterRange::new(6576, 6601),
    CharacterRange::new(6656, 6678), CharacterRange::new(6688, 6740), CharacterRange::new(6917, 6963), CharacterRange::new(6981, 6988), CharacterRange::new(7043, 7072), CharacterRange::new(7086, 7087),
    CharacterRange::new(7098, 7141), CharacterRange::new(7168, 7203), CharacterRange::new(7245, 7247), CharacterRange::new(7258, 7287), CharacterRange::new(7296, 7304), CharacterRange::new(7306, 7306),
    CharacterRange::new(7401, 7404), CharacterRange::new(7406, 7411), CharacterRange::new(7413, 7414), CharacterRange::new(7418, 7418), CharacterRange::new(7424, 7467), CharacterRange::new(7531, 7543),
    CharacterRange::new(7545, 7578), CharacterRange::new(7681, 7681), CharacterRange::new(7683, 7683), CharacterRange::new(7685, 7685), CharacterRange::new(7687, 7687), CharacterRange::new(7689, 7689),
    CharacterRange::new(7691, 7691), CharacterRange::new(7693, 7693), CharacterRange::new(7695, 7695), CharacterRange::new(7697, 7697), CharacterRange::new(7699, 7699), CharacterRange::new(7701, 7701),
    CharacterRange::new(7703, 7703), CharacterRange::new(7705, 7705), CharacterRange::new(7707, 7707), CharacterRange::new(7709, 7709), CharacterRange::new(7711, 7711), CharacterRange::new(7713, 7713),
    CharacterRange::new(7715, 7715), CharacterRange::new(7717, 7717), CharacterRange::new(7719, 7719), CharacterRange::new(7721, 7721), CharacterRange::new(7723, 7723), CharacterRange::new(7725, 7725),
    CharacterRange::new(7727, 7727), CharacterRange::new(7729, 7729), CharacterRange::new(7731, 7731), CharacterRange::new(7733, 7733), CharacterRange::new(7735, 7735), CharacterRange::new(7737, 7737),
    CharacterRange::new(7739, 7739), CharacterRange::new(7741, 7741), CharacterRange::new(7743, 7743), CharacterRange::new(7745, 7745), CharacterRange::new(7747, 7747), CharacterRange::new(7749, 7749),
    CharacterRange::new(7751, 7751), CharacterRange::new(7753, 7753), CharacterRange::new(7755, 7755), CharacterRange::new(7757, 7757), CharacterRange::new(7759, 7759), CharacterRange::new(7761, 7761),
    CharacterRange::new(7763, 7763), CharacterRange::new(7765, 7765), CharacterRange::new(7767, 7767), CharacterRange::new(7769, 7769), CharacterRange::new(7771, 7771), CharacterRange::new(7773, 7773),
    CharacterRange::new(7775, 7775), CharacterRange::new(7777, 7777), CharacterRange::new(7779, 7779), CharacterRange::new(7781, 7781), CharacterRange::new(7783, 7783), CharacterRange::new(7785, 7785),
    CharacterRange::new(7787, 7787), CharacterRange::new(7789, 7789), CharacterRange::new(7791, 7791), CharacterRange::new(7793, 7793), CharacterRange::new(7795, 7795), CharacterRange::new(7797, 7797),
    CharacterRange::new(7799, 7799), CharacterRange::new(7801, 7801), CharacterRange::new(7803, 7803), CharacterRange::new(7805, 7805), CharacterRange::new(7807, 7807), CharacterRange::new(7809, 7809),
    CharacterRange::new(7811, 7811), CharacterRange::new(7813, 7813), CharacterRange::new(7815, 7815), CharacterRange::new(7817, 7817), CharacterRange::new(7819, 7819), CharacterRange::new(7821, 7821),
    CharacterRange::new(7823, 7823), CharacterRange::new(7825, 7825), CharacterRange::new(7827, 7827), CharacterRange::new(7829, 7837), CharacterRange::new(7839, 7839), CharacterRange::new(7841, 7841),
    CharacterRange::new(7843, 7843), CharacterRange::new(7845, 7845), CharacterRange::new(7847, 7847), CharacterRange::new(7849, 7849), CharacterRange::new(7851, 7851), CharacterRange::new(7853, 7853),
    CharacterRange::new(7855, 7855), CharacterRange::new(7857, 7857), CharacterRange::new(7859, 7859), CharacterRange::new(7861, 7861), CharacterRange::new(7863, 7863), CharacterRange::new(7865, 7865),
    CharacterRange::new(7867, 7867), CharacterRange::new(7869, 7869), CharacterRange::new(7871, 7871), CharacterRange::new(7873, 7873), CharacterRange::new(7875, 7875), CharacterRange::new(7877, 7877),
    CharacterRange::new(7879, 7879), CharacterRange::new(7881, 7881), CharacterRange::new(7883, 7883), CharacterRange::new(7885, 7885), CharacterRange::new(7887, 7887), CharacterRange::new(7889, 7889),
    CharacterRange::new(7891, 7891), CharacterRange::new(7893, 7893), CharacterRange::new(7895, 7895), CharacterRange::new(7897, 7897), CharacterRange::new(7899, 7899), CharacterRange::new(7901, 7901),
    CharacterRange::new(7903, 7903), CharacterRange::new(7905, 7905), CharacterRange::new(7907, 7907), CharacterRange::new(7909, 7909), CharacterRange::new(7911, 7911), CharacterRange::new(7913, 7913),
    CharacterRange::new(7915, 7915), CharacterRange::new(7917, 7917), CharacterRange::new(7919, 7919), CharacterRange::new(7921, 7921), CharacterRange::new(7923, 7923), CharacterRange::new(7925, 7925),
    CharacterRange::new(7927, 7927), CharacterRange::new(7929, 7929), CharacterRange::new(7931, 7931), CharacterRange::new(7933, 7933), CharacterRange::new(7935, 7943), CharacterRange::new(7952, 7957),
    CharacterRange::new(7968, 7975), CharacterRange::new(7984, 7991), CharacterRange::new(8000, 8005), CharacterRange::new(8016, 8023), CharacterRange::new(8032, 8039), CharacterRange::new(8048, 8061),
    CharacterRange::new(8064, 8071), CharacterRange::new(8080, 8087), CharacterRange::new(8096, 8103), CharacterRange::new(8112, 8116), CharacterRange::new(8118, 8119), CharacterRange::new(8126, 8126),
    CharacterRange::new(8130, 8132), CharacterRange::new(8134, 8135), CharacterRange::new(8144, 8147), CharacterRange::new(8150, 8151), CharacterRange::new(8160, 8167), CharacterRange::new(8178, 8180),
    CharacterRange::new(8182, 8183), CharacterRange::new(8458, 8458), CharacterRange::new(8462, 8463), CharacterRange::new(8467, 8467), CharacterRange::new(8495, 8495), CharacterRange::new(8500, 8505),
    CharacterRange::new(8508, 8509), CharacterRange::new(8518, 8521), CharacterRange::new(8526, 8526), CharacterRange::new(8580, 8580), CharacterRange::new(11312, 11359), CharacterRange::new(11361, 11361),
    CharacterRange::new(11365, 11366), CharacterRange::new(11368, 11368), CharacterRange::new(11370, 11370), CharacterRange::new(11372, 11372), CharacterRange::new(11377, 11377), CharacterRange::new(11379, 11380),
    CharacterRange::new(11382, 11387), CharacterRange::new(11393, 11393), CharacterRange::new(11395, 11395), CharacterRange::new(11397, 11397), CharacterRange::new(11399, 11399), CharacterRange::new(11401, 11401),
    CharacterRange::new(11403, 11403), CharacterRange::new(11405, 11405), CharacterRange::new(11407, 11407), CharacterRange::new(11409, 11409), CharacterRange::new(11411, 11411), CharacterRange::new(11413, 11413),
    CharacterRange::new(11415, 11415), CharacterRange::new(11417, 11417), CharacterRange::new(11419, 11419), CharacterRange::new(11421, 11421), CharacterRange::new(11423, 11423), CharacterRange::new(11425, 11425),
    CharacterRange::new(11427, 11427), CharacterRange::new(11429, 11429), CharacterRange::new(11431, 11431), CharacterRange::new(11433, 11433), CharacterRange::new(11435, 11435), CharacterRange::new(11437, 11437),
    CharacterRange::new(11439, 11439), CharacterRange::new(11441, 11441), CharacterRange::new(11443, 11443), CharacterRange::new(11445, 11445), CharacterRange::new(11447, 11447), CharacterRange::new(11449, 11449),
    CharacterRange::new(11451, 11451), CharacterRange::new(11453, 11453), CharacterRange::new(11455, 11455), CharacterRange::new(11457, 11457), CharacterRange::new(11459, 11459), CharacterRange::new(11461, 11461),
    CharacterRange::new(11463, 11463), CharacterRange::new(11465, 11465), CharacterRange::new(11467, 11467), CharacterRange::new(11469, 11469), CharacterRange::new(11471, 11471), CharacterRange::new(11473, 11473),
    CharacterRange::new(11475, 11475), CharacterRange::new(11477, 11477), CharacterRange::new(11479, 11479), CharacterRange::new(11481, 11481), CharacterRange::new(11483, 11483), CharacterRange::new(11485, 11485),
    CharacterRange::new(11487, 11487), CharacterRange::new(11489, 11489), CharacterRange::new(11491, 11492), CharacterRange::new(11500, 11500), CharacterRange::new(11502, 11502), CharacterRange::new(11507, 11507),
    CharacterRange::new(11520, 11557), CharacterRange::new(11559, 11559), CharacterRange::new(11565, 11565), CharacterRange::new(11568, 11623), CharacterRange::new(11648, 11670), CharacterRange::new(11680, 11686),
    CharacterRange::new(11688, 11694), CharacterRange::new(11696, 11702), CharacterRange::new(11704, 11710), CharacterRange::new(11712, 11718), CharacterRange::new(11720, 11726), CharacterRange::new(11728, 11734),
    CharacterRange::new(11736, 11742), CharacterRange::new(12294, 12294), CharacterRange::new(12348, 12348), CharacterRange::new(12353, 12438), CharacterRange::new(12447, 12447), CharacterRange::new(12449, 12538),
    CharacterRange::new(12543, 12543), CharacterRange::new(12549, 12591), CharacterRange::new(12593, 12686), CharacterRange::new(12704, 12735), CharacterRange::new(12784, 12799), CharacterRange::new(13312, 19903),
    CharacterRange::new(19968, 40980), CharacterRange::new(40982, 42124), CharacterRange::new(42192, 42231), CharacterRange::new(42240, 42507), CharacterRange::new(42512, 42527), CharacterRange::new(42538, 42539),
    CharacterRange::new(42561, 42561), CharacterRange::new(42563, 42563), CharacterRange::new(42565, 42565), CharacterRange::new(42567, 42567), CharacterRange::new(42569, 42569), CharacterRange::new(42571, 42571),
    CharacterRange::new(42573, 42573), CharacterRange::new(42575, 42575), CharacterRange::new(42577, 42577), CharacterRange::new(42579, 42579), CharacterRange::new(42581, 42581), CharacterRange::new(42583, 42583),
    CharacterRange::new(42585, 42585), CharacterRange::new(42587, 42587), CharacterRange::new(42589, 42589), CharacterRange::new(42591, 42591), CharacterRange::new(42593, 42593), CharacterRange::new(42595, 42595),
    CharacterRange::new(42597, 42597), CharacterRange::new(42599, 42599), CharacterRange::new(42601, 42601), CharacterRange::new(42603, 42603), CharacterRange::new(42605, 42606), CharacterRange::new(42625, 42625),
    CharacterRange::new(42627, 42627), CharacterRange::new(42629, 42629), CharacterRange::new(42631, 42631), CharacterRange::new(42633, 42633), CharacterRange::new(42635, 42635), CharacterRange::new(42637, 42637),
    CharacterRange::new(42639, 42639), CharacterRange::new(42641, 42641), CharacterRange::new(42643, 42643), CharacterRange::new(42645, 42645), CharacterRange::new(42647, 42647), CharacterRange::new(42649, 42649),
    CharacterRange::new(42651, 42651), CharacterRange::new(42656, 42725), CharacterRange::new(42787, 42787), CharacterRange::new(42789, 42789), CharacterRange::new(42791, 42791), CharacterRange::new(42793, 42793),
    CharacterRange::new(42795, 42795), CharacterRange::new(42797, 42797), CharacterRange::new(42799, 42801), CharacterRange::new(42803, 42803), CharacterRange::new(42805, 42805), CharacterRange::new(42807, 42807),
    CharacterRange::new(42809, 42809), CharacterRange::new(42811, 42811), CharacterRange::new(42813, 42813), CharacterRange::new(42815, 42815), CharacterRange::new(42817, 42817), CharacterRange::new(42819, 42819),
    CharacterRange::new(42821, 42821), CharacterRange::new(42823, 42823), CharacterRange::new(42825, 42825), CharacterRange::new(42827, 42827), CharacterRange::new(42829, 42829), CharacterRange::new(42831, 42831),
    CharacterRange::new(42833, 42833), CharacterRange::new(42835, 42835), CharacterRange::new(42837, 42837), CharacterRange::new(42839, 42839), CharacterRange::new(42841, 42841), CharacterRange::new(42843, 42843),
    CharacterRange::new(42845, 42845), CharacterRange::new(42847, 42847), CharacterRange::new(42849, 42849), CharacterRange::new(42851, 42851), CharacterRange::new(42853, 42853), CharacterRange::new(42855, 42855),
    CharacterRange::new(42857, 42857), CharacterRange::new(42859, 42859), CharacterRange::new(42861, 42861), CharacterRange::new(42863, 42863), CharacterRange::new(42865, 42872), CharacterRange::new(42874, 42874),
    CharacterRange::new(42876, 42876), CharacterRange::new(42879, 42879), CharacterRange::new(42881, 42881), CharacterRange::new(42883, 42883), CharacterRange::new(42885, 42885), CharacterRange::new(42887, 42887),
    CharacterRange::new(42892, 42892), CharacterRange::new(42894, 42895), CharacterRange::new(42897, 42897), CharacterRange::new(42899, 42901), CharacterRange::new(42903, 42903), CharacterRange::new(42905, 42905),
    CharacterRange::new(42907, 42907), CharacterRange::new(42909, 42909), CharacterRange::new(42911, 42911), CharacterRange::new(42913, 42913), CharacterRange::new(42915, 42915), CharacterRange::new(42917, 42917),
    CharacterRange::new(42919, 42919), CharacterRange::new(42921, 42921), CharacterRange::new(42927, 42927), CharacterRange::new(42933, 42933), CharacterRange::new(42935, 42935), CharacterRange::new(42937, 42937),
    CharacterRange::new(42939, 42939), CharacterRange::new(42941, 42941), CharacterRange::new(42943, 42943), CharacterRange::new(42945, 42945), CharacterRange::new(42947, 42947), CharacterRange::new(42952, 42952),
    CharacterRange::new(42954, 42954), CharacterRange::new(42957, 42957), CharacterRange::new(42961, 42961), CharacterRange::new(42963, 42963), CharacterRange::new(42965, 42965), CharacterRange::new(42967, 42967),
    CharacterRange::new(42969, 42969), CharacterRange::new(42971, 42971), CharacterRange::new(42998, 42999), CharacterRange::new(43002, 43009), CharacterRange::new(43011, 43013), CharacterRange::new(43015, 43018),
    CharacterRange::new(43020, 43042), CharacterRange::new(43072, 43123), CharacterRange::new(43138, 43187), CharacterRange::new(43250, 43255), CharacterRange::new(43259, 43259), CharacterRange::new(43261, 43262),
    CharacterRange::new(43274, 43301), CharacterRange::new(43312, 43334), CharacterRange::new(43360, 43388), CharacterRange::new(43396, 43442), CharacterRange::new(43488, 43492), CharacterRange::new(43495, 43503),
    CharacterRange::new(43514, 43518), CharacterRange::new(43520, 43560), CharacterRange::new(43584, 43586), CharacterRange::new(43588, 43595), CharacterRange::new(43616, 43631), CharacterRange::new(43633, 43638),
    CharacterRange::new(43642, 43642), CharacterRange::new(43646, 43695), CharacterRange::new(43697, 43697), CharacterRange::new(43701, 43702), CharacterRange::new(43705, 43709), CharacterRange::new(43712, 43712),
    CharacterRange::new(43714, 43714), CharacterRange::new(43739, 43740), CharacterRange::new(43744, 43754), CharacterRange::new(43762, 43762), CharacterRange::new(43777, 43782), CharacterRange::new(43785, 43790),
    CharacterRange::new(43793, 43798), CharacterRange::new(43808, 43814), CharacterRange::new(43816, 43822), CharacterRange::new(43824, 43866), CharacterRange::new(43872, 43880), CharacterRange::new(43888, 44002),
    CharacterRange::new(44032, 55203), CharacterRange::new(55216, 55238), CharacterRange::new(55243, 55291), CharacterRange::new(63744, 64109), CharacterRange::new(64112, 64217), CharacterRange::new(64256, 64262),
    CharacterRange::new(64275, 64279), CharacterRange::new(64285, 64285), CharacterRange::new(64287, 64296), CharacterRange::new(64298, 64310), CharacterRange::new(64312, 64316), CharacterRange::new(64318, 64318),
    CharacterRange::new(64320, 64321), CharacterRange::new(64323, 64324), CharacterRange::new(64326, 64433), CharacterRange::new(64467, 64829), CharacterRange::new(64848, 64911), CharacterRange::new(64914, 64967),
    CharacterRange::new(65008, 65019), CharacterRange::new(65136, 65140), CharacterRange::new(65142, 65276), CharacterRange::new(65345, 65370), CharacterRange::new(65382, 65391), CharacterRange::new(65393, 65437),
    CharacterRange::new(65440, 65470), CharacterRange::new(65474, 65479), CharacterRange::new(65482, 65487), CharacterRange::new(65490, 65495), CharacterRange::new(65498, 65500), CharacterRange::new(65536, 65547),
    CharacterRange::new(65549, 65574), CharacterRange::new(65576, 65594), CharacterRange::new(65596, 65597), CharacterRange::new(65599, 65613), CharacterRange::new(65616, 65629), CharacterRange::new(65664, 65786),
    CharacterRange::new(66176, 66204), CharacterRange::new(66208, 66256), CharacterRange::new(66304, 66335), CharacterRange::new(66349, 66368), CharacterRange::new(66370, 66377), CharacterRange::new(66384, 66421),
    CharacterRange::new(66432, 66461), CharacterRange::new(66464, 66499), CharacterRange::new(66504, 66511), CharacterRange::new(66600, 66717), CharacterRange::new(66776, 66811), CharacterRange::new(66816, 66855),
    CharacterRange::new(66864, 66915), CharacterRange::new(66967, 66977), CharacterRange::new(66979, 66993), CharacterRange::new(66995, 67001), CharacterRange::new(67003, 67004), CharacterRange::new(67008, 67059),
    CharacterRange::new(67072, 67382), CharacterRange::new(67392, 67413), CharacterRange::new(67424, 67431), CharacterRange::new(67584, 67589), CharacterRange::new(67592, 67592), CharacterRange::new(67594, 67637),
    CharacterRange::new(67639, 67640), CharacterRange::new(67644, 67644), CharacterRange::new(67647, 67669), CharacterRange::new(67680, 67702), CharacterRange::new(67712, 67742), CharacterRange::new(67808, 67826),
    CharacterRange::new(67828, 67829), CharacterRange::new(67840, 67861), CharacterRange::new(67872, 67897), CharacterRange::new(67968, 68023), CharacterRange::new(68030, 68031), CharacterRange::new(68096, 68096),
    CharacterRange::new(68112, 68115), CharacterRange::new(68117, 68119), CharacterRange::new(68121, 68149), CharacterRange::new(68192, 68220), CharacterRange::new(68224, 68252), CharacterRange::new(68288, 68295),
    CharacterRange::new(68297, 68324), CharacterRange::new(68352, 68405), CharacterRange::new(68416, 68437), CharacterRange::new(68448, 68466), CharacterRange::new(68480, 68497), CharacterRange::new(68608, 68680),
    CharacterRange::new(68800, 68850), CharacterRange::new(68864, 68899), CharacterRange::new(68938, 68941), CharacterRange::new(68943, 68943), CharacterRange::new(68976, 68997), CharacterRange::new(69248, 69289),
    CharacterRange::new(69296, 69297), CharacterRange::new(69314, 69316), CharacterRange::new(69376, 69404), CharacterRange::new(69415, 69415), CharacterRange::new(69424, 69445), CharacterRange::new(69488, 69505),
    CharacterRange::new(69552, 69572), CharacterRange::new(69600, 69622), CharacterRange::new(69635, 69687), CharacterRange::new(69745, 69746), CharacterRange::new(69749, 69749), CharacterRange::new(69763, 69807),
    CharacterRange::new(69840, 69864), CharacterRange::new(69891, 69926), CharacterRange::new(69956, 69956), CharacterRange::new(69959, 69959), CharacterRange::new(69968, 70002), CharacterRange::new(70006, 70006),
    CharacterRange::new(70019, 70066), CharacterRange::new(70081, 70084), CharacterRange::new(70106, 70106), CharacterRange::new(70108, 70108), CharacterRange::new(70144, 70161), CharacterRange::new(70163, 70187),
    CharacterRange::new(70207, 70208), CharacterRange::new(70272, 70278), CharacterRange::new(70280, 70280), CharacterRange::new(70282, 70285), CharacterRange::new(70287, 70301), CharacterRange::new(70303, 70312),
    CharacterRange::new(70320, 70366), CharacterRange::new(70405, 70412), CharacterRange::new(70415, 70416), CharacterRange::new(70419, 70440), CharacterRange::new(70442, 70448), CharacterRange::new(70450, 70451),
    CharacterRange::new(70453, 70457), CharacterRange::new(70461, 70461), CharacterRange::new(70480, 70480), CharacterRange::new(70493, 70497), CharacterRange::new(70528, 70537), CharacterRange::new(70539, 70539),
    CharacterRange::new(70542, 70542), CharacterRange::new(70544, 70581), CharacterRange::new(70583, 70583), CharacterRange::new(70609, 70609), CharacterRange::new(70611, 70611), CharacterRange::new(70656, 70708),
    CharacterRange::new(70727, 70730), CharacterRange::new(70751, 70753), CharacterRange::new(70784, 70831), CharacterRange::new(70852, 70853), CharacterRange::new(70855, 70855), CharacterRange::new(71040, 71086),
    CharacterRange::new(71128, 71131), CharacterRange::new(71168, 71215), CharacterRange::new(71236, 71236), CharacterRange::new(71296, 71338), CharacterRange::new(71352, 71352), CharacterRange::new(71424, 71450),
    CharacterRange::new(71488, 71494), CharacterRange::new(71680, 71723), CharacterRange::new(71872, 71903), CharacterRange::new(71935, 71942), CharacterRange::new(71945, 71945), CharacterRange::new(71948, 71955),
    CharacterRange::new(71957, 71958), CharacterRange::new(71960, 71983), CharacterRange::new(71999, 71999), CharacterRange::new(72001, 72001), CharacterRange::new(72096, 72103), CharacterRange::new(72106, 72144),
    CharacterRange::new(72161, 72161), CharacterRange::new(72163, 72163), CharacterRange::new(72192, 72192), CharacterRange::new(72203, 72242), CharacterRange::new(72250, 72250), CharacterRange::new(72272, 72272),
    CharacterRange::new(72284, 72329), CharacterRange::new(72349, 72349), CharacterRange::new(72368, 72440), CharacterRange::new(72640, 72672), CharacterRange::new(72704, 72712), CharacterRange::new(72714, 72750),
    CharacterRange::new(72768, 72768), CharacterRange::new(72818, 72847), CharacterRange::new(72960, 72966), CharacterRange::new(72968, 72969), CharacterRange::new(72971, 73008), CharacterRange::new(73030, 73030),
    CharacterRange::new(73056, 73061), CharacterRange::new(73063, 73064), CharacterRange::new(73066, 73097), CharacterRange::new(73112, 73112), CharacterRange::new(73440, 73458), CharacterRange::new(73474, 73474),
    CharacterRange::new(73476, 73488), CharacterRange::new(73490, 73523), CharacterRange::new(73648, 73648), CharacterRange::new(73728, 74649), CharacterRange::new(74880, 75075), CharacterRange::new(77712, 77808),
    CharacterRange::new(77824, 78895), CharacterRange::new(78913, 78918), CharacterRange::new(78944, 82938), CharacterRange::new(82944, 83526), CharacterRange::new(90368, 90397), CharacterRange::new(92160, 92728),
    CharacterRange::new(92736, 92766), CharacterRange::new(92784, 92862), CharacterRange::new(92880, 92909), CharacterRange::new(92928, 92975), CharacterRange::new(93027, 93047), CharacterRange::new(93053, 93071),
    CharacterRange::new(93507, 93546), CharacterRange::new(93792, 93823), CharacterRange::new(93952, 94026), CharacterRange::new(94032, 94032), CharacterRange::new(94208, 100343), CharacterRange::new(100352, 101589),
    CharacterRange::new(101631, 101640), CharacterRange::new(110592, 110882), CharacterRange::new(110898, 110898), CharacterRange::new(110928, 110930), CharacterRange::new(110933, 110933), CharacterRange::new(110948, 110951),
    CharacterRange::new(110960, 111355), CharacterRange::new(113664, 113770), CharacterRange::new(113776, 113788), CharacterRange::new(113792, 113800), CharacterRange::new(113808, 113817), CharacterRange::new(119834, 119859),
    CharacterRange::new(119886, 119892), CharacterRange::new(119894, 119911), CharacterRange::new(119938, 119963), CharacterRange::new(119990, 119993), CharacterRange::new(119995, 119995), CharacterRange::new(119997, 120003),
    CharacterRange::new(120005, 120015), CharacterRange::new(120042, 120067), CharacterRange::new(120094, 120119), CharacterRange::new(120146, 120171), CharacterRange::new(120198, 120223), CharacterRange::new(120250, 120275),
    CharacterRange::new(120302, 120327), CharacterRange::new(120354, 120379), CharacterRange::new(120406, 120431), CharacterRange::new(120458, 120485), CharacterRange::new(120514, 120538), CharacterRange::new(120540, 120545),
    CharacterRange::new(120572, 120596), CharacterRange::new(120598, 120603), CharacterRange::new(120630, 120654), CharacterRange::new(120656, 120661), CharacterRange::new(120688, 120712), CharacterRange::new(120714, 120719),
    CharacterRange::new(120746, 120770), CharacterRange::new(120772, 120777), CharacterRange::new(120779, 120779), CharacterRange::new(122624, 122654), CharacterRange::new(122661, 122666), CharacterRange::new(123136, 123180),
    CharacterRange::new(123214, 123214), CharacterRange::new(123536, 123565), CharacterRange::new(123584, 123627), CharacterRange::new(124112, 124138), CharacterRange::new(124368, 124397), CharacterRange::new(124400, 124400),
    CharacterRange::new(124896, 124902), CharacterRange::new(124904, 124907), CharacterRange::new(124909, 124910), CharacterRange::new(124912, 124926), CharacterRange::new(124928, 125124), CharacterRange::new(125218, 125251),
    CharacterRange::new(126464, 126467), CharacterRange::new(126469, 126495), CharacterRange::new(126497, 126498), CharacterRange::new(126500, 126500), CharacterRange::new(126503, 126503), CharacterRange::new(126505, 126514),
    CharacterRange::new(126516, 126519), CharacterRange::new(126521, 126521), CharacterRange::new(126523, 126523), CharacterRange::new(126530, 126530), CharacterRange::new(126535, 126535), CharacterRange::new(126537, 126537),
    CharacterRange::new(126539, 126539), CharacterRange::new(126541, 126543), CharacterRange::new(126545, 126546), CharacterRange::new(126548, 126548), CharacterRange::new(126551, 126551), CharacterRange::new(126553, 126553),
    CharacterRange::new(126555, 126555), CharacterRange::new(126557, 126557), CharacterRange::new(126559, 126559), CharacterRange::new(126561, 126562), CharacterRange::new(126564, 126564), CharacterRange::new(126567, 126570),
    CharacterRange::new(126572, 126578), CharacterRange::new(126580, 126583), CharacterRange::new(126585, 126588), CharacterRange::new(126590, 126590), CharacterRange::new(126592, 126601), CharacterRange::new(126603, 126619),
    CharacterRange::new(126625, 126627), CharacterRange::new(126629, 126633), CharacterRange::new(126635, 126651), CharacterRange::new(131072, 173791), CharacterRange::new(173824, 177977), CharacterRange::new(177984, 178205),
    CharacterRange::new(178208, 183969), CharacterRange::new(183984, 191456), CharacterRange::new(191472, 192093), CharacterRange::new(194560, 195101), CharacterRange::new(196608, 201546), CharacterRange::new(201552, 205743),
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
                if eof { state = 72; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (10, 174), (33, 167), (34, 26), (35, 147), (36, 99), (37, 169), (39, 77), (40, 170),
                    (41, 171), (42, 79), (44, 74), (45, 93), (46, 83), (48, 124), (49, 95), (58, 33),
                    (59, 73), (60, 31), (61, 85), (63, 70), (64, 166), (91, 101), (92, 91), (93, 172),
                    (96, 140), (123, 75), (124, 150), (125, 76), (126, 168), (8592, 163), (8594, 159), (8658, 158),
                    (8704, 81), (8759, 165), (8888, 161), (9733, 80), (10214, 104), (10215, 103),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 50 <= lookahead && lookahead <= 57 { state = 125; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 32 || lookahead == 160 || lookahead == 5760 || 8192 <= lookahead && lookahead <= 8202 || lookahead == 8239 || lookahead == 8287 || lookahead == 12288 { state = 71; lexer.advance(true); continue; }
                if set_contains(&sym_name_character_set_1, lookahead) { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_variable_character_set_1, lookahead) { state = 152; lexer.advance(false); continue; }
                return result;
            }
            1 => {
                if let Some(next) = advance_map(&[
                    (10, 174), (34, 26), (35, 147), (39, 77), (40, 170), (41, 171), (42, 79), (44, 74),
                    (45, 93), (48, 124), (58, 33), (61, 34), (63, 70), (91, 101), (92, 91), (93, 172),
                    (96, 140), (123, 75), (124, 149), (125, 76), (8594, 159), (8658, 158), (8704, 81), (8759, 165),
                    (8888, 161), (9733, 80), (10214, 104),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 125; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 32 || lookahead == 160 || lookahead == 5760 || 8192 <= lookahead && lookahead <= 8202 || lookahead == 8239 || lookahead == 8287 || lookahead == 12288 { state = 2; lexer.advance(true); continue; }
                if set_contains(&sym_name_character_set_1, lookahead) { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_variable_character_set_1, lookahead) { state = 152; lexer.advance(false); continue; }
                return result;
            }
            2 => {
                if let Some(next) = advance_map(&[
                    (10, 174), (34, 26), (35, 143), (39, 77), (40, 170), (41, 171), (42, 79), (44, 74),
                    (45, 93), (48, 124), (58, 33), (61, 34), (63, 70), (91, 101), (92, 91), (93, 172),
                    (96, 140), (123, 75), (124, 88), (125, 76), (8594, 159), (8658, 158), (8704, 81), (8759, 165),
                    (8888, 161), (9733, 80), (10214, 104),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 125; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 32 || lookahead == 160 || lookahead == 5760 || 8192 <= lookahead && lookahead <= 8202 || lookahead == 8239 || lookahead == 8287 || lookahead == 12288 { state = 2; lexer.advance(true); continue; }
                if set_contains(&sym_name_character_set_1, lookahead) { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_variable_character_set_1, lookahead) { state = 152; lexer.advance(false); continue; }
                return result;
            }
            3 => {
                if let Some(next) = advance_map(&[
                    (10, 174), (34, 26), (35, 148), (39, 77), (40, 170), (41, 171), (42, 79), (44, 74),
                    (45, 93), (46, 32), (48, 124), (58, 33), (59, 73), (60, 31), (61, 84), (63, 70),
                    (91, 101), (92, 91), (93, 172), (96, 140), (123, 75), (124, 37), (125, 76), (8592, 163),
                    (8594, 159), (8704, 81), (8759, 165), (8888, 161), (9733, 80), (10214, 104), (10215, 103),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 125; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 32 || lookahead == 160 || lookahead == 5760 || 8192 <= lookahead && lookahead <= 8202 || lookahead == 8239 || lookahead == 8287 || lookahead == 12288 { state = 6; lexer.advance(true); continue; }
                if set_contains(&sym_name_character_set_1, lookahead) { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_variable_character_set_1, lookahead) { state = 152; lexer.advance(false); continue; }
                return result;
            }
            4 => {
                if let Some(next) = advance_map(&[
                    (10, 174), (34, 26), (35, 148), (39, 77), (40, 170), (41, 171), (42, 79), (44, 74),
                    (45, 93), (46, 82), (48, 124), (58, 33), (59, 73), (60, 31), (61, 84), (63, 70),
                    (91, 101), (92, 91), (93, 172), (96, 140), (123, 75), (124, 88), (125, 76), (8592, 163),
                    (8594, 159), (8759, 165), (8888, 161), (9733, 80), (10214, 104),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 125; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 32 || lookahead == 160 || lookahead == 5760 || 8192 <= lookahead && lookahead <= 8202 || lookahead == 8239 || lookahead == 8287 || lookahead == 12288 { state = 7; lexer.advance(true); continue; }
                if set_contains(&sym_name_character_set_1, lookahead) { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_variable_character_set_1, lookahead) { state = 152; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                if let Some(next) = advance_map(&[
                    (10, 174), (34, 26), (35, 148), (39, 77), (40, 170), (41, 171), (45, 92), (48, 124),
                    (63, 70), (91, 101), (92, 91), (124, 149), (10214, 104),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 125; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 32 || lookahead == 160 || lookahead == 5760 || 8192 <= lookahead && lookahead <= 8202 || lookahead == 8239 || lookahead == 8287 || lookahead == 12288 { state = 20; lexer.advance(true); continue; }
                if set_contains(&sym_name_character_set_1, lookahead) { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_variable_character_set_1, lookahead) { state = 152; lexer.advance(false); continue; }
                return result;
            }
            6 => {
                if let Some(next) = advance_map(&[
                    (10, 174), (34, 26), (35, 144), (39, 77), (40, 170), (41, 171), (42, 79), (44, 74),
                    (45, 93), (46, 32), (48, 124), (58, 33), (59, 73), (60, 31), (61, 84), (63, 70),
                    (91, 101), (92, 91), (93, 172), (96, 140), (123, 75), (124, 37), (125, 76), (8592, 163),
                    (8594, 159), (8704, 81), (8759, 165), (8888, 161), (9733, 80), (10214, 104), (10215, 103),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 125; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 32 || lookahead == 160 || lookahead == 5760 || 8192 <= lookahead && lookahead <= 8202 || lookahead == 8239 || lookahead == 8287 || lookahead == 12288 { state = 6; lexer.advance(true); continue; }
                if set_contains(&sym_name_character_set_1, lookahead) { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_variable_character_set_1, lookahead) { state = 152; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if let Some(next) = advance_map(&[
                    (10, 174), (34, 26), (35, 144), (39, 77), (40, 170), (41, 171), (42, 79), (44, 74),
                    (45, 93), (46, 82), (48, 124), (58, 33), (59, 73), (60, 31), (61, 84), (63, 70),
                    (91, 101), (92, 91), (93, 172), (96, 140), (123, 75), (124, 88), (125, 76), (8592, 163),
                    (8594, 159), (8759, 165), (8888, 161), (9733, 80), (10214, 104),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 125; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 32 || lookahead == 160 || lookahead == 5760 || 8192 <= lookahead && lookahead <= 8202 || lookahead == 8239 || lookahead == 8287 || lookahead == 12288 { state = 7; lexer.advance(true); continue; }
                if set_contains(&sym_name_character_set_1, lookahead) { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_variable_character_set_1, lookahead) { state = 152; lexer.advance(false); continue; }
                return result;
            }
            8 => {
                if let Some(next) = advance_map(&[
                    (10, 174), (34, 26), (35, 145), (39, 78), (40, 170), (41, 171), (42, 79), (44, 74),
                    (45, 93), (48, 124), (58, 33), (59, 73), (60, 31), (61, 84), (63, 70), (91, 101),
                    (93, 172), (96, 140), (123, 75), (124, 37), (125, 76), (8592, 163), (8594, 159), (8704, 81),
                    (8759, 165), (8888, 161), (9733, 80), (10215, 103),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 125; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 32 || lookahead == 160 || lookahead == 5760 || 8192 <= lookahead && lookahead <= 8202 || lookahead == 8239 || lookahead == 8287 || lookahead == 12288 { state = 12; lexer.advance(true); continue; }
                if set_contains(&sym_name_character_set_1, lookahead) { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_variable_character_set_1, lookahead) { state = 152; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if let Some(next) = advance_map(&[
                    (10, 174), (34, 26), (35, 145), (39, 78), (40, 170), (41, 171), (42, 79), (44, 74),
                    (45, 92), (48, 124), (58, 33), (59, 73), (61, 85), (91, 101), (96, 140), (124, 149),
                    (8658, 158), (8759, 165), (9733, 80),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 125; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 32 || lookahead == 160 || lookahead == 5760 || 8192 <= lookahead && lookahead <= 8202 || lookahead == 8239 || lookahead == 8287 || lookahead == 12288 { state = 13; lexer.advance(true); continue; }
                if set_contains(&sym_name_character_set_1, lookahead) { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_variable_character_set_1, lookahead) { state = 152; lexer.advance(false); continue; }
                return result;
            }
            10 => {
                if let Some(next) = advance_map(&[
                    (10, 174), (34, 26), (35, 145), (39, 78), (40, 170), (41, 171), (42, 79), (44, 74),
                    (45, 92), (48, 124), (58, 33), (61, 34), (91, 101), (96, 140), (124, 90), (8658, 158),
                    (8759, 165), (9733, 80),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 125; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 32 || lookahead == 160 || lookahead == 5760 || 8192 <= lookahead && lookahead <= 8202 || lookahead == 8239 || lookahead == 8287 || lookahead == 12288 { state = 14; lexer.advance(true); continue; }
                if set_contains(&sym_name_character_set_1, lookahead) { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_variable_character_set_1, lookahead) { state = 152; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if let Some(next) = advance_map(&[
                    (10, 174), (34, 26), (35, 145), (39, 78), (40, 170), (42, 79), (44, 74), (45, 93),
                    (48, 124), (58, 33), (59, 73), (61, 84), (91, 101), (93, 172), (96, 140), (123, 75),
                    (124, 88), (8594, 159), (8759, 165), (8888, 161), (9733, 80),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 125; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 32 || lookahead == 160 || lookahead == 5760 || 8192 <= lookahead && lookahead <= 8202 || lookahead == 8239 || lookahead == 8287 || lookahead == 12288 { state = 15; lexer.advance(true); continue; }
                if set_contains(&sym_name_character_set_1, lookahead) { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_variable_character_set_1, lookahead) { state = 152; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if let Some(next) = advance_map(&[
                    (10, 174), (34, 26), (35, 141), (39, 78), (40, 170), (41, 171), (42, 79), (44, 74),
                    (45, 93), (48, 124), (58, 33), (59, 73), (60, 31), (61, 84), (63, 70), (91, 101),
                    (93, 172), (96, 140), (123, 75), (124, 37), (125, 76), (8592, 163), (8594, 159), (8704, 81),
                    (8759, 165), (8888, 161), (9733, 80), (10215, 103),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 125; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 32 || lookahead == 160 || lookahead == 5760 || 8192 <= lookahead && lookahead <= 8202 || lookahead == 8239 || lookahead == 8287 || lookahead == 12288 { state = 12; lexer.advance(true); continue; }
                if set_contains(&sym_name_character_set_1, lookahead) { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_variable_character_set_1, lookahead) { state = 152; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if let Some(next) = advance_map(&[
                    (10, 174), (34, 26), (35, 141), (39, 78), (40, 170), (41, 171), (42, 79), (44, 74),
                    (45, 92), (48, 124), (58, 33), (59, 73), (61, 85), (91, 101), (96, 140), (8658, 158),
                    (8759, 165), (9733, 80),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 125; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 32 || lookahead == 160 || lookahead == 5760 || 8192 <= lookahead && lookahead <= 8202 || lookahead == 8239 || lookahead == 8287 || lookahead == 12288 { state = 13; lexer.advance(true); continue; }
                if set_contains(&sym_name_character_set_1, lookahead) { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_variable_character_set_1, lookahead) { state = 152; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if let Some(next) = advance_map(&[
                    (10, 174), (34, 26), (35, 141), (39, 78), (40, 170), (41, 171), (42, 79), (44, 74),
                    (45, 92), (48, 124), (58, 33), (61, 34), (91, 101), (96, 140), (124, 90), (8658, 158),
                    (8759, 165), (9733, 80),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 125; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 32 || lookahead == 160 || lookahead == 5760 || 8192 <= lookahead && lookahead <= 8202 || lookahead == 8239 || lookahead == 8287 || lookahead == 12288 { state = 14; lexer.advance(true); continue; }
                if set_contains(&sym_name_character_set_1, lookahead) { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_variable_character_set_1, lookahead) { state = 152; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if let Some(next) = advance_map(&[
                    (10, 174), (34, 26), (35, 141), (39, 78), (40, 170), (42, 79), (44, 74), (45, 93),
                    (48, 124), (58, 33), (59, 73), (61, 84), (91, 101), (93, 172), (96, 140), (123, 75),
                    (124, 88), (8594, 159), (8759, 165), (8888, 161), (9733, 80),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 125; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 32 || lookahead == 160 || lookahead == 5760 || 8192 <= lookahead && lookahead <= 8202 || lookahead == 8239 || lookahead == 8287 || lookahead == 12288 { state = 15; lexer.advance(true); continue; }
                if set_contains(&sym_name_character_set_1, lookahead) { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_variable_character_set_1, lookahead) { state = 152; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if let Some(next) = advance_map(&[
                    (10, 174), (34, 26), (35, 146), (39, 78), (40, 170), (41, 171), (42, 79), (44, 74),
                    (45, 93), (48, 124), (58, 33), (61, 85), (63, 70), (91, 101), (96, 140), (123, 75),
                    (124, 149), (8594, 159), (8658, 158), (8704, 81), (8759, 165), (8888, 161), (9733, 80),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 125; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 32 || lookahead == 160 || lookahead == 5760 || 8192 <= lookahead && lookahead <= 8202 || lookahead == 8239 || lookahead == 8287 || lookahead == 12288 { state = 17; lexer.advance(true); continue; }
                if set_contains(&sym_name_character_set_1, lookahead) { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_variable_character_set_1, lookahead) { state = 152; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if let Some(next) = advance_map(&[
                    (10, 174), (34, 26), (35, 142), (39, 78), (40, 170), (41, 171), (42, 79), (44, 74),
                    (45, 93), (48, 124), (58, 33), (61, 85), (63, 70), (91, 101), (96, 140), (123, 75),
                    (124, 88), (8594, 159), (8658, 158), (8704, 81), (8759, 165), (8888, 161), (9733, 80),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 125; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 32 || lookahead == 160 || lookahead == 5760 || 8192 <= lookahead && lookahead <= 8202 || lookahead == 8239 || lookahead == 8287 || lookahead == 12288 { state = 17; lexer.advance(true); continue; }
                if set_contains(&sym_name_character_set_1, lookahead) { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_variable_character_set_1, lookahead) { state = 152; lexer.advance(false); continue; }
                return result;
            }
            18 => {
                if let Some(next) = advance_map(&[
                    (10, 174), (34, 26), (35, 30), (39, 36), (40, 170), (41, 171), (44, 74), (45, 93),
                    (48, 124), (58, 33), (59, 73), (60, 31), (61, 84), (91, 101), (93, 172), (96, 140),
                    (123, 75), (124, 150), (125, 76), (8592, 163), (8594, 159), (8759, 165), (8888, 161), (10215, 103),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 125; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 32 || lookahead == 160 || lookahead == 5760 || 8192 <= lookahead && lookahead <= 8202 || lookahead == 8239 || lookahead == 8287 || lookahead == 12288 { state = 19; lexer.advance(true); continue; }
                if set_contains(&sym_name_character_set_1, lookahead) { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_variable_character_set_1, lookahead) { state = 152; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if let Some(next) = advance_map(&[
                    (10, 174), (34, 26), (35, 30), (39, 36), (40, 170), (41, 171), (44, 74), (45, 93),
                    (48, 124), (58, 33), (59, 73), (60, 31), (61, 84), (91, 101), (93, 172), (96, 140),
                    (123, 75), (124, 89), (125, 76), (8592, 163), (8594, 159), (8759, 165), (8888, 161), (10215, 103),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 125; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 32 || lookahead == 160 || lookahead == 5760 || 8192 <= lookahead && lookahead <= 8202 || lookahead == 8239 || lookahead == 8287 || lookahead == 12288 { state = 19; lexer.advance(true); continue; }
                if set_contains(&sym_name_character_set_1, lookahead) { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_variable_character_set_1, lookahead) { state = 152; lexer.advance(false); continue; }
                return result;
            }
            20 => {
                if let Some(next) = advance_map(&[
                    (10, 174), (34, 26), (35, 69), (39, 77), (40, 170), (41, 171), (45, 92), (48, 124),
                    (63, 70), (91, 101), (92, 91), (10214, 104),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 125; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 32 || lookahead == 160 || lookahead == 5760 || 8192 <= lookahead && lookahead <= 8202 || lookahead == 8239 || lookahead == 8287 || lookahead == 12288 { state = 20; lexer.advance(true); continue; }
                if set_contains(&sym_name_character_set_1, lookahead) { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_variable_character_set_1, lookahead) { state = 152; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                if let Some(next) = advance_map(&[
                    (10, 174), (35, 145), (42, 79), (45, 92), (48, 126), (96, 140), (99, 39), (106, 40),
                    (112, 54), (115, 57),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 127; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 32 || lookahead == 160 || lookahead == 5760 || 8192 <= lookahead && lookahead <= 8202 || lookahead == 8239 || lookahead == 8287 || lookahead == 12288 { state = 22; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 { state = 98; lexer.advance(false); continue; }
                return result;
            }
            22 => {
                if let Some(next) = advance_map(&[
                    (10, 174), (35, 141), (42, 79), (45, 92), (48, 126), (96, 140), (99, 39), (106, 40),
                    (112, 54), (115, 57),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 127; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 32 || lookahead == 160 || lookahead == 5760 || 8192 <= lookahead && lookahead <= 8202 || lookahead == 8239 || lookahead == 8287 || lookahead == 12288 { state = 22; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 { state = 98; lexer.advance(false); continue; }
                return result;
            }
            23 => {
                if lookahead == 10 { state = 174; lexer.advance(false); continue; }
                if lookahead == 44 { state = 74; lexer.advance(false); continue; }
                if lookahead == 49 { state = 94; lexer.advance(false); continue; }
                if lookahead == 59 { state = 73; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 32 || lookahead == 160 || lookahead == 5760 || 8192 <= lookahead && lookahead <= 8202 || lookahead == 8239 || lookahead == 8287 || lookahead == 12288 { state = 23; lexer.advance(true); continue; }
                return result;
            }
            24 => {
                if lookahead == 10 { state = 35; lexer.advance(false); continue; }
                if lookahead == 34 { state = 119; lexer.advance(false); continue; }
                if lookahead == 92 { state = 24; lexer.advance(false); continue; }
                if lookahead == 94 { state = 27; lexer.advance(false); continue; }
                if lookahead != 0 { state = 26; lexer.advance(false); continue; }
                return result;
            }
            25 => {
                if lookahead == 10 { state = 35; lexer.advance(false); continue; }
                if lookahead == 94 { state = 27; lexer.advance(false); continue; }
                if lookahead != 0 { state = 26; lexer.advance(false); continue; }
                return result;
            }
            26 => {
                if lookahead == 34 { state = 122; lexer.advance(false); continue; }
                if lookahead == 92 { state = 25; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 26; lexer.advance(false); continue; }
                return result;
            }
            27 => {
                if lookahead == 34 { state = 119; lexer.advance(false); continue; }
                if lookahead == 92 { state = 24; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 26; lexer.advance(false); continue; }
                return result;
            }
            28 => {
                if lookahead == 39 { state = 114; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 32 { state = 28; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                if lookahead == 39 { state = 115; lexer.advance(false); continue; }
                return result;
            }
            30 => {
                if lookahead == 41 { state = 173; lexer.advance(false); continue; }
                return result;
            }
            31 => {
                if lookahead == 45 { state = 162; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                if lookahead == 46 { state = 86; lexer.advance(false); continue; }
                return result;
            }
            33 => {
                if lookahead == 58 { state = 164; lexer.advance(false); continue; }
                return result;
            }
            34 => {
                if lookahead == 62 { state = 157; lexer.advance(false); continue; }
                return result;
            }
            35 => {
                if lookahead == 92 { state = 26; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 35; lexer.advance(false); continue; }
                return result;
            }
            36 => {
                if lookahead == 92 { state = 28; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 39 { state = 29; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                if lookahead == 93 { state = 102; lexer.advance(false); continue; }
                if lookahead == 124 { state = 38; lexer.advance(false); continue; }
                return result;
            }
            38 => {
                if lookahead == 93 { state = 106; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                if lookahead == 97 { state = 53; lexer.advance(false); continue; }
                if lookahead == 99 { state = 41; lexer.advance(false); continue; }
                return result;
            }
            40 => {
                if lookahead == 97 { state = 59; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                if lookahead == 97 { state = 50; lexer.advance(false); continue; }
                return result;
            }
            42 => {
                if lookahead == 97 { state = 56; lexer.advance(false); continue; }
                return result;
            }
            43 => {
                if lookahead == 99 { state = 41; lexer.advance(false); continue; }
                return result;
            }
            44 => {
                if lookahead == 99 { state = 55; lexer.advance(false); continue; }
                return result;
            }
            45 => {
                if lookahead == 100 { state = 43; lexer.advance(false); continue; }
                return result;
            }
            46 => {
                if lookahead == 105 { state = 51; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                if lookahead == 105 { state = 97; lexer.advance(false); continue; }
                return result;
            }
            48 => {
                if lookahead == 105 { state = 52; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                if lookahead == 108 { state = 97; lexer.advance(false); continue; }
                return result;
            }
            50 => {
                if lookahead == 108 { state = 49; lexer.advance(false); continue; }
                return result;
            }
            51 => {
                if lookahead == 109 { state = 97; lexer.advance(false); continue; }
                return result;
            }
            52 => {
                if lookahead == 112 { state = 58; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                if lookahead == 112 { state = 47; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                if lookahead == 114 { state = 46; lexer.advance(false); continue; }
                return result;
            }
            55 => {
                if lookahead == 114 { state = 48; lexer.advance(false); continue; }
                return result;
            }
            56 => {
                if lookahead == 115 { state = 44; lexer.advance(false); continue; }
                return result;
            }
            57 => {
                if lookahead == 116 { state = 45; lexer.advance(false); continue; }
                return result;
            }
            58 => {
                if lookahead == 116 { state = 97; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                if lookahead == 118 { state = 42; lexer.advance(false); continue; }
                return result;
            }
            60 => {
                if lookahead == 43 || lookahead == 45 { state = 65; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || lookahead == 95 { state = 109; lexer.advance(false); continue; }
                return result;
            }
            61 => {
                if lookahead == 43 || lookahead == 45 { state = 68; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || lookahead == 95 || 97 <= lookahead && lookahead <= 102 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            62 => {
                if lookahead == 48 || lookahead == 49 || lookahead == 95 { state = 130; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                if 48 <= lookahead && lookahead <= 55 { state = 133; lexer.advance(false); continue; }
                return result;
            }
            64 => {
                if 48 <= lookahead && lookahead <= 57 || lookahead == 95 { state = 108; lexer.advance(false); continue; }
                return result;
            }
            65 => {
                if 48 <= lookahead && lookahead <= 57 || lookahead == 95 { state = 109; lexer.advance(false); continue; }
                return result;
            }
            66 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || lookahead == 95 || 97 <= lookahead && lookahead <= 102 { state = 136; lexer.advance(false); continue; }
                return result;
            }
            67 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || lookahead == 95 || 97 <= lookahead && lookahead <= 102 { state = 137; lexer.advance(false); continue; }
                return result;
            }
            68 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || lookahead == 95 || 97 <= lookahead && lookahead <= 102 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            69 => {
                if set_contains(&sym_variable_character_set_1, lookahead) { state = 156; lexer.advance(false); continue; }
                return result;
            }
            70 => {
                if set_contains(&sym_variable_character_set_1, lookahead) { state = 153; lexer.advance(false); continue; }
                return result;
            }
            71 => {
                if eof { state = 72; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (10, 174), (33, 167), (34, 26), (35, 143), (36, 99), (37, 169), (39, 77), (40, 170),
                    (41, 171), (42, 79), (44, 74), (45, 93), (46, 83), (48, 124), (49, 95), (58, 33),
                    (59, 73), (60, 31), (61, 85), (63, 70), (64, 166), (91, 101), (92, 91), (93, 172),
                    (96, 140), (123, 75), (124, 89), (125, 76), (126, 168), (8592, 163), (8594, 159), (8658, 158),
                    (8704, 81), (8759, 165), (8888, 161), (9733, 80), (10214, 104), (10215, 103),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 50 <= lookahead && lookahead <= 57 { state = 125; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 32 || lookahead == 160 || lookahead == 5760 || 8192 <= lookahead && lookahead <= 8202 || lookahead == 8239 || lookahead == 8287 || lookahead == 12288 { state = 71; lexer.advance(true); continue; }
                if set_contains(&sym_name_character_set_1, lookahead) { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_variable_character_set_1, lookahead) { state = 152; lexer.advance(false); continue; }
                return result;
            }
            72 => {
                result = true; lexer.set_result_symbol(ts_builtin_sym_end); lexer.mark_end();
                return result;
            }
            73 => {
                result = true; lexer.set_result_symbol(anon_sym_SEMI); lexer.mark_end();
                return result;
            }
            74 => {
                result = true; lexer.set_result_symbol(anon_sym_COMMA); lexer.mark_end();
                return result;
            }
            75 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACE); lexer.mark_end();
                return result;
            }
            76 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACE); lexer.mark_end();
                return result;
            }
            77 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTE); lexer.mark_end();
                if lookahead == 39 { state = 87; lexer.advance(false); continue; }
                if lookahead == 92 { state = 28; lexer.advance(false); continue; }
                if lookahead != 0 { state = 29; lexer.advance(false); continue; }
                return result;
            }
            78 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTE); lexer.mark_end();
                if lookahead == 92 { state = 28; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 39 { state = 29; lexer.advance(false); continue; }
                return result;
            }
            79 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                return result;
            }
            80 => {
                result = true; lexer.set_result_symbol(anon_sym_u2605); lexer.mark_end();
                return result;
            }
            81 => {
                result = true; lexer.set_result_symbol(anon_sym_u2200); lexer.mark_end();
                return result;
            }
            82 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                return result;
            }
            83 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                if lookahead == 46 { state = 86; lexer.advance(false); continue; }
                return result;
            }
            84 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                return result;
            }
            85 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                if lookahead == 62 { state = 157; lexer.advance(false); continue; }
                return result;
            }
            86 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT_DOT); lexer.mark_end();
                return result;
            }
            87 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTE_SQUOTE); lexer.mark_end();
                return result;
            }
            88 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                return result;
            }
            89 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                if lookahead == 93 { state = 102; lexer.advance(false); continue; }
                return result;
            }
            90 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                if lookahead == 124 { state = 105; lexer.advance(false); continue; }
                return result;
            }
            91 => {
                result = true; lexer.set_result_symbol(anon_sym_BSLASH); lexer.mark_end();
                return result;
            }
            92 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                return result;
            }
            93 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 62 { state = 96; lexer.advance(false); continue; }
                return result;
            }
            94 => {
                result = true; lexer.set_result_symbol(anon_sym_1); lexer.mark_end();
                return result;
            }
            95 => {
                result = true; lexer.set_result_symbol(anon_sym_1); lexer.mark_end();
                if lookahead == 35 { state = 128; lexer.advance(false); continue; }
                if lookahead == 46 { state = 64; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 60; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || lookahead == 95 { state = 125; lexer.advance(false); continue; }
                return result;
            }
            96 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_GT); lexer.mark_end();
                if lookahead == 46 { state = 160; lexer.advance(false); continue; }
                return result;
            }
            97 => {
                result = true; lexer.set_result_symbol(sym_calling_convention); lexer.mark_end();
                return result;
            }
            98 => {
                result = true; lexer.set_result_symbol(sym_calling_convention); lexer.mark_end();
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 { state = 98; lexer.advance(false); continue; }
                return result;
            }
            99 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR); lexer.mark_end();
                if lookahead == 36 { state = 100; lexer.advance(false); continue; }
                return result;
            }
            100 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR_DOLLAR); lexer.mark_end();
                return result;
            }
            101 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK); lexer.mark_end();
                return result;
            }
            102 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE_RBRACK); lexer.mark_end();
                return result;
            }
            103 => {
                result = true; lexer.set_result_symbol(anon_sym_u27e7); lexer.mark_end();
                return result;
            }
            104 => {
                result = true; lexer.set_result_symbol(anon_sym_u27e6); lexer.mark_end();
                return result;
            }
            105 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE_PIPE); lexer.mark_end();
                return result;
            }
            106 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE_PIPE_RBRACK); lexer.mark_end();
                return result;
            }
            107 => {
                result = true; lexer.set_result_symbol(sym_float); lexer.mark_end();
                return result;
            }
            108 => {
                result = true; lexer.set_result_symbol(sym_float); lexer.mark_end();
                if lookahead == 35 { state = 110; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 60; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || lookahead == 95 { state = 108; lexer.advance(false); continue; }
                return result;
            }
            109 => {
                result = true; lexer.set_result_symbol(sym_float); lexer.mark_end();
                if lookahead == 35 { state = 110; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || lookahead == 95 { state = 109; lexer.advance(false); continue; }
                return result;
            }
            110 => {
                result = true; lexer.set_result_symbol(sym_float); lexer.mark_end();
                if lookahead == 35 { state = 107; lexer.advance(false); continue; }
                return result;
            }
            111 => {
                result = true; lexer.set_result_symbol(sym_char); lexer.mark_end();
                return result;
            }
            112 => {
                result = true; lexer.set_result_symbol(sym_char); lexer.mark_end();
                if lookahead == 35 { state = 116; lexer.advance(false); continue; }
                if lookahead == 39 { state = 114; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 32 { state = 28; lexer.advance(false); continue; }
                return result;
            }
            113 => {
                result = true; lexer.set_result_symbol(sym_char); lexer.mark_end();
                if lookahead == 35 { state = 111; lexer.advance(false); continue; }
                return result;
            }
            114 => {
                result = true; lexer.set_result_symbol(sym_char); lexer.mark_end();
                if lookahead == 35 { state = 112; lexer.advance(false); continue; }
                if lookahead == 39 { state = 114; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 32 { state = 28; lexer.advance(false); continue; }
                return result;
            }
            115 => {
                result = true; lexer.set_result_symbol(sym_char); lexer.mark_end();
                if lookahead == 35 { state = 113; lexer.advance(false); continue; }
                return result;
            }
            116 => {
                result = true; lexer.set_result_symbol(sym_char); lexer.mark_end();
                if lookahead == 39 { state = 114; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 32 { state = 28; lexer.advance(false); continue; }
                return result;
            }
            117 => {
                result = true; lexer.set_result_symbol(sym_string); lexer.mark_end();
                return result;
            }
            118 => {
                result = true; lexer.set_result_symbol(sym_string); lexer.mark_end();
                if lookahead == 34 { state = 122; lexer.advance(false); continue; }
                if lookahead == 35 { state = 120; lexer.advance(false); continue; }
                if lookahead == 92 { state = 25; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 26; lexer.advance(false); continue; }
                return result;
            }
            119 => {
                result = true; lexer.set_result_symbol(sym_string); lexer.mark_end();
                if lookahead == 34 { state = 122; lexer.advance(false); continue; }
                if lookahead == 35 { state = 118; lexer.advance(false); continue; }
                if lookahead == 92 { state = 25; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 26; lexer.advance(false); continue; }
                return result;
            }
            120 => {
                result = true; lexer.set_result_symbol(sym_string); lexer.mark_end();
                if lookahead == 34 { state = 122; lexer.advance(false); continue; }
                if lookahead == 92 { state = 25; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 26; lexer.advance(false); continue; }
                return result;
            }
            121 => {
                result = true; lexer.set_result_symbol(sym_string); lexer.mark_end();
                if lookahead == 35 { state = 117; lexer.advance(false); continue; }
                return result;
            }
            122 => {
                result = true; lexer.set_result_symbol(sym_string); lexer.mark_end();
                if lookahead == 35 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            123 => {
                result = true; lexer.set_result_symbol(sym__integer_literal); lexer.mark_end();
                return result;
            }
            124 => {
                result = true; lexer.set_result_symbol(sym__integer_literal); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (35, 128), (46, 64), (66, 62), (98, 62), (69, 60), (101, 60), (79, 63), (111, 63),
                    (88, 66), (120, 66),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || lookahead == 95 { state = 125; lexer.advance(false); continue; }
                return result;
            }
            125 => {
                result = true; lexer.set_result_symbol(sym__integer_literal); lexer.mark_end();
                if lookahead == 35 { state = 128; lexer.advance(false); continue; }
                if lookahead == 46 { state = 64; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 60; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || lookahead == 95 { state = 125; lexer.advance(false); continue; }
                return result;
            }
            126 => {
                result = true; lexer.set_result_symbol(sym__integer_literal); lexer.mark_end();
                if lookahead == 35 { state = 128; lexer.advance(false); continue; }
                if lookahead == 66 || lookahead == 98 { state = 62; lexer.advance(false); continue; }
                if lookahead == 79 || lookahead == 111 { state = 63; lexer.advance(false); continue; }
                if lookahead == 88 || lookahead == 120 { state = 66; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || lookahead == 95 { state = 127; lexer.advance(false); continue; }
                return result;
            }
            127 => {
                result = true; lexer.set_result_symbol(sym__integer_literal); lexer.mark_end();
                if lookahead == 35 { state = 128; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || lookahead == 95 { state = 127; lexer.advance(false); continue; }
                return result;
            }
            128 => {
                result = true; lexer.set_result_symbol(sym__integer_literal); lexer.mark_end();
                if lookahead == 35 { state = 123; lexer.advance(false); continue; }
                return result;
            }
            129 => {
                result = true; lexer.set_result_symbol(sym__binary_literal); lexer.mark_end();
                return result;
            }
            130 => {
                result = true; lexer.set_result_symbol(sym__binary_literal); lexer.mark_end();
                if lookahead == 35 { state = 131; lexer.advance(false); continue; }
                if lookahead == 48 || lookahead == 49 || lookahead == 95 { state = 130; lexer.advance(false); continue; }
                return result;
            }
            131 => {
                result = true; lexer.set_result_symbol(sym__binary_literal); lexer.mark_end();
                if lookahead == 35 { state = 129; lexer.advance(false); continue; }
                return result;
            }
            132 => {
                result = true; lexer.set_result_symbol(sym__octal_literal); lexer.mark_end();
                return result;
            }
            133 => {
                result = true; lexer.set_result_symbol(sym__octal_literal); lexer.mark_end();
                if lookahead == 35 { state = 134; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 55 { state = 133; lexer.advance(false); continue; }
                return result;
            }
            134 => {
                result = true; lexer.set_result_symbol(sym__octal_literal); lexer.mark_end();
                if lookahead == 35 { state = 132; lexer.advance(false); continue; }
                return result;
            }
            135 => {
                result = true; lexer.set_result_symbol(sym__hex_literal); lexer.mark_end();
                return result;
            }
            136 => {
                result = true; lexer.set_result_symbol(sym__hex_literal); lexer.mark_end();
                if lookahead == 35 { state = 139; lexer.advance(false); continue; }
                if lookahead == 46 { state = 67; lexer.advance(false); continue; }
                if lookahead == 80 || lookahead == 112 { state = 61; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || lookahead == 95 || 97 <= lookahead && lookahead <= 102 { state = 136; lexer.advance(false); continue; }
                return result;
            }
            137 => {
                result = true; lexer.set_result_symbol(sym__hex_literal); lexer.mark_end();
                if lookahead == 35 { state = 139; lexer.advance(false); continue; }
                if lookahead == 80 || lookahead == 112 { state = 61; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || lookahead == 95 || 97 <= lookahead && lookahead <= 102 { state = 137; lexer.advance(false); continue; }
                return result;
            }
            138 => {
                result = true; lexer.set_result_symbol(sym__hex_literal); lexer.mark_end();
                if lookahead == 35 { state = 139; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || lookahead == 95 || 97 <= lookahead && lookahead <= 102 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            139 => {
                result = true; lexer.set_result_symbol(sym__hex_literal); lexer.mark_end();
                if lookahead == 35 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            140 => {
                result = true; lexer.set_result_symbol(anon_sym_BQUOTE); lexer.mark_end();
                return result;
            }
            141 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND); lexer.mark_end();
                return result;
            }
            142 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND); lexer.mark_end();
                if lookahead == 41 { state = 173; lexer.advance(false); continue; }
                return result;
            }
            143 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND); lexer.mark_end();
                if lookahead == 41 { state = 173; lexer.advance(false); continue; }
                if set_contains(&sym_variable_character_set_1, lookahead) { state = 156; lexer.advance(false); continue; }
                return result;
            }
            144 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND); lexer.mark_end();
                if set_contains(&sym_variable_character_set_1, lookahead) { state = 156; lexer.advance(false); continue; }
                return result;
            }
            145 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND2); lexer.mark_end();
                return result;
            }
            146 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND2); lexer.mark_end();
                if lookahead == 41 { state = 173; lexer.advance(false); continue; }
                return result;
            }
            147 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND2); lexer.mark_end();
                if lookahead == 41 { state = 173; lexer.advance(false); continue; }
                if set_contains(&sym_variable_character_set_1, lookahead) { state = 156; lexer.advance(false); continue; }
                return result;
            }
            148 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND2); lexer.mark_end();
                if set_contains(&sym_variable_character_set_1, lookahead) { state = 156; lexer.advance(false); continue; }
                return result;
            }
            149 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE2); lexer.mark_end();
                return result;
            }
            150 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE2); lexer.mark_end();
                if lookahead == 93 { state = 102; lexer.advance(false); continue; }
                return result;
            }
            151 => {
                result = true; lexer.set_result_symbol(sym_variable); lexer.mark_end();
                if lookahead == 35 { state = 151; lexer.advance(false); continue; }
                return result;
            }
            152 => {
                result = true; lexer.set_result_symbol(sym_variable); lexer.mark_end();
                if lookahead == 35 { state = 151; lexer.advance(false); continue; }
                if set_contains(&sym_implicit_variable_character_set_1, lookahead) { state = 152; lexer.advance(false); continue; }
                return result;
            }
            153 => {
                result = true; lexer.set_result_symbol(sym_implicit_variable); lexer.mark_end();
                if set_contains(&sym_implicit_variable_character_set_1, lookahead) { state = 153; lexer.advance(false); continue; }
                return result;
            }
            154 => {
                result = true; lexer.set_result_symbol(sym_name); lexer.mark_end();
                if lookahead == 35 { state = 154; lexer.advance(false); continue; }
                return result;
            }
            155 => {
                result = true; lexer.set_result_symbol(sym_name); lexer.mark_end();
                if lookahead == 35 { state = 154; lexer.advance(false); continue; }
                if set_contains(&sym_implicit_variable_character_set_1, lookahead) { state = 155; lexer.advance(false); continue; }
                return result;
            }
            156 => {
                result = true; lexer.set_result_symbol(sym_label); lexer.mark_end();
                if set_contains(&sym_implicit_variable_character_set_1, lookahead) { state = 156; lexer.advance(false); continue; }
                return result;
            }
            157 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ_GT); lexer.mark_end();
                return result;
            }
            158 => {
                result = true; lexer.set_result_symbol(anon_sym_u21d2); lexer.mark_end();
                return result;
            }
            159 => {
                result = true; lexer.set_result_symbol(anon_sym_u2192); lexer.mark_end();
                return result;
            }
            160 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_GT_DOT); lexer.mark_end();
                return result;
            }
            161 => {
                result = true; lexer.set_result_symbol(anon_sym_u22b8); lexer.mark_end();
                return result;
            }
            162 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_DASH); lexer.mark_end();
                return result;
            }
            163 => {
                result = true; lexer.set_result_symbol(anon_sym_u2190); lexer.mark_end();
                return result;
            }
            164 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON_COLON); lexer.mark_end();
                return result;
            }
            165 => {
                result = true; lexer.set_result_symbol(anon_sym_u2237); lexer.mark_end();
                return result;
            }
            166 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                return result;
            }
            167 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG); lexer.mark_end();
                return result;
            }
            168 => {
                result = true; lexer.set_result_symbol(anon_sym_TILDE); lexer.mark_end();
                return result;
            }
            169 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT); lexer.mark_end();
                return result;
            }
            170 => {
                result = true; lexer.set_result_symbol(aux_sym__paren_open_token1); lexer.mark_end();
                return result;
            }
            171 => {
                result = true; lexer.set_result_symbol(aux_sym__paren_close_token1); lexer.mark_end();
                return result;
            }
            172 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACK); lexer.mark_end();
                return result;
            }
            173 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND_RPAREN); lexer.mark_end();
                return result;
            }
            174 => {
                result = true; lexer.set_result_symbol(aux_sym__token1); lexer.mark_end();
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
                    (95, 1), (97, 2), (98, 3), (99, 4), (100, 5), (101, 6), (102, 7), (103, 8),
                    (104, 9), (105, 10), (108, 11), (109, 12), (110, 13), (111, 14), (112, 15), (113, 16),
                    (114, 17), (115, 18), (116, 19), (117, 20), (118, 21), (119, 22),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 13 || lookahead == 32 || lookahead == 160 || lookahead == 5760 || 8192 <= lookahead && lookahead <= 8202 || lookahead == 8239 || lookahead == 8287 || lookahead == 12288 { state = 0; lexer.advance(true); continue; }
                return result;
            }
            1 => {
                result = true; lexer.set_result_symbol(anon_sym__); lexer.mark_end();
                return result;
            }
            2 => {
                if lookahead == 110 { state = 23; lexer.advance(false); continue; }
                if lookahead == 115 { state = 24; lexer.advance(false); continue; }
                return result;
            }
            3 => {
                if lookahead == 121 { state = 25; lexer.advance(false); continue; }
                return result;
            }
            4 => {
                if lookahead == 97 { state = 26; lexer.advance(false); continue; }
                if lookahead == 108 { state = 27; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                result = true; lexer.set_result_symbol(anon_sym_d); lexer.mark_end();
                if lookahead == 97 { state = 28; lexer.advance(false); continue; }
                if lookahead == 101 { state = 29; lexer.advance(false); continue; }
                if lookahead == 111 { state = 30; lexer.advance(false); continue; }
                return result;
            }
            6 => {
                result = true; lexer.set_result_symbol(anon_sym_e); lexer.mark_end();
                if lookahead == 108 { state = 31; lexer.advance(false); continue; }
                if lookahead == 120 { state = 32; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if lookahead == 97 { state = 33; lexer.advance(false); continue; }
                if lookahead == 111 { state = 34; lexer.advance(false); continue; }
                return result;
            }
            8 => {
                if lookahead == 114 { state = 35; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if lookahead == 105 { state = 36; lexer.advance(false); continue; }
                return result;
            }
            10 => {
                if lookahead == 102 { state = 37; lexer.advance(false); continue; }
                if lookahead == 109 { state = 38; lexer.advance(false); continue; }
                if lookahead == 110 { state = 39; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if lookahead == 101 { state = 40; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if lookahead == 100 { state = 41; lexer.advance(false); continue; }
                if lookahead == 111 { state = 42; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if lookahead == 101 { state = 43; lexer.advance(false); continue; }
                if lookahead == 111 { state = 44; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if lookahead == 102 { state = 45; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                result = true; lexer.set_result_symbol(anon_sym_p); lexer.mark_end();
                if lookahead == 97 { state = 46; lexer.advance(false); continue; }
                if lookahead == 104 { state = 47; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if lookahead == 117 { state = 48; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if lookahead == 101 { state = 49; lexer.advance(false); continue; }
                if lookahead == 111 { state = 50; lexer.advance(false); continue; }
                return result;
            }
            18 => {
                if lookahead == 97 { state = 51; lexer.advance(false); continue; }
                if lookahead == 116 { state = 52; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                result = true; lexer.set_result_symbol(anon_sym_t); lexer.mark_end();
                if lookahead == 104 { state = 53; lexer.advance(false); continue; }
                if lookahead == 121 { state = 54; lexer.advance(false); continue; }
                return result;
            }
            20 => {
                if lookahead == 110 { state = 55; lexer.advance(false); continue; }
                if lookahead == 115 { state = 56; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                if lookahead == 105 { state = 57; lexer.advance(false); continue; }
                return result;
            }
            22 => {
                if lookahead == 104 { state = 58; lexer.advance(false); continue; }
                return result;
            }
            23 => {
                if lookahead == 121 { state = 59; lexer.advance(false); continue; }
                return result;
            }
            24 => {
                result = true; lexer.set_result_symbol(anon_sym_as); lexer.mark_end();
                return result;
            }
            25 => {
                result = true; lexer.set_result_symbol(anon_sym_by); lexer.mark_end();
                return result;
            }
            26 => {
                if lookahead == 115 { state = 60; lexer.advance(false); continue; }
                return result;
            }
            27 => {
                if lookahead == 97 { state = 61; lexer.advance(false); continue; }
                return result;
            }
            28 => {
                if lookahead == 116 { state = 62; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                if lookahead == 102 { state = 63; lexer.advance(false); continue; }
                if lookahead == 114 { state = 64; lexer.advance(false); continue; }
                return result;
            }
            30 => {
                result = true; lexer.set_result_symbol(anon_sym_do); lexer.mark_end();
                return result;
            }
            31 => {
                if lookahead == 115 { state = 65; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                if lookahead == 112 { state = 66; lexer.advance(false); continue; }
                return result;
            }
            33 => {
                if lookahead == 109 { state = 67; lexer.advance(false); continue; }
                return result;
            }
            34 => {
                if lookahead == 114 { state = 68; lexer.advance(false); continue; }
                return result;
            }
            35 => {
                if lookahead == 111 { state = 69; lexer.advance(false); continue; }
                return result;
            }
            36 => {
                if lookahead == 100 { state = 70; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                result = true; lexer.set_result_symbol(anon_sym_if); lexer.mark_end();
                return result;
            }
            38 => {
                if lookahead == 112 { state = 71; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                result = true; lexer.set_result_symbol(anon_sym_in); lexer.mark_end();
                if lookahead == 102 { state = 72; lexer.advance(false); continue; }
                if lookahead == 115 { state = 73; lexer.advance(false); continue; }
                if lookahead == 116 { state = 74; lexer.advance(false); continue; }
                return result;
            }
            40 => {
                if lookahead == 116 { state = 75; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                if lookahead == 111 { state = 76; lexer.advance(false); continue; }
                return result;
            }
            42 => {
                if lookahead == 100 { state = 77; lexer.advance(false); continue; }
                return result;
            }
            43 => {
                if lookahead == 119 { state = 78; lexer.advance(false); continue; }
                return result;
            }
            44 => {
                if lookahead == 109 { state = 79; lexer.advance(false); continue; }
                return result;
            }
            45 => {
                result = true; lexer.set_result_symbol(anon_sym_of); lexer.mark_end();
                return result;
            }
            46 => {
                if lookahead == 116 { state = 80; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                if lookahead == 97 { state = 81; lexer.advance(false); continue; }
                return result;
            }
            48 => {
                if lookahead == 97 { state = 82; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                if lookahead == 99 { state = 83; lexer.advance(false); continue; }
                if lookahead == 112 { state = 84; lexer.advance(false); continue; }
                return result;
            }
            50 => {
                if lookahead == 108 { state = 85; lexer.advance(false); continue; }
                return result;
            }
            51 => {
                if lookahead == 102 { state = 86; lexer.advance(false); continue; }
                return result;
            }
            52 => {
                if lookahead == 111 { state = 87; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                if lookahead == 101 { state = 88; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                if lookahead == 112 { state = 89; lexer.advance(false); continue; }
                return result;
            }
            55 => {
                if lookahead == 115 { state = 90; lexer.advance(false); continue; }
                return result;
            }
            56 => {
                if lookahead == 105 { state = 91; lexer.advance(false); continue; }
                return result;
            }
            57 => {
                if lookahead == 97 { state = 92; lexer.advance(false); continue; }
                return result;
            }
            58 => {
                if lookahead == 101 { state = 93; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                if lookahead == 99 { state = 94; lexer.advance(false); continue; }
                return result;
            }
            60 => {
                if lookahead == 101 { state = 95; lexer.advance(false); continue; }
                return result;
            }
            61 => {
                if lookahead == 115 { state = 96; lexer.advance(false); continue; }
                return result;
            }
            62 => {
                if lookahead == 97 { state = 97; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                if lookahead == 97 { state = 98; lexer.advance(false); continue; }
                return result;
            }
            64 => {
                if lookahead == 105 { state = 99; lexer.advance(false); continue; }
                return result;
            }
            65 => {
                if lookahead == 101 { state = 100; lexer.advance(false); continue; }
                return result;
            }
            66 => {
                if lookahead == 111 { state = 101; lexer.advance(false); continue; }
                return result;
            }
            67 => {
                if lookahead == 105 { state = 102; lexer.advance(false); continue; }
                return result;
            }
            68 => {
                if lookahead == 97 { state = 103; lexer.advance(false); continue; }
                if lookahead == 101 { state = 104; lexer.advance(false); continue; }
                return result;
            }
            69 => {
                if lookahead == 117 { state = 105; lexer.advance(false); continue; }
                return result;
            }
            70 => {
                if lookahead == 105 { state = 106; lexer.advance(false); continue; }
                return result;
            }
            71 => {
                if lookahead == 111 { state = 107; lexer.advance(false); continue; }
                return result;
            }
            72 => {
                if lookahead == 105 { state = 108; lexer.advance(false); continue; }
                return result;
            }
            73 => {
                if lookahead == 116 { state = 109; lexer.advance(false); continue; }
                return result;
            }
            74 => {
                if lookahead == 101 { state = 110; lexer.advance(false); continue; }
                return result;
            }
            75 => {
                result = true; lexer.set_result_symbol(anon_sym_let); lexer.mark_end();
                return result;
            }
            76 => {
                result = true; lexer.set_result_symbol(anon_sym_mdo); lexer.mark_end();
                return result;
            }
            77 => {
                if lookahead == 117 { state = 111; lexer.advance(false); continue; }
                return result;
            }
            78 => {
                if lookahead == 116 { state = 112; lexer.advance(false); continue; }
                return result;
            }
            79 => {
                if lookahead == 105 { state = 113; lexer.advance(false); continue; }
                return result;
            }
            80 => {
                if lookahead == 116 { state = 114; lexer.advance(false); continue; }
                return result;
            }
            81 => {
                if lookahead == 110 { state = 115; lexer.advance(false); continue; }
                return result;
            }
            82 => {
                if lookahead == 108 { state = 116; lexer.advance(false); continue; }
                return result;
            }
            83 => {
                result = true; lexer.set_result_symbol(anon_sym_rec); lexer.mark_end();
                return result;
            }
            84 => {
                if lookahead == 114 { state = 117; lexer.advance(false); continue; }
                return result;
            }
            85 => {
                if lookahead == 101 { state = 118; lexer.advance(false); continue; }
                return result;
            }
            86 => {
                if lookahead == 101 { state = 119; lexer.advance(false); continue; }
                return result;
            }
            87 => {
                if lookahead == 99 { state = 120; lexer.advance(false); continue; }
                return result;
            }
            88 => {
                if lookahead == 110 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            89 => {
                if lookahead == 101 { state = 122; lexer.advance(false); continue; }
                return result;
            }
            90 => {
                if lookahead == 97 { state = 123; lexer.advance(false); continue; }
                return result;
            }
            91 => {
                if lookahead == 110 { state = 124; lexer.advance(false); continue; }
                return result;
            }
            92 => {
                result = true; lexer.set_result_symbol(anon_sym_via); lexer.mark_end();
                return result;
            }
            93 => {
                if lookahead == 114 { state = 125; lexer.advance(false); continue; }
                return result;
            }
            94 => {
                if lookahead == 108 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            95 => {
                result = true; lexer.set_result_symbol(anon_sym_case); lexer.mark_end();
                if lookahead == 115 { state = 127; lexer.advance(false); continue; }
                return result;
            }
            96 => {
                if lookahead == 115 { state = 128; lexer.advance(false); continue; }
                return result;
            }
            97 => {
                result = true; lexer.set_result_symbol(anon_sym_data); lexer.mark_end();
                return result;
            }
            98 => {
                if lookahead == 117 { state = 129; lexer.advance(false); continue; }
                return result;
            }
            99 => {
                if lookahead == 118 { state = 130; lexer.advance(false); continue; }
                return result;
            }
            100 => {
                result = true; lexer.set_result_symbol(anon_sym_else); lexer.mark_end();
                return result;
            }
            101 => {
                if lookahead == 114 { state = 131; lexer.advance(false); continue; }
                return result;
            }
            102 => {
                if lookahead == 108 { state = 132; lexer.advance(false); continue; }
                return result;
            }
            103 => {
                if lookahead == 108 { state = 133; lexer.advance(false); continue; }
                return result;
            }
            104 => {
                if lookahead == 105 { state = 134; lexer.advance(false); continue; }
                return result;
            }
            105 => {
                if lookahead == 112 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            106 => {
                if lookahead == 110 { state = 136; lexer.advance(false); continue; }
                return result;
            }
            107 => {
                if lookahead == 114 { state = 137; lexer.advance(false); continue; }
                return result;
            }
            108 => {
                if lookahead == 120 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            109 => {
                if lookahead == 97 { state = 139; lexer.advance(false); continue; }
                return result;
            }
            110 => {
                if lookahead == 114 { state = 140; lexer.advance(false); continue; }
                return result;
            }
            111 => {
                if lookahead == 108 { state = 141; lexer.advance(false); continue; }
                return result;
            }
            112 => {
                if lookahead == 121 { state = 142; lexer.advance(false); continue; }
                return result;
            }
            113 => {
                if lookahead == 110 { state = 143; lexer.advance(false); continue; }
                return result;
            }
            114 => {
                if lookahead == 101 { state = 144; lexer.advance(false); continue; }
                return result;
            }
            115 => {
                if lookahead == 116 { state = 145; lexer.advance(false); continue; }
                return result;
            }
            116 => {
                if lookahead == 105 { state = 146; lexer.advance(false); continue; }
                return result;
            }
            117 => {
                if lookahead == 101 { state = 147; lexer.advance(false); continue; }
                return result;
            }
            118 => {
                result = true; lexer.set_result_symbol(anon_sym_role); lexer.mark_end();
                return result;
            }
            119 => {
                result = true; lexer.set_result_symbol(sym_safety); lexer.mark_end();
                return result;
            }
            120 => {
                if lookahead == 107 { state = 148; lexer.advance(false); continue; }
                return result;
            }
            121 => {
                result = true; lexer.set_result_symbol(anon_sym_then); lexer.mark_end();
                return result;
            }
            122 => {
                result = true; lexer.set_result_symbol(anon_sym_type); lexer.mark_end();
                return result;
            }
            123 => {
                if lookahead == 102 { state = 149; lexer.advance(false); continue; }
                return result;
            }
            124 => {
                if lookahead == 103 { state = 150; lexer.advance(false); continue; }
                return result;
            }
            125 => {
                if lookahead == 101 { state = 151; lexer.advance(false); continue; }
                return result;
            }
            126 => {
                if lookahead == 97 { state = 152; lexer.advance(false); continue; }
                return result;
            }
            127 => {
                result = true; lexer.set_result_symbol(anon_sym_cases); lexer.mark_end();
                return result;
            }
            128 => {
                result = true; lexer.set_result_symbol(anon_sym_class); lexer.mark_end();
                return result;
            }
            129 => {
                if lookahead == 108 { state = 153; lexer.advance(false); continue; }
                return result;
            }
            130 => {
                if lookahead == 105 { state = 154; lexer.advance(false); continue; }
                return result;
            }
            131 => {
                if lookahead == 116 { state = 155; lexer.advance(false); continue; }
                return result;
            }
            132 => {
                if lookahead == 121 { state = 156; lexer.advance(false); continue; }
                return result;
            }
            133 => {
                if lookahead == 108 { state = 157; lexer.advance(false); continue; }
                return result;
            }
            134 => {
                if lookahead == 103 { state = 158; lexer.advance(false); continue; }
                return result;
            }
            135 => {
                result = true; lexer.set_result_symbol(anon_sym_group); lexer.mark_end();
                return result;
            }
            136 => {
                if lookahead == 103 { state = 159; lexer.advance(false); continue; }
                return result;
            }
            137 => {
                if lookahead == 116 { state = 160; lexer.advance(false); continue; }
                return result;
            }
            138 => {
                result = true; lexer.set_result_symbol(anon_sym_infix); lexer.mark_end();
                if lookahead == 108 { state = 161; lexer.advance(false); continue; }
                if lookahead == 114 { state = 162; lexer.advance(false); continue; }
                return result;
            }
            139 => {
                if lookahead == 110 { state = 163; lexer.advance(false); continue; }
                return result;
            }
            140 => {
                if lookahead == 114 { state = 164; lexer.advance(false); continue; }
                return result;
            }
            141 => {
                if lookahead == 101 { state = 165; lexer.advance(false); continue; }
                return result;
            }
            142 => {
                if lookahead == 112 { state = 166; lexer.advance(false); continue; }
                return result;
            }
            143 => {
                if lookahead == 97 { state = 167; lexer.advance(false); continue; }
                return result;
            }
            144 => {
                if lookahead == 114 { state = 168; lexer.advance(false); continue; }
                return result;
            }
            145 => {
                if lookahead == 111 { state = 169; lexer.advance(false); continue; }
                return result;
            }
            146 => {
                if lookahead == 102 { state = 170; lexer.advance(false); continue; }
                return result;
            }
            147 => {
                if lookahead == 115 { state = 171; lexer.advance(false); continue; }
                return result;
            }
            148 => {
                result = true; lexer.set_result_symbol(anon_sym_stock); lexer.mark_end();
                return result;
            }
            149 => {
                if lookahead == 101 { state = 119; lexer.advance(false); continue; }
                return result;
            }
            150 => {
                result = true; lexer.set_result_symbol(anon_sym_using); lexer.mark_end();
                return result;
            }
            151 => {
                result = true; lexer.set_result_symbol(anon_sym_where); lexer.mark_end();
                return result;
            }
            152 => {
                if lookahead == 115 { state = 172; lexer.advance(false); continue; }
                return result;
            }
            153 => {
                if lookahead == 116 { state = 173; lexer.advance(false); continue; }
                return result;
            }
            154 => {
                if lookahead == 110 { state = 174; lexer.advance(false); continue; }
                return result;
            }
            155 => {
                result = true; lexer.set_result_symbol(anon_sym_export); lexer.mark_end();
                return result;
            }
            156 => {
                result = true; lexer.set_result_symbol(anon_sym_family); lexer.mark_end();
                return result;
            }
            157 => {
                result = true; lexer.set_result_symbol(anon_sym_forall); lexer.mark_end();
                return result;
            }
            158 => {
                if lookahead == 110 { state = 175; lexer.advance(false); continue; }
                return result;
            }
            159 => {
                result = true; lexer.set_result_symbol(anon_sym_hiding); lexer.mark_end();
                return result;
            }
            160 => {
                result = true; lexer.set_result_symbol(anon_sym_import); lexer.mark_end();
                return result;
            }
            161 => {
                result = true; lexer.set_result_symbol(anon_sym_infixl); lexer.mark_end();
                return result;
            }
            162 => {
                result = true; lexer.set_result_symbol(anon_sym_infixr); lexer.mark_end();
                return result;
            }
            163 => {
                if lookahead == 99 { state = 176; lexer.advance(false); continue; }
                return result;
            }
            164 => {
                if lookahead == 117 { state = 177; lexer.advance(false); continue; }
                return result;
            }
            165 => {
                result = true; lexer.set_result_symbol(anon_sym_module); lexer.mark_end();
                return result;
            }
            166 => {
                if lookahead == 101 { state = 178; lexer.advance(false); continue; }
                return result;
            }
            167 => {
                if lookahead == 108 { state = 179; lexer.advance(false); continue; }
                return result;
            }
            168 => {
                if lookahead == 110 { state = 180; lexer.advance(false); continue; }
                return result;
            }
            169 => {
                if lookahead == 109 { state = 181; lexer.advance(false); continue; }
                return result;
            }
            170 => {
                if lookahead == 105 { state = 182; lexer.advance(false); continue; }
                return result;
            }
            171 => {
                if lookahead == 101 { state = 183; lexer.advance(false); continue; }
                return result;
            }
            172 => {
                if lookahead == 115 { state = 184; lexer.advance(false); continue; }
                return result;
            }
            173 => {
                result = true; lexer.set_result_symbol(anon_sym_default); lexer.mark_end();
                return result;
            }
            174 => {
                if lookahead == 103 { state = 185; lexer.advance(false); continue; }
                return result;
            }
            175 => {
                result = true; lexer.set_result_symbol(anon_sym_foreign); lexer.mark_end();
                return result;
            }
            176 => {
                if lookahead == 101 { state = 186; lexer.advance(false); continue; }
                return result;
            }
            177 => {
                if lookahead == 112 { state = 187; lexer.advance(false); continue; }
                return result;
            }
            178 => {
                result = true; lexer.set_result_symbol(anon_sym_newtype); lexer.mark_end();
                return result;
            }
            179 => {
                result = true; lexer.set_result_symbol(anon_sym_nominal); lexer.mark_end();
                return result;
            }
            180 => {
                result = true; lexer.set_result_symbol(anon_sym_pattern); lexer.mark_end();
                return result;
            }
            181 => {
                result = true; lexer.set_result_symbol(anon_sym_phantom); lexer.mark_end();
                return result;
            }
            182 => {
                if lookahead == 101 { state = 188; lexer.advance(false); continue; }
                return result;
            }
            183 => {
                if lookahead == 110 { state = 189; lexer.advance(false); continue; }
                return result;
            }
            184 => {
                result = true; lexer.set_result_symbol(anon_sym_anyclass); lexer.mark_end();
                return result;
            }
            185 => {
                result = true; lexer.set_result_symbol(anon_sym_deriving); lexer.mark_end();
                return result;
            }
            186 => {
                result = true; lexer.set_result_symbol(anon_sym_instance); lexer.mark_end();
                return result;
            }
            187 => {
                if lookahead == 116 { state = 190; lexer.advance(false); continue; }
                return result;
            }
            188 => {
                if lookahead == 100 { state = 191; lexer.advance(false); continue; }
                return result;
            }
            189 => {
                if lookahead == 116 { state = 192; lexer.advance(false); continue; }
                return result;
            }
            190 => {
                if lookahead == 105 { state = 193; lexer.advance(false); continue; }
                return result;
            }
            191 => {
                result = true; lexer.set_result_symbol(anon_sym_qualified); lexer.mark_end();
                return result;
            }
            192 => {
                if lookahead == 97 { state = 194; lexer.advance(false); continue; }
                return result;
            }
            193 => {
                if lookahead == 98 { state = 195; lexer.advance(false); continue; }
                return result;
            }
            194 => {
                if lookahead == 116 { state = 196; lexer.advance(false); continue; }
                return result;
            }
            195 => {
                if lookahead == 108 { state = 197; lexer.advance(false); continue; }
                return result;
            }
            196 => {
                if lookahead == 105 { state = 198; lexer.advance(false); continue; }
                return result;
            }
            197 => {
                if lookahead == 101 { state = 119; lexer.advance(false); continue; }
                return result;
            }
            198 => {
                if lookahead == 111 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            199 => {
                if lookahead == 110 { state = 200; lexer.advance(false); continue; }
                return result;
            }
            200 => {
                if lookahead == 97 { state = 201; lexer.advance(false); continue; }
                return result;
            }
            201 => {
                if lookahead == 108 { state = 202; lexer.advance(false); continue; }
                return result;
            }
            202 => {
                result = true; lexer.set_result_symbol(anon_sym_representational); lexer.mark_end();
                return result;
            }
            _ => return false,
        }
    }
}
