//! The `objc` grammar's lexer: `ts_lex` and `ts_lex_keywords`, transliterated from
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

const anon_sym_AMP: Symbol = 34;
const anon_sym_AMP_AMP: Symbol = 31;
const anon_sym_AMP_EQ: Symbol = 157;
const anon_sym_API_AVAILABLE: Symbol = 225;
const anon_sym_API_DEPRECATED: Symbol = 227;
const anon_sym_API_UNAVAILABLE: Symbol = 226;
const anon_sym_AT: Symbol = 184;
const anon_sym_ATautoreleasepool: Symbol = 75;
const anon_sym_ATavailable: Symbol = 268;
const anon_sym_ATcatch: Symbol = 261;
const anon_sym_ATcompatibility_alias: Symbol = 252;
const anon_sym_ATdefs: Symbol = 279;
const anon_sym_ATdynamic: Symbol = 256;
const anon_sym_ATencode: Symbol = 272;
const anon_sym_ATend: Symbol = 243;
const anon_sym_ATfinally: Symbol = 263;
const anon_sym_ATimplementation: Symbol = 245;
const anon_sym_ATimport: Symbol = 209;
const anon_sym_ATinterface: Symbol = 244;
const anon_sym_AToptional: Symbol = 253;
const anon_sym_ATpackage: Symbol = 250;
const anon_sym_ATprivate: Symbol = 248;
const anon_sym_ATproperty: Symbol = 258;
const anon_sym_ATprotected: Symbol = 249;
const anon_sym_ATprotocol: Symbol = 241;
const anon_sym_ATpublic: Symbol = 251;
const anon_sym_ATrequired: Symbol = 254;
const anon_sym_ATselector: Symbol = 266;
const anon_sym_ATsynchronized: Symbol = 273;
const anon_sym_ATsynthesize: Symbol = 255;
const anon_sym_ATthrow: Symbol = 265;
const anon_sym_ATtry: Symbol = 259;
const anon_sym_BANG: Symbol = 23;
const anon_sym_BANG_EQ: Symbol = 36;
const anon_sym_BOOL: Symbol = 274;
const anon_sym_CARET: Symbol = 33;
const anon_sym_CARET_EQ: Symbol = 158;
const anon_sym_CF_FORMAT_FUNCTION: Symbol = 221;
const anon_sym_CF_RETURNS_NOT_RETAINED: Symbol = 217;
const anon_sym_CF_RETURNS_RETAINED: Symbol = 216;
const anon_sym_CG_EXTERN: Symbol = 84;
const anon_sym_CG_INLINE: Symbol = 85;
const anon_sym_COLON: Symbol = 133;
const anon_sym_COLON2: Symbol = 285;
const anon_sym_COLON_COLON: Symbol = 51;
const anon_sym_COMMA: Symbol = 8;
const anon_sym_Class: Symbol = 277;
const anon_sym_DASH: Symbol = 25;
const anon_sym_DASH_DASH: Symbol = 160;
const anon_sym_DASH_EQ: Symbol = 154;
const anon_sym_DASH_GT: Symbol = 176;
const anon_sym_DEPRECATED_ATTRIBUTE: Symbol = 218;
const anon_sym_DEPRECATED_MSG_ATTRIBUTE: Symbol = 232;
const anon_sym_DOT: Symbol = 175;
const anon_sym_DOT_DOT_DOT: Symbol = 7;
const anon_sym_DQUOTE: Symbol = 185;
const anon_sym_EQ: Symbol = 74;
const anon_sym_EQ_EQ: Symbol = 35;
const anon_sym_FOUNDATION_EXPORT: Symbol = 86;
const anon_sym_FOUNDATION_EXTERN: Symbol = 87;
const anon_sym_FOUNDATION_STATIC_INLINE: Symbol = 88;
const anon_sym_GT: Symbol = 37;
const anon_sym_GT_EQ: Symbol = 38;
const anon_sym_GT_GT: Symbol = 42;
const anon_sym_GT_GT_EQ: Symbol = 156;
const anon_sym_IBInspectable: Symbol = 90;
const anon_sym_IBOutlet: Symbol = 89;
const anon_sym_IB_DESIGNABLE: Symbol = 91;
const anon_sym_IMP: Symbol = 275;
const anon_sym_LBRACE: Symbol = 67;
const anon_sym_LBRACK: Symbol = 52;
const anon_sym_LF: Symbol = 11;
const anon_sym_LPAREN: Symbol = 6;
const anon_sym_LPAREN2: Symbol = 21;
const anon_sym_LPARENclass_RPAREN: Symbol = 257;
const anon_sym_LT: Symbol = 40;
const anon_sym_LT2: Symbol = 174;
const anon_sym_LT_EQ: Symbol = 39;
const anon_sym_LT_LT: Symbol = 41;
const anon_sym_LT_LT_EQ: Symbol = 155;
const anon_sym_L_DQUOTE: Symbol = 186;
const anon_sym_L_SQUOTE: Symbol = 178;
const anon_sym_NS_AUTOMATED_REFCOUNT_UNAVAILABLE: Symbol = 212;
const anon_sym_NS_AVAILABLE: Symbol = 222;
const anon_sym_NS_AVAILABLE_IOS: Symbol = 224;
const anon_sym_NS_CLASS_AVAILABLE_IOS: Symbol = 238;
const anon_sym_NS_CLASS_DEPRECATED_IOS: Symbol = 239;
const anon_sym_NS_DEPRECATED_IOS: Symbol = 229;
const anon_sym_NS_ENUM_AVAILABLE_IOS: Symbol = 228;
const anon_sym_NS_ENUM_DEPRECATED_IOS: Symbol = 230;
const anon_sym_NS_EXTENSION_UNAVAILABLE_IOS: Symbol = 237;
const anon_sym_NS_FORMAT_FUNCTION: Symbol = 231;
const anon_sym_NS_INLINE: Symbol = 92;
const anon_sym_NS_REQUIRES_NIL_TERMINATION: Symbol = 215;
const anon_sym_NS_ROOT_CLASS: Symbol = 213;
const anon_sym_NS_SWIFT_NAME: Symbol = 235;
const anon_sym_NS_SWIFT_UNAVAILABLE: Symbol = 236;
const anon_sym_NS_UNAVAILABLE: Symbol = 214;
const anon_sym_NS_VALID_UNTIL_END_OF_SCOPE: Symbol = 93;
const anon_sym_NULL: Symbol = 195;
const anon_sym_OBJC_EXPORT: Symbol = 94;
const anon_sym_OBJC_ROOT_CLASS: Symbol = 95;
const anon_sym_PERCENT: Symbol = 29;
const anon_sym_PERCENT_EQ: Symbol = 152;
const anon_sym_PIPE: Symbol = 32;
const anon_sym_PIPE_EQ: Symbol = 159;
const anon_sym_PIPE_PIPE: Symbol = 30;
const anon_sym_PLUS: Symbol = 26;
const anon_sym_PLUS_EQ: Symbol = 153;
const anon_sym_PLUS_PLUS: Symbol = 161;
const anon_sym_POUND: Symbol = 211;
const anon_sym_QMARK: Symbol = 149;
const anon_sym_RBRACE: Symbol = 68;
const anon_sym_RBRACK: Symbol = 53;
const anon_sym_RPAREN: Symbol = 9;
const anon_sym_SEL: Symbol = 276;
const anon_sym_SEMI: Symbol = 43;
const anon_sym_SLASH: Symbol = 28;
const anon_sym_SLASH_EQ: Symbol = 151;
const anon_sym_SQUOTE: Symbol = 182;
const anon_sym_STAR: Symbol = 27;
const anon_sym_STAR_EQ: Symbol = 150;
const anon_sym_TILDE: Symbol = 24;
const anon_sym_UIKIT_EXTERN: Symbol = 96;
const anon_sym_UI_APPEARANCE_SELECTOR: Symbol = 219;
const anon_sym_UNAVAILABLE_ATTRIBUTE: Symbol = 220;
const anon_sym_U_DQUOTE: Symbol = 188;
const anon_sym_U_SQUOTE: Symbol = 180;
const anon_sym__Alignas: Symbol = 130;
const anon_sym__Alignof: Symbol = 167;
const anon_sym__Atomic: Symbol = 102;
const anon_sym__Complex: Symbol = 106;
const anon_sym__Generic: Symbol = 169;
const anon_sym__Nonnull: Symbol = 104;
const anon_sym__Noreturn: Symbol = 103;
const anon_sym__Null_unspecified: Symbol = 109;
const anon_sym__Nullable: Symbol = 107;
const anon_sym__Nullable_result: Symbol = 108;
const anon_sym___IOS_AVAILABLE: Symbol = 223;
const anon_sym___OSX_AVAILABLE_STARTING: Symbol = 240;
const anon_sym___alignof: Symbol = 164;
const anon_sym___alignof__: Symbol = 163;
const anon_sym___asm: Symbol = 172;
const anon_sym___asm__: Symbol = 171;
const anon_sym___attribute: Symbol = 48;
const anon_sym___attribute__: Symbol = 47;
const anon_sym___autoreleasing: Symbol = 110;
const anon_sym___based: Symbol = 55;
const anon_sym___block: Symbol = 111;
const anon_sym___bridge: Symbol = 112;
const anon_sym___bridge_retained: Symbol = 113;
const anon_sym___bridge_transfer: Symbol = 114;
const anon_sym___builtin_available: Symbol = 269;
const anon_sym___catch: Symbol = 262;
const anon_sym___cdecl: Symbol = 56;
const anon_sym___clrcall: Symbol = 57;
const anon_sym___complex: Symbol = 115;
const anon_sym___const: Symbol = 116;
const anon_sym___contravariant: Symbol = 247;
const anon_sym___covariant: Symbol = 246;
const anon_sym___declspec: Symbol = 54;
const anon_sym___deprecated_enum_msg: Symbol = 234;
const anon_sym___deprecated_msg: Symbol = 233;
const anon_sym___extension__: Symbol = 44;
const anon_sym___fastcall: Symbol = 59;
const anon_sym___finally: Symbol = 264;
const anon_sym___forceinline: Symbol = 81;
const anon_sym___imag: Symbol = 117;
const anon_sym___inline: Symbol = 79;
const anon_sym___inline__: Symbol = 80;
const anon_sym___kindof: Symbol = 118;
const anon_sym___nonnull: Symbol = 119;
const anon_sym___nullable: Symbol = 120;
const anon_sym___ptrauth_objc_class_ro: Symbol = 121;
const anon_sym___ptrauth_objc_isa_pointer: Symbol = 122;
const anon_sym___ptrauth_objc_super_pointer: Symbol = 123;
const anon_sym___real: Symbol = 124;
const anon_sym___restrict__: Symbol = 101;
const anon_sym___stdcall: Symbol = 58;
const anon_sym___strong: Symbol = 125;
const anon_sym___thiscall: Symbol = 60;
const anon_sym___thread: Symbol = 83;
const anon_sym___try: Symbol = 260;
const anon_sym___typeof: Symbol = 200;
const anon_sym___typeof__: Symbol = 199;
const anon_sym___unaligned: Symbol = 66;
const anon_sym___unsafe_unretained: Symbol = 126;
const anon_sym___unused: Symbol = 127;
const anon_sym___vectorcall: Symbol = 61;
const anon_sym___volatile__: Symbol = 173;
const anon_sym___weak: Symbol = 128;
const anon_sym__alignof: Symbol = 165;
const anon_sym__unaligned: Symbol = 65;
const anon_sym_alignas: Symbol = 129;
const anon_sym_alignof: Symbol = 166;
const anon_sym_asm: Symbol = 170;
const anon_sym_auto: Symbol = 76;
const anon_sym_availability: Symbol = 202;
const anon_sym_break: Symbol = 146;
const anon_sym_bycopy: Symbol = 282;
const anon_sym_byref: Symbol = 283;
const anon_sym_case: Symbol = 139;
const anon_sym_class: Symbol = 242;
const anon_sym_const: Symbol = 97;
const anon_sym_constexpr: Symbol = 98;
const anon_sym_continue: Symbol = 147;
const anon_sym_default: Symbol = 140;
const anon_sym_defined: Symbol = 22;
const anon_sym_do: Symbol = 142;
const anon_sym_else: Symbol = 137;
const anon_sym_enum: Symbol = 132;
const anon_sym_extern: Symbol = 46;
const anon_sym_for: Symbol = 143;
const anon_sym_goto: Symbol = 148;
const anon_sym_id: Symbol = 278;
const anon_sym_if: Symbol = 136;
const anon_sym_in: Symbol = 144;
const anon_sym_inline: Symbol = 78;
const anon_sym_inout: Symbol = 281;
const anon_sym_ios: Symbol = 204;
const anon_sym_long: Symbol = 71;
const anon_sym_macos: Symbol = 206;
const anon_sym_macosx: Symbol = 207;
const anon_sym_noreturn: Symbol = 49;
const anon_sym_nothrow: Symbol = 50;
const anon_sym_nullable: Symbol = 105;
const anon_sym_nullptr: Symbol = 196;
const anon_sym_objc_bridge_related: Symbol = 198;
const anon_sym_offsetof: Symbol = 168;
const anon_sym_oneway: Symbol = 284;
const anon_sym_out: Symbol = 280;
const anon_sym_register: Symbol = 77;
const anon_sym_restrict: Symbol = 100;
const anon_sym_return: Symbol = 145;
const anon_sym_short: Symbol = 72;
const anon_sym_signed: Symbol = 69;
const anon_sym_sizeof: Symbol = 162;
const anon_sym_static: Symbol = 73;
const anon_sym_struct: Symbol = 134;
const anon_sym_switch: Symbol = 138;
const anon_sym_thread_local: Symbol = 82;
const anon_sym_tvos: Symbol = 205;
const anon_sym_typedef: Symbol = 45;
const anon_sym_typeof: Symbol = 201;
const anon_sym_u8_DQUOTE: Symbol = 189;
const anon_sym_u8_SQUOTE: Symbol = 181;
const anon_sym_u_DQUOTE: Symbol = 187;
const anon_sym_u_SQUOTE: Symbol = 179;
const anon_sym_union: Symbol = 135;
const anon_sym_unsigned: Symbol = 70;
const anon_sym_va_arg: Symbol = 270;
const anon_sym_volatile: Symbol = 99;
const anon_sym_watchos: Symbol = 208;
const anon_sym_while: Symbol = 141;
const aux_sym_char_literal_token1: Symbol = 183;
const aux_sym_ms_asm_block_token1: Symbol = 271;
const aux_sym_preproc_def_token1: Symbol = 5;
const aux_sym_preproc_elif_token1: Symbol = 16;
const aux_sym_preproc_elifdef_token1: Symbol = 17;
const aux_sym_preproc_elifdef_token2: Symbol = 18;
const aux_sym_preproc_else_token1: Symbol = 15;
const aux_sym_preproc_if_token1: Symbol = 10;
const aux_sym_preproc_if_token2: Symbol = 12;
const aux_sym_preproc_ifdef_token1: Symbol = 13;
const aux_sym_preproc_ifdef_token2: Symbol = 14;
const aux_sym_preproc_include_token1: Symbol = 2;
const aux_sym_preproc_include_token2: Symbol = 3;
const aux_sym_preproc_include_token3: Symbol = 4;
const aux_sym_selector_expression_token1: Symbol = 267;
const aux_sym_string_literal_token1: Symbol = 190;
const sym_comment: Symbol = 197;
const sym_escape_sequence: Symbol = 191;
const sym_false: Symbol = 194;
const sym_identifier: Symbol = 1;
const sym_ms_restrict_modifier: Symbol = 62;
const sym_ms_signed_ptr_modifier: Symbol = 64;
const sym_ms_unsigned_ptr_modifier: Symbol = 63;
const sym_number_literal: Symbol = 177;
const sym_preproc_arg: Symbol = 19;
const sym_preproc_directive: Symbol = 20;
const sym_primitive_type: Symbol = 131;
const sym_system_lib_string: Symbol = 192;
const sym_true: Symbol = 193;
const sym_version_number: Symbol = 203;
const ts_builtin_sym_end: Symbol = 0;

#[rustfmt::skip]
static sym_identifier_character_set_1: [CharacterRange; 670] = [
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
    CharacterRange::new(7086, 7087), CharacterRange::new(7098, 7141), CharacterRange::new(7168, 7203), CharacterRange::new(7245, 7247), CharacterRange::new(7258, 7293), CharacterRange::new(7296, 7304),
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
    CharacterRange::new(42623, 42653), CharacterRange::new(42656, 42735), CharacterRange::new(42775, 42783), CharacterRange::new(42786, 42888), CharacterRange::new(42891, 42954), CharacterRange::new(42960, 42961),
    CharacterRange::new(42963, 42963), CharacterRange::new(42965, 42969), CharacterRange::new(42994, 43009), CharacterRange::new(43011, 43013), CharacterRange::new(43015, 43018), CharacterRange::new(43020, 43042),
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
    CharacterRange::new(66967, 66977), CharacterRange::new(66979, 66993), CharacterRange::new(66995, 67001), CharacterRange::new(67003, 67004), CharacterRange::new(67072, 67382), CharacterRange::new(67392, 67413),
    CharacterRange::new(67424, 67431), CharacterRange::new(67456, 67461), CharacterRange::new(67463, 67504), CharacterRange::new(67506, 67514), CharacterRange::new(67584, 67589), CharacterRange::new(67592, 67592),
    CharacterRange::new(67594, 67637), CharacterRange::new(67639, 67640), CharacterRange::new(67644, 67644), CharacterRange::new(67647, 67669), CharacterRange::new(67680, 67702), CharacterRange::new(67712, 67742),
    CharacterRange::new(67808, 67826), CharacterRange::new(67828, 67829), CharacterRange::new(67840, 67861), CharacterRange::new(67872, 67897), CharacterRange::new(67968, 68023), CharacterRange::new(68030, 68031),
    CharacterRange::new(68096, 68096), CharacterRange::new(68112, 68115), CharacterRange::new(68117, 68119), CharacterRange::new(68121, 68149), CharacterRange::new(68192, 68220), CharacterRange::new(68224, 68252),
    CharacterRange::new(68288, 68295), CharacterRange::new(68297, 68324), CharacterRange::new(68352, 68405), CharacterRange::new(68416, 68437), CharacterRange::new(68448, 68466), CharacterRange::new(68480, 68497),
    CharacterRange::new(68608, 68680), CharacterRange::new(68736, 68786), CharacterRange::new(68800, 68850), CharacterRange::new(68864, 68899), CharacterRange::new(69248, 69289), CharacterRange::new(69296, 69297),
    CharacterRange::new(69376, 69404), CharacterRange::new(69415, 69415), CharacterRange::new(69424, 69445), CharacterRange::new(69488, 69505), CharacterRange::new(69552, 69572), CharacterRange::new(69600, 69622),
    CharacterRange::new(69635, 69687), CharacterRange::new(69745, 69746), CharacterRange::new(69749, 69749), CharacterRange::new(69763, 69807), CharacterRange::new(69840, 69864), CharacterRange::new(69891, 69926),
    CharacterRange::new(69956, 69956), CharacterRange::new(69959, 69959), CharacterRange::new(69968, 70002), CharacterRange::new(70006, 70006), CharacterRange::new(70019, 70066), CharacterRange::new(70081, 70084),
    CharacterRange::new(70106, 70106), CharacterRange::new(70108, 70108), CharacterRange::new(70144, 70161), CharacterRange::new(70163, 70187), CharacterRange::new(70207, 70208), CharacterRange::new(70272, 70278),
    CharacterRange::new(70280, 70280), CharacterRange::new(70282, 70285), CharacterRange::new(70287, 70301), CharacterRange::new(70303, 70312), CharacterRange::new(70320, 70366), CharacterRange::new(70405, 70412),
    CharacterRange::new(70415, 70416), CharacterRange::new(70419, 70440), CharacterRange::new(70442, 70448), CharacterRange::new(70450, 70451), CharacterRange::new(70453, 70457), CharacterRange::new(70461, 70461),
    CharacterRange::new(70480, 70480), CharacterRange::new(70493, 70497), CharacterRange::new(70656, 70708), CharacterRange::new(70727, 70730), CharacterRange::new(70751, 70753), CharacterRange::new(70784, 70831),
    CharacterRange::new(70852, 70853), CharacterRange::new(70855, 70855), CharacterRange::new(71040, 71086), CharacterRange::new(71128, 71131), CharacterRange::new(71168, 71215), CharacterRange::new(71236, 71236),
    CharacterRange::new(71296, 71338), CharacterRange::new(71352, 71352), CharacterRange::new(71424, 71450), CharacterRange::new(71488, 71494), CharacterRange::new(71680, 71723), CharacterRange::new(71840, 71903),
    CharacterRange::new(71935, 71942), CharacterRange::new(71945, 71945), CharacterRange::new(71948, 71955), CharacterRange::new(71957, 71958), CharacterRange::new(71960, 71983), CharacterRange::new(71999, 71999),
    CharacterRange::new(72001, 72001), CharacterRange::new(72096, 72103), CharacterRange::new(72106, 72144), CharacterRange::new(72161, 72161), CharacterRange::new(72163, 72163), CharacterRange::new(72192, 72192),
    CharacterRange::new(72203, 72242), CharacterRange::new(72250, 72250), CharacterRange::new(72272, 72272), CharacterRange::new(72284, 72329), CharacterRange::new(72349, 72349), CharacterRange::new(72368, 72440),
    CharacterRange::new(72704, 72712), CharacterRange::new(72714, 72750), CharacterRange::new(72768, 72768), CharacterRange::new(72818, 72847), CharacterRange::new(72960, 72966), CharacterRange::new(72968, 72969),
    CharacterRange::new(72971, 73008), CharacterRange::new(73030, 73030), CharacterRange::new(73056, 73061), CharacterRange::new(73063, 73064), CharacterRange::new(73066, 73097), CharacterRange::new(73112, 73112),
    CharacterRange::new(73440, 73458), CharacterRange::new(73474, 73474), CharacterRange::new(73476, 73488), CharacterRange::new(73490, 73523), CharacterRange::new(73648, 73648), CharacterRange::new(73728, 74649),
    CharacterRange::new(74752, 74862), CharacterRange::new(74880, 75075), CharacterRange::new(77712, 77808), CharacterRange::new(77824, 78895), CharacterRange::new(78913, 78918), CharacterRange::new(82944, 83526),
    CharacterRange::new(92160, 92728), CharacterRange::new(92736, 92766), CharacterRange::new(92784, 92862), CharacterRange::new(92880, 92909), CharacterRange::new(92928, 92975), CharacterRange::new(92992, 92995),
    CharacterRange::new(93027, 93047), CharacterRange::new(93053, 93071), CharacterRange::new(93760, 93823), CharacterRange::new(93952, 94026), CharacterRange::new(94032, 94032), CharacterRange::new(94099, 94111),
    CharacterRange::new(94176, 94177), CharacterRange::new(94179, 94179), CharacterRange::new(94208, 100343), CharacterRange::new(100352, 101589), CharacterRange::new(101632, 101640), CharacterRange::new(110576, 110579),
    CharacterRange::new(110581, 110587), CharacterRange::new(110589, 110590), CharacterRange::new(110592, 110882), CharacterRange::new(110898, 110898), CharacterRange::new(110928, 110930), CharacterRange::new(110933, 110933),
    CharacterRange::new(110948, 110951), CharacterRange::new(110960, 111355), CharacterRange::new(113664, 113770), CharacterRange::new(113776, 113788), CharacterRange::new(113792, 113800), CharacterRange::new(113808, 113817),
    CharacterRange::new(119808, 119892), CharacterRange::new(119894, 119964), CharacterRange::new(119966, 119967), CharacterRange::new(119970, 119970), CharacterRange::new(119973, 119974), CharacterRange::new(119977, 119980),
    CharacterRange::new(119982, 119993), CharacterRange::new(119995, 119995), CharacterRange::new(119997, 120003), CharacterRange::new(120005, 120069), CharacterRange::new(120071, 120074), CharacterRange::new(120077, 120084),
    CharacterRange::new(120086, 120092), CharacterRange::new(120094, 120121), CharacterRange::new(120123, 120126), CharacterRange::new(120128, 120132), CharacterRange::new(120134, 120134), CharacterRange::new(120138, 120144),
    CharacterRange::new(120146, 120485), CharacterRange::new(120488, 120512), CharacterRange::new(120514, 120538), CharacterRange::new(120540, 120570), CharacterRange::new(120572, 120596), CharacterRange::new(120598, 120628),
    CharacterRange::new(120630, 120654), CharacterRange::new(120656, 120686), CharacterRange::new(120688, 120712), CharacterRange::new(120714, 120744), CharacterRange::new(120746, 120770), CharacterRange::new(120772, 120779),
    CharacterRange::new(122624, 122654), CharacterRange::new(122661, 122666), CharacterRange::new(122928, 122989), CharacterRange::new(123136, 123180), CharacterRange::new(123191, 123197), CharacterRange::new(123214, 123214),
    CharacterRange::new(123536, 123565), CharacterRange::new(123584, 123627), CharacterRange::new(124112, 124139), CharacterRange::new(124896, 124902), CharacterRange::new(124904, 124907), CharacterRange::new(124909, 124910),
    CharacterRange::new(124912, 124926), CharacterRange::new(124928, 125124), CharacterRange::new(125184, 125251), CharacterRange::new(125259, 125259), CharacterRange::new(126464, 126467), CharacterRange::new(126469, 126495),
    CharacterRange::new(126497, 126498), CharacterRange::new(126500, 126500), CharacterRange::new(126503, 126503), CharacterRange::new(126505, 126514), CharacterRange::new(126516, 126519), CharacterRange::new(126521, 126521),
    CharacterRange::new(126523, 126523), CharacterRange::new(126530, 126530), CharacterRange::new(126535, 126535), CharacterRange::new(126537, 126537), CharacterRange::new(126539, 126539), CharacterRange::new(126541, 126543),
    CharacterRange::new(126545, 126546), CharacterRange::new(126548, 126548), CharacterRange::new(126551, 126551), CharacterRange::new(126553, 126553), CharacterRange::new(126555, 126555), CharacterRange::new(126557, 126557),
    CharacterRange::new(126559, 126559), CharacterRange::new(126561, 126562), CharacterRange::new(126564, 126564), CharacterRange::new(126567, 126570), CharacterRange::new(126572, 126578), CharacterRange::new(126580, 126583),
    CharacterRange::new(126585, 126588), CharacterRange::new(126590, 126590), CharacterRange::new(126592, 126601), CharacterRange::new(126603, 126619), CharacterRange::new(126625, 126627), CharacterRange::new(126629, 126633),
    CharacterRange::new(126635, 126651), CharacterRange::new(131072, 173791), CharacterRange::new(173824, 177977), CharacterRange::new(177984, 178205), CharacterRange::new(178208, 183969), CharacterRange::new(183984, 191456),
    CharacterRange::new(191472, 192093), CharacterRange::new(194560, 195101), CharacterRange::new(196608, 201546), CharacterRange::new(201552, 205743),
];

#[rustfmt::skip]
static sym_identifier_character_set_2: [CharacterRange; 778] = [
    CharacterRange::new(36, 36), CharacterRange::new(48, 57), CharacterRange::new(65, 90), CharacterRange::new(92, 92), CharacterRange::new(95, 95), CharacterRange::new(97, 122),
    CharacterRange::new(170, 170), CharacterRange::new(181, 181), CharacterRange::new(183, 183), CharacterRange::new(186, 186), CharacterRange::new(192, 214), CharacterRange::new(216, 246),
    CharacterRange::new(248, 705), CharacterRange::new(710, 721), CharacterRange::new(736, 740), CharacterRange::new(748, 748), CharacterRange::new(750, 750), CharacterRange::new(768, 884),
    CharacterRange::new(886, 887), CharacterRange::new(891, 893), CharacterRange::new(895, 895), CharacterRange::new(902, 906), CharacterRange::new(908, 908), CharacterRange::new(910, 929),
    CharacterRange::new(931, 1013), CharacterRange::new(1015, 1153), CharacterRange::new(1155, 1159), CharacterRange::new(1162, 1327), CharacterRange::new(1329, 1366), CharacterRange::new(1369, 1369),
    CharacterRange::new(1376, 1416), CharacterRange::new(1425, 1469), CharacterRange::new(1471, 1471), CharacterRange::new(1473, 1474), CharacterRange::new(1476, 1477), CharacterRange::new(1479, 1479),
    CharacterRange::new(1488, 1514), CharacterRange::new(1519, 1522), CharacterRange::new(1552, 1562), CharacterRange::new(1568, 1641), CharacterRange::new(1646, 1747), CharacterRange::new(1749, 1756),
    CharacterRange::new(1759, 1768), CharacterRange::new(1770, 1788), CharacterRange::new(1791, 1791), CharacterRange::new(1808, 1866), CharacterRange::new(1869, 1969), CharacterRange::new(1984, 2037),
    CharacterRange::new(2042, 2042), CharacterRange::new(2045, 2045), CharacterRange::new(2048, 2093), CharacterRange::new(2112, 2139), CharacterRange::new(2144, 2154), CharacterRange::new(2160, 2183),
    CharacterRange::new(2185, 2190), CharacterRange::new(2200, 2273), CharacterRange::new(2275, 2403), CharacterRange::new(2406, 2415), CharacterRange::new(2417, 2435), CharacterRange::new(2437, 2444),
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
    CharacterRange::new(7232, 7241), CharacterRange::new(7245, 7293), CharacterRange::new(7296, 7304), CharacterRange::new(7312, 7354), CharacterRange::new(7357, 7359), CharacterRange::new(7376, 7378),
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
    CharacterRange::new(42623, 42737), CharacterRange::new(42775, 42783), CharacterRange::new(42786, 42888), CharacterRange::new(42891, 42954), CharacterRange::new(42960, 42961), CharacterRange::new(42963, 42963),
    CharacterRange::new(42965, 42969), CharacterRange::new(42994, 43047), CharacterRange::new(43052, 43052), CharacterRange::new(43072, 43123), CharacterRange::new(43136, 43205), CharacterRange::new(43216, 43225),
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
    CharacterRange::new(66995, 67001), CharacterRange::new(67003, 67004), CharacterRange::new(67072, 67382), CharacterRange::new(67392, 67413), CharacterRange::new(67424, 67431), CharacterRange::new(67456, 67461),
    CharacterRange::new(67463, 67504), CharacterRange::new(67506, 67514), CharacterRange::new(67584, 67589), CharacterRange::new(67592, 67592), CharacterRange::new(67594, 67637), CharacterRange::new(67639, 67640),
    CharacterRange::new(67644, 67644), CharacterRange::new(67647, 67669), CharacterRange::new(67680, 67702), CharacterRange::new(67712, 67742), CharacterRange::new(67808, 67826), CharacterRange::new(67828, 67829),
    CharacterRange::new(67840, 67861), CharacterRange::new(67872, 67897), CharacterRange::new(67968, 68023), CharacterRange::new(68030, 68031), CharacterRange::new(68096, 68099), CharacterRange::new(68101, 68102),
    CharacterRange::new(68108, 68115), CharacterRange::new(68117, 68119), CharacterRange::new(68121, 68149), CharacterRange::new(68152, 68154), CharacterRange::new(68159, 68159), CharacterRange::new(68192, 68220),
    CharacterRange::new(68224, 68252), CharacterRange::new(68288, 68295), CharacterRange::new(68297, 68326), CharacterRange::new(68352, 68405), CharacterRange::new(68416, 68437), CharacterRange::new(68448, 68466),
    CharacterRange::new(68480, 68497), CharacterRange::new(68608, 68680), CharacterRange::new(68736, 68786), CharacterRange::new(68800, 68850), CharacterRange::new(68864, 68903), CharacterRange::new(68912, 68921),
    CharacterRange::new(69248, 69289), CharacterRange::new(69291, 69292), CharacterRange::new(69296, 69297), CharacterRange::new(69373, 69404), CharacterRange::new(69415, 69415), CharacterRange::new(69424, 69456),
    CharacterRange::new(69488, 69509), CharacterRange::new(69552, 69572), CharacterRange::new(69600, 69622), CharacterRange::new(69632, 69702), CharacterRange::new(69734, 69749), CharacterRange::new(69759, 69818),
    CharacterRange::new(69826, 69826), CharacterRange::new(69840, 69864), CharacterRange::new(69872, 69881), CharacterRange::new(69888, 69940), CharacterRange::new(69942, 69951), CharacterRange::new(69956, 69959),
    CharacterRange::new(69968, 70003), CharacterRange::new(70006, 70006), CharacterRange::new(70016, 70084), CharacterRange::new(70089, 70092), CharacterRange::new(70094, 70106), CharacterRange::new(70108, 70108),
    CharacterRange::new(70144, 70161), CharacterRange::new(70163, 70199), CharacterRange::new(70206, 70209), CharacterRange::new(70272, 70278), CharacterRange::new(70280, 70280), CharacterRange::new(70282, 70285),
    CharacterRange::new(70287, 70301), CharacterRange::new(70303, 70312), CharacterRange::new(70320, 70378), CharacterRange::new(70384, 70393), CharacterRange::new(70400, 70403), CharacterRange::new(70405, 70412),
    CharacterRange::new(70415, 70416), CharacterRange::new(70419, 70440), CharacterRange::new(70442, 70448), CharacterRange::new(70450, 70451), CharacterRange::new(70453, 70457), CharacterRange::new(70459, 70468),
    CharacterRange::new(70471, 70472), CharacterRange::new(70475, 70477), CharacterRange::new(70480, 70480), CharacterRange::new(70487, 70487), CharacterRange::new(70493, 70499), CharacterRange::new(70502, 70508),
    CharacterRange::new(70512, 70516), CharacterRange::new(70656, 70730), CharacterRange::new(70736, 70745), CharacterRange::new(70750, 70753), CharacterRange::new(70784, 70853), CharacterRange::new(70855, 70855),
    CharacterRange::new(70864, 70873), CharacterRange::new(71040, 71093), CharacterRange::new(71096, 71104), CharacterRange::new(71128, 71133), CharacterRange::new(71168, 71232), CharacterRange::new(71236, 71236),
    CharacterRange::new(71248, 71257), CharacterRange::new(71296, 71352), CharacterRange::new(71360, 71369), CharacterRange::new(71424, 71450), CharacterRange::new(71453, 71467), CharacterRange::new(71472, 71481),
    CharacterRange::new(71488, 71494), CharacterRange::new(71680, 71738), CharacterRange::new(71840, 71913), CharacterRange::new(71935, 71942), CharacterRange::new(71945, 71945), CharacterRange::new(71948, 71955),
    CharacterRange::new(71957, 71958), CharacterRange::new(71960, 71989), CharacterRange::new(71991, 71992), CharacterRange::new(71995, 72003), CharacterRange::new(72016, 72025), CharacterRange::new(72096, 72103),
    CharacterRange::new(72106, 72151), CharacterRange::new(72154, 72161), CharacterRange::new(72163, 72164), CharacterRange::new(72192, 72254), CharacterRange::new(72263, 72263), CharacterRange::new(72272, 72345),
    CharacterRange::new(72349, 72349), CharacterRange::new(72368, 72440), CharacterRange::new(72704, 72712), CharacterRange::new(72714, 72758), CharacterRange::new(72760, 72768), CharacterRange::new(72784, 72793),
    CharacterRange::new(72818, 72847), CharacterRange::new(72850, 72871), CharacterRange::new(72873, 72886), CharacterRange::new(72960, 72966), CharacterRange::new(72968, 72969), CharacterRange::new(72971, 73014),
    CharacterRange::new(73018, 73018), CharacterRange::new(73020, 73021), CharacterRange::new(73023, 73031), CharacterRange::new(73040, 73049), CharacterRange::new(73056, 73061), CharacterRange::new(73063, 73064),
    CharacterRange::new(73066, 73102), CharacterRange::new(73104, 73105), CharacterRange::new(73107, 73112), CharacterRange::new(73120, 73129), CharacterRange::new(73440, 73462), CharacterRange::new(73472, 73488),
    CharacterRange::new(73490, 73530), CharacterRange::new(73534, 73538), CharacterRange::new(73552, 73561), CharacterRange::new(73648, 73648), CharacterRange::new(73728, 74649), CharacterRange::new(74752, 74862),
    CharacterRange::new(74880, 75075), CharacterRange::new(77712, 77808), CharacterRange::new(77824, 78895), CharacterRange::new(78912, 78933), CharacterRange::new(82944, 83526), CharacterRange::new(92160, 92728),
    CharacterRange::new(92736, 92766), CharacterRange::new(92768, 92777), CharacterRange::new(92784, 92862), CharacterRange::new(92864, 92873), CharacterRange::new(92880, 92909), CharacterRange::new(92912, 92916),
    CharacterRange::new(92928, 92982), CharacterRange::new(92992, 92995), CharacterRange::new(93008, 93017), CharacterRange::new(93027, 93047), CharacterRange::new(93053, 93071), CharacterRange::new(93760, 93823),
    CharacterRange::new(93952, 94026), CharacterRange::new(94031, 94087), CharacterRange::new(94095, 94111), CharacterRange::new(94176, 94177), CharacterRange::new(94179, 94180), CharacterRange::new(94192, 94193),
    CharacterRange::new(94208, 100343), CharacterRange::new(100352, 101589), CharacterRange::new(101632, 101640), CharacterRange::new(110576, 110579), CharacterRange::new(110581, 110587), CharacterRange::new(110589, 110590),
    CharacterRange::new(110592, 110882), CharacterRange::new(110898, 110898), CharacterRange::new(110928, 110930), CharacterRange::new(110933, 110933), CharacterRange::new(110948, 110951), CharacterRange::new(110960, 111355),
    CharacterRange::new(113664, 113770), CharacterRange::new(113776, 113788), CharacterRange::new(113792, 113800), CharacterRange::new(113808, 113817), CharacterRange::new(113821, 113822), CharacterRange::new(118528, 118573),
    CharacterRange::new(118576, 118598), CharacterRange::new(119141, 119145), CharacterRange::new(119149, 119154), CharacterRange::new(119163, 119170), CharacterRange::new(119173, 119179), CharacterRange::new(119210, 119213),
    CharacterRange::new(119362, 119364), CharacterRange::new(119808, 119892), CharacterRange::new(119894, 119964), CharacterRange::new(119966, 119967), CharacterRange::new(119970, 119970), CharacterRange::new(119973, 119974),
    CharacterRange::new(119977, 119980), CharacterRange::new(119982, 119993), CharacterRange::new(119995, 119995), CharacterRange::new(119997, 120003), CharacterRange::new(120005, 120069), CharacterRange::new(120071, 120074),
    CharacterRange::new(120077, 120084), CharacterRange::new(120086, 120092), CharacterRange::new(120094, 120121), CharacterRange::new(120123, 120126), CharacterRange::new(120128, 120132), CharacterRange::new(120134, 120134),
    CharacterRange::new(120138, 120144), CharacterRange::new(120146, 120485), CharacterRange::new(120488, 120512), CharacterRange::new(120514, 120538), CharacterRange::new(120540, 120570), CharacterRange::new(120572, 120596),
    CharacterRange::new(120598, 120628), CharacterRange::new(120630, 120654), CharacterRange::new(120656, 120686), CharacterRange::new(120688, 120712), CharacterRange::new(120714, 120744), CharacterRange::new(120746, 120770),
    CharacterRange::new(120772, 120779), CharacterRange::new(120782, 120831), CharacterRange::new(121344, 121398), CharacterRange::new(121403, 121452), CharacterRange::new(121461, 121461), CharacterRange::new(121476, 121476),
    CharacterRange::new(121499, 121503), CharacterRange::new(121505, 121519), CharacterRange::new(122624, 122654), CharacterRange::new(122661, 122666), CharacterRange::new(122880, 122886), CharacterRange::new(122888, 122904),
    CharacterRange::new(122907, 122913), CharacterRange::new(122915, 122916), CharacterRange::new(122918, 122922), CharacterRange::new(122928, 122989), CharacterRange::new(123023, 123023), CharacterRange::new(123136, 123180),
    CharacterRange::new(123184, 123197), CharacterRange::new(123200, 123209), CharacterRange::new(123214, 123214), CharacterRange::new(123536, 123566), CharacterRange::new(123584, 123641), CharacterRange::new(124112, 124153),
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
                if eof { state = 420; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 509), (34, 624), (35, 677), (37, 529), (38, 538), (39, 604), (40, 425), (41, 428),
                    (42, 525), (43, 520), (44, 427), (45, 512), (46, 578), (47, 527), (48, 584), (58, 731),
                    (59, 553), (60, 576), (61, 560), (62, 542), (63, 563), (64, 610), (76, 643), (85, 645),
                    (91, 555), (92, 2), (93, 556), (94, 535), (117, 647), (123, 557), (124, 532), (125, 558),
                    (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 415; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 585; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            1 => {
                if lookahead == 10 { state = 93; lexer.advance(true); continue; }
                return result;
            }
            2 => {
                if lookahead == 10 { state = 93; lexer.advance(true); continue; }
                if lookahead == 13 { state = 1; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            3 => {
                if lookahead == 10 { state = 101; lexer.advance(true); continue; }
                return result;
            }
            4 => {
                if lookahead == 10 { state = 101; lexer.advance(true); continue; }
                if lookahead == 13 { state = 3; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                if lookahead == 10 { state = 99; lexer.advance(true); continue; }
                return result;
            }
            6 => {
                if lookahead == 10 { state = 99; lexer.advance(true); continue; }
                if lookahead == 13 { state = 5; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if lookahead == 10 { state = 108; lexer.advance(true); continue; }
                return result;
            }
            8 => {
                if lookahead == 10 { state = 108; lexer.advance(true); continue; }
                if lookahead == 13 { state = 7; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if lookahead == 10 { state = 107; lexer.advance(true); continue; }
                return result;
            }
            10 => {
                if lookahead == 10 { state = 107; lexer.advance(true); continue; }
                if lookahead == 13 { state = 9; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if lookahead == 10 { state = 102; lexer.advance(true); continue; }
                return result;
            }
            12 => {
                if lookahead == 10 { state = 102; lexer.advance(true); continue; }
                if lookahead == 13 { state = 11; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if lookahead == 10 { state = 96; lexer.advance(true); continue; }
                return result;
            }
            14 => {
                if lookahead == 10 { state = 96; lexer.advance(true); continue; }
                if lookahead == 13 { state = 13; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if lookahead == 10 { state = 111; lexer.advance(true); continue; }
                return result;
            }
            16 => {
                if lookahead == 10 { state = 111; lexer.advance(true); continue; }
                if lookahead == 13 { state = 15; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if lookahead == 10 { state = 115; lexer.advance(true); continue; }
                return result;
            }
            18 => {
                if lookahead == 10 { state = 115; lexer.advance(true); continue; }
                if lookahead == 13 { state = 17; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if lookahead == 10 { state = 110; lexer.advance(true); continue; }
                return result;
            }
            20 => {
                if lookahead == 10 { state = 110; lexer.advance(true); continue; }
                if lookahead == 13 { state = 19; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                if lookahead == 10 { state = 114; lexer.advance(true); continue; }
                return result;
            }
            22 => {
                if lookahead == 10 { state = 114; lexer.advance(true); continue; }
                if lookahead == 13 { state = 21; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            23 => {
                if lookahead == 10 { state = 132; lexer.advance(true); continue; }
                return result;
            }
            24 => {
                if lookahead == 10 { state = 132; lexer.advance(true); continue; }
                if lookahead == 13 { state = 23; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            25 => {
                if lookahead == 10 { state = 127; lexer.advance(true); continue; }
                return result;
            }
            26 => {
                if lookahead == 10 { state = 127; lexer.advance(true); continue; }
                if lookahead == 13 { state = 25; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            27 => {
                if lookahead == 10 { state = 133; lexer.advance(true); continue; }
                return result;
            }
            28 => {
                if lookahead == 10 { state = 133; lexer.advance(true); continue; }
                if lookahead == 13 { state = 27; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                if lookahead == 10 { state = 120; lexer.advance(true); continue; }
                return result;
            }
            30 => {
                if lookahead == 10 { state = 120; lexer.advance(true); continue; }
                if lookahead == 13 { state = 29; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            31 => {
                if lookahead == 10 { state = 124; lexer.advance(true); continue; }
                return result;
            }
            32 => {
                if lookahead == 10 { state = 124; lexer.advance(true); continue; }
                if lookahead == 13 { state = 31; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            33 => {
                if lookahead == 10 { state = 112; lexer.advance(true); continue; }
                return result;
            }
            34 => {
                if lookahead == 10 { state = 112; lexer.advance(true); continue; }
                if lookahead == 13 { state = 33; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            35 => {
                if lookahead == 10 { state = 117; lexer.advance(true); continue; }
                return result;
            }
            36 => {
                if lookahead == 10 { state = 117; lexer.advance(true); continue; }
                if lookahead == 13 { state = 35; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                if lookahead == 10 { state = 123; lexer.advance(true); continue; }
                return result;
            }
            38 => {
                if lookahead == 10 { state = 123; lexer.advance(true); continue; }
                if lookahead == 13 { state = 37; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                if lookahead == 10 { state = 134; lexer.advance(true); continue; }
                return result;
            }
            40 => {
                if lookahead == 10 { state = 134; lexer.advance(true); continue; }
                if lookahead == 13 { state = 39; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                if lookahead == 10 { state = 130; lexer.advance(true); continue; }
                return result;
            }
            42 => {
                if lookahead == 10 { state = 130; lexer.advance(true); continue; }
                if lookahead == 13 { state = 41; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            43 => {
                if lookahead == 10 { state = 122; lexer.advance(true); continue; }
                return result;
            }
            44 => {
                if lookahead == 10 { state = 122; lexer.advance(true); continue; }
                if lookahead == 13 { state = 43; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            45 => {
                if lookahead == 10 { state = 121; lexer.advance(true); continue; }
                return result;
            }
            46 => {
                if lookahead == 10 { state = 121; lexer.advance(true); continue; }
                if lookahead == 13 { state = 45; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                if lookahead == 10 { state = 116; lexer.advance(true); continue; }
                return result;
            }
            48 => {
                if lookahead == 10 { state = 116; lexer.advance(true); continue; }
                if lookahead == 13 { state = 47; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                if lookahead == 10 { state = 51; lexer.advance(true); continue; }
                return result;
            }
            50 => {
                if lookahead == 10 { state = 51; lexer.advance(true); continue; }
                if lookahead == 13 { state = 49; lexer.advance(true); continue; }
                return result;
            }
            51 => {
                if let Some(next) = advance_map(&[
                    (10, 431), (33, 151), (37, 528), (38, 537), (40, 506), (42, 524), (43, 518), (45, 511),
                    (47, 526), (60, 548), (61, 152), (62, 543),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 92 { state = 50; lexer.advance(true); continue; }
                if lookahead == 94 { state = 534; lexer.advance(false); continue; }
                if lookahead == 124 { state = 533; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 51; lexer.advance(true); continue; }
                return result;
            }
            52 => {
                if lookahead == 10 { state = 129; lexer.advance(true); continue; }
                return result;
            }
            53 => {
                if lookahead == 10 { state = 129; lexer.advance(true); continue; }
                if lookahead == 13 { state = 52; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                if lookahead == 10 { state = 423; lexer.advance(false); continue; }
                if lookahead == 13 { state = 58; lexer.advance(false); continue; }
                if lookahead == 40 { state = 425; lexer.advance(false); continue; }
                if lookahead == 47 { state = 459; lexer.advance(false); continue; }
                if lookahead == 92 { state = 453; lexer.advance(false); continue; }
                if lookahead == 160 { state = 455; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 149; lexer.advance(true); continue; }
                if lookahead != 0 { state = 462; lexer.advance(false); continue; }
                return result;
            }
            55 => {
                if lookahead == 10 { state = 423; lexer.advance(false); continue; }
                if lookahead == 13 { state = 58; lexer.advance(false); continue; }
                if lookahead == 47 { state = 459; lexer.advance(false); continue; }
                if lookahead == 92 { state = 453; lexer.advance(false); continue; }
                if lookahead == 160 { state = 455; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 { state = 149; lexer.advance(true); continue; }
                if lookahead != 0 { state = 462; lexer.advance(false); continue; }
                return result;
            }
            56 => {
                if lookahead == 10 { state = 423; lexer.advance(false); continue; }
                if lookahead == 13 { state = 57; lexer.advance(false); continue; }
                if lookahead == 40 { state = 506; lexer.advance(false); continue; }
                if lookahead == 46 { state = 391; lexer.advance(false); continue; }
                if lookahead == 47 { state = 139; lexer.advance(false); continue; }
                if lookahead == 48 { state = 586; lexer.advance(false); continue; }
                if lookahead == 92 { state = 63; lexer.advance(true); continue; }
                if lookahead == 43 || lookahead == 45 { state = 143; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 12 || lookahead == 32 || lookahead == 160 { state = 136; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                return result;
            }
            57 => {
                if lookahead == 10 { state = 423; lexer.advance(false); continue; }
                if lookahead == 40 { state = 506; lexer.advance(false); continue; }
                if lookahead == 46 { state = 391; lexer.advance(false); continue; }
                if lookahead == 47 { state = 139; lexer.advance(false); continue; }
                if lookahead == 48 { state = 586; lexer.advance(false); continue; }
                if lookahead == 92 { state = 63; lexer.advance(true); continue; }
                if lookahead == 43 || lookahead == 45 { state = 143; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 136; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                return result;
            }
            58 => {
                if lookahead == 10 { state = 423; lexer.advance(false); continue; }
                if lookahead == 47 { state = 459; lexer.advance(false); continue; }
                if lookahead == 92 { state = 453; lexer.advance(false); continue; }
                if lookahead == 160 { state = 455; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 149; lexer.advance(true); continue; }
                if lookahead != 0 { state = 462; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                if lookahead == 10 { state = 131; lexer.advance(true); continue; }
                if lookahead == 34 { state = 624; lexer.advance(false); continue; }
                if lookahead == 47 { state = 629; lexer.advance(false); continue; }
                if lookahead == 92 { state = 60; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 632; lexer.advance(false); continue; }
                if lookahead != 0 { state = 633; lexer.advance(false); continue; }
                return result;
            }
            60 => {
                if lookahead == 10 { state = 635; lexer.advance(false); continue; }
                if lookahead == 13 { state = 634; lexer.advance(false); continue; }
                if lookahead == 85 { state = 413; lexer.advance(false); continue; }
                if lookahead == 117 { state = 405; lexer.advance(false); continue; }
                if lookahead == 120 { state = 399; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 637; lexer.advance(false); continue; }
                if lookahead != 0 { state = 634; lexer.advance(false); continue; }
                return result;
            }
            61 => {
                if lookahead == 10 { state = 135; lexer.advance(true); continue; }
                if lookahead == 39 { state = 604; lexer.advance(false); continue; }
                if lookahead == 47 { state = 607; lexer.advance(false); continue; }
                if lookahead == 92 { state = 606; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 608; lexer.advance(false); continue; }
                if lookahead != 0 { state = 605; lexer.advance(false); continue; }
                return result;
            }
            62 => {
                if lookahead == 10 { state = 136; lexer.advance(true); continue; }
                return result;
            }
            63 => {
                if lookahead == 10 { state = 136; lexer.advance(true); continue; }
                if lookahead == 13 { state = 62; lexer.advance(true); continue; }
                return result;
            }
            64 => {
                if lookahead == 10 { state = 137; lexer.advance(true); continue; }
                return result;
            }
            65 => {
                if lookahead == 10 { state = 137; lexer.advance(true); continue; }
                if lookahead == 13 { state = 64; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            66 => {
                if lookahead == 10 { state = 94; lexer.advance(true); continue; }
                return result;
            }
            67 => {
                if lookahead == 10 { state = 94; lexer.advance(true); continue; }
                if lookahead == 13 { state = 66; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            68 => {
                if lookahead == 10 { state = 105; lexer.advance(true); continue; }
                return result;
            }
            69 => {
                if lookahead == 10 { state = 105; lexer.advance(true); continue; }
                if lookahead == 13 { state = 68; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            70 => {
                if lookahead == 10 { state = 103; lexer.advance(true); continue; }
                return result;
            }
            71 => {
                if lookahead == 10 { state = 103; lexer.advance(true); continue; }
                if lookahead == 13 { state = 70; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            72 => {
                if lookahead == 10 { state = 97; lexer.advance(true); continue; }
                return result;
            }
            73 => {
                if lookahead == 10 { state = 97; lexer.advance(true); continue; }
                if lookahead == 13 { state = 72; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            74 => {
                if lookahead == 10 { state = 113; lexer.advance(true); continue; }
                return result;
            }
            75 => {
                if lookahead == 10 { state = 113; lexer.advance(true); continue; }
                if lookahead == 13 { state = 74; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            76 => {
                if lookahead == 10 { state = 125; lexer.advance(true); continue; }
                return result;
            }
            77 => {
                if lookahead == 10 { state = 125; lexer.advance(true); continue; }
                if lookahead == 13 { state = 76; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            78 => {
                if lookahead == 10 { state = 118; lexer.advance(true); continue; }
                return result;
            }
            79 => {
                if lookahead == 10 { state = 118; lexer.advance(true); continue; }
                if lookahead == 13 { state = 78; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            80 => {
                if lookahead == 10 { state = 95; lexer.advance(true); continue; }
                return result;
            }
            81 => {
                if lookahead == 10 { state = 95; lexer.advance(true); continue; }
                if lookahead == 13 { state = 80; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            82 => {
                if lookahead == 10 { state = 100; lexer.advance(true); continue; }
                return result;
            }
            83 => {
                if lookahead == 10 { state = 100; lexer.advance(true); continue; }
                if lookahead == 13 { state = 82; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            84 => {
                if lookahead == 10 { state = 128; lexer.advance(true); continue; }
                return result;
            }
            85 => {
                if lookahead == 10 { state = 128; lexer.advance(true); continue; }
                if lookahead == 13 { state = 84; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            86 => {
                if lookahead == 10 { state = 106; lexer.advance(true); continue; }
                return result;
            }
            87 => {
                if lookahead == 10 { state = 106; lexer.advance(true); continue; }
                if lookahead == 13 { state = 86; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            88 => {
                if lookahead == 10 { state = 104; lexer.advance(true); continue; }
                return result;
            }
            89 => {
                if lookahead == 10 { state = 104; lexer.advance(true); continue; }
                if lookahead == 13 { state = 88; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            90 => {
                if lookahead == 10 { state = 109; lexer.advance(true); continue; }
                return result;
            }
            91 => {
                if lookahead == 10 { state = 109; lexer.advance(true); continue; }
                if lookahead == 13 { state = 90; lexer.advance(true); continue; }
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            92 => {
                if lookahead == 13 { state = 671; lexer.advance(false); continue; }
                if lookahead == 92 { state = 661; lexer.advance(false); continue; }
                if lookahead != 0 { state = 670; lexer.advance(false); continue; }
                return result;
            }
            93 => {
                if let Some(next) = advance_map(&[
                    (33, 509), (34, 624), (35, 677), (37, 529), (38, 538), (39, 604), (40, 506), (41, 428),
                    (42, 525), (43, 520), (44, 427), (45, 512), (46, 578), (47, 527), (48, 584), (58, 562),
                    (59, 553), (60, 547), (61, 560), (62, 542), (63, 563), (64, 610), (76, 643), (85, 645),
                    (91, 555), (92, 2), (93, 556), (94, 535), (117, 647), (123, 557), (124, 532), (125, 558),
                    (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 93; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 585; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            94 => {
                if let Some(next) = advance_map(&[
                    (33, 509), (34, 624), (35, 270), (37, 529), (38, 538), (39, 604), (40, 506), (41, 428),
                    (42, 525), (43, 520), (44, 427), (45, 512), (46, 578), (47, 527), (48, 586), (58, 562),
                    (59, 553), (60, 547), (61, 560), (62, 542), (63, 563), (64, 621), (76, 643), (85, 645),
                    (91, 555), (92, 67), (93, 556), (94, 535), (117, 647), (123, 557), (124, 532), (125, 558),
                    (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 94; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            95 => {
                if let Some(next) = advance_map(&[
                    (33, 509), (34, 624), (35, 270), (37, 528), (38, 537), (39, 604), (40, 506), (41, 428),
                    (42, 524), (43, 521), (44, 427), (45, 513), (46, 578), (47, 526), (48, 586), (58, 562),
                    (59, 553), (60, 548), (61, 152), (62, 543), (63, 563), (64, 621), (76, 643), (85, 645),
                    (91, 555), (92, 81), (93, 556), (94, 534), (117, 647), (123, 557), (124, 533), (125, 558),
                    (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 95; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            96 => {
                if let Some(next) = advance_map(&[
                    (33, 509), (34, 624), (37, 529), (38, 538), (39, 604), (40, 506), (42, 525), (43, 520),
                    (44, 427), (45, 512), (46, 578), (47, 527), (48, 586), (59, 553), (60, 547), (61, 560),
                    (62, 542), (63, 563), (64, 619), (76, 643), (85, 645), (91, 555), (92, 14), (94, 535),
                    (117, 647), (123, 557), (124, 532), (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 96; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            97 => {
                if let Some(next) = advance_map(&[
                    (33, 509), (34, 624), (37, 528), (38, 537), (39, 604), (40, 506), (42, 524), (43, 521),
                    (44, 427), (45, 513), (46, 578), (47, 526), (48, 586), (59, 553), (60, 548), (61, 152),
                    (62, 543), (63, 563), (64, 619), (76, 643), (85, 645), (91, 555), (92, 73), (94, 534),
                    (117, 647), (123, 557), (124, 533), (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 97; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            98 => {
                if let Some(next) = advance_map(&[
                    (33, 508), (34, 624), (35, 677), (38, 536), (39, 604), (40, 506), (41, 428), (42, 524),
                    (43, 521), (45, 514), (46, 391), (47, 139), (48, 586), (58, 731), (59, 553), (64, 614),
                    (76, 643), (85, 645), (91, 555), (92, 6), (94, 534), (117, 647), (123, 557), (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 99; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            99 => {
                if let Some(next) = advance_map(&[
                    (33, 508), (34, 624), (35, 677), (38, 536), (39, 604), (40, 506), (41, 428), (42, 524),
                    (43, 521), (45, 514), (46, 391), (47, 139), (48, 586), (59, 553), (64, 614), (76, 643),
                    (85, 645), (91, 555), (92, 6), (94, 534), (117, 647), (123, 557), (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 99; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            100 => {
                if let Some(next) = advance_map(&[
                    (33, 508), (34, 624), (35, 677), (38, 536), (39, 604), (40, 506), (42, 524), (43, 521),
                    (45, 514), (46, 391), (47, 139), (48, 586), (59, 553), (64, 610), (76, 643), (85, 645),
                    (91, 555), (92, 83), (94, 534), (117, 647), (123, 557), (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 100; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            101 => {
                if let Some(next) = advance_map(&[
                    (33, 508), (34, 624), (35, 680), (38, 536), (39, 604), (40, 506), (41, 428), (42, 524),
                    (43, 521), (44, 427), (45, 514), (46, 147), (47, 139), (48, 586), (58, 562), (59, 553),
                    (60, 546), (62, 541), (64, 616), (76, 643), (85, 645), (91, 555), (92, 4), (93, 556),
                    (94, 534), (117, 647), (123, 557), (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 101; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            102 => {
                if let Some(next) = advance_map(&[
                    (33, 508), (34, 624), (35, 680), (38, 536), (39, 604), (40, 506), (42, 524), (43, 521),
                    (45, 514), (46, 391), (47, 139), (48, 586), (59, 553), (64, 615), (76, 643), (85, 645),
                    (91, 555), (92, 12), (94, 534), (117, 647), (123, 557), (125, 558), (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 102; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            103 => {
                if let Some(next) = advance_map(&[
                    (33, 508), (34, 624), (35, 680), (38, 536), (39, 604), (40, 506), (42, 524), (43, 521),
                    (45, 514), (46, 391), (47, 139), (48, 586), (59, 553), (64, 611), (76, 643), (85, 645),
                    (91, 555), (92, 71), (94, 534), (117, 647), (123, 557), (125, 558), (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 103; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            104 => {
                if let Some(next) = advance_map(&[
                    (33, 508), (34, 624), (35, 680), (38, 536), (39, 604), (40, 506), (42, 524), (43, 521),
                    (45, 514), (46, 391), (47, 139), (48, 586), (59, 553), (64, 612), (76, 643), (85, 645),
                    (91, 555), (92, 89), (94, 534), (117, 647), (123, 557), (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 104; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            105 => {
                if let Some(next) = advance_map(&[
                    (33, 508), (34, 624), (35, 239), (38, 536), (39, 604), (40, 506), (42, 524), (43, 521),
                    (45, 514), (46, 391), (47, 139), (48, 586), (59, 553), (64, 618), (76, 643), (85, 645),
                    (91, 555), (92, 69), (94, 534), (117, 647), (123, 557), (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 105; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            106 => {
                if let Some(next) = advance_map(&[
                    (33, 508), (34, 624), (35, 678), (38, 536), (39, 604), (40, 506), (42, 524), (43, 521),
                    (45, 514), (46, 391), (47, 139), (48, 586), (59, 553), (64, 610), (76, 643), (85, 645),
                    (91, 555), (92, 87), (94, 534), (117, 647), (123, 557), (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 106; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            107 => {
                if let Some(next) = advance_map(&[
                    (33, 508), (34, 624), (35, 678), (38, 536), (39, 604), (40, 506), (42, 524), (43, 521),
                    (45, 514), (46, 391), (47, 139), (48, 586), (59, 553), (64, 614), (76, 643), (85, 645),
                    (91, 555), (92, 10), (94, 534), (117, 647), (123, 557), (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 107; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            108 => {
                if let Some(next) = advance_map(&[
                    (33, 508), (34, 624), (35, 679), (38, 536), (39, 604), (40, 506), (42, 524), (43, 521),
                    (44, 427), (45, 514), (46, 391), (47, 139), (48, 586), (58, 150), (59, 553), (64, 614),
                    (76, 643), (85, 645), (91, 555), (92, 8), (93, 556), (94, 534), (117, 647), (123, 557),
                    (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 108; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            109 => {
                if let Some(next) = advance_map(&[
                    (33, 508), (34, 624), (35, 679), (38, 536), (39, 604), (40, 506), (42, 524), (43, 521),
                    (45, 514), (46, 391), (47, 139), (48, 586), (59, 553), (64, 610), (76, 643), (85, 645),
                    (91, 555), (92, 91), (94, 534), (117, 647), (123, 557), (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 109; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            110 => {
                if let Some(next) = advance_map(&[
                    (33, 508), (34, 624), (38, 536), (39, 604), (40, 506), (41, 428), (42, 524), (43, 521),
                    (44, 427), (45, 514), (46, 391), (47, 139), (48, 586), (58, 562), (59, 553), (62, 541),
                    (64, 617), (76, 643), (85, 645), (91, 555), (92, 20), (94, 534), (117, 647), (123, 557),
                    (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 110; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            111 => {
                if let Some(next) = advance_map(&[
                    (33, 508), (34, 624), (38, 536), (39, 604), (40, 506), (41, 428), (42, 524), (43, 521),
                    (44, 427), (45, 514), (46, 579), (47, 139), (48, 586), (58, 562), (59, 553), (64, 621),
                    (76, 643), (85, 645), (91, 555), (92, 16), (93, 556), (94, 534), (117, 647), (123, 557),
                    (125, 558), (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 111; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            112 => {
                if let Some(next) = advance_map(&[
                    (33, 508), (34, 624), (38, 536), (39, 604), (40, 506), (41, 428), (42, 524), (43, 521),
                    (45, 514), (46, 391), (47, 139), (48, 586), (60, 546), (64, 619), (76, 643), (85, 645),
                    (91, 555), (92, 34), (94, 534), (117, 647), (123, 557), (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 112; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            113 => {
                if let Some(next) = advance_map(&[
                    (33, 508), (34, 624), (38, 536), (39, 604), (40, 506), (42, 524), (43, 521), (45, 514),
                    (46, 391), (47, 139), (48, 584), (64, 621), (76, 643), (85, 645), (91, 555), (92, 75),
                    (94, 534), (117, 647), (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 113; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 585; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            114 => {
                if let Some(next) = advance_map(&[
                    (33, 508), (34, 624), (38, 536), (39, 604), (40, 506), (42, 524), (43, 521), (45, 514),
                    (46, 391), (47, 139), (48, 586), (59, 553), (64, 613), (76, 643), (85, 645), (91, 555),
                    (92, 22), (94, 534), (117, 647), (123, 557), (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 114; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            115 => {
                if let Some(next) = advance_map(&[
                    (33, 508), (34, 624), (38, 536), (39, 604), (40, 506), (42, 524), (43, 521), (45, 514),
                    (46, 391), (47, 139), (48, 586), (64, 620), (76, 643), (85, 645), (91, 555), (92, 18),
                    (94, 534), (117, 647), (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 115; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            116 => {
                if let Some(next) = advance_map(&[
                    (33, 508), (39, 604), (40, 506), (41, 428), (43, 523), (45, 517), (46, 391), (47, 139),
                    (48, 586), (60, 153), (76, 651), (85, 652), (92, 48), (117, 653), (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 116; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            117 => {
                if let Some(next) = advance_map(&[
                    (33, 151), (34, 624), (35, 270), (37, 529), (38, 538), (40, 506), (41, 428), (42, 525),
                    (43, 522), (44, 427), (45, 515), (46, 577), (47, 527), (58, 562), (59, 553), (60, 547),
                    (61, 560), (62, 542), (63, 563), (64, 609), (76, 644), (85, 646), (91, 555), (92, 36),
                    (93, 556), (94, 535), (117, 648), (124, 532), (125, 558),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 117; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            118 => {
                if let Some(next) = advance_map(&[
                    (33, 151), (34, 624), (35, 270), (37, 528), (38, 537), (40, 506), (41, 428), (42, 524),
                    (43, 519), (44, 427), (45, 516), (46, 577), (47, 526), (58, 562), (59, 553), (60, 548),
                    (61, 152), (62, 543), (63, 563), (64, 609), (76, 644), (85, 646), (91, 555), (92, 79),
                    (93, 556), (94, 534), (117, 648), (124, 533), (125, 558),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 118; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            119 => {
                if let Some(next) = advance_map(&[
                    (33, 151), (34, 624), (37, 529), (38, 538), (40, 506), (41, 428), (42, 525), (43, 522),
                    (44, 427), (45, 515), (46, 577), (47, 527), (58, 731), (60, 576), (61, 560), (62, 542),
                    (63, 563), (64, 609), (76, 644), (85, 646), (91, 555), (92, 46), (94, 535), (117, 648),
                    (124, 532),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 121; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 675; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            120 => {
                if let Some(next) = advance_map(&[
                    (33, 151), (34, 624), (37, 529), (38, 538), (40, 506), (41, 428), (42, 525), (43, 522),
                    (44, 427), (45, 515), (46, 577), (47, 527), (59, 553), (60, 547), (61, 560), (62, 542),
                    (63, 563), (64, 622), (76, 644), (85, 646), (91, 555), (92, 30), (94, 535), (117, 648),
                    (123, 557), (124, 532),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 120; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            121 => {
                if let Some(next) = advance_map(&[
                    (33, 151), (34, 624), (37, 529), (38, 538), (40, 506), (41, 428), (42, 525), (43, 522),
                    (44, 427), (45, 515), (46, 577), (47, 527), (60, 547), (61, 560), (62, 542), (63, 563),
                    (64, 609), (76, 644), (85, 646), (91, 555), (92, 46), (94, 535), (117, 648), (124, 532),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 121; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 675; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            122 => {
                if let Some(next) = advance_map(&[
                    (33, 151), (34, 624), (37, 529), (38, 538), (40, 506), (42, 525), (43, 522), (44, 427),
                    (45, 515), (46, 577), (47, 527), (58, 150), (60, 547), (61, 560), (62, 542), (63, 563),
                    (64, 609), (76, 644), (85, 646), (91, 555), (92, 44), (93, 556), (94, 535), (117, 648),
                    (124, 532),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 122; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            123 => {
                if let Some(next) = advance_map(&[
                    (33, 151), (34, 624), (37, 528), (38, 537), (40, 506), (42, 524), (43, 519), (44, 427),
                    (45, 516), (46, 577), (47, 526), (59, 553), (60, 548), (61, 152), (62, 543), (63, 563),
                    (64, 622), (76, 644), (85, 646), (91, 555), (92, 38), (94, 534), (117, 648), (123, 557),
                    (124, 533),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 123; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            124 => {
                if let Some(next) = advance_map(&[
                    (33, 151), (35, 223), (37, 529), (38, 538), (40, 506), (41, 428), (42, 525), (43, 522),
                    (44, 427), (45, 515), (46, 577), (47, 527), (58, 562), (59, 553), (60, 547), (61, 560),
                    (62, 542), (63, 563), (64, 162), (91, 555), (92, 32), (93, 556), (94, 535), (123, 557),
                    (124, 532), (125, 558),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 124; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            125 => {
                if let Some(next) = advance_map(&[
                    (33, 151), (35, 196), (37, 528), (38, 537), (40, 506), (41, 428), (42, 524), (43, 519),
                    (44, 427), (45, 516), (46, 577), (47, 526), (58, 562), (59, 553), (60, 548), (61, 152),
                    (62, 543), (63, 563), (64, 160), (91, 555), (92, 77), (93, 556), (94, 534), (123, 557),
                    (124, 533), (125, 558),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 125; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            126 => {
                if let Some(next) = advance_map(&[
                    (33, 151), (35, 200), (37, 528), (38, 537), (40, 506), (41, 428), (42, 524), (43, 518),
                    (44, 427), (45, 511), (47, 526), (58, 731), (59, 553), (60, 548), (61, 152), (62, 543),
                    (64, 203), (91, 555), (92, 26), (94, 534), (124, 533),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 127; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            127 => {
                if let Some(next) = advance_map(&[
                    (33, 151), (35, 200), (37, 528), (38, 537), (40, 506), (41, 428), (42, 524), (43, 518),
                    (44, 427), (45, 511), (47, 526), (59, 553), (60, 548), (61, 152), (62, 543), (64, 203),
                    (91, 555), (92, 26), (94, 534), (124, 533),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 127; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            128 => {
                if let Some(next) = advance_map(&[
                    (33, 151), (35, 247), (37, 528), (38, 537), (40, 506), (41, 428), (42, 524), (43, 519),
                    (44, 427), (45, 516), (46, 577), (47, 526), (58, 562), (59, 553), (60, 548), (61, 560),
                    (62, 543), (63, 563), (64, 159), (91, 555), (92, 85), (93, 556), (94, 534), (123, 557),
                    (124, 533), (125, 558),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 128; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            129 => {
                if let Some(next) = advance_map(&[
                    (34, 624), (47, 139), (60, 153), (64, 609), (76, 644), (85, 646), (92, 53), (117, 648),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 129; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            130 => {
                if let Some(next) = advance_map(&[
                    (34, 624), (47, 139), (64, 623), (76, 644), (85, 646), (91, 555), (92, 42), (117, 648),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 130; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            131 => {
                if lookahead == 34 { state = 624; lexer.advance(false); continue; }
                if lookahead == 47 { state = 139; lexer.advance(false); continue; }
                if lookahead == 92 { state = 60; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 131; lexer.advance(true); continue; }
                return result;
            }
            132 => {
                if let Some(next) = advance_map(&[
                    (35, 202), (40, 506), (41, 428), (42, 524), (43, 518), (44, 427), (45, 511), (46, 146),
                    (47, 139), (58, 562), (59, 553), (60, 546), (61, 559), (62, 541), (64, 158), (91, 555),
                    (92, 24), (93, 556), (94, 534), (123, 557), (125, 558),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 132; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 675; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            133 => {
                if let Some(next) = advance_map(&[
                    (35, 198), (40, 506), (42, 524), (43, 518), (45, 511), (47, 139), (59, 553), (64, 208),
                    (91, 555), (92, 28), (94, 534),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 133; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            134 => {
                if let Some(next) = advance_map(&[
                    (35, 273), (40, 506), (41, 428), (42, 524), (44, 427), (46, 391), (47, 139), (48, 586),
                    (58, 562), (59, 553), (60, 546), (61, 559), (62, 541), (64, 161), (91, 555), (92, 40),
                    (94, 534), (123, 557), (125, 558), (43, 143), (45, 143),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 134; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            135 => {
                if lookahead == 39 { state = 604; lexer.advance(false); continue; }
                if lookahead == 47 { state = 139; lexer.advance(false); continue; }
                if lookahead == 92 { state = 60; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 135; lexer.advance(true); continue; }
                return result;
            }
            136 => {
                if lookahead == 40 { state = 506; lexer.advance(false); continue; }
                if lookahead == 46 { state = 391; lexer.advance(false); continue; }
                if lookahead == 47 { state = 139; lexer.advance(false); continue; }
                if lookahead == 48 { state = 586; lexer.advance(false); continue; }
                if lookahead == 92 { state = 63; lexer.advance(true); continue; }
                if lookahead == 43 || lookahead == 45 { state = 143; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 136; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                return result;
            }
            137 => {
                if lookahead == 40 { state = 192; lexer.advance(false); continue; }
                if lookahead == 47 { state = 139; lexer.advance(false); continue; }
                if lookahead == 92 { state = 65; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 137; lexer.advance(true); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            138 => {
                if lookahead == 41 { state = 694; lexer.advance(false); continue; }
                return result;
            }
            139 => {
                if lookahead == 42 { state = 141; lexer.advance(false); continue; }
                if lookahead == 47 { state = 670; lexer.advance(false); continue; }
                return result;
            }
            140 => {
                if lookahead == 42 { state = 140; lexer.advance(false); continue; }
                if lookahead == 47 { state = 657; lexer.advance(false); continue; }
                if lookahead != 0 { state = 141; lexer.advance(false); continue; }
                return result;
            }
            141 => {
                if lookahead == 42 { state = 140; lexer.advance(false); continue; }
                if lookahead != 0 { state = 141; lexer.advance(false); continue; }
                return result;
            }
            142 => {
                if lookahead == 42 { state = 140; lexer.advance(false); continue; }
                if lookahead != 0 { state = 449; lexer.advance(false); continue; }
                return result;
            }
            143 => {
                if lookahead == 46 { state = 391; lexer.advance(false); continue; }
                if lookahead == 48 { state = 586; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                return result;
            }
            144 => {
                if lookahead == 46 { state = 391; lexer.advance(false); continue; }
                if lookahead == 48 { state = 582; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 583; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 595; lexer.advance(false); continue; }
                return result;
            }
            145 => {
                if lookahead == 46 { state = 391; lexer.advance(false); continue; }
                if lookahead == 48 { state = 587; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                return result;
            }
            146 => {
                if lookahead == 46 { state = 148; lexer.advance(false); continue; }
                return result;
            }
            147 => {
                if lookahead == 46 { state = 148; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 581; lexer.advance(false); continue; }
                return result;
            }
            148 => {
                if lookahead == 46 { state = 426; lexer.advance(false); continue; }
                return result;
            }
            149 => {
                if lookahead == 47 { state = 459; lexer.advance(false); continue; }
                if lookahead == 92 { state = 453; lexer.advance(false); continue; }
                if lookahead == 160 { state = 455; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 149; lexer.advance(true); continue; }
                if lookahead != 0 { state = 462; lexer.advance(false); continue; }
                return result;
            }
            150 => {
                if lookahead == 58 { state = 554; lexer.advance(false); continue; }
                return result;
            }
            151 => {
                if lookahead == 61 { state = 540; lexer.advance(false); continue; }
                return result;
            }
            152 => {
                if lookahead == 61 { state = 539; lexer.advance(false); continue; }
                return result;
            }
            153 => {
                if lookahead == 62 { state = 641; lexer.advance(false); continue; }
                if lookahead == 92 { state = 154; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 153; lexer.advance(false); continue; }
                return result;
            }
            154 => {
                if lookahead == 62 { state = 642; lexer.advance(false); continue; }
                if lookahead == 92 { state = 154; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 153; lexer.advance(false); continue; }
                return result;
            }
            155 => {
                if lookahead == 85 { state = 412; lexer.advance(false); continue; }
                if lookahead == 117 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            156 => {
                if lookahead == 95 { state = 177; lexer.advance(false); continue; }
                return result;
            }
            157 => {
                if lookahead == 97 { state = 182; lexer.advance(false); continue; }
                return result;
            }
            158 => {
                if let Some(next) = advance_map(&[
                    (97, 376), (99, 165), (100, 238), (101, 313), (102, 282), (105, 305), (111, 340), (112, 351),
                    (114, 222), (115, 387),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            159 => {
                if lookahead == 97 { state = 376; lexer.advance(false); continue; }
                if lookahead == 99 { state = 165; lexer.advance(false); continue; }
                if lookahead == 102 { state = 282; lexer.advance(false); continue; }
                return result;
            }
            160 => {
                if lookahead == 97 { state = 376; lexer.advance(false); continue; }
                if lookahead == 100 { state = 237; lexer.advance(false); continue; }
                if lookahead == 105 { state = 305; lexer.advance(false); continue; }
                if lookahead == 112 { state = 343; lexer.advance(false); continue; }
                return result;
            }
            161 => {
                if lookahead == 97 { state = 376; lexer.advance(false); continue; }
                if lookahead == 105 { state = 305; lexer.advance(false); continue; }
                if lookahead == 112 { state = 176; lexer.advance(false); continue; }
                return result;
            }
            162 => {
                if lookahead == 97 { state = 376; lexer.advance(false); continue; }
                if lookahead == 112 { state = 175; lexer.advance(false); continue; }
                return result;
            }
            163 => {
                if lookahead == 97 { state = 259; lexer.advance(false); continue; }
                return result;
            }
            164 => {
                if lookahead == 97 { state = 265; lexer.advance(false); continue; }
                return result;
            }
            165 => {
                if lookahead == 97 { state = 362; lexer.advance(false); continue; }
                return result;
            }
            166 => {
                if lookahead == 97 { state = 362; lexer.advance(false); continue; }
                if lookahead == 111 { state = 303; lexer.advance(false); continue; }
                return result;
            }
            167 => {
                if lookahead == 97 { state = 304; lexer.advance(false); continue; }
                return result;
            }
            168 => {
                if lookahead == 97 { state = 359; lexer.advance(false); continue; }
                return result;
            }
            169 => {
                if lookahead == 97 { state = 293; lexer.advance(false); continue; }
                return result;
            }
            170 => {
                if lookahead == 97 { state = 354; lexer.advance(false); continue; }
                return result;
            }
            171 => {
                if lookahead == 97 { state = 366; lexer.advance(false); continue; }
                return result;
            }
            172 => {
                if lookahead == 97 { state = 191; lexer.advance(false); continue; }
                return result;
            }
            173 => {
                if lookahead == 97 { state = 358; lexer.advance(false); continue; }
                return result;
            }
            174 => {
                if lookahead == 97 { state = 369; lexer.advance(false); continue; }
                return result;
            }
            175 => {
                if lookahead == 97 { state = 185; lexer.advance(false); continue; }
                if lookahead == 114 { state = 269; lexer.advance(false); continue; }
                if lookahead == 117 { state = 180; lexer.advance(false); continue; }
                return result;
            }
            176 => {
                if lookahead == 97 { state = 185; lexer.advance(false); continue; }
                if lookahead == 114 { state = 268; lexer.advance(false); continue; }
                if lookahead == 117 { state = 180; lexer.advance(false); continue; }
                return result;
            }
            177 => {
                if lookahead == 97 { state = 296; lexer.advance(false); continue; }
                return result;
            }
            178 => {
                if lookahead == 97 { state = 287; lexer.advance(false); continue; }
                return result;
            }
            179 => {
                if lookahead == 97 { state = 372; lexer.advance(false); continue; }
                return result;
            }
            180 => {
                if lookahead == 98 { state = 301; lexer.advance(false); continue; }
                return result;
            }
            181 => {
                if lookahead == 98 { state = 272; lexer.advance(false); continue; }
                return result;
            }
            182 => {
                if lookahead == 98 { state = 298; lexer.advance(false); continue; }
                return result;
            }
            183 => {
                if lookahead == 99 { state = 260; lexer.advance(false); continue; }
                return result;
            }
            184 => {
                if lookahead == 99 { state = 693; lexer.advance(false); continue; }
                return result;
            }
            185 => {
                if lookahead == 99 { state = 284; lexer.advance(false); continue; }
                return result;
            }
            186 => {
                if lookahead == 99 { state = 688; lexer.advance(false); continue; }
                return result;
            }
            187 => {
                if lookahead == 99 { state = 263; lexer.advance(false); continue; }
                return result;
            }
            188 => {
                if lookahead == 99 { state = 263; lexer.advance(false); continue; }
                if lookahead == 116 { state = 261; lexer.advance(false); continue; }
                return result;
            }
            189 => {
                if lookahead == 99 { state = 320; lexer.advance(false); continue; }
                return result;
            }
            190 => {
                if lookahead == 99 { state = 326; lexer.advance(false); continue; }
                return result;
            }
            191 => {
                if lookahead == 99 { state = 220; lexer.advance(false); continue; }
                return result;
            }
            192 => {
                if lookahead == 99 { state = 299; lexer.advance(false); continue; }
                return result;
            }
            193 => {
                if lookahead == 99 { state = 373; lexer.advance(false); continue; }
                return result;
            }
            194 => {
                if lookahead == 99 { state = 370; lexer.advance(false); continue; }
                return result;
            }
            195 => {
                if lookahead == 100 { state = 470; lexer.advance(false); continue; }
                if lookahead == 101 { state = 494; lexer.advance(false); continue; }
                if lookahead == 105 { state = 480; lexer.advance(false); continue; }
                if lookahead == 117 { state = 498; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 195; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            196 => {
                if lookahead == 100 { state = 470; lexer.advance(false); continue; }
                if lookahead == 101 { state = 494; lexer.advance(false); continue; }
                if lookahead == 105 { state = 481; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 196; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            197 => {
                if lookahead == 100 { state = 470; lexer.advance(false); continue; }
                if lookahead == 101 { state = 497; lexer.advance(false); continue; }
                if lookahead == 105 { state = 480; lexer.advance(false); continue; }
                if lookahead == 117 { state = 498; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 197; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            198 => {
                if lookahead == 100 { state = 470; lexer.advance(false); continue; }
                if lookahead == 101 { state = 497; lexer.advance(false); continue; }
                if lookahead == 105 { state = 481; lexer.advance(false); continue; }
                if lookahead == 117 { state = 498; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 198; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            199 => {
                if lookahead == 100 { state = 470; lexer.advance(false); continue; }
                if lookahead == 101 { state = 496; lexer.advance(false); continue; }
                if lookahead == 105 { state = 480; lexer.advance(false); continue; }
                if lookahead == 117 { state = 498; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 199; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            200 => {
                if lookahead == 100 { state = 470; lexer.advance(false); continue; }
                if lookahead == 101 { state = 496; lexer.advance(false); continue; }
                if lookahead == 105 { state = 481; lexer.advance(false); continue; }
                if lookahead == 117 { state = 498; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 200; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            201 => {
                if lookahead == 100 { state = 470; lexer.advance(false); continue; }
                if lookahead == 105 { state = 480; lexer.advance(false); continue; }
                if lookahead == 117 { state = 498; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 201; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            202 => {
                if lookahead == 100 { state = 470; lexer.advance(false); continue; }
                if lookahead == 105 { state = 481; lexer.advance(false); continue; }
                if lookahead == 117 { state = 498; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 202; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            203 => {
                if lookahead == 100 { state = 386; lexer.advance(false); continue; }
                if lookahead == 112 { state = 352; lexer.advance(false); continue; }
                if lookahead == 115 { state = 387; lexer.advance(false); continue; }
                return result;
            }
            204 => {
                if lookahead == 100 { state = 729; lexer.advance(false); continue; }
                return result;
            }
            205 => {
                if lookahead == 100 { state = 682; lexer.advance(false); continue; }
                return result;
            }
            206 => {
                if lookahead == 100 { state = 691; lexer.advance(false); continue; }
                return result;
            }
            207 => {
                if lookahead == 100 { state = 686; lexer.advance(false); continue; }
                return result;
            }
            208 => {
                if lookahead == 100 { state = 238; lexer.advance(false); continue; }
                if lookahead == 112 { state = 352; lexer.advance(false); continue; }
                if lookahead == 115 { state = 387; lexer.advance(false); continue; }
                return result;
            }
            209 => {
                if lookahead == 100 { state = 217; lexer.advance(false); continue; }
                return result;
            }
            210 => {
                if lookahead == 100 { state = 266; lexer.advance(false); continue; }
                return result;
            }
            211 => {
                if lookahead == 100 { state = 233; lexer.advance(false); continue; }
                if lookahead == 110 { state = 212; lexer.advance(false); continue; }
                return result;
            }
            212 => {
                if lookahead == 100 { state = 235; lexer.advance(false); continue; }
                return result;
            }
            213 => {
                if lookahead == 100 { state = 244; lexer.advance(false); continue; }
                return result;
            }
            214 => {
                if lookahead == 101 { state = 289; lexer.advance(false); continue; }
                return result;
            }
            215 => {
                if lookahead == 101 { state = 289; lexer.advance(false); continue; }
                if lookahead == 121 { state = 308; lexer.advance(false); continue; }
                return result;
            }
            216 => {
                if lookahead == 101 { state = 289; lexer.advance(false); continue; }
                if lookahead == 121 { state = 315; lexer.advance(false); continue; }
                return result;
            }
            217 => {
                if lookahead == 101 { state = 728; lexer.advance(false); continue; }
                return result;
            }
            218 => {
                if lookahead == 101 { state = 357; lexer.advance(false); continue; }
                return result;
            }
            219 => {
                if lookahead == 101 { state = 719; lexer.advance(false); continue; }
                return result;
            }
            220 => {
                if lookahead == 101 { state = 683; lexer.advance(false); continue; }
                return result;
            }
            221 => {
                if lookahead == 101 { state = 692; lexer.advance(false); continue; }
                return result;
            }
            222 => {
                if lookahead == 101 { state = 342; lexer.advance(false); continue; }
                return result;
            }
            223 => {
                if lookahead == 101 { state = 288; lexer.advance(false); continue; }
                if lookahead == 105 { state = 252; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 223; lexer.advance(false); continue; }
                return result;
            }
            224 => {
                if lookahead == 101 { state = 438; lexer.advance(false); continue; }
                return result;
            }
            225 => {
                if lookahead == 101 { state = 687; lexer.advance(false); continue; }
                return result;
            }
            226 => {
                if lookahead == 101 { state = 685; lexer.advance(false); continue; }
                return result;
            }
            227 => {
                if lookahead == 101 { state = 204; lexer.advance(false); continue; }
                return result;
            }
            228 => {
                if lookahead == 101 { state = 306; lexer.advance(false); continue; }
                return result;
            }
            229 => {
                if lookahead == 101 { state = 194; lexer.advance(false); continue; }
                return result;
            }
            230 => {
                if lookahead == 101 { state = 336; lexer.advance(false); continue; }
                return result;
            }
            231 => {
                if lookahead == 101 { state = 314; lexer.advance(false); continue; }
                return result;
            }
            232 => {
                if lookahead == 101 { state = 344; lexer.advance(false); continue; }
                return result;
            }
            233 => {
                if lookahead == 101 { state = 250; lexer.advance(false); continue; }
                return result;
            }
            234 => {
                if lookahead == 101 { state = 206; lexer.advance(false); continue; }
                return result;
            }
            235 => {
                if lookahead == 101 { state = 251; lexer.advance(false); continue; }
                return result;
            }
            236 => {
                if lookahead == 101 { state = 207; lexer.advance(false); continue; }
                return result;
            }
            237 => {
                if lookahead == 101 { state = 257; lexer.advance(false); continue; }
                return result;
            }
            238 => {
                if lookahead == 101 { state = 257; lexer.advance(false); continue; }
                if lookahead == 121 { state = 310; lexer.advance(false); continue; }
                return result;
            }
            239 => {
                if lookahead == 101 { state = 312; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 239; lexer.advance(false); continue; }
                return result;
            }
            240 => {
                if lookahead == 101 { state = 168; lexer.advance(false); continue; }
                return result;
            }
            241 => {
                if lookahead == 101 { state = 193; lexer.advance(false); continue; }
                return result;
            }
            242 => {
                if lookahead == 101 { state = 193; lexer.advance(false); continue; }
                if lookahead == 111 { state = 190; lexer.advance(false); continue; }
                return result;
            }
            243 => {
                if lookahead == 101 { state = 254; lexer.advance(false); continue; }
                return result;
            }
            244 => {
                if lookahead == 101 { state = 255; lexer.advance(false); continue; }
                return result;
            }
            245 => {
                if lookahead == 101 { state = 297; lexer.advance(false); continue; }
                return result;
            }
            246 => {
                if lookahead == 101 { state = 350; lexer.advance(false); continue; }
                return result;
            }
            247 => {
                if lookahead == 101 { state = 300; lexer.advance(false); continue; }
                if lookahead == 105 { state = 249; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 247; lexer.advance(false); continue; }
                return result;
            }
            248 => {
                if lookahead == 102 { state = 432; lexer.advance(false); continue; }
                return result;
            }
            249 => {
                if lookahead == 102 { state = 211; lexer.advance(false); continue; }
                return result;
            }
            250 => {
                if lookahead == 102 { state = 434; lexer.advance(false); continue; }
                return result;
            }
            251 => {
                if lookahead == 102 { state = 436; lexer.advance(false); continue; }
                return result;
            }
            252 => {
                if lookahead == 102 { state = 429; lexer.advance(false); continue; }
                return result;
            }
            253 => {
                if lookahead == 102 { state = 441; lexer.advance(false); continue; }
                return result;
            }
            254 => {
                if lookahead == 102 { state = 444; lexer.advance(false); continue; }
                return result;
            }
            255 => {
                if lookahead == 102 { state = 446; lexer.advance(false); continue; }
                return result;
            }
            256 => {
                if lookahead == 102 { state = 440; lexer.advance(false); continue; }
                return result;
            }
            257 => {
                if lookahead == 102 { state = 355; lexer.advance(false); continue; }
                return result;
            }
            258 => {
                if lookahead == 102 { state = 172; lexer.advance(false); continue; }
                return result;
            }
            259 => {
                if lookahead == 103 { state = 225; lexer.advance(false); continue; }
                return result;
            }
            260 => {
                if lookahead == 104 { state = 697; lexer.advance(false); continue; }
                return result;
            }
            261 => {
                if lookahead == 104 { state = 218; lexer.advance(false); continue; }
                return result;
            }
            262 => {
                if lookahead == 104 { state = 346; lexer.advance(false); continue; }
                if lookahead == 114 { state = 382; lexer.advance(false); continue; }
                return result;
            }
            263 => {
                if lookahead == 104 { state = 349; lexer.advance(false); continue; }
                return result;
            }
            264 => {
                if lookahead == 105 { state = 388; lexer.advance(false); continue; }
                return result;
            }
            265 => {
                if lookahead == 105 { state = 294; lexer.advance(false); continue; }
                return result;
            }
            266 => {
                if lookahead == 105 { state = 248; lexer.advance(false); continue; }
                return result;
            }
            267 => {
                if lookahead == 105 { state = 181; lexer.advance(false); continue; }
                return result;
            }
            268 => {
                if lookahead == 105 { state = 380; lexer.advance(false); continue; }
                if lookahead == 111 { state = 338; lexer.advance(false); continue; }
                return result;
            }
            269 => {
                if lookahead == 105 { state = 380; lexer.advance(false); continue; }
                if lookahead == 111 { state = 374; lexer.advance(false); continue; }
                return result;
            }
            270 => {
                if lookahead == 105 { state = 249; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 270; lexer.advance(false); continue; }
                return result;
            }
            271 => {
                if lookahead == 105 { state = 184; lexer.advance(false); continue; }
                return result;
            }
            272 => {
                if lookahead == 105 { state = 295; lexer.advance(false); continue; }
                return result;
            }
            273 => {
                if lookahead == 105 { state = 252; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 273; lexer.advance(false); continue; }
                return result;
            }
            274 => {
                if lookahead == 105 { state = 365; lexer.advance(false); continue; }
                return result;
            }
            275 => {
                if lookahead == 105 { state = 253; lexer.advance(false); continue; }
                if lookahead == 115 { state = 224; lexer.advance(false); continue; }
                return result;
            }
            276 => {
                if lookahead == 105 { state = 170; lexer.advance(false); continue; }
                return result;
            }
            277 => {
                if lookahead == 105 { state = 186; lexer.advance(false); continue; }
                return result;
            }
            278 => {
                if lookahead == 105 { state = 327; lexer.advance(false); continue; }
                return result;
            }
            279 => {
                if lookahead == 105 { state = 256; lexer.advance(false); continue; }
                if lookahead == 115 { state = 224; lexer.advance(false); continue; }
                return result;
            }
            280 => {
                if lookahead == 105 { state = 333; lexer.advance(false); continue; }
                return result;
            }
            281 => {
                if lookahead == 105 { state = 389; lexer.advance(false); continue; }
                return result;
            }
            282 => {
                if lookahead == 105 { state = 311; lexer.advance(false); continue; }
                return result;
            }
            283 => {
                if lookahead == 105 { state = 353; lexer.advance(false); continue; }
                return result;
            }
            284 => {
                if lookahead == 107 { state = 163; lexer.advance(false); continue; }
                return result;
            }
            285 => {
                if lookahead == 108 { state = 681; lexer.advance(false); continue; }
                return result;
            }
            286 => {
                if lookahead == 108 { state = 561; lexer.advance(false); continue; }
                return result;
            }
            287 => {
                if lookahead == 108 { state = 690; lexer.advance(false); continue; }
                return result;
            }
            288 => {
                if lookahead == 108 { state = 275; lexer.advance(false); continue; }
                if lookahead == 110 { state = 210; lexer.advance(false); continue; }
                return result;
            }
            289 => {
                if lookahead == 108 { state = 229; lexer.advance(false); continue; }
                return result;
            }
            290 => {
                if lookahead == 108 { state = 383; lexer.advance(false); continue; }
                return result;
            }
            291 => {
                if lookahead == 108 { state = 228; lexer.advance(false); continue; }
                return result;
            }
            292 => {
                if lookahead == 108 { state = 228; lexer.advance(false); continue; }
                if lookahead == 111 { state = 348; lexer.advance(false); continue; }
                return result;
            }
            293 => {
                if lookahead == 108 { state = 290; lexer.advance(false); continue; }
                return result;
            }
            294 => {
                if lookahead == 108 { state = 157; lexer.advance(false); continue; }
                return result;
            }
            295 => {
                if lookahead == 108 { state = 274; lexer.advance(false); continue; }
                return result;
            }
            296 => {
                if lookahead == 108 { state = 276; lexer.advance(false); continue; }
                return result;
            }
            297 => {
                if lookahead == 108 { state = 240; lexer.advance(false); continue; }
                return result;
            }
            298 => {
                if lookahead == 108 { state = 219; lexer.advance(false); continue; }
                return result;
            }
            299 => {
                if lookahead == 108 { state = 173; lexer.advance(false); continue; }
                return result;
            }
            300 => {
                if lookahead == 108 { state = 279; lexer.advance(false); continue; }
                if lookahead == 110 { state = 210; lexer.advance(false); continue; }
                return result;
            }
            301 => {
                if lookahead == 108 { state = 277; lexer.advance(false); continue; }
                return result;
            }
            302 => {
                if lookahead == 109 { state = 334; lexer.advance(false); continue; }
                if lookahead == 110 { state = 360; lexer.advance(false); continue; }
                return result;
            }
            303 => {
                if lookahead == 109 { state = 335; lexer.advance(false); continue; }
                return result;
            }
            304 => {
                if lookahead == 109 { state = 271; lexer.advance(false); continue; }
                return result;
            }
            305 => {
                if lookahead == 109 { state = 341; lexer.advance(false); continue; }
                if lookahead == 110 { state = 360; lexer.advance(false); continue; }
                return result;
            }
            306 => {
                if lookahead == 109 { state = 231; lexer.advance(false); continue; }
                return result;
            }
            307 => {
                if lookahead == 110 { state = 189; lexer.advance(false); continue; }
                return result;
            }
            308 => {
                if lookahead == 110 { state = 188; lexer.advance(false); continue; }
                return result;
            }
            309 => {
                if lookahead == 110 { state = 684; lexer.advance(false); continue; }
                return result;
            }
            310 => {
                if lookahead == 110 { state = 167; lexer.advance(false); continue; }
                return result;
            }
            311 => {
                if lookahead == 110 { state = 169; lexer.advance(false); continue; }
                return result;
            }
            312 => {
                if lookahead == 110 { state = 210; lexer.advance(false); continue; }
                return result;
            }
            313 => {
                if lookahead == 110 { state = 205; lexer.advance(false); continue; }
                return result;
            }
            314 => {
                if lookahead == 110 { state = 371; lexer.advance(false); continue; }
                return result;
            }
            315 => {
                if lookahead == 110 { state = 187; lexer.advance(false); continue; }
                return result;
            }
            316 => {
                if lookahead == 110 { state = 364; lexer.advance(false); continue; }
                return result;
            }
            317 => {
                if lookahead == 110 { state = 281; lexer.advance(false); continue; }
                return result;
            }
            318 => {
                if lookahead == 110 { state = 178; lexer.advance(false); continue; }
                return result;
            }
            319 => {
                if lookahead == 111 { state = 303; lexer.advance(false); continue; }
                return result;
            }
            320 => {
                if lookahead == 111 { state = 209; lexer.advance(false); continue; }
                return result;
            }
            321 => {
                if lookahead == 111 { state = 381; lexer.advance(false); continue; }
                return result;
            }
            322 => {
                if lookahead == 111 { state = 339; lexer.advance(false); continue; }
                return result;
            }
            323 => {
                if lookahead == 111 { state = 347; lexer.advance(false); continue; }
                return result;
            }
            324 => {
                if lookahead == 111 { state = 317; lexer.advance(false); continue; }
                return result;
            }
            325 => {
                if lookahead == 111 { state = 345; lexer.advance(false); continue; }
                return result;
            }
            326 => {
                if lookahead == 111 { state = 285; lexer.advance(false); continue; }
                return result;
            }
            327 => {
                if lookahead == 111 { state = 309; lexer.advance(false); continue; }
                return result;
            }
            328 => {
                if lookahead == 111 { state = 337; lexer.advance(false); continue; }
                return result;
            }
            329 => {
                if lookahead == 111 { state = 286; lexer.advance(false); continue; }
                return result;
            }
            330 => {
                if lookahead == 111 { state = 329; lexer.advance(false); continue; }
                return result;
            }
            331 => {
                if lookahead == 111 { state = 368; lexer.advance(false); continue; }
                return result;
            }
            332 => {
                if lookahead == 111 { state = 190; lexer.advance(false); continue; }
                return result;
            }
            333 => {
                if lookahead == 111 { state = 318; lexer.advance(false); continue; }
                return result;
            }
            334 => {
                if lookahead == 112 { state = 292; lexer.advance(false); continue; }
                return result;
            }
            335 => {
                if lookahead == 112 { state = 171; lexer.advance(false); continue; }
                return result;
            }
            336 => {
                if lookahead == 112 { state = 330; lexer.advance(false); continue; }
                return result;
            }
            337 => {
                if lookahead == 112 { state = 246; lexer.advance(false); continue; }
                return result;
            }
            338 => {
                if lookahead == 112 { state = 246; lexer.advance(false); continue; }
                if lookahead == 116 { state = 242; lexer.advance(false); continue; }
                return result;
            }
            339 => {
                if lookahead == 112 { state = 246; lexer.advance(false); continue; }
                if lookahead == 116 { state = 332; lexer.advance(false); continue; }
                return result;
            }
            340 => {
                if lookahead == 112 { state = 375; lexer.advance(false); continue; }
                return result;
            }
            341 => {
                if lookahead == 112 { state = 291; lexer.advance(false); continue; }
                return result;
            }
            342 => {
                if lookahead == 113 { state = 378; lexer.advance(false); continue; }
                return result;
            }
            343 => {
                if lookahead == 114 { state = 331; lexer.advance(false); continue; }
                return result;
            }
            344 => {
                if lookahead == 114 { state = 258; lexer.advance(false); continue; }
                return result;
            }
            345 => {
                if lookahead == 114 { state = 700; lexer.advance(false); continue; }
                return result;
            }
            346 => {
                if lookahead == 114 { state = 321; lexer.advance(false); continue; }
                return result;
            }
            347 => {
                if lookahead == 114 { state = 245; lexer.advance(false); continue; }
                return result;
            }
            348 => {
                if lookahead == 114 { state = 361; lexer.advance(false); continue; }
                return result;
            }
            349 => {
                if lookahead == 114 { state = 324; lexer.advance(false); continue; }
                return result;
            }
            350 => {
                if lookahead == 114 { state = 367; lexer.advance(false); continue; }
                return result;
            }
            351 => {
                if lookahead == 114 { state = 322; lexer.advance(false); continue; }
                return result;
            }
            352 => {
                if lookahead == 114 { state = 328; lexer.advance(false); continue; }
                return result;
            }
            353 => {
                if lookahead == 114 { state = 234; lexer.advance(false); continue; }
                return result;
            }
            354 => {
                if lookahead == 115 { state = 689; lexer.advance(false); continue; }
                return result;
            }
            355 => {
                if lookahead == 115 { state = 730; lexer.advance(false); continue; }
                return result;
            }
            356 => {
                if lookahead == 115 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            357 => {
                if lookahead == 115 { state = 264; lexer.advance(false); continue; }
                return result;
            }
            358 => {
                if lookahead == 115 { state = 356; lexer.advance(false); continue; }
                return result;
            }
            359 => {
                if lookahead == 115 { state = 230; lexer.advance(false); continue; }
                return result;
            }
            360 => {
                if lookahead == 116 { state = 232; lexer.advance(false); continue; }
                return result;
            }
            361 => {
                if lookahead == 116 { state = 676; lexer.advance(false); continue; }
                return result;
            }
            362 => {
                if lookahead == 116 { state = 183; lexer.advance(false); continue; }
                return result;
            }
            363 => {
                if lookahead == 116 { state = 323; lexer.advance(false); continue; }
                return result;
            }
            364 => {
                if lookahead == 116 { state = 261; lexer.advance(false); continue; }
                return result;
            }
            365 => {
                if lookahead == 116 { state = 384; lexer.advance(false); continue; }
                return result;
            }
            366 => {
                if lookahead == 116 { state = 267; lexer.advance(false); continue; }
                return result;
            }
            367 => {
                if lookahead == 116 { state = 385; lexer.advance(false); continue; }
                return result;
            }
            368 => {
                if lookahead == 116 { state = 332; lexer.advance(false); continue; }
                return result;
            }
            369 => {
                if lookahead == 116 { state = 278; lexer.advance(false); continue; }
                return result;
            }
            370 => {
                if lookahead == 116 { state = 325; lexer.advance(false); continue; }
                return result;
            }
            371 => {
                if lookahead == 116 { state = 174; lexer.advance(false); continue; }
                return result;
            }
            372 => {
                if lookahead == 116 { state = 226; lexer.advance(false); continue; }
                return result;
            }
            373 => {
                if lookahead == 116 { state = 236; lexer.advance(false); continue; }
                return result;
            }
            374 => {
                if lookahead == 116 { state = 241; lexer.advance(false); continue; }
                return result;
            }
            375 => {
                if lookahead == 116 { state = 280; lexer.advance(false); continue; }
                return result;
            }
            376 => {
                if lookahead == 117 { state = 363; lexer.advance(false); continue; }
                return result;
            }
            377 => {
                if lookahead == 117 { state = 363; lexer.advance(false); continue; }
                if lookahead == 118 { state = 164; lexer.advance(false); continue; }
                return result;
            }
            378 => {
                if lookahead == 117 { state = 283; lexer.advance(false); continue; }
                return result;
            }
            379 => {
                if lookahead == 118 { state = 164; lexer.advance(false); continue; }
                return result;
            }
            380 => {
                if lookahead == 118 { state = 179; lexer.advance(false); continue; }
                return result;
            }
            381 => {
                if lookahead == 119 { state = 699; lexer.advance(false); continue; }
                return result;
            }
            382 => {
                if lookahead == 121 { state = 696; lexer.advance(false); continue; }
                return result;
            }
            383 => {
                if lookahead == 121 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            384 => {
                if lookahead == 121 { state = 156; lexer.advance(false); continue; }
                return result;
            }
            385 => {
                if lookahead == 121 { state = 695; lexer.advance(false); continue; }
                return result;
            }
            386 => {
                if lookahead == 121 { state = 310; lexer.advance(false); continue; }
                return result;
            }
            387 => {
                if lookahead == 121 { state = 316; lexer.advance(false); continue; }
                return result;
            }
            388 => {
                if lookahead == 122 { state = 221; lexer.advance(false); continue; }
                return result;
            }
            389 => {
                if lookahead == 122 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            390 => {
                if 48 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                return result;
            }
            391 => {
                if 48 <= lookahead && lookahead <= 57 { state = 581; lexer.advance(false); continue; }
                return result;
            }
            392 => {
                if 48 <= lookahead && lookahead <= 57 { state = 675; lexer.advance(false); continue; }
                return result;
            }
            393 => {
                if 48 <= lookahead && lookahead <= 57 { state = 583; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 595; lexer.advance(false); continue; }
                return result;
            }
            394 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 655; lexer.advance(false); continue; }
                return result;
            }
            395 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 592; lexer.advance(false); continue; }
                return result;
            }
            396 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 595; lexer.advance(false); continue; }
                return result;
            }
            397 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 589; lexer.advance(false); continue; }
                return result;
            }
            398 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 634; lexer.advance(false); continue; }
                return result;
            }
            399 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 640; lexer.advance(false); continue; }
                return result;
            }
            400 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 394; lexer.advance(false); continue; }
                return result;
            }
            401 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 398; lexer.advance(false); continue; }
                return result;
            }
            402 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 400; lexer.advance(false); continue; }
                return result;
            }
            403 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 401; lexer.advance(false); continue; }
                return result;
            }
            404 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 402; lexer.advance(false); continue; }
                return result;
            }
            405 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 403; lexer.advance(false); continue; }
                return result;
            }
            406 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            407 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 405; lexer.advance(false); continue; }
                return result;
            }
            408 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 406; lexer.advance(false); continue; }
                return result;
            }
            409 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 407; lexer.advance(false); continue; }
                return result;
            }
            410 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 408; lexer.advance(false); continue; }
                return result;
            }
            411 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 409; lexer.advance(false); continue; }
                return result;
            }
            412 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 410; lexer.advance(false); continue; }
                return result;
            }
            413 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 411; lexer.advance(false); continue; }
                return result;
            }
            414 => {
                if lookahead != 0 && lookahead != 42 { state = 462; lexer.advance(false); continue; }
                return result;
            }
            415 => {
                if eof { state = 420; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 509), (34, 624), (35, 677), (37, 529), (38, 538), (39, 604), (40, 506), (41, 428),
                    (42, 525), (43, 520), (44, 427), (45, 512), (46, 578), (47, 527), (48, 584), (58, 562),
                    (59, 553), (60, 547), (61, 560), (62, 542), (63, 563), (64, 610), (76, 643), (85, 645),
                    (91, 555), (92, 2), (93, 556), (94, 535), (117, 647), (123, 557), (124, 532), (125, 558),
                    (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 415; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 585; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            416 => {
                if eof { state = 420; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 508), (34, 624), (35, 680), (38, 536), (39, 604), (40, 506), (41, 428), (42, 524),
                    (43, 521), (44, 427), (45, 514), (46, 147), (47, 139), (48, 586), (58, 562), (59, 553),
                    (60, 546), (62, 541), (64, 616), (76, 643), (85, 645), (91, 555), (92, 4), (93, 556),
                    (94, 534), (117, 647), (123, 557), (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 416; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            417 => {
                if eof { state = 420; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 508), (34, 624), (35, 680), (38, 536), (39, 604), (40, 506), (42, 524), (43, 521),
                    (45, 514), (46, 391), (47, 139), (48, 586), (59, 553), (64, 615), (76, 643), (85, 645),
                    (91, 555), (92, 12), (94, 534), (117, 647), (123, 557), (125, 558), (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 417; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            418 => {
                if eof { state = 420; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 508), (34, 624), (35, 680), (38, 536), (39, 604), (40, 506), (42, 524), (43, 521),
                    (45, 514), (46, 391), (47, 139), (48, 586), (59, 553), (64, 611), (76, 643), (85, 645),
                    (91, 555), (92, 71), (94, 534), (117, 647), (123, 557), (125, 558), (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 418; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            419 => {
                if eof { state = 420; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 508), (34, 624), (35, 680), (38, 536), (39, 604), (40, 506), (42, 524), (43, 521),
                    (45, 514), (46, 391), (47, 139), (48, 586), (59, 553), (64, 612), (76, 643), (85, 645),
                    (91, 555), (92, 89), (94, 534), (117, 647), (123, 557), (126, 510),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 419; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            420 => {
                result = true; lexer.set_result_symbol(ts_builtin_sym_end); lexer.mark_end();
                return result;
            }
            421 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_include_token1); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            422 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_include_token2); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            423 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_include_token3); lexer.mark_end();
                return result;
            }
            424 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_def_token1); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            425 => {
                result = true; lexer.set_result_symbol(anon_sym_LPAREN); lexer.mark_end();
                return result;
            }
            426 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT_DOT_DOT); lexer.mark_end();
                return result;
            }
            427 => {
                result = true; lexer.set_result_symbol(anon_sym_COMMA); lexer.mark_end();
                return result;
            }
            428 => {
                result = true; lexer.set_result_symbol(anon_sym_RPAREN); lexer.mark_end();
                return result;
            }
            429 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_if_token1); lexer.mark_end();
                if lookahead == 100 { state = 233; lexer.advance(false); continue; }
                if lookahead == 110 { state = 212; lexer.advance(false); continue; }
                return result;
            }
            430 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_if_token1); lexer.mark_end();
                if lookahead == 100 { state = 474; lexer.advance(false); continue; }
                if lookahead == 110 { state = 468; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            431 => {
                result = true; lexer.set_result_symbol(anon_sym_LF); lexer.mark_end();
                if lookahead == 10 { state = 431; lexer.advance(false); continue; }
                return result;
            }
            432 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_if_token2); lexer.mark_end();
                return result;
            }
            433 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_if_token2); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            434 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_ifdef_token1); lexer.mark_end();
                return result;
            }
            435 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_ifdef_token1); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            436 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_ifdef_token2); lexer.mark_end();
                return result;
            }
            437 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_ifdef_token2); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            438 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_else_token1); lexer.mark_end();
                return result;
            }
            439 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_else_token1); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            440 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_elif_token1); lexer.mark_end();
                return result;
            }
            441 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_elif_token1); lexer.mark_end();
                if lookahead == 100 { state = 243; lexer.advance(false); continue; }
                if lookahead == 110 { state = 213; lexer.advance(false); continue; }
                return result;
            }
            442 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_elif_token1); lexer.mark_end();
                if lookahead == 100 { state = 477; lexer.advance(false); continue; }
                if lookahead == 110 { state = 469; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            443 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_elif_token1); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            444 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_elifdef_token1); lexer.mark_end();
                return result;
            }
            445 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_elifdef_token1); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            446 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_elifdef_token2); lexer.mark_end();
                return result;
            }
            447 => {
                result = true; lexer.set_result_symbol(aux_sym_preproc_elifdef_token2); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            448 => {
                result = true; lexer.set_result_symbol(sym_preproc_arg); lexer.mark_end();
                if lookahead == 10 { state = 141; lexer.advance(false); continue; }
                if lookahead == 42 { state = 448; lexer.advance(false); continue; }
                if lookahead == 47 { state = 657; lexer.advance(false); continue; }
                if lookahead == 92 { state = 458; lexer.advance(false); continue; }
                if lookahead != 0 { state = 449; lexer.advance(false); continue; }
                return result;
            }
            449 => {
                result = true; lexer.set_result_symbol(sym_preproc_arg); lexer.mark_end();
                if lookahead == 10 { state = 141; lexer.advance(false); continue; }
                if lookahead == 42 { state = 448; lexer.advance(false); continue; }
                if lookahead == 47 { state = 142; lexer.advance(false); continue; }
                if lookahead == 92 { state = 458; lexer.advance(false); continue; }
                if lookahead != 0 { state = 449; lexer.advance(false); continue; }
                return result;
            }
            450 => {
                result = true; lexer.set_result_symbol(sym_preproc_arg); lexer.mark_end();
                if lookahead == 10 { state = 670; lexer.advance(false); continue; }
                if lookahead == 13 { state = 658; lexer.advance(false); continue; }
                if lookahead == 47 { state = 666; lexer.advance(false); continue; }
                if lookahead == 92 { state = 663; lexer.advance(false); continue; }
                if lookahead != 0 { state = 667; lexer.advance(false); continue; }
                return result;
            }
            451 => {
                result = true; lexer.set_result_symbol(sym_preproc_arg); lexer.mark_end();
                if lookahead == 10 { state = 455; lexer.advance(false); continue; }
                if lookahead == 13 { state = 452; lexer.advance(false); continue; }
                if lookahead == 47 { state = 414; lexer.advance(false); continue; }
                if lookahead == 92 { state = 456; lexer.advance(false); continue; }
                if lookahead != 0 { state = 462; lexer.advance(false); continue; }
                return result;
            }
            452 => {
                result = true; lexer.set_result_symbol(sym_preproc_arg); lexer.mark_end();
                if lookahead == 10 { state = 455; lexer.advance(false); continue; }
                if lookahead == 47 { state = 414; lexer.advance(false); continue; }
                if lookahead == 92 { state = 456; lexer.advance(false); continue; }
                if lookahead != 0 { state = 462; lexer.advance(false); continue; }
                return result;
            }
            453 => {
                result = true; lexer.set_result_symbol(sym_preproc_arg); lexer.mark_end();
                if lookahead == 10 { state = 149; lexer.advance(true); continue; }
                if lookahead == 13 { state = 454; lexer.advance(false); continue; }
                if lookahead == 47 { state = 414; lexer.advance(false); continue; }
                if lookahead == 92 { state = 456; lexer.advance(false); continue; }
                if lookahead != 0 { state = 462; lexer.advance(false); continue; }
                return result;
            }
            454 => {
                result = true; lexer.set_result_symbol(sym_preproc_arg); lexer.mark_end();
                if lookahead == 10 { state = 149; lexer.advance(true); continue; }
                if lookahead == 47 { state = 414; lexer.advance(false); continue; }
                if lookahead == 92 { state = 456; lexer.advance(false); continue; }
                if lookahead != 0 { state = 462; lexer.advance(false); continue; }
                return result;
            }
            455 => {
                result = true; lexer.set_result_symbol(sym_preproc_arg); lexer.mark_end();
                if lookahead == 10 { state = 149; lexer.advance(true); continue; }
                if lookahead == 47 { state = 460; lexer.advance(false); continue; }
                if lookahead == 92 { state = 451; lexer.advance(false); continue; }
                if lookahead == 160 { state = 455; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 455; lexer.advance(false); continue; }
                if lookahead != 0 { state = 462; lexer.advance(false); continue; }
                return result;
            }
            456 => {
                result = true; lexer.set_result_symbol(sym_preproc_arg); lexer.mark_end();
                if lookahead == 13 { state = 463; lexer.advance(false); continue; }
                if lookahead == 47 { state = 414; lexer.advance(false); continue; }
                if lookahead == 92 { state = 456; lexer.advance(false); continue; }
                if lookahead != 0 { state = 462; lexer.advance(false); continue; }
                return result;
            }
            457 => {
                result = true; lexer.set_result_symbol(sym_preproc_arg); lexer.mark_end();
                if lookahead == 13 { state = 668; lexer.advance(false); continue; }
                if lookahead == 47 { state = 666; lexer.advance(false); continue; }
                if lookahead == 92 { state = 663; lexer.advance(false); continue; }
                if lookahead != 0 { state = 667; lexer.advance(false); continue; }
                return result;
            }
            458 => {
                result = true; lexer.set_result_symbol(sym_preproc_arg); lexer.mark_end();
                if lookahead == 13 { state = 461; lexer.advance(false); continue; }
                if lookahead == 42 { state = 448; lexer.advance(false); continue; }
                if lookahead == 47 { state = 142; lexer.advance(false); continue; }
                if lookahead == 92 { state = 458; lexer.advance(false); continue; }
                if lookahead != 0 { state = 449; lexer.advance(false); continue; }
                return result;
            }
            459 => {
                result = true; lexer.set_result_symbol(sym_preproc_arg); lexer.mark_end();
                if lookahead == 42 { state = 449; lexer.advance(false); continue; }
                if lookahead == 47 { state = 666; lexer.advance(false); continue; }
                if lookahead == 92 { state = 456; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 462; lexer.advance(false); continue; }
                return result;
            }
            460 => {
                result = true; lexer.set_result_symbol(sym_preproc_arg); lexer.mark_end();
                if lookahead == 42 { state = 449; lexer.advance(false); continue; }
                if lookahead == 47 { state = 669; lexer.advance(false); continue; }
                if lookahead == 92 { state = 456; lexer.advance(false); continue; }
                if lookahead != 0 { state = 462; lexer.advance(false); continue; }
                return result;
            }
            461 => {
                result = true; lexer.set_result_symbol(sym_preproc_arg); lexer.mark_end();
                if lookahead == 42 { state = 448; lexer.advance(false); continue; }
                if lookahead == 47 { state = 142; lexer.advance(false); continue; }
                if lookahead == 92 { state = 458; lexer.advance(false); continue; }
                if lookahead != 0 { state = 449; lexer.advance(false); continue; }
                return result;
            }
            462 => {
                result = true; lexer.set_result_symbol(sym_preproc_arg); lexer.mark_end();
                if lookahead == 47 { state = 414; lexer.advance(false); continue; }
                if lookahead == 92 { state = 456; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 462; lexer.advance(false); continue; }
                return result;
            }
            463 => {
                result = true; lexer.set_result_symbol(sym_preproc_arg); lexer.mark_end();
                if lookahead == 47 { state = 414; lexer.advance(false); continue; }
                if lookahead == 92 { state = 456; lexer.advance(false); continue; }
                if lookahead != 0 { state = 462; lexer.advance(false); continue; }
                return result;
            }
            464 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 99 { state = 495; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            465 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 100 { state = 492; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            466 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 100 { state = 473; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            467 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 100 { state = 475; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            468 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 100 { state = 476; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            469 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 100 { state = 478; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            470 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 101 { state = 482; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            471 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 101 { state = 439; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            472 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 101 { state = 424; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            473 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 101 { state = 421; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            474 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 101 { state = 485; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            475 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 101 { state = 479; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            476 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 101 { state = 486; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            477 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 101 { state = 487; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            478 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 101 { state = 488; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            479 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 102 { state = 505; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            480 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 102 { state = 430; lexer.advance(false); continue; }
                if lookahead == 109 { state = 501; lexer.advance(false); continue; }
                if lookahead == 110 { state = 464; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            481 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 102 { state = 430; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            482 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 102 { state = 490; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            483 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 102 { state = 442; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            484 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 102 { state = 433; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            485 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 102 { state = 435; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            486 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 102 { state = 437; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            487 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 102 { state = 445; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            488 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 102 { state = 447; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            489 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 102 { state = 443; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            490 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 105 { state = 499; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            491 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 105 { state = 483; lexer.advance(false); continue; }
                if lookahead == 115 { state = 471; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            492 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 105 { state = 484; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            493 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 105 { state = 489; lexer.advance(false); continue; }
                if lookahead == 115 { state = 471; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            494 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 108 { state = 491; lexer.advance(false); continue; }
                if lookahead == 110 { state = 465; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            495 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 108 { state = 504; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            496 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 108 { state = 493; lexer.advance(false); continue; }
                if lookahead == 110 { state = 465; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            497 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 110 { state = 465; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            498 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 110 { state = 467; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            499 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 110 { state = 472; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            500 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 111 { state = 502; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            501 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 112 { state = 500; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            502 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 114 { state = 503; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            503 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 116 { state = 422; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            504 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if lookahead == 117 { state = 466; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            505 => {
                result = true; lexer.set_result_symbol(sym_preproc_directive); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            506 => {
                result = true; lexer.set_result_symbol(anon_sym_LPAREN2); lexer.mark_end();
                return result;
            }
            507 => {
                result = true; lexer.set_result_symbol(anon_sym_LPAREN2); lexer.mark_end();
                if lookahead != 0 && lookahead != 41 { state = 718; lexer.advance(false); continue; }
                return result;
            }
            508 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG); lexer.mark_end();
                return result;
            }
            509 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG); lexer.mark_end();
                if lookahead == 61 { state = 540; lexer.advance(false); continue; }
                return result;
            }
            510 => {
                result = true; lexer.set_result_symbol(anon_sym_TILDE); lexer.mark_end();
                return result;
            }
            511 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                return result;
            }
            512 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 574; lexer.advance(false); continue; }
                if lookahead == 46 { state = 391; lexer.advance(false); continue; }
                if lookahead == 48 { state = 586; lexer.advance(false); continue; }
                if lookahead == 61 { state = 568; lexer.advance(false); continue; }
                if lookahead == 62 { state = 580; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                return result;
            }
            513 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 574; lexer.advance(false); continue; }
                if lookahead == 46 { state = 391; lexer.advance(false); continue; }
                if lookahead == 48 { state = 586; lexer.advance(false); continue; }
                if lookahead == 62 { state = 580; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                return result;
            }
            514 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 574; lexer.advance(false); continue; }
                if lookahead == 46 { state = 391; lexer.advance(false); continue; }
                if lookahead == 48 { state = 586; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                return result;
            }
            515 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 574; lexer.advance(false); continue; }
                if lookahead == 61 { state = 568; lexer.advance(false); continue; }
                if lookahead == 62 { state = 580; lexer.advance(false); continue; }
                return result;
            }
            516 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 574; lexer.advance(false); continue; }
                if lookahead == 62 { state = 580; lexer.advance(false); continue; }
                return result;
            }
            517 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 46 { state = 391; lexer.advance(false); continue; }
                if lookahead == 48 { state = 586; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                return result;
            }
            518 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                return result;
            }
            519 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                if lookahead == 43 { state = 575; lexer.advance(false); continue; }
                return result;
            }
            520 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                if lookahead == 43 { state = 575; lexer.advance(false); continue; }
                if lookahead == 46 { state = 391; lexer.advance(false); continue; }
                if lookahead == 48 { state = 586; lexer.advance(false); continue; }
                if lookahead == 61 { state = 567; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                return result;
            }
            521 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                if lookahead == 43 { state = 575; lexer.advance(false); continue; }
                if lookahead == 46 { state = 391; lexer.advance(false); continue; }
                if lookahead == 48 { state = 586; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                return result;
            }
            522 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                if lookahead == 43 { state = 575; lexer.advance(false); continue; }
                if lookahead == 61 { state = 567; lexer.advance(false); continue; }
                return result;
            }
            523 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                if lookahead == 46 { state = 391; lexer.advance(false); continue; }
                if lookahead == 48 { state = 586; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                return result;
            }
            524 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                return result;
            }
            525 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                if lookahead == 61 { state = 564; lexer.advance(false); continue; }
                return result;
            }
            526 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH); lexer.mark_end();
                if lookahead == 42 { state = 141; lexer.advance(false); continue; }
                if lookahead == 47 { state = 670; lexer.advance(false); continue; }
                return result;
            }
            527 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH); lexer.mark_end();
                if lookahead == 42 { state = 141; lexer.advance(false); continue; }
                if lookahead == 47 { state = 670; lexer.advance(false); continue; }
                if lookahead == 61 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            528 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT); lexer.mark_end();
                return result;
            }
            529 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT); lexer.mark_end();
                if lookahead == 61 { state = 566; lexer.advance(false); continue; }
                return result;
            }
            530 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE_PIPE); lexer.mark_end();
                return result;
            }
            531 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP_AMP); lexer.mark_end();
                return result;
            }
            532 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                if lookahead == 61 { state = 573; lexer.advance(false); continue; }
                if lookahead == 124 { state = 530; lexer.advance(false); continue; }
                return result;
            }
            533 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                if lookahead == 124 { state = 530; lexer.advance(false); continue; }
                return result;
            }
            534 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET); lexer.mark_end();
                return result;
            }
            535 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET); lexer.mark_end();
                if lookahead == 61 { state = 572; lexer.advance(false); continue; }
                return result;
            }
            536 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP); lexer.mark_end();
                return result;
            }
            537 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP); lexer.mark_end();
                if lookahead == 38 { state = 531; lexer.advance(false); continue; }
                return result;
            }
            538 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP); lexer.mark_end();
                if lookahead == 38 { state = 531; lexer.advance(false); continue; }
                if lookahead == 61 { state = 571; lexer.advance(false); continue; }
                return result;
            }
            539 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ_EQ); lexer.mark_end();
                return result;
            }
            540 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG_EQ); lexer.mark_end();
                return result;
            }
            541 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                return result;
            }
            542 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                if lookahead == 61 { state = 544; lexer.advance(false); continue; }
                if lookahead == 62 { state = 552; lexer.advance(false); continue; }
                return result;
            }
            543 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                if lookahead == 61 { state = 544; lexer.advance(false); continue; }
                if lookahead == 62 { state = 551; lexer.advance(false); continue; }
                return result;
            }
            544 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_EQ); lexer.mark_end();
                return result;
            }
            545 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_EQ); lexer.mark_end();
                return result;
            }
            546 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                return result;
            }
            547 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 60 { state = 550; lexer.advance(false); continue; }
                if lookahead == 61 { state = 545; lexer.advance(false); continue; }
                return result;
            }
            548 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 60 { state = 549; lexer.advance(false); continue; }
                if lookahead == 61 { state = 545; lexer.advance(false); continue; }
                return result;
            }
            549 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT); lexer.mark_end();
                return result;
            }
            550 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT); lexer.mark_end();
                if lookahead == 61 { state = 569; lexer.advance(false); continue; }
                return result;
            }
            551 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT); lexer.mark_end();
                return result;
            }
            552 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT); lexer.mark_end();
                if lookahead == 61 { state = 570; lexer.advance(false); continue; }
                return result;
            }
            553 => {
                result = true; lexer.set_result_symbol(anon_sym_SEMI); lexer.mark_end();
                return result;
            }
            554 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON_COLON); lexer.mark_end();
                return result;
            }
            555 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK); lexer.mark_end();
                return result;
            }
            556 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACK); lexer.mark_end();
                return result;
            }
            557 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACE); lexer.mark_end();
                return result;
            }
            558 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACE); lexer.mark_end();
                return result;
            }
            559 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                return result;
            }
            560 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                if lookahead == 61 { state = 539; lexer.advance(false); continue; }
                return result;
            }
            561 => {
                result = true; lexer.set_result_symbol(anon_sym_ATautoreleasepool); lexer.mark_end();
                return result;
            }
            562 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON); lexer.mark_end();
                return result;
            }
            563 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK); lexer.mark_end();
                return result;
            }
            564 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR_EQ); lexer.mark_end();
                return result;
            }
            565 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_EQ); lexer.mark_end();
                return result;
            }
            566 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT_EQ); lexer.mark_end();
                return result;
            }
            567 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS_EQ); lexer.mark_end();
                return result;
            }
            568 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_EQ); lexer.mark_end();
                return result;
            }
            569 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT_EQ); lexer.mark_end();
                return result;
            }
            570 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT_EQ); lexer.mark_end();
                return result;
            }
            571 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP_EQ); lexer.mark_end();
                return result;
            }
            572 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET_EQ); lexer.mark_end();
                return result;
            }
            573 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE_EQ); lexer.mark_end();
                return result;
            }
            574 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_DASH); lexer.mark_end();
                return result;
            }
            575 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS_PLUS); lexer.mark_end();
                return result;
            }
            576 => {
                result = true; lexer.set_result_symbol(anon_sym_LT2); lexer.mark_end();
                if lookahead == 60 { state = 550; lexer.advance(false); continue; }
                if lookahead == 61 { state = 545; lexer.advance(false); continue; }
                return result;
            }
            577 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                if lookahead == 46 { state = 148; lexer.advance(false); continue; }
                return result;
            }
            578 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                if lookahead == 46 { state = 148; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 581; lexer.advance(false); continue; }
                return result;
            }
            579 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 { state = 581; lexer.advance(false); continue; }
                return result;
            }
            580 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_GT); lexer.mark_end();
                return result;
            }
            581 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (39, 391), (69, 596), (80, 596), (101, 596), (112, 596), (70, 599), (76, 599), (85, 599),
                    (102, 599), (105, 599), (108, 599), (117, 599),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 581; lexer.advance(false); continue; }
                return result;
            }
            582 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (39, 393), (46, 598), (98, 594), (120, 396), (69, 593), (101, 593), (70, 595), (102, 595),
                    (80, 596), (112, 596), (76, 599), (85, 599), (105, 599), (108, 599), (117, 599),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 68 || 97 <= lookahead && lookahead <= 100 { state = 595; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 583; lexer.advance(false); continue; }
                return result;
            }
            583 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (39, 393), (46, 598), (69, 593), (101, 593), (70, 595), (102, 595), (80, 596), (112, 596),
                    (76, 599), (85, 599), (105, 599), (108, 599), (117, 599),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 68 || 97 <= lookahead && lookahead <= 100 { state = 595; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 583; lexer.advance(false); continue; }
                return result;
            }
            584 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (39, 390), (46, 597), (95, 392), (98, 145), (120, 144), (69, 596), (80, 596), (101, 596),
                    (112, 596), (70, 599), (76, 599), (85, 599), (102, 599), (105, 599), (108, 599), (117, 599),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 585; lexer.advance(false); continue; }
                return result;
            }
            585 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (39, 390), (46, 597), (95, 392), (69, 596), (80, 596), (101, 596), (112, 596), (70, 599),
                    (76, 599), (85, 599), (102, 599), (105, 599), (108, 599), (117, 599),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 585; lexer.advance(false); continue; }
                return result;
            }
            586 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (39, 390), (46, 598), (98, 145), (120, 144), (69, 596), (80, 596), (101, 596), (112, 596),
                    (70, 599), (76, 599), (85, 599), (102, 599), (105, 599), (108, 599), (117, 599),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                return result;
            }
            587 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (39, 390), (46, 598), (98, 390), (120, 396), (69, 596), (80, 596), (101, 596), (112, 596),
                    (70, 599), (76, 599), (85, 599), (102, 599), (105, 599), (108, 599), (117, 599),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                return result;
            }
            588 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (39, 390), (46, 598), (69, 596), (80, 596), (101, 596), (112, 596), (70, 599), (76, 599),
                    (85, 599), (102, 599), (105, 599), (108, 599), (117, 599),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 588; lexer.advance(false); continue; }
                return result;
            }
            589 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (39, 397), (70, 589), (102, 589), (76, 599), (85, 599), (105, 599), (108, 599), (117, 599),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 69 || 97 <= lookahead && lookahead <= 101 { state = 589; lexer.advance(false); continue; }
                return result;
            }
            590 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (39, 395), (43, 397), (45, 397), (69, 590), (101, 590), (70, 592), (102, 592), (80, 596),
                    (112, 596), (76, 599), (85, 599), (105, 599), (108, 599), (117, 599),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 68 || 97 <= lookahead && lookahead <= 100 { state = 592; lexer.advance(false); continue; }
                return result;
            }
            591 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (39, 395), (46, 392), (95, 392), (69, 590), (101, 590), (70, 592), (102, 592), (80, 596),
                    (112, 596), (76, 599), (85, 599), (105, 599), (108, 599), (117, 599),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 68 || 97 <= lookahead && lookahead <= 100 { state = 592; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 591; lexer.advance(false); continue; }
                return result;
            }
            592 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (39, 395), (69, 590), (101, 590), (70, 592), (102, 592), (80, 596), (112, 596), (76, 599),
                    (85, 599), (105, 599), (108, 599), (117, 599),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 68 || 97 <= lookahead && lookahead <= 100 { state = 592; lexer.advance(false); continue; }
                return result;
            }
            593 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (39, 396), (46, 598), (43, 397), (45, 397), (69, 593), (101, 593), (70, 595), (102, 595),
                    (80, 596), (112, 596), (76, 599), (85, 599), (105, 599), (108, 599), (117, 599),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 68 || 97 <= lookahead && lookahead <= 100 { state = 595; lexer.advance(false); continue; }
                return result;
            }
            594 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (39, 396), (46, 598), (69, 593), (101, 593), (70, 595), (102, 595), (80, 596), (112, 596),
                    (76, 599), (85, 599), (105, 599), (108, 599), (117, 599),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 68 || 97 <= lookahead && lookahead <= 100 { state = 595; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 583; lexer.advance(false); continue; }
                return result;
            }
            595 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (39, 396), (46, 598), (69, 593), (101, 593), (70, 595), (102, 595), (80, 596), (112, 596),
                    (76, 599), (85, 599), (105, 599), (108, 599), (117, 599),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 68 || 97 <= lookahead && lookahead <= 100 { state = 595; lexer.advance(false); continue; }
                return result;
            }
            596 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (43, 397), (45, 397), (70, 589), (102, 589), (76, 599), (85, 599), (105, 599), (108, 599),
                    (117, 599),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 69 || 97 <= lookahead && lookahead <= 101 { state = 589; lexer.advance(false); continue; }
                return result;
            }
            597 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (69, 590), (101, 590), (70, 592), (102, 592), (80, 596), (112, 596), (76, 599), (85, 599),
                    (105, 599), (108, 599), (117, 599),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 68 || 97 <= lookahead && lookahead <= 100 { state = 592; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 591; lexer.advance(false); continue; }
                return result;
            }
            598 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (69, 590), (101, 590), (70, 592), (102, 592), (80, 596), (112, 596), (76, 599), (85, 599),
                    (105, 599), (108, 599), (117, 599),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 68 || 97 <= lookahead && lookahead <= 100 { state = 592; lexer.advance(false); continue; }
                return result;
            }
            599 => {
                result = true; lexer.set_result_symbol(sym_number_literal); lexer.mark_end();
                if lookahead == 70 || lookahead == 76 || lookahead == 85 || lookahead == 102 || lookahead == 105 || lookahead == 108 || lookahead == 117 { state = 599; lexer.advance(false); continue; }
                return result;
            }
            600 => {
                result = true; lexer.set_result_symbol(anon_sym_L_SQUOTE); lexer.mark_end();
                return result;
            }
            601 => {
                result = true; lexer.set_result_symbol(anon_sym_u_SQUOTE); lexer.mark_end();
                return result;
            }
            602 => {
                result = true; lexer.set_result_symbol(anon_sym_U_SQUOTE); lexer.mark_end();
                return result;
            }
            603 => {
                result = true; lexer.set_result_symbol(anon_sym_u8_SQUOTE); lexer.mark_end();
                return result;
            }
            604 => {
                result = true; lexer.set_result_symbol(anon_sym_SQUOTE); lexer.mark_end();
                return result;
            }
            605 => {
                result = true; lexer.set_result_symbol(aux_sym_char_literal_token1); lexer.mark_end();
                return result;
            }
            606 => {
                result = true; lexer.set_result_symbol(aux_sym_char_literal_token1); lexer.mark_end();
                if lookahead == 10 { state = 635; lexer.advance(false); continue; }
                if lookahead == 13 { state = 634; lexer.advance(false); continue; }
                if lookahead == 85 { state = 413; lexer.advance(false); continue; }
                if lookahead == 117 { state = 405; lexer.advance(false); continue; }
                if lookahead == 120 { state = 399; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 637; lexer.advance(false); continue; }
                if lookahead != 0 { state = 634; lexer.advance(false); continue; }
                return result;
            }
            607 => {
                result = true; lexer.set_result_symbol(aux_sym_char_literal_token1); lexer.mark_end();
                if lookahead == 42 { state = 141; lexer.advance(false); continue; }
                if lookahead == 47 { state = 670; lexer.advance(false); continue; }
                return result;
            }
            608 => {
                result = true; lexer.set_result_symbol(aux_sym_char_literal_token1); lexer.mark_end();
                if lookahead == 92 { state = 60; lexer.advance(false); continue; }
                return result;
            }
            609 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                return result;
            }
            610 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (97, 377), (99, 166), (100, 386), (101, 307), (102, 282), (105, 302), (112, 343), (115, 215),
                    (116, 262),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            611 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (97, 377), (99, 166), (101, 307), (102, 282), (105, 302), (112, 343), (115, 216), (116, 262),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            612 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                if lookahead == 97 { state = 377; lexer.advance(false); continue; }
                if lookahead == 99 { state = 166; lexer.advance(false); continue; }
                if lookahead == 101 { state = 307; lexer.advance(false); continue; }
                if lookahead == 102 { state = 282; lexer.advance(false); continue; }
                if lookahead == 105 { state = 302; lexer.advance(false); continue; }
                if lookahead == 112 { state = 343; lexer.advance(false); continue; }
                if lookahead == 115 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            613 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                if lookahead == 97 { state = 377; lexer.advance(false); continue; }
                if lookahead == 99 { state = 165; lexer.advance(false); continue; }
                if lookahead == 101 { state = 307; lexer.advance(false); continue; }
                if lookahead == 102 { state = 282; lexer.advance(false); continue; }
                if lookahead == 115 { state = 216; lexer.advance(false); continue; }
                if lookahead == 116 { state = 262; lexer.advance(false); continue; }
                return result;
            }
            614 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (97, 377), (99, 319), (100, 386), (101, 307), (105, 302), (112, 343), (115, 215), (116, 262),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            615 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                if lookahead == 97 { state = 377; lexer.advance(false); continue; }
                if lookahead == 99 { state = 319; lexer.advance(false); continue; }
                if lookahead == 101 { state = 307; lexer.advance(false); continue; }
                if lookahead == 105 { state = 302; lexer.advance(false); continue; }
                if lookahead == 112 { state = 343; lexer.advance(false); continue; }
                if lookahead == 115 { state = 216; lexer.advance(false); continue; }
                if lookahead == 116 { state = 262; lexer.advance(false); continue; }
                return result;
            }
            616 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                if lookahead == 97 { state = 377; lexer.advance(false); continue; }
                if lookahead == 99 { state = 319; lexer.advance(false); continue; }
                if lookahead == 101 { state = 307; lexer.advance(false); continue; }
                if lookahead == 105 { state = 302; lexer.advance(false); continue; }
                if lookahead == 112 { state = 343; lexer.advance(false); continue; }
                if lookahead == 115 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            617 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                if lookahead == 97 { state = 377; lexer.advance(false); continue; }
                if lookahead == 101 { state = 307; lexer.advance(false); continue; }
                if lookahead == 105 { state = 305; lexer.advance(false); continue; }
                if lookahead == 112 { state = 343; lexer.advance(false); continue; }
                if lookahead == 115 { state = 216; lexer.advance(false); continue; }
                if lookahead == 116 { state = 262; lexer.advance(false); continue; }
                return result;
            }
            618 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                if lookahead == 97 { state = 377; lexer.advance(false); continue; }
                if lookahead == 101 { state = 307; lexer.advance(false); continue; }
                if lookahead == 115 { state = 216; lexer.advance(false); continue; }
                if lookahead == 116 { state = 262; lexer.advance(false); continue; }
                return result;
            }
            619 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                if lookahead == 97 { state = 377; lexer.advance(false); continue; }
                if lookahead == 101 { state = 307; lexer.advance(false); continue; }
                if lookahead == 115 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            620 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                if lookahead == 97 { state = 379; lexer.advance(false); continue; }
                if lookahead == 101 { state = 307; lexer.advance(false); continue; }
                if lookahead == 105 { state = 305; lexer.advance(false); continue; }
                if lookahead == 112 { state = 343; lexer.advance(false); continue; }
                if lookahead == 115 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            621 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                if lookahead == 97 { state = 379; lexer.advance(false); continue; }
                if lookahead == 101 { state = 307; lexer.advance(false); continue; }
                if lookahead == 115 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            622 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                if lookahead == 97 { state = 376; lexer.advance(false); continue; }
                return result;
            }
            623 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                if lookahead == 105 { state = 305; lexer.advance(false); continue; }
                if lookahead == 112 { state = 343; lexer.advance(false); continue; }
                return result;
            }
            624 => {
                result = true; lexer.set_result_symbol(anon_sym_DQUOTE); lexer.mark_end();
                return result;
            }
            625 => {
                result = true; lexer.set_result_symbol(anon_sym_L_DQUOTE); lexer.mark_end();
                return result;
            }
            626 => {
                result = true; lexer.set_result_symbol(anon_sym_u_DQUOTE); lexer.mark_end();
                return result;
            }
            627 => {
                result = true; lexer.set_result_symbol(anon_sym_U_DQUOTE); lexer.mark_end();
                return result;
            }
            628 => {
                result = true; lexer.set_result_symbol(anon_sym_u8_DQUOTE); lexer.mark_end();
                return result;
            }
            629 => {
                result = true; lexer.set_result_symbol(aux_sym_string_literal_token1); lexer.mark_end();
                if lookahead == 42 { state = 631; lexer.advance(false); continue; }
                if lookahead == 47 { state = 633; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 34 && lookahead != 92 { state = 633; lexer.advance(false); continue; }
                return result;
            }
            630 => {
                result = true; lexer.set_result_symbol(aux_sym_string_literal_token1); lexer.mark_end();
                if lookahead == 42 { state = 630; lexer.advance(false); continue; }
                if lookahead == 47 { state = 633; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 34 && lookahead != 92 { state = 631; lexer.advance(false); continue; }
                return result;
            }
            631 => {
                result = true; lexer.set_result_symbol(aux_sym_string_literal_token1); lexer.mark_end();
                if lookahead == 42 { state = 630; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 && lookahead != 34 && lookahead != 92 { state = 631; lexer.advance(false); continue; }
                return result;
            }
            632 => {
                result = true; lexer.set_result_symbol(aux_sym_string_literal_token1); lexer.mark_end();
                if lookahead == 47 { state = 629; lexer.advance(false); continue; }
                if lookahead == 9 || 11 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 632; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 34 && lookahead != 92 { state = 633; lexer.advance(false); continue; }
                return result;
            }
            633 => {
                result = true; lexer.set_result_symbol(aux_sym_string_literal_token1); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 && lookahead != 34 && lookahead != 92 { state = 633; lexer.advance(false); continue; }
                return result;
            }
            634 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                return result;
            }
            635 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                if lookahead == 92 { state = 60; lexer.advance(false); continue; }
                return result;
            }
            636 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 { state = 634; lexer.advance(false); continue; }
                return result;
            }
            637 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 { state = 636; lexer.advance(false); continue; }
                return result;
            }
            638 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 634; lexer.advance(false); continue; }
                return result;
            }
            639 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 638; lexer.advance(false); continue; }
                return result;
            }
            640 => {
                result = true; lexer.set_result_symbol(sym_escape_sequence); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 639; lexer.advance(false); continue; }
                return result;
            }
            641 => {
                result = true; lexer.set_result_symbol(sym_system_lib_string); lexer.mark_end();
                return result;
            }
            642 => {
                result = true; lexer.set_result_symbol(sym_system_lib_string); lexer.mark_end();
                if lookahead == 62 { state = 641; lexer.advance(false); continue; }
                if lookahead == 92 { state = 154; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 153; lexer.advance(false); continue; }
                return result;
            }
            643 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 34 { state = 625; lexer.advance(false); continue; }
                if lookahead == 39 { state = 600; lexer.advance(false); continue; }
                if lookahead == 92 { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            644 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 34 { state = 625; lexer.advance(false); continue; }
                if lookahead == 92 { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            645 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 34 { state = 627; lexer.advance(false); continue; }
                if lookahead == 39 { state = 602; lexer.advance(false); continue; }
                if lookahead == 92 { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            646 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 34 { state = 627; lexer.advance(false); continue; }
                if lookahead == 92 { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            647 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 34 { state = 626; lexer.advance(false); continue; }
                if lookahead == 39 { state = 601; lexer.advance(false); continue; }
                if lookahead == 56 { state = 649; lexer.advance(false); continue; }
                if lookahead == 92 { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            648 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 34 { state = 626; lexer.advance(false); continue; }
                if lookahead == 56 { state = 650; lexer.advance(false); continue; }
                if lookahead == 92 { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            649 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 34 { state = 628; lexer.advance(false); continue; }
                if lookahead == 39 { state = 603; lexer.advance(false); continue; }
                if lookahead == 92 { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            650 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 34 { state = 628; lexer.advance(false); continue; }
                if lookahead == 92 { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            651 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 39 { state = 600; lexer.advance(false); continue; }
                if lookahead == 92 { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            652 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 39 { state = 602; lexer.advance(false); continue; }
                if lookahead == 92 { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            653 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 39 { state = 601; lexer.advance(false); continue; }
                if lookahead == 56 { state = 654; lexer.advance(false); continue; }
                if lookahead == 92 { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            654 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 39 { state = 603; lexer.advance(false); continue; }
                if lookahead == 92 { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            655 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 155; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 655; lexer.advance(false); continue; }
                return result;
            }
            656 => {
                result = true; lexer.set_result_symbol(sym_identifier); lexer.mark_end();
                if lookahead == 92 { state = 709; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_2, lookahead) { state = 656; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 41 { state = 718; lexer.advance(false); continue; }
                return result;
            }
            657 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                return result;
            }
            658 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 10 { state = 670; lexer.advance(false); continue; }
                if lookahead == 47 { state = 666; lexer.advance(false); continue; }
                if lookahead == 92 { state = 457; lexer.advance(false); continue; }
                if lookahead != 0 { state = 667; lexer.advance(false); continue; }
                return result;
            }
            659 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 10 { state = 718; lexer.advance(false); continue; }
                if lookahead == 41 { state = 670; lexer.advance(false); continue; }
                if lookahead == 92 { state = 703; lexer.advance(false); continue; }
                if lookahead != 0 { state = 659; lexer.advance(false); continue; }
                return result;
            }
            660 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 10 { state = 727; lexer.advance(false); continue; }
                if lookahead == 92 { state = 722; lexer.advance(false); continue; }
                if lookahead == 125 { state = 670; lexer.advance(false); continue; }
                if lookahead != 0 { state = 660; lexer.advance(false); continue; }
                return result;
            }
            661 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 13 { state = 671; lexer.advance(false); continue; }
                if lookahead == 92 { state = 661; lexer.advance(false); continue; }
                if lookahead != 0 { state = 670; lexer.advance(false); continue; }
                return result;
            }
            662 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 13 { state = 665; lexer.advance(false); continue; }
                if lookahead == 41 { state = 670; lexer.advance(false); continue; }
                if lookahead == 92 { state = 662; lexer.advance(false); continue; }
                if lookahead != 0 { state = 659; lexer.advance(false); continue; }
                return result;
            }
            663 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 13 { state = 668; lexer.advance(false); continue; }
                if lookahead == 47 { state = 666; lexer.advance(false); continue; }
                if lookahead == 92 { state = 663; lexer.advance(false); continue; }
                if lookahead != 0 { state = 667; lexer.advance(false); continue; }
                return result;
            }
            664 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 13 { state = 672; lexer.advance(false); continue; }
                if lookahead == 92 { state = 664; lexer.advance(false); continue; }
                if lookahead == 125 { state = 670; lexer.advance(false); continue; }
                if lookahead != 0 { state = 660; lexer.advance(false); continue; }
                return result;
            }
            665 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 41 { state = 670; lexer.advance(false); continue; }
                if lookahead == 92 { state = 703; lexer.advance(false); continue; }
                if lookahead != 0 { state = 659; lexer.advance(false); continue; }
                return result;
            }
            666 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 42 { state = 670; lexer.advance(false); continue; }
                if lookahead == 92 { state = 450; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 667; lexer.advance(false); continue; }
                return result;
            }
            667 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 47 { state = 666; lexer.advance(false); continue; }
                if lookahead == 92 { state = 457; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 667; lexer.advance(false); continue; }
                return result;
            }
            668 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 47 { state = 666; lexer.advance(false); continue; }
                if lookahead == 92 { state = 457; lexer.advance(false); continue; }
                if lookahead != 0 { state = 667; lexer.advance(false); continue; }
                return result;
            }
            669 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 47 { state = 669; lexer.advance(false); continue; }
                if lookahead == 92 { state = 457; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 667; lexer.advance(false); continue; }
                return result;
            }
            670 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 92 { state = 92; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 10 { state = 670; lexer.advance(false); continue; }
                return result;
            }
            671 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 92 { state = 92; lexer.advance(false); continue; }
                if lookahead != 0 { state = 670; lexer.advance(false); continue; }
                return result;
            }
            672 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead == 92 { state = 722; lexer.advance(false); continue; }
                if lookahead == 125 { state = 670; lexer.advance(false); continue; }
                if lookahead != 0 { state = 660; lexer.advance(false); continue; }
                return result;
            }
            673 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead != 0 && lookahead != 41 { state = 718; lexer.advance(false); continue; }
                return result;
            }
            674 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead != 0 && lookahead != 125 { state = 727; lexer.advance(false); continue; }
                return result;
            }
            675 => {
                result = true; lexer.set_result_symbol(sym_version_number); lexer.mark_end();
                if lookahead == 46 || lookahead == 95 { state = 392; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 675; lexer.advance(false); continue; }
                return result;
            }
            676 => {
                result = true; lexer.set_result_symbol(anon_sym_ATimport); lexer.mark_end();
                return result;
            }
            677 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND); lexer.mark_end();
                if lookahead == 100 { state = 470; lexer.advance(false); continue; }
                if lookahead == 101 { state = 494; lexer.advance(false); continue; }
                if lookahead == 105 { state = 480; lexer.advance(false); continue; }
                if lookahead == 117 { state = 498; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 195; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            678 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND); lexer.mark_end();
                if lookahead == 100 { state = 470; lexer.advance(false); continue; }
                if lookahead == 101 { state = 497; lexer.advance(false); continue; }
                if lookahead == 105 { state = 480; lexer.advance(false); continue; }
                if lookahead == 117 { state = 498; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 197; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            679 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND); lexer.mark_end();
                if lookahead == 100 { state = 470; lexer.advance(false); continue; }
                if lookahead == 101 { state = 496; lexer.advance(false); continue; }
                if lookahead == 105 { state = 480; lexer.advance(false); continue; }
                if lookahead == 117 { state = 498; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 199; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            680 => {
                result = true; lexer.set_result_symbol(anon_sym_POUND); lexer.mark_end();
                if lookahead == 100 { state = 470; lexer.advance(false); continue; }
                if lookahead == 105 { state = 480; lexer.advance(false); continue; }
                if lookahead == 117 { state = 498; lexer.advance(false); continue; }
                if lookahead == 9 || lookahead == 32 { state = 201; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || 97 <= lookahead && lookahead <= 122 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            681 => {
                result = true; lexer.set_result_symbol(anon_sym_ATprotocol); lexer.mark_end();
                return result;
            }
            682 => {
                result = true; lexer.set_result_symbol(anon_sym_ATend); lexer.mark_end();
                return result;
            }
            683 => {
                result = true; lexer.set_result_symbol(anon_sym_ATinterface); lexer.mark_end();
                return result;
            }
            684 => {
                result = true; lexer.set_result_symbol(anon_sym_ATimplementation); lexer.mark_end();
                return result;
            }
            685 => {
                result = true; lexer.set_result_symbol(anon_sym_ATprivate); lexer.mark_end();
                return result;
            }
            686 => {
                result = true; lexer.set_result_symbol(anon_sym_ATprotected); lexer.mark_end();
                return result;
            }
            687 => {
                result = true; lexer.set_result_symbol(anon_sym_ATpackage); lexer.mark_end();
                return result;
            }
            688 => {
                result = true; lexer.set_result_symbol(anon_sym_ATpublic); lexer.mark_end();
                return result;
            }
            689 => {
                result = true; lexer.set_result_symbol(anon_sym_ATcompatibility_alias); lexer.mark_end();
                return result;
            }
            690 => {
                result = true; lexer.set_result_symbol(anon_sym_AToptional); lexer.mark_end();
                return result;
            }
            691 => {
                result = true; lexer.set_result_symbol(anon_sym_ATrequired); lexer.mark_end();
                return result;
            }
            692 => {
                result = true; lexer.set_result_symbol(anon_sym_ATsynthesize); lexer.mark_end();
                return result;
            }
            693 => {
                result = true; lexer.set_result_symbol(anon_sym_ATdynamic); lexer.mark_end();
                return result;
            }
            694 => {
                result = true; lexer.set_result_symbol(anon_sym_LPARENclass_RPAREN); lexer.mark_end();
                return result;
            }
            695 => {
                result = true; lexer.set_result_symbol(anon_sym_ATproperty); lexer.mark_end();
                return result;
            }
            696 => {
                result = true; lexer.set_result_symbol(anon_sym_ATtry); lexer.mark_end();
                return result;
            }
            697 => {
                result = true; lexer.set_result_symbol(anon_sym_ATcatch); lexer.mark_end();
                return result;
            }
            698 => {
                result = true; lexer.set_result_symbol(anon_sym_ATfinally); lexer.mark_end();
                return result;
            }
            699 => {
                result = true; lexer.set_result_symbol(anon_sym_ATthrow); lexer.mark_end();
                return result;
            }
            700 => {
                result = true; lexer.set_result_symbol(anon_sym_ATselector); lexer.mark_end();
                return result;
            }
            701 => {
                result = true; lexer.set_result_symbol(aux_sym_selector_expression_token1); lexer.mark_end();
                if lookahead == 10 { state = 705; lexer.advance(false); continue; }
                if lookahead == 13 { state = 702; lexer.advance(false); continue; }
                if lookahead == 85 { state = 717; lexer.advance(false); continue; }
                if lookahead == 117 { state = 713; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 41 { state = 718; lexer.advance(false); continue; }
                return result;
            }
            702 => {
                result = true; lexer.set_result_symbol(aux_sym_selector_expression_token1); lexer.mark_end();
                if lookahead == 10 { state = 705; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 41 { state = 718; lexer.advance(false); continue; }
                return result;
            }
            703 => {
                result = true; lexer.set_result_symbol(aux_sym_selector_expression_token1); lexer.mark_end();
                if lookahead == 13 { state = 665; lexer.advance(false); continue; }
                if lookahead == 41 { state = 670; lexer.advance(false); continue; }
                if lookahead == 92 { state = 662; lexer.advance(false); continue; }
                if lookahead != 0 { state = 659; lexer.advance(false); continue; }
                return result;
            }
            704 => {
                result = true; lexer.set_result_symbol(aux_sym_selector_expression_token1); lexer.mark_end();
                if lookahead == 40 { state = 507; lexer.advance(false); continue; }
                if lookahead == 47 { state = 708; lexer.advance(false); continue; }
                if lookahead == 58 { state = 732; lexer.advance(false); continue; }
                if lookahead == 92 { state = 701; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 705; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 656; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 40 && lookahead != 41 { state = 718; lexer.advance(false); continue; }
                return result;
            }
            705 => {
                result = true; lexer.set_result_symbol(aux_sym_selector_expression_token1); lexer.mark_end();
                if lookahead == 40 { state = 507; lexer.advance(false); continue; }
                if lookahead == 47 { state = 708; lexer.advance(false); continue; }
                if lookahead == 92 { state = 701; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 705; lexer.advance(false); continue; }
                if set_contains(&sym_identifier_character_set_1, lookahead) { state = 656; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 40 && lookahead != 41 { state = 718; lexer.advance(false); continue; }
                return result;
            }
            706 => {
                result = true; lexer.set_result_symbol(aux_sym_selector_expression_token1); lexer.mark_end();
                if lookahead == 41 { state = 141; lexer.advance(false); continue; }
                if lookahead == 42 { state = 706; lexer.advance(false); continue; }
                if lookahead == 47 { state = 673; lexer.advance(false); continue; }
                if lookahead != 0 { state = 707; lexer.advance(false); continue; }
                return result;
            }
            707 => {
                result = true; lexer.set_result_symbol(aux_sym_selector_expression_token1); lexer.mark_end();
                if lookahead == 41 { state = 141; lexer.advance(false); continue; }
                if lookahead == 42 { state = 706; lexer.advance(false); continue; }
                if lookahead != 0 { state = 707; lexer.advance(false); continue; }
                return result;
            }
            708 => {
                result = true; lexer.set_result_symbol(aux_sym_selector_expression_token1); lexer.mark_end();
                if lookahead == 42 { state = 707; lexer.advance(false); continue; }
                if lookahead == 47 { state = 659; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 41 && lookahead != 42 { state = 718; lexer.advance(false); continue; }
                return result;
            }
            709 => {
                result = true; lexer.set_result_symbol(aux_sym_selector_expression_token1); lexer.mark_end();
                if lookahead == 85 { state = 717; lexer.advance(false); continue; }
                if lookahead == 117 { state = 713; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 41 { state = 718; lexer.advance(false); continue; }
                return result;
            }
            710 => {
                result = true; lexer.set_result_symbol(aux_sym_selector_expression_token1); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 656; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 41 { state = 718; lexer.advance(false); continue; }
                return result;
            }
            711 => {
                result = true; lexer.set_result_symbol(aux_sym_selector_expression_token1); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 710; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 41 { state = 718; lexer.advance(false); continue; }
                return result;
            }
            712 => {
                result = true; lexer.set_result_symbol(aux_sym_selector_expression_token1); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 711; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 41 { state = 718; lexer.advance(false); continue; }
                return result;
            }
            713 => {
                result = true; lexer.set_result_symbol(aux_sym_selector_expression_token1); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 712; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 41 { state = 718; lexer.advance(false); continue; }
                return result;
            }
            714 => {
                result = true; lexer.set_result_symbol(aux_sym_selector_expression_token1); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 713; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 41 { state = 718; lexer.advance(false); continue; }
                return result;
            }
            715 => {
                result = true; lexer.set_result_symbol(aux_sym_selector_expression_token1); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 714; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 41 { state = 718; lexer.advance(false); continue; }
                return result;
            }
            716 => {
                result = true; lexer.set_result_symbol(aux_sym_selector_expression_token1); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 715; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 41 { state = 718; lexer.advance(false); continue; }
                return result;
            }
            717 => {
                result = true; lexer.set_result_symbol(aux_sym_selector_expression_token1); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 716; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 41 { state = 718; lexer.advance(false); continue; }
                return result;
            }
            718 => {
                result = true; lexer.set_result_symbol(aux_sym_selector_expression_token1); lexer.mark_end();
                if lookahead != 0 && lookahead != 41 { state = 718; lexer.advance(false); continue; }
                return result;
            }
            719 => {
                result = true; lexer.set_result_symbol(anon_sym_ATavailable); lexer.mark_end();
                return result;
            }
            720 => {
                result = true; lexer.set_result_symbol(aux_sym_ms_asm_block_token1); lexer.mark_end();
                if lookahead == 10 { state = 726; lexer.advance(false); continue; }
                if lookahead == 13 { state = 721; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 125 { state = 727; lexer.advance(false); continue; }
                return result;
            }
            721 => {
                result = true; lexer.set_result_symbol(aux_sym_ms_asm_block_token1); lexer.mark_end();
                if lookahead == 10 { state = 726; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 125 { state = 727; lexer.advance(false); continue; }
                return result;
            }
            722 => {
                result = true; lexer.set_result_symbol(aux_sym_ms_asm_block_token1); lexer.mark_end();
                if lookahead == 13 { state = 672; lexer.advance(false); continue; }
                if lookahead == 92 { state = 664; lexer.advance(false); continue; }
                if lookahead == 125 { state = 670; lexer.advance(false); continue; }
                if lookahead != 0 { state = 660; lexer.advance(false); continue; }
                return result;
            }
            723 => {
                result = true; lexer.set_result_symbol(aux_sym_ms_asm_block_token1); lexer.mark_end();
                if lookahead == 42 { state = 725; lexer.advance(false); continue; }
                if lookahead == 47 { state = 660; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 125 { state = 727; lexer.advance(false); continue; }
                return result;
            }
            724 => {
                result = true; lexer.set_result_symbol(aux_sym_ms_asm_block_token1); lexer.mark_end();
                if lookahead == 42 { state = 724; lexer.advance(false); continue; }
                if lookahead == 47 { state = 674; lexer.advance(false); continue; }
                if lookahead == 125 { state = 141; lexer.advance(false); continue; }
                if lookahead != 0 { state = 725; lexer.advance(false); continue; }
                return result;
            }
            725 => {
                result = true; lexer.set_result_symbol(aux_sym_ms_asm_block_token1); lexer.mark_end();
                if lookahead == 42 { state = 724; lexer.advance(false); continue; }
                if lookahead == 125 { state = 141; lexer.advance(false); continue; }
                if lookahead != 0 { state = 725; lexer.advance(false); continue; }
                return result;
            }
            726 => {
                result = true; lexer.set_result_symbol(aux_sym_ms_asm_block_token1); lexer.mark_end();
                if lookahead == 47 { state = 723; lexer.advance(false); continue; }
                if lookahead == 92 { state = 720; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 726; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 125 { state = 727; lexer.advance(false); continue; }
                return result;
            }
            727 => {
                result = true; lexer.set_result_symbol(aux_sym_ms_asm_block_token1); lexer.mark_end();
                if lookahead != 0 && lookahead != 125 { state = 727; lexer.advance(false); continue; }
                return result;
            }
            728 => {
                result = true; lexer.set_result_symbol(anon_sym_ATencode); lexer.mark_end();
                return result;
            }
            729 => {
                result = true; lexer.set_result_symbol(anon_sym_ATsynchronized); lexer.mark_end();
                return result;
            }
            730 => {
                result = true; lexer.set_result_symbol(anon_sym_ATdefs); lexer.mark_end();
                return result;
            }
            731 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON2); lexer.mark_end();
                return result;
            }
            732 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON2); lexer.mark_end();
                if lookahead != 0 && lookahead != 41 { state = 718; lexer.advance(false); continue; }
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
                    (65, 1), (66, 2), (67, 3), (68, 4), (70, 5), (73, 6), (78, 7), (79, 8),
                    (83, 9), (84, 10), (85, 11),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 92 { state = 12; lexer.advance(true); continue; }
                if lookahead == 95 { state = 13; lexer.advance(false); continue; }
                if lookahead == 97 { state = 14; lexer.advance(false); continue; }
                if lookahead == 98 { state = 15; lexer.advance(false); continue; }
                if lookahead == 99 { state = 16; lexer.advance(false); continue; }
                if lookahead == 100 { state = 17; lexer.advance(false); continue; }
                if lookahead == 101 { state = 18; lexer.advance(false); continue; }
                if lookahead == 102 { state = 19; lexer.advance(false); continue; }
                if lookahead == 103 { state = 20; lexer.advance(false); continue; }
                if lookahead == 105 { state = 21; lexer.advance(false); continue; }
                if lookahead == 108 { state = 22; lexer.advance(false); continue; }
                if lookahead == 109 { state = 23; lexer.advance(false); continue; }
                if lookahead == 110 { state = 24; lexer.advance(false); continue; }
                if lookahead == 111 { state = 25; lexer.advance(false); continue; }
                if lookahead == 112 { state = 26; lexer.advance(false); continue; }
                if lookahead == 114 { state = 27; lexer.advance(false); continue; }
                if lookahead == 115 { state = 28; lexer.advance(false); continue; }
                if lookahead == 116 { state = 29; lexer.advance(false); continue; }
                if lookahead == 117 { state = 30; lexer.advance(false); continue; }
                if lookahead == 118 { state = 31; lexer.advance(false); continue; }
                if lookahead == 119 { state = 32; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 || lookahead == 160 { state = 0; lexer.advance(true); continue; }
                return result;
            }
            1 => {
                if lookahead == 80 { state = 33; lexer.advance(false); continue; }
                return result;
            }
            2 => {
                if lookahead == 79 { state = 34; lexer.advance(false); continue; }
                return result;
            }
            3 => {
                if lookahead == 70 { state = 35; lexer.advance(false); continue; }
                if lookahead == 71 { state = 36; lexer.advance(false); continue; }
                if lookahead == 108 { state = 37; lexer.advance(false); continue; }
                return result;
            }
            4 => {
                if lookahead == 69 { state = 38; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                if lookahead == 65 { state = 39; lexer.advance(false); continue; }
                if lookahead == 79 { state = 40; lexer.advance(false); continue; }
                return result;
            }
            6 => {
                if lookahead == 66 { state = 41; lexer.advance(false); continue; }
                if lookahead == 77 { state = 42; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if lookahead == 83 { state = 43; lexer.advance(false); continue; }
                if lookahead == 85 { state = 44; lexer.advance(false); continue; }
                return result;
            }
            8 => {
                if lookahead == 66 { state = 45; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if lookahead == 69 { state = 46; lexer.advance(false); continue; }
                return result;
            }
            10 => {
                if lookahead == 82 { state = 47; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if lookahead == 73 { state = 48; lexer.advance(false); continue; }
                if lookahead == 78 { state = 49; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if lookahead == 10 { state = 0; lexer.advance(true); continue; }
                if lookahead == 13 { state = 50; lexer.advance(true); continue; }
                return result;
            }
            13 => {
                if lookahead == 65 { state = 51; lexer.advance(false); continue; }
                if lookahead == 67 { state = 52; lexer.advance(false); continue; }
                if lookahead == 71 { state = 53; lexer.advance(false); continue; }
                if lookahead == 78 { state = 54; lexer.advance(false); continue; }
                if lookahead == 95 { state = 55; lexer.advance(false); continue; }
                if lookahead == 97 { state = 56; lexer.advance(false); continue; }
                if lookahead == 117 { state = 57; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if lookahead == 108 { state = 58; lexer.advance(false); continue; }
                if lookahead == 115 { state = 59; lexer.advance(false); continue; }
                if lookahead == 117 { state = 60; lexer.advance(false); continue; }
                if lookahead == 118 { state = 61; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if lookahead == 111 { state = 62; lexer.advance(false); continue; }
                if lookahead == 114 { state = 63; lexer.advance(false); continue; }
                if lookahead == 121 { state = 64; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if lookahead == 97 { state = 65; lexer.advance(false); continue; }
                if lookahead == 104 { state = 66; lexer.advance(false); continue; }
                if lookahead == 108 { state = 67; lexer.advance(false); continue; }
                if lookahead == 111 { state = 68; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if lookahead == 101 { state = 69; lexer.advance(false); continue; }
                if lookahead == 111 { state = 70; lexer.advance(false); continue; }
                return result;
            }
            18 => {
                if lookahead == 108 { state = 71; lexer.advance(false); continue; }
                if lookahead == 110 { state = 72; lexer.advance(false); continue; }
                if lookahead == 120 { state = 73; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if lookahead == 97 { state = 74; lexer.advance(false); continue; }
                if lookahead == 108 { state = 75; lexer.advance(false); continue; }
                if lookahead == 111 { state = 76; lexer.advance(false); continue; }
                return result;
            }
            20 => {
                if lookahead == 111 { state = 77; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                if lookahead == 100 { state = 78; lexer.advance(false); continue; }
                if lookahead == 102 { state = 79; lexer.advance(false); continue; }
                if lookahead == 110 { state = 80; lexer.advance(false); continue; }
                if lookahead == 111 { state = 81; lexer.advance(false); continue; }
                return result;
            }
            22 => {
                if lookahead == 111 { state = 82; lexer.advance(false); continue; }
                return result;
            }
            23 => {
                if lookahead == 97 { state = 83; lexer.advance(false); continue; }
                return result;
            }
            24 => {
                if lookahead == 111 { state = 84; lexer.advance(false); continue; }
                if lookahead == 117 { state = 85; lexer.advance(false); continue; }
                return result;
            }
            25 => {
                if lookahead == 98 { state = 86; lexer.advance(false); continue; }
                if lookahead == 102 { state = 87; lexer.advance(false); continue; }
                if lookahead == 110 { state = 88; lexer.advance(false); continue; }
                if lookahead == 117 { state = 89; lexer.advance(false); continue; }
                return result;
            }
            26 => {
                if lookahead == 116 { state = 90; lexer.advance(false); continue; }
                return result;
            }
            27 => {
                if lookahead == 101 { state = 91; lexer.advance(false); continue; }
                return result;
            }
            28 => {
                if lookahead == 104 { state = 92; lexer.advance(false); continue; }
                if lookahead == 105 { state = 93; lexer.advance(false); continue; }
                if lookahead == 115 { state = 94; lexer.advance(false); continue; }
                if lookahead == 116 { state = 95; lexer.advance(false); continue; }
                if lookahead == 119 { state = 96; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                if lookahead == 104 { state = 97; lexer.advance(false); continue; }
                if lookahead == 114 { state = 98; lexer.advance(false); continue; }
                if lookahead == 118 { state = 99; lexer.advance(false); continue; }
                if lookahead == 121 { state = 100; lexer.advance(false); continue; }
                return result;
            }
            30 => {
                if lookahead == 105 { state = 101; lexer.advance(false); continue; }
                if lookahead == 110 { state = 102; lexer.advance(false); continue; }
                return result;
            }
            31 => {
                if lookahead == 97 { state = 103; lexer.advance(false); continue; }
                if lookahead == 111 { state = 104; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                if lookahead == 97 { state = 105; lexer.advance(false); continue; }
                if lookahead == 104 { state = 106; lexer.advance(false); continue; }
                return result;
            }
            33 => {
                if lookahead == 73 { state = 107; lexer.advance(false); continue; }
                return result;
            }
            34 => {
                if lookahead == 79 { state = 108; lexer.advance(false); continue; }
                return result;
            }
            35 => {
                if lookahead == 95 { state = 109; lexer.advance(false); continue; }
                return result;
            }
            36 => {
                if lookahead == 95 { state = 110; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                if lookahead == 97 { state = 111; lexer.advance(false); continue; }
                return result;
            }
            38 => {
                if lookahead == 80 { state = 112; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                if lookahead == 76 { state = 113; lexer.advance(false); continue; }
                return result;
            }
            40 => {
                if lookahead == 85 { state = 114; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                if lookahead == 73 { state = 115; lexer.advance(false); continue; }
                if lookahead == 79 { state = 116; lexer.advance(false); continue; }
                if lookahead == 95 { state = 117; lexer.advance(false); continue; }
                return result;
            }
            42 => {
                if lookahead == 80 { state = 118; lexer.advance(false); continue; }
                return result;
            }
            43 => {
                if lookahead == 95 { state = 119; lexer.advance(false); continue; }
                return result;
            }
            44 => {
                if lookahead == 76 { state = 120; lexer.advance(false); continue; }
                return result;
            }
            45 => {
                if lookahead == 74 { state = 121; lexer.advance(false); continue; }
                return result;
            }
            46 => {
                if lookahead == 76 { state = 122; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                if lookahead == 85 { state = 123; lexer.advance(false); continue; }
                return result;
            }
            48 => {
                if lookahead == 75 { state = 124; lexer.advance(false); continue; }
                if lookahead == 95 { state = 125; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                if lookahead == 65 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            50 => {
                if lookahead == 10 { state = 0; lexer.advance(true); continue; }
                return result;
            }
            51 => {
                if lookahead == 108 { state = 127; lexer.advance(false); continue; }
                if lookahead == 116 { state = 128; lexer.advance(false); continue; }
                return result;
            }
            52 => {
                if lookahead == 111 { state = 129; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                if lookahead == 101 { state = 130; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                if lookahead == 111 { state = 131; lexer.advance(false); continue; }
                if lookahead == 117 { state = 132; lexer.advance(false); continue; }
                return result;
            }
            55 => {
                if let Some(next) = advance_map(&[
                    (73, 133), (79, 134), (97, 135), (98, 136), (99, 137), (100, 138), (101, 139), (102, 140),
                    (105, 141), (107, 142), (110, 143), (112, 144), (114, 145), (115, 146), (116, 147), (117, 148),
                    (118, 149), (119, 150),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            56 => {
                if lookahead == 108 { state = 151; lexer.advance(false); continue; }
                return result;
            }
            57 => {
                if lookahead == 110 { state = 152; lexer.advance(false); continue; }
                return result;
            }
            58 => {
                if lookahead == 105 { state = 153; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                if lookahead == 109 { state = 154; lexer.advance(false); continue; }
                return result;
            }
            60 => {
                if lookahead == 116 { state = 155; lexer.advance(false); continue; }
                return result;
            }
            61 => {
                if lookahead == 97 { state = 156; lexer.advance(false); continue; }
                return result;
            }
            62 => {
                if lookahead == 111 { state = 157; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                if lookahead == 101 { state = 158; lexer.advance(false); continue; }
                return result;
            }
            64 => {
                if lookahead == 99 { state = 159; lexer.advance(false); continue; }
                if lookahead == 114 { state = 160; lexer.advance(false); continue; }
                return result;
            }
            65 => {
                if lookahead == 115 { state = 161; lexer.advance(false); continue; }
                return result;
            }
            66 => {
                if lookahead == 97 { state = 162; lexer.advance(false); continue; }
                return result;
            }
            67 => {
                if lookahead == 97 { state = 163; lexer.advance(false); continue; }
                return result;
            }
            68 => {
                if lookahead == 110 { state = 164; lexer.advance(false); continue; }
                return result;
            }
            69 => {
                if lookahead == 102 { state = 165; lexer.advance(false); continue; }
                return result;
            }
            70 => {
                result = true; lexer.set_result_symbol(anon_sym_do); lexer.mark_end();
                if lookahead == 117 { state = 166; lexer.advance(false); continue; }
                return result;
            }
            71 => {
                if lookahead == 115 { state = 167; lexer.advance(false); continue; }
                return result;
            }
            72 => {
                if lookahead == 117 { state = 168; lexer.advance(false); continue; }
                return result;
            }
            73 => {
                if lookahead == 116 { state = 169; lexer.advance(false); continue; }
                return result;
            }
            74 => {
                if lookahead == 108 { state = 170; lexer.advance(false); continue; }
                return result;
            }
            75 => {
                if lookahead == 111 { state = 171; lexer.advance(false); continue; }
                return result;
            }
            76 => {
                if lookahead == 114 { state = 172; lexer.advance(false); continue; }
                return result;
            }
            77 => {
                if lookahead == 116 { state = 173; lexer.advance(false); continue; }
                return result;
            }
            78 => {
                result = true; lexer.set_result_symbol(anon_sym_id); lexer.mark_end();
                return result;
            }
            79 => {
                result = true; lexer.set_result_symbol(anon_sym_if); lexer.mark_end();
                return result;
            }
            80 => {
                result = true; lexer.set_result_symbol(anon_sym_in); lexer.mark_end();
                if lookahead == 108 { state = 174; lexer.advance(false); continue; }
                if lookahead == 111 { state = 175; lexer.advance(false); continue; }
                if lookahead == 116 { state = 176; lexer.advance(false); continue; }
                return result;
            }
            81 => {
                if lookahead == 115 { state = 177; lexer.advance(false); continue; }
                return result;
            }
            82 => {
                if lookahead == 110 { state = 178; lexer.advance(false); continue; }
                return result;
            }
            83 => {
                if lookahead == 99 { state = 179; lexer.advance(false); continue; }
                if lookahead == 120 { state = 180; lexer.advance(false); continue; }
                return result;
            }
            84 => {
                if lookahead == 114 { state = 181; lexer.advance(false); continue; }
                if lookahead == 116 { state = 182; lexer.advance(false); continue; }
                return result;
            }
            85 => {
                if lookahead == 108 { state = 183; lexer.advance(false); continue; }
                return result;
            }
            86 => {
                if lookahead == 106 { state = 184; lexer.advance(false); continue; }
                return result;
            }
            87 => {
                if lookahead == 102 { state = 185; lexer.advance(false); continue; }
                return result;
            }
            88 => {
                if lookahead == 101 { state = 186; lexer.advance(false); continue; }
                return result;
            }
            89 => {
                if lookahead == 116 { state = 187; lexer.advance(false); continue; }
                return result;
            }
            90 => {
                if lookahead == 114 { state = 188; lexer.advance(false); continue; }
                return result;
            }
            91 => {
                if lookahead == 103 { state = 189; lexer.advance(false); continue; }
                if lookahead == 115 { state = 190; lexer.advance(false); continue; }
                if lookahead == 116 { state = 191; lexer.advance(false); continue; }
                return result;
            }
            92 => {
                if lookahead == 111 { state = 192; lexer.advance(false); continue; }
                return result;
            }
            93 => {
                if lookahead == 103 { state = 193; lexer.advance(false); continue; }
                if lookahead == 122 { state = 194; lexer.advance(false); continue; }
                return result;
            }
            94 => {
                if lookahead == 105 { state = 195; lexer.advance(false); continue; }
                return result;
            }
            95 => {
                if lookahead == 97 { state = 196; lexer.advance(false); continue; }
                if lookahead == 114 { state = 197; lexer.advance(false); continue; }
                return result;
            }
            96 => {
                if lookahead == 105 { state = 198; lexer.advance(false); continue; }
                return result;
            }
            97 => {
                if lookahead == 114 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            98 => {
                if lookahead == 117 { state = 200; lexer.advance(false); continue; }
                return result;
            }
            99 => {
                if lookahead == 111 { state = 201; lexer.advance(false); continue; }
                return result;
            }
            100 => {
                if lookahead == 112 { state = 202; lexer.advance(false); continue; }
                return result;
            }
            101 => {
                if lookahead == 110 { state = 203; lexer.advance(false); continue; }
                return result;
            }
            102 => {
                if lookahead == 105 { state = 204; lexer.advance(false); continue; }
                if lookahead == 115 { state = 205; lexer.advance(false); continue; }
                return result;
            }
            103 => {
                if lookahead == 95 { state = 206; lexer.advance(false); continue; }
                return result;
            }
            104 => {
                if lookahead == 105 { state = 207; lexer.advance(false); continue; }
                if lookahead == 108 { state = 208; lexer.advance(false); continue; }
                return result;
            }
            105 => {
                if lookahead == 116 { state = 209; lexer.advance(false); continue; }
                return result;
            }
            106 => {
                if lookahead == 105 { state = 210; lexer.advance(false); continue; }
                return result;
            }
            107 => {
                if lookahead == 95 { state = 211; lexer.advance(false); continue; }
                return result;
            }
            108 => {
                if lookahead == 76 { state = 212; lexer.advance(false); continue; }
                return result;
            }
            109 => {
                if lookahead == 70 { state = 213; lexer.advance(false); continue; }
                if lookahead == 82 { state = 214; lexer.advance(false); continue; }
                return result;
            }
            110 => {
                if lookahead == 69 { state = 215; lexer.advance(false); continue; }
                if lookahead == 73 { state = 216; lexer.advance(false); continue; }
                return result;
            }
            111 => {
                if lookahead == 115 { state = 217; lexer.advance(false); continue; }
                return result;
            }
            112 => {
                if lookahead == 82 { state = 218; lexer.advance(false); continue; }
                return result;
            }
            113 => {
                if lookahead == 83 { state = 219; lexer.advance(false); continue; }
                return result;
            }
            114 => {
                if lookahead == 78 { state = 220; lexer.advance(false); continue; }
                return result;
            }
            115 => {
                if lookahead == 110 { state = 221; lexer.advance(false); continue; }
                return result;
            }
            116 => {
                if lookahead == 117 { state = 222; lexer.advance(false); continue; }
                return result;
            }
            117 => {
                if lookahead == 68 { state = 223; lexer.advance(false); continue; }
                return result;
            }
            118 => {
                result = true; lexer.set_result_symbol(anon_sym_IMP); lexer.mark_end();
                return result;
            }
            119 => {
                if let Some(next) = advance_map(&[
                    (65, 224), (67, 225), (68, 226), (69, 227), (70, 228), (73, 229), (82, 230), (83, 231),
                    (85, 232), (86, 233),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            120 => {
                if lookahead == 76 { state = 234; lexer.advance(false); continue; }
                return result;
            }
            121 => {
                if lookahead == 67 { state = 235; lexer.advance(false); continue; }
                return result;
            }
            122 => {
                result = true; lexer.set_result_symbol(anon_sym_SEL); lexer.mark_end();
                return result;
            }
            123 => {
                if lookahead == 69 { state = 236; lexer.advance(false); continue; }
                return result;
            }
            124 => {
                if lookahead == 73 { state = 237; lexer.advance(false); continue; }
                return result;
            }
            125 => {
                if lookahead == 65 { state = 238; lexer.advance(false); continue; }
                return result;
            }
            126 => {
                if lookahead == 86 { state = 239; lexer.advance(false); continue; }
                return result;
            }
            127 => {
                if lookahead == 105 { state = 240; lexer.advance(false); continue; }
                return result;
            }
            128 => {
                if lookahead == 111 { state = 241; lexer.advance(false); continue; }
                return result;
            }
            129 => {
                if lookahead == 109 { state = 242; lexer.advance(false); continue; }
                return result;
            }
            130 => {
                if lookahead == 110 { state = 243; lexer.advance(false); continue; }
                return result;
            }
            131 => {
                if lookahead == 110 { state = 244; lexer.advance(false); continue; }
                if lookahead == 114 { state = 245; lexer.advance(false); continue; }
                return result;
            }
            132 => {
                if lookahead == 108 { state = 246; lexer.advance(false); continue; }
                return result;
            }
            133 => {
                if lookahead == 79 { state = 247; lexer.advance(false); continue; }
                return result;
            }
            134 => {
                if lookahead == 83 { state = 248; lexer.advance(false); continue; }
                return result;
            }
            135 => {
                if lookahead == 108 { state = 249; lexer.advance(false); continue; }
                if lookahead == 115 { state = 250; lexer.advance(false); continue; }
                if lookahead == 116 { state = 251; lexer.advance(false); continue; }
                if lookahead == 117 { state = 252; lexer.advance(false); continue; }
                return result;
            }
            136 => {
                if lookahead == 97 { state = 253; lexer.advance(false); continue; }
                if lookahead == 108 { state = 254; lexer.advance(false); continue; }
                if lookahead == 114 { state = 255; lexer.advance(false); continue; }
                if lookahead == 117 { state = 256; lexer.advance(false); continue; }
                return result;
            }
            137 => {
                if lookahead == 97 { state = 257; lexer.advance(false); continue; }
                if lookahead == 100 { state = 258; lexer.advance(false); continue; }
                if lookahead == 108 { state = 259; lexer.advance(false); continue; }
                if lookahead == 111 { state = 260; lexer.advance(false); continue; }
                return result;
            }
            138 => {
                if lookahead == 101 { state = 261; lexer.advance(false); continue; }
                return result;
            }
            139 => {
                if lookahead == 120 { state = 262; lexer.advance(false); continue; }
                return result;
            }
            140 => {
                if lookahead == 97 { state = 263; lexer.advance(false); continue; }
                if lookahead == 105 { state = 264; lexer.advance(false); continue; }
                if lookahead == 111 { state = 265; lexer.advance(false); continue; }
                return result;
            }
            141 => {
                if lookahead == 109 { state = 266; lexer.advance(false); continue; }
                if lookahead == 110 { state = 267; lexer.advance(false); continue; }
                return result;
            }
            142 => {
                if lookahead == 105 { state = 268; lexer.advance(false); continue; }
                return result;
            }
            143 => {
                if lookahead == 111 { state = 269; lexer.advance(false); continue; }
                if lookahead == 117 { state = 270; lexer.advance(false); continue; }
                return result;
            }
            144 => {
                if lookahead == 116 { state = 271; lexer.advance(false); continue; }
                return result;
            }
            145 => {
                if lookahead == 101 { state = 272; lexer.advance(false); continue; }
                return result;
            }
            146 => {
                if lookahead == 112 { state = 273; lexer.advance(false); continue; }
                if lookahead == 116 { state = 274; lexer.advance(false); continue; }
                return result;
            }
            147 => {
                if lookahead == 104 { state = 275; lexer.advance(false); continue; }
                if lookahead == 114 { state = 276; lexer.advance(false); continue; }
                if lookahead == 121 { state = 277; lexer.advance(false); continue; }
                return result;
            }
            148 => {
                if lookahead == 110 { state = 278; lexer.advance(false); continue; }
                if lookahead == 112 { state = 279; lexer.advance(false); continue; }
                return result;
            }
            149 => {
                if lookahead == 101 { state = 280; lexer.advance(false); continue; }
                if lookahead == 111 { state = 281; lexer.advance(false); continue; }
                return result;
            }
            150 => {
                if lookahead == 101 { state = 282; lexer.advance(false); continue; }
                return result;
            }
            151 => {
                if lookahead == 105 { state = 283; lexer.advance(false); continue; }
                return result;
            }
            152 => {
                if lookahead == 97 { state = 284; lexer.advance(false); continue; }
                return result;
            }
            153 => {
                if lookahead == 103 { state = 285; lexer.advance(false); continue; }
                return result;
            }
            154 => {
                result = true; lexer.set_result_symbol(anon_sym_asm); lexer.mark_end();
                return result;
            }
            155 => {
                if lookahead == 111 { state = 286; lexer.advance(false); continue; }
                return result;
            }
            156 => {
                if lookahead == 105 { state = 287; lexer.advance(false); continue; }
                return result;
            }
            157 => {
                if lookahead == 108 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            158 => {
                if lookahead == 97 { state = 289; lexer.advance(false); continue; }
                return result;
            }
            159 => {
                if lookahead == 111 { state = 290; lexer.advance(false); continue; }
                return result;
            }
            160 => {
                if lookahead == 101 { state = 291; lexer.advance(false); continue; }
                return result;
            }
            161 => {
                if lookahead == 101 { state = 292; lexer.advance(false); continue; }
                return result;
            }
            162 => {
                if lookahead == 114 { state = 293; lexer.advance(false); continue; }
                return result;
            }
            163 => {
                if lookahead == 115 { state = 294; lexer.advance(false); continue; }
                return result;
            }
            164 => {
                if lookahead == 115 { state = 295; lexer.advance(false); continue; }
                if lookahead == 116 { state = 296; lexer.advance(false); continue; }
                return result;
            }
            165 => {
                if lookahead == 97 { state = 297; lexer.advance(false); continue; }
                if lookahead == 105 { state = 298; lexer.advance(false); continue; }
                return result;
            }
            166 => {
                if lookahead == 98 { state = 299; lexer.advance(false); continue; }
                return result;
            }
            167 => {
                if lookahead == 101 { state = 300; lexer.advance(false); continue; }
                return result;
            }
            168 => {
                if lookahead == 109 { state = 301; lexer.advance(false); continue; }
                return result;
            }
            169 => {
                if lookahead == 101 { state = 302; lexer.advance(false); continue; }
                return result;
            }
            170 => {
                if lookahead == 115 { state = 303; lexer.advance(false); continue; }
                return result;
            }
            171 => {
                if lookahead == 97 { state = 304; lexer.advance(false); continue; }
                return result;
            }
            172 => {
                result = true; lexer.set_result_symbol(anon_sym_for); lexer.mark_end();
                return result;
            }
            173 => {
                if lookahead == 111 { state = 305; lexer.advance(false); continue; }
                return result;
            }
            174 => {
                if lookahead == 105 { state = 306; lexer.advance(false); continue; }
                return result;
            }
            175 => {
                if lookahead == 117 { state = 307; lexer.advance(false); continue; }
                return result;
            }
            176 => {
                result = true; lexer.set_result_symbol(sym_primitive_type); lexer.mark_end();
                if lookahead == 49 { state = 308; lexer.advance(false); continue; }
                if lookahead == 51 { state = 309; lexer.advance(false); continue; }
                if lookahead == 54 { state = 310; lexer.advance(false); continue; }
                if lookahead == 56 { state = 311; lexer.advance(false); continue; }
                if lookahead == 112 { state = 312; lexer.advance(false); continue; }
                return result;
            }
            177 => {
                result = true; lexer.set_result_symbol(anon_sym_ios); lexer.mark_end();
                return result;
            }
            178 => {
                if lookahead == 103 { state = 313; lexer.advance(false); continue; }
                return result;
            }
            179 => {
                if lookahead == 111 { state = 314; lexer.advance(false); continue; }
                return result;
            }
            180 => {
                if lookahead == 95 { state = 315; lexer.advance(false); continue; }
                return result;
            }
            181 => {
                if lookahead == 101 { state = 316; lexer.advance(false); continue; }
                return result;
            }
            182 => {
                if lookahead == 104 { state = 317; lexer.advance(false); continue; }
                return result;
            }
            183 => {
                if lookahead == 108 { state = 318; lexer.advance(false); continue; }
                return result;
            }
            184 => {
                if lookahead == 99 { state = 319; lexer.advance(false); continue; }
                return result;
            }
            185 => {
                if lookahead == 115 { state = 320; lexer.advance(false); continue; }
                return result;
            }
            186 => {
                if lookahead == 119 { state = 321; lexer.advance(false); continue; }
                return result;
            }
            187 => {
                result = true; lexer.set_result_symbol(anon_sym_out); lexer.mark_end();
                return result;
            }
            188 => {
                if lookahead == 100 { state = 322; lexer.advance(false); continue; }
                return result;
            }
            189 => {
                if lookahead == 105 { state = 323; lexer.advance(false); continue; }
                return result;
            }
            190 => {
                if lookahead == 116 { state = 324; lexer.advance(false); continue; }
                return result;
            }
            191 => {
                if lookahead == 117 { state = 325; lexer.advance(false); continue; }
                return result;
            }
            192 => {
                if lookahead == 114 { state = 326; lexer.advance(false); continue; }
                return result;
            }
            193 => {
                if lookahead == 110 { state = 327; lexer.advance(false); continue; }
                return result;
            }
            194 => {
                if lookahead == 101 { state = 328; lexer.advance(false); continue; }
                return result;
            }
            195 => {
                if lookahead == 122 { state = 329; lexer.advance(false); continue; }
                return result;
            }
            196 => {
                if lookahead == 116 { state = 330; lexer.advance(false); continue; }
                return result;
            }
            197 => {
                if lookahead == 117 { state = 331; lexer.advance(false); continue; }
                return result;
            }
            198 => {
                if lookahead == 116 { state = 332; lexer.advance(false); continue; }
                return result;
            }
            199 => {
                if lookahead == 101 { state = 333; lexer.advance(false); continue; }
                return result;
            }
            200 => {
                if lookahead == 101 { state = 236; lexer.advance(false); continue; }
                return result;
            }
            201 => {
                if lookahead == 115 { state = 334; lexer.advance(false); continue; }
                return result;
            }
            202 => {
                if lookahead == 101 { state = 335; lexer.advance(false); continue; }
                return result;
            }
            203 => {
                if lookahead == 116 { state = 336; lexer.advance(false); continue; }
                return result;
            }
            204 => {
                if lookahead == 111 { state = 337; lexer.advance(false); continue; }
                return result;
            }
            205 => {
                if lookahead == 105 { state = 338; lexer.advance(false); continue; }
                return result;
            }
            206 => {
                if lookahead == 97 { state = 339; lexer.advance(false); continue; }
                return result;
            }
            207 => {
                if lookahead == 100 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            208 => {
                if lookahead == 97 { state = 340; lexer.advance(false); continue; }
                return result;
            }
            209 => {
                if lookahead == 99 { state = 341; lexer.advance(false); continue; }
                return result;
            }
            210 => {
                if lookahead == 108 { state = 342; lexer.advance(false); continue; }
                return result;
            }
            211 => {
                if lookahead == 65 { state = 343; lexer.advance(false); continue; }
                if lookahead == 68 { state = 344; lexer.advance(false); continue; }
                if lookahead == 85 { state = 345; lexer.advance(false); continue; }
                return result;
            }
            212 => {
                result = true; lexer.set_result_symbol(anon_sym_BOOL); lexer.mark_end();
                return result;
            }
            213 => {
                if lookahead == 79 { state = 346; lexer.advance(false); continue; }
                return result;
            }
            214 => {
                if lookahead == 69 { state = 347; lexer.advance(false); continue; }
                return result;
            }
            215 => {
                if lookahead == 88 { state = 348; lexer.advance(false); continue; }
                return result;
            }
            216 => {
                if lookahead == 78 { state = 349; lexer.advance(false); continue; }
                return result;
            }
            217 => {
                if lookahead == 115 { state = 350; lexer.advance(false); continue; }
                return result;
            }
            218 => {
                if lookahead == 69 { state = 351; lexer.advance(false); continue; }
                return result;
            }
            219 => {
                if lookahead == 69 { state = 352; lexer.advance(false); continue; }
                return result;
            }
            220 => {
                if lookahead == 68 { state = 353; lexer.advance(false); continue; }
                return result;
            }
            221 => {
                if lookahead == 115 { state = 354; lexer.advance(false); continue; }
                return result;
            }
            222 => {
                if lookahead == 116 { state = 355; lexer.advance(false); continue; }
                return result;
            }
            223 => {
                if lookahead == 69 { state = 356; lexer.advance(false); continue; }
                return result;
            }
            224 => {
                if lookahead == 85 { state = 357; lexer.advance(false); continue; }
                if lookahead == 86 { state = 358; lexer.advance(false); continue; }
                return result;
            }
            225 => {
                if lookahead == 76 { state = 359; lexer.advance(false); continue; }
                return result;
            }
            226 => {
                if lookahead == 69 { state = 360; lexer.advance(false); continue; }
                return result;
            }
            227 => {
                if lookahead == 78 { state = 361; lexer.advance(false); continue; }
                if lookahead == 88 { state = 362; lexer.advance(false); continue; }
                return result;
            }
            228 => {
                if lookahead == 79 { state = 363; lexer.advance(false); continue; }
                return result;
            }
            229 => {
                if lookahead == 78 { state = 364; lexer.advance(false); continue; }
                return result;
            }
            230 => {
                if lookahead == 69 { state = 365; lexer.advance(false); continue; }
                if lookahead == 79 { state = 366; lexer.advance(false); continue; }
                return result;
            }
            231 => {
                if lookahead == 87 { state = 367; lexer.advance(false); continue; }
                return result;
            }
            232 => {
                if lookahead == 78 { state = 368; lexer.advance(false); continue; }
                return result;
            }
            233 => {
                if lookahead == 65 { state = 369; lexer.advance(false); continue; }
                return result;
            }
            234 => {
                result = true; lexer.set_result_symbol(anon_sym_NULL); lexer.mark_end();
                return result;
            }
            235 => {
                if lookahead == 95 { state = 370; lexer.advance(false); continue; }
                return result;
            }
            236 => {
                result = true; lexer.set_result_symbol(sym_true); lexer.mark_end();
                return result;
            }
            237 => {
                if lookahead == 84 { state = 371; lexer.advance(false); continue; }
                return result;
            }
            238 => {
                if lookahead == 80 { state = 372; lexer.advance(false); continue; }
                return result;
            }
            239 => {
                if lookahead == 65 { state = 373; lexer.advance(false); continue; }
                return result;
            }
            240 => {
                if lookahead == 103 { state = 374; lexer.advance(false); continue; }
                return result;
            }
            241 => {
                if lookahead == 109 { state = 375; lexer.advance(false); continue; }
                return result;
            }
            242 => {
                if lookahead == 112 { state = 376; lexer.advance(false); continue; }
                return result;
            }
            243 => {
                if lookahead == 101 { state = 377; lexer.advance(false); continue; }
                return result;
            }
            244 => {
                if lookahead == 110 { state = 378; lexer.advance(false); continue; }
                return result;
            }
            245 => {
                if lookahead == 101 { state = 379; lexer.advance(false); continue; }
                return result;
            }
            246 => {
                if lookahead == 108 { state = 380; lexer.advance(false); continue; }
                return result;
            }
            247 => {
                if lookahead == 83 { state = 381; lexer.advance(false); continue; }
                return result;
            }
            248 => {
                if lookahead == 88 { state = 382; lexer.advance(false); continue; }
                return result;
            }
            249 => {
                if lookahead == 105 { state = 383; lexer.advance(false); continue; }
                return result;
            }
            250 => {
                if lookahead == 109 { state = 384; lexer.advance(false); continue; }
                return result;
            }
            251 => {
                if lookahead == 116 { state = 385; lexer.advance(false); continue; }
                return result;
            }
            252 => {
                if lookahead == 116 { state = 386; lexer.advance(false); continue; }
                return result;
            }
            253 => {
                if lookahead == 115 { state = 387; lexer.advance(false); continue; }
                return result;
            }
            254 => {
                if lookahead == 111 { state = 388; lexer.advance(false); continue; }
                return result;
            }
            255 => {
                if lookahead == 105 { state = 389; lexer.advance(false); continue; }
                return result;
            }
            256 => {
                if lookahead == 105 { state = 390; lexer.advance(false); continue; }
                return result;
            }
            257 => {
                if lookahead == 116 { state = 391; lexer.advance(false); continue; }
                return result;
            }
            258 => {
                if lookahead == 101 { state = 392; lexer.advance(false); continue; }
                return result;
            }
            259 => {
                if lookahead == 114 { state = 393; lexer.advance(false); continue; }
                return result;
            }
            260 => {
                if lookahead == 109 { state = 394; lexer.advance(false); continue; }
                if lookahead == 110 { state = 395; lexer.advance(false); continue; }
                if lookahead == 118 { state = 396; lexer.advance(false); continue; }
                return result;
            }
            261 => {
                if lookahead == 99 { state = 397; lexer.advance(false); continue; }
                if lookahead == 112 { state = 398; lexer.advance(false); continue; }
                return result;
            }
            262 => {
                if lookahead == 116 { state = 399; lexer.advance(false); continue; }
                return result;
            }
            263 => {
                if lookahead == 115 { state = 400; lexer.advance(false); continue; }
                return result;
            }
            264 => {
                if lookahead == 110 { state = 401; lexer.advance(false); continue; }
                return result;
            }
            265 => {
                if lookahead == 114 { state = 402; lexer.advance(false); continue; }
                return result;
            }
            266 => {
                if lookahead == 97 { state = 403; lexer.advance(false); continue; }
                return result;
            }
            267 => {
                if lookahead == 108 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            268 => {
                if lookahead == 110 { state = 405; lexer.advance(false); continue; }
                return result;
            }
            269 => {
                if lookahead == 110 { state = 406; lexer.advance(false); continue; }
                return result;
            }
            270 => {
                if lookahead == 108 { state = 407; lexer.advance(false); continue; }
                return result;
            }
            271 => {
                if lookahead == 114 { state = 408; lexer.advance(false); continue; }
                return result;
            }
            272 => {
                if lookahead == 97 { state = 409; lexer.advance(false); continue; }
                if lookahead == 115 { state = 410; lexer.advance(false); continue; }
                return result;
            }
            273 => {
                if lookahead == 116 { state = 411; lexer.advance(false); continue; }
                return result;
            }
            274 => {
                if lookahead == 100 { state = 412; lexer.advance(false); continue; }
                if lookahead == 114 { state = 413; lexer.advance(false); continue; }
                return result;
            }
            275 => {
                if lookahead == 105 { state = 414; lexer.advance(false); continue; }
                if lookahead == 114 { state = 415; lexer.advance(false); continue; }
                return result;
            }
            276 => {
                if lookahead == 121 { state = 416; lexer.advance(false); continue; }
                return result;
            }
            277 => {
                if lookahead == 112 { state = 417; lexer.advance(false); continue; }
                return result;
            }
            278 => {
                if lookahead == 97 { state = 418; lexer.advance(false); continue; }
                if lookahead == 115 { state = 419; lexer.advance(false); continue; }
                if lookahead == 117 { state = 420; lexer.advance(false); continue; }
                return result;
            }
            279 => {
                if lookahead == 116 { state = 421; lexer.advance(false); continue; }
                return result;
            }
            280 => {
                if lookahead == 99 { state = 422; lexer.advance(false); continue; }
                return result;
            }
            281 => {
                if lookahead == 108 { state = 423; lexer.advance(false); continue; }
                return result;
            }
            282 => {
                if lookahead == 97 { state = 424; lexer.advance(false); continue; }
                return result;
            }
            283 => {
                if lookahead == 103 { state = 425; lexer.advance(false); continue; }
                return result;
            }
            284 => {
                if lookahead == 108 { state = 426; lexer.advance(false); continue; }
                return result;
            }
            285 => {
                if lookahead == 110 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            286 => {
                result = true; lexer.set_result_symbol(anon_sym_auto); lexer.mark_end();
                return result;
            }
            287 => {
                if lookahead == 108 { state = 428; lexer.advance(false); continue; }
                return result;
            }
            288 => {
                result = true; lexer.set_result_symbol(sym_primitive_type); lexer.mark_end();
                return result;
            }
            289 => {
                if lookahead == 107 { state = 429; lexer.advance(false); continue; }
                return result;
            }
            290 => {
                if lookahead == 112 { state = 430; lexer.advance(false); continue; }
                return result;
            }
            291 => {
                if lookahead == 102 { state = 431; lexer.advance(false); continue; }
                return result;
            }
            292 => {
                result = true; lexer.set_result_symbol(anon_sym_case); lexer.mark_end();
                return result;
            }
            293 => {
                result = true; lexer.set_result_symbol(sym_primitive_type); lexer.mark_end();
                if lookahead == 49 { state = 432; lexer.advance(false); continue; }
                if lookahead == 51 { state = 433; lexer.advance(false); continue; }
                if lookahead == 54 { state = 434; lexer.advance(false); continue; }
                if lookahead == 56 { state = 435; lexer.advance(false); continue; }
                if lookahead == 112 { state = 436; lexer.advance(false); continue; }
                return result;
            }
            294 => {
                if lookahead == 115 { state = 437; lexer.advance(false); continue; }
                return result;
            }
            295 => {
                if lookahead == 116 { state = 438; lexer.advance(false); continue; }
                return result;
            }
            296 => {
                if lookahead == 105 { state = 439; lexer.advance(false); continue; }
                return result;
            }
            297 => {
                if lookahead == 117 { state = 440; lexer.advance(false); continue; }
                return result;
            }
            298 => {
                if lookahead == 110 { state = 441; lexer.advance(false); continue; }
                return result;
            }
            299 => {
                if lookahead == 108 { state = 442; lexer.advance(false); continue; }
                return result;
            }
            300 => {
                result = true; lexer.set_result_symbol(anon_sym_else); lexer.mark_end();
                return result;
            }
            301 => {
                result = true; lexer.set_result_symbol(anon_sym_enum); lexer.mark_end();
                return result;
            }
            302 => {
                if lookahead == 114 { state = 443; lexer.advance(false); continue; }
                return result;
            }
            303 => {
                if lookahead == 101 { state = 352; lexer.advance(false); continue; }
                return result;
            }
            304 => {
                if lookahead == 116 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            305 => {
                result = true; lexer.set_result_symbol(anon_sym_goto); lexer.mark_end();
                return result;
            }
            306 => {
                if lookahead == 110 { state = 444; lexer.advance(false); continue; }
                return result;
            }
            307 => {
                if lookahead == 116 { state = 445; lexer.advance(false); continue; }
                return result;
            }
            308 => {
                if lookahead == 54 { state = 446; lexer.advance(false); continue; }
                return result;
            }
            309 => {
                if lookahead == 50 { state = 447; lexer.advance(false); continue; }
                return result;
            }
            310 => {
                if lookahead == 52 { state = 448; lexer.advance(false); continue; }
                return result;
            }
            311 => {
                if lookahead == 95 { state = 449; lexer.advance(false); continue; }
                return result;
            }
            312 => {
                if lookahead == 116 { state = 450; lexer.advance(false); continue; }
                return result;
            }
            313 => {
                result = true; lexer.set_result_symbol(anon_sym_long); lexer.mark_end();
                return result;
            }
            314 => {
                if lookahead == 115 { state = 451; lexer.advance(false); continue; }
                return result;
            }
            315 => {
                if lookahead == 97 { state = 452; lexer.advance(false); continue; }
                return result;
            }
            316 => {
                if lookahead == 116 { state = 453; lexer.advance(false); continue; }
                return result;
            }
            317 => {
                if lookahead == 114 { state = 454; lexer.advance(false); continue; }
                return result;
            }
            318 => {
                if lookahead == 97 { state = 455; lexer.advance(false); continue; }
                if lookahead == 112 { state = 456; lexer.advance(false); continue; }
                return result;
            }
            319 => {
                if lookahead == 95 { state = 457; lexer.advance(false); continue; }
                return result;
            }
            320 => {
                if lookahead == 101 { state = 458; lexer.advance(false); continue; }
                return result;
            }
            321 => {
                if lookahead == 97 { state = 459; lexer.advance(false); continue; }
                return result;
            }
            322 => {
                if lookahead == 105 { state = 460; lexer.advance(false); continue; }
                return result;
            }
            323 => {
                if lookahead == 115 { state = 461; lexer.advance(false); continue; }
                return result;
            }
            324 => {
                if lookahead == 114 { state = 462; lexer.advance(false); continue; }
                return result;
            }
            325 => {
                if lookahead == 114 { state = 463; lexer.advance(false); continue; }
                return result;
            }
            326 => {
                if lookahead == 116 { state = 464; lexer.advance(false); continue; }
                return result;
            }
            327 => {
                if lookahead == 101 { state = 465; lexer.advance(false); continue; }
                return result;
            }
            328 => {
                if lookahead == 95 { state = 466; lexer.advance(false); continue; }
                if lookahead == 111 { state = 467; lexer.advance(false); continue; }
                return result;
            }
            329 => {
                if lookahead == 101 { state = 468; lexer.advance(false); continue; }
                return result;
            }
            330 => {
                if lookahead == 105 { state = 469; lexer.advance(false); continue; }
                return result;
            }
            331 => {
                if lookahead == 99 { state = 470; lexer.advance(false); continue; }
                return result;
            }
            332 => {
                if lookahead == 99 { state = 471; lexer.advance(false); continue; }
                return result;
            }
            333 => {
                if lookahead == 97 { state = 472; lexer.advance(false); continue; }
                return result;
            }
            334 => {
                result = true; lexer.set_result_symbol(anon_sym_tvos); lexer.mark_end();
                return result;
            }
            335 => {
                if lookahead == 100 { state = 473; lexer.advance(false); continue; }
                if lookahead == 111 { state = 474; lexer.advance(false); continue; }
                return result;
            }
            336 => {
                if lookahead == 49 { state = 475; lexer.advance(false); continue; }
                if lookahead == 51 { state = 476; lexer.advance(false); continue; }
                if lookahead == 54 { state = 477; lexer.advance(false); continue; }
                if lookahead == 56 { state = 478; lexer.advance(false); continue; }
                if lookahead == 112 { state = 479; lexer.advance(false); continue; }
                return result;
            }
            337 => {
                if lookahead == 110 { state = 480; lexer.advance(false); continue; }
                return result;
            }
            338 => {
                if lookahead == 103 { state = 481; lexer.advance(false); continue; }
                return result;
            }
            339 => {
                if lookahead == 114 { state = 482; lexer.advance(false); continue; }
                return result;
            }
            340 => {
                if lookahead == 116 { state = 483; lexer.advance(false); continue; }
                return result;
            }
            341 => {
                if lookahead == 104 { state = 484; lexer.advance(false); continue; }
                return result;
            }
            342 => {
                if lookahead == 101 { state = 485; lexer.advance(false); continue; }
                return result;
            }
            343 => {
                if lookahead == 86 { state = 486; lexer.advance(false); continue; }
                return result;
            }
            344 => {
                if lookahead == 69 { state = 487; lexer.advance(false); continue; }
                return result;
            }
            345 => {
                if lookahead == 78 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            346 => {
                if lookahead == 82 { state = 489; lexer.advance(false); continue; }
                return result;
            }
            347 => {
                if lookahead == 84 { state = 490; lexer.advance(false); continue; }
                return result;
            }
            348 => {
                if lookahead == 84 { state = 491; lexer.advance(false); continue; }
                return result;
            }
            349 => {
                if lookahead == 76 { state = 492; lexer.advance(false); continue; }
                return result;
            }
            350 => {
                result = true; lexer.set_result_symbol(anon_sym_Class); lexer.mark_end();
                return result;
            }
            351 => {
                if lookahead == 67 { state = 493; lexer.advance(false); continue; }
                return result;
            }
            352 => {
                result = true; lexer.set_result_symbol(sym_false); lexer.mark_end();
                return result;
            }
            353 => {
                if lookahead == 65 { state = 494; lexer.advance(false); continue; }
                return result;
            }
            354 => {
                if lookahead == 112 { state = 495; lexer.advance(false); continue; }
                return result;
            }
            355 => {
                if lookahead == 108 { state = 496; lexer.advance(false); continue; }
                return result;
            }
            356 => {
                if lookahead == 83 { state = 497; lexer.advance(false); continue; }
                return result;
            }
            357 => {
                if lookahead == 84 { state = 498; lexer.advance(false); continue; }
                return result;
            }
            358 => {
                if lookahead == 65 { state = 499; lexer.advance(false); continue; }
                return result;
            }
            359 => {
                if lookahead == 65 { state = 500; lexer.advance(false); continue; }
                return result;
            }
            360 => {
                if lookahead == 80 { state = 501; lexer.advance(false); continue; }
                return result;
            }
            361 => {
                if lookahead == 85 { state = 502; lexer.advance(false); continue; }
                return result;
            }
            362 => {
                if lookahead == 84 { state = 503; lexer.advance(false); continue; }
                return result;
            }
            363 => {
                if lookahead == 82 { state = 504; lexer.advance(false); continue; }
                return result;
            }
            364 => {
                if lookahead == 76 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            365 => {
                if lookahead == 81 { state = 506; lexer.advance(false); continue; }
                return result;
            }
            366 => {
                if lookahead == 79 { state = 507; lexer.advance(false); continue; }
                return result;
            }
            367 => {
                if lookahead == 73 { state = 508; lexer.advance(false); continue; }
                return result;
            }
            368 => {
                if lookahead == 65 { state = 509; lexer.advance(false); continue; }
                return result;
            }
            369 => {
                if lookahead == 76 { state = 510; lexer.advance(false); continue; }
                return result;
            }
            370 => {
                if lookahead == 69 { state = 511; lexer.advance(false); continue; }
                if lookahead == 82 { state = 512; lexer.advance(false); continue; }
                return result;
            }
            371 => {
                if lookahead == 95 { state = 513; lexer.advance(false); continue; }
                return result;
            }
            372 => {
                if lookahead == 80 { state = 514; lexer.advance(false); continue; }
                return result;
            }
            373 => {
                if lookahead == 73 { state = 515; lexer.advance(false); continue; }
                return result;
            }
            374 => {
                if lookahead == 110 { state = 516; lexer.advance(false); continue; }
                return result;
            }
            375 => {
                if lookahead == 105 { state = 517; lexer.advance(false); continue; }
                return result;
            }
            376 => {
                if lookahead == 108 { state = 518; lexer.advance(false); continue; }
                return result;
            }
            377 => {
                if lookahead == 114 { state = 519; lexer.advance(false); continue; }
                return result;
            }
            378 => {
                if lookahead == 117 { state = 520; lexer.advance(false); continue; }
                return result;
            }
            379 => {
                if lookahead == 116 { state = 521; lexer.advance(false); continue; }
                return result;
            }
            380 => {
                if lookahead == 95 { state = 522; lexer.advance(false); continue; }
                if lookahead == 97 { state = 523; lexer.advance(false); continue; }
                return result;
            }
            381 => {
                if lookahead == 95 { state = 524; lexer.advance(false); continue; }
                return result;
            }
            382 => {
                if lookahead == 95 { state = 525; lexer.advance(false); continue; }
                return result;
            }
            383 => {
                if lookahead == 103 { state = 526; lexer.advance(false); continue; }
                return result;
            }
            384 => {
                result = true; lexer.set_result_symbol(anon_sym___asm); lexer.mark_end();
                if lookahead == 95 { state = 527; lexer.advance(false); continue; }
                return result;
            }
            385 => {
                if lookahead == 114 { state = 528; lexer.advance(false); continue; }
                return result;
            }
            386 => {
                if lookahead == 111 { state = 529; lexer.advance(false); continue; }
                return result;
            }
            387 => {
                if lookahead == 101 { state = 530; lexer.advance(false); continue; }
                return result;
            }
            388 => {
                if lookahead == 99 { state = 531; lexer.advance(false); continue; }
                return result;
            }
            389 => {
                if lookahead == 100 { state = 532; lexer.advance(false); continue; }
                return result;
            }
            390 => {
                if lookahead == 108 { state = 533; lexer.advance(false); continue; }
                return result;
            }
            391 => {
                if lookahead == 99 { state = 534; lexer.advance(false); continue; }
                return result;
            }
            392 => {
                if lookahead == 99 { state = 535; lexer.advance(false); continue; }
                return result;
            }
            393 => {
                if lookahead == 99 { state = 536; lexer.advance(false); continue; }
                return result;
            }
            394 => {
                if lookahead == 112 { state = 537; lexer.advance(false); continue; }
                return result;
            }
            395 => {
                if lookahead == 115 { state = 538; lexer.advance(false); continue; }
                if lookahead == 116 { state = 539; lexer.advance(false); continue; }
                return result;
            }
            396 => {
                if lookahead == 97 { state = 540; lexer.advance(false); continue; }
                return result;
            }
            397 => {
                if lookahead == 108 { state = 541; lexer.advance(false); continue; }
                return result;
            }
            398 => {
                if lookahead == 114 { state = 542; lexer.advance(false); continue; }
                return result;
            }
            399 => {
                if lookahead == 101 { state = 543; lexer.advance(false); continue; }
                return result;
            }
            400 => {
                if lookahead == 116 { state = 544; lexer.advance(false); continue; }
                return result;
            }
            401 => {
                if lookahead == 97 { state = 545; lexer.advance(false); continue; }
                return result;
            }
            402 => {
                if lookahead == 99 { state = 546; lexer.advance(false); continue; }
                return result;
            }
            403 => {
                if lookahead == 103 { state = 547; lexer.advance(false); continue; }
                return result;
            }
            404 => {
                if lookahead == 105 { state = 548; lexer.advance(false); continue; }
                return result;
            }
            405 => {
                if lookahead == 100 { state = 549; lexer.advance(false); continue; }
                return result;
            }
            406 => {
                if lookahead == 110 { state = 550; lexer.advance(false); continue; }
                return result;
            }
            407 => {
                if lookahead == 108 { state = 551; lexer.advance(false); continue; }
                return result;
            }
            408 => {
                if lookahead == 97 { state = 552; lexer.advance(false); continue; }
                return result;
            }
            409 => {
                if lookahead == 108 { state = 553; lexer.advance(false); continue; }
                return result;
            }
            410 => {
                if lookahead == 116 { state = 554; lexer.advance(false); continue; }
                return result;
            }
            411 => {
                if lookahead == 114 { state = 555; lexer.advance(false); continue; }
                return result;
            }
            412 => {
                if lookahead == 99 { state = 556; lexer.advance(false); continue; }
                return result;
            }
            413 => {
                if lookahead == 111 { state = 557; lexer.advance(false); continue; }
                return result;
            }
            414 => {
                if lookahead == 115 { state = 558; lexer.advance(false); continue; }
                return result;
            }
            415 => {
                if lookahead == 101 { state = 559; lexer.advance(false); continue; }
                return result;
            }
            416 => {
                result = true; lexer.set_result_symbol(anon_sym___try); lexer.mark_end();
                return result;
            }
            417 => {
                if lookahead == 101 { state = 560; lexer.advance(false); continue; }
                return result;
            }
            418 => {
                if lookahead == 108 { state = 561; lexer.advance(false); continue; }
                return result;
            }
            419 => {
                if lookahead == 97 { state = 562; lexer.advance(false); continue; }
                return result;
            }
            420 => {
                if lookahead == 115 { state = 563; lexer.advance(false); continue; }
                return result;
            }
            421 => {
                if lookahead == 114 { state = 564; lexer.advance(false); continue; }
                return result;
            }
            422 => {
                if lookahead == 116 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            423 => {
                if lookahead == 97 { state = 566; lexer.advance(false); continue; }
                return result;
            }
            424 => {
                if lookahead == 107 { state = 567; lexer.advance(false); continue; }
                return result;
            }
            425 => {
                if lookahead == 110 { state = 568; lexer.advance(false); continue; }
                return result;
            }
            426 => {
                if lookahead == 105 { state = 569; lexer.advance(false); continue; }
                return result;
            }
            427 => {
                if lookahead == 97 { state = 570; lexer.advance(false); continue; }
                if lookahead == 111 { state = 571; lexer.advance(false); continue; }
                return result;
            }
            428 => {
                if lookahead == 97 { state = 572; lexer.advance(false); continue; }
                return result;
            }
            429 => {
                result = true; lexer.set_result_symbol(anon_sym_break); lexer.mark_end();
                return result;
            }
            430 => {
                if lookahead == 121 { state = 573; lexer.advance(false); continue; }
                return result;
            }
            431 => {
                result = true; lexer.set_result_symbol(anon_sym_byref); lexer.mark_end();
                return result;
            }
            432 => {
                if lookahead == 54 { state = 574; lexer.advance(false); continue; }
                return result;
            }
            433 => {
                if lookahead == 50 { state = 575; lexer.advance(false); continue; }
                return result;
            }
            434 => {
                if lookahead == 52 { state = 576; lexer.advance(false); continue; }
                return result;
            }
            435 => {
                if lookahead == 95 { state = 577; lexer.advance(false); continue; }
                return result;
            }
            436 => {
                if lookahead == 116 { state = 578; lexer.advance(false); continue; }
                return result;
            }
            437 => {
                result = true; lexer.set_result_symbol(anon_sym_class); lexer.mark_end();
                return result;
            }
            438 => {
                result = true; lexer.set_result_symbol(anon_sym_const); lexer.mark_end();
                if lookahead == 101 { state = 579; lexer.advance(false); continue; }
                return result;
            }
            439 => {
                if lookahead == 110 { state = 580; lexer.advance(false); continue; }
                return result;
            }
            440 => {
                if lookahead == 108 { state = 581; lexer.advance(false); continue; }
                return result;
            }
            441 => {
                if lookahead == 101 { state = 582; lexer.advance(false); continue; }
                return result;
            }
            442 => {
                if lookahead == 101 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            443 => {
                if lookahead == 110 { state = 583; lexer.advance(false); continue; }
                return result;
            }
            444 => {
                if lookahead == 101 { state = 584; lexer.advance(false); continue; }
                return result;
            }
            445 => {
                result = true; lexer.set_result_symbol(anon_sym_inout); lexer.mark_end();
                return result;
            }
            446 => {
                if lookahead == 95 { state = 585; lexer.advance(false); continue; }
                return result;
            }
            447 => {
                if lookahead == 95 { state = 586; lexer.advance(false); continue; }
                return result;
            }
            448 => {
                if lookahead == 95 { state = 587; lexer.advance(false); continue; }
                return result;
            }
            449 => {
                if lookahead == 116 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            450 => {
                if lookahead == 114 { state = 588; lexer.advance(false); continue; }
                return result;
            }
            451 => {
                result = true; lexer.set_result_symbol(anon_sym_macos); lexer.mark_end();
                if lookahead == 120 { state = 589; lexer.advance(false); continue; }
                return result;
            }
            452 => {
                if lookahead == 108 { state = 590; lexer.advance(false); continue; }
                return result;
            }
            453 => {
                if lookahead == 117 { state = 591; lexer.advance(false); continue; }
                return result;
            }
            454 => {
                if lookahead == 111 { state = 592; lexer.advance(false); continue; }
                return result;
            }
            455 => {
                if lookahead == 98 { state = 593; lexer.advance(false); continue; }
                return result;
            }
            456 => {
                if lookahead == 116 { state = 594; lexer.advance(false); continue; }
                return result;
            }
            457 => {
                if lookahead == 98 { state = 595; lexer.advance(false); continue; }
                return result;
            }
            458 => {
                if lookahead == 116 { state = 596; lexer.advance(false); continue; }
                return result;
            }
            459 => {
                if lookahead == 121 { state = 597; lexer.advance(false); continue; }
                return result;
            }
            460 => {
                if lookahead == 102 { state = 598; lexer.advance(false); continue; }
                return result;
            }
            461 => {
                if lookahead == 116 { state = 599; lexer.advance(false); continue; }
                return result;
            }
            462 => {
                if lookahead == 105 { state = 600; lexer.advance(false); continue; }
                return result;
            }
            463 => {
                if lookahead == 110 { state = 601; lexer.advance(false); continue; }
                return result;
            }
            464 => {
                result = true; lexer.set_result_symbol(anon_sym_short); lexer.mark_end();
                return result;
            }
            465 => {
                if lookahead == 100 { state = 602; lexer.advance(false); continue; }
                return result;
            }
            466 => {
                if lookahead == 116 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            467 => {
                if lookahead == 102 { state = 603; lexer.advance(false); continue; }
                return result;
            }
            468 => {
                if lookahead == 95 { state = 604; lexer.advance(false); continue; }
                return result;
            }
            469 => {
                if lookahead == 99 { state = 605; lexer.advance(false); continue; }
                return result;
            }
            470 => {
                if lookahead == 116 { state = 606; lexer.advance(false); continue; }
                return result;
            }
            471 => {
                if lookahead == 104 { state = 607; lexer.advance(false); continue; }
                return result;
            }
            472 => {
                if lookahead == 100 { state = 608; lexer.advance(false); continue; }
                return result;
            }
            473 => {
                if lookahead == 101 { state = 609; lexer.advance(false); continue; }
                return result;
            }
            474 => {
                if lookahead == 102 { state = 610; lexer.advance(false); continue; }
                return result;
            }
            475 => {
                if lookahead == 54 { state = 611; lexer.advance(false); continue; }
                return result;
            }
            476 => {
                if lookahead == 50 { state = 612; lexer.advance(false); continue; }
                return result;
            }
            477 => {
                if lookahead == 52 { state = 613; lexer.advance(false); continue; }
                return result;
            }
            478 => {
                if lookahead == 95 { state = 614; lexer.advance(false); continue; }
                return result;
            }
            479 => {
                if lookahead == 116 { state = 615; lexer.advance(false); continue; }
                return result;
            }
            480 => {
                result = true; lexer.set_result_symbol(anon_sym_union); lexer.mark_end();
                return result;
            }
            481 => {
                if lookahead == 110 { state = 616; lexer.advance(false); continue; }
                return result;
            }
            482 => {
                if lookahead == 103 { state = 617; lexer.advance(false); continue; }
                return result;
            }
            483 => {
                if lookahead == 105 { state = 618; lexer.advance(false); continue; }
                return result;
            }
            484 => {
                if lookahead == 111 { state = 619; lexer.advance(false); continue; }
                return result;
            }
            485 => {
                result = true; lexer.set_result_symbol(anon_sym_while); lexer.mark_end();
                return result;
            }
            486 => {
                if lookahead == 65 { state = 620; lexer.advance(false); continue; }
                return result;
            }
            487 => {
                if lookahead == 80 { state = 621; lexer.advance(false); continue; }
                return result;
            }
            488 => {
                if lookahead == 65 { state = 622; lexer.advance(false); continue; }
                return result;
            }
            489 => {
                if lookahead == 77 { state = 623; lexer.advance(false); continue; }
                return result;
            }
            490 => {
                if lookahead == 85 { state = 624; lexer.advance(false); continue; }
                return result;
            }
            491 => {
                if lookahead == 69 { state = 625; lexer.advance(false); continue; }
                return result;
            }
            492 => {
                if lookahead == 73 { state = 626; lexer.advance(false); continue; }
                return result;
            }
            493 => {
                if lookahead == 65 { state = 627; lexer.advance(false); continue; }
                return result;
            }
            494 => {
                if lookahead == 84 { state = 628; lexer.advance(false); continue; }
                return result;
            }
            495 => {
                if lookahead == 101 { state = 629; lexer.advance(false); continue; }
                return result;
            }
            496 => {
                if lookahead == 101 { state = 630; lexer.advance(false); continue; }
                return result;
            }
            497 => {
                if lookahead == 73 { state = 631; lexer.advance(false); continue; }
                return result;
            }
            498 => {
                if lookahead == 79 { state = 632; lexer.advance(false); continue; }
                return result;
            }
            499 => {
                if lookahead == 73 { state = 633; lexer.advance(false); continue; }
                return result;
            }
            500 => {
                if lookahead == 83 { state = 634; lexer.advance(false); continue; }
                return result;
            }
            501 => {
                if lookahead == 82 { state = 635; lexer.advance(false); continue; }
                return result;
            }
            502 => {
                if lookahead == 77 { state = 636; lexer.advance(false); continue; }
                return result;
            }
            503 => {
                if lookahead == 69 { state = 637; lexer.advance(false); continue; }
                return result;
            }
            504 => {
                if lookahead == 77 { state = 638; lexer.advance(false); continue; }
                return result;
            }
            505 => {
                if lookahead == 73 { state = 639; lexer.advance(false); continue; }
                return result;
            }
            506 => {
                if lookahead == 85 { state = 640; lexer.advance(false); continue; }
                return result;
            }
            507 => {
                if lookahead == 84 { state = 641; lexer.advance(false); continue; }
                return result;
            }
            508 => {
                if lookahead == 70 { state = 642; lexer.advance(false); continue; }
                return result;
            }
            509 => {
                if lookahead == 86 { state = 643; lexer.advance(false); continue; }
                return result;
            }
            510 => {
                if lookahead == 73 { state = 644; lexer.advance(false); continue; }
                return result;
            }
            511 => {
                if lookahead == 88 { state = 645; lexer.advance(false); continue; }
                return result;
            }
            512 => {
                if lookahead == 79 { state = 646; lexer.advance(false); continue; }
                return result;
            }
            513 => {
                if lookahead == 69 { state = 647; lexer.advance(false); continue; }
                return result;
            }
            514 => {
                if lookahead == 69 { state = 648; lexer.advance(false); continue; }
                return result;
            }
            515 => {
                if lookahead == 76 { state = 649; lexer.advance(false); continue; }
                return result;
            }
            516 => {
                if lookahead == 97 { state = 650; lexer.advance(false); continue; }
                if lookahead == 111 { state = 651; lexer.advance(false); continue; }
                return result;
            }
            517 => {
                if lookahead == 99 { state = 652; lexer.advance(false); continue; }
                return result;
            }
            518 => {
                if lookahead == 101 { state = 653; lexer.advance(false); continue; }
                return result;
            }
            519 => {
                if lookahead == 105 { state = 654; lexer.advance(false); continue; }
                return result;
            }
            520 => {
                if lookahead == 108 { state = 655; lexer.advance(false); continue; }
                return result;
            }
            521 => {
                if lookahead == 117 { state = 656; lexer.advance(false); continue; }
                return result;
            }
            522 => {
                if lookahead == 117 { state = 657; lexer.advance(false); continue; }
                return result;
            }
            523 => {
                if lookahead == 98 { state = 658; lexer.advance(false); continue; }
                return result;
            }
            524 => {
                if lookahead == 65 { state = 659; lexer.advance(false); continue; }
                return result;
            }
            525 => {
                if lookahead == 65 { state = 660; lexer.advance(false); continue; }
                return result;
            }
            526 => {
                if lookahead == 110 { state = 661; lexer.advance(false); continue; }
                return result;
            }
            527 => {
                if lookahead == 95 { state = 662; lexer.advance(false); continue; }
                return result;
            }
            528 => {
                if lookahead == 105 { state = 663; lexer.advance(false); continue; }
                return result;
            }
            529 => {
                if lookahead == 114 { state = 664; lexer.advance(false); continue; }
                return result;
            }
            530 => {
                if lookahead == 100 { state = 665; lexer.advance(false); continue; }
                return result;
            }
            531 => {
                if lookahead == 107 { state = 666; lexer.advance(false); continue; }
                return result;
            }
            532 => {
                if lookahead == 103 { state = 667; lexer.advance(false); continue; }
                return result;
            }
            533 => {
                if lookahead == 116 { state = 668; lexer.advance(false); continue; }
                return result;
            }
            534 => {
                if lookahead == 104 { state = 669; lexer.advance(false); continue; }
                return result;
            }
            535 => {
                if lookahead == 108 { state = 670; lexer.advance(false); continue; }
                return result;
            }
            536 => {
                if lookahead == 97 { state = 671; lexer.advance(false); continue; }
                return result;
            }
            537 => {
                if lookahead == 108 { state = 672; lexer.advance(false); continue; }
                return result;
            }
            538 => {
                if lookahead == 116 { state = 673; lexer.advance(false); continue; }
                return result;
            }
            539 => {
                if lookahead == 114 { state = 674; lexer.advance(false); continue; }
                return result;
            }
            540 => {
                if lookahead == 114 { state = 675; lexer.advance(false); continue; }
                return result;
            }
            541 => {
                if lookahead == 115 { state = 676; lexer.advance(false); continue; }
                return result;
            }
            542 => {
                if lookahead == 101 { state = 677; lexer.advance(false); continue; }
                return result;
            }
            543 => {
                if lookahead == 110 { state = 678; lexer.advance(false); continue; }
                return result;
            }
            544 => {
                if lookahead == 99 { state = 679; lexer.advance(false); continue; }
                return result;
            }
            545 => {
                if lookahead == 108 { state = 680; lexer.advance(false); continue; }
                return result;
            }
            546 => {
                if lookahead == 101 { state = 681; lexer.advance(false); continue; }
                return result;
            }
            547 => {
                result = true; lexer.set_result_symbol(anon_sym___imag); lexer.mark_end();
                return result;
            }
            548 => {
                if lookahead == 110 { state = 682; lexer.advance(false); continue; }
                return result;
            }
            549 => {
                if lookahead == 111 { state = 683; lexer.advance(false); continue; }
                return result;
            }
            550 => {
                if lookahead == 117 { state = 684; lexer.advance(false); continue; }
                return result;
            }
            551 => {
                if lookahead == 97 { state = 685; lexer.advance(false); continue; }
                return result;
            }
            552 => {
                if lookahead == 117 { state = 686; lexer.advance(false); continue; }
                return result;
            }
            553 => {
                result = true; lexer.set_result_symbol(anon_sym___real); lexer.mark_end();
                return result;
            }
            554 => {
                if lookahead == 114 { state = 687; lexer.advance(false); continue; }
                return result;
            }
            555 => {
                result = true; lexer.set_result_symbol(sym_ms_signed_ptr_modifier); lexer.mark_end();
                return result;
            }
            556 => {
                if lookahead == 97 { state = 688; lexer.advance(false); continue; }
                return result;
            }
            557 => {
                if lookahead == 110 { state = 689; lexer.advance(false); continue; }
                return result;
            }
            558 => {
                if lookahead == 99 { state = 690; lexer.advance(false); continue; }
                return result;
            }
            559 => {
                if lookahead == 97 { state = 691; lexer.advance(false); continue; }
                return result;
            }
            560 => {
                if lookahead == 111 { state = 692; lexer.advance(false); continue; }
                return result;
            }
            561 => {
                if lookahead == 105 { state = 693; lexer.advance(false); continue; }
                return result;
            }
            562 => {
                if lookahead == 102 { state = 694; lexer.advance(false); continue; }
                return result;
            }
            563 => {
                if lookahead == 101 { state = 695; lexer.advance(false); continue; }
                return result;
            }
            564 => {
                result = true; lexer.set_result_symbol(sym_ms_unsigned_ptr_modifier); lexer.mark_end();
                return result;
            }
            565 => {
                if lookahead == 111 { state = 696; lexer.advance(false); continue; }
                return result;
            }
            566 => {
                if lookahead == 116 { state = 697; lexer.advance(false); continue; }
                return result;
            }
            567 => {
                result = true; lexer.set_result_symbol(anon_sym___weak); lexer.mark_end();
                return result;
            }
            568 => {
                if lookahead == 111 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            569 => {
                if lookahead == 103 { state = 699; lexer.advance(false); continue; }
                return result;
            }
            570 => {
                if lookahead == 115 { state = 700; lexer.advance(false); continue; }
                return result;
            }
            571 => {
                if lookahead == 102 { state = 701; lexer.advance(false); continue; }
                return result;
            }
            572 => {
                if lookahead == 98 { state = 702; lexer.advance(false); continue; }
                return result;
            }
            573 => {
                result = true; lexer.set_result_symbol(anon_sym_bycopy); lexer.mark_end();
                return result;
            }
            574 => {
                if lookahead == 95 { state = 703; lexer.advance(false); continue; }
                return result;
            }
            575 => {
                if lookahead == 95 { state = 704; lexer.advance(false); continue; }
                return result;
            }
            576 => {
                if lookahead == 95 { state = 705; lexer.advance(false); continue; }
                return result;
            }
            577 => {
                if lookahead == 116 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            578 => {
                if lookahead == 114 { state = 706; lexer.advance(false); continue; }
                return result;
            }
            579 => {
                if lookahead == 120 { state = 707; lexer.advance(false); continue; }
                return result;
            }
            580 => {
                if lookahead == 117 { state = 708; lexer.advance(false); continue; }
                return result;
            }
            581 => {
                if lookahead == 116 { state = 709; lexer.advance(false); continue; }
                return result;
            }
            582 => {
                if lookahead == 100 { state = 710; lexer.advance(false); continue; }
                return result;
            }
            583 => {
                result = true; lexer.set_result_symbol(anon_sym_extern); lexer.mark_end();
                return result;
            }
            584 => {
                result = true; lexer.set_result_symbol(anon_sym_inline); lexer.mark_end();
                return result;
            }
            585 => {
                if lookahead == 116 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            586 => {
                if lookahead == 116 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            587 => {
                if lookahead == 116 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            588 => {
                if lookahead == 95 { state = 711; lexer.advance(false); continue; }
                return result;
            }
            589 => {
                result = true; lexer.set_result_symbol(anon_sym_macosx); lexer.mark_end();
                return result;
            }
            590 => {
                if lookahead == 105 { state = 712; lexer.advance(false); continue; }
                return result;
            }
            591 => {
                if lookahead == 114 { state = 713; lexer.advance(false); continue; }
                return result;
            }
            592 => {
                if lookahead == 119 { state = 714; lexer.advance(false); continue; }
                return result;
            }
            593 => {
                if lookahead == 108 { state = 715; lexer.advance(false); continue; }
                return result;
            }
            594 => {
                if lookahead == 114 { state = 716; lexer.advance(false); continue; }
                return result;
            }
            595 => {
                if lookahead == 114 { state = 717; lexer.advance(false); continue; }
                return result;
            }
            596 => {
                if lookahead == 111 { state = 718; lexer.advance(false); continue; }
                return result;
            }
            597 => {
                result = true; lexer.set_result_symbol(anon_sym_oneway); lexer.mark_end();
                return result;
            }
            598 => {
                if lookahead == 102 { state = 719; lexer.advance(false); continue; }
                return result;
            }
            599 => {
                if lookahead == 101 { state = 720; lexer.advance(false); continue; }
                return result;
            }
            600 => {
                if lookahead == 99 { state = 721; lexer.advance(false); continue; }
                return result;
            }
            601 => {
                result = true; lexer.set_result_symbol(anon_sym_return); lexer.mark_end();
                return result;
            }
            602 => {
                result = true; lexer.set_result_symbol(anon_sym_signed); lexer.mark_end();
                return result;
            }
            603 => {
                result = true; lexer.set_result_symbol(anon_sym_sizeof); lexer.mark_end();
                return result;
            }
            604 => {
                if lookahead == 116 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            605 => {
                result = true; lexer.set_result_symbol(anon_sym_static); lexer.mark_end();
                return result;
            }
            606 => {
                result = true; lexer.set_result_symbol(anon_sym_struct); lexer.mark_end();
                return result;
            }
            607 => {
                result = true; lexer.set_result_symbol(anon_sym_switch); lexer.mark_end();
                return result;
            }
            608 => {
                if lookahead == 95 { state = 722; lexer.advance(false); continue; }
                return result;
            }
            609 => {
                if lookahead == 102 { state = 723; lexer.advance(false); continue; }
                return result;
            }
            610 => {
                result = true; lexer.set_result_symbol(anon_sym_typeof); lexer.mark_end();
                return result;
            }
            611 => {
                if lookahead == 95 { state = 724; lexer.advance(false); continue; }
                return result;
            }
            612 => {
                if lookahead == 95 { state = 725; lexer.advance(false); continue; }
                return result;
            }
            613 => {
                if lookahead == 95 { state = 726; lexer.advance(false); continue; }
                return result;
            }
            614 => {
                if lookahead == 116 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            615 => {
                if lookahead == 114 { state = 727; lexer.advance(false); continue; }
                return result;
            }
            616 => {
                if lookahead == 101 { state = 728; lexer.advance(false); continue; }
                return result;
            }
            617 => {
                result = true; lexer.set_result_symbol(anon_sym_va_arg); lexer.mark_end();
                return result;
            }
            618 => {
                if lookahead == 108 { state = 729; lexer.advance(false); continue; }
                return result;
            }
            619 => {
                if lookahead == 115 { state = 730; lexer.advance(false); continue; }
                return result;
            }
            620 => {
                if lookahead == 73 { state = 731; lexer.advance(false); continue; }
                return result;
            }
            621 => {
                if lookahead == 82 { state = 732; lexer.advance(false); continue; }
                return result;
            }
            622 => {
                if lookahead == 86 { state = 733; lexer.advance(false); continue; }
                return result;
            }
            623 => {
                if lookahead == 65 { state = 734; lexer.advance(false); continue; }
                return result;
            }
            624 => {
                if lookahead == 82 { state = 735; lexer.advance(false); continue; }
                return result;
            }
            625 => {
                if lookahead == 82 { state = 736; lexer.advance(false); continue; }
                return result;
            }
            626 => {
                if lookahead == 78 { state = 737; lexer.advance(false); continue; }
                return result;
            }
            627 => {
                if lookahead == 84 { state = 738; lexer.advance(false); continue; }
                return result;
            }
            628 => {
                if lookahead == 73 { state = 739; lexer.advance(false); continue; }
                return result;
            }
            629 => {
                if lookahead == 99 { state = 740; lexer.advance(false); continue; }
                return result;
            }
            630 => {
                if lookahead == 116 { state = 741; lexer.advance(false); continue; }
                return result;
            }
            631 => {
                if lookahead == 71 { state = 742; lexer.advance(false); continue; }
                return result;
            }
            632 => {
                if lookahead == 77 { state = 743; lexer.advance(false); continue; }
                return result;
            }
            633 => {
                if lookahead == 76 { state = 744; lexer.advance(false); continue; }
                return result;
            }
            634 => {
                if lookahead == 83 { state = 745; lexer.advance(false); continue; }
                return result;
            }
            635 => {
                if lookahead == 69 { state = 746; lexer.advance(false); continue; }
                return result;
            }
            636 => {
                if lookahead == 95 { state = 747; lexer.advance(false); continue; }
                return result;
            }
            637 => {
                if lookahead == 78 { state = 748; lexer.advance(false); continue; }
                return result;
            }
            638 => {
                if lookahead == 65 { state = 749; lexer.advance(false); continue; }
                return result;
            }
            639 => {
                if lookahead == 78 { state = 750; lexer.advance(false); continue; }
                return result;
            }
            640 => {
                if lookahead == 73 { state = 751; lexer.advance(false); continue; }
                return result;
            }
            641 => {
                if lookahead == 95 { state = 752; lexer.advance(false); continue; }
                return result;
            }
            642 => {
                if lookahead == 84 { state = 753; lexer.advance(false); continue; }
                return result;
            }
            643 => {
                if lookahead == 65 { state = 754; lexer.advance(false); continue; }
                return result;
            }
            644 => {
                if lookahead == 68 { state = 755; lexer.advance(false); continue; }
                return result;
            }
            645 => {
                if lookahead == 80 { state = 756; lexer.advance(false); continue; }
                return result;
            }
            646 => {
                if lookahead == 79 { state = 757; lexer.advance(false); continue; }
                return result;
            }
            647 => {
                if lookahead == 88 { state = 758; lexer.advance(false); continue; }
                return result;
            }
            648 => {
                if lookahead == 65 { state = 759; lexer.advance(false); continue; }
                return result;
            }
            649 => {
                if lookahead == 65 { state = 760; lexer.advance(false); continue; }
                return result;
            }
            650 => {
                if lookahead == 115 { state = 761; lexer.advance(false); continue; }
                return result;
            }
            651 => {
                if lookahead == 102 { state = 762; lexer.advance(false); continue; }
                return result;
            }
            652 => {
                result = true; lexer.set_result_symbol(anon_sym__Atomic); lexer.mark_end();
                return result;
            }
            653 => {
                if lookahead == 120 { state = 763; lexer.advance(false); continue; }
                return result;
            }
            654 => {
                if lookahead == 99 { state = 764; lexer.advance(false); continue; }
                return result;
            }
            655 => {
                if lookahead == 108 { state = 765; lexer.advance(false); continue; }
                return result;
            }
            656 => {
                if lookahead == 114 { state = 766; lexer.advance(false); continue; }
                return result;
            }
            657 => {
                if lookahead == 110 { state = 767; lexer.advance(false); continue; }
                return result;
            }
            658 => {
                if lookahead == 108 { state = 768; lexer.advance(false); continue; }
                return result;
            }
            659 => {
                if lookahead == 86 { state = 769; lexer.advance(false); continue; }
                return result;
            }
            660 => {
                if lookahead == 86 { state = 770; lexer.advance(false); continue; }
                return result;
            }
            661 => {
                if lookahead == 111 { state = 771; lexer.advance(false); continue; }
                return result;
            }
            662 => {
                result = true; lexer.set_result_symbol(anon_sym___asm__); lexer.mark_end();
                return result;
            }
            663 => {
                if lookahead == 98 { state = 772; lexer.advance(false); continue; }
                return result;
            }
            664 => {
                if lookahead == 101 { state = 773; lexer.advance(false); continue; }
                return result;
            }
            665 => {
                result = true; lexer.set_result_symbol(anon_sym___based); lexer.mark_end();
                return result;
            }
            666 => {
                result = true; lexer.set_result_symbol(anon_sym___block); lexer.mark_end();
                return result;
            }
            667 => {
                if lookahead == 101 { state = 774; lexer.advance(false); continue; }
                return result;
            }
            668 => {
                if lookahead == 105 { state = 775; lexer.advance(false); continue; }
                return result;
            }
            669 => {
                result = true; lexer.set_result_symbol(anon_sym___catch); lexer.mark_end();
                return result;
            }
            670 => {
                result = true; lexer.set_result_symbol(anon_sym___cdecl); lexer.mark_end();
                return result;
            }
            671 => {
                if lookahead == 108 { state = 776; lexer.advance(false); continue; }
                return result;
            }
            672 => {
                if lookahead == 101 { state = 777; lexer.advance(false); continue; }
                return result;
            }
            673 => {
                result = true; lexer.set_result_symbol(anon_sym___const); lexer.mark_end();
                return result;
            }
            674 => {
                if lookahead == 97 { state = 778; lexer.advance(false); continue; }
                return result;
            }
            675 => {
                if lookahead == 105 { state = 779; lexer.advance(false); continue; }
                return result;
            }
            676 => {
                if lookahead == 112 { state = 780; lexer.advance(false); continue; }
                return result;
            }
            677 => {
                if lookahead == 99 { state = 781; lexer.advance(false); continue; }
                return result;
            }
            678 => {
                if lookahead == 115 { state = 782; lexer.advance(false); continue; }
                return result;
            }
            679 => {
                if lookahead == 97 { state = 783; lexer.advance(false); continue; }
                return result;
            }
            680 => {
                if lookahead == 108 { state = 784; lexer.advance(false); continue; }
                return result;
            }
            681 => {
                if lookahead == 105 { state = 785; lexer.advance(false); continue; }
                return result;
            }
            682 => {
                if lookahead == 101 { state = 786; lexer.advance(false); continue; }
                return result;
            }
            683 => {
                if lookahead == 102 { state = 787; lexer.advance(false); continue; }
                return result;
            }
            684 => {
                if lookahead == 108 { state = 788; lexer.advance(false); continue; }
                return result;
            }
            685 => {
                if lookahead == 98 { state = 789; lexer.advance(false); continue; }
                return result;
            }
            686 => {
                if lookahead == 116 { state = 790; lexer.advance(false); continue; }
                return result;
            }
            687 => {
                if lookahead == 105 { state = 791; lexer.advance(false); continue; }
                return result;
            }
            688 => {
                if lookahead == 108 { state = 792; lexer.advance(false); continue; }
                return result;
            }
            689 => {
                if lookahead == 103 { state = 793; lexer.advance(false); continue; }
                return result;
            }
            690 => {
                if lookahead == 97 { state = 794; lexer.advance(false); continue; }
                return result;
            }
            691 => {
                if lookahead == 100 { state = 795; lexer.advance(false); continue; }
                return result;
            }
            692 => {
                if lookahead == 102 { state = 796; lexer.advance(false); continue; }
                return result;
            }
            693 => {
                if lookahead == 103 { state = 797; lexer.advance(false); continue; }
                return result;
            }
            694 => {
                if lookahead == 101 { state = 798; lexer.advance(false); continue; }
                return result;
            }
            695 => {
                if lookahead == 100 { state = 799; lexer.advance(false); continue; }
                return result;
            }
            696 => {
                if lookahead == 114 { state = 800; lexer.advance(false); continue; }
                return result;
            }
            697 => {
                if lookahead == 105 { state = 801; lexer.advance(false); continue; }
                return result;
            }
            698 => {
                if lookahead == 102 { state = 802; lexer.advance(false); continue; }
                return result;
            }
            699 => {
                if lookahead == 110 { state = 803; lexer.advance(false); continue; }
                return result;
            }
            700 => {
                result = true; lexer.set_result_symbol(anon_sym_alignas); lexer.mark_end();
                return result;
            }
            701 => {
                result = true; lexer.set_result_symbol(anon_sym_alignof); lexer.mark_end();
                return result;
            }
            702 => {
                if lookahead == 105 { state = 804; lexer.advance(false); continue; }
                return result;
            }
            703 => {
                if lookahead == 116 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            704 => {
                if lookahead == 116 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            705 => {
                if lookahead == 116 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            706 => {
                if lookahead == 95 { state = 805; lexer.advance(false); continue; }
                return result;
            }
            707 => {
                if lookahead == 112 { state = 806; lexer.advance(false); continue; }
                return result;
            }
            708 => {
                if lookahead == 101 { state = 807; lexer.advance(false); continue; }
                return result;
            }
            709 => {
                result = true; lexer.set_result_symbol(anon_sym_default); lexer.mark_end();
                return result;
            }
            710 => {
                result = true; lexer.set_result_symbol(anon_sym_defined); lexer.mark_end();
                return result;
            }
            711 => {
                if lookahead == 116 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            712 => {
                if lookahead == 103 { state = 808; lexer.advance(false); continue; }
                return result;
            }
            713 => {
                if lookahead == 110 { state = 809; lexer.advance(false); continue; }
                return result;
            }
            714 => {
                result = true; lexer.set_result_symbol(anon_sym_nothrow); lexer.mark_end();
                return result;
            }
            715 => {
                if lookahead == 101 { state = 810; lexer.advance(false); continue; }
                return result;
            }
            716 => {
                result = true; lexer.set_result_symbol(anon_sym_nullptr); lexer.mark_end();
                if lookahead == 95 { state = 811; lexer.advance(false); continue; }
                return result;
            }
            717 => {
                if lookahead == 105 { state = 812; lexer.advance(false); continue; }
                return result;
            }
            718 => {
                if lookahead == 102 { state = 813; lexer.advance(false); continue; }
                return result;
            }
            719 => {
                if lookahead == 95 { state = 814; lexer.advance(false); continue; }
                return result;
            }
            720 => {
                if lookahead == 114 { state = 815; lexer.advance(false); continue; }
                return result;
            }
            721 => {
                if lookahead == 116 { state = 816; lexer.advance(false); continue; }
                return result;
            }
            722 => {
                if lookahead == 108 { state = 817; lexer.advance(false); continue; }
                return result;
            }
            723 => {
                result = true; lexer.set_result_symbol(anon_sym_typedef); lexer.mark_end();
                return result;
            }
            724 => {
                if lookahead == 116 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            725 => {
                if lookahead == 116 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            726 => {
                if lookahead == 116 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            727 => {
                if lookahead == 95 { state = 818; lexer.advance(false); continue; }
                return result;
            }
            728 => {
                if lookahead == 100 { state = 819; lexer.advance(false); continue; }
                return result;
            }
            729 => {
                if lookahead == 101 { state = 820; lexer.advance(false); continue; }
                return result;
            }
            730 => {
                result = true; lexer.set_result_symbol(anon_sym_watchos); lexer.mark_end();
                return result;
            }
            731 => {
                if lookahead == 76 { state = 821; lexer.advance(false); continue; }
                return result;
            }
            732 => {
                if lookahead == 69 { state = 822; lexer.advance(false); continue; }
                return result;
            }
            733 => {
                if lookahead == 65 { state = 823; lexer.advance(false); continue; }
                return result;
            }
            734 => {
                if lookahead == 84 { state = 824; lexer.advance(false); continue; }
                return result;
            }
            735 => {
                if lookahead == 78 { state = 825; lexer.advance(false); continue; }
                return result;
            }
            736 => {
                if lookahead == 78 { state = 826; lexer.advance(false); continue; }
                return result;
            }
            737 => {
                if lookahead == 69 { state = 827; lexer.advance(false); continue; }
                return result;
            }
            738 => {
                if lookahead == 69 { state = 828; lexer.advance(false); continue; }
                return result;
            }
            739 => {
                if lookahead == 79 { state = 829; lexer.advance(false); continue; }
                return result;
            }
            740 => {
                if lookahead == 116 { state = 830; lexer.advance(false); continue; }
                return result;
            }
            741 => {
                result = true; lexer.set_result_symbol(anon_sym_IBOutlet); lexer.mark_end();
                return result;
            }
            742 => {
                if lookahead == 78 { state = 831; lexer.advance(false); continue; }
                return result;
            }
            743 => {
                if lookahead == 65 { state = 832; lexer.advance(false); continue; }
                return result;
            }
            744 => {
                if lookahead == 65 { state = 833; lexer.advance(false); continue; }
                return result;
            }
            745 => {
                if lookahead == 95 { state = 834; lexer.advance(false); continue; }
                return result;
            }
            746 => {
                if lookahead == 67 { state = 835; lexer.advance(false); continue; }
                return result;
            }
            747 => {
                if lookahead == 65 { state = 836; lexer.advance(false); continue; }
                if lookahead == 68 { state = 837; lexer.advance(false); continue; }
                return result;
            }
            748 => {
                if lookahead == 83 { state = 838; lexer.advance(false); continue; }
                return result;
            }
            749 => {
                if lookahead == 84 { state = 839; lexer.advance(false); continue; }
                return result;
            }
            750 => {
                if lookahead == 69 { state = 840; lexer.advance(false); continue; }
                return result;
            }
            751 => {
                if lookahead == 82 { state = 841; lexer.advance(false); continue; }
                return result;
            }
            752 => {
                if lookahead == 67 { state = 842; lexer.advance(false); continue; }
                return result;
            }
            753 => {
                if lookahead == 95 { state = 843; lexer.advance(false); continue; }
                return result;
            }
            754 => {
                if lookahead == 73 { state = 844; lexer.advance(false); continue; }
                return result;
            }
            755 => {
                if lookahead == 95 { state = 845; lexer.advance(false); continue; }
                return result;
            }
            756 => {
                if lookahead == 79 { state = 846; lexer.advance(false); continue; }
                return result;
            }
            757 => {
                if lookahead == 84 { state = 847; lexer.advance(false); continue; }
                return result;
            }
            758 => {
                if lookahead == 84 { state = 848; lexer.advance(false); continue; }
                return result;
            }
            759 => {
                if lookahead == 82 { state = 849; lexer.advance(false); continue; }
                return result;
            }
            760 => {
                if lookahead == 66 { state = 850; lexer.advance(false); continue; }
                return result;
            }
            761 => {
                result = true; lexer.set_result_symbol(anon_sym__Alignas); lexer.mark_end();
                return result;
            }
            762 => {
                result = true; lexer.set_result_symbol(anon_sym__Alignof); lexer.mark_end();
                return result;
            }
            763 => {
                result = true; lexer.set_result_symbol(anon_sym__Complex); lexer.mark_end();
                return result;
            }
            764 => {
                result = true; lexer.set_result_symbol(anon_sym__Generic); lexer.mark_end();
                return result;
            }
            765 => {
                result = true; lexer.set_result_symbol(anon_sym__Nonnull); lexer.mark_end();
                return result;
            }
            766 => {
                if lookahead == 110 { state = 851; lexer.advance(false); continue; }
                return result;
            }
            767 => {
                if lookahead == 115 { state = 852; lexer.advance(false); continue; }
                return result;
            }
            768 => {
                if lookahead == 101 { state = 853; lexer.advance(false); continue; }
                return result;
            }
            769 => {
                if lookahead == 65 { state = 854; lexer.advance(false); continue; }
                return result;
            }
            770 => {
                if lookahead == 65 { state = 855; lexer.advance(false); continue; }
                return result;
            }
            771 => {
                if lookahead == 102 { state = 856; lexer.advance(false); continue; }
                return result;
            }
            772 => {
                if lookahead == 117 { state = 857; lexer.advance(false); continue; }
                return result;
            }
            773 => {
                if lookahead == 108 { state = 858; lexer.advance(false); continue; }
                return result;
            }
            774 => {
                result = true; lexer.set_result_symbol(anon_sym___bridge); lexer.mark_end();
                if lookahead == 95 { state = 859; lexer.advance(false); continue; }
                return result;
            }
            775 => {
                if lookahead == 110 { state = 860; lexer.advance(false); continue; }
                return result;
            }
            776 => {
                if lookahead == 108 { state = 861; lexer.advance(false); continue; }
                return result;
            }
            777 => {
                if lookahead == 120 { state = 862; lexer.advance(false); continue; }
                return result;
            }
            778 => {
                if lookahead == 118 { state = 863; lexer.advance(false); continue; }
                return result;
            }
            779 => {
                if lookahead == 97 { state = 864; lexer.advance(false); continue; }
                return result;
            }
            780 => {
                if lookahead == 101 { state = 865; lexer.advance(false); continue; }
                return result;
            }
            781 => {
                if lookahead == 97 { state = 866; lexer.advance(false); continue; }
                return result;
            }
            782 => {
                if lookahead == 105 { state = 867; lexer.advance(false); continue; }
                return result;
            }
            783 => {
                if lookahead == 108 { state = 868; lexer.advance(false); continue; }
                return result;
            }
            784 => {
                if lookahead == 121 { state = 869; lexer.advance(false); continue; }
                return result;
            }
            785 => {
                if lookahead == 110 { state = 870; lexer.advance(false); continue; }
                return result;
            }
            786 => {
                result = true; lexer.set_result_symbol(anon_sym___inline); lexer.mark_end();
                if lookahead == 95 { state = 871; lexer.advance(false); continue; }
                return result;
            }
            787 => {
                result = true; lexer.set_result_symbol(anon_sym___kindof); lexer.mark_end();
                return result;
            }
            788 => {
                if lookahead == 108 { state = 872; lexer.advance(false); continue; }
                return result;
            }
            789 => {
                if lookahead == 108 { state = 873; lexer.advance(false); continue; }
                return result;
            }
            790 => {
                if lookahead == 104 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            791 => {
                if lookahead == 99 { state = 875; lexer.advance(false); continue; }
                return result;
            }
            792 => {
                if lookahead == 108 { state = 876; lexer.advance(false); continue; }
                return result;
            }
            793 => {
                result = true; lexer.set_result_symbol(anon_sym___strong); lexer.mark_end();
                return result;
            }
            794 => {
                if lookahead == 108 { state = 877; lexer.advance(false); continue; }
                return result;
            }
            795 => {
                result = true; lexer.set_result_symbol(anon_sym___thread); lexer.mark_end();
                return result;
            }
            796 => {
                result = true; lexer.set_result_symbol(anon_sym___typeof); lexer.mark_end();
                if lookahead == 95 { state = 878; lexer.advance(false); continue; }
                return result;
            }
            797 => {
                if lookahead == 110 { state = 879; lexer.advance(false); continue; }
                return result;
            }
            798 => {
                if lookahead == 95 { state = 880; lexer.advance(false); continue; }
                return result;
            }
            799 => {
                result = true; lexer.set_result_symbol(anon_sym___unused); lexer.mark_end();
                return result;
            }
            800 => {
                if lookahead == 99 { state = 881; lexer.advance(false); continue; }
                return result;
            }
            801 => {
                if lookahead == 108 { state = 882; lexer.advance(false); continue; }
                return result;
            }
            802 => {
                result = true; lexer.set_result_symbol(anon_sym__alignof); lexer.mark_end();
                return result;
            }
            803 => {
                if lookahead == 101 { state = 883; lexer.advance(false); continue; }
                return result;
            }
            804 => {
                if lookahead == 108 { state = 884; lexer.advance(false); continue; }
                return result;
            }
            805 => {
                if lookahead == 116 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            806 => {
                if lookahead == 114 { state = 885; lexer.advance(false); continue; }
                return result;
            }
            807 => {
                result = true; lexer.set_result_symbol(anon_sym_continue); lexer.mark_end();
                return result;
            }
            808 => {
                if lookahead == 110 { state = 886; lexer.advance(false); continue; }
                return result;
            }
            809 => {
                result = true; lexer.set_result_symbol(anon_sym_noreturn); lexer.mark_end();
                return result;
            }
            810 => {
                result = true; lexer.set_result_symbol(anon_sym_nullable); lexer.mark_end();
                return result;
            }
            811 => {
                if lookahead == 116 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            812 => {
                if lookahead == 100 { state = 887; lexer.advance(false); continue; }
                return result;
            }
            813 => {
                result = true; lexer.set_result_symbol(anon_sym_offsetof); lexer.mark_end();
                return result;
            }
            814 => {
                if lookahead == 116 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            815 => {
                result = true; lexer.set_result_symbol(anon_sym_register); lexer.mark_end();
                return result;
            }
            816 => {
                result = true; lexer.set_result_symbol(anon_sym_restrict); lexer.mark_end();
                return result;
            }
            817 => {
                if lookahead == 111 { state = 888; lexer.advance(false); continue; }
                return result;
            }
            818 => {
                if lookahead == 116 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            819 => {
                result = true; lexer.set_result_symbol(anon_sym_unsigned); lexer.mark_end();
                return result;
            }
            820 => {
                result = true; lexer.set_result_symbol(anon_sym_volatile); lexer.mark_end();
                return result;
            }
            821 => {
                if lookahead == 65 { state = 889; lexer.advance(false); continue; }
                return result;
            }
            822 => {
                if lookahead == 67 { state = 890; lexer.advance(false); continue; }
                return result;
            }
            823 => {
                if lookahead == 73 { state = 891; lexer.advance(false); continue; }
                return result;
            }
            824 => {
                if lookahead == 95 { state = 892; lexer.advance(false); continue; }
                return result;
            }
            825 => {
                if lookahead == 83 { state = 893; lexer.advance(false); continue; }
                return result;
            }
            826 => {
                result = true; lexer.set_result_symbol(anon_sym_CG_EXTERN); lexer.mark_end();
                return result;
            }
            827 => {
                result = true; lexer.set_result_symbol(anon_sym_CG_INLINE); lexer.mark_end();
                return result;
            }
            828 => {
                if lookahead == 68 { state = 894; lexer.advance(false); continue; }
                return result;
            }
            829 => {
                if lookahead == 78 { state = 895; lexer.advance(false); continue; }
                return result;
            }
            830 => {
                if lookahead == 97 { state = 896; lexer.advance(false); continue; }
                return result;
            }
            831 => {
                if lookahead == 65 { state = 897; lexer.advance(false); continue; }
                return result;
            }
            832 => {
                if lookahead == 84 { state = 898; lexer.advance(false); continue; }
                return result;
            }
            833 => {
                if lookahead == 66 { state = 899; lexer.advance(false); continue; }
                return result;
            }
            834 => {
                if lookahead == 65 { state = 900; lexer.advance(false); continue; }
                if lookahead == 68 { state = 901; lexer.advance(false); continue; }
                return result;
            }
            835 => {
                if lookahead == 65 { state = 902; lexer.advance(false); continue; }
                return result;
            }
            836 => {
                if lookahead == 86 { state = 903; lexer.advance(false); continue; }
                return result;
            }
            837 => {
                if lookahead == 69 { state = 904; lexer.advance(false); continue; }
                return result;
            }
            838 => {
                if lookahead == 73 { state = 905; lexer.advance(false); continue; }
                return result;
            }
            839 => {
                if lookahead == 95 { state = 906; lexer.advance(false); continue; }
                return result;
            }
            840 => {
                result = true; lexer.set_result_symbol(anon_sym_NS_INLINE); lexer.mark_end();
                return result;
            }
            841 => {
                if lookahead == 69 { state = 907; lexer.advance(false); continue; }
                return result;
            }
            842 => {
                if lookahead == 76 { state = 908; lexer.advance(false); continue; }
                return result;
            }
            843 => {
                if lookahead == 78 { state = 909; lexer.advance(false); continue; }
                if lookahead == 85 { state = 910; lexer.advance(false); continue; }
                return result;
            }
            844 => {
                if lookahead == 76 { state = 911; lexer.advance(false); continue; }
                return result;
            }
            845 => {
                if lookahead == 85 { state = 912; lexer.advance(false); continue; }
                return result;
            }
            846 => {
                if lookahead == 82 { state = 913; lexer.advance(false); continue; }
                return result;
            }
            847 => {
                if lookahead == 95 { state = 914; lexer.advance(false); continue; }
                return result;
            }
            848 => {
                if lookahead == 69 { state = 915; lexer.advance(false); continue; }
                return result;
            }
            849 => {
                if lookahead == 65 { state = 916; lexer.advance(false); continue; }
                return result;
            }
            850 => {
                if lookahead == 76 { state = 917; lexer.advance(false); continue; }
                return result;
            }
            851 => {
                result = true; lexer.set_result_symbol(anon_sym__Noreturn); lexer.mark_end();
                return result;
            }
            852 => {
                if lookahead == 112 { state = 918; lexer.advance(false); continue; }
                return result;
            }
            853 => {
                result = true; lexer.set_result_symbol(anon_sym__Nullable); lexer.mark_end();
                if lookahead == 95 { state = 919; lexer.advance(false); continue; }
                return result;
            }
            854 => {
                if lookahead == 73 { state = 920; lexer.advance(false); continue; }
                return result;
            }
            855 => {
                if lookahead == 73 { state = 921; lexer.advance(false); continue; }
                return result;
            }
            856 => {
                result = true; lexer.set_result_symbol(anon_sym___alignof); lexer.mark_end();
                if lookahead == 95 { state = 922; lexer.advance(false); continue; }
                return result;
            }
            857 => {
                if lookahead == 116 { state = 923; lexer.advance(false); continue; }
                return result;
            }
            858 => {
                if lookahead == 101 { state = 924; lexer.advance(false); continue; }
                return result;
            }
            859 => {
                if lookahead == 114 { state = 925; lexer.advance(false); continue; }
                if lookahead == 116 { state = 926; lexer.advance(false); continue; }
                return result;
            }
            860 => {
                if lookahead == 95 { state = 927; lexer.advance(false); continue; }
                return result;
            }
            861 => {
                result = true; lexer.set_result_symbol(anon_sym___clrcall); lexer.mark_end();
                return result;
            }
            862 => {
                result = true; lexer.set_result_symbol(anon_sym___complex); lexer.mark_end();
                return result;
            }
            863 => {
                if lookahead == 97 { state = 928; lexer.advance(false); continue; }
                return result;
            }
            864 => {
                if lookahead == 110 { state = 929; lexer.advance(false); continue; }
                return result;
            }
            865 => {
                if lookahead == 99 { state = 930; lexer.advance(false); continue; }
                return result;
            }
            866 => {
                if lookahead == 116 { state = 931; lexer.advance(false); continue; }
                return result;
            }
            867 => {
                if lookahead == 111 { state = 932; lexer.advance(false); continue; }
                return result;
            }
            868 => {
                if lookahead == 108 { state = 933; lexer.advance(false); continue; }
                return result;
            }
            869 => {
                result = true; lexer.set_result_symbol(anon_sym___finally); lexer.mark_end();
                return result;
            }
            870 => {
                if lookahead == 108 { state = 934; lexer.advance(false); continue; }
                return result;
            }
            871 => {
                if lookahead == 95 { state = 935; lexer.advance(false); continue; }
                return result;
            }
            872 => {
                result = true; lexer.set_result_symbol(anon_sym___nonnull); lexer.mark_end();
                return result;
            }
            873 => {
                if lookahead == 101 { state = 936; lexer.advance(false); continue; }
                return result;
            }
            874 => {
                if lookahead == 95 { state = 937; lexer.advance(false); continue; }
                return result;
            }
            875 => {
                if lookahead == 116 { state = 938; lexer.advance(false); continue; }
                return result;
            }
            876 => {
                result = true; lexer.set_result_symbol(anon_sym___stdcall); lexer.mark_end();
                return result;
            }
            877 => {
                if lookahead == 108 { state = 939; lexer.advance(false); continue; }
                return result;
            }
            878 => {
                if lookahead == 95 { state = 940; lexer.advance(false); continue; }
                return result;
            }
            879 => {
                if lookahead == 101 { state = 941; lexer.advance(false); continue; }
                return result;
            }
            880 => {
                if lookahead == 117 { state = 942; lexer.advance(false); continue; }
                return result;
            }
            881 => {
                if lookahead == 97 { state = 943; lexer.advance(false); continue; }
                return result;
            }
            882 => {
                if lookahead == 101 { state = 944; lexer.advance(false); continue; }
                return result;
            }
            883 => {
                if lookahead == 100 { state = 945; lexer.advance(false); continue; }
                return result;
            }
            884 => {
                if lookahead == 105 { state = 946; lexer.advance(false); continue; }
                return result;
            }
            885 => {
                result = true; lexer.set_result_symbol(anon_sym_constexpr); lexer.mark_end();
                return result;
            }
            886 => {
                if lookahead == 95 { state = 947; lexer.advance(false); continue; }
                return result;
            }
            887 => {
                if lookahead == 103 { state = 948; lexer.advance(false); continue; }
                return result;
            }
            888 => {
                if lookahead == 99 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            889 => {
                if lookahead == 66 { state = 950; lexer.advance(false); continue; }
                return result;
            }
            890 => {
                if lookahead == 65 { state = 951; lexer.advance(false); continue; }
                return result;
            }
            891 => {
                if lookahead == 76 { state = 952; lexer.advance(false); continue; }
                return result;
            }
            892 => {
                if lookahead == 70 { state = 953; lexer.advance(false); continue; }
                return result;
            }
            893 => {
                if lookahead == 95 { state = 954; lexer.advance(false); continue; }
                return result;
            }
            894 => {
                if lookahead == 95 { state = 955; lexer.advance(false); continue; }
                return result;
            }
            895 => {
                if lookahead == 95 { state = 956; lexer.advance(false); continue; }
                return result;
            }
            896 => {
                if lookahead == 98 { state = 957; lexer.advance(false); continue; }
                return result;
            }
            897 => {
                if lookahead == 66 { state = 958; lexer.advance(false); continue; }
                return result;
            }
            898 => {
                if lookahead == 69 { state = 959; lexer.advance(false); continue; }
                return result;
            }
            899 => {
                if lookahead == 76 { state = 960; lexer.advance(false); continue; }
                return result;
            }
            900 => {
                if lookahead == 86 { state = 961; lexer.advance(false); continue; }
                return result;
            }
            901 => {
                if lookahead == 69 { state = 962; lexer.advance(false); continue; }
                return result;
            }
            902 => {
                if lookahead == 84 { state = 963; lexer.advance(false); continue; }
                return result;
            }
            903 => {
                if lookahead == 65 { state = 964; lexer.advance(false); continue; }
                return result;
            }
            904 => {
                if lookahead == 80 { state = 965; lexer.advance(false); continue; }
                return result;
            }
            905 => {
                if lookahead == 79 { state = 966; lexer.advance(false); continue; }
                return result;
            }
            906 => {
                if lookahead == 70 { state = 967; lexer.advance(false); continue; }
                return result;
            }
            907 => {
                if lookahead == 83 { state = 968; lexer.advance(false); continue; }
                return result;
            }
            908 => {
                if lookahead == 65 { state = 969; lexer.advance(false); continue; }
                return result;
            }
            909 => {
                if lookahead == 65 { state = 970; lexer.advance(false); continue; }
                return result;
            }
            910 => {
                if lookahead == 78 { state = 971; lexer.advance(false); continue; }
                return result;
            }
            911 => {
                if lookahead == 65 { state = 972; lexer.advance(false); continue; }
                return result;
            }
            912 => {
                if lookahead == 78 { state = 973; lexer.advance(false); continue; }
                return result;
            }
            913 => {
                if lookahead == 84 { state = 974; lexer.advance(false); continue; }
                return result;
            }
            914 => {
                if lookahead == 67 { state = 975; lexer.advance(false); continue; }
                return result;
            }
            915 => {
                if lookahead == 82 { state = 976; lexer.advance(false); continue; }
                return result;
            }
            916 => {
                if lookahead == 78 { state = 977; lexer.advance(false); continue; }
                return result;
            }
            917 => {
                if lookahead == 69 { state = 978; lexer.advance(false); continue; }
                return result;
            }
            918 => {
                if lookahead == 101 { state = 979; lexer.advance(false); continue; }
                return result;
            }
            919 => {
                if lookahead == 114 { state = 980; lexer.advance(false); continue; }
                return result;
            }
            920 => {
                if lookahead == 76 { state = 981; lexer.advance(false); continue; }
                return result;
            }
            921 => {
                if lookahead == 76 { state = 982; lexer.advance(false); continue; }
                return result;
            }
            922 => {
                if lookahead == 95 { state = 983; lexer.advance(false); continue; }
                return result;
            }
            923 => {
                if lookahead == 101 { state = 984; lexer.advance(false); continue; }
                return result;
            }
            924 => {
                if lookahead == 97 { state = 985; lexer.advance(false); continue; }
                return result;
            }
            925 => {
                if lookahead == 101 { state = 986; lexer.advance(false); continue; }
                return result;
            }
            926 => {
                if lookahead == 114 { state = 987; lexer.advance(false); continue; }
                return result;
            }
            927 => {
                if lookahead == 97 { state = 988; lexer.advance(false); continue; }
                return result;
            }
            928 => {
                if lookahead == 114 { state = 989; lexer.advance(false); continue; }
                return result;
            }
            929 => {
                if lookahead == 116 { state = 990; lexer.advance(false); continue; }
                return result;
            }
            930 => {
                result = true; lexer.set_result_symbol(anon_sym___declspec); lexer.mark_end();
                return result;
            }
            931 => {
                if lookahead == 101 { state = 991; lexer.advance(false); continue; }
                return result;
            }
            932 => {
                if lookahead == 110 { state = 992; lexer.advance(false); continue; }
                return result;
            }
            933 => {
                result = true; lexer.set_result_symbol(anon_sym___fastcall); lexer.mark_end();
                return result;
            }
            934 => {
                if lookahead == 105 { state = 993; lexer.advance(false); continue; }
                return result;
            }
            935 => {
                result = true; lexer.set_result_symbol(anon_sym___inline__); lexer.mark_end();
                return result;
            }
            936 => {
                result = true; lexer.set_result_symbol(anon_sym___nullable); lexer.mark_end();
                return result;
            }
            937 => {
                if lookahead == 111 { state = 994; lexer.advance(false); continue; }
                return result;
            }
            938 => {
                result = true; lexer.set_result_symbol(sym_ms_restrict_modifier); lexer.mark_end();
                if lookahead == 95 { state = 995; lexer.advance(false); continue; }
                return result;
            }
            939 => {
                result = true; lexer.set_result_symbol(anon_sym___thiscall); lexer.mark_end();
                return result;
            }
            940 => {
                result = true; lexer.set_result_symbol(anon_sym___typeof__); lexer.mark_end();
                return result;
            }
            941 => {
                if lookahead == 100 { state = 996; lexer.advance(false); continue; }
                return result;
            }
            942 => {
                if lookahead == 110 { state = 997; lexer.advance(false); continue; }
                return result;
            }
            943 => {
                if lookahead == 108 { state = 998; lexer.advance(false); continue; }
                return result;
            }
            944 => {
                if lookahead == 95 { state = 999; lexer.advance(false); continue; }
                return result;
            }
            945 => {
                result = true; lexer.set_result_symbol(anon_sym__unaligned); lexer.mark_end();
                return result;
            }
            946 => {
                if lookahead == 116 { state = 1000; lexer.advance(false); continue; }
                return result;
            }
            947 => {
                if lookahead == 116 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            948 => {
                if lookahead == 101 { state = 1001; lexer.advance(false); continue; }
                return result;
            }
            949 => {
                if lookahead == 97 { state = 1002; lexer.advance(false); continue; }
                return result;
            }
            950 => {
                if lookahead == 76 { state = 1003; lexer.advance(false); continue; }
                return result;
            }
            951 => {
                if lookahead == 84 { state = 1004; lexer.advance(false); continue; }
                return result;
            }
            952 => {
                if lookahead == 65 { state = 1005; lexer.advance(false); continue; }
                return result;
            }
            953 => {
                if lookahead == 85 { state = 1006; lexer.advance(false); continue; }
                return result;
            }
            954 => {
                if lookahead == 78 { state = 1007; lexer.advance(false); continue; }
                if lookahead == 82 { state = 1008; lexer.advance(false); continue; }
                return result;
            }
            955 => {
                if lookahead == 65 { state = 1009; lexer.advance(false); continue; }
                if lookahead == 77 { state = 1010; lexer.advance(false); continue; }
                return result;
            }
            956 => {
                if lookahead == 69 { state = 1011; lexer.advance(false); continue; }
                if lookahead == 83 { state = 1012; lexer.advance(false); continue; }
                return result;
            }
            957 => {
                if lookahead == 108 { state = 1013; lexer.advance(false); continue; }
                return result;
            }
            958 => {
                if lookahead == 76 { state = 1014; lexer.advance(false); continue; }
                return result;
            }
            959 => {
                if lookahead == 68 { state = 1015; lexer.advance(false); continue; }
                return result;
            }
            960 => {
                if lookahead == 69 { state = 1016; lexer.advance(false); continue; }
                return result;
            }
            961 => {
                if lookahead == 65 { state = 1017; lexer.advance(false); continue; }
                return result;
            }
            962 => {
                if lookahead == 80 { state = 1018; lexer.advance(false); continue; }
                return result;
            }
            963 => {
                if lookahead == 69 { state = 1019; lexer.advance(false); continue; }
                return result;
            }
            964 => {
                if lookahead == 73 { state = 1020; lexer.advance(false); continue; }
                return result;
            }
            965 => {
                if lookahead == 82 { state = 1021; lexer.advance(false); continue; }
                return result;
            }
            966 => {
                if lookahead == 78 { state = 1022; lexer.advance(false); continue; }
                return result;
            }
            967 => {
                if lookahead == 85 { state = 1023; lexer.advance(false); continue; }
                return result;
            }
            968 => {
                if lookahead == 95 { state = 1024; lexer.advance(false); continue; }
                return result;
            }
            969 => {
                if lookahead == 83 { state = 1025; lexer.advance(false); continue; }
                return result;
            }
            970 => {
                if lookahead == 77 { state = 1026; lexer.advance(false); continue; }
                return result;
            }
            971 => {
                if lookahead == 65 { state = 1027; lexer.advance(false); continue; }
                return result;
            }
            972 => {
                if lookahead == 66 { state = 1028; lexer.advance(false); continue; }
                return result;
            }
            973 => {
                if lookahead == 84 { state = 1029; lexer.advance(false); continue; }
                return result;
            }
            974 => {
                result = true; lexer.set_result_symbol(anon_sym_OBJC_EXPORT); lexer.mark_end();
                return result;
            }
            975 => {
                if lookahead == 76 { state = 1030; lexer.advance(false); continue; }
                return result;
            }
            976 => {
                if lookahead == 78 { state = 1031; lexer.advance(false); continue; }
                return result;
            }
            977 => {
                if lookahead == 67 { state = 1032; lexer.advance(false); continue; }
                return result;
            }
            978 => {
                if lookahead == 95 { state = 1033; lexer.advance(false); continue; }
                return result;
            }
            979 => {
                if lookahead == 99 { state = 1034; lexer.advance(false); continue; }
                return result;
            }
            980 => {
                if lookahead == 101 { state = 1035; lexer.advance(false); continue; }
                return result;
            }
            981 => {
                if lookahead == 65 { state = 1036; lexer.advance(false); continue; }
                return result;
            }
            982 => {
                if lookahead == 65 { state = 1037; lexer.advance(false); continue; }
                return result;
            }
            983 => {
                result = true; lexer.set_result_symbol(anon_sym___alignof__); lexer.mark_end();
                return result;
            }
            984 => {
                result = true; lexer.set_result_symbol(anon_sym___attribute); lexer.mark_end();
                if lookahead == 95 { state = 1038; lexer.advance(false); continue; }
                return result;
            }
            985 => {
                if lookahead == 115 { state = 1039; lexer.advance(false); continue; }
                return result;
            }
            986 => {
                if lookahead == 116 { state = 1040; lexer.advance(false); continue; }
                return result;
            }
            987 => {
                if lookahead == 97 { state = 1041; lexer.advance(false); continue; }
                return result;
            }
            988 => {
                if lookahead == 118 { state = 1042; lexer.advance(false); continue; }
                return result;
            }
            989 => {
                if lookahead == 105 { state = 1043; lexer.advance(false); continue; }
                return result;
            }
            990 => {
                result = true; lexer.set_result_symbol(anon_sym___covariant); lexer.mark_end();
                return result;
            }
            991 => {
                if lookahead == 100 { state = 1044; lexer.advance(false); continue; }
                return result;
            }
            992 => {
                if lookahead == 95 { state = 1045; lexer.advance(false); continue; }
                return result;
            }
            993 => {
                if lookahead == 110 { state = 1046; lexer.advance(false); continue; }
                return result;
            }
            994 => {
                if lookahead == 98 { state = 1047; lexer.advance(false); continue; }
                return result;
            }
            995 => {
                if lookahead == 95 { state = 1048; lexer.advance(false); continue; }
                return result;
            }
            996 => {
                result = true; lexer.set_result_symbol(anon_sym___unaligned); lexer.mark_end();
                return result;
            }
            997 => {
                if lookahead == 114 { state = 1049; lexer.advance(false); continue; }
                return result;
            }
            998 => {
                if lookahead == 108 { state = 1050; lexer.advance(false); continue; }
                return result;
            }
            999 => {
                if lookahead == 95 { state = 1051; lexer.advance(false); continue; }
                return result;
            }
            1000 => {
                if lookahead == 121 { state = 1052; lexer.advance(false); continue; }
                return result;
            }
            1001 => {
                if lookahead == 95 { state = 1053; lexer.advance(false); continue; }
                return result;
            }
            1002 => {
                if lookahead == 108 { state = 1054; lexer.advance(false); continue; }
                return result;
            }
            1003 => {
                if lookahead == 69 { state = 1055; lexer.advance(false); continue; }
                return result;
            }
            1004 => {
                if lookahead == 69 { state = 1056; lexer.advance(false); continue; }
                return result;
            }
            1005 => {
                if lookahead == 66 { state = 1057; lexer.advance(false); continue; }
                return result;
            }
            1006 => {
                if lookahead == 78 { state = 1058; lexer.advance(false); continue; }
                return result;
            }
            1007 => {
                if lookahead == 79 { state = 1059; lexer.advance(false); continue; }
                return result;
            }
            1008 => {
                if lookahead == 69 { state = 1060; lexer.advance(false); continue; }
                return result;
            }
            1009 => {
                if lookahead == 84 { state = 1061; lexer.advance(false); continue; }
                return result;
            }
            1010 => {
                if lookahead == 83 { state = 1062; lexer.advance(false); continue; }
                return result;
            }
            1011 => {
                if lookahead == 88 { state = 1063; lexer.advance(false); continue; }
                return result;
            }
            1012 => {
                if lookahead == 84 { state = 1064; lexer.advance(false); continue; }
                return result;
            }
            1013 => {
                if lookahead == 101 { state = 1065; lexer.advance(false); continue; }
                return result;
            }
            1014 => {
                if lookahead == 69 { state = 1066; lexer.advance(false); continue; }
                return result;
            }
            1015 => {
                if lookahead == 95 { state = 1067; lexer.advance(false); continue; }
                return result;
            }
            1016 => {
                result = true; lexer.set_result_symbol(anon_sym_NS_AVAILABLE); lexer.mark_end();
                if lookahead == 95 { state = 1068; lexer.advance(false); continue; }
                return result;
            }
            1017 => {
                if lookahead == 73 { state = 1069; lexer.advance(false); continue; }
                return result;
            }
            1018 => {
                if lookahead == 82 { state = 1070; lexer.advance(false); continue; }
                return result;
            }
            1019 => {
                if lookahead == 68 { state = 1071; lexer.advance(false); continue; }
                return result;
            }
            1020 => {
                if lookahead == 76 { state = 1072; lexer.advance(false); continue; }
                return result;
            }
            1021 => {
                if lookahead == 69 { state = 1073; lexer.advance(false); continue; }
                return result;
            }
            1022 => {
                if lookahead == 95 { state = 1074; lexer.advance(false); continue; }
                return result;
            }
            1023 => {
                if lookahead == 78 { state = 1075; lexer.advance(false); continue; }
                return result;
            }
            1024 => {
                if lookahead == 78 { state = 1076; lexer.advance(false); continue; }
                return result;
            }
            1025 => {
                if lookahead == 83 { state = 1077; lexer.advance(false); continue; }
                return result;
            }
            1026 => {
                if lookahead == 69 { state = 1078; lexer.advance(false); continue; }
                return result;
            }
            1027 => {
                if lookahead == 86 { state = 1079; lexer.advance(false); continue; }
                return result;
            }
            1028 => {
                if lookahead == 76 { state = 1080; lexer.advance(false); continue; }
                return result;
            }
            1029 => {
                if lookahead == 73 { state = 1081; lexer.advance(false); continue; }
                return result;
            }
            1030 => {
                if lookahead == 65 { state = 1082; lexer.advance(false); continue; }
                return result;
            }
            1031 => {
                result = true; lexer.set_result_symbol(anon_sym_UIKIT_EXTERN); lexer.mark_end();
                return result;
            }
            1032 => {
                if lookahead == 69 { state = 1083; lexer.advance(false); continue; }
                return result;
            }
            1033 => {
                if lookahead == 65 { state = 1084; lexer.advance(false); continue; }
                return result;
            }
            1034 => {
                if lookahead == 105 { state = 1085; lexer.advance(false); continue; }
                return result;
            }
            1035 => {
                if lookahead == 115 { state = 1086; lexer.advance(false); continue; }
                return result;
            }
            1036 => {
                if lookahead == 66 { state = 1087; lexer.advance(false); continue; }
                return result;
            }
            1037 => {
                if lookahead == 66 { state = 1088; lexer.advance(false); continue; }
                return result;
            }
            1038 => {
                if lookahead == 95 { state = 1089; lexer.advance(false); continue; }
                return result;
            }
            1039 => {
                if lookahead == 105 { state = 1090; lexer.advance(false); continue; }
                return result;
            }
            1040 => {
                if lookahead == 97 { state = 1091; lexer.advance(false); continue; }
                return result;
            }
            1041 => {
                if lookahead == 110 { state = 1092; lexer.advance(false); continue; }
                return result;
            }
            1042 => {
                if lookahead == 97 { state = 1093; lexer.advance(false); continue; }
                return result;
            }
            1043 => {
                if lookahead == 97 { state = 1094; lexer.advance(false); continue; }
                return result;
            }
            1044 => {
                if lookahead == 95 { state = 1095; lexer.advance(false); continue; }
                return result;
            }
            1045 => {
                if lookahead == 95 { state = 1096; lexer.advance(false); continue; }
                return result;
            }
            1046 => {
                if lookahead == 101 { state = 1097; lexer.advance(false); continue; }
                return result;
            }
            1047 => {
                if lookahead == 106 { state = 1098; lexer.advance(false); continue; }
                return result;
            }
            1048 => {
                result = true; lexer.set_result_symbol(anon_sym___restrict__); lexer.mark_end();
                return result;
            }
            1049 => {
                if lookahead == 101 { state = 1099; lexer.advance(false); continue; }
                return result;
            }
            1050 => {
                result = true; lexer.set_result_symbol(anon_sym___vectorcall); lexer.mark_end();
                return result;
            }
            1051 => {
                result = true; lexer.set_result_symbol(anon_sym___volatile__); lexer.mark_end();
                return result;
            }
            1052 => {
                result = true; lexer.set_result_symbol(anon_sym_availability); lexer.mark_end();
                return result;
            }
            1053 => {
                if lookahead == 114 { state = 1100; lexer.advance(false); continue; }
                return result;
            }
            1054 => {
                result = true; lexer.set_result_symbol(anon_sym_thread_local); lexer.mark_end();
                return result;
            }
            1055 => {
                result = true; lexer.set_result_symbol(anon_sym_API_AVAILABLE); lexer.mark_end();
                return result;
            }
            1056 => {
                if lookahead == 68 { state = 1101; lexer.advance(false); continue; }
                return result;
            }
            1057 => {
                if lookahead == 76 { state = 1102; lexer.advance(false); continue; }
                return result;
            }
            1058 => {
                if lookahead == 67 { state = 1103; lexer.advance(false); continue; }
                return result;
            }
            1059 => {
                if lookahead == 84 { state = 1104; lexer.advance(false); continue; }
                return result;
            }
            1060 => {
                if lookahead == 84 { state = 1105; lexer.advance(false); continue; }
                return result;
            }
            1061 => {
                if lookahead == 84 { state = 1106; lexer.advance(false); continue; }
                return result;
            }
            1062 => {
                if lookahead == 71 { state = 1107; lexer.advance(false); continue; }
                return result;
            }
            1063 => {
                if lookahead == 80 { state = 1108; lexer.advance(false); continue; }
                if lookahead == 84 { state = 1109; lexer.advance(false); continue; }
                return result;
            }
            1064 => {
                if lookahead == 65 { state = 1110; lexer.advance(false); continue; }
                return result;
            }
            1065 => {
                result = true; lexer.set_result_symbol(anon_sym_IBInspectable); lexer.mark_end();
                return result;
            }
            1066 => {
                result = true; lexer.set_result_symbol(anon_sym_IB_DESIGNABLE); lexer.mark_end();
                return result;
            }
            1067 => {
                if lookahead == 82 { state = 1111; lexer.advance(false); continue; }
                return result;
            }
            1068 => {
                if lookahead == 73 { state = 1112; lexer.advance(false); continue; }
                return result;
            }
            1069 => {
                if lookahead == 76 { state = 1113; lexer.advance(false); continue; }
                return result;
            }
            1070 => {
                if lookahead == 69 { state = 1114; lexer.advance(false); continue; }
                return result;
            }
            1071 => {
                if lookahead == 95 { state = 1115; lexer.advance(false); continue; }
                return result;
            }
            1072 => {
                if lookahead == 65 { state = 1116; lexer.advance(false); continue; }
                return result;
            }
            1073 => {
                if lookahead == 67 { state = 1117; lexer.advance(false); continue; }
                return result;
            }
            1074 => {
                if lookahead == 85 { state = 1118; lexer.advance(false); continue; }
                return result;
            }
            1075 => {
                if lookahead == 67 { state = 1119; lexer.advance(false); continue; }
                return result;
            }
            1076 => {
                if lookahead == 73 { state = 1120; lexer.advance(false); continue; }
                return result;
            }
            1077 => {
                result = true; lexer.set_result_symbol(anon_sym_NS_ROOT_CLASS); lexer.mark_end();
                return result;
            }
            1078 => {
                result = true; lexer.set_result_symbol(anon_sym_NS_SWIFT_NAME); lexer.mark_end();
                return result;
            }
            1079 => {
                if lookahead == 65 { state = 1121; lexer.advance(false); continue; }
                return result;
            }
            1080 => {
                if lookahead == 69 { state = 1122; lexer.advance(false); continue; }
                return result;
            }
            1081 => {
                if lookahead == 76 { state = 1123; lexer.advance(false); continue; }
                return result;
            }
            1082 => {
                if lookahead == 83 { state = 1124; lexer.advance(false); continue; }
                return result;
            }
            1083 => {
                if lookahead == 95 { state = 1125; lexer.advance(false); continue; }
                return result;
            }
            1084 => {
                if lookahead == 84 { state = 1126; lexer.advance(false); continue; }
                return result;
            }
            1085 => {
                if lookahead == 102 { state = 1127; lexer.advance(false); continue; }
                return result;
            }
            1086 => {
                if lookahead == 117 { state = 1128; lexer.advance(false); continue; }
                return result;
            }
            1087 => {
                if lookahead == 76 { state = 1129; lexer.advance(false); continue; }
                return result;
            }
            1088 => {
                if lookahead == 76 { state = 1130; lexer.advance(false); continue; }
                return result;
            }
            1089 => {
                result = true; lexer.set_result_symbol(anon_sym___attribute__); lexer.mark_end();
                return result;
            }
            1090 => {
                if lookahead == 110 { state = 1131; lexer.advance(false); continue; }
                return result;
            }
            1091 => {
                if lookahead == 105 { state = 1132; lexer.advance(false); continue; }
                return result;
            }
            1092 => {
                if lookahead == 115 { state = 1133; lexer.advance(false); continue; }
                return result;
            }
            1093 => {
                if lookahead == 105 { state = 1134; lexer.advance(false); continue; }
                return result;
            }
            1094 => {
                if lookahead == 110 { state = 1135; lexer.advance(false); continue; }
                return result;
            }
            1095 => {
                if lookahead == 101 { state = 1136; lexer.advance(false); continue; }
                if lookahead == 109 { state = 1137; lexer.advance(false); continue; }
                return result;
            }
            1096 => {
                result = true; lexer.set_result_symbol(anon_sym___extension__); lexer.mark_end();
                return result;
            }
            1097 => {
                result = true; lexer.set_result_symbol(anon_sym___forceinline); lexer.mark_end();
                return result;
            }
            1098 => {
                if lookahead == 99 { state = 1138; lexer.advance(false); continue; }
                return result;
            }
            1099 => {
                if lookahead == 116 { state = 1139; lexer.advance(false); continue; }
                return result;
            }
            1100 => {
                if lookahead == 101 { state = 1140; lexer.advance(false); continue; }
                return result;
            }
            1101 => {
                result = true; lexer.set_result_symbol(anon_sym_API_DEPRECATED); lexer.mark_end();
                return result;
            }
            1102 => {
                if lookahead == 69 { state = 1141; lexer.advance(false); continue; }
                return result;
            }
            1103 => {
                if lookahead == 84 { state = 1142; lexer.advance(false); continue; }
                return result;
            }
            1104 => {
                if lookahead == 95 { state = 1143; lexer.advance(false); continue; }
                return result;
            }
            1105 => {
                if lookahead == 65 { state = 1144; lexer.advance(false); continue; }
                return result;
            }
            1106 => {
                if lookahead == 82 { state = 1145; lexer.advance(false); continue; }
                return result;
            }
            1107 => {
                if lookahead == 95 { state = 1146; lexer.advance(false); continue; }
                return result;
            }
            1108 => {
                if lookahead == 79 { state = 1147; lexer.advance(false); continue; }
                return result;
            }
            1109 => {
                if lookahead == 69 { state = 1148; lexer.advance(false); continue; }
                return result;
            }
            1110 => {
                if lookahead == 84 { state = 1149; lexer.advance(false); continue; }
                return result;
            }
            1111 => {
                if lookahead == 69 { state = 1150; lexer.advance(false); continue; }
                return result;
            }
            1112 => {
                if lookahead == 79 { state = 1151; lexer.advance(false); continue; }
                return result;
            }
            1113 => {
                if lookahead == 65 { state = 1152; lexer.advance(false); continue; }
                return result;
            }
            1114 => {
                if lookahead == 67 { state = 1153; lexer.advance(false); continue; }
                return result;
            }
            1115 => {
                if lookahead == 73 { state = 1154; lexer.advance(false); continue; }
                return result;
            }
            1116 => {
                if lookahead == 66 { state = 1155; lexer.advance(false); continue; }
                return result;
            }
            1117 => {
                if lookahead == 65 { state = 1156; lexer.advance(false); continue; }
                return result;
            }
            1118 => {
                if lookahead == 78 { state = 1157; lexer.advance(false); continue; }
                return result;
            }
            1119 => {
                if lookahead == 84 { state = 1158; lexer.advance(false); continue; }
                return result;
            }
            1120 => {
                if lookahead == 76 { state = 1159; lexer.advance(false); continue; }
                return result;
            }
            1121 => {
                if lookahead == 73 { state = 1160; lexer.advance(false); continue; }
                return result;
            }
            1122 => {
                result = true; lexer.set_result_symbol(anon_sym_NS_UNAVAILABLE); lexer.mark_end();
                return result;
            }
            1123 => {
                if lookahead == 95 { state = 1161; lexer.advance(false); continue; }
                return result;
            }
            1124 => {
                if lookahead == 83 { state = 1162; lexer.advance(false); continue; }
                return result;
            }
            1125 => {
                if lookahead == 83 { state = 1163; lexer.advance(false); continue; }
                return result;
            }
            1126 => {
                if lookahead == 84 { state = 1164; lexer.advance(false); continue; }
                return result;
            }
            1127 => {
                if lookahead == 105 { state = 1165; lexer.advance(false); continue; }
                return result;
            }
            1128 => {
                if lookahead == 108 { state = 1166; lexer.advance(false); continue; }
                return result;
            }
            1129 => {
                if lookahead == 69 { state = 1167; lexer.advance(false); continue; }
                return result;
            }
            1130 => {
                if lookahead == 69 { state = 1168; lexer.advance(false); continue; }
                return result;
            }
            1131 => {
                if lookahead == 103 { state = 1169; lexer.advance(false); continue; }
                return result;
            }
            1132 => {
                if lookahead == 110 { state = 1170; lexer.advance(false); continue; }
                return result;
            }
            1133 => {
                if lookahead == 102 { state = 1171; lexer.advance(false); continue; }
                return result;
            }
            1134 => {
                if lookahead == 108 { state = 1172; lexer.advance(false); continue; }
                return result;
            }
            1135 => {
                if lookahead == 116 { state = 1173; lexer.advance(false); continue; }
                return result;
            }
            1136 => {
                if lookahead == 110 { state = 1174; lexer.advance(false); continue; }
                return result;
            }
            1137 => {
                if lookahead == 115 { state = 1175; lexer.advance(false); continue; }
                return result;
            }
            1138 => {
                if lookahead == 95 { state = 1176; lexer.advance(false); continue; }
                return result;
            }
            1139 => {
                if lookahead == 97 { state = 1177; lexer.advance(false); continue; }
                return result;
            }
            1140 => {
                if lookahead == 108 { state = 1178; lexer.advance(false); continue; }
                return result;
            }
            1141 => {
                result = true; lexer.set_result_symbol(anon_sym_API_UNAVAILABLE); lexer.mark_end();
                return result;
            }
            1142 => {
                if lookahead == 73 { state = 1179; lexer.advance(false); continue; }
                return result;
            }
            1143 => {
                if lookahead == 82 { state = 1180; lexer.advance(false); continue; }
                return result;
            }
            1144 => {
                if lookahead == 73 { state = 1181; lexer.advance(false); continue; }
                return result;
            }
            1145 => {
                if lookahead == 73 { state = 1182; lexer.advance(false); continue; }
                return result;
            }
            1146 => {
                if lookahead == 65 { state = 1183; lexer.advance(false); continue; }
                return result;
            }
            1147 => {
                if lookahead == 82 { state = 1184; lexer.advance(false); continue; }
                return result;
            }
            1148 => {
                if lookahead == 82 { state = 1185; lexer.advance(false); continue; }
                return result;
            }
            1149 => {
                if lookahead == 73 { state = 1186; lexer.advance(false); continue; }
                return result;
            }
            1150 => {
                if lookahead == 70 { state = 1187; lexer.advance(false); continue; }
                return result;
            }
            1151 => {
                if lookahead == 83 { state = 1188; lexer.advance(false); continue; }
                return result;
            }
            1152 => {
                if lookahead == 66 { state = 1189; lexer.advance(false); continue; }
                return result;
            }
            1153 => {
                if lookahead == 65 { state = 1190; lexer.advance(false); continue; }
                return result;
            }
            1154 => {
                if lookahead == 79 { state = 1191; lexer.advance(false); continue; }
                return result;
            }
            1155 => {
                if lookahead == 76 { state = 1192; lexer.advance(false); continue; }
                return result;
            }
            1156 => {
                if lookahead == 84 { state = 1193; lexer.advance(false); continue; }
                return result;
            }
            1157 => {
                if lookahead == 65 { state = 1194; lexer.advance(false); continue; }
                return result;
            }
            1158 => {
                if lookahead == 73 { state = 1195; lexer.advance(false); continue; }
                return result;
            }
            1159 => {
                if lookahead == 95 { state = 1196; lexer.advance(false); continue; }
                return result;
            }
            1160 => {
                if lookahead == 76 { state = 1197; lexer.advance(false); continue; }
                return result;
            }
            1161 => {
                if lookahead == 69 { state = 1198; lexer.advance(false); continue; }
                return result;
            }
            1162 => {
                result = true; lexer.set_result_symbol(anon_sym_OBJC_ROOT_CLASS); lexer.mark_end();
                return result;
            }
            1163 => {
                if lookahead == 69 { state = 1199; lexer.advance(false); continue; }
                return result;
            }
            1164 => {
                if lookahead == 82 { state = 1200; lexer.advance(false); continue; }
                return result;
            }
            1165 => {
                if lookahead == 101 { state = 1201; lexer.advance(false); continue; }
                return result;
            }
            1166 => {
                if lookahead == 116 { state = 1202; lexer.advance(false); continue; }
                return result;
            }
            1167 => {
                result = true; lexer.set_result_symbol(anon_sym___IOS_AVAILABLE); lexer.mark_end();
                return result;
            }
            1168 => {
                if lookahead == 95 { state = 1203; lexer.advance(false); continue; }
                return result;
            }
            1169 => {
                result = true; lexer.set_result_symbol(anon_sym___autoreleasing); lexer.mark_end();
                return result;
            }
            1170 => {
                if lookahead == 101 { state = 1204; lexer.advance(false); continue; }
                return result;
            }
            1171 => {
                if lookahead == 101 { state = 1205; lexer.advance(false); continue; }
                return result;
            }
            1172 => {
                if lookahead == 97 { state = 1206; lexer.advance(false); continue; }
                return result;
            }
            1173 => {
                result = true; lexer.set_result_symbol(anon_sym___contravariant); lexer.mark_end();
                return result;
            }
            1174 => {
                if lookahead == 117 { state = 1207; lexer.advance(false); continue; }
                return result;
            }
            1175 => {
                if lookahead == 103 { state = 1208; lexer.advance(false); continue; }
                return result;
            }
            1176 => {
                if lookahead == 99 { state = 1209; lexer.advance(false); continue; }
                if lookahead == 105 { state = 1210; lexer.advance(false); continue; }
                if lookahead == 115 { state = 1211; lexer.advance(false); continue; }
                return result;
            }
            1177 => {
                if lookahead == 105 { state = 1212; lexer.advance(false); continue; }
                return result;
            }
            1178 => {
                if lookahead == 97 { state = 1213; lexer.advance(false); continue; }
                return result;
            }
            1179 => {
                if lookahead == 79 { state = 1214; lexer.advance(false); continue; }
                return result;
            }
            1180 => {
                if lookahead == 69 { state = 1215; lexer.advance(false); continue; }
                return result;
            }
            1181 => {
                if lookahead == 78 { state = 1216; lexer.advance(false); continue; }
                return result;
            }
            1182 => {
                if lookahead == 66 { state = 1217; lexer.advance(false); continue; }
                return result;
            }
            1183 => {
                if lookahead == 84 { state = 1218; lexer.advance(false); continue; }
                return result;
            }
            1184 => {
                if lookahead == 84 { state = 1219; lexer.advance(false); continue; }
                return result;
            }
            1185 => {
                if lookahead == 78 { state = 1220; lexer.advance(false); continue; }
                return result;
            }
            1186 => {
                if lookahead == 67 { state = 1221; lexer.advance(false); continue; }
                return result;
            }
            1187 => {
                if lookahead == 67 { state = 1222; lexer.advance(false); continue; }
                return result;
            }
            1188 => {
                result = true; lexer.set_result_symbol(anon_sym_NS_AVAILABLE_IOS); lexer.mark_end();
                return result;
            }
            1189 => {
                if lookahead == 76 { state = 1223; lexer.advance(false); continue; }
                return result;
            }
            1190 => {
                if lookahead == 84 { state = 1224; lexer.advance(false); continue; }
                return result;
            }
            1191 => {
                if lookahead == 83 { state = 1225; lexer.advance(false); continue; }
                return result;
            }
            1192 => {
                if lookahead == 69 { state = 1226; lexer.advance(false); continue; }
                return result;
            }
            1193 => {
                if lookahead == 69 { state = 1227; lexer.advance(false); continue; }
                return result;
            }
            1194 => {
                if lookahead == 86 { state = 1228; lexer.advance(false); continue; }
                return result;
            }
            1195 => {
                if lookahead == 79 { state = 1229; lexer.advance(false); continue; }
                return result;
            }
            1196 => {
                if lookahead == 84 { state = 1230; lexer.advance(false); continue; }
                return result;
            }
            1197 => {
                if lookahead == 65 { state = 1231; lexer.advance(false); continue; }
                return result;
            }
            1198 => {
                if lookahead == 78 { state = 1232; lexer.advance(false); continue; }
                return result;
            }
            1199 => {
                if lookahead == 76 { state = 1233; lexer.advance(false); continue; }
                return result;
            }
            1200 => {
                if lookahead == 73 { state = 1234; lexer.advance(false); continue; }
                return result;
            }
            1201 => {
                if lookahead == 100 { state = 1235; lexer.advance(false); continue; }
                return result;
            }
            1202 => {
                result = true; lexer.set_result_symbol(anon_sym__Nullable_result); lexer.mark_end();
                return result;
            }
            1203 => {
                if lookahead == 83 { state = 1236; lexer.advance(false); continue; }
                return result;
            }
            1204 => {
                if lookahead == 100 { state = 1237; lexer.advance(false); continue; }
                return result;
            }
            1205 => {
                if lookahead == 114 { state = 1238; lexer.advance(false); continue; }
                return result;
            }
            1206 => {
                if lookahead == 98 { state = 1239; lexer.advance(false); continue; }
                return result;
            }
            1207 => {
                if lookahead == 109 { state = 1240; lexer.advance(false); continue; }
                return result;
            }
            1208 => {
                result = true; lexer.set_result_symbol(anon_sym___deprecated_msg); lexer.mark_end();
                return result;
            }
            1209 => {
                if lookahead == 108 { state = 1241; lexer.advance(false); continue; }
                return result;
            }
            1210 => {
                if lookahead == 115 { state = 1242; lexer.advance(false); continue; }
                return result;
            }
            1211 => {
                if lookahead == 117 { state = 1243; lexer.advance(false); continue; }
                return result;
            }
            1212 => {
                if lookahead == 110 { state = 1244; lexer.advance(false); continue; }
                return result;
            }
            1213 => {
                if lookahead == 116 { state = 1245; lexer.advance(false); continue; }
                return result;
            }
            1214 => {
                if lookahead == 78 { state = 1246; lexer.advance(false); continue; }
                return result;
            }
            1215 => {
                if lookahead == 84 { state = 1247; lexer.advance(false); continue; }
                return result;
            }
            1216 => {
                if lookahead == 69 { state = 1248; lexer.advance(false); continue; }
                return result;
            }
            1217 => {
                if lookahead == 85 { state = 1249; lexer.advance(false); continue; }
                return result;
            }
            1218 => {
                if lookahead == 84 { state = 1250; lexer.advance(false); continue; }
                return result;
            }
            1219 => {
                result = true; lexer.set_result_symbol(anon_sym_FOUNDATION_EXPORT); lexer.mark_end();
                return result;
            }
            1220 => {
                result = true; lexer.set_result_symbol(anon_sym_FOUNDATION_EXTERN); lexer.mark_end();
                return result;
            }
            1221 => {
                if lookahead == 95 { state = 1251; lexer.advance(false); continue; }
                return result;
            }
            1222 => {
                if lookahead == 79 { state = 1252; lexer.advance(false); continue; }
                return result;
            }
            1223 => {
                if lookahead == 69 { state = 1253; lexer.advance(false); continue; }
                return result;
            }
            1224 => {
                if lookahead == 69 { state = 1254; lexer.advance(false); continue; }
                return result;
            }
            1225 => {
                result = true; lexer.set_result_symbol(anon_sym_NS_DEPRECATED_IOS); lexer.mark_end();
                return result;
            }
            1226 => {
                if lookahead == 95 { state = 1255; lexer.advance(false); continue; }
                return result;
            }
            1227 => {
                if lookahead == 68 { state = 1256; lexer.advance(false); continue; }
                return result;
            }
            1228 => {
                if lookahead == 65 { state = 1257; lexer.advance(false); continue; }
                return result;
            }
            1229 => {
                if lookahead == 78 { state = 1258; lexer.advance(false); continue; }
                return result;
            }
            1230 => {
                if lookahead == 69 { state = 1259; lexer.advance(false); continue; }
                return result;
            }
            1231 => {
                if lookahead == 66 { state = 1260; lexer.advance(false); continue; }
                return result;
            }
            1232 => {
                if lookahead == 68 { state = 1261; lexer.advance(false); continue; }
                return result;
            }
            1233 => {
                if lookahead == 69 { state = 1262; lexer.advance(false); continue; }
                return result;
            }
            1234 => {
                if lookahead == 66 { state = 1263; lexer.advance(false); continue; }
                return result;
            }
            1235 => {
                result = true; lexer.set_result_symbol(anon_sym__Null_unspecified); lexer.mark_end();
                return result;
            }
            1236 => {
                if lookahead == 84 { state = 1264; lexer.advance(false); continue; }
                return result;
            }
            1237 => {
                result = true; lexer.set_result_symbol(anon_sym___bridge_retained); lexer.mark_end();
                return result;
            }
            1238 => {
                result = true; lexer.set_result_symbol(anon_sym___bridge_transfer); lexer.mark_end();
                return result;
            }
            1239 => {
                if lookahead == 108 { state = 1265; lexer.advance(false); continue; }
                return result;
            }
            1240 => {
                if lookahead == 95 { state = 1266; lexer.advance(false); continue; }
                return result;
            }
            1241 => {
                if lookahead == 97 { state = 1267; lexer.advance(false); continue; }
                return result;
            }
            1242 => {
                if lookahead == 97 { state = 1268; lexer.advance(false); continue; }
                return result;
            }
            1243 => {
                if lookahead == 112 { state = 1269; lexer.advance(false); continue; }
                return result;
            }
            1244 => {
                if lookahead == 101 { state = 1270; lexer.advance(false); continue; }
                return result;
            }
            1245 => {
                if lookahead == 101 { state = 1271; lexer.advance(false); continue; }
                return result;
            }
            1246 => {
                result = true; lexer.set_result_symbol(anon_sym_CF_FORMAT_FUNCTION); lexer.mark_end();
                return result;
            }
            1247 => {
                if lookahead == 65 { state = 1272; lexer.advance(false); continue; }
                return result;
            }
            1248 => {
                if lookahead == 68 { state = 1273; lexer.advance(false); continue; }
                return result;
            }
            1249 => {
                if lookahead == 84 { state = 1274; lexer.advance(false); continue; }
                return result;
            }
            1250 => {
                if lookahead == 82 { state = 1275; lexer.advance(false); continue; }
                return result;
            }
            1251 => {
                if lookahead == 73 { state = 1276; lexer.advance(false); continue; }
                return result;
            }
            1252 => {
                if lookahead == 85 { state = 1277; lexer.advance(false); continue; }
                return result;
            }
            1253 => {
                if lookahead == 95 { state = 1278; lexer.advance(false); continue; }
                return result;
            }
            1254 => {
                if lookahead == 68 { state = 1279; lexer.advance(false); continue; }
                return result;
            }
            1255 => {
                if lookahead == 73 { state = 1280; lexer.advance(false); continue; }
                return result;
            }
            1256 => {
                if lookahead == 95 { state = 1281; lexer.advance(false); continue; }
                return result;
            }
            1257 => {
                if lookahead == 73 { state = 1282; lexer.advance(false); continue; }
                return result;
            }
            1258 => {
                result = true; lexer.set_result_symbol(anon_sym_NS_FORMAT_FUNCTION); lexer.mark_end();
                return result;
            }
            1259 => {
                if lookahead == 82 { state = 1283; lexer.advance(false); continue; }
                return result;
            }
            1260 => {
                if lookahead == 76 { state = 1284; lexer.advance(false); continue; }
                return result;
            }
            1261 => {
                if lookahead == 95 { state = 1285; lexer.advance(false); continue; }
                return result;
            }
            1262 => {
                if lookahead == 67 { state = 1286; lexer.advance(false); continue; }
                return result;
            }
            1263 => {
                if lookahead == 85 { state = 1287; lexer.advance(false); continue; }
                return result;
            }
            1264 => {
                if lookahead == 65 { state = 1288; lexer.advance(false); continue; }
                return result;
            }
            1265 => {
                if lookahead == 101 { state = 1289; lexer.advance(false); continue; }
                return result;
            }
            1266 => {
                if lookahead == 109 { state = 1290; lexer.advance(false); continue; }
                return result;
            }
            1267 => {
                if lookahead == 115 { state = 1291; lexer.advance(false); continue; }
                return result;
            }
            1268 => {
                if lookahead == 95 { state = 1292; lexer.advance(false); continue; }
                return result;
            }
            1269 => {
                if lookahead == 101 { state = 1293; lexer.advance(false); continue; }
                return result;
            }
            1270 => {
                if lookahead == 100 { state = 1294; lexer.advance(false); continue; }
                return result;
            }
            1271 => {
                if lookahead == 100 { state = 1295; lexer.advance(false); continue; }
                return result;
            }
            1272 => {
                if lookahead == 73 { state = 1296; lexer.advance(false); continue; }
                return result;
            }
            1273 => {
                result = true; lexer.set_result_symbol(anon_sym_CF_RETURNS_RETAINED); lexer.mark_end();
                return result;
            }
            1274 => {
                if lookahead == 69 { state = 1297; lexer.advance(false); continue; }
                return result;
            }
            1275 => {
                if lookahead == 73 { state = 1298; lexer.advance(false); continue; }
                return result;
            }
            1276 => {
                if lookahead == 78 { state = 1299; lexer.advance(false); continue; }
                return result;
            }
            1277 => {
                if lookahead == 78 { state = 1300; lexer.advance(false); continue; }
                return result;
            }
            1278 => {
                if lookahead == 73 { state = 1301; lexer.advance(false); continue; }
                return result;
            }
            1279 => {
                if lookahead == 95 { state = 1302; lexer.advance(false); continue; }
                return result;
            }
            1280 => {
                if lookahead == 79 { state = 1303; lexer.advance(false); continue; }
                return result;
            }
            1281 => {
                if lookahead == 73 { state = 1304; lexer.advance(false); continue; }
                return result;
            }
            1282 => {
                if lookahead == 76 { state = 1305; lexer.advance(false); continue; }
                return result;
            }
            1283 => {
                if lookahead == 77 { state = 1306; lexer.advance(false); continue; }
                return result;
            }
            1284 => {
                if lookahead == 69 { state = 1307; lexer.advance(false); continue; }
                return result;
            }
            1285 => {
                if lookahead == 79 { state = 1308; lexer.advance(false); continue; }
                return result;
            }
            1286 => {
                if lookahead == 84 { state = 1309; lexer.advance(false); continue; }
                return result;
            }
            1287 => {
                if lookahead == 84 { state = 1310; lexer.advance(false); continue; }
                return result;
            }
            1288 => {
                if lookahead == 82 { state = 1311; lexer.advance(false); continue; }
                return result;
            }
            1289 => {
                result = true; lexer.set_result_symbol(anon_sym___builtin_available); lexer.mark_end();
                return result;
            }
            1290 => {
                if lookahead == 115 { state = 1312; lexer.advance(false); continue; }
                return result;
            }
            1291 => {
                if lookahead == 115 { state = 1313; lexer.advance(false); continue; }
                return result;
            }
            1292 => {
                if lookahead == 112 { state = 1314; lexer.advance(false); continue; }
                return result;
            }
            1293 => {
                if lookahead == 114 { state = 1315; lexer.advance(false); continue; }
                return result;
            }
            1294 => {
                result = true; lexer.set_result_symbol(anon_sym___unsafe_unretained); lexer.mark_end();
                return result;
            }
            1295 => {
                result = true; lexer.set_result_symbol(anon_sym_objc_bridge_related); lexer.mark_end();
                return result;
            }
            1296 => {
                if lookahead == 78 { state = 1316; lexer.advance(false); continue; }
                return result;
            }
            1297 => {
                result = true; lexer.set_result_symbol(anon_sym_DEPRECATED_ATTRIBUTE); lexer.mark_end();
                return result;
            }
            1298 => {
                if lookahead == 66 { state = 1317; lexer.advance(false); continue; }
                return result;
            }
            1299 => {
                if lookahead == 76 { state = 1318; lexer.advance(false); continue; }
                return result;
            }
            1300 => {
                if lookahead == 84 { state = 1319; lexer.advance(false); continue; }
                return result;
            }
            1301 => {
                if lookahead == 79 { state = 1320; lexer.advance(false); continue; }
                return result;
            }
            1302 => {
                if lookahead == 73 { state = 1321; lexer.advance(false); continue; }
                return result;
            }
            1303 => {
                if lookahead == 83 { state = 1322; lexer.advance(false); continue; }
                return result;
            }
            1304 => {
                if lookahead == 79 { state = 1323; lexer.advance(false); continue; }
                return result;
            }
            1305 => {
                if lookahead == 65 { state = 1324; lexer.advance(false); continue; }
                return result;
            }
            1306 => {
                if lookahead == 73 { state = 1325; lexer.advance(false); continue; }
                return result;
            }
            1307 => {
                result = true; lexer.set_result_symbol(anon_sym_NS_SWIFT_UNAVAILABLE); lexer.mark_end();
                return result;
            }
            1308 => {
                if lookahead == 70 { state = 1326; lexer.advance(false); continue; }
                return result;
            }
            1309 => {
                if lookahead == 79 { state = 1327; lexer.advance(false); continue; }
                return result;
            }
            1310 => {
                if lookahead == 69 { state = 1328; lexer.advance(false); continue; }
                return result;
            }
            1311 => {
                if lookahead == 84 { state = 1329; lexer.advance(false); continue; }
                return result;
            }
            1312 => {
                if lookahead == 103 { state = 1330; lexer.advance(false); continue; }
                return result;
            }
            1313 => {
                if lookahead == 95 { state = 1331; lexer.advance(false); continue; }
                return result;
            }
            1314 => {
                if lookahead == 111 { state = 1332; lexer.advance(false); continue; }
                return result;
            }
            1315 => {
                if lookahead == 95 { state = 1333; lexer.advance(false); continue; }
                return result;
            }
            1316 => {
                if lookahead == 69 { state = 1334; lexer.advance(false); continue; }
                return result;
            }
            1317 => {
                if lookahead == 85 { state = 1335; lexer.advance(false); continue; }
                return result;
            }
            1318 => {
                if lookahead == 73 { state = 1336; lexer.advance(false); continue; }
                return result;
            }
            1319 => {
                if lookahead == 95 { state = 1337; lexer.advance(false); continue; }
                return result;
            }
            1320 => {
                if lookahead == 83 { state = 1338; lexer.advance(false); continue; }
                return result;
            }
            1321 => {
                if lookahead == 79 { state = 1339; lexer.advance(false); continue; }
                return result;
            }
            1322 => {
                result = true; lexer.set_result_symbol(anon_sym_NS_ENUM_AVAILABLE_IOS); lexer.mark_end();
                return result;
            }
            1323 => {
                if lookahead == 83 { state = 1340; lexer.advance(false); continue; }
                return result;
            }
            1324 => {
                if lookahead == 66 { state = 1341; lexer.advance(false); continue; }
                return result;
            }
            1325 => {
                if lookahead == 78 { state = 1342; lexer.advance(false); continue; }
                return result;
            }
            1326 => {
                if lookahead == 95 { state = 1343; lexer.advance(false); continue; }
                return result;
            }
            1327 => {
                if lookahead == 82 { state = 1344; lexer.advance(false); continue; }
                return result;
            }
            1328 => {
                result = true; lexer.set_result_symbol(anon_sym_UNAVAILABLE_ATTRIBUTE); lexer.mark_end();
                return result;
            }
            1329 => {
                if lookahead == 73 { state = 1345; lexer.advance(false); continue; }
                return result;
            }
            1330 => {
                result = true; lexer.set_result_symbol(anon_sym___deprecated_enum_msg); lexer.mark_end();
                return result;
            }
            1331 => {
                if lookahead == 114 { state = 1346; lexer.advance(false); continue; }
                return result;
            }
            1332 => {
                if lookahead == 105 { state = 1347; lexer.advance(false); continue; }
                return result;
            }
            1333 => {
                if lookahead == 112 { state = 1348; lexer.advance(false); continue; }
                return result;
            }
            1334 => {
                if lookahead == 68 { state = 1349; lexer.advance(false); continue; }
                return result;
            }
            1335 => {
                if lookahead == 84 { state = 1350; lexer.advance(false); continue; }
                return result;
            }
            1336 => {
                if lookahead == 78 { state = 1351; lexer.advance(false); continue; }
                return result;
            }
            1337 => {
                if lookahead == 85 { state = 1352; lexer.advance(false); continue; }
                return result;
            }
            1338 => {
                result = true; lexer.set_result_symbol(anon_sym_NS_CLASS_AVAILABLE_IOS); lexer.mark_end();
                return result;
            }
            1339 => {
                if lookahead == 83 { state = 1353; lexer.advance(false); continue; }
                return result;
            }
            1340 => {
                result = true; lexer.set_result_symbol(anon_sym_NS_ENUM_DEPRECATED_IOS); lexer.mark_end();
                return result;
            }
            1341 => {
                if lookahead == 76 { state = 1354; lexer.advance(false); continue; }
                return result;
            }
            1342 => {
                if lookahead == 65 { state = 1355; lexer.advance(false); continue; }
                return result;
            }
            1343 => {
                if lookahead == 83 { state = 1356; lexer.advance(false); continue; }
                return result;
            }
            1344 => {
                result = true; lexer.set_result_symbol(anon_sym_UI_APPEARANCE_SELECTOR); lexer.mark_end();
                return result;
            }
            1345 => {
                if lookahead == 78 { state = 1357; lexer.advance(false); continue; }
                return result;
            }
            1346 => {
                if lookahead == 111 { state = 1358; lexer.advance(false); continue; }
                return result;
            }
            1347 => {
                if lookahead == 110 { state = 1359; lexer.advance(false); continue; }
                return result;
            }
            1348 => {
                if lookahead == 111 { state = 1360; lexer.advance(false); continue; }
                return result;
            }
            1349 => {
                result = true; lexer.set_result_symbol(anon_sym_CF_RETURNS_NOT_RETAINED); lexer.mark_end();
                return result;
            }
            1350 => {
                if lookahead == 69 { state = 1361; lexer.advance(false); continue; }
                return result;
            }
            1351 => {
                if lookahead == 69 { state = 1362; lexer.advance(false); continue; }
                return result;
            }
            1352 => {
                if lookahead == 78 { state = 1363; lexer.advance(false); continue; }
                return result;
            }
            1353 => {
                result = true; lexer.set_result_symbol(anon_sym_NS_CLASS_DEPRECATED_IOS); lexer.mark_end();
                return result;
            }
            1354 => {
                if lookahead == 69 { state = 1364; lexer.advance(false); continue; }
                return result;
            }
            1355 => {
                if lookahead == 84 { state = 1365; lexer.advance(false); continue; }
                return result;
            }
            1356 => {
                if lookahead == 67 { state = 1366; lexer.advance(false); continue; }
                return result;
            }
            1357 => {
                if lookahead == 71 { state = 1367; lexer.advance(false); continue; }
                return result;
            }
            1358 => {
                result = true; lexer.set_result_symbol(anon_sym___ptrauth_objc_class_ro); lexer.mark_end();
                return result;
            }
            1359 => {
                if lookahead == 116 { state = 1368; lexer.advance(false); continue; }
                return result;
            }
            1360 => {
                if lookahead == 105 { state = 1369; lexer.advance(false); continue; }
                return result;
            }
            1361 => {
                result = true; lexer.set_result_symbol(anon_sym_DEPRECATED_MSG_ATTRIBUTE); lexer.mark_end();
                return result;
            }
            1362 => {
                result = true; lexer.set_result_symbol(anon_sym_FOUNDATION_STATIC_INLINE); lexer.mark_end();
                return result;
            }
            1363 => {
                if lookahead == 65 { state = 1370; lexer.advance(false); continue; }
                return result;
            }
            1364 => {
                if lookahead == 95 { state = 1371; lexer.advance(false); continue; }
                return result;
            }
            1365 => {
                if lookahead == 73 { state = 1372; lexer.advance(false); continue; }
                return result;
            }
            1366 => {
                if lookahead == 79 { state = 1373; lexer.advance(false); continue; }
                return result;
            }
            1367 => {
                result = true; lexer.set_result_symbol(anon_sym___OSX_AVAILABLE_STARTING); lexer.mark_end();
                return result;
            }
            1368 => {
                if lookahead == 101 { state = 1374; lexer.advance(false); continue; }
                return result;
            }
            1369 => {
                if lookahead == 110 { state = 1375; lexer.advance(false); continue; }
                return result;
            }
            1370 => {
                if lookahead == 86 { state = 1376; lexer.advance(false); continue; }
                return result;
            }
            1371 => {
                if lookahead == 73 { state = 1377; lexer.advance(false); continue; }
                return result;
            }
            1372 => {
                if lookahead == 79 { state = 1378; lexer.advance(false); continue; }
                return result;
            }
            1373 => {
                if lookahead == 80 { state = 1379; lexer.advance(false); continue; }
                return result;
            }
            1374 => {
                if lookahead == 114 { state = 1380; lexer.advance(false); continue; }
                return result;
            }
            1375 => {
                if lookahead == 116 { state = 1381; lexer.advance(false); continue; }
                return result;
            }
            1376 => {
                if lookahead == 65 { state = 1382; lexer.advance(false); continue; }
                return result;
            }
            1377 => {
                if lookahead == 79 { state = 1383; lexer.advance(false); continue; }
                return result;
            }
            1378 => {
                if lookahead == 78 { state = 1384; lexer.advance(false); continue; }
                return result;
            }
            1379 => {
                if lookahead == 69 { state = 1385; lexer.advance(false); continue; }
                return result;
            }
            1380 => {
                result = true; lexer.set_result_symbol(anon_sym___ptrauth_objc_isa_pointer); lexer.mark_end();
                return result;
            }
            1381 => {
                if lookahead == 101 { state = 1386; lexer.advance(false); continue; }
                return result;
            }
            1382 => {
                if lookahead == 73 { state = 1387; lexer.advance(false); continue; }
                return result;
            }
            1383 => {
                if lookahead == 83 { state = 1388; lexer.advance(false); continue; }
                return result;
            }
            1384 => {
                result = true; lexer.set_result_symbol(anon_sym_NS_REQUIRES_NIL_TERMINATION); lexer.mark_end();
                return result;
            }
            1385 => {
                result = true; lexer.set_result_symbol(anon_sym_NS_VALID_UNTIL_END_OF_SCOPE); lexer.mark_end();
                return result;
            }
            1386 => {
                if lookahead == 114 { state = 1389; lexer.advance(false); continue; }
                return result;
            }
            1387 => {
                if lookahead == 76 { state = 1390; lexer.advance(false); continue; }
                return result;
            }
            1388 => {
                result = true; lexer.set_result_symbol(anon_sym_NS_EXTENSION_UNAVAILABLE_IOS); lexer.mark_end();
                return result;
            }
            1389 => {
                result = true; lexer.set_result_symbol(anon_sym___ptrauth_objc_super_pointer); lexer.mark_end();
                return result;
            }
            1390 => {
                if lookahead == 65 { state = 1391; lexer.advance(false); continue; }
                return result;
            }
            1391 => {
                if lookahead == 66 { state = 1392; lexer.advance(false); continue; }
                return result;
            }
            1392 => {
                if lookahead == 76 { state = 1393; lexer.advance(false); continue; }
                return result;
            }
            1393 => {
                if lookahead == 69 { state = 1394; lexer.advance(false); continue; }
                return result;
            }
            1394 => {
                result = true; lexer.set_result_symbol(anon_sym_NS_AUTOMATED_REFCOUNT_UNAVAILABLE); lexer.mark_end();
                return result;
            }
            _ => return false,
        }
    }
}
