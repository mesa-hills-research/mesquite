//! The `perl` grammar's lexer: `ts_lex` and `ts_lex_keywords`, transliterated from
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

const anon_sym_ADJUST: Symbol = 10;
const anon_sym_AMP: Symbol = 63;
const anon_sym_AMP2: Symbol = 198;
const anon_sym_AMP_AMP: Symbol = 60;
const anon_sym_AMP_AMP_EQ: Symbol = 54;
const anon_sym_AMP_EQ: Symbol = 49;
const anon_sym_AT: Symbol = 14;
const anon_sym_AT_STAR: Symbol = 311;
const anon_sym_BANG: Symbol = 91;
const anon_sym_BANG_EQ: Symbol = 73;
const anon_sym_BANG_TILDE: Symbol = 71;
const anon_sym_BEGIN: Symbol = 209;
const anon_sym_BSLASH: Symbol = 96;
const anon_sym_BUILD: Symbol = 9;
const anon_sym_CARET: Symbol = 62;
const anon_sym_CARET_CARET: Symbol = 59;
const anon_sym_CARET_EQ: Symbol = 51;
const anon_sym_CHECK: Symbol = 211;
const anon_sym_COLON: Symbol = 4;
const anon_sym_COMMA: Symbol = 15;
const anon_sym_DASH: Symbol = 66;
const anon_sym_DASH_DASH: Symbol = 94;
const anon_sym_DASH_EQ: Symbol = 43;
const anon_sym_DASH_GT: Symbol = 33;
const anon_sym_DASH_GT2: Symbol = 307;
const anon_sym_DOLLAR: Symbol = 11;
const anon_sym_DOLLAR_POUND: Symbol = 103;
const anon_sym_DOLLAR_STAR: Symbol = 310;
const anon_sym_DOT: Symbol = 67;
const anon_sym_DOT_DOT: Symbol = 55;
const anon_sym_DOT_DOT_DOT: Symbol = 27;
const anon_sym_DOT_EQ: Symbol = 44;
const anon_sym_END: Symbol = 213;
const anon_sym_EOT: Symbol = 303;
const anon_sym_EQ: Symbol = 8;
const anon_sym_EQ_EQ: Symbol = 72;
const anon_sym_EQ_EQ_EQ: Symbol = 75;
const anon_sym_EQ_GT: Symbol = 201;
const anon_sym_EQ_TILDE: Symbol = 70;
const anon_sym_GT: Symbol = 38;
const anon_sym_GT_EQ: Symbol = 84;
const anon_sym_GT_GT: Symbol = 40;
const anon_sym_GT_GT2: Symbol = 64;
const anon_sym_GT_GT_EQ: Symbol = 53;
const anon_sym_INIT: Symbol = 210;
const anon_sym_LBRACE: Symbol = 2;
const anon_sym_LBRACE2: Symbol = 309;
const anon_sym_LBRACK: Symbol = 35;
const anon_sym_LBRACK2: Symbol = 308;
const anon_sym_LPAREN: Symbol = 22;
const anon_sym_LT: Symbol = 82;
const anon_sym_LT_EQ: Symbol = 83;
const anon_sym_LT_EQ_GT: Symbol = 79;
const anon_sym_LT_LT: Symbol = 39;
const anon_sym_LT_LT_EQ: Symbol = 52;
const anon_sym_PERCENT: Symbol = 37;
const anon_sym_PERCENT2: Symbol = 197;
const anon_sym_PERCENT_EQ: Symbol = 47;
const anon_sym_PIPE: Symbol = 61;
const anon_sym_PIPE_EQ: Symbol = 50;
const anon_sym_PIPE_PIPE: Symbol = 57;
const anon_sym_PIPE_PIPE_EQ: Symbol = 12;
const anon_sym_PLUS: Symbol = 65;
const anon_sym_PLUS_EQ: Symbol = 42;
const anon_sym_PLUS_PLUS: Symbol = 93;
const anon_sym_QMARK: Symbol = 95;
const anon_sym_RBRACE: Symbol = 3;
const anon_sym_RBRACK: Symbol = 36;
const anon_sym_RPAREN: Symbol = 16;
const anon_sym_SEMI: Symbol = 5;
const anon_sym_SLASH: Symbol = 68;
const anon_sym_SLASH_EQ: Symbol = 46;
const anon_sym_SLASH_SLASH: Symbol = 58;
const anon_sym_SLASH_SLASH_EQ: Symbol = 13;
const anon_sym_STAR: Symbol = 34;
const anon_sym_STAR2: Symbol = 199;
const anon_sym_STAR_EQ: Symbol = 45;
const anon_sym_STAR_STAR: Symbol = 56;
const anon_sym_STAR_STAR_EQ: Symbol = 41;
const anon_sym_STDERR: Symbol = 113;
const anon_sym_STDIN: Symbol = 111;
const anon_sym_STDOUT: Symbol = 112;
const anon_sym_TILDE: Symbol = 90;
const anon_sym_TILDE_TILDE: Symbol = 81;
const anon_sym_UNITCHECK: Symbol = 212;
const anon_sym___DATA__: Symbol = 300;
const anon_sym___END__: Symbol = 301;
const anon_sym___FILE__: Symbol = 214;
const anon_sym___LINE__: Symbol = 215;
const anon_sym___PACKAGE__: Symbol = 216;
const anon_sym___SUB__: Symbol = 217;
const anon_sym_abs: Symbol = 225;
const anon_sym_accept: Symbol = 119;
const anon_sym_alarm: Symbol = 226;
const anon_sym_and: Symbol = 30;
const anon_sym_atan2: Symbol = 120;
const anon_sym_await: Symbol = 99;
const anon_sym_bind: Symbol = 121;
const anon_sym_binmode: Symbol = 122;
const anon_sym_bless: Symbol = 123;
const anon_sym_break: Symbol = 218;
const anon_sym_caller: Symbol = 231;
const anon_sym_catch: Symbol = 24;
const anon_sym_chdir: Symbol = 228;
const anon_sym_chmod: Symbol = 125;
const anon_sym_chomp: Symbol = 232;
const anon_sym_chop: Symbol = 227;
const anon_sym_chown: Symbol = 126;
const anon_sym_chr: Symbol = 233;
const anon_sym_chroot: Symbol = 235;
const anon_sym_close: Symbol = 229;
const anon_sym_closedir: Symbol = 230;
const anon_sym_cmp: Symbol = 80;
const anon_sym_connect: Symbol = 127;
const anon_sym_continue: Symbol = 23;
const anon_sym_cos: Symbol = 234;
const anon_sym_crypt: Symbol = 124;
const anon_sym_dbmclose: Symbol = 238;
const anon_sym_dbmopen: Symbol = 129;
const anon_sym_defer: Symbol = 26;
const anon_sym_defined: Symbol = 236;
const anon_sym_delete: Symbol = 237;
const anon_sym_die: Symbol = 128;
const anon_sym_do: Symbol = 97;
const anon_sym_dynamically: Symbol = 102;
const anon_sym_each: Symbol = 243;
const anon_sym_else: Symbol = 28;
const anon_sym_elsif: Symbol = 29;
const anon_sym_eof: Symbol = 241;
const anon_sym_eq: Symbol = 74;
const anon_sym_eqr: Symbol = 77;
const anon_sym_equ: Symbol = 76;
const anon_sym_eval: Symbol = 98;
const anon_sym_exec: Symbol = 117;
const anon_sym_exists: Symbol = 239;
const anon_sym_exit: Symbol = 240;
const anon_sym_exp: Symbol = 242;
const anon_sym_extended: Symbol = 20;
const anon_sym_false: Symbol = 335;
const anon_sym_fc: Symbol = 244;
const anon_sym_fcntl: Symbol = 130;
const anon_sym_field: Symbol = 100;
const anon_sym_fileno: Symbol = 245;
const anon_sym_finally: Symbol = 25;
const anon_sym_flock: Symbol = 131;
const anon_sym_for: Symbol = 204;
const anon_sym_foreach: Symbol = 205;
const anon_sym_fork: Symbol = 219;
const anon_sym_format: Symbol = 7;
const anon_sym_formline: Symbol = 195;
const anon_sym_ge: Symbol = 87;
const anon_sym_getc: Symbol = 247;
const anon_sym_getgrgid: Symbol = 256;
const anon_sym_getgrnam: Symbol = 255;
const anon_sym_gethostbyaddr: Symbol = 134;
const anon_sym_getnetbyaddr: Symbol = 135;
const anon_sym_getnetbyname: Symbol = 253;
const anon_sym_getpeername: Symbol = 252;
const anon_sym_getpgrp: Symbol = 248;
const anon_sym_getppid: Symbol = 220;
const anon_sym_getpriority: Symbol = 132;
const anon_sym_getprotobyname: Symbol = 249;
const anon_sym_getprotobynumber: Symbol = 133;
const anon_sym_getpwname: Symbol = 250;
const anon_sym_getpwuid: Symbol = 251;
const anon_sym_getservbyname: Symbol = 136;
const anon_sym_getservbyport: Symbol = 137;
const anon_sym_getsockname: Symbol = 254;
const anon_sym_getsockopt: Symbol = 138;
const anon_sym_glob: Symbol = 139;
const anon_sym_gmtime: Symbol = 246;
const anon_sym_goto: Symbol = 108;
const anon_sym_grep: Symbol = 106;
const anon_sym_gt: Symbol = 88;
const anon_sym_hex: Symbol = 257;
const anon_sym_if: Symbol = 327;
const anon_sym_index: Symbol = 140;
const anon_sym_int: Symbol = 258;
const anon_sym_ioctl: Symbol = 141;
const anon_sym_isa: Symbol = 89;
const anon_sym_join: Symbol = 142;
const anon_sym_keys: Symbol = 259;
const anon_sym_kill: Symbol = 143;
const anon_sym_last: Symbol = 206;
const anon_sym_lc: Symbol = 260;
const anon_sym_lcfirst: Symbol = 261;
const anon_sym_le: Symbol = 86;
const anon_sym_length: Symbol = 262;
const anon_sym_link: Symbol = 144;
const anon_sym_listen: Symbol = 145;
const anon_sym_local: Symbol = 101;
const anon_sym_localtime: Symbol = 263;
const anon_sym_lock: Symbol = 265;
const anon_sym_log: Symbol = 264;
const anon_sym_lstat: Symbol = 266;
const anon_sym_lt: Symbol = 85;
const anon_sym_m: Symbol = 317;
const anon_sym_map: Symbol = 105;
const anon_sym_mkdir: Symbol = 146;
const anon_sym_msgctl: Symbol = 147;
const anon_sym_msgget: Symbol = 148;
const anon_sym_msgrcv: Symbol = 149;
const anon_sym_msgsend: Symbol = 150;
const anon_sym_my: Symbol = 17;
const anon_sym_ne: Symbol = 78;
const anon_sym_next: Symbol = 207;
const anon_sym_no: Symbol = 203;
const anon_sym_not: Symbol = 92;
const anon_sym_oct: Symbol = 267;
const anon_sym_open: Symbol = 196;
const anon_sym_opendir: Symbol = 151;
const anon_sym_or: Symbol = 31;
const anon_sym_ord: Symbol = 268;
const anon_sym_our: Symbol = 19;
const anon_sym_pack: Symbol = 153;
const anon_sym_package: Symbol = 6;
const anon_sym_pipe: Symbol = 154;
const anon_sym_pop: Symbol = 270;
const anon_sym_pos: Symbol = 271;
const anon_sym_print: Symbol = 114;
const anon_sym_printf: Symbol = 115;
const anon_sym_prototype: Symbol = 269;
const anon_sym_push: Symbol = 152;
const anon_sym_q: Symbol = 305;
const anon_sym_qq: Symbol = 306;
const anon_sym_qr: Symbol = 316;
const anon_sym_quotemeta: Symbol = 272;
const anon_sym_qw: Symbol = 314;
const anon_sym_qx: Symbol = 315;
const anon_sym_rand: Symbol = 274;
const anon_sym_read: Symbol = 157;
const anon_sym_readdir: Symbol = 276;
const anon_sym_readline: Symbol = 277;
const anon_sym_readlink: Symbol = 280;
const anon_sym_readpipe: Symbol = 278;
const anon_sym_recv: Symbol = 158;
const anon_sym_redo: Symbol = 208;
const anon_sym_ref: Symbol = 281;
const anon_sym_rename: Symbol = 155;
const anon_sym_require: Symbol = 104;
const anon_sym_reset: Symbol = 273;
const anon_sym_return: Symbol = 109;
const anon_sym_reverse: Symbol = 159;
const anon_sym_rewinddir: Symbol = 279;
const anon_sym_rindex: Symbol = 156;
const anon_sym_rmdir: Symbol = 275;
const anon_sym_s: Symbol = 318;
const anon_sym_say: Symbol = 116;
const anon_sym_scalar: Symbol = 282;
const anon_sym_seek: Symbol = 161;
const anon_sym_seekdir: Symbol = 168;
const anon_sym_select: Symbol = 160;
const anon_sym_semctl: Symbol = 162;
const anon_sym_semget: Symbol = 163;
const anon_sym_semop: Symbol = 164;
const anon_sym_send: Symbol = 165;
const anon_sym_setpgrp: Symbol = 166;
const anon_sym_setpriority: Symbol = 167;
const anon_sym_setsockopt: Symbol = 169;
const anon_sym_shift: Symbol = 283;
const anon_sym_shmctl: Symbol = 170;
const anon_sym_shmread: Symbol = 171;
const anon_sym_shmwrite: Symbol = 172;
const anon_sym_shutdown: Symbol = 173;
const anon_sym_sin: Symbol = 284;
const anon_sym_sleep: Symbol = 285;
const anon_sym_socket: Symbol = 174;
const anon_sym_socketpair: Symbol = 175;
const anon_sym_sort: Symbol = 107;
const anon_sym_splice: Symbol = 178;
const anon_sym_split: Symbol = 176;
const anon_sym_sprintf: Symbol = 177;
const anon_sym_sqrt: Symbol = 286;
const anon_sym_srand: Symbol = 287;
const anon_sym_stat: Symbol = 288;
const anon_sym_state: Symbol = 18;
const anon_sym_study: Symbol = 289;
const anon_sym_sub: Symbol = 21;
const anon_sym_substr: Symbol = 179;
const anon_sym_symlink: Symbol = 180;
const anon_sym_syscall: Symbol = 181;
const anon_sym_sysopen: Symbol = 182;
const anon_sym_sysread: Symbol = 184;
const anon_sym_sysseek: Symbol = 183;
const anon_sym_system: Symbol = 118;
const anon_sym_syswrite: Symbol = 185;
const anon_sym_tell: Symbol = 290;
const anon_sym_telldir: Symbol = 291;
const anon_sym_tie: Symbol = 186;
const anon_sym_tied: Symbol = 292;
const anon_sym_time: Symbol = 221;
const anon_sym_times: Symbol = 222;
const anon_sym_tr: Symbol = 324;
const anon_sym_true: Symbol = 334;
const anon_sym_truncate: Symbol = 187;
const anon_sym_uc: Symbol = 293;
const anon_sym_ucfirst: Symbol = 294;
const anon_sym_umask: Symbol = 296;
const anon_sym_undef: Symbol = 110;
const anon_sym_unless: Symbol = 328;
const anon_sym_unlink: Symbol = 188;
const anon_sym_unpack: Symbol = 189;
const anon_sym_unshift: Symbol = 191;
const anon_sym_untie: Symbol = 295;
const anon_sym_until: Symbol = 330;
const anon_sym_use: Symbol = 202;
const anon_sym_utime: Symbol = 190;
const anon_sym_values: Symbol = 297;
const anon_sym_vec: Symbol = 192;
const anon_sym_wait: Symbol = 223;
const anon_sym_waitpid: Symbol = 194;
const anon_sym_wantarray: Symbol = 224;
const anon_sym_warn: Symbol = 193;
const anon_sym_while: Symbol = 329;
const anon_sym_write: Symbol = 298;
const anon_sym_x: Symbol = 69;
const anon_sym_x_EQ: Symbol = 48;
const anon_sym_xor: Symbol = 32;
const anon_sym_y: Symbol = 325;
const aux_sym___DATA___token1: Symbol = 302;
const aux_sym__bareword_token1: Symbol = 332;
const aux_sym__interpolated_transliteration_content_token1: Symbol = 323;
const aux_sym__interpolation_fallbacks_token1: Symbol = 312;
const aux_sym__interpolation_fallbacks_token2: Symbol = 313;
const aux_sym__literal_token1: Symbol = 304;
const aux_sym__var_indirob_autoquote_token1: Symbol = 200;
const sym__identifier: Symbol = 1;
const sym__special_var_name: Symbol = 331;
const sym_comment: Symbol = 299;
const sym_match_regexp_modifiers: Symbol = 320;
const sym_number: Symbol = 333;
const sym_quoted_regexp_modifiers: Symbol = 319;
const sym_substitution_regexp_modifiers: Symbol = 321;
const sym_transliteration_modifiers: Symbol = 322;
const sym_version: Symbol = 326;
const ts_builtin_sym_end: Symbol = 0;

#[rustfmt::skip]
static aux_sym__bareword_token1_character_set_1: [CharacterRange; 686] = [
    CharacterRange::new(58, 58), CharacterRange::new(65, 90), CharacterRange::new(95, 95), CharacterRange::new(97, 122), CharacterRange::new(170, 170), CharacterRange::new(181, 181),
    CharacterRange::new(186, 186), CharacterRange::new(192, 214), CharacterRange::new(216, 246), CharacterRange::new(248, 705), CharacterRange::new(710, 721), CharacterRange::new(736, 740),
    CharacterRange::new(748, 748), CharacterRange::new(750, 750), CharacterRange::new(880, 884), CharacterRange::new(886, 887), CharacterRange::new(891, 893), CharacterRange::new(895, 895),
    CharacterRange::new(902, 902), CharacterRange::new(904, 906), CharacterRange::new(908, 908), CharacterRange::new(910, 929), CharacterRange::new(931, 1013), CharacterRange::new(1015, 1153),
    CharacterRange::new(1162, 1327), CharacterRange::new(1329, 1366), CharacterRange::new(1369, 1369), CharacterRange::new(1376, 1416), CharacterRange::new(1488, 1514), CharacterRange::new(1519, 1522),
    CharacterRange::new(1568, 1610), CharacterRange::new(1646, 1647), CharacterRange::new(1649, 1747), CharacterRange::new(1749, 1749), CharacterRange::new(1765, 1766), CharacterRange::new(1774, 1775),
    CharacterRange::new(1786, 1788), CharacterRange::new(1791, 1791), CharacterRange::new(1808, 1808), CharacterRange::new(1810, 1839), CharacterRange::new(1869, 1957), CharacterRange::new(1969, 1969),
    CharacterRange::new(1994, 2026), CharacterRange::new(2036, 2037), CharacterRange::new(2042, 2042), CharacterRange::new(2048, 2069), CharacterRange::new(2074, 2074), CharacterRange::new(2084, 2084),
    CharacterRange::new(2088, 2088), CharacterRange::new(2112, 2136), CharacterRange::new(2144, 2154), CharacterRange::new(2160, 2183), CharacterRange::new(2185, 2190), CharacterRange::new(2208, 2249),
    CharacterRange::new(2308, 2361), CharacterRange::new(2365, 2365), CharacterRange::new(2384, 2384), CharacterRange::new(2392, 2401), CharacterRange::new(2417, 2432), CharacterRange::new(2437, 2444),
    CharacterRange::new(2447, 2448), CharacterRange::new(2451, 2472), CharacterRange::new(2474, 2480), CharacterRange::new(2482, 2482), CharacterRange::new(2486, 2489), CharacterRange::new(2493, 2493),
    CharacterRange::new(2510, 2510), CharacterRange::new(2524, 2525), CharacterRange::new(2527, 2529), CharacterRange::new(2544, 2545), CharacterRange::new(2556, 2556), CharacterRange::new(2565, 2570),
    CharacterRange::new(2575, 2576), CharacterRange::new(2579, 2600), CharacterRange::new(2602, 2608), CharacterRange::new(2610, 2611), CharacterRange::new(2613, 2614), CharacterRange::new(2616, 2617),
    CharacterRange::new(2649, 2652), CharacterRange::new(2654, 2654), CharacterRange::new(2674, 2676), CharacterRange::new(2693, 2701), CharacterRange::new(2703, 2705), CharacterRange::new(2707, 2728),
    CharacterRange::new(2730, 2736), CharacterRange::new(2738, 2739), CharacterRange::new(2741, 2745), CharacterRange::new(2749, 2749), CharacterRange::new(2768, 2768), CharacterRange::new(2784, 2785),
    CharacterRange::new(2809, 2809), CharacterRange::new(2821, 2828), CharacterRange::new(2831, 2832), CharacterRange::new(2835, 2856), CharacterRange::new(2858, 2864), CharacterRange::new(2866, 2867),
    CharacterRange::new(2869, 2873), CharacterRange::new(2877, 2877), CharacterRange::new(2908, 2909), CharacterRange::new(2911, 2913), CharacterRange::new(2929, 2929), CharacterRange::new(2947, 2947),
    CharacterRange::new(2949, 2954), CharacterRange::new(2958, 2960), CharacterRange::new(2962, 2965), CharacterRange::new(2969, 2970), CharacterRange::new(2972, 2972), CharacterRange::new(2974, 2975),
    CharacterRange::new(2979, 2980), CharacterRange::new(2984, 2986), CharacterRange::new(2990, 3001), CharacterRange::new(3024, 3024), CharacterRange::new(3077, 3084), CharacterRange::new(3086, 3088),
    CharacterRange::new(3090, 3112), CharacterRange::new(3114, 3129), CharacterRange::new(3133, 3133), CharacterRange::new(3160, 3162), CharacterRange::new(3165, 3165), CharacterRange::new(3168, 3169),
    CharacterRange::new(3200, 3200), CharacterRange::new(3205, 3212), CharacterRange::new(3214, 3216), CharacterRange::new(3218, 3240), CharacterRange::new(3242, 3251), CharacterRange::new(3253, 3257),
    CharacterRange::new(3261, 3261), CharacterRange::new(3293, 3294), CharacterRange::new(3296, 3297), CharacterRange::new(3313, 3314), CharacterRange::new(3332, 3340), CharacterRange::new(3342, 3344),
    CharacterRange::new(3346, 3386), CharacterRange::new(3389, 3389), CharacterRange::new(3406, 3406), CharacterRange::new(3412, 3414), CharacterRange::new(3423, 3425), CharacterRange::new(3450, 3455),
    CharacterRange::new(3461, 3478), CharacterRange::new(3482, 3505), CharacterRange::new(3507, 3515), CharacterRange::new(3517, 3517), CharacterRange::new(3520, 3526), CharacterRange::new(3585, 3632),
    CharacterRange::new(3634, 3634), CharacterRange::new(3648, 3654), CharacterRange::new(3713, 3714), CharacterRange::new(3716, 3716), CharacterRange::new(3718, 3722), CharacterRange::new(3724, 3747),
    CharacterRange::new(3749, 3749), CharacterRange::new(3751, 3760), CharacterRange::new(3762, 3762), CharacterRange::new(3773, 3773), CharacterRange::new(3776, 3780), CharacterRange::new(3782, 3782),
    CharacterRange::new(3804, 3807), CharacterRange::new(3840, 3840), CharacterRange::new(3904, 3911), CharacterRange::new(3913, 3948), CharacterRange::new(3976, 3980), CharacterRange::new(4096, 4138),
    CharacterRange::new(4159, 4159), CharacterRange::new(4176, 4181), CharacterRange::new(4186, 4189), CharacterRange::new(4193, 4193), CharacterRange::new(4197, 4198), CharacterRange::new(4206, 4208),
    CharacterRange::new(4213, 4225), CharacterRange::new(4238, 4238), CharacterRange::new(4256, 4293), CharacterRange::new(4295, 4295), CharacterRange::new(4301, 4301), CharacterRange::new(4304, 4346),
    CharacterRange::new(4348, 4680), CharacterRange::new(4682, 4685), CharacterRange::new(4688, 4694), CharacterRange::new(4696, 4696), CharacterRange::new(4698, 4701), CharacterRange::new(4704, 4744),
    CharacterRange::new(4746, 4749), CharacterRange::new(4752, 4784), CharacterRange::new(4786, 4789), CharacterRange::new(4792, 4798), CharacterRange::new(4800, 4800), CharacterRange::new(4802, 4805),
    CharacterRange::new(4808, 4822), CharacterRange::new(4824, 4880), CharacterRange::new(4882, 4885), CharacterRange::new(4888, 4954), CharacterRange::new(4992, 5007), CharacterRange::new(5024, 5109),
    CharacterRange::new(5112, 5117), CharacterRange::new(5121, 5740), CharacterRange::new(5743, 5759), CharacterRange::new(5761, 5786), CharacterRange::new(5792, 5866), CharacterRange::new(5870, 5880),
    CharacterRange::new(5888, 5905), CharacterRange::new(5919, 5937), CharacterRange::new(5952, 5969), CharacterRange::new(5984, 5996), CharacterRange::new(5998, 6000), CharacterRange::new(6016, 6067),
    CharacterRange::new(6103, 6103), CharacterRange::new(6108, 6108), CharacterRange::new(6176, 6264), CharacterRange::new(6272, 6312), CharacterRange::new(6314, 6314), CharacterRange::new(6320, 6389),
    CharacterRange::new(6400, 6430), CharacterRange::new(6480, 6509), CharacterRange::new(6512, 6516), CharacterRange::new(6528, 6571), CharacterRange::new(6576, 6601), CharacterRange::new(6656, 6678),
    CharacterRange::new(6688, 6740), CharacterRange::new(6823, 6823), CharacterRange::new(6917, 6963), CharacterRange::new(6981, 6988), CharacterRange::new(7043, 7072), CharacterRange::new(7086, 7087),
    CharacterRange::new(7098, 7141), CharacterRange::new(7168, 7203), CharacterRange::new(7245, 7247), CharacterRange::new(7258, 7293), CharacterRange::new(7296, 7306), CharacterRange::new(7312, 7354),
    CharacterRange::new(7357, 7359), CharacterRange::new(7401, 7404), CharacterRange::new(7406, 7411), CharacterRange::new(7413, 7414), CharacterRange::new(7418, 7418), CharacterRange::new(7424, 7615),
    CharacterRange::new(7680, 7957), CharacterRange::new(7960, 7965), CharacterRange::new(7968, 8005), CharacterRange::new(8008, 8013), CharacterRange::new(8016, 8023), CharacterRange::new(8025, 8025),
    CharacterRange::new(8027, 8027), CharacterRange::new(8029, 8029), CharacterRange::new(8031, 8061), CharacterRange::new(8064, 8116), CharacterRange::new(8118, 8124), CharacterRange::new(8126, 8126),
    CharacterRange::new(8130, 8132), CharacterRange::new(8134, 8140), CharacterRange::new(8144, 8147), CharacterRange::new(8150, 8155), CharacterRange::new(8160, 8172), CharacterRange::new(8178, 8180),
    CharacterRange::new(8182, 8188), CharacterRange::new(8305, 8305), CharacterRange::new(8319, 8319), CharacterRange::new(8336, 8348), CharacterRange::new(8450, 8450), CharacterRange::new(8455, 8455),
    CharacterRange::new(8458, 8467), CharacterRange::new(8469, 8469), CharacterRange::new(8472, 8477), CharacterRange::new(8484, 8484), CharacterRange::new(8486, 8486), CharacterRange::new(8488, 8488),
    CharacterRange::new(8490, 8505), CharacterRange::new(8508, 8511), CharacterRange::new(8517, 8521), CharacterRange::new(8526, 8526), CharacterRange::new(8544, 8584), CharacterRange::new(11264, 11492),
    CharacterRange::new(11499, 11502), CharacterRange::new(11506, 11507), CharacterRange::new(11520, 11557), CharacterRange::new(11559, 11559), CharacterRange::new(11565, 11565), CharacterRange::new(11568, 11623),
    CharacterRange::new(11631, 11631), CharacterRange::new(11648, 11670), CharacterRange::new(11680, 11686), CharacterRange::new(11688, 11694), CharacterRange::new(11696, 11702), CharacterRange::new(11704, 11710),
    CharacterRange::new(11712, 11718), CharacterRange::new(11720, 11726), CharacterRange::new(11728, 11734), CharacterRange::new(11736, 11742), CharacterRange::new(12293, 12295), CharacterRange::new(12321, 12329),
    CharacterRange::new(12337, 12341), CharacterRange::new(12344, 12348), CharacterRange::new(12353, 12438), CharacterRange::new(12445, 12447), CharacterRange::new(12449, 12538), CharacterRange::new(12540, 12543),
    CharacterRange::new(12549, 12591), CharacterRange::new(12593, 12686), CharacterRange::new(12704, 12735), CharacterRange::new(12784, 12799), CharacterRange::new(13312, 19903), CharacterRange::new(19968, 42124),
    CharacterRange::new(42192, 42237), CharacterRange::new(42240, 42508), CharacterRange::new(42512, 42527), CharacterRange::new(42538, 42539), CharacterRange::new(42560, 42606), CharacterRange::new(42623, 42653),
    CharacterRange::new(42656, 42735), CharacterRange::new(42775, 42783), CharacterRange::new(42786, 42888), CharacterRange::new(42891, 42957), CharacterRange::new(42960, 42961), CharacterRange::new(42963, 42963),
    CharacterRange::new(42965, 42972), CharacterRange::new(42994, 43009), CharacterRange::new(43011, 43013), CharacterRange::new(43015, 43018), CharacterRange::new(43020, 43042), CharacterRange::new(43072, 43123),
    CharacterRange::new(43138, 43187), CharacterRange::new(43250, 43255), CharacterRange::new(43259, 43259), CharacterRange::new(43261, 43262), CharacterRange::new(43274, 43301), CharacterRange::new(43312, 43334),
    CharacterRange::new(43360, 43388), CharacterRange::new(43396, 43442), CharacterRange::new(43471, 43471), CharacterRange::new(43488, 43492), CharacterRange::new(43494, 43503), CharacterRange::new(43514, 43518),
    CharacterRange::new(43520, 43560), CharacterRange::new(43584, 43586), CharacterRange::new(43588, 43595), CharacterRange::new(43616, 43638), CharacterRange::new(43642, 43642), CharacterRange::new(43646, 43695),
    CharacterRange::new(43697, 43697), CharacterRange::new(43701, 43702), CharacterRange::new(43705, 43709), CharacterRange::new(43712, 43712), CharacterRange::new(43714, 43714), CharacterRange::new(43739, 43741),
    CharacterRange::new(43744, 43754), CharacterRange::new(43762, 43764), CharacterRange::new(43777, 43782), CharacterRange::new(43785, 43790), CharacterRange::new(43793, 43798), CharacterRange::new(43808, 43814),
    CharacterRange::new(43816, 43822), CharacterRange::new(43824, 43866), CharacterRange::new(43868, 43881), CharacterRange::new(43888, 44002), CharacterRange::new(44032, 55203), CharacterRange::new(55216, 55238),
    CharacterRange::new(55243, 55291), CharacterRange::new(63744, 64109), CharacterRange::new(64112, 64217), CharacterRange::new(64256, 64262), CharacterRange::new(64275, 64279), CharacterRange::new(64285, 64285),
    CharacterRange::new(64287, 64296), CharacterRange::new(64298, 64310), CharacterRange::new(64312, 64316), CharacterRange::new(64318, 64318), CharacterRange::new(64320, 64321), CharacterRange::new(64323, 64324),
    CharacterRange::new(64326, 64433), CharacterRange::new(64467, 64605), CharacterRange::new(64612, 64829), CharacterRange::new(64848, 64911), CharacterRange::new(64914, 64967), CharacterRange::new(65008, 65017),
    CharacterRange::new(65137, 65137), CharacterRange::new(65139, 65139), CharacterRange::new(65143, 65143), CharacterRange::new(65145, 65145), CharacterRange::new(65147, 65147), CharacterRange::new(65149, 65149),
    CharacterRange::new(65151, 65276), CharacterRange::new(65313, 65338), CharacterRange::new(65345, 65370), CharacterRange::new(65382, 65437), CharacterRange::new(65440, 65470), CharacterRange::new(65474, 65479),
    CharacterRange::new(65482, 65487), CharacterRange::new(65490, 65495), CharacterRange::new(65498, 65500), CharacterRange::new(65536, 65547), CharacterRange::new(65549, 65574), CharacterRange::new(65576, 65594),
    CharacterRange::new(65596, 65597), CharacterRange::new(65599, 65613), CharacterRange::new(65616, 65629), CharacterRange::new(65664, 65786), CharacterRange::new(65856, 65908), CharacterRange::new(66176, 66204),
    CharacterRange::new(66208, 66256), CharacterRange::new(66304, 66335), CharacterRange::new(66349, 66378), CharacterRange::new(66384, 66421), CharacterRange::new(66432, 66461), CharacterRange::new(66464, 66499),
    CharacterRange::new(66504, 66511), CharacterRange::new(66513, 66517), CharacterRange::new(66560, 66717), CharacterRange::new(66736, 66771), CharacterRange::new(66776, 66811), CharacterRange::new(66816, 66855),
    CharacterRange::new(66864, 66915), CharacterRange::new(66928, 66938), CharacterRange::new(66940, 66954), CharacterRange::new(66956, 66962), CharacterRange::new(66964, 66965), CharacterRange::new(66967, 66977),
    CharacterRange::new(66979, 66993), CharacterRange::new(66995, 67001), CharacterRange::new(67003, 67004), CharacterRange::new(67008, 67059), CharacterRange::new(67072, 67382), CharacterRange::new(67392, 67413),
    CharacterRange::new(67424, 67431), CharacterRange::new(67456, 67461), CharacterRange::new(67463, 67504), CharacterRange::new(67506, 67514), CharacterRange::new(67584, 67589), CharacterRange::new(67592, 67592),
    CharacterRange::new(67594, 67637), CharacterRange::new(67639, 67640), CharacterRange::new(67644, 67644), CharacterRange::new(67647, 67669), CharacterRange::new(67680, 67702), CharacterRange::new(67712, 67742),
    CharacterRange::new(67808, 67826), CharacterRange::new(67828, 67829), CharacterRange::new(67840, 67861), CharacterRange::new(67872, 67897), CharacterRange::new(67968, 68023), CharacterRange::new(68030, 68031),
    CharacterRange::new(68096, 68096), CharacterRange::new(68112, 68115), CharacterRange::new(68117, 68119), CharacterRange::new(68121, 68149), CharacterRange::new(68192, 68220), CharacterRange::new(68224, 68252),
    CharacterRange::new(68288, 68295), CharacterRange::new(68297, 68324), CharacterRange::new(68352, 68405), CharacterRange::new(68416, 68437), CharacterRange::new(68448, 68466), CharacterRange::new(68480, 68497),
    CharacterRange::new(68608, 68680), CharacterRange::new(68736, 68786), CharacterRange::new(68800, 68850), CharacterRange::new(68864, 68899), CharacterRange::new(68938, 68965), CharacterRange::new(68975, 68997),
    CharacterRange::new(69248, 69289), CharacterRange::new(69296, 69297), CharacterRange::new(69314, 69316), CharacterRange::new(69376, 69404), CharacterRange::new(69415, 69415), CharacterRange::new(69424, 69445),
    CharacterRange::new(69488, 69505), CharacterRange::new(69552, 69572), CharacterRange::new(69600, 69622), CharacterRange::new(69635, 69687), CharacterRange::new(69745, 69746), CharacterRange::new(69749, 69749),
    CharacterRange::new(69763, 69807), CharacterRange::new(69840, 69864), CharacterRange::new(69891, 69926), CharacterRange::new(69956, 69956), CharacterRange::new(69959, 69959), CharacterRange::new(69968, 70002),
    CharacterRange::new(70006, 70006), CharacterRange::new(70019, 70066), CharacterRange::new(70081, 70084), CharacterRange::new(70106, 70106), CharacterRange::new(70108, 70108), CharacterRange::new(70144, 70161),
    CharacterRange::new(70163, 70187), CharacterRange::new(70207, 70208), CharacterRange::new(70272, 70278), CharacterRange::new(70280, 70280), CharacterRange::new(70282, 70285), CharacterRange::new(70287, 70301),
    CharacterRange::new(70303, 70312), CharacterRange::new(70320, 70366), CharacterRange::new(70405, 70412), CharacterRange::new(70415, 70416), CharacterRange::new(70419, 70440), CharacterRange::new(70442, 70448),
    CharacterRange::new(70450, 70451), CharacterRange::new(70453, 70457), CharacterRange::new(70461, 70461), CharacterRange::new(70480, 70480), CharacterRange::new(70493, 70497), CharacterRange::new(70528, 70537),
    CharacterRange::new(70539, 70539), CharacterRange::new(70542, 70542), CharacterRange::new(70544, 70581), CharacterRange::new(70583, 70583), CharacterRange::new(70609, 70609), CharacterRange::new(70611, 70611),
    CharacterRange::new(70656, 70708), CharacterRange::new(70727, 70730), CharacterRange::new(70751, 70753), CharacterRange::new(70784, 70831), CharacterRange::new(70852, 70853), CharacterRange::new(70855, 70855),
    CharacterRange::new(71040, 71086), CharacterRange::new(71128, 71131), CharacterRange::new(71168, 71215), CharacterRange::new(71236, 71236), CharacterRange::new(71296, 71338), CharacterRange::new(71352, 71352),
    CharacterRange::new(71424, 71450), CharacterRange::new(71488, 71494), CharacterRange::new(71680, 71723), CharacterRange::new(71840, 71903), CharacterRange::new(71935, 71942), CharacterRange::new(71945, 71945),
    CharacterRange::new(71948, 71955), CharacterRange::new(71957, 71958), CharacterRange::new(71960, 71983), CharacterRange::new(71999, 71999), CharacterRange::new(72001, 72001), CharacterRange::new(72096, 72103),
    CharacterRange::new(72106, 72144), CharacterRange::new(72161, 72161), CharacterRange::new(72163, 72163), CharacterRange::new(72192, 72192), CharacterRange::new(72203, 72242), CharacterRange::new(72250, 72250),
    CharacterRange::new(72272, 72272), CharacterRange::new(72284, 72329), CharacterRange::new(72349, 72349), CharacterRange::new(72368, 72440), CharacterRange::new(72640, 72672), CharacterRange::new(72704, 72712),
    CharacterRange::new(72714, 72750), CharacterRange::new(72768, 72768), CharacterRange::new(72818, 72847), CharacterRange::new(72960, 72966), CharacterRange::new(72968, 72969), CharacterRange::new(72971, 73008),
    CharacterRange::new(73030, 73030), CharacterRange::new(73056, 73061), CharacterRange::new(73063, 73064), CharacterRange::new(73066, 73097), CharacterRange::new(73112, 73112), CharacterRange::new(73440, 73458),
    CharacterRange::new(73474, 73474), CharacterRange::new(73476, 73488), CharacterRange::new(73490, 73523), CharacterRange::new(73648, 73648), CharacterRange::new(73728, 74649), CharacterRange::new(74752, 74862),
    CharacterRange::new(74880, 75075), CharacterRange::new(77712, 77808), CharacterRange::new(77824, 78895), CharacterRange::new(78913, 78918), CharacterRange::new(78944, 82938), CharacterRange::new(82944, 83526),
    CharacterRange::new(90368, 90397), CharacterRange::new(92160, 92728), CharacterRange::new(92736, 92766), CharacterRange::new(92784, 92862), CharacterRange::new(92880, 92909), CharacterRange::new(92928, 92975),
    CharacterRange::new(92992, 92995), CharacterRange::new(93027, 93047), CharacterRange::new(93053, 93071), CharacterRange::new(93504, 93548), CharacterRange::new(93760, 93823), CharacterRange::new(93952, 94026),
    CharacterRange::new(94032, 94032), CharacterRange::new(94099, 94111), CharacterRange::new(94176, 94177), CharacterRange::new(94179, 94179), CharacterRange::new(94208, 100343), CharacterRange::new(100352, 101589),
    CharacterRange::new(101631, 101640), CharacterRange::new(110576, 110579), CharacterRange::new(110581, 110587), CharacterRange::new(110589, 110590), CharacterRange::new(110592, 110882), CharacterRange::new(110898, 110898),
    CharacterRange::new(110928, 110930), CharacterRange::new(110933, 110933), CharacterRange::new(110948, 110951), CharacterRange::new(110960, 111355), CharacterRange::new(113664, 113770), CharacterRange::new(113776, 113788),
    CharacterRange::new(113792, 113800), CharacterRange::new(113808, 113817), CharacterRange::new(119808, 119892), CharacterRange::new(119894, 119964), CharacterRange::new(119966, 119967), CharacterRange::new(119970, 119970),
    CharacterRange::new(119973, 119974), CharacterRange::new(119977, 119980), CharacterRange::new(119982, 119993), CharacterRange::new(119995, 119995), CharacterRange::new(119997, 120003), CharacterRange::new(120005, 120069),
    CharacterRange::new(120071, 120074), CharacterRange::new(120077, 120084), CharacterRange::new(120086, 120092), CharacterRange::new(120094, 120121), CharacterRange::new(120123, 120126), CharacterRange::new(120128, 120132),
    CharacterRange::new(120134, 120134), CharacterRange::new(120138, 120144), CharacterRange::new(120146, 120485), CharacterRange::new(120488, 120512), CharacterRange::new(120514, 120538), CharacterRange::new(120540, 120570),
    CharacterRange::new(120572, 120596), CharacterRange::new(120598, 120628), CharacterRange::new(120630, 120654), CharacterRange::new(120656, 120686), CharacterRange::new(120688, 120712), CharacterRange::new(120714, 120744),
    CharacterRange::new(120746, 120770), CharacterRange::new(120772, 120779), CharacterRange::new(122624, 122654), CharacterRange::new(122661, 122666), CharacterRange::new(122928, 122989), CharacterRange::new(123136, 123180),
    CharacterRange::new(123191, 123197), CharacterRange::new(123214, 123214), CharacterRange::new(123536, 123565), CharacterRange::new(123584, 123627), CharacterRange::new(124112, 124139), CharacterRange::new(124368, 124397),
    CharacterRange::new(124400, 124400), CharacterRange::new(124896, 124902), CharacterRange::new(124904, 124907), CharacterRange::new(124909, 124910), CharacterRange::new(124912, 124926), CharacterRange::new(124928, 125124),
    CharacterRange::new(125184, 125251), CharacterRange::new(125259, 125259), CharacterRange::new(126464, 126467), CharacterRange::new(126469, 126495), CharacterRange::new(126497, 126498), CharacterRange::new(126500, 126500),
    CharacterRange::new(126503, 126503), CharacterRange::new(126505, 126514), CharacterRange::new(126516, 126519), CharacterRange::new(126521, 126521), CharacterRange::new(126523, 126523), CharacterRange::new(126530, 126530),
    CharacterRange::new(126535, 126535), CharacterRange::new(126537, 126537), CharacterRange::new(126539, 126539), CharacterRange::new(126541, 126543), CharacterRange::new(126545, 126546), CharacterRange::new(126548, 126548),
    CharacterRange::new(126551, 126551), CharacterRange::new(126553, 126553), CharacterRange::new(126555, 126555), CharacterRange::new(126557, 126557), CharacterRange::new(126559, 126559), CharacterRange::new(126561, 126562),
    CharacterRange::new(126564, 126564), CharacterRange::new(126567, 126570), CharacterRange::new(126572, 126578), CharacterRange::new(126580, 126583), CharacterRange::new(126585, 126588), CharacterRange::new(126590, 126590),
    CharacterRange::new(126592, 126601), CharacterRange::new(126603, 126619), CharacterRange::new(126625, 126627), CharacterRange::new(126629, 126633), CharacterRange::new(126635, 126651), CharacterRange::new(131072, 173791),
    CharacterRange::new(173824, 177977), CharacterRange::new(177984, 178205), CharacterRange::new(178208, 183969), CharacterRange::new(183984, 191456), CharacterRange::new(191472, 192093), CharacterRange::new(194560, 195101),
    CharacterRange::new(196608, 201546), CharacterRange::new(201552, 205743),
];

#[rustfmt::skip]
static aux_sym__bareword_token1_character_set_2: [CharacterRange; 800] = [
    CharacterRange::new(48, 58), CharacterRange::new(65, 90), CharacterRange::new(95, 95), CharacterRange::new(97, 122), CharacterRange::new(170, 170), CharacterRange::new(181, 181),
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

#[rustfmt::skip]
static aux_sym__interpolation_fallbacks_token1_character_set_1: [CharacterRange; 9] = [
    CharacterRange::new(0, 35), CharacterRange::new(37, 38), CharacterRange::new(40, 42), CharacterRange::new(44, 44), CharacterRange::new(46, 47), CharacterRange::new(59, 64),
    CharacterRange::new(91, 94), CharacterRange::new(96, 96), CharacterRange::new(123, 1114111),
];

#[rustfmt::skip]
static extras_character_set_1: [CharacterRange; 10] = [
    CharacterRange::new(9, 13), CharacterRange::new(32, 32), CharacterRange::new(133, 133), CharacterRange::new(160, 160), CharacterRange::new(5760, 5760), CharacterRange::new(8192, 8202),
    CharacterRange::new(8232, 8233), CharacterRange::new(8239, 8239), CharacterRange::new(8287, 8287), CharacterRange::new(12288, 12288),
];

#[rustfmt::skip]
static sym__identifier_character_set_1: [CharacterRange; 685] = [
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
static sym__identifier_character_set_2: [CharacterRange; 800] = [
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

#[rustfmt::skip]
static sym_substitution_regexp_modifiers_character_set_1: [CharacterRange; 9] = [
    CharacterRange::new(97, 97), CharacterRange::new(99, 101), CharacterRange::new(103, 103), CharacterRange::new(105, 105), CharacterRange::new(108, 109), CharacterRange::new(111, 112),
    CharacterRange::new(114, 115), CharacterRange::new(117, 117), CharacterRange::new(120, 120),
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
                if eof { state = 81; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (4, 234), (33, 186), (35, 231), (36, 91), (37, 203), (38, 205), (40, 101), (41, 98),
                    (42, 207), (43, 153), (44, 97), (45, 156), (46, 159), (47, 160), (48, 410), (58, 85),
                    (59, 86), (60, 174), (61, 88), (62, 122), (63, 189), (64, 96), (91, 237), (92, 190),
                    (93, 118), (94, 148), (97, 257), (99, 289), (100, 252), (101, 335), (103, 308), (105, 264),
                    (108, 260), (109, 250), (110, 260), (111, 265), (112, 254), (114, 310), (115, 255), (117, 263),
                    (118, 374), (120, 260), (123, 238), (124, 147), (125, 83), (126, 184),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 411; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 77; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            1 => {
                if let Some(next) = advance_map(&[
                    (4, 234), (33, 186), (35, 231), (36, 90), (37, 203), (38, 205), (40, 101), (41, 98),
                    (42, 207), (43, 153), (44, 97), (45, 156), (46, 159), (47, 160), (48, 412), (58, 85),
                    (59, 86), (60, 174), (61, 88), (62, 122), (63, 189), (64, 95), (91, 117), (92, 190),
                    (93, 118), (94, 148), (97, 331), (99, 289), (100, 325), (101, 373), (103, 308), (105, 371),
                    (108, 297), (109, 247), (111, 358), (112, 327), (114, 310), (115, 248), (117, 300), (118, 375),
                    (120, 161), (123, 82), (124, 147), (125, 83), (126, 184),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 413; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 1; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            2 => {
                if let Some(next) = advance_map(&[
                    (4, 234), (33, 185), (35, 231), (36, 90), (37, 202), (38, 204), (40, 101), (41, 98),
                    (42, 206), (43, 152), (44, 97), (45, 155), (46, 69), (47, 50), (48, 412), (58, 52),
                    (59, 86), (60, 53), (61, 57), (62, 58), (64, 95), (91, 117), (92, 190), (93, 118),
                    (97, 331), (99, 290), (100, 325), (101, 373), (103, 357), (108, 298), (109, 247), (111, 358),
                    (112, 327), (114, 310), (115, 248), (117, 300), (118, 375), (120, 346), (123, 82), (125, 83),
                    (126, 183),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 413; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 3; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            3 => {
                if let Some(next) = advance_map(&[
                    (4, 234), (33, 185), (35, 231), (36, 90), (37, 202), (38, 204), (40, 101), (41, 98),
                    (42, 206), (43, 152), (44, 97), (45, 155), (46, 69), (47, 50), (48, 412), (58, 52),
                    (59, 86), (60, 53), (61, 57), (64, 95), (91, 117), (92, 190), (93, 118), (97, 331),
                    (99, 290), (100, 325), (101, 373), (103, 357), (108, 298), (109, 247), (111, 358), (112, 327),
                    (114, 310), (115, 248), (117, 300), (118, 375), (120, 346), (123, 82), (125, 83), (126, 183),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 413; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 3; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            4 => {
                if let Some(next) = advance_map(&[
                    (4, 234), (33, 54), (35, 231), (36, 46), (37, 120), (38, 150), (41, 98), (42, 116),
                    (43, 153), (44, 97), (45, 156), (46, 158), (47, 160), (58, 85), (59, 86), (60, 174),
                    (61, 88), (62, 122), (63, 189), (64, 96), (91, 117), (93, 118), (94, 148), (97, 343),
                    (99, 339), (103, 309), (105, 371), (108, 312), (111, 366), (120, 161), (123, 82), (124, 147),
                    (125, 83), (126, 65),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 6; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                if let Some(next) = advance_map(&[
                    (4, 234), (33, 54), (35, 231), (36, 89), (37, 120), (38, 150), (40, 101), (41, 98),
                    (42, 116), (43, 153), (44, 97), (45, 156), (46, 158), (47, 160), (58, 84), (59, 86),
                    (60, 174), (61, 88), (62, 122), (63, 189), (91, 117), (92, 190), (93, 118), (94, 148),
                    (97, 382), (99, 381), (103, 379), (105, 388), (108, 380), (111, 385), (120, 163), (123, 82),
                    (124, 147), (125, 83), (126, 65),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 5; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            6 => {
                if let Some(next) = advance_map(&[
                    (4, 234), (33, 54), (35, 231), (37, 120), (38, 150), (41, 98), (42, 116), (43, 153),
                    (44, 97), (45, 156), (46, 158), (47, 160), (58, 85), (59, 86), (60, 174), (61, 88),
                    (62, 122), (63, 189), (64, 95), (91, 117), (93, 118), (94, 148), (97, 343), (99, 339),
                    (103, 309), (105, 371), (108, 312), (111, 366), (120, 161), (123, 82), (124, 147), (125, 83),
                    (126, 65),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 6; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if let Some(next) = advance_map(&[
                    (4, 234), (33, 54), (35, 231), (37, 120), (38, 150), (41, 98), (42, 116), (43, 153),
                    (44, 97), (45, 156), (46, 158), (47, 160), (59, 86), (60, 174), (61, 88), (62, 122),
                    (63, 189), (93, 118), (94, 148), (97, 382), (99, 381), (103, 379), (105, 388), (108, 380),
                    (111, 385), (120, 163), (124, 147), (125, 83), (126, 65),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 7; lexer.advance(true); continue; }
                if set_contains(&sym__identifier_character_set_1, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            8 => {
                if let Some(next) = advance_map(&[
                    (4, 234), (33, 54), (35, 231), (37, 120), (38, 150), (41, 98), (42, 116), (43, 153),
                    (44, 97), (45, 156), (46, 158), (47, 160), (59, 86), (60, 174), (61, 88), (62, 122),
                    (63, 189), (93, 118), (94, 148), (97, 382), (99, 283), (103, 379), (105, 388), (108, 380),
                    (111, 385), (120, 163), (124, 147), (125, 83), (126, 65), (100, 283), (114, 283), (115, 283),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 7; lexer.advance(true); continue; }
                if set_contains(&sym__identifier_character_set_1, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if let Some(next) = advance_map(&[
                    (4, 234), (33, 54), (35, 231), (37, 120), (38, 150), (41, 98), (42, 116), (43, 153),
                    (44, 97), (45, 156), (46, 158), (47, 160), (59, 86), (60, 174), (61, 88), (62, 122),
                    (63, 189), (93, 118), (94, 148), (97, 270), (99, 269), (103, 274), (105, 273), (108, 274),
                    (111, 274), (120, 271), (124, 147), (125, 83), (126, 65), (100, 274), (109, 274), (110, 274),
                    (112, 274), (115, 274), (117, 274),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 7; lexer.advance(true); continue; }
                if set_contains(&sym__identifier_character_set_1, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            10 => {
                if let Some(next) = advance_map(&[
                    (4, 234), (33, 54), (35, 231), (37, 120), (38, 150), (41, 98), (42, 116), (43, 153),
                    (44, 97), (45, 156), (46, 158), (47, 160), (59, 86), (60, 174), (61, 88), (62, 122),
                    (63, 189), (93, 118), (94, 148), (97, 282), (99, 277), (103, 276), (105, 281), (108, 276),
                    (111, 280), (120, 278), (124, 147), (125, 83), (126, 65), (100, 282), (101, 282), (109, 282),
                    (112, 282), (114, 282), (115, 282), (117, 282),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 7; lexer.advance(true); continue; }
                if set_contains(&sym__identifier_character_set_1, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if let Some(next) = advance_map(&[
                    (4, 234), (33, 54), (35, 231), (37, 120), (38, 150), (41, 98), (42, 116), (43, 153),
                    (44, 97), (45, 156), (46, 158), (47, 160), (59, 86), (60, 174), (61, 88), (62, 122),
                    (63, 189), (93, 118), (94, 148), (97, 258), (99, 381), (103, 379), (105, 264), (108, 266),
                    (111, 266), (120, 260), (124, 147), (125, 83), (126, 65), (100, 266), (109, 266), (110, 266),
                    (112, 266), (115, 266), (117, 266),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 7; lexer.advance(true); continue; }
                if set_contains(&sym__identifier_character_set_1, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if let Some(next) = advance_map(&[
                    (4, 234), (35, 231), (36, 90), (41, 98), (45, 154), (59, 86), (64, 95), (91, 117),
                    (97, 382), (111, 385), (120, 383), (123, 82), (125, 83),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 12; lexer.advance(true); continue; }
                if set_contains(&sym__identifier_character_set_1, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if let Some(next) = advance_map(&[
                    (4, 234), (35, 231), (36, 90), (41, 98), (45, 157), (59, 86), (64, 95), (91, 237),
                    (97, 382), (111, 385), (120, 383), (123, 238), (125, 83),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 12; lexer.advance(true); continue; }
                if set_contains(&sym__identifier_character_set_1, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if let Some(next) = advance_map(&[
                    (4, 234), (35, 231), (36, 89), (37, 202), (40, 101), (41, 98), (44, 97), (47, 51),
                    (58, 84), (59, 86), (61, 87), (64, 95), (92, 190), (123, 82), (124, 64),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 14; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if lookahead == 10 { state = 15; lexer.advance(true); continue; }
                if lookahead == 35 { state = 231; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 285; lexer.advance(false); continue; }
                if lookahead != 0 { state = 284; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if let Some(next) = advance_map(&[
                    (33, 186), (35, 231), (36, 90), (37, 203), (38, 205), (40, 101), (41, 98), (42, 207),
                    (43, 153), (44, 97), (45, 156), (46, 159), (47, 160), (48, 412), (58, 85), (59, 86),
                    (60, 174), (61, 88), (62, 122), (63, 189), (64, 95), (91, 117), (92, 190), (94, 148),
                    (97, 332), (99, 289), (100, 325), (101, 373), (103, 308), (105, 371), (108, 297), (109, 247),
                    (111, 368), (112, 327), (114, 310), (115, 248), (117, 300), (118, 375), (120, 162), (123, 82),
                    (124, 147), (125, 83), (126, 184),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 413; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 16; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if let Some(next) = advance_map(&[
                    (33, 185), (35, 231), (36, 90), (37, 202), (38, 204), (40, 101), (41, 98), (42, 206),
                    (43, 152), (44, 97), (45, 155), (46, 69), (47, 50), (48, 412), (58, 85), (59, 86),
                    (60, 53), (61, 57), (64, 95), (91, 117), (92, 190), (93, 118), (97, 332), (99, 290),
                    (100, 325), (101, 373), (103, 357), (108, 298), (109, 247), (111, 368), (112, 327), (114, 310),
                    (115, 248), (117, 300), (118, 375), (123, 82), (125, 83), (126, 183),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 413; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 17; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            18 => {
                if let Some(next) = advance_map(&[
                    (33, 185), (35, 231), (36, 90), (37, 202), (38, 204), (40, 101), (41, 98), (42, 206),
                    (43, 152), (45, 155), (46, 69), (47, 50), (48, 412), (58, 52), (60, 53), (64, 95),
                    (91, 117), (92, 190), (97, 332), (99, 290), (100, 325), (101, 335), (103, 357), (108, 298),
                    (109, 247), (111, 368), (112, 327), (114, 310), (115, 248), (117, 300), (118, 375), (123, 82),
                    (126, 183),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 413; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 18; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if let Some(next) = advance_map(&[
                    (33, 185), (35, 231), (36, 90), (37, 202), (38, 204), (40, 101), (42, 206), (43, 152),
                    (45, 155), (46, 397), (47, 400), (48, 398), (58, 401), (59, 86), (60, 402), (64, 95),
                    (91, 117), (92, 190), (94, 405), (97, 332), (99, 290), (100, 325), (101, 373), (103, 357),
                    (108, 298), (109, 247), (110, 344), (111, 368), (112, 327), (114, 310), (115, 248), (117, 299),
                    (118, 375), (123, 82), (125, 83), (126, 183),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 19; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 399; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 392; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 376; lexer.advance(false); continue; }
                if lookahead != 0 { state = 391; lexer.advance(false); continue; }
                return result;
            }
            20 => {
                if let Some(next) = advance_map(&[
                    (33, 185), (35, 231), (36, 90), (37, 202), (38, 204), (40, 101), (42, 206), (43, 152),
                    (45, 155), (46, 69), (47, 50), (48, 410), (58, 52), (59, 86), (60, 53), (64, 95),
                    (91, 117), (92, 190), (97, 332), (99, 290), (100, 325), (101, 373), (103, 357), (108, 298),
                    (109, 247), (111, 368), (112, 327), (114, 310), (115, 248), (117, 300), (118, 374), (123, 82),
                    (126, 183),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 411; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 20; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                if let Some(next) = advance_map(&[
                    (33, 54), (35, 231), (36, 89), (37, 203), (38, 150), (40, 101), (41, 98), (42, 116),
                    (43, 153), (44, 97), (45, 156), (46, 158), (47, 160), (58, 84), (60, 174), (61, 88),
                    (62, 122), (63, 189), (64, 95), (91, 117), (94, 148), (99, 381), (103, 379), (105, 388),
                    (108, 380), (120, 164), (123, 82), (124, 147), (126, 65),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 21; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            22 => {
                if let Some(next) = advance_map(&[
                    (33, 54), (35, 231), (36, 89), (37, 203), (38, 150), (41, 98), (42, 116), (43, 153),
                    (44, 97), (45, 156), (46, 158), (47, 160), (58, 85), (60, 174), (61, 88), (62, 122),
                    (63, 189), (64, 95), (94, 148), (99, 339), (103, 309), (105, 371), (108, 312), (120, 162),
                    (124, 147), (126, 65),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 22; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            23 => {
                if let Some(next) = advance_map(&[
                    (33, 54), (35, 231), (36, 89), (37, 203), (38, 150), (41, 98), (42, 116), (43, 153),
                    (44, 97), (45, 156), (46, 158), (47, 160), (58, 84), (60, 174), (61, 88), (62, 122),
                    (63, 189), (64, 95), (94, 148), (99, 381), (103, 379), (105, 264), (108, 266), (120, 266),
                    (124, 147), (126, 65),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 97 || lookahead == 100 || 109 <= lookahead && lookahead <= 112 || lookahead == 115 || lookahead == 117 { state = 266; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 24; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            24 => {
                if let Some(next) = advance_map(&[
                    (33, 54), (35, 231), (36, 89), (37, 203), (38, 150), (41, 98), (42, 116), (43, 153),
                    (44, 97), (45, 156), (46, 158), (47, 160), (58, 84), (60, 174), (61, 88), (62, 122),
                    (63, 189), (64, 95), (94, 148), (99, 381), (103, 379), (105, 388), (108, 380), (120, 164),
                    (124, 147), (126, 65),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 24; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            25 => {
                if let Some(next) = advance_map(&[
                    (33, 54), (35, 231), (36, 89), (37, 203), (38, 150), (41, 98), (42, 116), (43, 153),
                    (44, 97), (45, 156), (46, 158), (47, 160), (58, 84), (60, 174), (61, 88), (62, 122),
                    (63, 189), (64, 95), (94, 148), (99, 283), (103, 379), (105, 388), (108, 380), (120, 164),
                    (124, 147), (126, 65), (100, 283), (114, 283), (115, 283),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 24; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            26 => {
                if let Some(next) = advance_map(&[
                    (33, 54), (35, 231), (36, 89), (37, 203), (38, 150), (41, 98), (42, 116), (43, 153),
                    (44, 97), (45, 156), (46, 158), (47, 160), (58, 84), (60, 174), (61, 88), (62, 122),
                    (63, 189), (64, 95), (94, 148), (99, 269), (103, 274), (105, 273), (108, 274), (120, 274),
                    (124, 147), (126, 65),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 97 || lookahead == 100 || 109 <= lookahead && lookahead <= 112 || lookahead == 115 || lookahead == 117 { state = 274; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 24; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            27 => {
                if let Some(next) = advance_map(&[
                    (33, 54), (35, 231), (36, 89), (37, 203), (38, 150), (41, 98), (42, 116), (43, 153),
                    (44, 97), (45, 156), (46, 158), (47, 160), (58, 84), (60, 174), (61, 88), (62, 122),
                    (63, 189), (64, 95), (94, 148), (99, 277), (103, 276), (105, 281), (108, 276), (120, 282),
                    (124, 147), (126, 65), (97, 282), (100, 282), (101, 282), (109, 282), (111, 282), (112, 282),
                    (114, 282), (115, 282), (117, 282),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 24; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            28 => {
                if let Some(next) = advance_map(&[
                    (33, 54), (35, 231), (37, 120), (38, 150), (40, 101), (41, 98), (42, 116), (43, 153),
                    (44, 97), (45, 156), (46, 158), (47, 160), (58, 84), (59, 86), (60, 174), (61, 88),
                    (62, 122), (63, 189), (91, 117), (94, 148), (99, 381), (103, 379), (105, 388), (108, 380),
                    (120, 164), (123, 82), (124, 147), (125, 83), (126, 65),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 28; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                if let Some(next) = advance_map(&[
                    (33, 54), (35, 231), (37, 120), (38, 150), (41, 98), (42, 116), (43, 153), (44, 97),
                    (45, 156), (46, 158), (47, 160), (58, 85), (59, 86), (60, 174), (61, 88), (62, 122),
                    (63, 189), (94, 148), (99, 339), (103, 309), (105, 371), (108, 312), (120, 162), (124, 147),
                    (125, 83), (126, 65),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 29; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            30 => {
                if let Some(next) = advance_map(&[
                    (33, 54), (35, 231), (37, 120), (38, 150), (41, 98), (42, 116), (43, 153), (44, 97),
                    (45, 156), (46, 158), (47, 160), (58, 84), (59, 86), (60, 174), (61, 88), (62, 122),
                    (63, 189), (94, 148), (99, 381), (103, 379), (105, 264), (108, 266), (120, 266), (124, 147),
                    (125, 83), (126, 65),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 97 || lookahead == 100 || 109 <= lookahead && lookahead <= 112 || lookahead == 115 || lookahead == 117 { state = 266; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 31; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            31 => {
                if let Some(next) = advance_map(&[
                    (33, 54), (35, 231), (37, 120), (38, 150), (41, 98), (42, 116), (43, 153), (44, 97),
                    (45, 156), (46, 158), (47, 160), (58, 84), (59, 86), (60, 174), (61, 88), (62, 122),
                    (63, 189), (94, 148), (99, 381), (103, 379), (105, 388), (108, 380), (120, 164), (124, 147),
                    (125, 83), (126, 65),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 31; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                if let Some(next) = advance_map(&[
                    (33, 54), (35, 231), (37, 120), (38, 150), (41, 98), (42, 116), (43, 153), (44, 97),
                    (45, 156), (46, 158), (47, 160), (58, 84), (59, 86), (60, 174), (61, 88), (62, 122),
                    (63, 189), (94, 148), (99, 283), (103, 379), (105, 388), (108, 380), (120, 164), (124, 147),
                    (125, 83), (126, 65), (100, 283), (114, 283), (115, 283),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 31; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            33 => {
                if let Some(next) = advance_map(&[
                    (33, 54), (35, 231), (37, 120), (38, 150), (41, 98), (42, 116), (43, 153), (44, 97),
                    (45, 156), (46, 158), (47, 160), (58, 84), (59, 86), (60, 174), (61, 88), (62, 122),
                    (63, 189), (94, 148), (99, 269), (103, 274), (105, 273), (108, 274), (120, 274), (124, 147),
                    (125, 83), (126, 65),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if lookahead == 97 || lookahead == 100 || 109 <= lookahead && lookahead <= 112 || lookahead == 115 || lookahead == 117 { state = 274; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 31; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            34 => {
                if let Some(next) = advance_map(&[
                    (33, 54), (35, 231), (37, 120), (38, 150), (41, 98), (42, 116), (43, 153), (44, 97),
                    (45, 156), (46, 158), (47, 160), (58, 84), (59, 86), (60, 174), (61, 88), (62, 122),
                    (63, 189), (94, 148), (99, 277), (103, 276), (105, 281), (108, 276), (120, 282), (124, 147),
                    (125, 83), (126, 65), (97, 282), (100, 282), (101, 282), (109, 282), (111, 282), (112, 282),
                    (114, 282), (115, 282), (117, 282),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 31; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            35 => {
                if let Some(next) = advance_map(&[
                    (35, 231), (36, 90), (37, 119), (38, 149), (40, 101), (42, 115), (45, 154), (58, 85),
                    (59, 86), (61, 87), (64, 95), (91, 117), (123, 82),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 35; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            36 => {
                if let Some(next) = advance_map(&[
                    (35, 231), (36, 89), (37, 202), (40, 101), (42, 115), (45, 154), (58, 52), (62, 121),
                    (64, 95), (91, 117), (92, 190), (123, 82),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 36; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                if lookahead == 35 { state = 231; lexer.advance(false); continue; }
                if lookahead == 36 { state = 89; lexer.advance(false); continue; }
                if lookahead == 40 { state = 101; lexer.advance(false); continue; }
                if lookahead == 92 { state = 190; lexer.advance(false); continue; }
                if lookahead == 111 { state = 389; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 37; lexer.advance(true); continue; }
                if set_contains(&sym__identifier_character_set_1, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            38 => {
                if lookahead == 35 { state = 231; lexer.advance(false); continue; }
                if lookahead == 36 { state = 89; lexer.advance(false); continue; }
                if lookahead == 42 { state = 115; lexer.advance(false); continue; }
                if lookahead == 58 { state = 401; lexer.advance(false); continue; }
                if lookahead == 94 { state = 407; lexer.advance(false); continue; }
                if lookahead == 123 { state = 82; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 38; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 406; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 393; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 376; lexer.advance(false); continue; }
                if lookahead != 0 { state = 391; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                if let Some(next) = advance_map(&[
                    (35, 231), (36, 89), (58, 401), (94, 244), (123, 82), (39, 391), (43, 391), (45, 391),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 242; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 406; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 242; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 376; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 243; lexer.advance(false); continue; }
                if lookahead != 0 { state = 241; lexer.advance(false); continue; }
                return result;
            }
            40 => {
                if lookahead == 35 { state = 231; lexer.advance(false); continue; }
                if lookahead == 36 { state = 89; lexer.advance(false); continue; }
                if lookahead == 58 { state = 401; lexer.advance(false); continue; }
                if lookahead == 94 { state = 407; lexer.advance(false); continue; }
                if lookahead == 123 { state = 82; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 40; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 406; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 394; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 376; lexer.advance(false); continue; }
                if lookahead != 0 { state = 391; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                if lookahead == 35 { state = 231; lexer.advance(false); continue; }
                if lookahead == 36 { state = 89; lexer.advance(false); continue; }
                if lookahead == 94 { state = 407; lexer.advance(false); continue; }
                if lookahead == 123 { state = 82; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 41; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 406; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 395; lexer.advance(false); continue; }
                if set_contains(&sym__identifier_character_set_1, lookahead) { state = 390; lexer.advance(false); continue; }
                if lookahead != 0 { state = 391; lexer.advance(false); continue; }
                return result;
            }
            42 => {
                if lookahead == 35 { state = 231; lexer.advance(false); continue; }
                if lookahead == 36 { state = 92; lexer.advance(false); continue; }
                if lookahead == 64 { state = 96; lexer.advance(false); continue; }
                if lookahead == 91 { state = 117; lexer.advance(false); continue; }
                if lookahead == 123 { state = 82; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 246; lexer.advance(false); continue; }
                if lookahead != 0 { state = 245; lexer.advance(false); continue; }
                return result;
            }
            43 => {
                if let Some(next) = advance_map(&[
                    (35, 231), (41, 98), (46, 69), (48, 410), (58, 84), (59, 86), (93, 118), (97, 60),
                    (111, 62), (118, 70), (120, 61), (123, 82), (125, 83),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 411; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 43; lexer.advance(true); continue; }
                return result;
            }
            44 => {
                if lookahead == 35 { state = 231; lexer.advance(false); continue; }
                if lookahead == 46 { state = 69; lexer.advance(false); continue; }
                if lookahead == 48 { state = 410; lexer.advance(false); continue; }
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 118 { state = 374; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 411; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 44; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            45 => {
                if lookahead == 35 { state = 231; lexer.advance(false); continue; }
                if lookahead == 58 { state = 401; lexer.advance(false); continue; }
                if lookahead == 94 { state = 405; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 45; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 406; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 396; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 376; lexer.advance(false); continue; }
                if lookahead != 0 { state = 391; lexer.advance(false); continue; }
                return result;
            }
            46 => {
                if lookahead == 42 { state = 239; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                if lookahead == 46 { state = 102; lexer.advance(false); continue; }
                return result;
            }
            48 => {
                if lookahead == 46 { state = 47; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 421; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                if lookahead == 46 { state = 73; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 49; lexer.advance(false); continue; }
                return result;
            }
            50 => {
                if lookahead == 47 { state = 143; lexer.advance(false); continue; }
                return result;
            }
            51 => {
                if lookahead == 47 { state = 55; lexer.advance(false); continue; }
                return result;
            }
            52 => {
                if lookahead == 58 { state = 408; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                if lookahead == 60 { state = 123; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                if lookahead == 61 { state = 168; lexer.advance(false); continue; }
                if lookahead == 126 { state = 166; lexer.advance(false); continue; }
                return result;
            }
            55 => {
                if lookahead == 61 { state = 94; lexer.advance(false); continue; }
                return result;
            }
            56 => {
                if lookahead == 61 { state = 93; lexer.advance(false); continue; }
                return result;
            }
            57 => {
                if lookahead == 62 { state = 209; lexer.advance(false); continue; }
                return result;
            }
            58 => {
                if lookahead == 62 { state = 125; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                if lookahead == 100 { state = 104; lexer.advance(false); continue; }
                return result;
            }
            60 => {
                if lookahead == 110 { state = 59; lexer.advance(false); continue; }
                return result;
            }
            61 => {
                if lookahead == 111 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            62 => {
                if lookahead == 114 { state = 107; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                if lookahead == 114 { state = 111; lexer.advance(false); continue; }
                return result;
            }
            64 => {
                if lookahead == 124 { state = 56; lexer.advance(false); continue; }
                return result;
            }
            65 => {
                if lookahead == 126 { state = 173; lexer.advance(false); continue; }
                return result;
            }
            66 => {
                if lookahead == 43 || lookahead == 45 { state = 71; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 417; lexer.advance(false); continue; }
                return result;
            }
            67 => {
                if lookahead == 48 || lookahead == 49 { state = 418; lexer.advance(false); continue; }
                return result;
            }
            68 => {
                if lookahead == 56 || lookahead == 57 { state = 417; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 55 { state = 420; lexer.advance(false); continue; }
                return result;
            }
            69 => {
                if 48 <= lookahead && lookahead <= 57 { state = 421; lexer.advance(false); continue; }
                return result;
            }
            70 => {
                if 48 <= lookahead && lookahead <= 57 { state = 286; lexer.advance(false); continue; }
                return result;
            }
            71 => {
                if 48 <= lookahead && lookahead <= 57 { state = 417; lexer.advance(false); continue; }
                return result;
            }
            72 => {
                if 48 <= lookahead && lookahead <= 57 { state = 49; lexer.advance(false); continue; }
                return result;
            }
            73 => {
                if 48 <= lookahead && lookahead <= 57 { state = 287; lexer.advance(false); continue; }
                return result;
            }
            74 => {
                if 48 <= lookahead && lookahead <= 57 { state = 235; lexer.advance(false); continue; }
                return result;
            }
            75 => {
                if 48 <= lookahead && lookahead <= 57 { state = 413; lexer.advance(false); continue; }
                return result;
            }
            76 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 419; lexer.advance(false); continue; }
                return result;
            }
            77 => {
                if eof { state = 81; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (4, 234), (33, 186), (35, 231), (36, 90), (37, 203), (38, 205), (40, 101), (41, 98),
                    (42, 207), (43, 153), (44, 97), (45, 156), (46, 159), (47, 160), (48, 410), (58, 85),
                    (59, 86), (60, 174), (61, 88), (62, 122), (63, 189), (64, 95), (91, 117), (92, 190),
                    (93, 118), (94, 148), (97, 331), (99, 289), (100, 325), (101, 335), (103, 308), (105, 371),
                    (108, 297), (109, 247), (110, 344), (111, 358), (112, 327), (114, 310), (115, 248), (117, 299),
                    (118, 374), (120, 161), (123, 82), (124, 147), (125, 83), (126, 184),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 411; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 77; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            78 => {
                if eof { state = 81; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (4, 234), (33, 186), (35, 231), (36, 90), (37, 203), (38, 205), (40, 101), (41, 98),
                    (42, 207), (43, 153), (44, 97), (45, 156), (46, 159), (47, 160), (48, 412), (58, 52),
                    (59, 86), (60, 174), (61, 88), (62, 122), (63, 189), (64, 95), (91, 117), (92, 190),
                    (94, 148), (97, 331), (99, 289), (100, 325), (101, 373), (103, 308), (105, 371), (108, 297),
                    (109, 247), (110, 344), (111, 358), (112, 327), (114, 310), (115, 248), (117, 299), (118, 375),
                    (120, 161), (123, 82), (124, 147), (125, 83), (126, 184),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 413; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 78; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            79 => {
                if eof { state = 81; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (4, 234), (33, 185), (35, 231), (36, 90), (37, 202), (38, 204), (40, 101), (41, 98),
                    (42, 206), (43, 152), (44, 97), (45, 155), (46, 48), (47, 50), (48, 412), (58, 52),
                    (59, 86), (60, 53), (61, 87), (62, 121), (64, 95), (91, 117), (92, 190), (97, 332),
                    (99, 290), (100, 325), (101, 373), (103, 357), (108, 298), (109, 247), (110, 344), (111, 368),
                    (112, 327), (114, 310), (115, 248), (117, 299), (118, 375), (123, 82), (125, 83), (126, 183),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 413; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 79; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            80 => {
                if eof { state = 81; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (4, 234), (33, 185), (35, 231), (36, 90), (37, 202), (38, 204), (40, 101), (42, 206),
                    (43, 152), (45, 155), (46, 48), (47, 50), (48, 412), (58, 52), (59, 86), (60, 53),
                    (64, 95), (91, 117), (92, 190), (97, 332), (99, 290), (100, 325), (101, 335), (103, 357),
                    (108, 298), (109, 247), (110, 344), (111, 368), (112, 327), (114, 310), (115, 248), (117, 299),
                    (118, 375), (123, 82), (125, 83), (126, 183),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 413; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 80; lexer.advance(true); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            81 => {
                result = true; lexer.set_result_symbol(ts_builtin_sym_end); lexer.mark_end();
                return result;
            }
            82 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACE); lexer.mark_end();
                return result;
            }
            83 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACE); lexer.mark_end();
                return result;
            }
            84 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON); lexer.mark_end();
                return result;
            }
            85 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON); lexer.mark_end();
                if lookahead == 58 { state = 408; lexer.advance(false); continue; }
                return result;
            }
            86 => {
                result = true; lexer.set_result_symbol(anon_sym_SEMI); lexer.mark_end();
                return result;
            }
            87 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                return result;
            }
            88 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                if lookahead == 61 { state = 167; lexer.advance(false); continue; }
                if lookahead == 62 { state = 209; lexer.advance(false); continue; }
                if lookahead == 126 { state = 165; lexer.advance(false); continue; }
                return result;
            }
            89 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR); lexer.mark_end();
                return result;
            }
            90 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR); lexer.mark_end();
                if lookahead == 35 { state = 193; lexer.advance(false); continue; }
                return result;
            }
            91 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR); lexer.mark_end();
                if lookahead == 35 { state = 193; lexer.advance(false); continue; }
                if lookahead == 42 { state = 239; lexer.advance(false); continue; }
                return result;
            }
            92 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR); lexer.mark_end();
                if lookahead == 42 { state = 239; lexer.advance(false); continue; }
                return result;
            }
            93 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE_PIPE_EQ); lexer.mark_end();
                return result;
            }
            94 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_SLASH_EQ); lexer.mark_end();
                return result;
            }
            95 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                return result;
            }
            96 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                if lookahead == 42 { state = 240; lexer.advance(false); continue; }
                return result;
            }
            97 => {
                result = true; lexer.set_result_symbol(anon_sym_COMMA); lexer.mark_end();
                return result;
            }
            98 => {
                result = true; lexer.set_result_symbol(anon_sym_RPAREN); lexer.mark_end();
                return result;
            }
            99 => {
                result = true; lexer.set_result_symbol(anon_sym_our); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            100 => {
                result = true; lexer.set_result_symbol(anon_sym_our); lexer.mark_end();
                if set_contains(&sym__identifier_character_set_2, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            101 => {
                result = true; lexer.set_result_symbol(anon_sym_LPAREN); lexer.mark_end();
                return result;
            }
            102 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT_DOT_DOT); lexer.mark_end();
                return result;
            }
            103 => {
                result = true; lexer.set_result_symbol(anon_sym_else); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            104 => {
                result = true; lexer.set_result_symbol(anon_sym_and); lexer.mark_end();
                return result;
            }
            105 => {
                result = true; lexer.set_result_symbol(anon_sym_and); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            106 => {
                result = true; lexer.set_result_symbol(anon_sym_and); lexer.mark_end();
                if set_contains(&sym__identifier_character_set_2, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            107 => {
                result = true; lexer.set_result_symbol(anon_sym_or); lexer.mark_end();
                return result;
            }
            108 => {
                result = true; lexer.set_result_symbol(anon_sym_or); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 100 { state = 221; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            109 => {
                result = true; lexer.set_result_symbol(anon_sym_or); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            110 => {
                result = true; lexer.set_result_symbol(anon_sym_or); lexer.mark_end();
                if set_contains(&sym__identifier_character_set_2, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            111 => {
                result = true; lexer.set_result_symbol(anon_sym_xor); lexer.mark_end();
                return result;
            }
            112 => {
                result = true; lexer.set_result_symbol(anon_sym_xor); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            113 => {
                result = true; lexer.set_result_symbol(anon_sym_xor); lexer.mark_end();
                if set_contains(&sym__identifier_character_set_2, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            114 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_GT); lexer.mark_end();
                return result;
            }
            115 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                return result;
            }
            116 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                if lookahead == 42 { state = 141; lexer.advance(false); continue; }
                if lookahead == 61 { state = 130; lexer.advance(false); continue; }
                return result;
            }
            117 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK); lexer.mark_end();
                return result;
            }
            118 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACK); lexer.mark_end();
                return result;
            }
            119 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT); lexer.mark_end();
                return result;
            }
            120 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT); lexer.mark_end();
                if lookahead == 61 { state = 132; lexer.advance(false); continue; }
                return result;
            }
            121 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                return result;
            }
            122 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                if lookahead == 61 { state = 176; lexer.advance(false); continue; }
                if lookahead == 62 { state = 151; lexer.advance(false); continue; }
                return result;
            }
            123 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT); lexer.mark_end();
                return result;
            }
            124 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT); lexer.mark_end();
                if lookahead == 61 { state = 137; lexer.advance(false); continue; }
                return result;
            }
            125 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT); lexer.mark_end();
                return result;
            }
            126 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR_STAR_EQ); lexer.mark_end();
                return result;
            }
            127 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS_EQ); lexer.mark_end();
                return result;
            }
            128 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_EQ); lexer.mark_end();
                return result;
            }
            129 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT_EQ); lexer.mark_end();
                return result;
            }
            130 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR_EQ); lexer.mark_end();
                return result;
            }
            131 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_EQ); lexer.mark_end();
                return result;
            }
            132 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT_EQ); lexer.mark_end();
                return result;
            }
            133 => {
                result = true; lexer.set_result_symbol(anon_sym_x_EQ); lexer.mark_end();
                return result;
            }
            134 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP_EQ); lexer.mark_end();
                return result;
            }
            135 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE_EQ); lexer.mark_end();
                return result;
            }
            136 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET_EQ); lexer.mark_end();
                return result;
            }
            137 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_LT_EQ); lexer.mark_end();
                return result;
            }
            138 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT_EQ); lexer.mark_end();
                return result;
            }
            139 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP_AMP_EQ); lexer.mark_end();
                return result;
            }
            140 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT_DOT); lexer.mark_end();
                if lookahead == 46 { state = 102; lexer.advance(false); continue; }
                return result;
            }
            141 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR_STAR); lexer.mark_end();
                if lookahead == 61 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            142 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE_PIPE); lexer.mark_end();
                if lookahead == 61 { state = 93; lexer.advance(false); continue; }
                return result;
            }
            143 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_SLASH); lexer.mark_end();
                return result;
            }
            144 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH_SLASH); lexer.mark_end();
                if lookahead == 61 { state = 94; lexer.advance(false); continue; }
                return result;
            }
            145 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET_CARET); lexer.mark_end();
                return result;
            }
            146 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP_AMP); lexer.mark_end();
                if lookahead == 61 { state = 139; lexer.advance(false); continue; }
                return result;
            }
            147 => {
                result = true; lexer.set_result_symbol(anon_sym_PIPE); lexer.mark_end();
                if lookahead == 61 { state = 135; lexer.advance(false); continue; }
                if lookahead == 124 { state = 142; lexer.advance(false); continue; }
                return result;
            }
            148 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET); lexer.mark_end();
                if lookahead == 61 { state = 136; lexer.advance(false); continue; }
                if lookahead == 94 { state = 145; lexer.advance(false); continue; }
                return result;
            }
            149 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP); lexer.mark_end();
                return result;
            }
            150 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP); lexer.mark_end();
                if lookahead == 38 { state = 146; lexer.advance(false); continue; }
                if lookahead == 61 { state = 134; lexer.advance(false); continue; }
                return result;
            }
            151 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_GT2); lexer.mark_end();
                if lookahead == 61 { state = 138; lexer.advance(false); continue; }
                return result;
            }
            152 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                if lookahead == 43 { state = 187; lexer.advance(false); continue; }
                return result;
            }
            153 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                if lookahead == 43 { state = 187; lexer.advance(false); continue; }
                if lookahead == 61 { state = 127; lexer.advance(false); continue; }
                return result;
            }
            154 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                return result;
            }
            155 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 188; lexer.advance(false); continue; }
                return result;
            }
            156 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 188; lexer.advance(false); continue; }
                if lookahead == 61 { state = 128; lexer.advance(false); continue; }
                if lookahead == 62 { state = 114; lexer.advance(false); continue; }
                return result;
            }
            157 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 62 { state = 236; lexer.advance(false); continue; }
                return result;
            }
            158 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                if lookahead == 46 { state = 140; lexer.advance(false); continue; }
                if lookahead == 61 { state = 129; lexer.advance(false); continue; }
                return result;
            }
            159 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                if lookahead == 46 { state = 140; lexer.advance(false); continue; }
                if lookahead == 61 { state = 129; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 421; lexer.advance(false); continue; }
                return result;
            }
            160 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH); lexer.mark_end();
                if lookahead == 47 { state = 144; lexer.advance(false); continue; }
                if lookahead == 61 { state = 131; lexer.advance(false); continue; }
                return result;
            }
            161 => {
                result = true; lexer.set_result_symbol(anon_sym_x); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 61 { state = 133; lexer.advance(false); continue; }
                if lookahead == 111 { state = 360; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            162 => {
                result = true; lexer.set_result_symbol(anon_sym_x); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 61 { state = 133; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            163 => {
                result = true; lexer.set_result_symbol(anon_sym_x); lexer.mark_end();
                if lookahead == 61 { state = 133; lexer.advance(false); continue; }
                if lookahead == 111 { state = 386; lexer.advance(false); continue; }
                if set_contains(&sym__identifier_character_set_2, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            164 => {
                result = true; lexer.set_result_symbol(anon_sym_x); lexer.mark_end();
                if lookahead == 61 { state = 133; lexer.advance(false); continue; }
                if set_contains(&sym__identifier_character_set_2, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            165 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ_TILDE); lexer.mark_end();
                return result;
            }
            166 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG_TILDE); lexer.mark_end();
                return result;
            }
            167 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ_EQ); lexer.mark_end();
                if lookahead == 61 { state = 169; lexer.advance(false); continue; }
                return result;
            }
            168 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG_EQ); lexer.mark_end();
                return result;
            }
            169 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ_EQ_EQ); lexer.mark_end();
                return result;
            }
            170 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_EQ_GT); lexer.mark_end();
                return result;
            }
            171 => {
                result = true; lexer.set_result_symbol(anon_sym_cmp); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            172 => {
                result = true; lexer.set_result_symbol(anon_sym_cmp); lexer.mark_end();
                if set_contains(&sym__identifier_character_set_2, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            173 => {
                result = true; lexer.set_result_symbol(anon_sym_TILDE_TILDE); lexer.mark_end();
                return result;
            }
            174 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 60 { state = 124; lexer.advance(false); continue; }
                if lookahead == 61 { state = 175; lexer.advance(false); continue; }
                return result;
            }
            175 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_EQ); lexer.mark_end();
                if lookahead == 62 { state = 170; lexer.advance(false); continue; }
                return result;
            }
            176 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_EQ); lexer.mark_end();
                return result;
            }
            177 => {
                result = true; lexer.set_result_symbol(anon_sym_le); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            178 => {
                result = true; lexer.set_result_symbol(anon_sym_le); lexer.mark_end();
                if set_contains(&sym__identifier_character_set_2, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            179 => {
                result = true; lexer.set_result_symbol(anon_sym_ge); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            180 => {
                result = true; lexer.set_result_symbol(anon_sym_ge); lexer.mark_end();
                if set_contains(&sym__identifier_character_set_2, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            181 => {
                result = true; lexer.set_result_symbol(anon_sym_isa); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            182 => {
                result = true; lexer.set_result_symbol(anon_sym_isa); lexer.mark_end();
                if set_contains(&sym__identifier_character_set_2, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            183 => {
                result = true; lexer.set_result_symbol(anon_sym_TILDE); lexer.mark_end();
                return result;
            }
            184 => {
                result = true; lexer.set_result_symbol(anon_sym_TILDE); lexer.mark_end();
                if lookahead == 126 { state = 173; lexer.advance(false); continue; }
                return result;
            }
            185 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG); lexer.mark_end();
                return result;
            }
            186 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG); lexer.mark_end();
                if lookahead == 61 { state = 168; lexer.advance(false); continue; }
                if lookahead == 126 { state = 166; lexer.advance(false); continue; }
                return result;
            }
            187 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS_PLUS); lexer.mark_end();
                return result;
            }
            188 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_DASH); lexer.mark_end();
                return result;
            }
            189 => {
                result = true; lexer.set_result_symbol(anon_sym_QMARK); lexer.mark_end();
                return result;
            }
            190 => {
                result = true; lexer.set_result_symbol(anon_sym_BSLASH); lexer.mark_end();
                return result;
            }
            191 => {
                result = true; lexer.set_result_symbol(anon_sym_do); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            192 => {
                result = true; lexer.set_result_symbol(anon_sym_local); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            193 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR_POUND); lexer.mark_end();
                return result;
            }
            194 => {
                result = true; lexer.set_result_symbol(anon_sym_map); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            195 => {
                result = true; lexer.set_result_symbol(anon_sym_grep); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            196 => {
                result = true; lexer.set_result_symbol(anon_sym_exec); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            197 => {
                result = true; lexer.set_result_symbol(anon_sym_die); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            198 => {
                result = true; lexer.set_result_symbol(anon_sym_pipe); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            199 => {
                result = true; lexer.set_result_symbol(anon_sym_read); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 100 { state = 328; lexer.advance(false); continue; }
                if lookahead == 112 { state = 330; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            200 => {
                result = true; lexer.set_result_symbol(anon_sym_semop); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            201 => {
                result = true; lexer.set_result_symbol(anon_sym_splice); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            202 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT2); lexer.mark_end();
                return result;
            }
            203 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT2); lexer.mark_end();
                if lookahead == 61 { state = 132; lexer.advance(false); continue; }
                return result;
            }
            204 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP2); lexer.mark_end();
                return result;
            }
            205 => {
                result = true; lexer.set_result_symbol(anon_sym_AMP2); lexer.mark_end();
                if lookahead == 38 { state = 146; lexer.advance(false); continue; }
                if lookahead == 61 { state = 134; lexer.advance(false); continue; }
                return result;
            }
            206 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR2); lexer.mark_end();
                return result;
            }
            207 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR2); lexer.mark_end();
                if lookahead == 42 { state = 141; lexer.advance(false); continue; }
                if lookahead == 61 { state = 130; lexer.advance(false); continue; }
                return result;
            }
            208 => {
                result = true; lexer.set_result_symbol(aux_sym__var_indirob_autoquote_token1); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 { state = 208; lexer.advance(false); continue; }
                return result;
            }
            209 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ_GT); lexer.mark_end();
                return result;
            }
            210 => {
                result = true; lexer.set_result_symbol(anon_sym_use); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            211 => {
                result = true; lexer.set_result_symbol(anon_sym_no); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            212 => {
                result = true; lexer.set_result_symbol(anon_sym_redo); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            213 => {
                result = true; lexer.set_result_symbol(anon_sym_alarm); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            214 => {
                result = true; lexer.set_result_symbol(anon_sym_close); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 100 { state = 329; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            215 => {
                result = true; lexer.set_result_symbol(anon_sym_closedir); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            216 => {
                result = true; lexer.set_result_symbol(anon_sym_caller); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            217 => {
                result = true; lexer.set_result_symbol(anon_sym_cos); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            218 => {
                result = true; lexer.set_result_symbol(anon_sym_exp); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            219 => {
                result = true; lexer.set_result_symbol(anon_sym_lc); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            220 => {
                result = true; lexer.set_result_symbol(anon_sym_log); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            221 => {
                result = true; lexer.set_result_symbol(anon_sym_ord); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            222 => {
                result = true; lexer.set_result_symbol(anon_sym_pop); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            223 => {
                result = true; lexer.set_result_symbol(anon_sym_pos); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            224 => {
                result = true; lexer.set_result_symbol(anon_sym_rmdir); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            225 => {
                result = true; lexer.set_result_symbol(anon_sym_readdir); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            226 => {
                result = true; lexer.set_result_symbol(anon_sym_readpipe); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            227 => {
                result = true; lexer.set_result_symbol(anon_sym_scalar); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            228 => {
                result = true; lexer.set_result_symbol(anon_sym_sin); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            229 => {
                result = true; lexer.set_result_symbol(anon_sym_sleep); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            230 => {
                result = true; lexer.set_result_symbol(anon_sym_uc); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            231 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 { state = 231; lexer.advance(false); continue; }
                return result;
            }
            232 => {
                result = true; lexer.set_result_symbol(aux_sym___DATA___token1); lexer.mark_end();
                if lookahead == 35 { state = 231; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) && lookahead != 10 { state = 232; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) { state = 233; lexer.advance(false); continue; }
                return result;
            }
            233 => {
                result = true; lexer.set_result_symbol(aux_sym___DATA___token1); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 { state = 233; lexer.advance(false); continue; }
                return result;
            }
            234 => {
                result = true; lexer.set_result_symbol(anon_sym_EOT); lexer.mark_end();
                return result;
            }
            235 => {
                result = true; lexer.set_result_symbol(aux_sym__literal_token1); lexer.mark_end();
                if lookahead == 46 { state = 74; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 235; lexer.advance(false); continue; }
                return result;
            }
            236 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH_GT2); lexer.mark_end();
                return result;
            }
            237 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK2); lexer.mark_end();
                return result;
            }
            238 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACE2); lexer.mark_end();
                return result;
            }
            239 => {
                result = true; lexer.set_result_symbol(anon_sym_DOLLAR_STAR); lexer.mark_end();
                return result;
            }
            240 => {
                result = true; lexer.set_result_symbol(anon_sym_AT_STAR); lexer.mark_end();
                return result;
            }
            241 => {
                result = true; lexer.set_result_symbol(aux_sym__interpolation_fallbacks_token1); lexer.mark_end();
                return result;
            }
            242 => {
                result = true; lexer.set_result_symbol(aux_sym__interpolation_fallbacks_token1); lexer.mark_end();
                if lookahead == 35 { state = 231; lexer.advance(false); continue; }
                if lookahead == 94 { state = 244; lexer.advance(false); continue; }
                if lookahead == 123 { state = 82; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 242; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 242; lexer.advance(false); continue; }
                if set_contains(&sym__identifier_character_set_1, lookahead) && (lookahead < 65 || 90 < lookahead) && lookahead != 95 && (lookahead < 97 || 122 < lookahead) { state = 243; lexer.advance(false); continue; }
                if !eof && set_contains(&aux_sym__interpolation_fallbacks_token1_character_set_1, lookahead) { state = 241; lexer.advance(false); continue; }
                return result;
            }
            243 => {
                result = true; lexer.set_result_symbol(aux_sym__interpolation_fallbacks_token1); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            244 => {
                result = true; lexer.set_result_symbol(aux_sym__interpolation_fallbacks_token1); lexer.mark_end();
                if lookahead == 63 || 65 <= lookahead && lookahead <= 90 || lookahead == 94 || lookahead == 95 || lookahead == 124 { state = 391; lexer.advance(false); continue; }
                return result;
            }
            245 => {
                result = true; lexer.set_result_symbol(aux_sym__interpolation_fallbacks_token2); lexer.mark_end();
                return result;
            }
            246 => {
                result = true; lexer.set_result_symbol(aux_sym__interpolation_fallbacks_token2); lexer.mark_end();
                if lookahead == 35 { state = 231; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 246; lexer.advance(false); continue; }
                if lookahead != 0 && lookahead != 35 && lookahead != 36 && lookahead != 64 && lookahead != 91 && lookahead != 123 { state = 245; lexer.advance(false); continue; }
                return result;
            }
            247 => {
                result = true; lexer.set_result_symbol(anon_sym_m); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 97 { state = 351; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            248 => {
                result = true; lexer.set_result_symbol(anon_sym_s); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 99 { state = 293; lexer.advance(false); continue; }
                if lookahead == 101 { state = 341; lexer.advance(false); continue; }
                if lookahead == 105 { state = 342; lexer.advance(false); continue; }
                if lookahead == 108 { state = 323; lexer.advance(false); continue; }
                if lookahead == 112 { state = 336; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            249 => {
                result = true; lexer.set_result_symbol(sym_quoted_regexp_modifiers); lexer.mark_end();
                if lookahead == 97 { state = 266; lexer.advance(false); continue; }
                if lookahead == 100 || lookahead == 105 || 108 <= lookahead && lookahead <= 112 || lookahead == 115 || lookahead == 117 || lookahead == 120 { state = 266; lexer.advance(false); continue; }
                return result;
            }
            250 => {
                result = true; lexer.set_result_symbol(sym_quoted_regexp_modifiers); lexer.mark_end();
                if lookahead == 97 { state = 262; lexer.advance(false); continue; }
                if lookahead == 100 || lookahead == 105 || 108 <= lookahead && lookahead <= 112 || lookahead == 115 || lookahead == 117 || lookahead == 120 { state = 266; lexer.advance(false); continue; }
                return result;
            }
            251 => {
                result = true; lexer.set_result_symbol(sym_quoted_regexp_modifiers); lexer.mark_end();
                if lookahead == 100 { state = 266; lexer.advance(false); continue; }
                if lookahead == 97 || lookahead == 105 || 108 <= lookahead && lookahead <= 112 || lookahead == 115 || lookahead == 117 || lookahead == 120 { state = 266; lexer.advance(false); continue; }
                return result;
            }
            252 => {
                result = true; lexer.set_result_symbol(sym_quoted_regexp_modifiers); lexer.mark_end();
                if lookahead == 105 { state = 266; lexer.advance(false); continue; }
                if lookahead == 111 { state = 266; lexer.advance(false); continue; }
                if lookahead == 97 || lookahead == 100 || 108 <= lookahead && lookahead <= 112 || lookahead == 115 || lookahead == 117 || lookahead == 120 { state = 266; lexer.advance(false); continue; }
                return result;
            }
            253 => {
                result = true; lexer.set_result_symbol(sym_quoted_regexp_modifiers); lexer.mark_end();
                if lookahead == 105 { state = 266; lexer.advance(false); continue; }
                if lookahead == 97 || lookahead == 100 || 108 <= lookahead && lookahead <= 112 || lookahead == 115 || lookahead == 117 || lookahead == 120 { state = 266; lexer.advance(false); continue; }
                return result;
            }
            254 => {
                result = true; lexer.set_result_symbol(sym_quoted_regexp_modifiers); lexer.mark_end();
                if lookahead == 105 { state = 262; lexer.advance(false); continue; }
                if lookahead == 111 { state = 261; lexer.advance(false); continue; }
                if lookahead == 97 || lookahead == 100 || 108 <= lookahead && lookahead <= 112 || lookahead == 115 || lookahead == 117 || lookahead == 120 { state = 266; lexer.advance(false); continue; }
                return result;
            }
            255 => {
                result = true; lexer.set_result_symbol(sym_quoted_regexp_modifiers); lexer.mark_end();
                if lookahead == 105 { state = 259; lexer.advance(false); continue; }
                if lookahead == 108 { state = 266; lexer.advance(false); continue; }
                if lookahead == 112 { state = 256; lexer.advance(false); continue; }
                if lookahead == 97 || lookahead == 100 || 109 <= lookahead && lookahead <= 111 || lookahead == 115 || lookahead == 117 || lookahead == 120 { state = 266; lexer.advance(false); continue; }
                return result;
            }
            256 => {
                result = true; lexer.set_result_symbol(sym_quoted_regexp_modifiers); lexer.mark_end();
                if lookahead == 108 { state = 253; lexer.advance(false); continue; }
                if lookahead == 97 || lookahead == 100 || lookahead == 105 || 109 <= lookahead && lookahead <= 112 || lookahead == 115 || lookahead == 117 || lookahead == 120 { state = 266; lexer.advance(false); continue; }
                return result;
            }
            257 => {
                result = true; lexer.set_result_symbol(sym_quoted_regexp_modifiers); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (108, 249), (110, 251), (97, 266), (100, 266), (105, 266), (109, 266), (111, 266), (112, 266),
                    (115, 266), (117, 266), (120, 266),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            258 => {
                result = true; lexer.set_result_symbol(sym_quoted_regexp_modifiers); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (110, 251), (97, 266), (100, 266), (105, 266), (108, 266), (109, 266), (111, 266), (112, 266),
                    (115, 266), (117, 266), (120, 266),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            259 => {
                result = true; lexer.set_result_symbol(sym_quoted_regexp_modifiers); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (110, 266), (97, 266), (100, 266), (105, 266), (108, 266), (109, 266), (111, 266), (112, 266),
                    (115, 266), (117, 266), (120, 266),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            260 => {
                result = true; lexer.set_result_symbol(sym_quoted_regexp_modifiers); lexer.mark_end();
                if lookahead == 111 { state = 266; lexer.advance(false); continue; }
                if lookahead == 97 || lookahead == 100 || lookahead == 105 || 108 <= lookahead && lookahead <= 112 || lookahead == 115 || lookahead == 117 || lookahead == 120 { state = 266; lexer.advance(false); continue; }
                return result;
            }
            261 => {
                result = true; lexer.set_result_symbol(sym_quoted_regexp_modifiers); lexer.mark_end();
                if lookahead == 112 { state = 266; lexer.advance(false); continue; }
                if lookahead == 115 { state = 266; lexer.advance(false); continue; }
                if lookahead == 97 || lookahead == 100 || lookahead == 105 || 108 <= lookahead && lookahead <= 111 || lookahead == 117 || lookahead == 120 { state = 266; lexer.advance(false); continue; }
                return result;
            }
            262 => {
                result = true; lexer.set_result_symbol(sym_quoted_regexp_modifiers); lexer.mark_end();
                if lookahead == 112 { state = 266; lexer.advance(false); continue; }
                if lookahead == 97 || lookahead == 100 || lookahead == 105 || 108 <= lookahead && lookahead <= 111 || lookahead == 115 || lookahead == 117 || lookahead == 120 { state = 266; lexer.advance(false); continue; }
                return result;
            }
            263 => {
                result = true; lexer.set_result_symbol(sym_quoted_regexp_modifiers); lexer.mark_end();
                if lookahead == 115 { state = 266; lexer.advance(false); continue; }
                if lookahead == 97 || lookahead == 100 || lookahead == 105 || 108 <= lookahead && lookahead <= 112 || lookahead == 117 || lookahead == 120 { state = 266; lexer.advance(false); continue; }
                return result;
            }
            264 => {
                result = true; lexer.set_result_symbol(sym_quoted_regexp_modifiers); lexer.mark_end();
                if lookahead == 115 { state = 249; lexer.advance(false); continue; }
                if lookahead == 97 || lookahead == 100 || lookahead == 105 || 108 <= lookahead && lookahead <= 112 || lookahead == 117 || lookahead == 120 { state = 266; lexer.advance(false); continue; }
                return result;
            }
            265 => {
                result = true; lexer.set_result_symbol(sym_quoted_regexp_modifiers); lexer.mark_end();
                if lookahead == 117 { state = 266; lexer.advance(false); continue; }
                if lookahead == 97 || lookahead == 100 || lookahead == 105 || 108 <= lookahead && lookahead <= 112 || lookahead == 115 || lookahead == 120 { state = 266; lexer.advance(false); continue; }
                return result;
            }
            266 => {
                result = true; lexer.set_result_symbol(sym_quoted_regexp_modifiers); lexer.mark_end();
                if lookahead == 97 || lookahead == 100 || lookahead == 105 || 108 <= lookahead && lookahead <= 112 || lookahead == 115 || lookahead == 117 || lookahead == 120 { state = 266; lexer.advance(false); continue; }
                return result;
            }
            267 => {
                result = true; lexer.set_result_symbol(sym_match_regexp_modifiers); lexer.mark_end();
                if lookahead == 97 { state = 274; lexer.advance(false); continue; }
                if lookahead == 99 || lookahead == 100 || lookahead == 103 || lookahead == 105 || 108 <= lookahead && lookahead <= 112 || lookahead == 115 || lookahead == 117 || lookahead == 120 { state = 274; lexer.advance(false); continue; }
                return result;
            }
            268 => {
                result = true; lexer.set_result_symbol(sym_match_regexp_modifiers); lexer.mark_end();
                if lookahead == 100 { state = 274; lexer.advance(false); continue; }
                if (set_contains(&sym_substitution_regexp_modifiers_character_set_1, lookahead) || lookahead == 110) && lookahead != 100 && lookahead != 101 && lookahead != 114 { state = 274; lexer.advance(false); continue; }
                return result;
            }
            269 => {
                result = true; lexer.set_result_symbol(sym_match_regexp_modifiers); lexer.mark_end();
                if lookahead == 109 { state = 272; lexer.advance(false); continue; }
                if (set_contains(&sym_substitution_regexp_modifiers_character_set_1, lookahead) || lookahead == 110) && lookahead != 101 && lookahead != 114 { state = 274; lexer.advance(false); continue; }
                return result;
            }
            270 => {
                result = true; lexer.set_result_symbol(sym_match_regexp_modifiers); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (110, 268), (97, 274), (99, 274), (100, 274), (103, 274), (105, 274), (108, 274), (109, 274),
                    (111, 274), (112, 274), (115, 274), (117, 274), (120, 274),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            271 => {
                result = true; lexer.set_result_symbol(sym_match_regexp_modifiers); lexer.mark_end();
                if lookahead == 111 { state = 274; lexer.advance(false); continue; }
                if (set_contains(&sym_substitution_regexp_modifiers_character_set_1, lookahead) || lookahead == 110) && lookahead != 101 && lookahead != 114 { state = 274; lexer.advance(false); continue; }
                return result;
            }
            272 => {
                result = true; lexer.set_result_symbol(sym_match_regexp_modifiers); lexer.mark_end();
                if lookahead == 112 { state = 274; lexer.advance(false); continue; }
                if (set_contains(&sym_substitution_regexp_modifiers_character_set_1, lookahead) || lookahead == 110) && lookahead != 101 && lookahead != 114 { state = 274; lexer.advance(false); continue; }
                return result;
            }
            273 => {
                result = true; lexer.set_result_symbol(sym_match_regexp_modifiers); lexer.mark_end();
                if lookahead == 115 { state = 267; lexer.advance(false); continue; }
                if lookahead == 97 || lookahead == 99 || lookahead == 100 || lookahead == 103 || lookahead == 105 || 108 <= lookahead && lookahead <= 112 || lookahead == 117 || lookahead == 120 { state = 274; lexer.advance(false); continue; }
                return result;
            }
            274 => {
                result = true; lexer.set_result_symbol(sym_match_regexp_modifiers); lexer.mark_end();
                if (set_contains(&sym_substitution_regexp_modifiers_character_set_1, lookahead) || lookahead == 110) && lookahead != 101 && lookahead != 114 { state = 274; lexer.advance(false); continue; }
                return result;
            }
            275 => {
                result = true; lexer.set_result_symbol(sym_substitution_regexp_modifiers); lexer.mark_end();
                if lookahead == 97 { state = 282; lexer.advance(false); continue; }
                if set_contains(&sym_substitution_regexp_modifiers_character_set_1, lookahead) { state = 282; lexer.advance(false); continue; }
                return result;
            }
            276 => {
                result = true; lexer.set_result_symbol(sym_substitution_regexp_modifiers); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (101, 282), (97, 282), (99, 282), (100, 282), (103, 282), (105, 282), (108, 282), (109, 282),
                    (111, 282), (112, 282), (114, 282), (115, 282), (117, 282), (120, 282),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            277 => {
                result = true; lexer.set_result_symbol(sym_substitution_regexp_modifiers); lexer.mark_end();
                if lookahead == 109 { state = 279; lexer.advance(false); continue; }
                if set_contains(&sym_substitution_regexp_modifiers_character_set_1, lookahead) { state = 282; lexer.advance(false); continue; }
                return result;
            }
            278 => {
                result = true; lexer.set_result_symbol(sym_substitution_regexp_modifiers); lexer.mark_end();
                if lookahead == 111 { state = 280; lexer.advance(false); continue; }
                if set_contains(&sym_substitution_regexp_modifiers_character_set_1, lookahead) { state = 282; lexer.advance(false); continue; }
                return result;
            }
            279 => {
                result = true; lexer.set_result_symbol(sym_substitution_regexp_modifiers); lexer.mark_end();
                if lookahead == 112 { state = 282; lexer.advance(false); continue; }
                if set_contains(&sym_substitution_regexp_modifiers_character_set_1, lookahead) { state = 282; lexer.advance(false); continue; }
                return result;
            }
            280 => {
                result = true; lexer.set_result_symbol(sym_substitution_regexp_modifiers); lexer.mark_end();
                if lookahead == 114 { state = 282; lexer.advance(false); continue; }
                if set_contains(&sym_substitution_regexp_modifiers_character_set_1, lookahead) { state = 282; lexer.advance(false); continue; }
                return result;
            }
            281 => {
                result = true; lexer.set_result_symbol(sym_substitution_regexp_modifiers); lexer.mark_end();
                if lookahead == 115 { state = 275; lexer.advance(false); continue; }
                if set_contains(&sym_substitution_regexp_modifiers_character_set_1, lookahead) { state = 282; lexer.advance(false); continue; }
                return result;
            }
            282 => {
                result = true; lexer.set_result_symbol(sym_substitution_regexp_modifiers); lexer.mark_end();
                if set_contains(&sym_substitution_regexp_modifiers_character_set_1, lookahead) { state = 282; lexer.advance(false); continue; }
                return result;
            }
            283 => {
                result = true; lexer.set_result_symbol(sym_transliteration_modifiers); lexer.mark_end();
                if lookahead == 99 || lookahead == 100 || lookahead == 114 || lookahead == 115 { state = 283; lexer.advance(false); continue; }
                return result;
            }
            284 => {
                result = true; lexer.set_result_symbol(aux_sym__interpolated_transliteration_content_token1); lexer.mark_end();
                return result;
            }
            285 => {
                result = true; lexer.set_result_symbol(aux_sym__interpolated_transliteration_content_token1); lexer.mark_end();
                if lookahead == 35 { state = 231; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) && lookahead != 10 { state = 285; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) { state = 284; lexer.advance(false); continue; }
                return result;
            }
            286 => {
                result = true; lexer.set_result_symbol(sym_version); lexer.mark_end();
                if lookahead == 46 { state = 70; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 286; lexer.advance(false); continue; }
                return result;
            }
            287 => {
                result = true; lexer.set_result_symbol(sym_version); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 { state = 287; lexer.advance(false); continue; }
                return result;
            }
            288 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 46 { state = 74; lexer.advance(false); continue; }
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 288; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            289 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 97 { state = 334; lexer.advance(false); continue; }
                if lookahead == 108 { state = 348; lexer.advance(false); continue; }
                if lookahead == 109 { state = 349; lexer.advance(false); continue; }
                if lookahead == 111 { state = 369; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            290 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 97 { state = 334; lexer.advance(false); continue; }
                if lookahead == 108 { state = 348; lexer.advance(false); continue; }
                if lookahead == 111 { state = 369; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            291 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 97 { state = 181; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            292 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 97 { state = 305; lexer.advance(false); continue; }
                if lookahead == 100 { state = 345; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            293 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 97 { state = 338; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            294 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 97 { state = 367; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            295 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 97 { state = 333; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            296 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 97 { state = 364; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            297 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 99 { state = 219; lexer.advance(false); continue; }
                if lookahead == 101 { state = 177; lexer.advance(false); continue; }
                if lookahead == 111 { state = 303; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            298 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 99 { state = 219; lexer.advance(false); continue; }
                if lookahead == 111 { state = 303; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            299 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 99 { state = 230; lexer.advance(false); continue; }
                if lookahead == 115 { state = 315; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            300 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 99 { state = 230; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            301 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 99 { state = 196; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            302 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 99 { state = 318; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            303 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 99 { state = 295; lexer.advance(false); continue; }
                if lookahead == 103 { state = 220; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            304 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 100 { state = 326; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            305 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 100 { state = 199; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            306 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 100 { state = 105; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            307 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 100 { state = 221; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            308 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 101 { state = 179; lexer.advance(false); continue; }
                if lookahead == 114 { state = 320; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            309 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 101 { state = 179; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            310 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 101 { state = 292; lexer.advance(false); continue; }
                if lookahead == 109 { state = 304; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            311 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 101 { state = 301; lexer.advance(false); continue; }
                if lookahead == 112 { state = 218; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            312 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 101 { state = 177; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            313 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 101 { state = 103; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            314 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 101 { state = 197; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            315 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 101 { state = 210; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            316 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 101 { state = 214; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            317 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 101 { state = 198; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            318 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 101 { state = 201; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            319 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 101 { state = 226; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            320 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 101 { state = 350; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            321 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 101 { state = 362; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            322 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 101 { state = 354; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            323 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 101 { state = 322; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            324 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 105 { state = 302; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            325 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 105 { state = 314; lexer.advance(false); continue; }
                if lookahead == 111 { state = 191; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            326 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 105 { state = 361; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            327 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 105 { state = 355; lexer.advance(false); continue; }
                if lookahead == 111 { state = 352; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            328 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 105 { state = 363; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            329 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 105 { state = 365; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            330 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 105 { state = 356; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            331 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 108 { state = 294; lexer.advance(false); continue; }
                if lookahead == 110 { state = 306; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            332 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 108 { state = 294; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            333 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 108 { state = 192; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            334 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 108 { state = 337; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            335 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 108 { state = 370; lexer.advance(false); continue; }
                if lookahead == 120 { state = 311; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            336 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 108 { state = 324; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            337 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 108 { state = 321; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            338 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 108 { state = 296; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            339 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 109 { state = 349; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            340 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 109 { state = 213; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            341 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 109 { state = 347; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            342 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 110 { state = 228; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            343 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 110 { state = 306; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            344 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 111 { state = 211; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            345 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 111 { state = 212; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            346 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 111 { state = 360; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            347 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 111 { state = 353; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            348 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 111 { state = 372; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            349 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 112 { state = 171; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            350 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 112 { state = 195; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            351 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 112 { state = 194; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            352 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 112 { state = 222; lexer.advance(false); continue; }
                if lookahead == 115 { state = 223; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            353 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 112 { state = 200; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            354 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 112 { state = 229; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            355 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 112 { state = 317; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            356 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 112 { state = 319; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            357 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 114 { state = 320; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            358 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 114 { state = 108; lexer.advance(false); continue; }
                if lookahead == 117 { state = 359; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            359 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 114 { state = 99; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            360 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 114 { state = 112; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            361 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 114 { state = 224; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            362 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 114 { state = 216; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            363 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 114 { state = 225; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            364 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 114 { state = 227; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            365 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 114 { state = 215; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            366 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 114 { state = 109; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            367 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 114 { state = 340; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            368 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 114 { state = 307; lexer.advance(false); continue; }
                if lookahead == 117 { state = 359; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            369 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 115 { state = 217; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            370 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 115 { state = 313; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            371 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 115 { state = 291; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            372 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 115 { state = 316; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            373 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if lookahead == 120 { state = 311; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            374 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 286; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            375 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 288; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            376 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 376; lexer.advance(false); continue; }
                return result;
            }
            377 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 97 { state = 182; lexer.advance(false); continue; }
                if set_contains(&sym__identifier_character_set_2, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            378 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 100 { state = 106; lexer.advance(false); continue; }
                if set_contains(&sym__identifier_character_set_2, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            379 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 101 { state = 180; lexer.advance(false); continue; }
                if set_contains(&sym__identifier_character_set_2, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            380 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 101 { state = 178; lexer.advance(false); continue; }
                if set_contains(&sym__identifier_character_set_2, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            381 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 109 { state = 384; lexer.advance(false); continue; }
                if set_contains(&sym__identifier_character_set_2, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            382 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 110 { state = 378; lexer.advance(false); continue; }
                if set_contains(&sym__identifier_character_set_2, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            383 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 111 { state = 386; lexer.advance(false); continue; }
                if set_contains(&sym__identifier_character_set_2, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            384 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 112 { state = 172; lexer.advance(false); continue; }
                if set_contains(&sym__identifier_character_set_2, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            385 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 114 { state = 110; lexer.advance(false); continue; }
                if set_contains(&sym__identifier_character_set_2, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            386 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 114 { state = 113; lexer.advance(false); continue; }
                if set_contains(&sym__identifier_character_set_2, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            387 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 114 { state = 100; lexer.advance(false); continue; }
                if set_contains(&sym__identifier_character_set_2, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            388 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 115 { state = 377; lexer.advance(false); continue; }
                if set_contains(&sym__identifier_character_set_2, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            389 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 117 { state = 387; lexer.advance(false); continue; }
                if set_contains(&sym__identifier_character_set_2, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            390 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if set_contains(&sym__identifier_character_set_2, lookahead) { state = 390; lexer.advance(false); continue; }
                return result;
            }
            391 => {
                result = true; lexer.set_result_symbol(sym__special_var_name); lexer.mark_end();
                return result;
            }
            392 => {
                result = true; lexer.set_result_symbol(sym__special_var_name); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (33, 185), (35, 231), (36, 90), (37, 202), (38, 204), (40, 101), (42, 206), (43, 152),
                    (45, 155), (46, 397), (47, 400), (48, 398), (58, 401), (59, 86), (60, 402), (64, 95),
                    (91, 117), (92, 190), (94, 405), (97, 332), (99, 290), (100, 325), (101, 373), (103, 357),
                    (108, 298), (109, 247), (110, 344), (111, 368), (112, 327), (114, 310), (115, 248), (117, 299),
                    (118, 375), (123, 82), (125, 83), (126, 183),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 399; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) && (lookahead < 9 || 13 < lookahead) && lookahead != 32 { state = 392; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 376; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 32 && lookahead != 33 { state = 391; lexer.advance(false); continue; }
                return result;
            }
            393 => {
                result = true; lexer.set_result_symbol(sym__special_var_name); lexer.mark_end();
                if lookahead == 35 { state = 231; lexer.advance(false); continue; }
                if lookahead == 36 { state = 89; lexer.advance(false); continue; }
                if lookahead == 42 { state = 115; lexer.advance(false); continue; }
                if lookahead == 58 { state = 401; lexer.advance(false); continue; }
                if lookahead == 94 { state = 407; lexer.advance(false); continue; }
                if lookahead == 123 { state = 82; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 406; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) && (lookahead < 9 || 13 < lookahead) && lookahead != 32 { state = 393; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 376; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 32 { state = 391; lexer.advance(false); continue; }
                return result;
            }
            394 => {
                result = true; lexer.set_result_symbol(sym__special_var_name); lexer.mark_end();
                if lookahead == 35 { state = 231; lexer.advance(false); continue; }
                if lookahead == 36 { state = 89; lexer.advance(false); continue; }
                if lookahead == 58 { state = 401; lexer.advance(false); continue; }
                if lookahead == 94 { state = 407; lexer.advance(false); continue; }
                if lookahead == 123 { state = 82; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 406; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) && (lookahead < 9 || 13 < lookahead) && lookahead != 32 { state = 394; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 376; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 32 { state = 391; lexer.advance(false); continue; }
                return result;
            }
            395 => {
                result = true; lexer.set_result_symbol(sym__special_var_name); lexer.mark_end();
                if lookahead == 35 { state = 231; lexer.advance(false); continue; }
                if lookahead == 36 { state = 89; lexer.advance(false); continue; }
                if lookahead == 94 { state = 407; lexer.advance(false); continue; }
                if lookahead == 123 { state = 82; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 406; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) && (lookahead < 9 || 13 < lookahead) && lookahead != 32 { state = 395; lexer.advance(false); continue; }
                if set_contains(&sym__identifier_character_set_1, lookahead) { state = 390; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 32 { state = 391; lexer.advance(false); continue; }
                return result;
            }
            396 => {
                result = true; lexer.set_result_symbol(sym__special_var_name); lexer.mark_end();
                if lookahead == 35 { state = 231; lexer.advance(false); continue; }
                if lookahead == 58 { state = 401; lexer.advance(false); continue; }
                if lookahead == 94 { state = 405; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 406; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) && (lookahead < 9 || 13 < lookahead) && lookahead != 32 { state = 396; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 376; lexer.advance(false); continue; }
                if lookahead != 0 && (lookahead < 9 || 13 < lookahead) && lookahead != 32 { state = 391; lexer.advance(false); continue; }
                return result;
            }
            397 => {
                result = true; lexer.set_result_symbol(sym__special_var_name); lexer.mark_end();
                if lookahead == 46 { state = 47; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 421; lexer.advance(false); continue; }
                return result;
            }
            398 => {
                result = true; lexer.set_result_symbol(sym__special_var_name); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (46, 422), (95, 71), (66, 67), (98, 67), (69, 66), (101, 66), (88, 76), (120, 76),
                    (56, 403), (57, 403),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 55 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            399 => {
                result = true; lexer.set_result_symbol(sym__special_var_name); lexer.mark_end();
                if lookahead == 46 { state = 422; lexer.advance(false); continue; }
                if lookahead == 95 { state = 75; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 66; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 399; lexer.advance(false); continue; }
                return result;
            }
            400 => {
                result = true; lexer.set_result_symbol(sym__special_var_name); lexer.mark_end();
                if lookahead == 47 { state = 143; lexer.advance(false); continue; }
                return result;
            }
            401 => {
                result = true; lexer.set_result_symbol(sym__special_var_name); lexer.mark_end();
                if lookahead == 58 { state = 408; lexer.advance(false); continue; }
                return result;
            }
            402 => {
                result = true; lexer.set_result_symbol(sym__special_var_name); lexer.mark_end();
                if lookahead == 60 { state = 123; lexer.advance(false); continue; }
                return result;
            }
            403 => {
                result = true; lexer.set_result_symbol(sym__special_var_name); lexer.mark_end();
                if lookahead == 95 { state = 71; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 403; lexer.advance(false); continue; }
                return result;
            }
            404 => {
                result = true; lexer.set_result_symbol(sym__special_var_name); lexer.mark_end();
                if lookahead == 95 { state = 68; lexer.advance(false); continue; }
                if lookahead == 56 || lookahead == 57 { state = 403; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 55 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            405 => {
                result = true; lexer.set_result_symbol(sym__special_var_name); lexer.mark_end();
                if lookahead == 63 || lookahead == 94 || lookahead == 124 { state = 391; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 { state = 208; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 97 <= lookahead && lookahead <= 122 { state = 208; lexer.advance(false); continue; }
                return result;
            }
            406 => {
                result = true; lexer.set_result_symbol(sym__special_var_name); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 { state = 406; lexer.advance(false); continue; }
                return result;
            }
            407 => {
                result = true; lexer.set_result_symbol(sym__special_var_name); lexer.mark_end();
                if lookahead == 63 || 65 <= lookahead && lookahead <= 90 || lookahead == 94 || lookahead == 95 || lookahead == 124 { state = 391; lexer.advance(false); continue; }
                return result;
            }
            408 => {
                result = true; lexer.set_result_symbol(aux_sym__bareword_token1); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_1, lookahead) { state = 409; lexer.advance(false); continue; }
                return result;
            }
            409 => {
                result = true; lexer.set_result_symbol(aux_sym__bareword_token1); lexer.mark_end();
                if lookahead == 58 { state = 52; lexer.advance(false); continue; }
                if set_contains(&aux_sym__bareword_token1_character_set_2, lookahead) { state = 409; lexer.advance(false); continue; }
                return result;
            }
            410 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (46, 423), (95, 71), (66, 67), (98, 67), (69, 66), (101, 66), (88, 76), (120, 76),
                    (56, 414), (57, 414),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 55 { state = 415; lexer.advance(false); continue; }
                return result;
            }
            411 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if lookahead == 46 { state = 423; lexer.advance(false); continue; }
                if lookahead == 95 { state = 75; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 66; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 411; lexer.advance(false); continue; }
                return result;
            }
            412 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (46, 422), (95, 71), (66, 67), (98, 67), (69, 66), (101, 66), (88, 76), (120, 76),
                    (56, 417), (57, 417),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 55 { state = 420; lexer.advance(false); continue; }
                return result;
            }
            413 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if lookahead == 46 { state = 422; lexer.advance(false); continue; }
                if lookahead == 95 { state = 75; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 66; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 413; lexer.advance(false); continue; }
                return result;
            }
            414 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if lookahead == 46 { state = 72; lexer.advance(false); continue; }
                if lookahead == 95 { state = 71; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 414; lexer.advance(false); continue; }
                return result;
            }
            415 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if lookahead == 46 { state = 72; lexer.advance(false); continue; }
                if lookahead == 95 { state = 68; lexer.advance(false); continue; }
                if lookahead == 56 || lookahead == 57 { state = 414; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 55 { state = 415; lexer.advance(false); continue; }
                return result;
            }
            416 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if lookahead == 46 { state = 73; lexer.advance(false); continue; }
                if lookahead == 95 { state = 69; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 66; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 416; lexer.advance(false); continue; }
                return result;
            }
            417 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if lookahead == 95 { state = 71; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 417; lexer.advance(false); continue; }
                return result;
            }
            418 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if lookahead == 95 { state = 67; lexer.advance(false); continue; }
                if lookahead == 48 || lookahead == 49 { state = 418; lexer.advance(false); continue; }
                return result;
            }
            419 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if lookahead == 95 { state = 76; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 419; lexer.advance(false); continue; }
                return result;
            }
            420 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if lookahead == 95 { state = 68; lexer.advance(false); continue; }
                if lookahead == 56 || lookahead == 57 { state = 417; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 55 { state = 420; lexer.advance(false); continue; }
                return result;
            }
            421 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if lookahead == 95 { state = 69; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 66; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 421; lexer.advance(false); continue; }
                return result;
            }
            422 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if lookahead == 69 || lookahead == 101 { state = 66; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 421; lexer.advance(false); continue; }
                return result;
            }
            423 => {
                result = true; lexer.set_result_symbol(sym_number); lexer.mark_end();
                if lookahead == 69 || lookahead == 101 { state = 66; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 416; lexer.advance(false); continue; }
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
                    (65, 1), (66, 2), (67, 3), (69, 4), (73, 5), (83, 6), (85, 7), (95, 8),
                    (97, 9), (98, 10), (99, 11), (100, 12), (101, 13), (102, 14), (103, 15), (104, 16),
                    (105, 17), (106, 18), (107, 19), (108, 20), (109, 21), (110, 22), (111, 23), (112, 24),
                    (113, 25), (114, 26), (115, 27), (116, 28), (117, 29), (118, 30), (119, 31), (121, 32),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if set_contains(&extras_character_set_1, lookahead) { state = 0; lexer.advance(true); continue; }
                return result;
            }
            1 => {
                if lookahead == 68 { state = 33; lexer.advance(false); continue; }
                return result;
            }
            2 => {
                if lookahead == 69 { state = 34; lexer.advance(false); continue; }
                if lookahead == 85 { state = 35; lexer.advance(false); continue; }
                return result;
            }
            3 => {
                if lookahead == 72 { state = 36; lexer.advance(false); continue; }
                return result;
            }
            4 => {
                if lookahead == 78 { state = 37; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                if lookahead == 78 { state = 38; lexer.advance(false); continue; }
                return result;
            }
            6 => {
                if lookahead == 84 { state = 39; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if lookahead == 78 { state = 40; lexer.advance(false); continue; }
                return result;
            }
            8 => {
                if lookahead == 95 { state = 41; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if lookahead == 98 { state = 42; lexer.advance(false); continue; }
                if lookahead == 99 { state = 43; lexer.advance(false); continue; }
                if lookahead == 116 { state = 44; lexer.advance(false); continue; }
                if lookahead == 119 { state = 45; lexer.advance(false); continue; }
                return result;
            }
            10 => {
                if lookahead == 105 { state = 46; lexer.advance(false); continue; }
                if lookahead == 108 { state = 47; lexer.advance(false); continue; }
                if lookahead == 114 { state = 48; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if lookahead == 97 { state = 49; lexer.advance(false); continue; }
                if lookahead == 104 { state = 50; lexer.advance(false); continue; }
                if lookahead == 111 { state = 51; lexer.advance(false); continue; }
                if lookahead == 114 { state = 52; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if lookahead == 98 { state = 53; lexer.advance(false); continue; }
                if lookahead == 101 { state = 54; lexer.advance(false); continue; }
                if lookahead == 121 { state = 55; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if lookahead == 97 { state = 56; lexer.advance(false); continue; }
                if lookahead == 108 { state = 57; lexer.advance(false); continue; }
                if lookahead == 111 { state = 58; lexer.advance(false); continue; }
                if lookahead == 113 { state = 59; lexer.advance(false); continue; }
                if lookahead == 118 { state = 60; lexer.advance(false); continue; }
                if lookahead == 120 { state = 61; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if lookahead == 97 { state = 62; lexer.advance(false); continue; }
                if lookahead == 99 { state = 63; lexer.advance(false); continue; }
                if lookahead == 105 { state = 64; lexer.advance(false); continue; }
                if lookahead == 108 { state = 65; lexer.advance(false); continue; }
                if lookahead == 111 { state = 66; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if lookahead == 101 { state = 67; lexer.advance(false); continue; }
                if lookahead == 108 { state = 68; lexer.advance(false); continue; }
                if lookahead == 109 { state = 69; lexer.advance(false); continue; }
                if lookahead == 111 { state = 70; lexer.advance(false); continue; }
                if lookahead == 116 { state = 71; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if lookahead == 101 { state = 72; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if lookahead == 102 { state = 73; lexer.advance(false); continue; }
                if lookahead == 110 { state = 74; lexer.advance(false); continue; }
                if lookahead == 111 { state = 75; lexer.advance(false); continue; }
                return result;
            }
            18 => {
                if lookahead == 111 { state = 76; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if lookahead == 101 { state = 77; lexer.advance(false); continue; }
                if lookahead == 105 { state = 78; lexer.advance(false); continue; }
                return result;
            }
            20 => {
                if lookahead == 97 { state = 79; lexer.advance(false); continue; }
                if lookahead == 99 { state = 80; lexer.advance(false); continue; }
                if lookahead == 101 { state = 81; lexer.advance(false); continue; }
                if lookahead == 105 { state = 82; lexer.advance(false); continue; }
                if lookahead == 111 { state = 83; lexer.advance(false); continue; }
                if lookahead == 115 { state = 84; lexer.advance(false); continue; }
                if lookahead == 116 { state = 85; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                if lookahead == 107 { state = 86; lexer.advance(false); continue; }
                if lookahead == 115 { state = 87; lexer.advance(false); continue; }
                if lookahead == 121 { state = 88; lexer.advance(false); continue; }
                return result;
            }
            22 => {
                if lookahead == 101 { state = 89; lexer.advance(false); continue; }
                if lookahead == 111 { state = 90; lexer.advance(false); continue; }
                return result;
            }
            23 => {
                if lookahead == 99 { state = 91; lexer.advance(false); continue; }
                if lookahead == 112 { state = 92; lexer.advance(false); continue; }
                return result;
            }
            24 => {
                if lookahead == 97 { state = 93; lexer.advance(false); continue; }
                if lookahead == 114 { state = 94; lexer.advance(false); continue; }
                if lookahead == 117 { state = 95; lexer.advance(false); continue; }
                return result;
            }
            25 => {
                result = true; lexer.set_result_symbol(anon_sym_q); lexer.mark_end();
                if lookahead == 113 { state = 96; lexer.advance(false); continue; }
                if lookahead == 114 { state = 97; lexer.advance(false); continue; }
                if lookahead == 117 { state = 98; lexer.advance(false); continue; }
                if lookahead == 119 { state = 99; lexer.advance(false); continue; }
                if lookahead == 120 { state = 100; lexer.advance(false); continue; }
                return result;
            }
            26 => {
                if lookahead == 97 { state = 101; lexer.advance(false); continue; }
                if lookahead == 101 { state = 102; lexer.advance(false); continue; }
                if lookahead == 105 { state = 103; lexer.advance(false); continue; }
                return result;
            }
            27 => {
                if let Some(next) = advance_map(&[
                    (97, 104), (101, 105), (104, 106), (111, 107), (112, 108), (113, 109), (114, 110), (116, 111),
                    (117, 112), (121, 113),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            28 => {
                if lookahead == 101 { state = 114; lexer.advance(false); continue; }
                if lookahead == 105 { state = 115; lexer.advance(false); continue; }
                if lookahead == 114 { state = 116; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                if lookahead == 99 { state = 117; lexer.advance(false); continue; }
                if lookahead == 109 { state = 118; lexer.advance(false); continue; }
                if lookahead == 110 { state = 119; lexer.advance(false); continue; }
                if lookahead == 116 { state = 120; lexer.advance(false); continue; }
                return result;
            }
            30 => {
                if lookahead == 97 { state = 121; lexer.advance(false); continue; }
                if lookahead == 101 { state = 122; lexer.advance(false); continue; }
                return result;
            }
            31 => {
                if lookahead == 97 { state = 123; lexer.advance(false); continue; }
                if lookahead == 104 { state = 124; lexer.advance(false); continue; }
                if lookahead == 114 { state = 125; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                result = true; lexer.set_result_symbol(anon_sym_y); lexer.mark_end();
                return result;
            }
            33 => {
                if lookahead == 74 { state = 126; lexer.advance(false); continue; }
                return result;
            }
            34 => {
                if lookahead == 71 { state = 127; lexer.advance(false); continue; }
                return result;
            }
            35 => {
                if lookahead == 73 { state = 128; lexer.advance(false); continue; }
                return result;
            }
            36 => {
                if lookahead == 69 { state = 129; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                if lookahead == 68 { state = 130; lexer.advance(false); continue; }
                return result;
            }
            38 => {
                if lookahead == 73 { state = 131; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                if lookahead == 68 { state = 132; lexer.advance(false); continue; }
                return result;
            }
            40 => {
                if lookahead == 73 { state = 133; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                if lookahead == 68 { state = 134; lexer.advance(false); continue; }
                if lookahead == 69 { state = 135; lexer.advance(false); continue; }
                if lookahead == 70 { state = 136; lexer.advance(false); continue; }
                if lookahead == 76 { state = 137; lexer.advance(false); continue; }
                if lookahead == 80 { state = 138; lexer.advance(false); continue; }
                if lookahead == 83 { state = 139; lexer.advance(false); continue; }
                return result;
            }
            42 => {
                if lookahead == 115 { state = 140; lexer.advance(false); continue; }
                return result;
            }
            43 => {
                if lookahead == 99 { state = 141; lexer.advance(false); continue; }
                return result;
            }
            44 => {
                if lookahead == 97 { state = 142; lexer.advance(false); continue; }
                return result;
            }
            45 => {
                if lookahead == 97 { state = 143; lexer.advance(false); continue; }
                return result;
            }
            46 => {
                if lookahead == 110 { state = 144; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                if lookahead == 101 { state = 145; lexer.advance(false); continue; }
                return result;
            }
            48 => {
                if lookahead == 101 { state = 146; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                if lookahead == 116 { state = 147; lexer.advance(false); continue; }
                return result;
            }
            50 => {
                if lookahead == 100 { state = 148; lexer.advance(false); continue; }
                if lookahead == 109 { state = 149; lexer.advance(false); continue; }
                if lookahead == 111 { state = 150; lexer.advance(false); continue; }
                if lookahead == 114 { state = 151; lexer.advance(false); continue; }
                return result;
            }
            51 => {
                if lookahead == 110 { state = 152; lexer.advance(false); continue; }
                return result;
            }
            52 => {
                if lookahead == 121 { state = 153; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                if lookahead == 109 { state = 154; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                if lookahead == 102 { state = 155; lexer.advance(false); continue; }
                if lookahead == 108 { state = 156; lexer.advance(false); continue; }
                return result;
            }
            55 => {
                if lookahead == 110 { state = 157; lexer.advance(false); continue; }
                return result;
            }
            56 => {
                if lookahead == 99 { state = 158; lexer.advance(false); continue; }
                return result;
            }
            57 => {
                if lookahead == 115 { state = 159; lexer.advance(false); continue; }
                return result;
            }
            58 => {
                if lookahead == 102 { state = 160; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                result = true; lexer.set_result_symbol(anon_sym_eq); lexer.mark_end();
                if lookahead == 114 { state = 161; lexer.advance(false); continue; }
                if lookahead == 117 { state = 162; lexer.advance(false); continue; }
                return result;
            }
            60 => {
                if lookahead == 97 { state = 163; lexer.advance(false); continue; }
                return result;
            }
            61 => {
                if lookahead == 105 { state = 164; lexer.advance(false); continue; }
                if lookahead == 116 { state = 165; lexer.advance(false); continue; }
                return result;
            }
            62 => {
                if lookahead == 108 { state = 166; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                result = true; lexer.set_result_symbol(anon_sym_fc); lexer.mark_end();
                if lookahead == 110 { state = 167; lexer.advance(false); continue; }
                return result;
            }
            64 => {
                if lookahead == 101 { state = 168; lexer.advance(false); continue; }
                if lookahead == 108 { state = 169; lexer.advance(false); continue; }
                if lookahead == 110 { state = 170; lexer.advance(false); continue; }
                return result;
            }
            65 => {
                if lookahead == 111 { state = 171; lexer.advance(false); continue; }
                return result;
            }
            66 => {
                if lookahead == 114 { state = 172; lexer.advance(false); continue; }
                return result;
            }
            67 => {
                if lookahead == 116 { state = 173; lexer.advance(false); continue; }
                return result;
            }
            68 => {
                if lookahead == 111 { state = 174; lexer.advance(false); continue; }
                return result;
            }
            69 => {
                if lookahead == 116 { state = 175; lexer.advance(false); continue; }
                return result;
            }
            70 => {
                if lookahead == 116 { state = 176; lexer.advance(false); continue; }
                return result;
            }
            71 => {
                result = true; lexer.set_result_symbol(anon_sym_gt); lexer.mark_end();
                return result;
            }
            72 => {
                if lookahead == 120 { state = 177; lexer.advance(false); continue; }
                return result;
            }
            73 => {
                result = true; lexer.set_result_symbol(anon_sym_if); lexer.mark_end();
                return result;
            }
            74 => {
                if lookahead == 100 { state = 178; lexer.advance(false); continue; }
                if lookahead == 116 { state = 179; lexer.advance(false); continue; }
                return result;
            }
            75 => {
                if lookahead == 99 { state = 180; lexer.advance(false); continue; }
                return result;
            }
            76 => {
                if lookahead == 105 { state = 181; lexer.advance(false); continue; }
                return result;
            }
            77 => {
                if lookahead == 121 { state = 182; lexer.advance(false); continue; }
                return result;
            }
            78 => {
                if lookahead == 108 { state = 183; lexer.advance(false); continue; }
                return result;
            }
            79 => {
                if lookahead == 115 { state = 184; lexer.advance(false); continue; }
                return result;
            }
            80 => {
                if lookahead == 102 { state = 185; lexer.advance(false); continue; }
                return result;
            }
            81 => {
                if lookahead == 110 { state = 186; lexer.advance(false); continue; }
                return result;
            }
            82 => {
                if lookahead == 110 { state = 187; lexer.advance(false); continue; }
                if lookahead == 115 { state = 188; lexer.advance(false); continue; }
                return result;
            }
            83 => {
                if lookahead == 99 { state = 189; lexer.advance(false); continue; }
                return result;
            }
            84 => {
                if lookahead == 116 { state = 190; lexer.advance(false); continue; }
                return result;
            }
            85 => {
                result = true; lexer.set_result_symbol(anon_sym_lt); lexer.mark_end();
                return result;
            }
            86 => {
                if lookahead == 100 { state = 191; lexer.advance(false); continue; }
                return result;
            }
            87 => {
                if lookahead == 103 { state = 192; lexer.advance(false); continue; }
                return result;
            }
            88 => {
                result = true; lexer.set_result_symbol(anon_sym_my); lexer.mark_end();
                return result;
            }
            89 => {
                result = true; lexer.set_result_symbol(anon_sym_ne); lexer.mark_end();
                if lookahead == 120 { state = 193; lexer.advance(false); continue; }
                return result;
            }
            90 => {
                if lookahead == 116 { state = 194; lexer.advance(false); continue; }
                return result;
            }
            91 => {
                if lookahead == 116 { state = 195; lexer.advance(false); continue; }
                return result;
            }
            92 => {
                if lookahead == 101 { state = 196; lexer.advance(false); continue; }
                return result;
            }
            93 => {
                if lookahead == 99 { state = 197; lexer.advance(false); continue; }
                return result;
            }
            94 => {
                if lookahead == 105 { state = 198; lexer.advance(false); continue; }
                if lookahead == 111 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            95 => {
                if lookahead == 115 { state = 200; lexer.advance(false); continue; }
                return result;
            }
            96 => {
                result = true; lexer.set_result_symbol(anon_sym_qq); lexer.mark_end();
                return result;
            }
            97 => {
                result = true; lexer.set_result_symbol(anon_sym_qr); lexer.mark_end();
                return result;
            }
            98 => {
                if lookahead == 111 { state = 201; lexer.advance(false); continue; }
                return result;
            }
            99 => {
                result = true; lexer.set_result_symbol(anon_sym_qw); lexer.mark_end();
                return result;
            }
            100 => {
                result = true; lexer.set_result_symbol(anon_sym_qx); lexer.mark_end();
                return result;
            }
            101 => {
                if lookahead == 110 { state = 202; lexer.advance(false); continue; }
                return result;
            }
            102 => {
                if let Some(next) = advance_map(&[
                    (97, 203), (99, 204), (102, 205), (110, 206), (113, 207), (115, 208), (116, 209), (118, 210),
                    (119, 211),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            103 => {
                if lookahead == 110 { state = 212; lexer.advance(false); continue; }
                return result;
            }
            104 => {
                if lookahead == 121 { state = 213; lexer.advance(false); continue; }
                return result;
            }
            105 => {
                if lookahead == 101 { state = 214; lexer.advance(false); continue; }
                if lookahead == 108 { state = 215; lexer.advance(false); continue; }
                if lookahead == 109 { state = 216; lexer.advance(false); continue; }
                if lookahead == 110 { state = 217; lexer.advance(false); continue; }
                if lookahead == 116 { state = 218; lexer.advance(false); continue; }
                return result;
            }
            106 => {
                if lookahead == 105 { state = 219; lexer.advance(false); continue; }
                if lookahead == 109 { state = 220; lexer.advance(false); continue; }
                if lookahead == 117 { state = 221; lexer.advance(false); continue; }
                return result;
            }
            107 => {
                if lookahead == 99 { state = 222; lexer.advance(false); continue; }
                if lookahead == 114 { state = 223; lexer.advance(false); continue; }
                return result;
            }
            108 => {
                if lookahead == 108 { state = 224; lexer.advance(false); continue; }
                if lookahead == 114 { state = 225; lexer.advance(false); continue; }
                return result;
            }
            109 => {
                if lookahead == 114 { state = 226; lexer.advance(false); continue; }
                return result;
            }
            110 => {
                if lookahead == 97 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            111 => {
                if lookahead == 97 { state = 228; lexer.advance(false); continue; }
                if lookahead == 117 { state = 229; lexer.advance(false); continue; }
                return result;
            }
            112 => {
                if lookahead == 98 { state = 230; lexer.advance(false); continue; }
                return result;
            }
            113 => {
                if lookahead == 109 { state = 231; lexer.advance(false); continue; }
                if lookahead == 115 { state = 232; lexer.advance(false); continue; }
                return result;
            }
            114 => {
                if lookahead == 108 { state = 233; lexer.advance(false); continue; }
                return result;
            }
            115 => {
                if lookahead == 101 { state = 234; lexer.advance(false); continue; }
                if lookahead == 109 { state = 235; lexer.advance(false); continue; }
                return result;
            }
            116 => {
                result = true; lexer.set_result_symbol(anon_sym_tr); lexer.mark_end();
                if lookahead == 117 { state = 236; lexer.advance(false); continue; }
                return result;
            }
            117 => {
                if lookahead == 102 { state = 237; lexer.advance(false); continue; }
                return result;
            }
            118 => {
                if lookahead == 97 { state = 238; lexer.advance(false); continue; }
                return result;
            }
            119 => {
                if lookahead == 100 { state = 239; lexer.advance(false); continue; }
                if lookahead == 108 { state = 240; lexer.advance(false); continue; }
                if lookahead == 112 { state = 241; lexer.advance(false); continue; }
                if lookahead == 115 { state = 242; lexer.advance(false); continue; }
                if lookahead == 116 { state = 243; lexer.advance(false); continue; }
                return result;
            }
            120 => {
                if lookahead == 105 { state = 244; lexer.advance(false); continue; }
                return result;
            }
            121 => {
                if lookahead == 108 { state = 245; lexer.advance(false); continue; }
                return result;
            }
            122 => {
                if lookahead == 99 { state = 246; lexer.advance(false); continue; }
                return result;
            }
            123 => {
                if lookahead == 105 { state = 247; lexer.advance(false); continue; }
                if lookahead == 110 { state = 248; lexer.advance(false); continue; }
                if lookahead == 114 { state = 249; lexer.advance(false); continue; }
                return result;
            }
            124 => {
                if lookahead == 105 { state = 250; lexer.advance(false); continue; }
                return result;
            }
            125 => {
                if lookahead == 105 { state = 251; lexer.advance(false); continue; }
                return result;
            }
            126 => {
                if lookahead == 85 { state = 252; lexer.advance(false); continue; }
                return result;
            }
            127 => {
                if lookahead == 73 { state = 253; lexer.advance(false); continue; }
                return result;
            }
            128 => {
                if lookahead == 76 { state = 254; lexer.advance(false); continue; }
                return result;
            }
            129 => {
                if lookahead == 67 { state = 255; lexer.advance(false); continue; }
                return result;
            }
            130 => {
                result = true; lexer.set_result_symbol(anon_sym_END); lexer.mark_end();
                return result;
            }
            131 => {
                if lookahead == 84 { state = 256; lexer.advance(false); continue; }
                return result;
            }
            132 => {
                if lookahead == 69 { state = 257; lexer.advance(false); continue; }
                if lookahead == 73 { state = 258; lexer.advance(false); continue; }
                if lookahead == 79 { state = 259; lexer.advance(false); continue; }
                return result;
            }
            133 => {
                if lookahead == 84 { state = 260; lexer.advance(false); continue; }
                return result;
            }
            134 => {
                if lookahead == 65 { state = 261; lexer.advance(false); continue; }
                return result;
            }
            135 => {
                if lookahead == 78 { state = 262; lexer.advance(false); continue; }
                return result;
            }
            136 => {
                if lookahead == 73 { state = 263; lexer.advance(false); continue; }
                return result;
            }
            137 => {
                if lookahead == 73 { state = 264; lexer.advance(false); continue; }
                return result;
            }
            138 => {
                if lookahead == 65 { state = 265; lexer.advance(false); continue; }
                return result;
            }
            139 => {
                if lookahead == 85 { state = 266; lexer.advance(false); continue; }
                return result;
            }
            140 => {
                result = true; lexer.set_result_symbol(anon_sym_abs); lexer.mark_end();
                return result;
            }
            141 => {
                if lookahead == 101 { state = 267; lexer.advance(false); continue; }
                return result;
            }
            142 => {
                if lookahead == 110 { state = 268; lexer.advance(false); continue; }
                return result;
            }
            143 => {
                if lookahead == 105 { state = 269; lexer.advance(false); continue; }
                return result;
            }
            144 => {
                if lookahead == 100 { state = 270; lexer.advance(false); continue; }
                if lookahead == 109 { state = 271; lexer.advance(false); continue; }
                return result;
            }
            145 => {
                if lookahead == 115 { state = 272; lexer.advance(false); continue; }
                return result;
            }
            146 => {
                if lookahead == 97 { state = 273; lexer.advance(false); continue; }
                return result;
            }
            147 => {
                if lookahead == 99 { state = 274; lexer.advance(false); continue; }
                return result;
            }
            148 => {
                if lookahead == 105 { state = 275; lexer.advance(false); continue; }
                return result;
            }
            149 => {
                if lookahead == 111 { state = 276; lexer.advance(false); continue; }
                return result;
            }
            150 => {
                if lookahead == 109 { state = 277; lexer.advance(false); continue; }
                if lookahead == 112 { state = 278; lexer.advance(false); continue; }
                if lookahead == 119 { state = 279; lexer.advance(false); continue; }
                return result;
            }
            151 => {
                result = true; lexer.set_result_symbol(anon_sym_chr); lexer.mark_end();
                if lookahead == 111 { state = 280; lexer.advance(false); continue; }
                return result;
            }
            152 => {
                if lookahead == 110 { state = 281; lexer.advance(false); continue; }
                if lookahead == 116 { state = 282; lexer.advance(false); continue; }
                return result;
            }
            153 => {
                if lookahead == 112 { state = 283; lexer.advance(false); continue; }
                return result;
            }
            154 => {
                if lookahead == 99 { state = 284; lexer.advance(false); continue; }
                if lookahead == 111 { state = 285; lexer.advance(false); continue; }
                return result;
            }
            155 => {
                if lookahead == 101 { state = 286; lexer.advance(false); continue; }
                if lookahead == 105 { state = 287; lexer.advance(false); continue; }
                return result;
            }
            156 => {
                if lookahead == 101 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            157 => {
                if lookahead == 97 { state = 289; lexer.advance(false); continue; }
                return result;
            }
            158 => {
                if lookahead == 104 { state = 290; lexer.advance(false); continue; }
                return result;
            }
            159 => {
                if lookahead == 105 { state = 291; lexer.advance(false); continue; }
                return result;
            }
            160 => {
                result = true; lexer.set_result_symbol(anon_sym_eof); lexer.mark_end();
                return result;
            }
            161 => {
                result = true; lexer.set_result_symbol(anon_sym_eqr); lexer.mark_end();
                return result;
            }
            162 => {
                result = true; lexer.set_result_symbol(anon_sym_equ); lexer.mark_end();
                return result;
            }
            163 => {
                if lookahead == 108 { state = 292; lexer.advance(false); continue; }
                return result;
            }
            164 => {
                if lookahead == 115 { state = 293; lexer.advance(false); continue; }
                if lookahead == 116 { state = 294; lexer.advance(false); continue; }
                return result;
            }
            165 => {
                if lookahead == 101 { state = 295; lexer.advance(false); continue; }
                return result;
            }
            166 => {
                if lookahead == 115 { state = 296; lexer.advance(false); continue; }
                return result;
            }
            167 => {
                if lookahead == 116 { state = 297; lexer.advance(false); continue; }
                return result;
            }
            168 => {
                if lookahead == 108 { state = 298; lexer.advance(false); continue; }
                return result;
            }
            169 => {
                if lookahead == 101 { state = 299; lexer.advance(false); continue; }
                return result;
            }
            170 => {
                if lookahead == 97 { state = 300; lexer.advance(false); continue; }
                return result;
            }
            171 => {
                if lookahead == 99 { state = 301; lexer.advance(false); continue; }
                return result;
            }
            172 => {
                result = true; lexer.set_result_symbol(anon_sym_for); lexer.mark_end();
                if lookahead == 101 { state = 302; lexer.advance(false); continue; }
                if lookahead == 107 { state = 303; lexer.advance(false); continue; }
                if lookahead == 109 { state = 304; lexer.advance(false); continue; }
                return result;
            }
            173 => {
                if lookahead == 99 { state = 305; lexer.advance(false); continue; }
                if lookahead == 103 { state = 306; lexer.advance(false); continue; }
                if lookahead == 104 { state = 307; lexer.advance(false); continue; }
                if lookahead == 110 { state = 308; lexer.advance(false); continue; }
                if lookahead == 112 { state = 309; lexer.advance(false); continue; }
                if lookahead == 115 { state = 310; lexer.advance(false); continue; }
                return result;
            }
            174 => {
                if lookahead == 98 { state = 311; lexer.advance(false); continue; }
                return result;
            }
            175 => {
                if lookahead == 105 { state = 312; lexer.advance(false); continue; }
                return result;
            }
            176 => {
                if lookahead == 111 { state = 313; lexer.advance(false); continue; }
                return result;
            }
            177 => {
                result = true; lexer.set_result_symbol(anon_sym_hex); lexer.mark_end();
                return result;
            }
            178 => {
                if lookahead == 101 { state = 314; lexer.advance(false); continue; }
                return result;
            }
            179 => {
                result = true; lexer.set_result_symbol(anon_sym_int); lexer.mark_end();
                return result;
            }
            180 => {
                if lookahead == 116 { state = 315; lexer.advance(false); continue; }
                return result;
            }
            181 => {
                if lookahead == 110 { state = 316; lexer.advance(false); continue; }
                return result;
            }
            182 => {
                if lookahead == 115 { state = 317; lexer.advance(false); continue; }
                return result;
            }
            183 => {
                if lookahead == 108 { state = 318; lexer.advance(false); continue; }
                return result;
            }
            184 => {
                if lookahead == 116 { state = 319; lexer.advance(false); continue; }
                return result;
            }
            185 => {
                if lookahead == 105 { state = 320; lexer.advance(false); continue; }
                return result;
            }
            186 => {
                if lookahead == 103 { state = 321; lexer.advance(false); continue; }
                return result;
            }
            187 => {
                if lookahead == 107 { state = 322; lexer.advance(false); continue; }
                return result;
            }
            188 => {
                if lookahead == 116 { state = 323; lexer.advance(false); continue; }
                return result;
            }
            189 => {
                if lookahead == 97 { state = 324; lexer.advance(false); continue; }
                if lookahead == 107 { state = 325; lexer.advance(false); continue; }
                return result;
            }
            190 => {
                if lookahead == 97 { state = 326; lexer.advance(false); continue; }
                return result;
            }
            191 => {
                if lookahead == 105 { state = 327; lexer.advance(false); continue; }
                return result;
            }
            192 => {
                if lookahead == 99 { state = 328; lexer.advance(false); continue; }
                if lookahead == 103 { state = 329; lexer.advance(false); continue; }
                if lookahead == 114 { state = 330; lexer.advance(false); continue; }
                if lookahead == 115 { state = 331; lexer.advance(false); continue; }
                return result;
            }
            193 => {
                if lookahead == 116 { state = 332; lexer.advance(false); continue; }
                return result;
            }
            194 => {
                result = true; lexer.set_result_symbol(anon_sym_not); lexer.mark_end();
                return result;
            }
            195 => {
                result = true; lexer.set_result_symbol(anon_sym_oct); lexer.mark_end();
                return result;
            }
            196 => {
                if lookahead == 110 { state = 333; lexer.advance(false); continue; }
                return result;
            }
            197 => {
                if lookahead == 107 { state = 334; lexer.advance(false); continue; }
                return result;
            }
            198 => {
                if lookahead == 110 { state = 335; lexer.advance(false); continue; }
                return result;
            }
            199 => {
                if lookahead == 116 { state = 336; lexer.advance(false); continue; }
                return result;
            }
            200 => {
                if lookahead == 104 { state = 337; lexer.advance(false); continue; }
                return result;
            }
            201 => {
                if lookahead == 116 { state = 338; lexer.advance(false); continue; }
                return result;
            }
            202 => {
                if lookahead == 100 { state = 339; lexer.advance(false); continue; }
                return result;
            }
            203 => {
                if lookahead == 100 { state = 340; lexer.advance(false); continue; }
                return result;
            }
            204 => {
                if lookahead == 118 { state = 341; lexer.advance(false); continue; }
                return result;
            }
            205 => {
                result = true; lexer.set_result_symbol(anon_sym_ref); lexer.mark_end();
                return result;
            }
            206 => {
                if lookahead == 97 { state = 342; lexer.advance(false); continue; }
                return result;
            }
            207 => {
                if lookahead == 117 { state = 343; lexer.advance(false); continue; }
                return result;
            }
            208 => {
                if lookahead == 101 { state = 344; lexer.advance(false); continue; }
                return result;
            }
            209 => {
                if lookahead == 117 { state = 345; lexer.advance(false); continue; }
                return result;
            }
            210 => {
                if lookahead == 101 { state = 346; lexer.advance(false); continue; }
                return result;
            }
            211 => {
                if lookahead == 105 { state = 347; lexer.advance(false); continue; }
                return result;
            }
            212 => {
                if lookahead == 100 { state = 348; lexer.advance(false); continue; }
                return result;
            }
            213 => {
                result = true; lexer.set_result_symbol(anon_sym_say); lexer.mark_end();
                return result;
            }
            214 => {
                if lookahead == 107 { state = 349; lexer.advance(false); continue; }
                return result;
            }
            215 => {
                if lookahead == 101 { state = 350; lexer.advance(false); continue; }
                return result;
            }
            216 => {
                if lookahead == 99 { state = 351; lexer.advance(false); continue; }
                if lookahead == 103 { state = 352; lexer.advance(false); continue; }
                return result;
            }
            217 => {
                if lookahead == 100 { state = 353; lexer.advance(false); continue; }
                return result;
            }
            218 => {
                if lookahead == 112 { state = 354; lexer.advance(false); continue; }
                if lookahead == 115 { state = 355; lexer.advance(false); continue; }
                return result;
            }
            219 => {
                if lookahead == 102 { state = 356; lexer.advance(false); continue; }
                return result;
            }
            220 => {
                if lookahead == 99 { state = 357; lexer.advance(false); continue; }
                if lookahead == 114 { state = 358; lexer.advance(false); continue; }
                if lookahead == 119 { state = 359; lexer.advance(false); continue; }
                return result;
            }
            221 => {
                if lookahead == 116 { state = 360; lexer.advance(false); continue; }
                return result;
            }
            222 => {
                if lookahead == 107 { state = 361; lexer.advance(false); continue; }
                return result;
            }
            223 => {
                if lookahead == 116 { state = 362; lexer.advance(false); continue; }
                return result;
            }
            224 => {
                if lookahead == 105 { state = 363; lexer.advance(false); continue; }
                return result;
            }
            225 => {
                if lookahead == 105 { state = 364; lexer.advance(false); continue; }
                return result;
            }
            226 => {
                if lookahead == 116 { state = 365; lexer.advance(false); continue; }
                return result;
            }
            227 => {
                if lookahead == 110 { state = 366; lexer.advance(false); continue; }
                return result;
            }
            228 => {
                if lookahead == 116 { state = 367; lexer.advance(false); continue; }
                return result;
            }
            229 => {
                if lookahead == 100 { state = 368; lexer.advance(false); continue; }
                return result;
            }
            230 => {
                result = true; lexer.set_result_symbol(anon_sym_sub); lexer.mark_end();
                if lookahead == 115 { state = 369; lexer.advance(false); continue; }
                return result;
            }
            231 => {
                if lookahead == 108 { state = 370; lexer.advance(false); continue; }
                return result;
            }
            232 => {
                if lookahead == 99 { state = 371; lexer.advance(false); continue; }
                if lookahead == 111 { state = 372; lexer.advance(false); continue; }
                if lookahead == 114 { state = 373; lexer.advance(false); continue; }
                if lookahead == 115 { state = 374; lexer.advance(false); continue; }
                if lookahead == 116 { state = 375; lexer.advance(false); continue; }
                if lookahead == 119 { state = 376; lexer.advance(false); continue; }
                return result;
            }
            233 => {
                if lookahead == 108 { state = 377; lexer.advance(false); continue; }
                return result;
            }
            234 => {
                result = true; lexer.set_result_symbol(anon_sym_tie); lexer.mark_end();
                if lookahead == 100 { state = 378; lexer.advance(false); continue; }
                return result;
            }
            235 => {
                if lookahead == 101 { state = 379; lexer.advance(false); continue; }
                return result;
            }
            236 => {
                if lookahead == 101 { state = 380; lexer.advance(false); continue; }
                if lookahead == 110 { state = 381; lexer.advance(false); continue; }
                return result;
            }
            237 => {
                if lookahead == 105 { state = 382; lexer.advance(false); continue; }
                return result;
            }
            238 => {
                if lookahead == 115 { state = 383; lexer.advance(false); continue; }
                return result;
            }
            239 => {
                if lookahead == 101 { state = 384; lexer.advance(false); continue; }
                return result;
            }
            240 => {
                if lookahead == 101 { state = 385; lexer.advance(false); continue; }
                if lookahead == 105 { state = 386; lexer.advance(false); continue; }
                return result;
            }
            241 => {
                if lookahead == 97 { state = 387; lexer.advance(false); continue; }
                return result;
            }
            242 => {
                if lookahead == 104 { state = 388; lexer.advance(false); continue; }
                return result;
            }
            243 => {
                if lookahead == 105 { state = 389; lexer.advance(false); continue; }
                return result;
            }
            244 => {
                if lookahead == 109 { state = 390; lexer.advance(false); continue; }
                return result;
            }
            245 => {
                if lookahead == 117 { state = 391; lexer.advance(false); continue; }
                return result;
            }
            246 => {
                result = true; lexer.set_result_symbol(anon_sym_vec); lexer.mark_end();
                return result;
            }
            247 => {
                if lookahead == 116 { state = 392; lexer.advance(false); continue; }
                return result;
            }
            248 => {
                if lookahead == 116 { state = 393; lexer.advance(false); continue; }
                return result;
            }
            249 => {
                if lookahead == 110 { state = 394; lexer.advance(false); continue; }
                return result;
            }
            250 => {
                if lookahead == 108 { state = 395; lexer.advance(false); continue; }
                return result;
            }
            251 => {
                if lookahead == 116 { state = 396; lexer.advance(false); continue; }
                return result;
            }
            252 => {
                if lookahead == 83 { state = 397; lexer.advance(false); continue; }
                return result;
            }
            253 => {
                if lookahead == 78 { state = 398; lexer.advance(false); continue; }
                return result;
            }
            254 => {
                if lookahead == 68 { state = 399; lexer.advance(false); continue; }
                return result;
            }
            255 => {
                if lookahead == 75 { state = 400; lexer.advance(false); continue; }
                return result;
            }
            256 => {
                result = true; lexer.set_result_symbol(anon_sym_INIT); lexer.mark_end();
                return result;
            }
            257 => {
                if lookahead == 82 { state = 401; lexer.advance(false); continue; }
                return result;
            }
            258 => {
                if lookahead == 78 { state = 402; lexer.advance(false); continue; }
                return result;
            }
            259 => {
                if lookahead == 85 { state = 403; lexer.advance(false); continue; }
                return result;
            }
            260 => {
                if lookahead == 67 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            261 => {
                if lookahead == 84 { state = 405; lexer.advance(false); continue; }
                return result;
            }
            262 => {
                if lookahead == 68 { state = 406; lexer.advance(false); continue; }
                return result;
            }
            263 => {
                if lookahead == 76 { state = 407; lexer.advance(false); continue; }
                return result;
            }
            264 => {
                if lookahead == 78 { state = 408; lexer.advance(false); continue; }
                return result;
            }
            265 => {
                if lookahead == 67 { state = 409; lexer.advance(false); continue; }
                return result;
            }
            266 => {
                if lookahead == 66 { state = 410; lexer.advance(false); continue; }
                return result;
            }
            267 => {
                if lookahead == 112 { state = 411; lexer.advance(false); continue; }
                return result;
            }
            268 => {
                if lookahead == 50 { state = 412; lexer.advance(false); continue; }
                return result;
            }
            269 => {
                if lookahead == 116 { state = 413; lexer.advance(false); continue; }
                return result;
            }
            270 => {
                result = true; lexer.set_result_symbol(anon_sym_bind); lexer.mark_end();
                return result;
            }
            271 => {
                if lookahead == 111 { state = 414; lexer.advance(false); continue; }
                return result;
            }
            272 => {
                if lookahead == 115 { state = 415; lexer.advance(false); continue; }
                return result;
            }
            273 => {
                if lookahead == 107 { state = 416; lexer.advance(false); continue; }
                return result;
            }
            274 => {
                if lookahead == 104 { state = 417; lexer.advance(false); continue; }
                return result;
            }
            275 => {
                if lookahead == 114 { state = 418; lexer.advance(false); continue; }
                return result;
            }
            276 => {
                if lookahead == 100 { state = 419; lexer.advance(false); continue; }
                return result;
            }
            277 => {
                if lookahead == 112 { state = 420; lexer.advance(false); continue; }
                return result;
            }
            278 => {
                result = true; lexer.set_result_symbol(anon_sym_chop); lexer.mark_end();
                return result;
            }
            279 => {
                if lookahead == 110 { state = 421; lexer.advance(false); continue; }
                return result;
            }
            280 => {
                if lookahead == 111 { state = 422; lexer.advance(false); continue; }
                return result;
            }
            281 => {
                if lookahead == 101 { state = 423; lexer.advance(false); continue; }
                return result;
            }
            282 => {
                if lookahead == 105 { state = 424; lexer.advance(false); continue; }
                return result;
            }
            283 => {
                if lookahead == 116 { state = 425; lexer.advance(false); continue; }
                return result;
            }
            284 => {
                if lookahead == 108 { state = 426; lexer.advance(false); continue; }
                return result;
            }
            285 => {
                if lookahead == 112 { state = 427; lexer.advance(false); continue; }
                return result;
            }
            286 => {
                if lookahead == 114 { state = 428; lexer.advance(false); continue; }
                return result;
            }
            287 => {
                if lookahead == 110 { state = 429; lexer.advance(false); continue; }
                return result;
            }
            288 => {
                if lookahead == 116 { state = 430; lexer.advance(false); continue; }
                return result;
            }
            289 => {
                if lookahead == 109 { state = 431; lexer.advance(false); continue; }
                return result;
            }
            290 => {
                result = true; lexer.set_result_symbol(anon_sym_each); lexer.mark_end();
                return result;
            }
            291 => {
                if lookahead == 102 { state = 432; lexer.advance(false); continue; }
                return result;
            }
            292 => {
                result = true; lexer.set_result_symbol(anon_sym_eval); lexer.mark_end();
                return result;
            }
            293 => {
                if lookahead == 116 { state = 433; lexer.advance(false); continue; }
                return result;
            }
            294 => {
                result = true; lexer.set_result_symbol(anon_sym_exit); lexer.mark_end();
                return result;
            }
            295 => {
                if lookahead == 110 { state = 434; lexer.advance(false); continue; }
                return result;
            }
            296 => {
                if lookahead == 101 { state = 435; lexer.advance(false); continue; }
                return result;
            }
            297 => {
                if lookahead == 108 { state = 436; lexer.advance(false); continue; }
                return result;
            }
            298 => {
                if lookahead == 100 { state = 437; lexer.advance(false); continue; }
                return result;
            }
            299 => {
                if lookahead == 110 { state = 438; lexer.advance(false); continue; }
                return result;
            }
            300 => {
                if lookahead == 108 { state = 439; lexer.advance(false); continue; }
                return result;
            }
            301 => {
                if lookahead == 107 { state = 440; lexer.advance(false); continue; }
                return result;
            }
            302 => {
                if lookahead == 97 { state = 441; lexer.advance(false); continue; }
                return result;
            }
            303 => {
                result = true; lexer.set_result_symbol(anon_sym_fork); lexer.mark_end();
                return result;
            }
            304 => {
                if lookahead == 97 { state = 442; lexer.advance(false); continue; }
                if lookahead == 108 { state = 443; lexer.advance(false); continue; }
                return result;
            }
            305 => {
                result = true; lexer.set_result_symbol(anon_sym_getc); lexer.mark_end();
                return result;
            }
            306 => {
                if lookahead == 114 { state = 444; lexer.advance(false); continue; }
                return result;
            }
            307 => {
                if lookahead == 111 { state = 445; lexer.advance(false); continue; }
                return result;
            }
            308 => {
                if lookahead == 101 { state = 446; lexer.advance(false); continue; }
                return result;
            }
            309 => {
                if lookahead == 101 { state = 447; lexer.advance(false); continue; }
                if lookahead == 103 { state = 448; lexer.advance(false); continue; }
                if lookahead == 112 { state = 449; lexer.advance(false); continue; }
                if lookahead == 114 { state = 450; lexer.advance(false); continue; }
                if lookahead == 119 { state = 451; lexer.advance(false); continue; }
                return result;
            }
            310 => {
                if lookahead == 101 { state = 452; lexer.advance(false); continue; }
                if lookahead == 111 { state = 453; lexer.advance(false); continue; }
                return result;
            }
            311 => {
                result = true; lexer.set_result_symbol(anon_sym_glob); lexer.mark_end();
                return result;
            }
            312 => {
                if lookahead == 109 { state = 454; lexer.advance(false); continue; }
                return result;
            }
            313 => {
                result = true; lexer.set_result_symbol(anon_sym_goto); lexer.mark_end();
                return result;
            }
            314 => {
                if lookahead == 120 { state = 455; lexer.advance(false); continue; }
                return result;
            }
            315 => {
                if lookahead == 108 { state = 456; lexer.advance(false); continue; }
                return result;
            }
            316 => {
                result = true; lexer.set_result_symbol(anon_sym_join); lexer.mark_end();
                return result;
            }
            317 => {
                result = true; lexer.set_result_symbol(anon_sym_keys); lexer.mark_end();
                return result;
            }
            318 => {
                result = true; lexer.set_result_symbol(anon_sym_kill); lexer.mark_end();
                return result;
            }
            319 => {
                result = true; lexer.set_result_symbol(anon_sym_last); lexer.mark_end();
                return result;
            }
            320 => {
                if lookahead == 114 { state = 457; lexer.advance(false); continue; }
                return result;
            }
            321 => {
                if lookahead == 116 { state = 458; lexer.advance(false); continue; }
                return result;
            }
            322 => {
                result = true; lexer.set_result_symbol(anon_sym_link); lexer.mark_end();
                return result;
            }
            323 => {
                if lookahead == 101 { state = 459; lexer.advance(false); continue; }
                return result;
            }
            324 => {
                if lookahead == 108 { state = 460; lexer.advance(false); continue; }
                return result;
            }
            325 => {
                result = true; lexer.set_result_symbol(anon_sym_lock); lexer.mark_end();
                return result;
            }
            326 => {
                if lookahead == 116 { state = 461; lexer.advance(false); continue; }
                return result;
            }
            327 => {
                if lookahead == 114 { state = 462; lexer.advance(false); continue; }
                return result;
            }
            328 => {
                if lookahead == 116 { state = 463; lexer.advance(false); continue; }
                return result;
            }
            329 => {
                if lookahead == 101 { state = 464; lexer.advance(false); continue; }
                return result;
            }
            330 => {
                if lookahead == 99 { state = 465; lexer.advance(false); continue; }
                return result;
            }
            331 => {
                if lookahead == 101 { state = 466; lexer.advance(false); continue; }
                return result;
            }
            332 => {
                result = true; lexer.set_result_symbol(anon_sym_next); lexer.mark_end();
                return result;
            }
            333 => {
                result = true; lexer.set_result_symbol(anon_sym_open); lexer.mark_end();
                if lookahead == 100 { state = 467; lexer.advance(false); continue; }
                return result;
            }
            334 => {
                result = true; lexer.set_result_symbol(anon_sym_pack); lexer.mark_end();
                if lookahead == 97 { state = 468; lexer.advance(false); continue; }
                return result;
            }
            335 => {
                if lookahead == 116 { state = 469; lexer.advance(false); continue; }
                return result;
            }
            336 => {
                if lookahead == 111 { state = 470; lexer.advance(false); continue; }
                return result;
            }
            337 => {
                result = true; lexer.set_result_symbol(anon_sym_push); lexer.mark_end();
                return result;
            }
            338 => {
                if lookahead == 101 { state = 471; lexer.advance(false); continue; }
                return result;
            }
            339 => {
                result = true; lexer.set_result_symbol(anon_sym_rand); lexer.mark_end();
                return result;
            }
            340 => {
                if lookahead == 108 { state = 472; lexer.advance(false); continue; }
                return result;
            }
            341 => {
                result = true; lexer.set_result_symbol(anon_sym_recv); lexer.mark_end();
                return result;
            }
            342 => {
                if lookahead == 109 { state = 473; lexer.advance(false); continue; }
                return result;
            }
            343 => {
                if lookahead == 105 { state = 474; lexer.advance(false); continue; }
                return result;
            }
            344 => {
                if lookahead == 116 { state = 475; lexer.advance(false); continue; }
                return result;
            }
            345 => {
                if lookahead == 114 { state = 476; lexer.advance(false); continue; }
                return result;
            }
            346 => {
                if lookahead == 114 { state = 477; lexer.advance(false); continue; }
                return result;
            }
            347 => {
                if lookahead == 110 { state = 478; lexer.advance(false); continue; }
                return result;
            }
            348 => {
                if lookahead == 101 { state = 479; lexer.advance(false); continue; }
                return result;
            }
            349 => {
                result = true; lexer.set_result_symbol(anon_sym_seek); lexer.mark_end();
                if lookahead == 100 { state = 480; lexer.advance(false); continue; }
                return result;
            }
            350 => {
                if lookahead == 99 { state = 481; lexer.advance(false); continue; }
                return result;
            }
            351 => {
                if lookahead == 116 { state = 482; lexer.advance(false); continue; }
                return result;
            }
            352 => {
                if lookahead == 101 { state = 483; lexer.advance(false); continue; }
                return result;
            }
            353 => {
                result = true; lexer.set_result_symbol(anon_sym_send); lexer.mark_end();
                return result;
            }
            354 => {
                if lookahead == 103 { state = 484; lexer.advance(false); continue; }
                if lookahead == 114 { state = 485; lexer.advance(false); continue; }
                return result;
            }
            355 => {
                if lookahead == 111 { state = 486; lexer.advance(false); continue; }
                return result;
            }
            356 => {
                if lookahead == 116 { state = 487; lexer.advance(false); continue; }
                return result;
            }
            357 => {
                if lookahead == 116 { state = 488; lexer.advance(false); continue; }
                return result;
            }
            358 => {
                if lookahead == 101 { state = 489; lexer.advance(false); continue; }
                return result;
            }
            359 => {
                if lookahead == 114 { state = 490; lexer.advance(false); continue; }
                return result;
            }
            360 => {
                if lookahead == 100 { state = 491; lexer.advance(false); continue; }
                return result;
            }
            361 => {
                if lookahead == 101 { state = 492; lexer.advance(false); continue; }
                return result;
            }
            362 => {
                result = true; lexer.set_result_symbol(anon_sym_sort); lexer.mark_end();
                return result;
            }
            363 => {
                if lookahead == 116 { state = 493; lexer.advance(false); continue; }
                return result;
            }
            364 => {
                if lookahead == 110 { state = 494; lexer.advance(false); continue; }
                return result;
            }
            365 => {
                result = true; lexer.set_result_symbol(anon_sym_sqrt); lexer.mark_end();
                return result;
            }
            366 => {
                if lookahead == 100 { state = 495; lexer.advance(false); continue; }
                return result;
            }
            367 => {
                result = true; lexer.set_result_symbol(anon_sym_stat); lexer.mark_end();
                if lookahead == 101 { state = 496; lexer.advance(false); continue; }
                return result;
            }
            368 => {
                if lookahead == 121 { state = 497; lexer.advance(false); continue; }
                return result;
            }
            369 => {
                if lookahead == 116 { state = 498; lexer.advance(false); continue; }
                return result;
            }
            370 => {
                if lookahead == 105 { state = 499; lexer.advance(false); continue; }
                return result;
            }
            371 => {
                if lookahead == 97 { state = 500; lexer.advance(false); continue; }
                return result;
            }
            372 => {
                if lookahead == 112 { state = 501; lexer.advance(false); continue; }
                return result;
            }
            373 => {
                if lookahead == 101 { state = 502; lexer.advance(false); continue; }
                return result;
            }
            374 => {
                if lookahead == 101 { state = 503; lexer.advance(false); continue; }
                return result;
            }
            375 => {
                if lookahead == 101 { state = 504; lexer.advance(false); continue; }
                return result;
            }
            376 => {
                if lookahead == 114 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            377 => {
                result = true; lexer.set_result_symbol(anon_sym_tell); lexer.mark_end();
                if lookahead == 100 { state = 506; lexer.advance(false); continue; }
                return result;
            }
            378 => {
                result = true; lexer.set_result_symbol(anon_sym_tied); lexer.mark_end();
                return result;
            }
            379 => {
                result = true; lexer.set_result_symbol(anon_sym_time); lexer.mark_end();
                if lookahead == 115 { state = 507; lexer.advance(false); continue; }
                return result;
            }
            380 => {
                result = true; lexer.set_result_symbol(anon_sym_true); lexer.mark_end();
                return result;
            }
            381 => {
                if lookahead == 99 { state = 508; lexer.advance(false); continue; }
                return result;
            }
            382 => {
                if lookahead == 114 { state = 509; lexer.advance(false); continue; }
                return result;
            }
            383 => {
                if lookahead == 107 { state = 510; lexer.advance(false); continue; }
                return result;
            }
            384 => {
                if lookahead == 102 { state = 511; lexer.advance(false); continue; }
                return result;
            }
            385 => {
                if lookahead == 115 { state = 512; lexer.advance(false); continue; }
                return result;
            }
            386 => {
                if lookahead == 110 { state = 513; lexer.advance(false); continue; }
                return result;
            }
            387 => {
                if lookahead == 99 { state = 514; lexer.advance(false); continue; }
                return result;
            }
            388 => {
                if lookahead == 105 { state = 515; lexer.advance(false); continue; }
                return result;
            }
            389 => {
                if lookahead == 101 { state = 516; lexer.advance(false); continue; }
                if lookahead == 108 { state = 517; lexer.advance(false); continue; }
                return result;
            }
            390 => {
                if lookahead == 101 { state = 518; lexer.advance(false); continue; }
                return result;
            }
            391 => {
                if lookahead == 101 { state = 519; lexer.advance(false); continue; }
                return result;
            }
            392 => {
                result = true; lexer.set_result_symbol(anon_sym_wait); lexer.mark_end();
                if lookahead == 112 { state = 520; lexer.advance(false); continue; }
                return result;
            }
            393 => {
                if lookahead == 97 { state = 521; lexer.advance(false); continue; }
                return result;
            }
            394 => {
                result = true; lexer.set_result_symbol(anon_sym_warn); lexer.mark_end();
                return result;
            }
            395 => {
                if lookahead == 101 { state = 522; lexer.advance(false); continue; }
                return result;
            }
            396 => {
                if lookahead == 101 { state = 523; lexer.advance(false); continue; }
                return result;
            }
            397 => {
                if lookahead == 84 { state = 524; lexer.advance(false); continue; }
                return result;
            }
            398 => {
                result = true; lexer.set_result_symbol(anon_sym_BEGIN); lexer.mark_end();
                return result;
            }
            399 => {
                result = true; lexer.set_result_symbol(anon_sym_BUILD); lexer.mark_end();
                return result;
            }
            400 => {
                result = true; lexer.set_result_symbol(anon_sym_CHECK); lexer.mark_end();
                return result;
            }
            401 => {
                if lookahead == 82 { state = 525; lexer.advance(false); continue; }
                return result;
            }
            402 => {
                result = true; lexer.set_result_symbol(anon_sym_STDIN); lexer.mark_end();
                return result;
            }
            403 => {
                if lookahead == 84 { state = 526; lexer.advance(false); continue; }
                return result;
            }
            404 => {
                if lookahead == 72 { state = 527; lexer.advance(false); continue; }
                return result;
            }
            405 => {
                if lookahead == 65 { state = 528; lexer.advance(false); continue; }
                return result;
            }
            406 => {
                if lookahead == 95 { state = 529; lexer.advance(false); continue; }
                return result;
            }
            407 => {
                if lookahead == 69 { state = 530; lexer.advance(false); continue; }
                return result;
            }
            408 => {
                if lookahead == 69 { state = 531; lexer.advance(false); continue; }
                return result;
            }
            409 => {
                if lookahead == 75 { state = 532; lexer.advance(false); continue; }
                return result;
            }
            410 => {
                if lookahead == 95 { state = 533; lexer.advance(false); continue; }
                return result;
            }
            411 => {
                if lookahead == 116 { state = 534; lexer.advance(false); continue; }
                return result;
            }
            412 => {
                result = true; lexer.set_result_symbol(anon_sym_atan2); lexer.mark_end();
                return result;
            }
            413 => {
                result = true; lexer.set_result_symbol(anon_sym_await); lexer.mark_end();
                return result;
            }
            414 => {
                if lookahead == 100 { state = 535; lexer.advance(false); continue; }
                return result;
            }
            415 => {
                result = true; lexer.set_result_symbol(anon_sym_bless); lexer.mark_end();
                return result;
            }
            416 => {
                result = true; lexer.set_result_symbol(anon_sym_break); lexer.mark_end();
                return result;
            }
            417 => {
                result = true; lexer.set_result_symbol(anon_sym_catch); lexer.mark_end();
                return result;
            }
            418 => {
                result = true; lexer.set_result_symbol(anon_sym_chdir); lexer.mark_end();
                return result;
            }
            419 => {
                result = true; lexer.set_result_symbol(anon_sym_chmod); lexer.mark_end();
                return result;
            }
            420 => {
                result = true; lexer.set_result_symbol(anon_sym_chomp); lexer.mark_end();
                return result;
            }
            421 => {
                result = true; lexer.set_result_symbol(anon_sym_chown); lexer.mark_end();
                return result;
            }
            422 => {
                if lookahead == 116 { state = 536; lexer.advance(false); continue; }
                return result;
            }
            423 => {
                if lookahead == 99 { state = 537; lexer.advance(false); continue; }
                return result;
            }
            424 => {
                if lookahead == 110 { state = 538; lexer.advance(false); continue; }
                return result;
            }
            425 => {
                result = true; lexer.set_result_symbol(anon_sym_crypt); lexer.mark_end();
                return result;
            }
            426 => {
                if lookahead == 111 { state = 539; lexer.advance(false); continue; }
                return result;
            }
            427 => {
                if lookahead == 101 { state = 540; lexer.advance(false); continue; }
                return result;
            }
            428 => {
                result = true; lexer.set_result_symbol(anon_sym_defer); lexer.mark_end();
                return result;
            }
            429 => {
                if lookahead == 101 { state = 541; lexer.advance(false); continue; }
                return result;
            }
            430 => {
                if lookahead == 101 { state = 542; lexer.advance(false); continue; }
                return result;
            }
            431 => {
                if lookahead == 105 { state = 543; lexer.advance(false); continue; }
                return result;
            }
            432 => {
                result = true; lexer.set_result_symbol(anon_sym_elsif); lexer.mark_end();
                return result;
            }
            433 => {
                if lookahead == 115 { state = 544; lexer.advance(false); continue; }
                return result;
            }
            434 => {
                if lookahead == 100 { state = 545; lexer.advance(false); continue; }
                return result;
            }
            435 => {
                result = true; lexer.set_result_symbol(anon_sym_false); lexer.mark_end();
                return result;
            }
            436 => {
                result = true; lexer.set_result_symbol(anon_sym_fcntl); lexer.mark_end();
                return result;
            }
            437 => {
                result = true; lexer.set_result_symbol(anon_sym_field); lexer.mark_end();
                return result;
            }
            438 => {
                if lookahead == 111 { state = 546; lexer.advance(false); continue; }
                return result;
            }
            439 => {
                if lookahead == 108 { state = 547; lexer.advance(false); continue; }
                return result;
            }
            440 => {
                result = true; lexer.set_result_symbol(anon_sym_flock); lexer.mark_end();
                return result;
            }
            441 => {
                if lookahead == 99 { state = 548; lexer.advance(false); continue; }
                return result;
            }
            442 => {
                if lookahead == 116 { state = 549; lexer.advance(false); continue; }
                return result;
            }
            443 => {
                if lookahead == 105 { state = 550; lexer.advance(false); continue; }
                return result;
            }
            444 => {
                if lookahead == 103 { state = 551; lexer.advance(false); continue; }
                if lookahead == 110 { state = 552; lexer.advance(false); continue; }
                return result;
            }
            445 => {
                if lookahead == 115 { state = 553; lexer.advance(false); continue; }
                return result;
            }
            446 => {
                if lookahead == 116 { state = 554; lexer.advance(false); continue; }
                return result;
            }
            447 => {
                if lookahead == 101 { state = 555; lexer.advance(false); continue; }
                return result;
            }
            448 => {
                if lookahead == 114 { state = 556; lexer.advance(false); continue; }
                return result;
            }
            449 => {
                if lookahead == 105 { state = 557; lexer.advance(false); continue; }
                return result;
            }
            450 => {
                if lookahead == 105 { state = 558; lexer.advance(false); continue; }
                if lookahead == 111 { state = 559; lexer.advance(false); continue; }
                return result;
            }
            451 => {
                if lookahead == 110 { state = 560; lexer.advance(false); continue; }
                if lookahead == 117 { state = 561; lexer.advance(false); continue; }
                return result;
            }
            452 => {
                if lookahead == 114 { state = 562; lexer.advance(false); continue; }
                return result;
            }
            453 => {
                if lookahead == 99 { state = 563; lexer.advance(false); continue; }
                return result;
            }
            454 => {
                if lookahead == 101 { state = 564; lexer.advance(false); continue; }
                return result;
            }
            455 => {
                result = true; lexer.set_result_symbol(anon_sym_index); lexer.mark_end();
                return result;
            }
            456 => {
                result = true; lexer.set_result_symbol(anon_sym_ioctl); lexer.mark_end();
                return result;
            }
            457 => {
                if lookahead == 115 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            458 => {
                if lookahead == 104 { state = 566; lexer.advance(false); continue; }
                return result;
            }
            459 => {
                if lookahead == 110 { state = 567; lexer.advance(false); continue; }
                return result;
            }
            460 => {
                if lookahead == 116 { state = 568; lexer.advance(false); continue; }
                return result;
            }
            461 => {
                result = true; lexer.set_result_symbol(anon_sym_lstat); lexer.mark_end();
                return result;
            }
            462 => {
                result = true; lexer.set_result_symbol(anon_sym_mkdir); lexer.mark_end();
                return result;
            }
            463 => {
                if lookahead == 108 { state = 569; lexer.advance(false); continue; }
                return result;
            }
            464 => {
                if lookahead == 116 { state = 570; lexer.advance(false); continue; }
                return result;
            }
            465 => {
                if lookahead == 118 { state = 571; lexer.advance(false); continue; }
                return result;
            }
            466 => {
                if lookahead == 110 { state = 572; lexer.advance(false); continue; }
                return result;
            }
            467 => {
                if lookahead == 105 { state = 573; lexer.advance(false); continue; }
                return result;
            }
            468 => {
                if lookahead == 103 { state = 574; lexer.advance(false); continue; }
                return result;
            }
            469 => {
                result = true; lexer.set_result_symbol(anon_sym_print); lexer.mark_end();
                if lookahead == 102 { state = 575; lexer.advance(false); continue; }
                return result;
            }
            470 => {
                if lookahead == 116 { state = 576; lexer.advance(false); continue; }
                return result;
            }
            471 => {
                if lookahead == 109 { state = 577; lexer.advance(false); continue; }
                return result;
            }
            472 => {
                if lookahead == 105 { state = 578; lexer.advance(false); continue; }
                return result;
            }
            473 => {
                if lookahead == 101 { state = 579; lexer.advance(false); continue; }
                return result;
            }
            474 => {
                if lookahead == 114 { state = 580; lexer.advance(false); continue; }
                return result;
            }
            475 => {
                result = true; lexer.set_result_symbol(anon_sym_reset); lexer.mark_end();
                return result;
            }
            476 => {
                if lookahead == 110 { state = 581; lexer.advance(false); continue; }
                return result;
            }
            477 => {
                if lookahead == 115 { state = 582; lexer.advance(false); continue; }
                return result;
            }
            478 => {
                if lookahead == 100 { state = 583; lexer.advance(false); continue; }
                return result;
            }
            479 => {
                if lookahead == 120 { state = 584; lexer.advance(false); continue; }
                return result;
            }
            480 => {
                if lookahead == 105 { state = 585; lexer.advance(false); continue; }
                return result;
            }
            481 => {
                if lookahead == 116 { state = 586; lexer.advance(false); continue; }
                return result;
            }
            482 => {
                if lookahead == 108 { state = 587; lexer.advance(false); continue; }
                return result;
            }
            483 => {
                if lookahead == 116 { state = 588; lexer.advance(false); continue; }
                return result;
            }
            484 => {
                if lookahead == 114 { state = 589; lexer.advance(false); continue; }
                return result;
            }
            485 => {
                if lookahead == 105 { state = 590; lexer.advance(false); continue; }
                return result;
            }
            486 => {
                if lookahead == 99 { state = 591; lexer.advance(false); continue; }
                return result;
            }
            487 => {
                result = true; lexer.set_result_symbol(anon_sym_shift); lexer.mark_end();
                return result;
            }
            488 => {
                if lookahead == 108 { state = 592; lexer.advance(false); continue; }
                return result;
            }
            489 => {
                if lookahead == 97 { state = 593; lexer.advance(false); continue; }
                return result;
            }
            490 => {
                if lookahead == 105 { state = 594; lexer.advance(false); continue; }
                return result;
            }
            491 => {
                if lookahead == 111 { state = 595; lexer.advance(false); continue; }
                return result;
            }
            492 => {
                if lookahead == 116 { state = 596; lexer.advance(false); continue; }
                return result;
            }
            493 => {
                result = true; lexer.set_result_symbol(anon_sym_split); lexer.mark_end();
                return result;
            }
            494 => {
                if lookahead == 116 { state = 597; lexer.advance(false); continue; }
                return result;
            }
            495 => {
                result = true; lexer.set_result_symbol(anon_sym_srand); lexer.mark_end();
                return result;
            }
            496 => {
                result = true; lexer.set_result_symbol(anon_sym_state); lexer.mark_end();
                return result;
            }
            497 => {
                result = true; lexer.set_result_symbol(anon_sym_study); lexer.mark_end();
                return result;
            }
            498 => {
                if lookahead == 114 { state = 598; lexer.advance(false); continue; }
                return result;
            }
            499 => {
                if lookahead == 110 { state = 599; lexer.advance(false); continue; }
                return result;
            }
            500 => {
                if lookahead == 108 { state = 600; lexer.advance(false); continue; }
                return result;
            }
            501 => {
                if lookahead == 101 { state = 601; lexer.advance(false); continue; }
                return result;
            }
            502 => {
                if lookahead == 97 { state = 602; lexer.advance(false); continue; }
                return result;
            }
            503 => {
                if lookahead == 101 { state = 603; lexer.advance(false); continue; }
                return result;
            }
            504 => {
                if lookahead == 109 { state = 604; lexer.advance(false); continue; }
                return result;
            }
            505 => {
                if lookahead == 105 { state = 605; lexer.advance(false); continue; }
                return result;
            }
            506 => {
                if lookahead == 105 { state = 606; lexer.advance(false); continue; }
                return result;
            }
            507 => {
                result = true; lexer.set_result_symbol(anon_sym_times); lexer.mark_end();
                return result;
            }
            508 => {
                if lookahead == 97 { state = 607; lexer.advance(false); continue; }
                return result;
            }
            509 => {
                if lookahead == 115 { state = 608; lexer.advance(false); continue; }
                return result;
            }
            510 => {
                result = true; lexer.set_result_symbol(anon_sym_umask); lexer.mark_end();
                return result;
            }
            511 => {
                result = true; lexer.set_result_symbol(anon_sym_undef); lexer.mark_end();
                return result;
            }
            512 => {
                if lookahead == 115 { state = 609; lexer.advance(false); continue; }
                return result;
            }
            513 => {
                if lookahead == 107 { state = 610; lexer.advance(false); continue; }
                return result;
            }
            514 => {
                if lookahead == 107 { state = 611; lexer.advance(false); continue; }
                return result;
            }
            515 => {
                if lookahead == 102 { state = 612; lexer.advance(false); continue; }
                return result;
            }
            516 => {
                result = true; lexer.set_result_symbol(anon_sym_untie); lexer.mark_end();
                return result;
            }
            517 => {
                result = true; lexer.set_result_symbol(anon_sym_until); lexer.mark_end();
                return result;
            }
            518 => {
                result = true; lexer.set_result_symbol(anon_sym_utime); lexer.mark_end();
                return result;
            }
            519 => {
                if lookahead == 115 { state = 613; lexer.advance(false); continue; }
                return result;
            }
            520 => {
                if lookahead == 105 { state = 614; lexer.advance(false); continue; }
                return result;
            }
            521 => {
                if lookahead == 114 { state = 615; lexer.advance(false); continue; }
                return result;
            }
            522 => {
                result = true; lexer.set_result_symbol(anon_sym_while); lexer.mark_end();
                return result;
            }
            523 => {
                result = true; lexer.set_result_symbol(anon_sym_write); lexer.mark_end();
                return result;
            }
            524 => {
                result = true; lexer.set_result_symbol(anon_sym_ADJUST); lexer.mark_end();
                return result;
            }
            525 => {
                result = true; lexer.set_result_symbol(anon_sym_STDERR); lexer.mark_end();
                return result;
            }
            526 => {
                result = true; lexer.set_result_symbol(anon_sym_STDOUT); lexer.mark_end();
                return result;
            }
            527 => {
                if lookahead == 69 { state = 616; lexer.advance(false); continue; }
                return result;
            }
            528 => {
                if lookahead == 95 { state = 617; lexer.advance(false); continue; }
                return result;
            }
            529 => {
                if lookahead == 95 { state = 618; lexer.advance(false); continue; }
                return result;
            }
            530 => {
                if lookahead == 95 { state = 619; lexer.advance(false); continue; }
                return result;
            }
            531 => {
                if lookahead == 95 { state = 620; lexer.advance(false); continue; }
                return result;
            }
            532 => {
                if lookahead == 65 { state = 621; lexer.advance(false); continue; }
                return result;
            }
            533 => {
                if lookahead == 95 { state = 622; lexer.advance(false); continue; }
                return result;
            }
            534 => {
                result = true; lexer.set_result_symbol(anon_sym_accept); lexer.mark_end();
                return result;
            }
            535 => {
                if lookahead == 101 { state = 623; lexer.advance(false); continue; }
                return result;
            }
            536 => {
                result = true; lexer.set_result_symbol(anon_sym_chroot); lexer.mark_end();
                return result;
            }
            537 => {
                if lookahead == 116 { state = 624; lexer.advance(false); continue; }
                return result;
            }
            538 => {
                if lookahead == 117 { state = 625; lexer.advance(false); continue; }
                return result;
            }
            539 => {
                if lookahead == 115 { state = 626; lexer.advance(false); continue; }
                return result;
            }
            540 => {
                if lookahead == 110 { state = 627; lexer.advance(false); continue; }
                return result;
            }
            541 => {
                if lookahead == 100 { state = 628; lexer.advance(false); continue; }
                return result;
            }
            542 => {
                result = true; lexer.set_result_symbol(anon_sym_delete); lexer.mark_end();
                return result;
            }
            543 => {
                if lookahead == 99 { state = 629; lexer.advance(false); continue; }
                return result;
            }
            544 => {
                result = true; lexer.set_result_symbol(anon_sym_exists); lexer.mark_end();
                return result;
            }
            545 => {
                if lookahead == 101 { state = 630; lexer.advance(false); continue; }
                return result;
            }
            546 => {
                result = true; lexer.set_result_symbol(anon_sym_fileno); lexer.mark_end();
                return result;
            }
            547 => {
                if lookahead == 121 { state = 631; lexer.advance(false); continue; }
                return result;
            }
            548 => {
                if lookahead == 104 { state = 632; lexer.advance(false); continue; }
                return result;
            }
            549 => {
                result = true; lexer.set_result_symbol(anon_sym_format); lexer.mark_end();
                return result;
            }
            550 => {
                if lookahead == 110 { state = 633; lexer.advance(false); continue; }
                return result;
            }
            551 => {
                if lookahead == 105 { state = 634; lexer.advance(false); continue; }
                return result;
            }
            552 => {
                if lookahead == 97 { state = 635; lexer.advance(false); continue; }
                return result;
            }
            553 => {
                if lookahead == 116 { state = 636; lexer.advance(false); continue; }
                return result;
            }
            554 => {
                if lookahead == 98 { state = 637; lexer.advance(false); continue; }
                return result;
            }
            555 => {
                if lookahead == 114 { state = 638; lexer.advance(false); continue; }
                return result;
            }
            556 => {
                if lookahead == 112 { state = 639; lexer.advance(false); continue; }
                return result;
            }
            557 => {
                if lookahead == 100 { state = 640; lexer.advance(false); continue; }
                return result;
            }
            558 => {
                if lookahead == 111 { state = 641; lexer.advance(false); continue; }
                return result;
            }
            559 => {
                if lookahead == 116 { state = 642; lexer.advance(false); continue; }
                return result;
            }
            560 => {
                if lookahead == 97 { state = 643; lexer.advance(false); continue; }
                return result;
            }
            561 => {
                if lookahead == 105 { state = 644; lexer.advance(false); continue; }
                return result;
            }
            562 => {
                if lookahead == 118 { state = 645; lexer.advance(false); continue; }
                return result;
            }
            563 => {
                if lookahead == 107 { state = 646; lexer.advance(false); continue; }
                return result;
            }
            564 => {
                result = true; lexer.set_result_symbol(anon_sym_gmtime); lexer.mark_end();
                return result;
            }
            565 => {
                if lookahead == 116 { state = 647; lexer.advance(false); continue; }
                return result;
            }
            566 => {
                result = true; lexer.set_result_symbol(anon_sym_length); lexer.mark_end();
                return result;
            }
            567 => {
                result = true; lexer.set_result_symbol(anon_sym_listen); lexer.mark_end();
                return result;
            }
            568 => {
                if lookahead == 105 { state = 648; lexer.advance(false); continue; }
                return result;
            }
            569 => {
                result = true; lexer.set_result_symbol(anon_sym_msgctl); lexer.mark_end();
                return result;
            }
            570 => {
                result = true; lexer.set_result_symbol(anon_sym_msgget); lexer.mark_end();
                return result;
            }
            571 => {
                result = true; lexer.set_result_symbol(anon_sym_msgrcv); lexer.mark_end();
                return result;
            }
            572 => {
                if lookahead == 100 { state = 649; lexer.advance(false); continue; }
                return result;
            }
            573 => {
                if lookahead == 114 { state = 650; lexer.advance(false); continue; }
                return result;
            }
            574 => {
                if lookahead == 101 { state = 651; lexer.advance(false); continue; }
                return result;
            }
            575 => {
                result = true; lexer.set_result_symbol(anon_sym_printf); lexer.mark_end();
                return result;
            }
            576 => {
                if lookahead == 121 { state = 652; lexer.advance(false); continue; }
                return result;
            }
            577 => {
                if lookahead == 101 { state = 653; lexer.advance(false); continue; }
                return result;
            }
            578 => {
                if lookahead == 110 { state = 654; lexer.advance(false); continue; }
                return result;
            }
            579 => {
                result = true; lexer.set_result_symbol(anon_sym_rename); lexer.mark_end();
                return result;
            }
            580 => {
                if lookahead == 101 { state = 655; lexer.advance(false); continue; }
                return result;
            }
            581 => {
                result = true; lexer.set_result_symbol(anon_sym_return); lexer.mark_end();
                return result;
            }
            582 => {
                if lookahead == 101 { state = 656; lexer.advance(false); continue; }
                return result;
            }
            583 => {
                if lookahead == 100 { state = 657; lexer.advance(false); continue; }
                return result;
            }
            584 => {
                result = true; lexer.set_result_symbol(anon_sym_rindex); lexer.mark_end();
                return result;
            }
            585 => {
                if lookahead == 114 { state = 658; lexer.advance(false); continue; }
                return result;
            }
            586 => {
                result = true; lexer.set_result_symbol(anon_sym_select); lexer.mark_end();
                return result;
            }
            587 => {
                result = true; lexer.set_result_symbol(anon_sym_semctl); lexer.mark_end();
                return result;
            }
            588 => {
                result = true; lexer.set_result_symbol(anon_sym_semget); lexer.mark_end();
                return result;
            }
            589 => {
                if lookahead == 112 { state = 659; lexer.advance(false); continue; }
                return result;
            }
            590 => {
                if lookahead == 111 { state = 660; lexer.advance(false); continue; }
                return result;
            }
            591 => {
                if lookahead == 107 { state = 661; lexer.advance(false); continue; }
                return result;
            }
            592 => {
                result = true; lexer.set_result_symbol(anon_sym_shmctl); lexer.mark_end();
                return result;
            }
            593 => {
                if lookahead == 100 { state = 662; lexer.advance(false); continue; }
                return result;
            }
            594 => {
                if lookahead == 116 { state = 663; lexer.advance(false); continue; }
                return result;
            }
            595 => {
                if lookahead == 119 { state = 664; lexer.advance(false); continue; }
                return result;
            }
            596 => {
                result = true; lexer.set_result_symbol(anon_sym_socket); lexer.mark_end();
                if lookahead == 112 { state = 665; lexer.advance(false); continue; }
                return result;
            }
            597 => {
                if lookahead == 102 { state = 666; lexer.advance(false); continue; }
                return result;
            }
            598 => {
                result = true; lexer.set_result_symbol(anon_sym_substr); lexer.mark_end();
                return result;
            }
            599 => {
                if lookahead == 107 { state = 667; lexer.advance(false); continue; }
                return result;
            }
            600 => {
                if lookahead == 108 { state = 668; lexer.advance(false); continue; }
                return result;
            }
            601 => {
                if lookahead == 110 { state = 669; lexer.advance(false); continue; }
                return result;
            }
            602 => {
                if lookahead == 100 { state = 670; lexer.advance(false); continue; }
                return result;
            }
            603 => {
                if lookahead == 107 { state = 671; lexer.advance(false); continue; }
                return result;
            }
            604 => {
                result = true; lexer.set_result_symbol(anon_sym_system); lexer.mark_end();
                return result;
            }
            605 => {
                if lookahead == 116 { state = 672; lexer.advance(false); continue; }
                return result;
            }
            606 => {
                if lookahead == 114 { state = 673; lexer.advance(false); continue; }
                return result;
            }
            607 => {
                if lookahead == 116 { state = 674; lexer.advance(false); continue; }
                return result;
            }
            608 => {
                if lookahead == 116 { state = 675; lexer.advance(false); continue; }
                return result;
            }
            609 => {
                result = true; lexer.set_result_symbol(anon_sym_unless); lexer.mark_end();
                return result;
            }
            610 => {
                result = true; lexer.set_result_symbol(anon_sym_unlink); lexer.mark_end();
                return result;
            }
            611 => {
                result = true; lexer.set_result_symbol(anon_sym_unpack); lexer.mark_end();
                return result;
            }
            612 => {
                if lookahead == 116 { state = 676; lexer.advance(false); continue; }
                return result;
            }
            613 => {
                result = true; lexer.set_result_symbol(anon_sym_values); lexer.mark_end();
                return result;
            }
            614 => {
                if lookahead == 100 { state = 677; lexer.advance(false); continue; }
                return result;
            }
            615 => {
                if lookahead == 114 { state = 678; lexer.advance(false); continue; }
                return result;
            }
            616 => {
                if lookahead == 67 { state = 679; lexer.advance(false); continue; }
                return result;
            }
            617 => {
                if lookahead == 95 { state = 680; lexer.advance(false); continue; }
                return result;
            }
            618 => {
                result = true; lexer.set_result_symbol(anon_sym___END__); lexer.mark_end();
                return result;
            }
            619 => {
                if lookahead == 95 { state = 681; lexer.advance(false); continue; }
                return result;
            }
            620 => {
                if lookahead == 95 { state = 682; lexer.advance(false); continue; }
                return result;
            }
            621 => {
                if lookahead == 71 { state = 683; lexer.advance(false); continue; }
                return result;
            }
            622 => {
                result = true; lexer.set_result_symbol(anon_sym___SUB__); lexer.mark_end();
                return result;
            }
            623 => {
                result = true; lexer.set_result_symbol(anon_sym_binmode); lexer.mark_end();
                return result;
            }
            624 => {
                result = true; lexer.set_result_symbol(anon_sym_connect); lexer.mark_end();
                return result;
            }
            625 => {
                if lookahead == 101 { state = 684; lexer.advance(false); continue; }
                return result;
            }
            626 => {
                if lookahead == 101 { state = 685; lexer.advance(false); continue; }
                return result;
            }
            627 => {
                result = true; lexer.set_result_symbol(anon_sym_dbmopen); lexer.mark_end();
                return result;
            }
            628 => {
                result = true; lexer.set_result_symbol(anon_sym_defined); lexer.mark_end();
                return result;
            }
            629 => {
                if lookahead == 97 { state = 686; lexer.advance(false); continue; }
                return result;
            }
            630 => {
                if lookahead == 100 { state = 687; lexer.advance(false); continue; }
                return result;
            }
            631 => {
                result = true; lexer.set_result_symbol(anon_sym_finally); lexer.mark_end();
                return result;
            }
            632 => {
                result = true; lexer.set_result_symbol(anon_sym_foreach); lexer.mark_end();
                return result;
            }
            633 => {
                if lookahead == 101 { state = 688; lexer.advance(false); continue; }
                return result;
            }
            634 => {
                if lookahead == 100 { state = 689; lexer.advance(false); continue; }
                return result;
            }
            635 => {
                if lookahead == 109 { state = 690; lexer.advance(false); continue; }
                return result;
            }
            636 => {
                if lookahead == 98 { state = 691; lexer.advance(false); continue; }
                return result;
            }
            637 => {
                if lookahead == 121 { state = 692; lexer.advance(false); continue; }
                return result;
            }
            638 => {
                if lookahead == 110 { state = 693; lexer.advance(false); continue; }
                return result;
            }
            639 => {
                result = true; lexer.set_result_symbol(anon_sym_getpgrp); lexer.mark_end();
                return result;
            }
            640 => {
                result = true; lexer.set_result_symbol(anon_sym_getppid); lexer.mark_end();
                return result;
            }
            641 => {
                if lookahead == 114 { state = 694; lexer.advance(false); continue; }
                return result;
            }
            642 => {
                if lookahead == 111 { state = 695; lexer.advance(false); continue; }
                return result;
            }
            643 => {
                if lookahead == 109 { state = 696; lexer.advance(false); continue; }
                return result;
            }
            644 => {
                if lookahead == 100 { state = 697; lexer.advance(false); continue; }
                return result;
            }
            645 => {
                if lookahead == 98 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            646 => {
                if lookahead == 110 { state = 699; lexer.advance(false); continue; }
                if lookahead == 111 { state = 700; lexer.advance(false); continue; }
                return result;
            }
            647 => {
                result = true; lexer.set_result_symbol(anon_sym_lcfirst); lexer.mark_end();
                return result;
            }
            648 => {
                if lookahead == 109 { state = 701; lexer.advance(false); continue; }
                return result;
            }
            649 => {
                result = true; lexer.set_result_symbol(anon_sym_msgsend); lexer.mark_end();
                return result;
            }
            650 => {
                result = true; lexer.set_result_symbol(anon_sym_opendir); lexer.mark_end();
                return result;
            }
            651 => {
                result = true; lexer.set_result_symbol(anon_sym_package); lexer.mark_end();
                return result;
            }
            652 => {
                if lookahead == 112 { state = 702; lexer.advance(false); continue; }
                return result;
            }
            653 => {
                if lookahead == 116 { state = 703; lexer.advance(false); continue; }
                return result;
            }
            654 => {
                if lookahead == 101 { state = 704; lexer.advance(false); continue; }
                if lookahead == 107 { state = 705; lexer.advance(false); continue; }
                return result;
            }
            655 => {
                result = true; lexer.set_result_symbol(anon_sym_require); lexer.mark_end();
                return result;
            }
            656 => {
                result = true; lexer.set_result_symbol(anon_sym_reverse); lexer.mark_end();
                return result;
            }
            657 => {
                if lookahead == 105 { state = 706; lexer.advance(false); continue; }
                return result;
            }
            658 => {
                result = true; lexer.set_result_symbol(anon_sym_seekdir); lexer.mark_end();
                return result;
            }
            659 => {
                result = true; lexer.set_result_symbol(anon_sym_setpgrp); lexer.mark_end();
                return result;
            }
            660 => {
                if lookahead == 114 { state = 707; lexer.advance(false); continue; }
                return result;
            }
            661 => {
                if lookahead == 111 { state = 708; lexer.advance(false); continue; }
                return result;
            }
            662 => {
                result = true; lexer.set_result_symbol(anon_sym_shmread); lexer.mark_end();
                return result;
            }
            663 => {
                if lookahead == 101 { state = 709; lexer.advance(false); continue; }
                return result;
            }
            664 => {
                if lookahead == 110 { state = 710; lexer.advance(false); continue; }
                return result;
            }
            665 => {
                if lookahead == 97 { state = 711; lexer.advance(false); continue; }
                return result;
            }
            666 => {
                result = true; lexer.set_result_symbol(anon_sym_sprintf); lexer.mark_end();
                return result;
            }
            667 => {
                result = true; lexer.set_result_symbol(anon_sym_symlink); lexer.mark_end();
                return result;
            }
            668 => {
                result = true; lexer.set_result_symbol(anon_sym_syscall); lexer.mark_end();
                return result;
            }
            669 => {
                result = true; lexer.set_result_symbol(anon_sym_sysopen); lexer.mark_end();
                return result;
            }
            670 => {
                result = true; lexer.set_result_symbol(anon_sym_sysread); lexer.mark_end();
                return result;
            }
            671 => {
                result = true; lexer.set_result_symbol(anon_sym_sysseek); lexer.mark_end();
                return result;
            }
            672 => {
                if lookahead == 101 { state = 712; lexer.advance(false); continue; }
                return result;
            }
            673 => {
                result = true; lexer.set_result_symbol(anon_sym_telldir); lexer.mark_end();
                return result;
            }
            674 => {
                if lookahead == 101 { state = 713; lexer.advance(false); continue; }
                return result;
            }
            675 => {
                result = true; lexer.set_result_symbol(anon_sym_ucfirst); lexer.mark_end();
                return result;
            }
            676 => {
                result = true; lexer.set_result_symbol(anon_sym_unshift); lexer.mark_end();
                return result;
            }
            677 => {
                result = true; lexer.set_result_symbol(anon_sym_waitpid); lexer.mark_end();
                return result;
            }
            678 => {
                if lookahead == 97 { state = 714; lexer.advance(false); continue; }
                return result;
            }
            679 => {
                if lookahead == 75 { state = 715; lexer.advance(false); continue; }
                return result;
            }
            680 => {
                result = true; lexer.set_result_symbol(anon_sym___DATA__); lexer.mark_end();
                return result;
            }
            681 => {
                result = true; lexer.set_result_symbol(anon_sym___FILE__); lexer.mark_end();
                return result;
            }
            682 => {
                result = true; lexer.set_result_symbol(anon_sym___LINE__); lexer.mark_end();
                return result;
            }
            683 => {
                if lookahead == 69 { state = 716; lexer.advance(false); continue; }
                return result;
            }
            684 => {
                result = true; lexer.set_result_symbol(anon_sym_continue); lexer.mark_end();
                return result;
            }
            685 => {
                result = true; lexer.set_result_symbol(anon_sym_dbmclose); lexer.mark_end();
                return result;
            }
            686 => {
                if lookahead == 108 { state = 717; lexer.advance(false); continue; }
                return result;
            }
            687 => {
                result = true; lexer.set_result_symbol(anon_sym_extended); lexer.mark_end();
                return result;
            }
            688 => {
                result = true; lexer.set_result_symbol(anon_sym_formline); lexer.mark_end();
                return result;
            }
            689 => {
                result = true; lexer.set_result_symbol(anon_sym_getgrgid); lexer.mark_end();
                return result;
            }
            690 => {
                result = true; lexer.set_result_symbol(anon_sym_getgrnam); lexer.mark_end();
                return result;
            }
            691 => {
                if lookahead == 121 { state = 718; lexer.advance(false); continue; }
                return result;
            }
            692 => {
                if lookahead == 97 { state = 719; lexer.advance(false); continue; }
                if lookahead == 110 { state = 720; lexer.advance(false); continue; }
                return result;
            }
            693 => {
                if lookahead == 97 { state = 721; lexer.advance(false); continue; }
                return result;
            }
            694 => {
                if lookahead == 105 { state = 722; lexer.advance(false); continue; }
                return result;
            }
            695 => {
                if lookahead == 98 { state = 723; lexer.advance(false); continue; }
                return result;
            }
            696 => {
                if lookahead == 101 { state = 724; lexer.advance(false); continue; }
                return result;
            }
            697 => {
                result = true; lexer.set_result_symbol(anon_sym_getpwuid); lexer.mark_end();
                return result;
            }
            698 => {
                if lookahead == 121 { state = 725; lexer.advance(false); continue; }
                return result;
            }
            699 => {
                if lookahead == 97 { state = 726; lexer.advance(false); continue; }
                return result;
            }
            700 => {
                if lookahead == 112 { state = 727; lexer.advance(false); continue; }
                return result;
            }
            701 => {
                if lookahead == 101 { state = 728; lexer.advance(false); continue; }
                return result;
            }
            702 => {
                if lookahead == 101 { state = 729; lexer.advance(false); continue; }
                return result;
            }
            703 => {
                if lookahead == 97 { state = 730; lexer.advance(false); continue; }
                return result;
            }
            704 => {
                result = true; lexer.set_result_symbol(anon_sym_readline); lexer.mark_end();
                return result;
            }
            705 => {
                result = true; lexer.set_result_symbol(anon_sym_readlink); lexer.mark_end();
                return result;
            }
            706 => {
                if lookahead == 114 { state = 731; lexer.advance(false); continue; }
                return result;
            }
            707 => {
                if lookahead == 105 { state = 732; lexer.advance(false); continue; }
                return result;
            }
            708 => {
                if lookahead == 112 { state = 733; lexer.advance(false); continue; }
                return result;
            }
            709 => {
                result = true; lexer.set_result_symbol(anon_sym_shmwrite); lexer.mark_end();
                return result;
            }
            710 => {
                result = true; lexer.set_result_symbol(anon_sym_shutdown); lexer.mark_end();
                return result;
            }
            711 => {
                if lookahead == 105 { state = 734; lexer.advance(false); continue; }
                return result;
            }
            712 => {
                result = true; lexer.set_result_symbol(anon_sym_syswrite); lexer.mark_end();
                return result;
            }
            713 => {
                result = true; lexer.set_result_symbol(anon_sym_truncate); lexer.mark_end();
                return result;
            }
            714 => {
                if lookahead == 121 { state = 735; lexer.advance(false); continue; }
                return result;
            }
            715 => {
                result = true; lexer.set_result_symbol(anon_sym_UNITCHECK); lexer.mark_end();
                return result;
            }
            716 => {
                if lookahead == 95 { state = 736; lexer.advance(false); continue; }
                return result;
            }
            717 => {
                if lookahead == 108 { state = 737; lexer.advance(false); continue; }
                return result;
            }
            718 => {
                if lookahead == 97 { state = 738; lexer.advance(false); continue; }
                return result;
            }
            719 => {
                if lookahead == 100 { state = 739; lexer.advance(false); continue; }
                return result;
            }
            720 => {
                if lookahead == 97 { state = 740; lexer.advance(false); continue; }
                return result;
            }
            721 => {
                if lookahead == 109 { state = 741; lexer.advance(false); continue; }
                return result;
            }
            722 => {
                if lookahead == 116 { state = 742; lexer.advance(false); continue; }
                return result;
            }
            723 => {
                if lookahead == 121 { state = 743; lexer.advance(false); continue; }
                return result;
            }
            724 => {
                result = true; lexer.set_result_symbol(anon_sym_getpwname); lexer.mark_end();
                return result;
            }
            725 => {
                if lookahead == 110 { state = 744; lexer.advance(false); continue; }
                if lookahead == 112 { state = 745; lexer.advance(false); continue; }
                return result;
            }
            726 => {
                if lookahead == 109 { state = 746; lexer.advance(false); continue; }
                return result;
            }
            727 => {
                if lookahead == 116 { state = 747; lexer.advance(false); continue; }
                return result;
            }
            728 => {
                result = true; lexer.set_result_symbol(anon_sym_localtime); lexer.mark_end();
                return result;
            }
            729 => {
                result = true; lexer.set_result_symbol(anon_sym_prototype); lexer.mark_end();
                return result;
            }
            730 => {
                result = true; lexer.set_result_symbol(anon_sym_quotemeta); lexer.mark_end();
                return result;
            }
            731 => {
                result = true; lexer.set_result_symbol(anon_sym_rewinddir); lexer.mark_end();
                return result;
            }
            732 => {
                if lookahead == 116 { state = 748; lexer.advance(false); continue; }
                return result;
            }
            733 => {
                if lookahead == 116 { state = 749; lexer.advance(false); continue; }
                return result;
            }
            734 => {
                if lookahead == 114 { state = 750; lexer.advance(false); continue; }
                return result;
            }
            735 => {
                result = true; lexer.set_result_symbol(anon_sym_wantarray); lexer.mark_end();
                return result;
            }
            736 => {
                if lookahead == 95 { state = 751; lexer.advance(false); continue; }
                return result;
            }
            737 => {
                if lookahead == 121 { state = 752; lexer.advance(false); continue; }
                return result;
            }
            738 => {
                if lookahead == 100 { state = 753; lexer.advance(false); continue; }
                return result;
            }
            739 => {
                if lookahead == 100 { state = 754; lexer.advance(false); continue; }
                return result;
            }
            740 => {
                if lookahead == 109 { state = 755; lexer.advance(false); continue; }
                return result;
            }
            741 => {
                if lookahead == 101 { state = 756; lexer.advance(false); continue; }
                return result;
            }
            742 => {
                if lookahead == 121 { state = 757; lexer.advance(false); continue; }
                return result;
            }
            743 => {
                if lookahead == 110 { state = 758; lexer.advance(false); continue; }
                return result;
            }
            744 => {
                if lookahead == 97 { state = 759; lexer.advance(false); continue; }
                return result;
            }
            745 => {
                if lookahead == 111 { state = 760; lexer.advance(false); continue; }
                return result;
            }
            746 => {
                if lookahead == 101 { state = 761; lexer.advance(false); continue; }
                return result;
            }
            747 => {
                result = true; lexer.set_result_symbol(anon_sym_getsockopt); lexer.mark_end();
                return result;
            }
            748 => {
                if lookahead == 121 { state = 762; lexer.advance(false); continue; }
                return result;
            }
            749 => {
                result = true; lexer.set_result_symbol(anon_sym_setsockopt); lexer.mark_end();
                return result;
            }
            750 => {
                result = true; lexer.set_result_symbol(anon_sym_socketpair); lexer.mark_end();
                return result;
            }
            751 => {
                result = true; lexer.set_result_symbol(anon_sym___PACKAGE__); lexer.mark_end();
                return result;
            }
            752 => {
                result = true; lexer.set_result_symbol(anon_sym_dynamically); lexer.mark_end();
                return result;
            }
            753 => {
                if lookahead == 100 { state = 763; lexer.advance(false); continue; }
                return result;
            }
            754 => {
                if lookahead == 114 { state = 764; lexer.advance(false); continue; }
                return result;
            }
            755 => {
                if lookahead == 101 { state = 765; lexer.advance(false); continue; }
                return result;
            }
            756 => {
                result = true; lexer.set_result_symbol(anon_sym_getpeername); lexer.mark_end();
                return result;
            }
            757 => {
                result = true; lexer.set_result_symbol(anon_sym_getpriority); lexer.mark_end();
                return result;
            }
            758 => {
                if lookahead == 97 { state = 766; lexer.advance(false); continue; }
                if lookahead == 117 { state = 767; lexer.advance(false); continue; }
                return result;
            }
            759 => {
                if lookahead == 109 { state = 768; lexer.advance(false); continue; }
                return result;
            }
            760 => {
                if lookahead == 114 { state = 769; lexer.advance(false); continue; }
                return result;
            }
            761 => {
                result = true; lexer.set_result_symbol(anon_sym_getsockname); lexer.mark_end();
                return result;
            }
            762 => {
                result = true; lexer.set_result_symbol(anon_sym_setpriority); lexer.mark_end();
                return result;
            }
            763 => {
                if lookahead == 114 { state = 770; lexer.advance(false); continue; }
                return result;
            }
            764 => {
                result = true; lexer.set_result_symbol(anon_sym_getnetbyaddr); lexer.mark_end();
                return result;
            }
            765 => {
                result = true; lexer.set_result_symbol(anon_sym_getnetbyname); lexer.mark_end();
                return result;
            }
            766 => {
                if lookahead == 109 { state = 771; lexer.advance(false); continue; }
                return result;
            }
            767 => {
                if lookahead == 109 { state = 772; lexer.advance(false); continue; }
                return result;
            }
            768 => {
                if lookahead == 101 { state = 773; lexer.advance(false); continue; }
                return result;
            }
            769 => {
                if lookahead == 116 { state = 774; lexer.advance(false); continue; }
                return result;
            }
            770 => {
                result = true; lexer.set_result_symbol(anon_sym_gethostbyaddr); lexer.mark_end();
                return result;
            }
            771 => {
                if lookahead == 101 { state = 775; lexer.advance(false); continue; }
                return result;
            }
            772 => {
                if lookahead == 98 { state = 776; lexer.advance(false); continue; }
                return result;
            }
            773 => {
                result = true; lexer.set_result_symbol(anon_sym_getservbyname); lexer.mark_end();
                return result;
            }
            774 => {
                result = true; lexer.set_result_symbol(anon_sym_getservbyport); lexer.mark_end();
                return result;
            }
            775 => {
                result = true; lexer.set_result_symbol(anon_sym_getprotobyname); lexer.mark_end();
                return result;
            }
            776 => {
                if lookahead == 101 { state = 777; lexer.advance(false); continue; }
                return result;
            }
            777 => {
                if lookahead == 114 { state = 778; lexer.advance(false); continue; }
                return result;
            }
            778 => {
                result = true; lexer.set_result_symbol(anon_sym_getprotobynumber); lexer.mark_end();
                return result;
            }
            _ => return false,
        }
    }
}
