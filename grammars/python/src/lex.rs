//! The `python` grammar's lexer: `ts_lex` and `ts_lex_keywords`, transliterated from
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

const anon_sym_AMP: Symbol = 61;
const anon_sym_AMP_EQ: Symbol = 84;
const anon_sym_AT: Symbol = 48;
const anon_sym_AT_EQ: Symbol = 78;
const anon_sym_BANG_EQ: Symbol = 69;
const anon_sym_BSLASH: Symbol = 90;
const anon_sym_CARET: Symbol = 62;
const anon_sym_CARET_EQ: Symbol = 85;
const anon_sym_COLON: Symbol = 23;
const anon_sym_COLON_EQ: Symbol = 15;
const anon_sym_COMMA: Symbol = 9;
const anon_sym_DASH: Symbol = 49;
const anon_sym_DASH_EQ: Symbol = 75;
const anon_sym_DASH_GT: Symbol = 38;
const anon_sym_DOT: Symbol = 4;
const anon_sym_EQ: Symbol = 44;
const anon_sym_EQ_EQ: Symbol = 68;
const anon_sym_GT: Symbol = 71;
const anon_sym_GT_EQ: Symbol = 70;
const anon_sym_GT_GT: Symbol = 13;
const anon_sym_GT_GT_EQ: Symbol = 82;
const anon_sym_LBRACE: Symbol = 52;
const anon_sym_LBRACK: Symbol = 46;
const anon_sym_LPAREN: Symbol = 7;
const anon_sym_LT: Symbol = 66;
const anon_sym_LT_EQ: Symbol = 67;
const anon_sym_LT_GT: Symbol = 72;
const anon_sym_LT_LT: Symbol = 63;
const anon_sym_LT_LT_EQ: Symbol = 83;
const anon_sym_PERCENT: Symbol = 59;
const anon_sym_PERCENT_EQ: Symbol = 80;
const anon_sym_PIPE: Symbol = 51;
const anon_sym_PIPE_EQ: Symbol = 86;
const anon_sym_PLUS: Symbol = 54;
const anon_sym_PLUS_EQ: Symbol = 74;
const anon_sym_RBRACE: Symbol = 53;
const anon_sym_RBRACK: Symbol = 47;
const anon_sym_RPAREN: Symbol = 8;
const anon_sym_SEMI: Symbol = 2;
const anon_sym_SLASH: Symbol = 58;
const anon_sym_SLASH_EQ: Symbol = 77;
const anon_sym_SLASH_SLASH: Symbol = 60;
const anon_sym_SLASH_SLASH_EQ: Symbol = 79;
const anon_sym_STAR: Symbol = 11;
const anon_sym_STAR2: Symbol = 34;
const anon_sym_STAR_EQ: Symbol = 76;
const anon_sym_STAR_STAR: Symbol = 39;
const anon_sym_STAR_STAR_EQ: Symbol = 81;
const anon_sym_TILDE: Symbol = 64;
const anon_sym__: Symbol = 50;
const anon_sym___future__: Symbol = 6;
const anon_sym_and: Symbol = 56;
const anon_sym_as: Symbol = 10;
const anon_sym_assert: Symbol = 14;
const anon_sym_async: Symbol = 28;
const anon_sym_await: Symbol = 95;
const anon_sym_break: Symbol = 20;
const anon_sym_case: Symbol = 27;
const anon_sym_class: Symbol = 45;
const anon_sym_continue: Symbol = 21;
const anon_sym_def: Symbol = 37;
const anon_sym_del: Symbol = 17;
const anon_sym_elif: Symbol = 24;
const anon_sym_else: Symbol = 25;
const anon_sym_except: Symbol = 33;
const anon_sym_exec: Symbol = 42;
const anon_sym_finally: Symbol = 35;
const anon_sym_for: Symbol = 29;
const anon_sym_from: Symbol = 5;
const anon_sym_global: Symbol = 40;
const anon_sym_if: Symbol = 22;
const anon_sym_import: Symbol = 3;
const anon_sym_in: Symbol = 30;
const anon_sym_is: Symbol = 65;
const anon_sym_lambda: Symbol = 73;
const anon_sym_match: Symbol = 26;
const anon_sym_nonlocal: Symbol = 41;
const anon_sym_not: Symbol = 55;
const anon_sym_or: Symbol = 57;
const anon_sym_pass: Symbol = 19;
const anon_sym_print: Symbol = 12;
const anon_sym_raise: Symbol = 18;
const anon_sym_return: Symbol = 16;
const anon_sym_try: Symbol = 32;
const anon_sym_type: Symbol = 43;
const anon_sym_while: Symbol = 31;
const anon_sym_with: Symbol = 36;
const anon_sym_yield: Symbol = 87;
const aux_sym_format_specifier_token1: Symbol = 91;
const sym_comment: Symbol = 99;
const sym_ellipsis: Symbol = 88;
const sym_escape_sequence: Symbol = 89;
const sym_false: Symbol = 97;
const sym_float: Symbol = 94;
const sym_identifier: Symbol = 1;
const sym_integer: Symbol = 93;
const sym_line_continuation: Symbol = 100;
const sym_none: Symbol = 98;
const sym_true: Symbol = 96;
const sym_type_conversion: Symbol = 92;
const ts_builtin_sym_end: Symbol = 0;

#[rustfmt::skip]
static sym_identifier_character_set_1: [CharacterRange; 685] = [
    CharacterRange::new(65, 90), CharacterRange::new(95, 95), CharacterRange::new(97, 122), CharacterRange::new(170, 170), CharacterRange::new(181, 181), CharacterRange::new(186, 186),
    CharacterRange::new(192, 214), CharacterRange::new(216, 246), CharacterRange::new(248, 705), CharacterRange::new(710, 721), CharacterRange::new(736, 740), CharacterRange::new(748, 748),
    CharacterRange::new(750, 750), CharacterRange::new(880, 884), CharacterRange::new(886, 887), CharacterRange::new(891, 893), CharacterRange::new(895, 895), CharacterRange::new(902, 902),
    CharacterRange::new(904, 906), CharacterRange::new(908, 908), CharacterRange::new(910, 929), CharacterRange::new(931, 1013), CharacterRange::new(1015, 1153), CharacterRange::new(1162, 1327),
    CharacterRange::new(1329, 1366), CharacterRange::new(1369, 1369), CharacterRange::new(1376, 1416), CharacterRange::new(1488, 1514), CharacterRange::new(1519, 1522), CharacterRange::new(1568, 1610),
    CharacterRange::new(1646, 1647), CharacterRange::new(1649, 1747), CharacterRange::new(1749, 1749), CharacterRange::new(1765, 1766), CharacterRange::new(1774, 1775), CharacterRange::new(1786, 1788),
    CharacterRange::new(1791, 1791), CharacterRange::new(1808, 1808), CharacterRange::new(1810, 1839), CharacterRange::new(1869, 1957), CharacterRange::new(1969, 1969), CharacterRange::new(1994, 2026),
    CharacterRange::new(2036, 2037), CharacterRange::new(2042, 2042), CharacterRange::new(2048, 2069), CharacterRange::new(2074, 2074), CharacterRange::new(2084, 2084), CharacterRange::new(2088, 2088),
    CharacterRange::new(2112, 2136), CharacterRange::new(2144, 2154), CharacterRange::new(2160, 2183), CharacterRange::new(2185, 2190), CharacterRange::new(2208, 2249), CharacterRange::new(2308, 2361),
    CharacterRange::new(2365, 2365), CharacterRange::new(2384, 2384), CharacterRange::new(2392, 2401), CharacterRange::new(2417, 2432), CharacterRange::new(2437, 2444), CharacterRange::new(2447, 2448),
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
    CharacterRange::new(3482, 3505), CharacterRange::new(3507, 3515), CharacterRange::new(3517, 3517), CharacterRange::new(3520, 3526), CharacterRange::new(3585, 3632), CharacterRange::new(3634, 3634),
    CharacterRange::new(3648, 3654), CharacterRange::new(3713, 3714), CharacterRange::new(3716, 3716), CharacterRange::new(3718, 3722), CharacterRange::new(3724, 3747), CharacterRange::new(3749, 3749),
    CharacterRange::new(3751, 3760), CharacterRange::new(3762, 3762), CharacterRange::new(3773, 3773), CharacterRange::new(3776, 3780), CharacterRange::new(3782, 3782), CharacterRange::new(3804, 3807),
    CharacterRange::new(3840, 3840), CharacterRange::new(3904, 3911), CharacterRange::new(3913, 3948), CharacterRange::new(3976, 3980), CharacterRange::new(4096, 4138), CharacterRange::new(4159, 4159),
    CharacterRange::new(4176, 4181), CharacterRange::new(4186, 4189), CharacterRange::new(4193, 4193), CharacterRange::new(4197, 4198), CharacterRange::new(4206, 4208), CharacterRange::new(4213, 4225),
    CharacterRange::new(4238, 4238), CharacterRange::new(4256, 4293), CharacterRange::new(4295, 4295), CharacterRange::new(4301, 4301), CharacterRange::new(4304, 4346), CharacterRange::new(4348, 4680),
    CharacterRange::new(4682, 4685), CharacterRange::new(4688, 4694), CharacterRange::new(4696, 4696), CharacterRange::new(4698, 4701), CharacterRange::new(4704, 4744), CharacterRange::new(4746, 4749),
    CharacterRange::new(4752, 4784), CharacterRange::new(4786, 4789), CharacterRange::new(4792, 4798), CharacterRange::new(4800, 4800), CharacterRange::new(4802, 4805), CharacterRange::new(4808, 4822),
    CharacterRange::new(4824, 4880), CharacterRange::new(4882, 4885), CharacterRange::new(4888, 4954), CharacterRange::new(4992, 5007), CharacterRange::new(5024, 5109), CharacterRange::new(5112, 5117),
    CharacterRange::new(5121, 5740), CharacterRange::new(5743, 5759), CharacterRange::new(5761, 5786), CharacterRange::new(5792, 5866), CharacterRange::new(5870, 5880), CharacterRange::new(5888, 5905),
    CharacterRange::new(5919, 5937), CharacterRange::new(5952, 5969), CharacterRange::new(5984, 5996), CharacterRange::new(5998, 6000), CharacterRange::new(6016, 6067), CharacterRange::new(6103, 6103),
    CharacterRange::new(6108, 6108), CharacterRange::new(6176, 6264), CharacterRange::new(6272, 6312), CharacterRange::new(6314, 6314), CharacterRange::new(6320, 6389), CharacterRange::new(6400, 6430),
    CharacterRange::new(6480, 6509), CharacterRange::new(6512, 6516), CharacterRange::new(6528, 6571), CharacterRange::new(6576, 6601), CharacterRange::new(6656, 6678), CharacterRange::new(6688, 6740),
    CharacterRange::new(6823, 6823), CharacterRange::new(6917, 6963), CharacterRange::new(6981, 6988), CharacterRange::new(7043, 7072), CharacterRange::new(7086, 7087), CharacterRange::new(7098, 7141),
    CharacterRange::new(7168, 7203), CharacterRange::new(7245, 7247), CharacterRange::new(7258, 7293), CharacterRange::new(7296, 7306), CharacterRange::new(7312, 7354), CharacterRange::new(7357, 7359),
    CharacterRange::new(7401, 7404), CharacterRange::new(7406, 7411), CharacterRange::new(7413, 7414), CharacterRange::new(7418, 7418), CharacterRange::new(7424, 7615), CharacterRange::new(7680, 7957),
    CharacterRange::new(7960, 7965), CharacterRange::new(7968, 8005), CharacterRange::new(8008, 8013), CharacterRange::new(8016, 8023), CharacterRange::new(8025, 8025), CharacterRange::new(8027, 8027),
    CharacterRange::new(8029, 8029), CharacterRange::new(8031, 8061), CharacterRange::new(8064, 8116), CharacterRange::new(8118, 8124), CharacterRange::new(8126, 8126), CharacterRange::new(8130, 8132),
    CharacterRange::new(8134, 8140), CharacterRange::new(8144, 8147), CharacterRange::new(8150, 8155), CharacterRange::new(8160, 8172), CharacterRange::new(8178, 8180), CharacterRange::new(8182, 8188),
    CharacterRange::new(8305, 8305), CharacterRange::new(8319, 8319), CharacterRange::new(8336, 8348), CharacterRange::new(8450, 8450), CharacterRange::new(8455, 8455), CharacterRange::new(8458, 8467),
    CharacterRange::new(8469, 8469), CharacterRange::new(8472, 8477), CharacterRange::new(8484, 8484), CharacterRange::new(8486, 8486), CharacterRange::new(8488, 8488), CharacterRange::new(8490, 8505),
    CharacterRange::new(8508, 8511), CharacterRange::new(8517, 8521), CharacterRange::new(8526, 8526), CharacterRange::new(8544, 8584), CharacterRange::new(11264, 11492), CharacterRange::new(11499, 11502),
    CharacterRange::new(11506, 11507), CharacterRange::new(11520, 11557), CharacterRange::new(11559, 11559), CharacterRange::new(11565, 11565), CharacterRange::new(11568, 11623), CharacterRange::new(11631, 11631),
    CharacterRange::new(11648, 11670), CharacterRange::new(11680, 11686), CharacterRange::new(11688, 11694), CharacterRange::new(11696, 11702), CharacterRange::new(11704, 11710), CharacterRange::new(11712, 11718),
    CharacterRange::new(11720, 11726), CharacterRange::new(11728, 11734), CharacterRange::new(11736, 11742), CharacterRange::new(12293, 12295), CharacterRange::new(12321, 12329), CharacterRange::new(12337, 12341),
    CharacterRange::new(12344, 12348), CharacterRange::new(12353, 12438), CharacterRange::new(12445, 12447), CharacterRange::new(12449, 12538), CharacterRange::new(12540, 12543), CharacterRange::new(12549, 12591),
    CharacterRange::new(12593, 12686), CharacterRange::new(12704, 12735), CharacterRange::new(12784, 12799), CharacterRange::new(13312, 19903), CharacterRange::new(19968, 42124), CharacterRange::new(42192, 42237),
    CharacterRange::new(42240, 42508), CharacterRange::new(42512, 42527), CharacterRange::new(42538, 42539), CharacterRange::new(42560, 42606), CharacterRange::new(42623, 42653), CharacterRange::new(42656, 42735),
    CharacterRange::new(42775, 42783), CharacterRange::new(42786, 42888), CharacterRange::new(42891, 42957), CharacterRange::new(42960, 42961), CharacterRange::new(42963, 42963), CharacterRange::new(42965, 42972),
    CharacterRange::new(42994, 43009), CharacterRange::new(43011, 43013), CharacterRange::new(43015, 43018), CharacterRange::new(43020, 43042), CharacterRange::new(43072, 43123), CharacterRange::new(43138, 43187),
    CharacterRange::new(43250, 43255), CharacterRange::new(43259, 43259), CharacterRange::new(43261, 43262), CharacterRange::new(43274, 43301), CharacterRange::new(43312, 43334), CharacterRange::new(43360, 43388),
    CharacterRange::new(43396, 43442), CharacterRange::new(43471, 43471), CharacterRange::new(43488, 43492), CharacterRange::new(43494, 43503), CharacterRange::new(43514, 43518), CharacterRange::new(43520, 43560),
    CharacterRange::new(43584, 43586), CharacterRange::new(43588, 43595), CharacterRange::new(43616, 43638), CharacterRange::new(43642, 43642), CharacterRange::new(43646, 43695), CharacterRange::new(43697, 43697),
    CharacterRange::new(43701, 43702), CharacterRange::new(43705, 43709), CharacterRange::new(43712, 43712), CharacterRange::new(43714, 43714), CharacterRange::new(43739, 43741), CharacterRange::new(43744, 43754),
    CharacterRange::new(43762, 43764), CharacterRange::new(43777, 43782), CharacterRange::new(43785, 43790), CharacterRange::new(43793, 43798), CharacterRange::new(43808, 43814), CharacterRange::new(43816, 43822),
    CharacterRange::new(43824, 43866), CharacterRange::new(43868, 43881), CharacterRange::new(43888, 44002), CharacterRange::new(44032, 55203), CharacterRange::new(55216, 55238), CharacterRange::new(55243, 55291),
    CharacterRange::new(63744, 64109), CharacterRange::new(64112, 64217), CharacterRange::new(64256, 64262), CharacterRange::new(64275, 64279), CharacterRange::new(64285, 64285), CharacterRange::new(64287, 64296),
    CharacterRange::new(64298, 64310), CharacterRange::new(64312, 64316), CharacterRange::new(64318, 64318), CharacterRange::new(64320, 64321), CharacterRange::new(64323, 64324), CharacterRange::new(64326, 64433),
    CharacterRange::new(64467, 64605), CharacterRange::new(64612, 64829), CharacterRange::new(64848, 64911), CharacterRange::new(64914, 64967), CharacterRange::new(65008, 65017), CharacterRange::new(65137, 65137),
    CharacterRange::new(65139, 65139), CharacterRange::new(65143, 65143), CharacterRange::new(65145, 65145), CharacterRange::new(65147, 65147), CharacterRange::new(65149, 65149), CharacterRange::new(65151, 65276),
    CharacterRange::new(65313, 65338), CharacterRange::new(65345, 65370), CharacterRange::new(65382, 65437), CharacterRange::new(65440, 65470), CharacterRange::new(65474, 65479), CharacterRange::new(65482, 65487),
    CharacterRange::new(65490, 65495), CharacterRange::new(65498, 65500), CharacterRange::new(65536, 65547), CharacterRange::new(65549, 65574), CharacterRange::new(65576, 65594), CharacterRange::new(65596, 65597),
    CharacterRange::new(65599, 65613), CharacterRange::new(65616, 65629), CharacterRange::new(65664, 65786), CharacterRange::new(65856, 65908), CharacterRange::new(66176, 66204), CharacterRange::new(66208, 66256),
    CharacterRange::new(66304, 66335), CharacterRange::new(66349, 66378), CharacterRange::new(66384, 66421), CharacterRange::new(66432, 66461), CharacterRange::new(66464, 66499), CharacterRange::new(66504, 66511),
    CharacterRange::new(66513, 66517), CharacterRange::new(66560, 66717), CharacterRange::new(66736, 66771), CharacterRange::new(66776, 66811), CharacterRange::new(66816, 66855), CharacterRange::new(66864, 66915),
    CharacterRange::new(66928, 66938), CharacterRange::new(66940, 66954), CharacterRange::new(66956, 66962), CharacterRange::new(66964, 66965), CharacterRange::new(66967, 66977), CharacterRange::new(66979, 66993),
    CharacterRange::new(66995, 67001), CharacterRange::new(67003, 67004), CharacterRange::new(67008, 67059), CharacterRange::new(67072, 67382), CharacterRange::new(67392, 67413), CharacterRange::new(67424, 67431),
    CharacterRange::new(67456, 67461), CharacterRange::new(67463, 67504), CharacterRange::new(67506, 67514), CharacterRange::new(67584, 67589), CharacterRange::new(67592, 67592), CharacterRange::new(67594, 67637),
    CharacterRange::new(67639, 67640), CharacterRange::new(67644, 67644), CharacterRange::new(67647, 67669), CharacterRange::new(67680, 67702), CharacterRange::new(67712, 67742), CharacterRange::new(67808, 67826),
    CharacterRange::new(67828, 67829), CharacterRange::new(67840, 67861), CharacterRange::new(67872, 67897), CharacterRange::new(67968, 68023), CharacterRange::new(68030, 68031), CharacterRange::new(68096, 68096),
    CharacterRange::new(68112, 68115), CharacterRange::new(68117, 68119), CharacterRange::new(68121, 68149), CharacterRange::new(68192, 68220), CharacterRange::new(68224, 68252), CharacterRange::new(68288, 68295),
    CharacterRange::new(68297, 68324), CharacterRange::new(68352, 68405), CharacterRange::new(68416, 68437), CharacterRange::new(68448, 68466), CharacterRange::new(68480, 68497), CharacterRange::new(68608, 68680),
    CharacterRange::new(68736, 68786), CharacterRange::new(68800, 68850), CharacterRange::new(68864, 68899), CharacterRange::new(68938, 68965), CharacterRange::new(68975, 68997), CharacterRange::new(69248, 69289),
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
    CharacterRange::new(71488, 71494), CharacterRange::new(71680, 71723), CharacterRange::new(71840, 71903), CharacterRange::new(71935, 71942), CharacterRange::new(71945, 71945), CharacterRange::new(71948, 71955),
    CharacterRange::new(71957, 71958), CharacterRange::new(71960, 71983), CharacterRange::new(71999, 71999), CharacterRange::new(72001, 72001), CharacterRange::new(72096, 72103), CharacterRange::new(72106, 72144),
    CharacterRange::new(72161, 72161), CharacterRange::new(72163, 72163), CharacterRange::new(72192, 72192), CharacterRange::new(72203, 72242), CharacterRange::new(72250, 72250), CharacterRange::new(72272, 72272),
    CharacterRange::new(72284, 72329), CharacterRange::new(72349, 72349), CharacterRange::new(72368, 72440), CharacterRange::new(72640, 72672), CharacterRange::new(72704, 72712), CharacterRange::new(72714, 72750),
    CharacterRange::new(72768, 72768), CharacterRange::new(72818, 72847), CharacterRange::new(72960, 72966), CharacterRange::new(72968, 72969), CharacterRange::new(72971, 73008), CharacterRange::new(73030, 73030),
    CharacterRange::new(73056, 73061), CharacterRange::new(73063, 73064), CharacterRange::new(73066, 73097), CharacterRange::new(73112, 73112), CharacterRange::new(73440, 73458), CharacterRange::new(73474, 73474),
    CharacterRange::new(73476, 73488), CharacterRange::new(73490, 73523), CharacterRange::new(73648, 73648), CharacterRange::new(73728, 74649), CharacterRange::new(74752, 74862), CharacterRange::new(74880, 75075),
    CharacterRange::new(77712, 77808), CharacterRange::new(77824, 78895), CharacterRange::new(78913, 78918), CharacterRange::new(78944, 82938), CharacterRange::new(82944, 83526), CharacterRange::new(90368, 90397),
    CharacterRange::new(92160, 92728), CharacterRange::new(92736, 92766), CharacterRange::new(92784, 92862), CharacterRange::new(92880, 92909), CharacterRange::new(92928, 92975), CharacterRange::new(92992, 92995),
    CharacterRange::new(93027, 93047), CharacterRange::new(93053, 93071), CharacterRange::new(93504, 93548), CharacterRange::new(93760, 93823), CharacterRange::new(93952, 94026), CharacterRange::new(94032, 94032),
    CharacterRange::new(94099, 94111), CharacterRange::new(94176, 94177), CharacterRange::new(94179, 94179), CharacterRange::new(94208, 100343), CharacterRange::new(100352, 101589), CharacterRange::new(101631, 101640),
    CharacterRange::new(110576, 110579), CharacterRange::new(110581, 110587), CharacterRange::new(110589, 110590), CharacterRange::new(110592, 110882), CharacterRange::new(110898, 110898), CharacterRange::new(110928, 110930),
    CharacterRange::new(110933, 110933), CharacterRange::new(110948, 110951), CharacterRange::new(110960, 111355), CharacterRange::new(113664, 113770), CharacterRange::new(113776, 113788), CharacterRange::new(113792, 113800),
    CharacterRange::new(113808, 113817), CharacterRange::new(119808, 119892), CharacterRange::new(119894, 119964), CharacterRange::new(119966, 119967), CharacterRange::new(119970, 119970), CharacterRange::new(119973, 119974),
    CharacterRange::new(119977, 119980), CharacterRange::new(119982, 119993), CharacterRange::new(119995, 119995), CharacterRange::new(119997, 120003), CharacterRange::new(120005, 120069), CharacterRange::new(120071, 120074),
    CharacterRange::new(120077, 120084), CharacterRange::new(120086, 120092), CharacterRange::new(120094, 120121), CharacterRange::new(120123, 120126), CharacterRange::new(120128, 120132), CharacterRange::new(120134, 120134),
    CharacterRange::new(120138, 120144), CharacterRange::new(120146, 120485), CharacterRange::new(120488, 120512), CharacterRange::new(120514, 120538), CharacterRange::new(120540, 120570), CharacterRange::new(120572, 120596),
    CharacterRange::new(120598, 120628), CharacterRange::new(120630, 120654), CharacterRange::new(120656, 120686), CharacterRange::new(120688, 120712), CharacterRange::new(120714, 120744), CharacterRange::new(120746, 120770),
    CharacterRange::new(120772, 120779), CharacterRange::new(122624, 122654), CharacterRange::new(122661, 122666), CharacterRange::new(122928, 122989), CharacterRange::new(123136, 123180), CharacterRange::new(123191, 123197),
    CharacterRange::new(123214, 123214), CharacterRange::new(123536, 123565), CharacterRange::new(123584, 123627), CharacterRange::new(124112, 124139), CharacterRange::new(124368, 124397), CharacterRange::new(124400, 124400),
    CharacterRange::new(124896, 124902), CharacterRange::new(124904, 124907), CharacterRange::new(124909, 124910), CharacterRange::new(124912, 124926), CharacterRange::new(124928, 125124), CharacterRange::new(125184, 125251),
    CharacterRange::new(125259, 125259), CharacterRange::new(126464, 126467), CharacterRange::new(126469, 126495), CharacterRange::new(126497, 126498), CharacterRange::new(126500, 126500), CharacterRange::new(126503, 126503),
    CharacterRange::new(126505, 126514), CharacterRange::new(126516, 126519), CharacterRange::new(126521, 126521), CharacterRange::new(126523, 126523), CharacterRange::new(126530, 126530), CharacterRange::new(126535, 126535),
    CharacterRange::new(126537, 126537), CharacterRange::new(126539, 126539), CharacterRange::new(126541, 126543), CharacterRange::new(126545, 126546), CharacterRange::new(126548, 126548), CharacterRange::new(126551, 126551),
    CharacterRange::new(126553, 126553), CharacterRange::new(126555, 126555), CharacterRange::new(126557, 126557), CharacterRange::new(126559, 126559), CharacterRange::new(126561, 126562), CharacterRange::new(126564, 126564),
    CharacterRange::new(126567, 126570), CharacterRange::new(126572, 126578), CharacterRange::new(126580, 126583), CharacterRange::new(126585, 126588), CharacterRange::new(126590, 126590), CharacterRange::new(126592, 126601),
    CharacterRange::new(126603, 126619), CharacterRange::new(126625, 126627), CharacterRange::new(126629, 126633), CharacterRange::new(126635, 126651), CharacterRange::new(131072, 173791), CharacterRange::new(173824, 177977),
    CharacterRange::new(177984, 178205), CharacterRange::new(178208, 183969), CharacterRange::new(183984, 191456), CharacterRange::new(191472, 192093), CharacterRange::new(194560, 195101), CharacterRange::new(196608, 201546),
    CharacterRange::new(201552, 205743),
];

#[rustfmt::skip]
static sym_identifier_character_set_2: [CharacterRange; 800] = [
    CharacterRange::new(48, 57), CharacterRange::new(65, 90), CharacterRange::new(95, 95), CharacterRange::new(97, 122), CharacterRange::new(170, 170), CharacterRange::new(181, 181),
    CharacterRange::new(183, 183), CharacterRange::new(186, 186), CharacterRange::new(192, 214), CharacterRange::new(216, 246), CharacterRange::new(248, 705), CharacterRange::new(710, 721),
    CharacterRange::new(736, 740), CharacterRange::new(748, 748), CharacterRange::new(750, 750), CharacterRange::new(768, 884), CharacterRange::new(886, 887), CharacterRange::new(891, 893),
    CharacterRange::new(895, 895), CharacterRange::new(902, 906), CharacterRange::new(908, 908), CharacterRange::new(910, 929), CharacterRange::new(931, 1013), CharacterRange::new(1015, 1153),
    CharacterRange::new(1155, 1159), CharacterRange::new(1162, 1327), CharacterRange::new(1329, 1366), CharacterRange::new(1369, 1369), CharacterRange::new(1376, 1416), CharacterRange::new(1425, 1469),
    CharacterRange::new(1471, 1471), CharacterRange::new(1473, 1474), CharacterRange::new(1476, 1477), CharacterRange::new(1479, 1479), CharacterRange::new(1488, 1514), CharacterRange::new(1519, 1522),
    CharacterRange::new(1552, 1562), CharacterRange::new(1568, 1641), CharacterRange::new(1646, 1747), CharacterRange::new(1749, 1756), CharacterRange::new(1759, 1768), CharacterRange::new(1770, 1788),
    CharacterRange::new(1791, 1791), CharacterRange::new(1808, 1866), CharacterRange::new(1869, 1969), CharacterRange::new(1984, 2037), CharacterRange::new(2042, 2042), CharacterRange::new(2045, 2045),
    CharacterRange::new(2048, 2093), CharacterRange::new(2112, 2139), CharacterRange::new(2144, 2154), CharacterRange::new(2160, 2183), CharacterRange::new(2185, 2190), CharacterRange::new(2199, 2273),
    CharacterRange::new(2275, 2403), CharacterRange::new(2406, 2415), CharacterRange::new(2417, 2435), CharacterRange::new(2437, 2444), CharacterRange::new(2447, 2448), CharacterRange::new(2451, 2472),
    CharacterRange::new(2474, 2480), CharacterRange::new(2482, 2482), CharacterRange::new(2486, 2489), CharacterRange::new(2492, 2500), CharacterRange::new(2503, 2504), CharacterRange::new(2507, 2510),
    CharacterRange::new(2519, 2519), CharacterRange::new(2524, 2525), CharacterRange::new(2527, 2531), CharacterRange::new(2534, 2545), CharacterRange::new(2556, 2556), CharacterRange::new(2558, 2558),
    CharacterRange::new(2561, 2563), CharacterRange::new(2565, 2570), CharacterRange::new(2575, 2576), CharacterRange::new(2579, 2600), CharacterRange::new(2602, 2608), CharacterRange::new(2610, 2611),
    CharacterRange::new(2613, 2614), CharacterRange::new(2616, 2617), CharacterRange::new(2620, 2620), CharacterRange::new(2622, 2626), CharacterRange::new(2631, 2632), CharacterRange::new(2635, 2637),
    CharacterRange::new(2641, 2641), CharacterRange::new(2649, 2652), CharacterRange::new(2654, 2654), CharacterRange::new(2662, 2677), CharacterRange::new(2689, 2691), CharacterRange::new(2693, 2701),
    CharacterRange::new(2703, 2705), CharacterRange::new(2707, 2728), CharacterRange::new(2730, 2736), CharacterRange::new(2738, 2739), CharacterRange::new(2741, 2745), CharacterRange::new(2748, 2757),
    CharacterRange::new(2759, 2761), CharacterRange::new(2763, 2765), CharacterRange::new(2768, 2768), CharacterRange::new(2784, 2787), CharacterRange::new(2790, 2799), CharacterRange::new(2809, 2815),
    CharacterRange::new(2817, 2819), CharacterRange::new(2821, 2828), CharacterRange::new(2831, 2832), CharacterRange::new(2835, 2856), CharacterRange::new(2858, 2864), CharacterRange::new(2866, 2867),
    CharacterRange::new(2869, 2873), CharacterRange::new(2876, 2884), CharacterRange::new(2887, 2888), CharacterRange::new(2891, 2893), CharacterRange::new(2901, 2903), CharacterRange::new(2908, 2909),
    CharacterRange::new(2911, 2915), CharacterRange::new(2918, 2927), CharacterRange::new(2929, 2929), CharacterRange::new(2946, 2947), CharacterRange::new(2949, 2954), CharacterRange::new(2958, 2960),
    CharacterRange::new(2962, 2965), CharacterRange::new(2969, 2970), CharacterRange::new(2972, 2972), CharacterRange::new(2974, 2975), CharacterRange::new(2979, 2980), CharacterRange::new(2984, 2986),
    CharacterRange::new(2990, 3001), CharacterRange::new(3006, 3010), CharacterRange::new(3014, 3016), CharacterRange::new(3018, 3021), CharacterRange::new(3024, 3024), CharacterRange::new(3031, 3031),
    CharacterRange::new(3046, 3055), CharacterRange::new(3072, 3084), CharacterRange::new(3086, 3088), CharacterRange::new(3090, 3112), CharacterRange::new(3114, 3129), CharacterRange::new(3132, 3140),
    CharacterRange::new(3142, 3144), CharacterRange::new(3146, 3149), CharacterRange::new(3157, 3158), CharacterRange::new(3160, 3162), CharacterRange::new(3165, 3165), CharacterRange::new(3168, 3171),
    CharacterRange::new(3174, 3183), CharacterRange::new(3200, 3203), CharacterRange::new(3205, 3212), CharacterRange::new(3214, 3216), CharacterRange::new(3218, 3240), CharacterRange::new(3242, 3251),
    CharacterRange::new(3253, 3257), CharacterRange::new(3260, 3268), CharacterRange::new(3270, 3272), CharacterRange::new(3274, 3277), CharacterRange::new(3285, 3286), CharacterRange::new(3293, 3294),
    CharacterRange::new(3296, 3299), CharacterRange::new(3302, 3311), CharacterRange::new(3313, 3315), CharacterRange::new(3328, 3340), CharacterRange::new(3342, 3344), CharacterRange::new(3346, 3396),
    CharacterRange::new(3398, 3400), CharacterRange::new(3402, 3406), CharacterRange::new(3412, 3415), CharacterRange::new(3423, 3427), CharacterRange::new(3430, 3439), CharacterRange::new(3450, 3455),
    CharacterRange::new(3457, 3459), CharacterRange::new(3461, 3478), CharacterRange::new(3482, 3505), CharacterRange::new(3507, 3515), CharacterRange::new(3517, 3517), CharacterRange::new(3520, 3526),
    CharacterRange::new(3530, 3530), CharacterRange::new(3535, 3540), CharacterRange::new(3542, 3542), CharacterRange::new(3544, 3551), CharacterRange::new(3558, 3567), CharacterRange::new(3570, 3571),
    CharacterRange::new(3585, 3642), CharacterRange::new(3648, 3662), CharacterRange::new(3664, 3673), CharacterRange::new(3713, 3714), CharacterRange::new(3716, 3716), CharacterRange::new(3718, 3722),
    CharacterRange::new(3724, 3747), CharacterRange::new(3749, 3749), CharacterRange::new(3751, 3773), CharacterRange::new(3776, 3780), CharacterRange::new(3782, 3782), CharacterRange::new(3784, 3790),
    CharacterRange::new(3792, 3801), CharacterRange::new(3804, 3807), CharacterRange::new(3840, 3840), CharacterRange::new(3864, 3865), CharacterRange::new(3872, 3881), CharacterRange::new(3893, 3893),
    CharacterRange::new(3895, 3895), CharacterRange::new(3897, 3897), CharacterRange::new(3902, 3911), CharacterRange::new(3913, 3948), CharacterRange::new(3953, 3972), CharacterRange::new(3974, 3991),
    CharacterRange::new(3993, 4028), CharacterRange::new(4038, 4038), CharacterRange::new(4096, 4169), CharacterRange::new(4176, 4253), CharacterRange::new(4256, 4293), CharacterRange::new(4295, 4295),
    CharacterRange::new(4301, 4301), CharacterRange::new(4304, 4346), CharacterRange::new(4348, 4680), CharacterRange::new(4682, 4685), CharacterRange::new(4688, 4694), CharacterRange::new(4696, 4696),
    CharacterRange::new(4698, 4701), CharacterRange::new(4704, 4744), CharacterRange::new(4746, 4749), CharacterRange::new(4752, 4784), CharacterRange::new(4786, 4789), CharacterRange::new(4792, 4798),
    CharacterRange::new(4800, 4800), CharacterRange::new(4802, 4805), CharacterRange::new(4808, 4822), CharacterRange::new(4824, 4880), CharacterRange::new(4882, 4885), CharacterRange::new(4888, 4954),
    CharacterRange::new(4957, 4959), CharacterRange::new(4969, 4977), CharacterRange::new(4992, 5007), CharacterRange::new(5024, 5109), CharacterRange::new(5112, 5117), CharacterRange::new(5121, 5740),
    CharacterRange::new(5743, 5759), CharacterRange::new(5761, 5786), CharacterRange::new(5792, 5866), CharacterRange::new(5870, 5880), CharacterRange::new(5888, 5909), CharacterRange::new(5919, 5940),
    CharacterRange::new(5952, 5971), CharacterRange::new(5984, 5996), CharacterRange::new(5998, 6000), CharacterRange::new(6002, 6003), CharacterRange::new(6016, 6099), CharacterRange::new(6103, 6103),
    CharacterRange::new(6108, 6109), CharacterRange::new(6112, 6121), CharacterRange::new(6155, 6157), CharacterRange::new(6159, 6169), CharacterRange::new(6176, 6264), CharacterRange::new(6272, 6314),
    CharacterRange::new(6320, 6389), CharacterRange::new(6400, 6430), CharacterRange::new(6432, 6443), CharacterRange::new(6448, 6459), CharacterRange::new(6470, 6509), CharacterRange::new(6512, 6516),
    CharacterRange::new(6528, 6571), CharacterRange::new(6576, 6601), CharacterRange::new(6608, 6618), CharacterRange::new(6656, 6683), CharacterRange::new(6688, 6750), CharacterRange::new(6752, 6780),
    CharacterRange::new(6783, 6793), CharacterRange::new(6800, 6809), CharacterRange::new(6823, 6823), CharacterRange::new(6832, 6845), CharacterRange::new(6847, 6862), CharacterRange::new(6912, 6988),
    CharacterRange::new(6992, 7001), CharacterRange::new(7019, 7027), CharacterRange::new(7040, 7155), CharacterRange::new(7168, 7223), CharacterRange::new(7232, 7241), CharacterRange::new(7245, 7293),
    CharacterRange::new(7296, 7306), CharacterRange::new(7312, 7354), CharacterRange::new(7357, 7359), CharacterRange::new(7376, 7378), CharacterRange::new(7380, 7418), CharacterRange::new(7424, 7957),
    CharacterRange::new(7960, 7965), CharacterRange::new(7968, 8005), CharacterRange::new(8008, 8013), CharacterRange::new(8016, 8023), CharacterRange::new(8025, 8025), CharacterRange::new(8027, 8027),
    CharacterRange::new(8029, 8029), CharacterRange::new(8031, 8061), CharacterRange::new(8064, 8116), CharacterRange::new(8118, 8124), CharacterRange::new(8126, 8126), CharacterRange::new(8130, 8132),
    CharacterRange::new(8134, 8140), CharacterRange::new(8144, 8147), CharacterRange::new(8150, 8155), CharacterRange::new(8160, 8172), CharacterRange::new(8178, 8180), CharacterRange::new(8182, 8188),
    CharacterRange::new(8204, 8205), CharacterRange::new(8255, 8256), CharacterRange::new(8276, 8276), CharacterRange::new(8305, 8305), CharacterRange::new(8319, 8319), CharacterRange::new(8336, 8348),
    CharacterRange::new(8400, 8412), CharacterRange::new(8417, 8417), CharacterRange::new(8421, 8432), CharacterRange::new(8450, 8450), CharacterRange::new(8455, 8455), CharacterRange::new(8458, 8467),
    CharacterRange::new(8469, 8469), CharacterRange::new(8472, 8477), CharacterRange::new(8484, 8484), CharacterRange::new(8486, 8486), CharacterRange::new(8488, 8488), CharacterRange::new(8490, 8505),
    CharacterRange::new(8508, 8511), CharacterRange::new(8517, 8521), CharacterRange::new(8526, 8526), CharacterRange::new(8544, 8584), CharacterRange::new(11264, 11492), CharacterRange::new(11499, 11507),
    CharacterRange::new(11520, 11557), CharacterRange::new(11559, 11559), CharacterRange::new(11565, 11565), CharacterRange::new(11568, 11623), CharacterRange::new(11631, 11631), CharacterRange::new(11647, 11670),
    CharacterRange::new(11680, 11686), CharacterRange::new(11688, 11694), CharacterRange::new(11696, 11702), CharacterRange::new(11704, 11710), CharacterRange::new(11712, 11718), CharacterRange::new(11720, 11726),
    CharacterRange::new(11728, 11734), CharacterRange::new(11736, 11742), CharacterRange::new(11744, 11775), CharacterRange::new(12293, 12295), CharacterRange::new(12321, 12335), CharacterRange::new(12337, 12341),
    CharacterRange::new(12344, 12348), CharacterRange::new(12353, 12438), CharacterRange::new(12441, 12442), CharacterRange::new(12445, 12447), CharacterRange::new(12449, 12543), CharacterRange::new(12549, 12591),
    CharacterRange::new(12593, 12686), CharacterRange::new(12704, 12735), CharacterRange::new(12784, 12799), CharacterRange::new(13312, 19903), CharacterRange::new(19968, 42124), CharacterRange::new(42192, 42237),
    CharacterRange::new(42240, 42508), CharacterRange::new(42512, 42539), CharacterRange::new(42560, 42607), CharacterRange::new(42612, 42621), CharacterRange::new(42623, 42737), CharacterRange::new(42775, 42783),
    CharacterRange::new(42786, 42888), CharacterRange::new(42891, 42957), CharacterRange::new(42960, 42961), CharacterRange::new(42963, 42963), CharacterRange::new(42965, 42972), CharacterRange::new(42994, 43047),
    CharacterRange::new(43052, 43052), CharacterRange::new(43072, 43123), CharacterRange::new(43136, 43205), CharacterRange::new(43216, 43225), CharacterRange::new(43232, 43255), CharacterRange::new(43259, 43259),
    CharacterRange::new(43261, 43309), CharacterRange::new(43312, 43347), CharacterRange::new(43360, 43388), CharacterRange::new(43392, 43456), CharacterRange::new(43471, 43481), CharacterRange::new(43488, 43518),
    CharacterRange::new(43520, 43574), CharacterRange::new(43584, 43597), CharacterRange::new(43600, 43609), CharacterRange::new(43616, 43638), CharacterRange::new(43642, 43714), CharacterRange::new(43739, 43741),
    CharacterRange::new(43744, 43759), CharacterRange::new(43762, 43766), CharacterRange::new(43777, 43782), CharacterRange::new(43785, 43790), CharacterRange::new(43793, 43798), CharacterRange::new(43808, 43814),
    CharacterRange::new(43816, 43822), CharacterRange::new(43824, 43866), CharacterRange::new(43868, 43881), CharacterRange::new(43888, 44010), CharacterRange::new(44012, 44013), CharacterRange::new(44016, 44025),
    CharacterRange::new(44032, 55203), CharacterRange::new(55216, 55238), CharacterRange::new(55243, 55291), CharacterRange::new(63744, 64109), CharacterRange::new(64112, 64217), CharacterRange::new(64256, 64262),
    CharacterRange::new(64275, 64279), CharacterRange::new(64285, 64296), CharacterRange::new(64298, 64310), CharacterRange::new(64312, 64316), CharacterRange::new(64318, 64318), CharacterRange::new(64320, 64321),
    CharacterRange::new(64323, 64324), CharacterRange::new(64326, 64433), CharacterRange::new(64467, 64605), CharacterRange::new(64612, 64829), CharacterRange::new(64848, 64911), CharacterRange::new(64914, 64967),
    CharacterRange::new(65008, 65017), CharacterRange::new(65024, 65039), CharacterRange::new(65056, 65071), CharacterRange::new(65075, 65076), CharacterRange::new(65101, 65103), CharacterRange::new(65137, 65137),
    CharacterRange::new(65139, 65139), CharacterRange::new(65143, 65143), CharacterRange::new(65145, 65145), CharacterRange::new(65147, 65147), CharacterRange::new(65149, 65149), CharacterRange::new(65151, 65276),
    CharacterRange::new(65296, 65305), CharacterRange::new(65313, 65338), CharacterRange::new(65343, 65343), CharacterRange::new(65345, 65370), CharacterRange::new(65381, 65470), CharacterRange::new(65474, 65479),
    CharacterRange::new(65482, 65487), CharacterRange::new(65490, 65495), CharacterRange::new(65498, 65500), CharacterRange::new(65536, 65547), CharacterRange::new(65549, 65574), CharacterRange::new(65576, 65594),
    CharacterRange::new(65596, 65597), CharacterRange::new(65599, 65613), CharacterRange::new(65616, 65629), CharacterRange::new(65664, 65786), CharacterRange::new(65856, 65908), CharacterRange::new(66045, 66045),
    CharacterRange::new(66176, 66204), CharacterRange::new(66208, 66256), CharacterRange::new(66272, 66272), CharacterRange::new(66304, 66335), CharacterRange::new(66349, 66378), CharacterRange::new(66384, 66426),
    CharacterRange::new(66432, 66461), CharacterRange::new(66464, 66499), CharacterRange::new(66504, 66511), CharacterRange::new(66513, 66517), CharacterRange::new(66560, 66717), CharacterRange::new(66720, 66729),
    CharacterRange::new(66736, 66771), CharacterRange::new(66776, 66811), CharacterRange::new(66816, 66855), CharacterRange::new(66864, 66915), CharacterRange::new(66928, 66938), CharacterRange::new(66940, 66954),
    CharacterRange::new(66956, 66962), CharacterRange::new(66964, 66965), CharacterRange::new(66967, 66977), CharacterRange::new(66979, 66993), CharacterRange::new(66995, 67001), CharacterRange::new(67003, 67004),
    CharacterRange::new(67008, 67059), CharacterRange::new(67072, 67382), CharacterRange::new(67392, 67413), CharacterRange::new(67424, 67431), CharacterRange::new(67456, 67461), CharacterRange::new(67463, 67504),
    CharacterRange::new(67506, 67514), CharacterRange::new(67584, 67589), CharacterRange::new(67592, 67592), CharacterRange::new(67594, 67637), CharacterRange::new(67639, 67640), CharacterRange::new(67644, 67644),
    CharacterRange::new(67647, 67669), CharacterRange::new(67680, 67702), CharacterRange::new(67712, 67742), CharacterRange::new(67808, 67826), CharacterRange::new(67828, 67829), CharacterRange::new(67840, 67861),
    CharacterRange::new(67872, 67897), CharacterRange::new(67968, 68023), CharacterRange::new(68030, 68031), CharacterRange::new(68096, 68099), CharacterRange::new(68101, 68102), CharacterRange::new(68108, 68115),
    CharacterRange::new(68117, 68119), CharacterRange::new(68121, 68149), CharacterRange::new(68152, 68154), CharacterRange::new(68159, 68159), CharacterRange::new(68192, 68220), CharacterRange::new(68224, 68252),
    CharacterRange::new(68288, 68295), CharacterRange::new(68297, 68326), CharacterRange::new(68352, 68405), CharacterRange::new(68416, 68437), CharacterRange::new(68448, 68466), CharacterRange::new(68480, 68497),
    CharacterRange::new(68608, 68680), CharacterRange::new(68736, 68786), CharacterRange::new(68800, 68850), CharacterRange::new(68864, 68903), CharacterRange::new(68912, 68921), CharacterRange::new(68928, 68965),
    CharacterRange::new(68969, 68973), CharacterRange::new(68975, 68997), CharacterRange::new(69248, 69289), CharacterRange::new(69291, 69292), CharacterRange::new(69296, 69297), CharacterRange::new(69314, 69316),
    CharacterRange::new(69372, 69404), CharacterRange::new(69415, 69415), CharacterRange::new(69424, 69456), CharacterRange::new(69488, 69509), CharacterRange::new(69552, 69572), CharacterRange::new(69600, 69622),
    CharacterRange::new(69632, 69702), CharacterRange::new(69734, 69749), CharacterRange::new(69759, 69818), CharacterRange::new(69826, 69826), CharacterRange::new(69840, 69864), CharacterRange::new(69872, 69881),
    CharacterRange::new(69888, 69940), CharacterRange::new(69942, 69951), CharacterRange::new(69956, 69959), CharacterRange::new(69968, 70003), CharacterRange::new(70006, 70006), CharacterRange::new(70016, 70084),
    CharacterRange::new(70089, 70092), CharacterRange::new(70094, 70106), CharacterRange::new(70108, 70108), CharacterRange::new(70144, 70161), CharacterRange::new(70163, 70199), CharacterRange::new(70206, 70209),
    CharacterRange::new(70272, 70278), CharacterRange::new(70280, 70280), CharacterRange::new(70282, 70285), CharacterRange::new(70287, 70301), CharacterRange::new(70303, 70312), CharacterRange::new(70320, 70378),
    CharacterRange::new(70384, 70393), CharacterRange::new(70400, 70403), CharacterRange::new(70405, 70412), CharacterRange::new(70415, 70416), CharacterRange::new(70419, 70440), CharacterRange::new(70442, 70448),
    CharacterRange::new(70450, 70451), CharacterRange::new(70453, 70457), CharacterRange::new(70459, 70468), CharacterRange::new(70471, 70472), CharacterRange::new(70475, 70477), CharacterRange::new(70480, 70480),
    CharacterRange::new(70487, 70487), CharacterRange::new(70493, 70499), CharacterRange::new(70502, 70508), CharacterRange::new(70512, 70516), CharacterRange::new(70528, 70537), CharacterRange::new(70539, 70539),
    CharacterRange::new(70542, 70542), CharacterRange::new(70544, 70581), CharacterRange::new(70583, 70592), CharacterRange::new(70594, 70594), CharacterRange::new(70597, 70597), CharacterRange::new(70599, 70602),
    CharacterRange::new(70604, 70611), CharacterRange::new(70625, 70626), CharacterRange::new(70656, 70730), CharacterRange::new(70736, 70745), CharacterRange::new(70750, 70753), CharacterRange::new(70784, 70853),
    CharacterRange::new(70855, 70855), CharacterRange::new(70864, 70873), CharacterRange::new(71040, 71093), CharacterRange::new(71096, 71104), CharacterRange::new(71128, 71133), CharacterRange::new(71168, 71232),
    CharacterRange::new(71236, 71236), CharacterRange::new(71248, 71257), CharacterRange::new(71296, 71352), CharacterRange::new(71360, 71369), CharacterRange::new(71376, 71395), CharacterRange::new(71424, 71450),
    CharacterRange::new(71453, 71467), CharacterRange::new(71472, 71481), CharacterRange::new(71488, 71494), CharacterRange::new(71680, 71738), CharacterRange::new(71840, 71913), CharacterRange::new(71935, 71942),
    CharacterRange::new(71945, 71945), CharacterRange::new(71948, 71955), CharacterRange::new(71957, 71958), CharacterRange::new(71960, 71989), CharacterRange::new(71991, 71992), CharacterRange::new(71995, 72003),
    CharacterRange::new(72016, 72025), CharacterRange::new(72096, 72103), CharacterRange::new(72106, 72151), CharacterRange::new(72154, 72161), CharacterRange::new(72163, 72164), CharacterRange::new(72192, 72254),
    CharacterRange::new(72263, 72263), CharacterRange::new(72272, 72345), CharacterRange::new(72349, 72349), CharacterRange::new(72368, 72440), CharacterRange::new(72640, 72672), CharacterRange::new(72688, 72697),
    CharacterRange::new(72704, 72712), CharacterRange::new(72714, 72758), CharacterRange::new(72760, 72768), CharacterRange::new(72784, 72793), CharacterRange::new(72818, 72847), CharacterRange::new(72850, 72871),
    CharacterRange::new(72873, 72886), CharacterRange::new(72960, 72966), CharacterRange::new(72968, 72969), CharacterRange::new(72971, 73014), CharacterRange::new(73018, 73018), CharacterRange::new(73020, 73021),
    CharacterRange::new(73023, 73031), CharacterRange::new(73040, 73049), CharacterRange::new(73056, 73061), CharacterRange::new(73063, 73064), CharacterRange::new(73066, 73102), CharacterRange::new(73104, 73105),
    CharacterRange::new(73107, 73112), CharacterRange::new(73120, 73129), CharacterRange::new(73440, 73462), CharacterRange::new(73472, 73488), CharacterRange::new(73490, 73530), CharacterRange::new(73534, 73538),
    CharacterRange::new(73552, 73562), CharacterRange::new(73648, 73648), CharacterRange::new(73728, 74649), CharacterRange::new(74752, 74862), CharacterRange::new(74880, 75075), CharacterRange::new(77712, 77808),
    CharacterRange::new(77824, 78895), CharacterRange::new(78912, 78933), CharacterRange::new(78944, 82938), CharacterRange::new(82944, 83526), CharacterRange::new(90368, 90425), CharacterRange::new(92160, 92728),
    CharacterRange::new(92736, 92766), CharacterRange::new(92768, 92777), CharacterRange::new(92784, 92862), CharacterRange::new(92864, 92873), CharacterRange::new(92880, 92909), CharacterRange::new(92912, 92916),
    CharacterRange::new(92928, 92982), CharacterRange::new(92992, 92995), CharacterRange::new(93008, 93017), CharacterRange::new(93027, 93047), CharacterRange::new(93053, 93071), CharacterRange::new(93504, 93548),
    CharacterRange::new(93552, 93561), CharacterRange::new(93760, 93823), CharacterRange::new(93952, 94026), CharacterRange::new(94031, 94087), CharacterRange::new(94095, 94111), CharacterRange::new(94176, 94177),
    CharacterRange::new(94179, 94180), CharacterRange::new(94192, 94193), CharacterRange::new(94208, 100343), CharacterRange::new(100352, 101589), CharacterRange::new(101631, 101640), CharacterRange::new(110576, 110579),
    CharacterRange::new(110581, 110587), CharacterRange::new(110589, 110590), CharacterRange::new(110592, 110882), CharacterRange::new(110898, 110898), CharacterRange::new(110928, 110930), CharacterRange::new(110933, 110933),
    CharacterRange::new(110948, 110951), CharacterRange::new(110960, 111355), CharacterRange::new(113664, 113770), CharacterRange::new(113776, 113788), CharacterRange::new(113792, 113800), CharacterRange::new(113808, 113817),
    CharacterRange::new(113821, 113822), CharacterRange::new(118000, 118009), CharacterRange::new(118528, 118573), CharacterRange::new(118576, 118598), CharacterRange::new(119141, 119145), CharacterRange::new(119149, 119154),
    CharacterRange::new(119163, 119170), CharacterRange::new(119173, 119179), CharacterRange::new(119210, 119213), CharacterRange::new(119362, 119364), CharacterRange::new(119808, 119892), CharacterRange::new(119894, 119964),
    CharacterRange::new(119966, 119967), CharacterRange::new(119970, 119970), CharacterRange::new(119973, 119974), CharacterRange::new(119977, 119980), CharacterRange::new(119982, 119993), CharacterRange::new(119995, 119995),
    CharacterRange::new(119997, 120003), CharacterRange::new(120005, 120069), CharacterRange::new(120071, 120074), CharacterRange::new(120077, 120084), CharacterRange::new(120086, 120092), CharacterRange::new(120094, 120121),
    CharacterRange::new(120123, 120126), CharacterRange::new(120128, 120132), CharacterRange::new(120134, 120134), CharacterRange::new(120138, 120144), CharacterRange::new(120146, 120485), CharacterRange::new(120488, 120512),
    CharacterRange::new(120514, 120538), CharacterRange::new(120540, 120570), CharacterRange::new(120572, 120596), CharacterRange::new(120598, 120628), CharacterRange::new(120630, 120654), CharacterRange::new(120656, 120686),
    CharacterRange::new(120688, 120712), CharacterRange::new(120714, 120744), CharacterRange::new(120746, 120770), CharacterRange::new(120772, 120779), CharacterRange::new(120782, 120831), CharacterRange::new(121344, 121398),
    CharacterRange::new(121403, 121452), CharacterRange::new(121461, 121461), CharacterRange::new(121476, 121476), CharacterRange::new(121499, 121503), CharacterRange::new(121505, 121519), CharacterRange::new(122624, 122654),
    CharacterRange::new(122661, 122666), CharacterRange::new(122880, 122886), CharacterRange::new(122888, 122904), CharacterRange::new(122907, 122913), CharacterRange::new(122915, 122916), CharacterRange::new(122918, 122922),
    CharacterRange::new(122928, 122989), CharacterRange::new(123023, 123023), CharacterRange::new(123136, 123180), CharacterRange::new(123184, 123197), CharacterRange::new(123200, 123209), CharacterRange::new(123214, 123214),
    CharacterRange::new(123536, 123566), CharacterRange::new(123584, 123641), CharacterRange::new(124112, 124153), CharacterRange::new(124368, 124410), CharacterRange::new(124896, 124902), CharacterRange::new(124904, 124907),
    CharacterRange::new(124909, 124910), CharacterRange::new(124912, 124926), CharacterRange::new(124928, 125124), CharacterRange::new(125136, 125142), CharacterRange::new(125184, 125259), CharacterRange::new(125264, 125273),
    CharacterRange::new(126464, 126467), CharacterRange::new(126469, 126495), CharacterRange::new(126497, 126498), CharacterRange::new(126500, 126500), CharacterRange::new(126503, 126503), CharacterRange::new(126505, 126514),
    CharacterRange::new(126516, 126519), CharacterRange::new(126521, 126521), CharacterRange::new(126523, 126523), CharacterRange::new(126530, 126530), CharacterRange::new(126535, 126535), CharacterRange::new(126537, 126537),
    CharacterRange::new(126539, 126539), CharacterRange::new(126541, 126543), CharacterRange::new(126545, 126546), CharacterRange::new(126548, 126548), CharacterRange::new(126551, 126551), CharacterRange::new(126553, 126553),
    CharacterRange::new(126555, 126555), CharacterRange::new(126557, 126557), CharacterRange::new(126559, 126559), CharacterRange::new(126561, 126562), CharacterRange::new(126564, 126564), CharacterRange::new(126567, 126570),
    CharacterRange::new(126572, 126578), CharacterRange::new(126580, 126583), CharacterRange::new(126585, 126588), CharacterRange::new(126590, 126590), CharacterRange::new(126592, 126601), CharacterRange::new(126603, 126619),
    CharacterRange::new(126625, 126627), CharacterRange::new(126629, 126633), CharacterRange::new(126635, 126651), CharacterRange::new(130032, 130041), CharacterRange::new(131072, 173791), CharacterRange::new(173824, 177977),
    CharacterRange::new(177984, 178205), CharacterRange::new(178208, 183969), CharacterRange::new(183984, 191456), CharacterRange::new(191472, 192093), CharacterRange::new(194560, 195101), CharacterRange::new(196608, 201546),
    CharacterRange::new(201552, 205743), CharacterRange::new(917760, 917999),
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
                if eof { state = 54; lexer.advance(false); continue; }
                if lookahead == 13 { state = 52; lexer.advance(true); continue; }
                if lookahead == 33 { state = 21; lexer.advance(false); continue; }
                if lookahead == 35 { state = 148; lexer.advance(false); continue; }
                if lookahead == 37 { state = 92; lexer.advance(false); continue; }
                if lookahead == 38 { state = 96; lexer.advance(false); continue; }
                if lookahead == 40 { state = 58; lexer.advance(false); continue; }
                if lookahead == 41 { state = 59; lexer.advance(false); continue; }
                if lookahead == 42 { state = 70; lexer.advance(false); continue; }
                if lookahead == 43 { state = 88; lexer.advance(false); continue; }
                if lookahead == 44 { state = 60; lexer.advance(false); continue; }
                if lookahead == 45 { state = 82; lexer.advance(false); continue; }
                if lookahead == 46 { state = 57; lexer.advance(false); continue; }
                if lookahead == 47 { state = 89; lexer.advance(false); continue; }
                if lookahead == 48 { state = 136; lexer.advance(false); continue; }
                if lookahead == 58 { state = 69; lexer.advance(false); continue; }
                if lookahead == 59 { state = 55; lexer.advance(false); continue; }
                if lookahead == 60 { state = 102; lexer.advance(false); continue; }
                if lookahead == 61 { state = 75; lexer.advance(false); continue; }
                if lookahead == 62 { state = 110; lexer.advance(false); continue; }
                if lookahead == 64 { state = 79; lexer.advance(false); continue; }
                if lookahead == 91 { state = 76; lexer.advance(false); continue; }
                if lookahead == 92 { state = 130; lexer.advance(false); continue; }
                if lookahead == 93 { state = 77; lexer.advance(false); continue; }
                if lookahead == 94 { state = 98; lexer.advance(false); continue; }
                if lookahead == 123 { state = 85; lexer.advance(false); continue; }
                if lookahead == 124 { state = 84; lexer.advance(false); continue; }
                if lookahead == 125 { state = 86; lexer.advance(false); continue; }
                if lookahead == 126 { state = 101; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 || lookahead == 8203 || lookahead == 8288 || lookahead == 65279 { state = 52; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 137; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 147; lexer.advance(false); continue; }
                return result;
            }
            1 => {
                if lookahead == 10 { state = 149; lexer.advance(false); continue; }
                return result;
            }
            2 => {
                if lookahead == 10 { state = 127; lexer.advance(false); continue; }
                return result;
            }
            3 => {
                if lookahead == 10 { state = 3; lexer.advance(true); continue; }
                if lookahead == 13 { state = 132; lexer.advance(false); continue; }
                if lookahead == 35 { state = 133; lexer.advance(false); continue; }
                if lookahead == 92 { state = 131; lexer.advance(false); continue; }
                if lookahead == 123 { state = 85; lexer.advance(false); continue; }
                if lookahead == 125 { state = 86; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 || lookahead == 8203 || lookahead == 8288 || lookahead == 65279 { state = 132; lexer.advance(false); continue; }
                if lookahead != 0 { state = 133; lexer.advance(false); continue; }
                return result;
            }
            4 => {
                if lookahead == 13 { state = 4; lexer.advance(true); continue; }
                if lookahead == 33 { state = 20; lexer.advance(false); continue; }
                if lookahead == 35 { state = 148; lexer.advance(false); continue; }
                if lookahead == 37 { state = 92; lexer.advance(false); continue; }
                if lookahead == 38 { state = 96; lexer.advance(false); continue; }
                if lookahead == 40 { state = 58; lexer.advance(false); continue; }
                if lookahead == 42 { state = 62; lexer.advance(false); continue; }
                if lookahead == 43 { state = 88; lexer.advance(false); continue; }
                if lookahead == 44 { state = 60; lexer.advance(false); continue; }
                if lookahead == 45 { state = 81; lexer.advance(false); continue; }
                if lookahead == 46 { state = 57; lexer.advance(false); continue; }
                if lookahead == 47 { state = 89; lexer.advance(false); continue; }
                if lookahead == 48 { state = 136; lexer.advance(false); continue; }
                if lookahead == 58 { state = 69; lexer.advance(false); continue; }
                if lookahead == 59 { state = 55; lexer.advance(false); continue; }
                if lookahead == 60 { state = 102; lexer.advance(false); continue; }
                if lookahead == 61 { state = 75; lexer.advance(false); continue; }
                if lookahead == 62 { state = 110; lexer.advance(false); continue; }
                if lookahead == 64 { state = 79; lexer.advance(false); continue; }
                if lookahead == 91 { state = 76; lexer.advance(false); continue; }
                if lookahead == 92 { state = 11; lexer.advance(false); continue; }
                if lookahead == 94 { state = 98; lexer.advance(false); continue; }
                if lookahead == 123 { state = 85; lexer.advance(false); continue; }
                if lookahead == 124 { state = 84; lexer.advance(false); continue; }
                if lookahead == 126 { state = 101; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 || lookahead == 8203 || lookahead == 8288 || lookahead == 65279 { state = 4; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 137; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 147; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                if lookahead == 13 { state = 5; lexer.advance(true); continue; }
                if lookahead == 33 { state = 21; lexer.advance(false); continue; }
                if lookahead == 35 { state = 148; lexer.advance(false); continue; }
                if lookahead == 37 { state = 91; lexer.advance(false); continue; }
                if lookahead == 38 { state = 95; lexer.advance(false); continue; }
                if lookahead == 40 { state = 58; lexer.advance(false); continue; }
                if lookahead == 41 { state = 59; lexer.advance(false); continue; }
                if lookahead == 42 { state = 63; lexer.advance(false); continue; }
                if lookahead == 43 { state = 87; lexer.advance(false); continue; }
                if lookahead == 44 { state = 60; lexer.advance(false); continue; }
                if lookahead == 45 { state = 80; lexer.advance(false); continue; }
                if lookahead == 46 { state = 57; lexer.advance(false); continue; }
                if lookahead == 47 { state = 90; lexer.advance(false); continue; }
                if lookahead == 48 { state = 136; lexer.advance(false); continue; }
                if lookahead == 58 { state = 69; lexer.advance(false); continue; }
                if lookahead == 59 { state = 55; lexer.advance(false); continue; }
                if lookahead == 60 { state = 103; lexer.advance(false); continue; }
                if lookahead == 61 { state = 75; lexer.advance(false); continue; }
                if lookahead == 62 { state = 111; lexer.advance(false); continue; }
                if lookahead == 64 { state = 78; lexer.advance(false); continue; }
                if lookahead == 91 { state = 76; lexer.advance(false); continue; }
                if lookahead == 92 { state = 11; lexer.advance(false); continue; }
                if lookahead == 93 { state = 77; lexer.advance(false); continue; }
                if lookahead == 94 { state = 97; lexer.advance(false); continue; }
                if lookahead == 123 { state = 85; lexer.advance(false); continue; }
                if lookahead == 124 { state = 83; lexer.advance(false); continue; }
                if lookahead == 125 { state = 86; lexer.advance(false); continue; }
                if lookahead == 126 { state = 101; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 || lookahead == 8203 || lookahead == 8288 || lookahead == 65279 { state = 5; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 137; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 147; lexer.advance(false); continue; }
                return result;
            }
            6 => {
                if lookahead == 13 { state = 6; lexer.advance(true); continue; }
                if lookahead == 35 { state = 148; lexer.advance(false); continue; }
                if lookahead == 37 { state = 22; lexer.advance(false); continue; }
                if lookahead == 38 { state = 23; lexer.advance(false); continue; }
                if lookahead == 40 { state = 58; lexer.advance(false); continue; }
                if lookahead == 42 { state = 64; lexer.advance(false); continue; }
                if lookahead == 43 { state = 88; lexer.advance(false); continue; }
                if lookahead == 45 { state = 81; lexer.advance(false); continue; }
                if lookahead == 46 { state = 16; lexer.advance(false); continue; }
                if lookahead == 47 { state = 18; lexer.advance(false); continue; }
                if lookahead == 48 { state = 136; lexer.advance(false); continue; }
                if lookahead == 58 { state = 68; lexer.advance(false); continue; }
                if lookahead == 59 { state = 55; lexer.advance(false); continue; }
                if lookahead == 60 { state = 19; lexer.advance(false); continue; }
                if lookahead == 61 { state = 74; lexer.advance(false); continue; }
                if lookahead == 62 { state = 32; lexer.advance(false); continue; }
                if lookahead == 64 { state = 24; lexer.advance(false); continue; }
                if lookahead == 91 { state = 76; lexer.advance(false); continue; }
                if lookahead == 92 { state = 11; lexer.advance(false); continue; }
                if lookahead == 94 { state = 25; lexer.advance(false); continue; }
                if lookahead == 123 { state = 85; lexer.advance(false); continue; }
                if lookahead == 124 { state = 26; lexer.advance(false); continue; }
                if lookahead == 126 { state = 101; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 || lookahead == 8203 || lookahead == 8288 || lookahead == 65279 { state = 6; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 137; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 147; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if lookahead == 13 { state = 7; lexer.advance(true); continue; }
                if lookahead == 35 { state = 148; lexer.advance(false); continue; }
                if lookahead == 40 { state = 58; lexer.advance(false); continue; }
                if lookahead == 42 { state = 70; lexer.advance(false); continue; }
                if lookahead == 43 { state = 87; lexer.advance(false); continue; }
                if lookahead == 45 { state = 80; lexer.advance(false); continue; }
                if lookahead == 46 { state = 16; lexer.advance(false); continue; }
                if lookahead == 48 { state = 136; lexer.advance(false); continue; }
                if lookahead == 58 { state = 68; lexer.advance(false); continue; }
                if lookahead == 91 { state = 76; lexer.advance(false); continue; }
                if lookahead == 92 { state = 11; lexer.advance(false); continue; }
                if lookahead == 123 { state = 85; lexer.advance(false); continue; }
                if lookahead == 126 { state = 101; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 || lookahead == 8203 || lookahead == 8288 || lookahead == 65279 { state = 7; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 137; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 147; lexer.advance(false); continue; }
                return result;
            }
            8 => {
                if lookahead == 13 { state = 8; lexer.advance(true); continue; }
                if lookahead == 33 { state = 20; lexer.advance(false); continue; }
                if lookahead == 35 { state = 148; lexer.advance(false); continue; }
                if lookahead == 37 { state = 92; lexer.advance(false); continue; }
                if lookahead == 38 { state = 96; lexer.advance(false); continue; }
                if lookahead == 40 { state = 58; lexer.advance(false); continue; }
                if lookahead == 41 { state = 59; lexer.advance(false); continue; }
                if lookahead == 42 { state = 62; lexer.advance(false); continue; }
                if lookahead == 43 { state = 88; lexer.advance(false); continue; }
                if lookahead == 44 { state = 60; lexer.advance(false); continue; }
                if lookahead == 45 { state = 81; lexer.advance(false); continue; }
                if lookahead == 46 { state = 56; lexer.advance(false); continue; }
                if lookahead == 47 { state = 89; lexer.advance(false); continue; }
                if lookahead == 58 { state = 69; lexer.advance(false); continue; }
                if lookahead == 59 { state = 55; lexer.advance(false); continue; }
                if lookahead == 60 { state = 102; lexer.advance(false); continue; }
                if lookahead == 61 { state = 75; lexer.advance(false); continue; }
                if lookahead == 62 { state = 110; lexer.advance(false); continue; }
                if lookahead == 64 { state = 79; lexer.advance(false); continue; }
                if lookahead == 91 { state = 76; lexer.advance(false); continue; }
                if lookahead == 92 { state = 11; lexer.advance(false); continue; }
                if lookahead == 93 { state = 77; lexer.advance(false); continue; }
                if lookahead == 94 { state = 98; lexer.advance(false); continue; }
                if lookahead == 124 { state = 84; lexer.advance(false); continue; }
                if lookahead == 125 { state = 86; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 || lookahead == 8203 || lookahead == 8288 || lookahead == 65279 { state = 8; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 147; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if lookahead == 13 { state = 9; lexer.advance(true); continue; }
                if lookahead == 33 { state = 20; lexer.advance(false); continue; }
                if lookahead == 35 { state = 148; lexer.advance(false); continue; }
                if lookahead == 37 { state = 92; lexer.advance(false); continue; }
                if lookahead == 38 { state = 96; lexer.advance(false); continue; }
                if lookahead == 40 { state = 58; lexer.advance(false); continue; }
                if lookahead == 41 { state = 59; lexer.advance(false); continue; }
                if lookahead == 42 { state = 62; lexer.advance(false); continue; }
                if lookahead == 43 { state = 88; lexer.advance(false); continue; }
                if lookahead == 44 { state = 60; lexer.advance(false); continue; }
                if lookahead == 45 { state = 81; lexer.advance(false); continue; }
                if lookahead == 46 { state = 56; lexer.advance(false); continue; }
                if lookahead == 47 { state = 89; lexer.advance(false); continue; }
                if lookahead == 58 { state = 68; lexer.advance(false); continue; }
                if lookahead == 59 { state = 55; lexer.advance(false); continue; }
                if lookahead == 60 { state = 102; lexer.advance(false); continue; }
                if lookahead == 61 { state = 75; lexer.advance(false); continue; }
                if lookahead == 62 { state = 110; lexer.advance(false); continue; }
                if lookahead == 64 { state = 79; lexer.advance(false); continue; }
                if lookahead == 91 { state = 76; lexer.advance(false); continue; }
                if lookahead == 92 { state = 11; lexer.advance(false); continue; }
                if lookahead == 93 { state = 77; lexer.advance(false); continue; }
                if lookahead == 94 { state = 98; lexer.advance(false); continue; }
                if lookahead == 124 { state = 84; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 || lookahead == 8203 || lookahead == 8288 || lookahead == 65279 { state = 9; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 147; lexer.advance(false); continue; }
                return result;
            }
            10 => {
                if lookahead == 13 { state = 10; lexer.advance(true); continue; }
                if lookahead == 35 { state = 148; lexer.advance(false); continue; }
                if lookahead == 45 { state = 31; lexer.advance(false); continue; }
                if lookahead == 58 { state = 68; lexer.advance(false); continue; }
                if lookahead == 92 { state = 11; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 || lookahead == 8203 || lookahead == 8288 || lookahead == 65279 { state = 10; lexer.advance(true); continue; }
                return result;
            }
            11 => {
                if lookahead == 13 { state = 1; lexer.advance(false); continue; }
                if !eof && lookahead == 0 || lookahead == 10 { state = 149; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if lookahead == 13 { state = 12; lexer.advance(true); continue; }
                if lookahead == 33 { state = 20; lexer.advance(false); continue; }
                if lookahead == 35 { state = 148; lexer.advance(false); continue; }
                if lookahead == 37 { state = 92; lexer.advance(false); continue; }
                if lookahead == 38 { state = 96; lexer.advance(false); continue; }
                if lookahead == 40 { state = 58; lexer.advance(false); continue; }
                if lookahead == 42 { state = 62; lexer.advance(false); continue; }
                if lookahead == 43 { state = 88; lexer.advance(false); continue; }
                if lookahead == 44 { state = 60; lexer.advance(false); continue; }
                if lookahead == 45 { state = 81; lexer.advance(false); continue; }
                if lookahead == 46 { state = 57; lexer.advance(false); continue; }
                if lookahead == 47 { state = 89; lexer.advance(false); continue; }
                if lookahead == 48 { state = 136; lexer.advance(false); continue; }
                if lookahead == 58 { state = 68; lexer.advance(false); continue; }
                if lookahead == 59 { state = 55; lexer.advance(false); continue; }
                if lookahead == 60 { state = 102; lexer.advance(false); continue; }
                if lookahead == 61 { state = 75; lexer.advance(false); continue; }
                if lookahead == 62 { state = 110; lexer.advance(false); continue; }
                if lookahead == 64 { state = 79; lexer.advance(false); continue; }
                if lookahead == 91 { state = 76; lexer.advance(false); continue; }
                if lookahead == 92 { state = 11; lexer.advance(false); continue; }
                if lookahead == 94 { state = 98; lexer.advance(false); continue; }
                if lookahead == 123 { state = 85; lexer.advance(false); continue; }
                if lookahead == 124 { state = 84; lexer.advance(false); continue; }
                if lookahead == 126 { state = 101; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 || lookahead == 8203 || lookahead == 8288 || lookahead == 65279 { state = 12; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 137; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 147; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if lookahead == 13 { state = 13; lexer.advance(true); continue; }
                if lookahead == 33 { state = 21; lexer.advance(false); continue; }
                if lookahead == 35 { state = 148; lexer.advance(false); continue; }
                if lookahead == 37 { state = 91; lexer.advance(false); continue; }
                if lookahead == 38 { state = 95; lexer.advance(false); continue; }
                if lookahead == 40 { state = 58; lexer.advance(false); continue; }
                if lookahead == 41 { state = 59; lexer.advance(false); continue; }
                if lookahead == 42 { state = 63; lexer.advance(false); continue; }
                if lookahead == 43 { state = 87; lexer.advance(false); continue; }
                if lookahead == 44 { state = 60; lexer.advance(false); continue; }
                if lookahead == 45 { state = 80; lexer.advance(false); continue; }
                if lookahead == 46 { state = 57; lexer.advance(false); continue; }
                if lookahead == 47 { state = 90; lexer.advance(false); continue; }
                if lookahead == 48 { state = 136; lexer.advance(false); continue; }
                if lookahead == 58 { state = 68; lexer.advance(false); continue; }
                if lookahead == 59 { state = 55; lexer.advance(false); continue; }
                if lookahead == 60 { state = 103; lexer.advance(false); continue; }
                if lookahead == 61 { state = 75; lexer.advance(false); continue; }
                if lookahead == 62 { state = 111; lexer.advance(false); continue; }
                if lookahead == 64 { state = 78; lexer.advance(false); continue; }
                if lookahead == 91 { state = 76; lexer.advance(false); continue; }
                if lookahead == 92 { state = 11; lexer.advance(false); continue; }
                if lookahead == 93 { state = 77; lexer.advance(false); continue; }
                if lookahead == 94 { state = 97; lexer.advance(false); continue; }
                if lookahead == 123 { state = 85; lexer.advance(false); continue; }
                if lookahead == 124 { state = 83; lexer.advance(false); continue; }
                if lookahead == 125 { state = 86; lexer.advance(false); continue; }
                if lookahead == 126 { state = 101; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 || lookahead == 8203 || lookahead == 8288 || lookahead == 65279 { state = 13; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 137; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 147; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if lookahead == 13 { state = 14; lexer.advance(true); continue; }
                if lookahead == 33 { state = 21; lexer.advance(false); continue; }
                if lookahead == 35 { state = 148; lexer.advance(false); continue; }
                if lookahead == 37 { state = 91; lexer.advance(false); continue; }
                if lookahead == 38 { state = 95; lexer.advance(false); continue; }
                if lookahead == 40 { state = 58; lexer.advance(false); continue; }
                if lookahead == 41 { state = 59; lexer.advance(false); continue; }
                if lookahead == 42 { state = 63; lexer.advance(false); continue; }
                if lookahead == 43 { state = 87; lexer.advance(false); continue; }
                if lookahead == 44 { state = 60; lexer.advance(false); continue; }
                if lookahead == 45 { state = 80; lexer.advance(false); continue; }
                if lookahead == 46 { state = 56; lexer.advance(false); continue; }
                if lookahead == 47 { state = 90; lexer.advance(false); continue; }
                if lookahead == 58 { state = 68; lexer.advance(false); continue; }
                if lookahead == 59 { state = 55; lexer.advance(false); continue; }
                if lookahead == 60 { state = 103; lexer.advance(false); continue; }
                if lookahead == 61 { state = 75; lexer.advance(false); continue; }
                if lookahead == 62 { state = 111; lexer.advance(false); continue; }
                if lookahead == 64 { state = 78; lexer.advance(false); continue; }
                if lookahead == 91 { state = 76; lexer.advance(false); continue; }
                if lookahead == 92 { state = 11; lexer.advance(false); continue; }
                if lookahead == 93 { state = 77; lexer.advance(false); continue; }
                if lookahead == 94 { state = 97; lexer.advance(false); continue; }
                if lookahead == 124 { state = 83; lexer.advance(false); continue; }
                if lookahead == 125 { state = 86; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 || lookahead == 8203 || lookahead == 8288 || lookahead == 65279 { state = 14; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 147; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if lookahead == 13 { state = 15; lexer.advance(true); continue; }
                if lookahead == 33 { state = 21; lexer.advance(false); continue; }
                if lookahead == 35 { state = 148; lexer.advance(false); continue; }
                if lookahead == 37 { state = 91; lexer.advance(false); continue; }
                if lookahead == 38 { state = 95; lexer.advance(false); continue; }
                if lookahead == 40 { state = 58; lexer.advance(false); continue; }
                if lookahead == 41 { state = 59; lexer.advance(false); continue; }
                if lookahead == 42 { state = 63; lexer.advance(false); continue; }
                if lookahead == 43 { state = 87; lexer.advance(false); continue; }
                if lookahead == 44 { state = 60; lexer.advance(false); continue; }
                if lookahead == 45 { state = 80; lexer.advance(false); continue; }
                if lookahead == 46 { state = 56; lexer.advance(false); continue; }
                if lookahead == 47 { state = 90; lexer.advance(false); continue; }
                if lookahead == 58 { state = 69; lexer.advance(false); continue; }
                if lookahead == 59 { state = 55; lexer.advance(false); continue; }
                if lookahead == 60 { state = 103; lexer.advance(false); continue; }
                if lookahead == 61 { state = 75; lexer.advance(false); continue; }
                if lookahead == 62 { state = 111; lexer.advance(false); continue; }
                if lookahead == 64 { state = 78; lexer.advance(false); continue; }
                if lookahead == 91 { state = 76; lexer.advance(false); continue; }
                if lookahead == 92 { state = 11; lexer.advance(false); continue; }
                if lookahead == 93 { state = 77; lexer.advance(false); continue; }
                if lookahead == 94 { state = 97; lexer.advance(false); continue; }
                if lookahead == 124 { state = 83; lexer.advance(false); continue; }
                if lookahead == 125 { state = 86; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 || lookahead == 8203 || lookahead == 8288 || lookahead == 65279 { state = 15; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 147; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if lookahead == 46 { state = 17; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 143; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if lookahead == 46 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            18 => {
                if lookahead == 47 { state = 27; lexer.advance(false); continue; }
                if lookahead == 61 { state = 116; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if lookahead == 60 { state = 28; lexer.advance(false); continue; }
                return result;
            }
            20 => {
                if lookahead == 61 { state = 107; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                if lookahead == 61 { state = 107; lexer.advance(false); continue; }
                if 97 <= lookahead && lookahead <= 122 { state = 134; lexer.advance(false); continue; }
                return result;
            }
            22 => {
                if lookahead == 61 { state = 119; lexer.advance(false); continue; }
                return result;
            }
            23 => {
                if lookahead == 61 { state = 123; lexer.advance(false); continue; }
                return result;
            }
            24 => {
                if lookahead == 61 { state = 117; lexer.advance(false); continue; }
                return result;
            }
            25 => {
                if lookahead == 61 { state = 124; lexer.advance(false); continue; }
                return result;
            }
            26 => {
                if lookahead == 61 { state = 125; lexer.advance(false); continue; }
                return result;
            }
            27 => {
                if lookahead == 61 { state = 118; lexer.advance(false); continue; }
                return result;
            }
            28 => {
                if lookahead == 61 { state = 122; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                if lookahead == 61 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            30 => {
                if lookahead == 61 { state = 120; lexer.advance(false); continue; }
                return result;
            }
            31 => {
                if lookahead == 62 { state = 71; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                if lookahead == 62 { state = 29; lexer.advance(false); continue; }
                return result;
            }
            33 => {
                if lookahead == 95 { state = 39; lexer.advance(false); continue; }
                if lookahead == 48 || lookahead == 49 { state = 139; lexer.advance(false); continue; }
                return result;
            }
            34 => {
                if lookahead == 95 { state = 40; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 55 { state = 140; lexer.advance(false); continue; }
                return result;
            }
            35 => {
                if lookahead == 95 { state = 43; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 141; lexer.advance(false); continue; }
                return result;
            }
            36 => {
                if lookahead == 123 { state = 51; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                if lookahead == 125 { state = 127; lexer.advance(false); continue; }
                if lookahead != 0 { state = 37; lexer.advance(false); continue; }
                return result;
            }
            38 => {
                if lookahead == 43 || lookahead == 45 { state = 41; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 144; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                if lookahead == 48 || lookahead == 49 { state = 139; lexer.advance(false); continue; }
                return result;
            }
            40 => {
                if 48 <= lookahead && lookahead <= 55 { state = 140; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                if 48 <= lookahead && lookahead <= 57 { state = 144; lexer.advance(false); continue; }
                return result;
            }
            42 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 127; lexer.advance(false); continue; }
                return result;
            }
            43 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 141; lexer.advance(false); continue; }
                return result;
            }
            44 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 42; lexer.advance(false); continue; }
                return result;
            }
            45 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 44; lexer.advance(false); continue; }
                return result;
            }
            46 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 45; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 46; lexer.advance(false); continue; }
                return result;
            }
            48 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 47; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 48; lexer.advance(false); continue; }
                return result;
            }
            50 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 49; lexer.advance(false); continue; }
                return result;
            }
            51 => {
                if lookahead != 0 && lookahead != 125 { state = 37; lexer.advance(false); continue; }
                return result;
            }
            52 => {
                if eof { state = 54; lexer.advance(false); continue; }
                if lookahead == 13 { state = 52; lexer.advance(true); continue; }
                if lookahead == 33 { state = 21; lexer.advance(false); continue; }
                if lookahead == 35 { state = 148; lexer.advance(false); continue; }
                if lookahead == 37 { state = 92; lexer.advance(false); continue; }
                if lookahead == 38 { state = 96; lexer.advance(false); continue; }
                if lookahead == 40 { state = 58; lexer.advance(false); continue; }
                if lookahead == 41 { state = 59; lexer.advance(false); continue; }
                if lookahead == 42 { state = 70; lexer.advance(false); continue; }
                if lookahead == 43 { state = 88; lexer.advance(false); continue; }
                if lookahead == 44 { state = 60; lexer.advance(false); continue; }
                if lookahead == 45 { state = 82; lexer.advance(false); continue; }
                if lookahead == 46 { state = 57; lexer.advance(false); continue; }
                if lookahead == 47 { state = 89; lexer.advance(false); continue; }
                if lookahead == 48 { state = 136; lexer.advance(false); continue; }
                if lookahead == 58 { state = 69; lexer.advance(false); continue; }
                if lookahead == 59 { state = 55; lexer.advance(false); continue; }
                if lookahead == 60 { state = 102; lexer.advance(false); continue; }
                if lookahead == 61 { state = 75; lexer.advance(false); continue; }
                if lookahead == 62 { state = 110; lexer.advance(false); continue; }
                if lookahead == 64 { state = 79; lexer.advance(false); continue; }
                if lookahead == 91 { state = 76; lexer.advance(false); continue; }
                if lookahead == 92 { state = 11; lexer.advance(false); continue; }
                if lookahead == 93 { state = 77; lexer.advance(false); continue; }
                if lookahead == 94 { state = 98; lexer.advance(false); continue; }
                if lookahead == 123 { state = 85; lexer.advance(false); continue; }
                if lookahead == 124 { state = 84; lexer.advance(false); continue; }
                if lookahead == 125 { state = 86; lexer.advance(false); continue; }
                if lookahead == 126 { state = 101; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 || lookahead == 8203 || lookahead == 8288 || lookahead == 65279 { state = 52; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 137; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 147; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                if eof { state = 54; lexer.advance(false); continue; }
                if lookahead == 13 { state = 53; lexer.advance(true); continue; }
                if lookahead == 33 { state = 21; lexer.advance(false); continue; }
                if lookahead == 35 { state = 148; lexer.advance(false); continue; }
                if lookahead == 40 { state = 58; lexer.advance(false); continue; }
                if lookahead == 41 { state = 59; lexer.advance(false); continue; }
                if lookahead == 42 { state = 61; lexer.advance(false); continue; }
                if lookahead == 43 { state = 87; lexer.advance(false); continue; }
                if lookahead == 44 { state = 60; lexer.advance(false); continue; }
                if lookahead == 45 { state = 80; lexer.advance(false); continue; }
                if lookahead == 46 { state = 16; lexer.advance(false); continue; }
                if lookahead == 48 { state = 136; lexer.advance(false); continue; }
                if lookahead == 58 { state = 68; lexer.advance(false); continue; }
                if lookahead == 59 { state = 55; lexer.advance(false); continue; }
                if lookahead == 60 { state = 104; lexer.advance(false); continue; }
                if lookahead == 61 { state = 75; lexer.advance(false); continue; }
                if lookahead == 62 { state = 109; lexer.advance(false); continue; }
                if lookahead == 64 { state = 78; lexer.advance(false); continue; }
                if lookahead == 91 { state = 76; lexer.advance(false); continue; }
                if lookahead == 92 { state = 11; lexer.advance(false); continue; }
                if lookahead == 93 { state = 77; lexer.advance(false); continue; }
                if lookahead == 123 { state = 85; lexer.advance(false); continue; }
                if lookahead == 124 { state = 83; lexer.advance(false); continue; }
                if lookahead == 125 { state = 86; lexer.advance(false); continue; }
                if lookahead == 126 { state = 101; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 || lookahead == 8203 || lookahead == 8288 || lookahead == 65279 { state = 53; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 137; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 147; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                result = true; lexer.set_result_symbol(ts_builtin_sym_end); lexer.mark_end();
                return result;
            }
            55 => {
                result = true; lexer.set_result_symbol(anon_sym_SEMI); lexer.mark_end();
                return result;
            }
            56 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                return result;
            }
            57 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                if lookahead == 46 { state = 17; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 143; lexer.advance(false); continue; }
                return result;
            }
            58 => {
                result = true; lexer.set_result_symbol(anon_sym_LPAREN); lexer.mark_end();
                return result;
            }
            59 => {
                result = true; lexer.set_result_symbol(anon_sym_RPAREN); lexer.mark_end();
                return result;
            }
            60 => {
                result = true; lexer.set_result_symbol(anon_sym_COMMA); lexer.mark_end();
                return result;
            }
            61 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                return result;
            }
            62 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                if lookahead == 42 { state = 73; lexer.advance(false); continue; }
                if lookahead == 61 { state = 115; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                if lookahead == 42 { state = 72; lexer.advance(false); continue; }
                return result;
            }
            64 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                if lookahead == 42 { state = 30; lexer.advance(false); continue; }
                if lookahead == 61 { state = 115; lexer.advance(false); continue; }
                return result;
            }
            65 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT); lexer.mark_end();
                return result;
            }
            66 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT); lexer.mark_end();
                if lookahead == 61 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            67 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON_EQ); lexer.mark_end();
                return result;
            }
            68 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON); lexer.mark_end();
                return result;
            }
            69 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON); lexer.mark_end();
                if lookahead == 61 { state = 67; lexer.advance(false); continue; }
                return result;
            }
            70 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR2); lexer.mark_end();
                return result;
            }
            71 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_GT); lexer.mark_end();
                return result;
            }
            72 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR_STAR); lexer.mark_end();
                return result;
            }
            73 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR_STAR); lexer.mark_end();
                if lookahead == 61 { state = 120; lexer.advance(false); continue; }
                return result;
            }
            74 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                return result;
            }
            75 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                if lookahead == 61 { state = 106; lexer.advance(false); continue; }
                return result;
            }
            76 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK); lexer.mark_end();
                return result;
            }
            77 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACK); lexer.mark_end();
                return result;
            }
            78 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                return result;
            }
            79 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                if lookahead == 61 { state = 117; lexer.advance(false); continue; }
                return result;
            }
            80 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                return result;
            }
            81 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 61 { state = 114; lexer.advance(false); continue; }
                return result;
            }
            82 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 61 { state = 114; lexer.advance(false); continue; }
                if lookahead == 62 { state = 71; lexer.advance(false); continue; }
                return result;
            }
            83 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                return result;
            }
            84 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                if lookahead == 61 { state = 125; lexer.advance(false); continue; }
                return result;
            }
            85 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACE); lexer.mark_end();
                return result;
            }
            86 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACE); lexer.mark_end();
                return result;
            }
            87 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                return result;
            }
            88 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                if lookahead == 61 { state = 113; lexer.advance(false); continue; }
                return result;
            }
            89 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH); lexer.mark_end();
                if lookahead == 47 { state = 94; lexer.advance(false); continue; }
                if lookahead == 61 { state = 116; lexer.advance(false); continue; }
                return result;
            }
            90 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH); lexer.mark_end();
                if lookahead == 47 { state = 93; lexer.advance(false); continue; }
                return result;
            }
            91 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT); lexer.mark_end();
                return result;
            }
            92 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT); lexer.mark_end();
                if lookahead == 61 { state = 119; lexer.advance(false); continue; }
                return result;
            }
            93 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_SLASH); lexer.mark_end();
                return result;
            }
            94 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_SLASH); lexer.mark_end();
                if lookahead == 61 { state = 118; lexer.advance(false); continue; }
                return result;
            }
            95 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP); lexer.mark_end();
                return result;
            }
            96 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP); lexer.mark_end();
                if lookahead == 61 { state = 123; lexer.advance(false); continue; }
                return result;
            }
            97 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET); lexer.mark_end();
                return result;
            }
            98 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET); lexer.mark_end();
                if lookahead == 61 { state = 124; lexer.advance(false); continue; }
                return result;
            }
            99 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT); lexer.mark_end();
                return result;
            }
            100 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT); lexer.mark_end();
                if lookahead == 61 { state = 122; lexer.advance(false); continue; }
                return result;
            }
            101 => {
                result = true; lexer.set_result_symbol(anon_sym_TILDE); lexer.mark_end();
                return result;
            }
            102 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 60 { state = 100; lexer.advance(false); continue; }
                if lookahead == 61 { state = 105; lexer.advance(false); continue; }
                if lookahead == 62 { state = 112; lexer.advance(false); continue; }
                return result;
            }
            103 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 60 { state = 99; lexer.advance(false); continue; }
                if lookahead == 61 { state = 105; lexer.advance(false); continue; }
                if lookahead == 62 { state = 112; lexer.advance(false); continue; }
                return result;
            }
            104 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 61 { state = 105; lexer.advance(false); continue; }
                if lookahead == 62 { state = 112; lexer.advance(false); continue; }
                return result;
            }
            105 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_EQ); lexer.mark_end();
                return result;
            }
            106 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ_EQ); lexer.mark_end();
                return result;
            }
            107 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG_EQ); lexer.mark_end();
                return result;
            }
            108 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_EQ); lexer.mark_end();
                return result;
            }
            109 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                if lookahead == 61 { state = 108; lexer.advance(false); continue; }
                return result;
            }
            110 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                if lookahead == 61 { state = 108; lexer.advance(false); continue; }
                if lookahead == 62 { state = 66; lexer.advance(false); continue; }
                return result;
            }
            111 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                if lookahead == 61 { state = 108; lexer.advance(false); continue; }
                if lookahead == 62 { state = 65; lexer.advance(false); continue; }
                return result;
            }
            112 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_GT); lexer.mark_end();
                return result;
            }
            113 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS_EQ); lexer.mark_end();
                return result;
            }
            114 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_EQ); lexer.mark_end();
                return result;
            }
            115 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR_EQ); lexer.mark_end();
                return result;
            }
            116 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_EQ); lexer.mark_end();
                return result;
            }
            117 => {
                result = true; lexer.set_result_symbol(anon_sym_AT_EQ); lexer.mark_end();
                return result;
            }
            118 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_SLASH_EQ); lexer.mark_end();
                return result;
            }
            119 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT_EQ); lexer.mark_end();
                return result;
            }
            120 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR_STAR_EQ); lexer.mark_end();
                return result;
            }
            121 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT_EQ); lexer.mark_end();
                return result;
            }
            122 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT_EQ); lexer.mark_end();
                return result;
            }
            123 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP_EQ); lexer.mark_end();
                return result;
            }
            124 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET_EQ); lexer.mark_end();
                return result;
            }
            125 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE_EQ); lexer.mark_end();
                return result;
            }
            126 => {
                result = true; lexer.set_result_symbol(sym_ellipsis); lexer.mark_end();
                return result;
            }
            127 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                return result;
            }
            128 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 { state = 127; lexer.advance(false); continue; }
                return result;
            }
            129 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 { state = 128; lexer.advance(false); continue; }
                return result;
            }
            130 => {
                result = true; lexer.set_result_symbol(anon_sym_BSLASH); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (0, 149), (10, 127), (13, 2), (78, 36), (85, 50), (117, 46), (120, 44), (34, 127),
                    (39, 127), (92, 127), (97, 127), (98, 127), (102, 127), (110, 127), (114, 127), (116, 127),
                    (118, 127),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 129; lexer.advance(false); continue; }
                return result;
            }
            131 => {
                result = true; lexer.set_result_symbol(aux_sym_format_specifier_token1); lexer.mark_end();
                if !eof && lookahead == 0 { state = 133; lexer.advance(false); continue; }
                if lookahead == 13 { state = 133; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 123 && lookahead != 125 { state = 133; lexer.advance(false); continue; }
                return result;
            }
            132 => {
                result = true; lexer.set_result_symbol(aux_sym_format_specifier_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (13, 132), (35, 133), (92, 131), (9, 132), (11, 132), (12, 132), (32, 132), (8203, 132),
                    (8288, 132), (65279, 132),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 123 && lookahead != 125 { state = 133; lexer.advance(false); continue; }
                return result;
            }
            133 => {
                result = true; lexer.set_result_symbol(aux_sym_format_specifier_token1); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 && lookahead != 123 && lookahead != 125 { state = 133; lexer.advance(false); continue; }
                return result;
            }
            134 => {
                result = true; lexer.set_result_symbol(sym_type_conversion); lexer.mark_end();
                return result;
            }
            135 => {
                result = true; lexer.set_result_symbol(sym_integer); lexer.mark_end();
                return result;
            }
            136 => {
                result = true; lexer.set_result_symbol(sym_integer); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (46, 145), (95, 138), (66, 33), (98, 33), (69, 38), (101, 38), (79, 34), (111, 34),
                    (88, 35), (120, 35), (74, 135), (76, 135), (106, 135), (108, 135),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 137; lexer.advance(false); continue; }
                return result;
            }
            137 => {
                result = true; lexer.set_result_symbol(sym_integer); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (46, 145), (95, 138), (69, 38), (101, 38), (74, 135), (76, 135), (106, 135), (108, 135),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 137; lexer.advance(false); continue; }
                return result;
            }
            138 => {
                result = true; lexer.set_result_symbol(sym_integer); lexer.mark_end();
                if lookahead == 46 { state = 145; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 38; lexer.advance(false); continue; }
                if lookahead == 74 || lookahead == 76 || lookahead == 106 || lookahead == 108 { state = 135; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 137; lexer.advance(false); continue; }
                return result;
            }
            139 => {
                result = true; lexer.set_result_symbol(sym_integer); lexer.mark_end();
                if lookahead == 95 { state = 39; lexer.advance(false); continue; }
                if lookahead == 76 || lookahead == 108 { state = 135; lexer.advance(false); continue; }
                if lookahead == 48 || lookahead == 49 { state = 139; lexer.advance(false); continue; }
                return result;
            }
            140 => {
                result = true; lexer.set_result_symbol(sym_integer); lexer.mark_end();
                if lookahead == 95 { state = 40; lexer.advance(false); continue; }
                if lookahead == 76 || lookahead == 108 { state = 135; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 55 { state = 140; lexer.advance(false); continue; }
                return result;
            }
            141 => {
                result = true; lexer.set_result_symbol(sym_integer); lexer.mark_end();
                if lookahead == 95 { state = 43; lexer.advance(false); continue; }
                if lookahead == 76 || lookahead == 108 { state = 135; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 141; lexer.advance(false); continue; }
                return result;
            }
            142 => {
                result = true; lexer.set_result_symbol(sym_float); lexer.mark_end();
                return result;
            }
            143 => {
                result = true; lexer.set_result_symbol(sym_float); lexer.mark_end();
                if lookahead == 95 { state = 145; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 38; lexer.advance(false); continue; }
                if lookahead == 74 || lookahead == 106 { state = 142; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 143; lexer.advance(false); continue; }
                return result;
            }
            144 => {
                result = true; lexer.set_result_symbol(sym_float); lexer.mark_end();
                if lookahead == 95 { state = 146; lexer.advance(false); continue; }
                if lookahead == 74 || lookahead == 106 { state = 142; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 144; lexer.advance(false); continue; }
                return result;
            }
            145 => {
                result = true; lexer.set_result_symbol(sym_float); lexer.mark_end();
                if lookahead == 69 || lookahead == 101 { state = 38; lexer.advance(false); continue; }
                if lookahead == 74 || lookahead == 106 { state = 142; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 143; lexer.advance(false); continue; }
                return result;
            }
            146 => {
                result = true; lexer.set_result_symbol(sym_float); lexer.mark_end();
                if lookahead == 74 || lookahead == 106 { state = 142; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 144; lexer.advance(false); continue; }
                return result;
            }
            147 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 147; lexer.advance(false); continue; }
                return result;
            }
            148 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 { state = 148; lexer.advance(false); continue; }
                return result;
            }
            149 => {
                result = true; lexer.set_result_symbol(sym_line_continuation); lexer.mark_end();
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
                if lookahead == 13 { state = 0; lexer.advance(true); continue; }
                if lookahead == 70 { state = 1; lexer.advance(false); continue; }
                if lookahead == 78 { state = 2; lexer.advance(false); continue; }
                if lookahead == 84 { state = 3; lexer.advance(false); continue; }
                if lookahead == 95 { state = 4; lexer.advance(false); continue; }
                if lookahead == 97 { state = 5; lexer.advance(false); continue; }
                if lookahead == 98 { state = 6; lexer.advance(false); continue; }
                if lookahead == 99 { state = 7; lexer.advance(false); continue; }
                if lookahead == 100 { state = 8; lexer.advance(false); continue; }
                if lookahead == 101 { state = 9; lexer.advance(false); continue; }
                if lookahead == 102 { state = 10; lexer.advance(false); continue; }
                if lookahead == 103 { state = 11; lexer.advance(false); continue; }
                if lookahead == 105 { state = 12; lexer.advance(false); continue; }
                if lookahead == 108 { state = 13; lexer.advance(false); continue; }
                if lookahead == 109 { state = 14; lexer.advance(false); continue; }
                if lookahead == 110 { state = 15; lexer.advance(false); continue; }
                if lookahead == 111 { state = 16; lexer.advance(false); continue; }
                if lookahead == 112 { state = 17; lexer.advance(false); continue; }
                if lookahead == 114 { state = 18; lexer.advance(false); continue; }
                if lookahead == 116 { state = 19; lexer.advance(false); continue; }
                if lookahead == 119 { state = 20; lexer.advance(false); continue; }
                if lookahead == 121 { state = 21; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 || lookahead == 8203 || lookahead == 8288 || lookahead == 65279 { state = 0; lexer.advance(true); continue; }
                return result;
            }
            1 => {
                if lookahead == 97 { state = 22; lexer.advance(false); continue; }
                return result;
            }
            2 => {
                if lookahead == 111 { state = 23; lexer.advance(false); continue; }
                return result;
            }
            3 => {
                if lookahead == 114 { state = 24; lexer.advance(false); continue; }
                return result;
            }
            4 => {
                result = true; lexer.set_result_symbol(anon_sym__); lexer.mark_end();
                if lookahead == 95 { state = 25; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                if lookahead == 110 { state = 26; lexer.advance(false); continue; }
                if lookahead == 115 { state = 27; lexer.advance(false); continue; }
                if lookahead == 119 { state = 28; lexer.advance(false); continue; }
                return result;
            }
            6 => {
                if lookahead == 114 { state = 29; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if lookahead == 97 { state = 30; lexer.advance(false); continue; }
                if lookahead == 108 { state = 31; lexer.advance(false); continue; }
                if lookahead == 111 { state = 32; lexer.advance(false); continue; }
                return result;
            }
            8 => {
                if lookahead == 101 { state = 33; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if lookahead == 108 { state = 34; lexer.advance(false); continue; }
                if lookahead == 120 { state = 35; lexer.advance(false); continue; }
                return result;
            }
            10 => {
                if lookahead == 105 { state = 36; lexer.advance(false); continue; }
                if lookahead == 111 { state = 37; lexer.advance(false); continue; }
                if lookahead == 114 { state = 38; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if lookahead == 108 { state = 39; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if lookahead == 102 { state = 40; lexer.advance(false); continue; }
                if lookahead == 109 { state = 41; lexer.advance(false); continue; }
                if lookahead == 110 { state = 42; lexer.advance(false); continue; }
                if lookahead == 115 { state = 43; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if lookahead == 97 { state = 44; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if lookahead == 97 { state = 45; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if lookahead == 111 { state = 46; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if lookahead == 114 { state = 47; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if lookahead == 97 { state = 48; lexer.advance(false); continue; }
                if lookahead == 114 { state = 49; lexer.advance(false); continue; }
                return result;
            }
            18 => {
                if lookahead == 97 { state = 50; lexer.advance(false); continue; }
                if lookahead == 101 { state = 51; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if lookahead == 114 { state = 52; lexer.advance(false); continue; }
                if lookahead == 121 { state = 53; lexer.advance(false); continue; }
                return result;
            }
            20 => {
                if lookahead == 104 { state = 54; lexer.advance(false); continue; }
                if lookahead == 105 { state = 55; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                if lookahead == 105 { state = 56; lexer.advance(false); continue; }
                return result;
            }
            22 => {
                if lookahead == 108 { state = 57; lexer.advance(false); continue; }
                return result;
            }
            23 => {
                if lookahead == 110 { state = 58; lexer.advance(false); continue; }
                return result;
            }
            24 => {
                if lookahead == 117 { state = 59; lexer.advance(false); continue; }
                return result;
            }
            25 => {
                if lookahead == 102 { state = 60; lexer.advance(false); continue; }
                return result;
            }
            26 => {
                if lookahead == 100 { state = 61; lexer.advance(false); continue; }
                return result;
            }
            27 => {
                result = true; lexer.set_result_symbol(anon_sym_as); lexer.mark_end();
                if lookahead == 115 { state = 62; lexer.advance(false); continue; }
                if lookahead == 121 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            28 => {
                if lookahead == 97 { state = 64; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                if lookahead == 101 { state = 65; lexer.advance(false); continue; }
                return result;
            }
            30 => {
                if lookahead == 115 { state = 66; lexer.advance(false); continue; }
                return result;
            }
            31 => {
                if lookahead == 97 { state = 67; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                if lookahead == 110 { state = 68; lexer.advance(false); continue; }
                return result;
            }
            33 => {
                if lookahead == 102 { state = 69; lexer.advance(false); continue; }
                if lookahead == 108 { state = 70; lexer.advance(false); continue; }
                return result;
            }
            34 => {
                if lookahead == 105 { state = 71; lexer.advance(false); continue; }
                if lookahead == 115 { state = 72; lexer.advance(false); continue; }
                return result;
            }
            35 => {
                if lookahead == 99 { state = 73; lexer.advance(false); continue; }
                if lookahead == 101 { state = 74; lexer.advance(false); continue; }
                return result;
            }
            36 => {
                if lookahead == 110 { state = 75; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                if lookahead == 114 { state = 76; lexer.advance(false); continue; }
                return result;
            }
            38 => {
                if lookahead == 111 { state = 77; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                if lookahead == 111 { state = 78; lexer.advance(false); continue; }
                return result;
            }
            40 => {
                result = true; lexer.set_result_symbol(anon_sym_if); lexer.mark_end();
                return result;
            }
            41 => {
                if lookahead == 112 { state = 79; lexer.advance(false); continue; }
                return result;
            }
            42 => {
                result = true; lexer.set_result_symbol(anon_sym_in); lexer.mark_end();
                return result;
            }
            43 => {
                result = true; lexer.set_result_symbol(anon_sym_is); lexer.mark_end();
                return result;
            }
            44 => {
                if lookahead == 109 { state = 80; lexer.advance(false); continue; }
                return result;
            }
            45 => {
                if lookahead == 116 { state = 81; lexer.advance(false); continue; }
                return result;
            }
            46 => {
                if lookahead == 110 { state = 82; lexer.advance(false); continue; }
                if lookahead == 116 { state = 83; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                result = true; lexer.set_result_symbol(anon_sym_or); lexer.mark_end();
                return result;
            }
            48 => {
                if lookahead == 115 { state = 84; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                if lookahead == 105 { state = 85; lexer.advance(false); continue; }
                return result;
            }
            50 => {
                if lookahead == 105 { state = 86; lexer.advance(false); continue; }
                return result;
            }
            51 => {
                if lookahead == 116 { state = 87; lexer.advance(false); continue; }
                return result;
            }
            52 => {
                if lookahead == 121 { state = 88; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                if lookahead == 112 { state = 89; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                if lookahead == 105 { state = 90; lexer.advance(false); continue; }
                return result;
            }
            55 => {
                if lookahead == 116 { state = 91; lexer.advance(false); continue; }
                return result;
            }
            56 => {
                if lookahead == 101 { state = 92; lexer.advance(false); continue; }
                return result;
            }
            57 => {
                if lookahead == 115 { state = 93; lexer.advance(false); continue; }
                return result;
            }
            58 => {
                if lookahead == 101 { state = 94; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                if lookahead == 101 { state = 95; lexer.advance(false); continue; }
                return result;
            }
            60 => {
                if lookahead == 117 { state = 96; lexer.advance(false); continue; }
                return result;
            }
            61 => {
                result = true; lexer.set_result_symbol(anon_sym_and); lexer.mark_end();
                return result;
            }
            62 => {
                if lookahead == 101 { state = 97; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                if lookahead == 110 { state = 98; lexer.advance(false); continue; }
                return result;
            }
            64 => {
                if lookahead == 105 { state = 99; lexer.advance(false); continue; }
                return result;
            }
            65 => {
                if lookahead == 97 { state = 100; lexer.advance(false); continue; }
                return result;
            }
            66 => {
                if lookahead == 101 { state = 101; lexer.advance(false); continue; }
                return result;
            }
            67 => {
                if lookahead == 115 { state = 102; lexer.advance(false); continue; }
                return result;
            }
            68 => {
                if lookahead == 116 { state = 103; lexer.advance(false); continue; }
                return result;
            }
            69 => {
                result = true; lexer.set_result_symbol(anon_sym_def); lexer.mark_end();
                return result;
            }
            70 => {
                result = true; lexer.set_result_symbol(anon_sym_del); lexer.mark_end();
                return result;
            }
            71 => {
                if lookahead == 102 { state = 104; lexer.advance(false); continue; }
                return result;
            }
            72 => {
                if lookahead == 101 { state = 105; lexer.advance(false); continue; }
                return result;
            }
            73 => {
                if lookahead == 101 { state = 106; lexer.advance(false); continue; }
                return result;
            }
            74 => {
                if lookahead == 99 { state = 107; lexer.advance(false); continue; }
                return result;
            }
            75 => {
                if lookahead == 97 { state = 108; lexer.advance(false); continue; }
                return result;
            }
            76 => {
                result = true; lexer.set_result_symbol(anon_sym_for); lexer.mark_end();
                return result;
            }
            77 => {
                if lookahead == 109 { state = 109; lexer.advance(false); continue; }
                return result;
            }
            78 => {
                if lookahead == 98 { state = 110; lexer.advance(false); continue; }
                return result;
            }
            79 => {
                if lookahead == 111 { state = 111; lexer.advance(false); continue; }
                return result;
            }
            80 => {
                if lookahead == 98 { state = 112; lexer.advance(false); continue; }
                return result;
            }
            81 => {
                if lookahead == 99 { state = 113; lexer.advance(false); continue; }
                return result;
            }
            82 => {
                if lookahead == 108 { state = 114; lexer.advance(false); continue; }
                return result;
            }
            83 => {
                result = true; lexer.set_result_symbol(anon_sym_not); lexer.mark_end();
                return result;
            }
            84 => {
                if lookahead == 115 { state = 115; lexer.advance(false); continue; }
                return result;
            }
            85 => {
                if lookahead == 110 { state = 116; lexer.advance(false); continue; }
                return result;
            }
            86 => {
                if lookahead == 115 { state = 117; lexer.advance(false); continue; }
                return result;
            }
            87 => {
                if lookahead == 117 { state = 118; lexer.advance(false); continue; }
                return result;
            }
            88 => {
                result = true; lexer.set_result_symbol(anon_sym_try); lexer.mark_end();
                return result;
            }
            89 => {
                if lookahead == 101 { state = 119; lexer.advance(false); continue; }
                return result;
            }
            90 => {
                if lookahead == 108 { state = 120; lexer.advance(false); continue; }
                return result;
            }
            91 => {
                if lookahead == 104 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            92 => {
                if lookahead == 108 { state = 122; lexer.advance(false); continue; }
                return result;
            }
            93 => {
                if lookahead == 101 { state = 123; lexer.advance(false); continue; }
                return result;
            }
            94 => {
                result = true; lexer.set_result_symbol(sym_none); lexer.mark_end();
                return result;
            }
            95 => {
                result = true; lexer.set_result_symbol(sym_true); lexer.mark_end();
                return result;
            }
            96 => {
                if lookahead == 116 { state = 124; lexer.advance(false); continue; }
                return result;
            }
            97 => {
                if lookahead == 114 { state = 125; lexer.advance(false); continue; }
                return result;
            }
            98 => {
                if lookahead == 99 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            99 => {
                if lookahead == 116 { state = 127; lexer.advance(false); continue; }
                return result;
            }
            100 => {
                if lookahead == 107 { state = 128; lexer.advance(false); continue; }
                return result;
            }
            101 => {
                result = true; lexer.set_result_symbol(anon_sym_case); lexer.mark_end();
                return result;
            }
            102 => {
                if lookahead == 115 { state = 129; lexer.advance(false); continue; }
                return result;
            }
            103 => {
                if lookahead == 105 { state = 130; lexer.advance(false); continue; }
                return result;
            }
            104 => {
                result = true; lexer.set_result_symbol(anon_sym_elif); lexer.mark_end();
                return result;
            }
            105 => {
                result = true; lexer.set_result_symbol(anon_sym_else); lexer.mark_end();
                return result;
            }
            106 => {
                if lookahead == 112 { state = 131; lexer.advance(false); continue; }
                return result;
            }
            107 => {
                result = true; lexer.set_result_symbol(anon_sym_exec); lexer.mark_end();
                return result;
            }
            108 => {
                if lookahead == 108 { state = 132; lexer.advance(false); continue; }
                return result;
            }
            109 => {
                result = true; lexer.set_result_symbol(anon_sym_from); lexer.mark_end();
                return result;
            }
            110 => {
                if lookahead == 97 { state = 133; lexer.advance(false); continue; }
                return result;
            }
            111 => {
                if lookahead == 114 { state = 134; lexer.advance(false); continue; }
                return result;
            }
            112 => {
                if lookahead == 100 { state = 135; lexer.advance(false); continue; }
                return result;
            }
            113 => {
                if lookahead == 104 { state = 136; lexer.advance(false); continue; }
                return result;
            }
            114 => {
                if lookahead == 111 { state = 137; lexer.advance(false); continue; }
                return result;
            }
            115 => {
                result = true; lexer.set_result_symbol(anon_sym_pass); lexer.mark_end();
                return result;
            }
            116 => {
                if lookahead == 116 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            117 => {
                if lookahead == 101 { state = 139; lexer.advance(false); continue; }
                return result;
            }
            118 => {
                if lookahead == 114 { state = 140; lexer.advance(false); continue; }
                return result;
            }
            119 => {
                result = true; lexer.set_result_symbol(anon_sym_type); lexer.mark_end();
                return result;
            }
            120 => {
                if lookahead == 101 { state = 141; lexer.advance(false); continue; }
                return result;
            }
            121 => {
                result = true; lexer.set_result_symbol(anon_sym_with); lexer.mark_end();
                return result;
            }
            122 => {
                if lookahead == 100 { state = 142; lexer.advance(false); continue; }
                return result;
            }
            123 => {
                result = true; lexer.set_result_symbol(sym_false); lexer.mark_end();
                return result;
            }
            124 => {
                if lookahead == 117 { state = 143; lexer.advance(false); continue; }
                return result;
            }
            125 => {
                if lookahead == 116 { state = 144; lexer.advance(false); continue; }
                return result;
            }
            126 => {
                result = true; lexer.set_result_symbol(anon_sym_async); lexer.mark_end();
                return result;
            }
            127 => {
                result = true; lexer.set_result_symbol(anon_sym_await); lexer.mark_end();
                return result;
            }
            128 => {
                result = true; lexer.set_result_symbol(anon_sym_break); lexer.mark_end();
                return result;
            }
            129 => {
                result = true; lexer.set_result_symbol(anon_sym_class); lexer.mark_end();
                return result;
            }
            130 => {
                if lookahead == 110 { state = 145; lexer.advance(false); continue; }
                return result;
            }
            131 => {
                if lookahead == 116 { state = 146; lexer.advance(false); continue; }
                return result;
            }
            132 => {
                if lookahead == 108 { state = 147; lexer.advance(false); continue; }
                return result;
            }
            133 => {
                if lookahead == 108 { state = 148; lexer.advance(false); continue; }
                return result;
            }
            134 => {
                if lookahead == 116 { state = 149; lexer.advance(false); continue; }
                return result;
            }
            135 => {
                if lookahead == 97 { state = 150; lexer.advance(false); continue; }
                return result;
            }
            136 => {
                result = true; lexer.set_result_symbol(anon_sym_match); lexer.mark_end();
                return result;
            }
            137 => {
                if lookahead == 99 { state = 151; lexer.advance(false); continue; }
                return result;
            }
            138 => {
                result = true; lexer.set_result_symbol(anon_sym_print); lexer.mark_end();
                return result;
            }
            139 => {
                result = true; lexer.set_result_symbol(anon_sym_raise); lexer.mark_end();
                return result;
            }
            140 => {
                if lookahead == 110 { state = 152; lexer.advance(false); continue; }
                return result;
            }
            141 => {
                result = true; lexer.set_result_symbol(anon_sym_while); lexer.mark_end();
                return result;
            }
            142 => {
                result = true; lexer.set_result_symbol(anon_sym_yield); lexer.mark_end();
                return result;
            }
            143 => {
                if lookahead == 114 { state = 153; lexer.advance(false); continue; }
                return result;
            }
            144 => {
                result = true; lexer.set_result_symbol(anon_sym_assert); lexer.mark_end();
                return result;
            }
            145 => {
                if lookahead == 117 { state = 154; lexer.advance(false); continue; }
                return result;
            }
            146 => {
                result = true; lexer.set_result_symbol(anon_sym_except); lexer.mark_end();
                return result;
            }
            147 => {
                if lookahead == 121 { state = 155; lexer.advance(false); continue; }
                return result;
            }
            148 => {
                result = true; lexer.set_result_symbol(anon_sym_global); lexer.mark_end();
                return result;
            }
            149 => {
                result = true; lexer.set_result_symbol(anon_sym_import); lexer.mark_end();
                return result;
            }
            150 => {
                result = true; lexer.set_result_symbol(anon_sym_lambda); lexer.mark_end();
                return result;
            }
            151 => {
                if lookahead == 97 { state = 156; lexer.advance(false); continue; }
                return result;
            }
            152 => {
                result = true; lexer.set_result_symbol(anon_sym_return); lexer.mark_end();
                return result;
            }
            153 => {
                if lookahead == 101 { state = 157; lexer.advance(false); continue; }
                return result;
            }
            154 => {
                if lookahead == 101 { state = 158; lexer.advance(false); continue; }
                return result;
            }
            155 => {
                result = true; lexer.set_result_symbol(anon_sym_finally); lexer.mark_end();
                return result;
            }
            156 => {
                if lookahead == 108 { state = 159; lexer.advance(false); continue; }
                return result;
            }
            157 => {
                if lookahead == 95 { state = 160; lexer.advance(false); continue; }
                return result;
            }
            158 => {
                result = true; lexer.set_result_symbol(anon_sym_continue); lexer.mark_end();
                return result;
            }
            159 => {
                result = true; lexer.set_result_symbol(anon_sym_nonlocal); lexer.mark_end();
                return result;
            }
            160 => {
                if lookahead == 95 { state = 161; lexer.advance(false); continue; }
                return result;
            }
            161 => {
                result = true; lexer.set_result_symbol(anon_sym___future__); lexer.mark_end();
                return result;
            }
            _ => return false,
        }
    }
}
