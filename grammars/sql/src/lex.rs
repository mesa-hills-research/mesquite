//! The `sql` grammar's lexer: `ts_lex` and `ts_lex_keywords`, transliterated from
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

const anon_sym_AT: Symbol = 408;
const anon_sym_BANG_EQ: Symbol = 395;
const anon_sym_BQUOTE: Symbol = 407;
const anon_sym_CARET: Symbol = 394;
const anon_sym_COLON: Symbol = 388;
const anon_sym_COLON_COLON: Symbol = 383;
const anon_sym_COLON_EQ: Symbol = 379;
const anon_sym_COMMA: Symbol = 374;
const anon_sym_DASH: Symbol = 391;
const anon_sym_DOT: Symbol = 380;
const anon_sym_EQ: Symbol = 378;
const anon_sym_GT: Symbol = 385;
const anon_sym_GT_EQ: Symbol = 387;
const anon_sym_LBRACK: Symbol = 370;
const anon_sym_LPAREN: Symbol = 372;
const anon_sym_LT: Symbol = 384;
const anon_sym_LT_EQ: Symbol = 386;
const anon_sym_LT_GT: Symbol = 396;
const anon_sym_PERCENT: Symbol = 393;
const anon_sym_PLUS: Symbol = 390;
const anon_sym_RBRACK: Symbol = 371;
const anon_sym_RPAREN: Symbol = 373;
const anon_sym_SEMI: Symbol = 2;
const anon_sym_SLASH: Symbol = 392;
const anon_sym_STAR: Symbol = 381;
const aux_sym__bit_string_token1: Symbol = 405;
const aux_sym__decimal_number_token1: Symbol = 404;
const aux_sym__integer_token1: Symbol = 403;
const aux_sym__single_quote_string_token1: Symbol = 399;
const aux_sym__single_quote_string_token2: Symbol = 400;
const aux_sym_double_token1: Symbol = 375;
const aux_sym_keyword_bigint_token1: Symbol = 324;
const aux_sym_keyword_bigint_token2: Symbol = 325;
const aux_sym_keyword_bigserial_token1: Symbol = 313;
const aux_sym_keyword_bigserial_token2: Symbol = 314;
const aux_sym_keyword_char_token1: Symbol = 337;
const aux_sym_keyword_character_token1: Symbol = 95;
const aux_sym_keyword_int_token1: Symbol = 321;
const aux_sym_keyword_int_token2: Symbol = 322;
const aux_sym_keyword_int_token3: Symbol = 323;
const aux_sym_keyword_like_token1: Symbol = 165;
const aux_sym_keyword_like_token2: Symbol = 166;
const aux_sym_keyword_mediumint_token1: Symbol = 319;
const aux_sym_keyword_mediumint_token2: Symbol = 320;
const aux_sym_keyword_real_token1: Symbol = 328;
const aux_sym_keyword_real_token2: Symbol = 329;
const aux_sym_keyword_serial_token1: Symbol = 311;
const aux_sym_keyword_serial_token2: Symbol = 312;
const aux_sym_keyword_smallint_token1: Symbol = 317;
const aux_sym_keyword_smallint_token2: Symbol = 318;
const aux_sym_keyword_smallserial_token1: Symbol = 309;
const aux_sym_keyword_smallserial_token2: Symbol = 310;
const aux_sym_keyword_tinyint_token1: Symbol = 315;
const aux_sym_keyword_tinyint_token2: Symbol = 316;
const aux_sym_keyword_varchar_token1: Symbol = 339;
const sym__double_quote_string: Symbol = 398;
const sym__identifier: Symbol = 1;
const sym__natural_number: Symbol = 402;
const sym__postgres_escape_string: Symbol = 401;
const sym_bang: Symbol = 406;
const sym_comment: Symbol = 376;
const sym_keyword_action: Symbol = 187;
const sym_keyword_add: Symbol = 49;
const sym_keyword_admin: Symbol = 111;
const sym_keyword_after: Symbol = 138;
const sym_keyword_all: Symbol = 125;
const sym_keyword_alter: Symbol = 42;
const sym_keyword_always: Symbol = 93;
const sym_keyword_analyze: Symbol = 44;
const sym_keyword_and: Symbol = 79;
const sym_keyword_any: Symbol = 126;
const sym_keyword_array: Symbol = 369;
const sym_keyword_as: Symbol = 67;
const sym_keyword_asc: Symbol = 37;
const sym_keyword_atomic: Symbol = 211;
const sym_keyword_attribute: Symbol = 185;
const sym_keyword_authorization: Symbol = 186;
const sym_keyword_auto_increment: Symbol = 91;
const sym_keyword_avro: Symbol = 297;
const sym_keyword_before: Symbol = 139;
const sym_keyword_begin: Symbol = 131;
const sym_keyword_between: Symbol = 145;
const sym_keyword_bin_pack: Symbol = 277;
const sym_keyword_binary: Symbol = 306;
const sym_keyword_bit: Symbol = 305;
const sym_keyword_boolean: Symbol = 304;
const sym_keyword_box2d: Symbol = 360;
const sym_keyword_box3d: Symbol = 361;
const sym_keyword_brin: Symbol = 164;
const sym_keyword_btree: Symbol = 159;
const sym_keyword_by: Symbol = 34;
const sym_keyword_bytea: Symbol = 347;
const sym_keyword_cache: Symbol = 290;
const sym_keyword_cached: Symbol = 268;
const sym_keyword_called: Symbol = 222;
const sym_keyword_cascade: Symbol = 98;
const sym_keyword_cascaded: Symbol = 177;
const sym_keyword_case: Symbol = 73;
const sym_keyword_cast: Symbol = 71;
const sym_keyword_change: Symbol = 43;
const sym_keyword_characteristics: Symbol = 251;
const sym_keyword_check: Symbol = 180;
const sym_keyword_collate: Symbol = 94;
const sym_keyword_column: Symbol = 53;
const sym_keyword_columns: Symbol = 54;
const sym_keyword_comment: Symbol = 281;
const sym_keyword_commit: Symbol = 132;
const sym_keyword_committed: Symbol = 242;
const sym_keyword_compression: Symbol = 205;
const sym_keyword_compute: Symbol = 272;
const sym_keyword_concurrently: Symbol = 158;
const sym_keyword_conflict: Symbol = 170;
const sym_keyword_connection: Symbol = 116;
const sym_keyword_constraint: Symbol = 69;
const sym_keyword_constraints: Symbol = 249;
const sym_keyword_copy: Symbol = 189;
const sym_keyword_cost: Symbol = 225;
const sym_keyword_create: Symbol = 41;
const sym_keyword_cross: Symbol = 24;
const sym_keyword_csv: Symbol = 295;
const sym_keyword_current: Symbol = 150;
const sym_keyword_current_timestamp: Symbol = 179;
const sym_keyword_cycle: Symbol = 123;
const sym_keyword_data: Symbol = 103;
const sym_keyword_database: Symbol = 107;
const sym_keyword_date: Symbol = 349;
const sym_keyword_datetime: Symbol = 350;
const sym_keyword_datetime2: Symbol = 351;
const sym_keyword_datetimeoffset: Symbol = 353;
const sym_keyword_decimal: Symbol = 326;
const sym_keyword_declare: Symbol = 212;
const sym_keyword_default: Symbol = 97;
const sym_keyword_deferrable: Symbol = 244;
const sym_keyword_deferred: Symbol = 248;
const sym_keyword_definer: Symbol = 227;
const sym_keyword_delayed: Symbol = 175;
const sym_keyword_delete: Symbol = 4;
const sym_keyword_delimited: Symbol = 284;
const sym_keyword_delimiter: Symbol = 285;
const sym_keyword_desc: Symbol = 36;
const sym_keyword_distinct: Symbol = 68;
const sym_keyword_do: Symbol = 171;
const sym_keyword_double: Symbol = 331;
const sym_keyword_drop: Symbol = 48;
const sym_keyword_duplicate: Symbol = 66;
const sym_keyword_each: Symbol = 254;
const sym_keyword_else: Symbol = 76;
const sym_keyword_encoding: Symbol = 193;
const sym_keyword_encrypted: Symbol = 113;
const sym_keyword_end: Symbol = 77;
const sym_keyword_engine: Symbol = 96;
const sym_keyword_enum: Symbol = 348;
const sym_keyword_escape: Symbol = 192;
const sym_keyword_escaped: Symbol = 288;
const sym_keyword_except: Symbol = 128;
const sym_keyword_exclude: Symbol = 149;
const sym_keyword_execute: Symbol = 262;
const sym_keyword_exists: Symbol = 90;
const sym_keyword_explain: Symbol = 45;
const sym_keyword_extended: Symbol = 202;
const sym_keyword_extension: Symbol = 188;
const sym_keyword_external: Symbol = 265;
const sym_keyword_false: Symbol = 303;
const sym_keyword_fields: Symbol = 286;
const sym_keyword_filter: Symbol = 70;
const sym_keyword_first: Symbol = 137;
const sym_keyword_float: Symbol = 330;
const sym_keyword_following: Symbol = 148;
const sym_keyword_follows: Symbol = 252;
const sym_keyword_for: Symbol = 88;
const sym_keyword_force: Symbol = 83;
const sym_keyword_force_not_null: Symbol = 197;
const sym_keyword_force_null: Symbol = 196;
const sym_keyword_force_quote: Symbol = 194;
const sym_keyword_foreign: Symbol = 156;
const sym_keyword_format: Symbol = 283;
const sym_keyword_freeze: Symbol = 191;
const sym_keyword_from: Symbol = 18;
const sym_keyword_full: Symbol = 22;
const sym_keyword_function: Symbol = 207;
const sym_keyword_generated: Symbol = 92;
const sym_keyword_geography: Symbol = 359;
const sym_keyword_geometry: Symbol = 358;
const sym_keyword_gin: Symbol = 163;
const sym_keyword_gist: Symbol = 161;
const sym_keyword_group: Symbol = 32;
const sym_keyword_groups: Symbol = 144;
const sym_keyword_hash: Symbol = 160;
const sym_keyword_having: Symbol = 35;
const sym_keyword_header: Symbol = 198;
const sym_keyword_high_priority: Symbol = 173;
const sym_keyword_if: Symbol = 89;
const sym_keyword_ignore: Symbol = 84;
const sym_keyword_image: Symbol = 308;
const sym_keyword_immediate: Symbol = 247;
const sym_keyword_immutable: Symbol = 214;
const sym_keyword_in: Symbol = 78;
const sym_keyword_increment: Symbol = 58;
const sym_keyword_incremental: Symbol = 278;
const sym_keyword_index: Symbol = 87;
const sym_keyword_inet: Symbol = 333;
const sym_keyword_initially: Symbol = 257;
const sym_keyword_inner: Symbol = 21;
const sym_keyword_inout: Symbol = 232;
const sym_keyword_input: Symbol = 223;
const sym_keyword_insert: Symbol = 5;
const sym_keyword_instead: Symbol = 255;
const sym_keyword_intersect: Symbol = 129;
const sym_keyword_interval: Symbol = 357;
const sym_keyword_into: Symbol = 12;
const sym_keyword_invoker: Symbol = 228;
const sym_keyword_is: Symbol = 81;
const sym_keyword_isolation: Symbol = 236;
const sym_keyword_join: Symbol = 25;
const sym_keyword_json: Symbol = 344;
const sym_keyword_jsonb: Symbol = 345;
const sym_keyword_jsonfile: Symbol = 300;
const sym_keyword_key: Symbol = 65;
const sym_keyword_language: Symbol = 213;
const sym_keyword_last: Symbol = 140;
const sym_keyword_lateral: Symbol = 26;
const sym_keyword_leakproof: Symbol = 217;
const sym_keyword_left: Symbol = 19;
const sym_keyword_level: Symbol = 237;
const sym_keyword_limit: Symbol = 38;
const sym_keyword_lines: Symbol = 289;
const sym_keyword_local: Symbol = 178;
const sym_keyword_location: Symbol = 279;
const sym_keyword_logged: Symbol = 122;
const sym_keyword_low_priority: Symbol = 174;
const sym_keyword_main: Symbol = 203;
const sym_keyword_match: Symbol = 199;
const sym_keyword_matched: Symbol = 16;
const sym_keyword_materialized: Symbol = 55;
const sym_keyword_maxvalue: Symbol = 60;
const sym_keyword_merge: Symbol = 9;
const sym_keyword_metadata: Symbol = 291;
const sym_keyword_minvalue: Symbol = 59;
const sym_keyword_modify: Symbol = 47;
const sym_keyword_money: Symbol = 334;
const sym_keyword_name: Symbol = 364;
const sym_keyword_names: Symbol = 245;
const sym_keyword_natural: Symbol = 27;
const sym_keyword_nchar: Symbol = 338;
const sym_keyword_new: Symbol = 259;
const sym_keyword_no: Symbol = 102;
const sym_keyword_none: Symbol = 61;
const sym_keyword_noscan: Symbol = 292;
const sym_keyword_not: Symbol = 82;
const sym_keyword_nothing: Symbol = 172;
const sym_keyword_nowait: Symbol = 184;
const sym_keyword_null: Symbol = 301;
const sym_keyword_nulls: Symbol = 136;
const sym_keyword_numeric: Symbol = 327;
const sym_keyword_nvarchar: Symbol = 340;
const sym_keyword_object_id: Symbol = 264;
const sym_keyword_of: Symbol = 256;
const sym_keyword_off: Symbol = 29;
const sym_keyword_offset: Symbol = 39;
const sym_keyword_oid: Symbol = 362;
const sym_keyword_oids: Symbol = 363;
const sym_keyword_old: Symbol = 258;
const sym_keyword_on: Symbol = 28;
const sym_keyword_only: Symbol = 154;
const sym_keyword_optimize: Symbol = 275;
const sym_keyword_option: Symbol = 181;
const sym_keyword_or: Symbol = 80;
const sym_keyword_orc: Symbol = 299;
const sym_keyword_order: Symbol = 31;
const sym_keyword_ordinality: Symbol = 234;
const sym_keyword_others: Symbol = 153;
const sym_keyword_out: Symbol = 231;
const sym_keyword_outer: Symbol = 23;
const sym_keyword_over: Symbol = 135;
const sym_keyword_overwrite: Symbol = 13;
const sym_keyword_owned: Symbol = 62;
const sym_keyword_owner: Symbol = 109;
const sym_keyword_parallel: Symbol = 218;
const sym_keyword_parquet: Symbol = 293;
const sym_keyword_partition: Symbol = 33;
const sym_keyword_partitioned: Symbol = 280;
const sym_keyword_password: Symbol = 112;
const sym_keyword_plain: Symbol = 201;
const sym_keyword_precedes: Symbol = 253;
const sym_keyword_preceding: Symbol = 147;
const sym_keyword_precision: Symbol = 332;
const sym_keyword_primary: Symbol = 40;
const sym_keyword_procedure: Symbol = 263;
const sym_keyword_program: Symbol = 200;
const sym_keyword_quote: Symbol = 195;
const sym_keyword_range: Symbol = 142;
const sym_keyword_rcfile: Symbol = 294;
const sym_keyword_read: Symbol = 240;
const sym_keyword_recursive: Symbol = 176;
const sym_keyword_references: Symbol = 157;
const sym_keyword_referencing: Symbol = 260;
const sym_keyword_regclass: Symbol = 365;
const sym_keyword_regnamespace: Symbol = 366;
const sym_keyword_regproc: Symbol = 367;
const sym_keyword_regtype: Symbol = 368;
const sym_keyword_rename: Symbol = 105;
const sym_keyword_repeatable: Symbol = 239;
const sym_keyword_replace: Symbol = 6;
const sym_keyword_replication: Symbol = 270;
const sym_keyword_reset: Symbol = 118;
const sym_keyword_restart: Symbol = 64;
const sym_keyword_restrict: Symbol = 99;
const sym_keyword_restricted: Symbol = 221;
const sym_keyword_return: Symbol = 209;
const sym_keyword_returning: Symbol = 130;
const sym_keyword_returns: Symbol = 208;
const sym_keyword_rewrite: Symbol = 276;
const sym_keyword_right: Symbol = 20;
const sym_keyword_role: Symbol = 117;
const sym_keyword_rollback: Symbol = 133;
const sym_keyword_row: Symbol = 151;
const sym_keyword_rows: Symbol = 143;
const sym_keyword_safe: Symbol = 219;
const sym_keyword_schema: Symbol = 108;
const sym_keyword_security: Symbol = 229;
const sym_keyword_select: Symbol = 3;
const sym_keyword_separator: Symbol = 72;
const sym_keyword_sequence: Symbol = 57;
const sym_keyword_sequencefile: Symbol = 298;
const sym_keyword_serializable: Symbol = 238;
const sym_keyword_session: Symbol = 235;
const sym_keyword_set: Symbol = 17;
const sym_keyword_setof: Symbol = 210;
const sym_keyword_show: Symbol = 10;
const sym_keyword_similar: Symbol = 167;
const sym_keyword_smalldatetime: Symbol = 352;
const sym_keyword_smallmoney: Symbol = 335;
const sym_keyword_snapshot: Symbol = 250;
const sym_keyword_some: Symbol = 127;
const sym_keyword_sort: Symbol = 282;
const sym_keyword_spgist: Symbol = 162;
const sym_keyword_stable: Symbol = 215;
const sym_keyword_start: Symbol = 63;
const sym_keyword_statement: Symbol = 261;
const sym_keyword_statistics: Symbol = 274;
const sym_keyword_stats: Symbol = 273;
const sym_keyword_stdin: Symbol = 190;
const sym_keyword_storage: Symbol = 204;
const sym_keyword_stored: Symbol = 266;
const sym_keyword_strict: Symbol = 224;
const sym_keyword_string: Symbol = 342;
const sym_keyword_support: Symbol = 226;
const sym_keyword_table: Symbol = 50;
const sym_keyword_tables: Symbol = 51;
const sym_keyword_tablespace: Symbol = 56;
const sym_keyword_tblproperties: Symbol = 271;
const sym_keyword_temp: Symbol = 119;
const sym_keyword_temporary: Symbol = 120;
const sym_keyword_terminated: Symbol = 287;
const sym_keyword_text: Symbol = 341;
const sym_keyword_textfile: Symbol = 296;
const sym_keyword_then: Symbol = 75;
const sym_keyword_ties: Symbol = 152;
const sym_keyword_time: Symbol = 354;
const sym_keyword_timestamp: Symbol = 355;
const sym_keyword_timestamptz: Symbol = 356;
const sym_keyword_to: Symbol = 106;
const sym_keyword_transaction: Symbol = 134;
const sym_keyword_trigger: Symbol = 206;
const sym_keyword_true: Symbol = 302;
const sym_keyword_truncate: Symbol = 8;
const sym_keyword_type: Symbol = 104;
const sym_keyword_unbounded: Symbol = 146;
const sym_keyword_uncached: Symbol = 269;
const sym_keyword_uncommitted: Symbol = 243;
const sym_keyword_union: Symbol = 124;
const sym_keyword_unique: Symbol = 155;
const sym_keyword_unload: Symbol = 11;
const sym_keyword_unlogged: Symbol = 121;
const sym_keyword_unsafe: Symbol = 220;
const sym_keyword_unsigned: Symbol = 168;
const sym_keyword_until: Symbol = 115;
const sym_keyword_update: Symbol = 7;
const sym_keyword_use: Symbol = 86;
const sym_keyword_user: Symbol = 110;
const sym_keyword_using: Symbol = 85;
const sym_keyword_uuid: Symbol = 343;
const sym_keyword_vacuum: Symbol = 182;
const sym_keyword_valid: Symbol = 114;
const sym_keyword_value: Symbol = 15;
const sym_keyword_values: Symbol = 14;
const sym_keyword_varbinary: Symbol = 307;
const sym_keyword_variadic: Symbol = 233;
const sym_keyword_varying: Symbol = 336;
const sym_keyword_verbose: Symbol = 46;
const sym_keyword_version: Symbol = 230;
const sym_keyword_view: Symbol = 52;
const sym_keyword_virtual: Symbol = 267;
const sym_keyword_volatile: Symbol = 216;
const sym_keyword_wait: Symbol = 183;
const sym_keyword_when: Symbol = 74;
const sym_keyword_where: Symbol = 30;
const sym_keyword_window: Symbol = 141;
const sym_keyword_with: Symbol = 100;
const sym_keyword_without: Symbol = 101;
const sym_keyword_write: Symbol = 241;
const sym_keyword_xml: Symbol = 346;
const sym_keyword_zerofill: Symbol = 169;
const sym_keyword_zone: Symbol = 246;
const sym_marginalia: Symbol = 377;
const sym_op_other: Symbol = 389;
const sym_op_unary_other: Symbol = 397;
const sym_parameter: Symbol = 382;
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
                if eof { state = 39; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 103), (34, 3), (36, 26), (37, 78), (39, 5), (40, 43), (41, 44), (42, 51),
                    (43, 74), (44, 45), (45, 75), (46, 50), (47, 77), (58, 62), (59, 40), (60, 56),
                    (61, 48), (62, 57), (63, 52), (64, 106), (91, 41), (93, 42), (94, 79), (96, 104),
                    (124, 17), (35, 83), (126, 83), (69, 110), (101, 110), (78, 109), (110, 109), (85, 108),
                    (117, 108), (66, 111), (88, 111), (98, 111), (120, 111),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 33; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 89; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 || 192 <= lookahead && lookahead <= 383 { state = 112; lexer.advance(false); continue; }
                return result;
            }
            1 => {
                if let Some(next) = advance_map(&[
                    (33, 102), (34, 3), (36, 26), (39, 5), (40, 43), (41, 44), (42, 51), (43, 74),
                    (45, 75), (46, 30), (47, 10), (48, 90), (63, 52), (64, 106), (93, 42), (96, 104),
                    (124, 17), (35, 83), (126, 83), (69, 110), (101, 110), (78, 109), (110, 109), (85, 108),
                    (117, 108), (66, 111), (88, 111), (98, 111), (120, 111),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 1; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 91; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 || 192 <= lookahead && lookahead <= 383 { state = 112; lexer.advance(false); continue; }
                return result;
            }
            2 => {
                if let Some(next) = advance_map(&[
                    (34, 3), (39, 5), (40, 43), (43, 74), (45, 75), (46, 30), (47, 10), (48, 90),
                    (64, 105), (96, 104), (69, 110), (101, 110), (78, 109), (110, 109), (85, 108), (117, 108),
                    (66, 111), (88, 111), (98, 111), (120, 111),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 2; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 91; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 || 192 <= lookahead && lookahead <= 383 { state = 112; lexer.advance(false); continue; }
                return result;
            }
            3 => {
                if lookahead == 34 { state = 84; lexer.advance(false); continue; }
                if lookahead != 0 { state = 3; lexer.advance(false); continue; }
                return result;
            }
            4 => {
                if lookahead == 39 { state = 5; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                if lookahead == 39 { state = 85; lexer.advance(false); continue; }
                if lookahead != 0 { state = 5; lexer.advance(false); continue; }
                return result;
            }
            6 => {
                if lookahead == 39 { state = 87; lexer.advance(false); continue; }
                if lookahead == 92 { state = 8; lexer.advance(false); continue; }
                if lookahead != 0 { state = 6; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if lookahead == 39 { state = 101; lexer.advance(false); continue; }
                if lookahead != 0 { state = 7; lexer.advance(false); continue; }
                return result;
            }
            8 => {
                if lookahead == 39 { state = 88; lexer.advance(false); continue; }
                if lookahead == 92 { state = 8; lexer.advance(false); continue; }
                if lookahead != 0 { state = 6; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if lookahead == 39 { state = 86; lexer.advance(false); continue; }
                if lookahead != 0 { state = 9; lexer.advance(false); continue; }
                return result;
            }
            10 => {
                if lookahead == 42 { state = 12; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if lookahead == 42 { state = 11; lexer.advance(false); continue; }
                if lookahead == 47 { state = 47; lexer.advance(false); continue; }
                if lookahead != 0 { state = 12; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if lookahead == 42 { state = 11; lexer.advance(false); continue; }
                if lookahead != 0 { state = 12; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if lookahead == 43 { state = 74; lexer.advance(false); continue; }
                if lookahead == 45 { state = 75; lexer.advance(false); continue; }
                if lookahead == 47 { state = 10; lexer.advance(false); continue; }
                if lookahead == 48 { state = 95; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 13; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 96; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 || 192 <= lookahead && lookahead <= 383 { state = 112; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if lookahead == 45 { state = 46; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if lookahead == 45 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if lookahead == 47 { state = 83; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if lookahead == 47 { state = 83; lexer.advance(false); continue; }
                if lookahead == 124 { state = 16; lexer.advance(false); continue; }
                return result;
            }
            18 => {
                if lookahead == 58 { state = 54; lexer.advance(false); continue; }
                if lookahead == 61 { state = 49; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if lookahead == 61 { state = 81; lexer.advance(false); continue; }
                if lookahead == 126 { state = 65; lexer.advance(false); continue; }
                return result;
            }
            20 => {
                if lookahead == 62 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                if lookahead == 64 { state = 83; lexer.advance(false); continue; }
                return result;
            }
            22 => {
                if lookahead == 43 || lookahead == 45 { state = 28; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 97; lexer.advance(false); continue; }
                return result;
            }
            23 => {
                if lookahead == 43 || lookahead == 45 { state = 31; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 99; lexer.advance(false); continue; }
                return result;
            }
            24 => {
                if lookahead == 48 || lookahead == 49 { state = 92; lexer.advance(false); continue; }
                return result;
            }
            25 => {
                if 48 <= lookahead && lookahead <= 55 { state = 93; lexer.advance(false); continue; }
                return result;
            }
            26 => {
                if 48 <= lookahead && lookahead <= 57 { state = 53; lexer.advance(false); continue; }
                return result;
            }
            27 => {
                if 48 <= lookahead && lookahead <= 57 { state = 96; lexer.advance(false); continue; }
                return result;
            }
            28 => {
                if 48 <= lookahead && lookahead <= 57 { state = 97; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                if 48 <= lookahead && lookahead <= 57 { state = 91; lexer.advance(false); continue; }
                return result;
            }
            30 => {
                if 48 <= lookahead && lookahead <= 57 { state = 98; lexer.advance(false); continue; }
                return result;
            }
            31 => {
                if 48 <= lookahead && lookahead <= 57 { state = 99; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 94; lexer.advance(false); continue; }
                return result;
            }
            33 => {
                if eof { state = 39; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 103), (34, 3), (36, 26), (37, 78), (39, 5), (40, 43), (41, 44), (42, 51),
                    (43, 74), (44, 45), (45, 75), (46, 50), (47, 77), (58, 62), (59, 40), (60, 56),
                    (61, 48), (62, 57), (63, 52), (64, 106), (91, 41), (93, 42), (94, 79), (96, 104),
                    (124, 17), (35, 83), (126, 83), (69, 110), (101, 110), (78, 109), (110, 109), (85, 108),
                    (117, 108), (66, 111), (88, 111), (98, 111), (120, 111),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 33; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 89; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 || 192 <= lookahead && lookahead <= 383 { state = 112; lexer.advance(false); continue; }
                return result;
            }
            34 => {
                if eof { state = 39; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 19), (34, 3), (35, 69), (37, 78), (38, 67), (39, 5), (40, 43), (41, 44),
                    (42, 51), (43, 74), (44, 45), (45, 76), (46, 50), (47, 77), (48, 95), (58, 18),
                    (59, 40), (60, 55), (61, 48), (62, 58), (63, 66), (64, 107), (91, 41), (94, 80),
                    (96, 104), (124, 64), (126, 65), (78, 109), (110, 109), (85, 108), (117, 108),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 34; lexer.advance(true); continue; }
                if 49 <= lookahead && lookahead <= 57 { state = 96; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 || 192 <= lookahead && lookahead <= 383 { state = 112; lexer.advance(false); continue; }
                return result;
            }
            35 => {
                if eof { state = 39; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 19), (34, 3), (35, 69), (37, 78), (38, 67), (39, 5), (40, 43), (41, 44),
                    (42, 51), (43, 74), (44, 45), (45, 76), (46, 50), (47, 77), (58, 61), (59, 40),
                    (60, 55), (61, 48), (62, 58), (63, 66), (64, 107), (91, 41), (93, 42), (94, 80),
                    (96, 104), (124, 64), (126, 65), (78, 109), (110, 109), (85, 108), (117, 108),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 35; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 || 192 <= lookahead && lookahead <= 383 { state = 112; lexer.advance(false); continue; }
                return result;
            }
            36 => {
                if eof { state = 39; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 19), (34, 3), (35, 69), (37, 78), (38, 67), (39, 9), (40, 43), (41, 44),
                    (42, 51), (43, 74), (44, 45), (45, 76), (46, 50), (47, 77), (58, 18), (59, 40),
                    (60, 55), (61, 48), (62, 58), (63, 66), (64, 107), (91, 41), (93, 42), (94, 80),
                    (96, 104), (124, 64), (126, 65),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 36; lexer.advance(true); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 89; lexer.advance(false); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 || 192 <= lookahead && lookahead <= 383 { state = 112; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                if eof { state = 39; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (33, 19), (34, 3), (35, 69), (37, 78), (38, 67), (39, 9), (40, 43), (41, 44),
                    (42, 51), (43, 74), (44, 45), (45, 76), (46, 50), (47, 77), (58, 61), (59, 40),
                    (60, 55), (61, 48), (62, 58), (63, 66), (64, 107), (91, 41), (93, 42), (94, 80),
                    (96, 104), (124, 64), (126, 65),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 37; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 || 192 <= lookahead && lookahead <= 383 { state = 112; lexer.advance(false); continue; }
                return result;
            }
            38 => {
                if eof { state = 39; lexer.advance(false); continue; }
                if let Some(next) = advance_map(&[
                    (34, 3), (39, 5), (40, 43), (41, 44), (45, 14), (46, 50), (47, 10), (59, 40),
                    (64, 105), (96, 104), (69, 110), (101, 110), (78, 109), (110, 109), (85, 108), (117, 108),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 38; lexer.advance(true); continue; }
                if 65 <= lookahead && lookahead <= 90 || 95 <= lookahead && lookahead <= 122 || 192 <= lookahead && lookahead <= 383 { state = 112; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                result = true; lexer.set_result_symbol(ts_builtin_sym_end); lexer.mark_end();
                return result;
            }
            40 => {
                result = true; lexer.set_result_symbol(anon_sym_SEMI); lexer.mark_end();
                return result;
            }
            41 => {
                result = true; lexer.set_result_symbol(anon_sym_LBRACK); lexer.mark_end();
                return result;
            }
            42 => {
                result = true; lexer.set_result_symbol(anon_sym_RBRACK); lexer.mark_end();
                return result;
            }
            43 => {
                result = true; lexer.set_result_symbol(anon_sym_LPAREN); lexer.mark_end();
                return result;
            }
            44 => {
                result = true; lexer.set_result_symbol(anon_sym_RPAREN); lexer.mark_end();
                return result;
            }
            45 => {
                result = true; lexer.set_result_symbol(anon_sym_COMMA); lexer.mark_end();
                return result;
            }
            46 => {
                result = true; lexer.set_result_symbol(sym_comment); lexer.mark_end();
                if lookahead != 0 && lookahead != 10 { state = 46; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                result = true; lexer.set_result_symbol(sym_marginalia); lexer.mark_end();
                return result;
            }
            48 => {
                result = true; lexer.set_result_symbol(anon_sym_EQ); lexer.mark_end();
                return result;
            }
            49 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON_EQ); lexer.mark_end();
                return result;
            }
            50 => {
                result = true; lexer.set_result_symbol(anon_sym_DOT); lexer.mark_end();
                return result;
            }
            51 => {
                result = true; lexer.set_result_symbol(anon_sym_STAR); lexer.mark_end();
                return result;
            }
            52 => {
                result = true; lexer.set_result_symbol(sym_parameter); lexer.mark_end();
                if lookahead == 45 || lookahead == 124 { state = 83; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                result = true; lexer.set_result_symbol(sym_parameter); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 { state = 53; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON_COLON); lexer.mark_end();
                return result;
            }
            55 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 45 { state = 20; lexer.advance(false); continue; }
                if lookahead == 60 { state = 73; lexer.advance(false); continue; }
                if lookahead == 61 { state = 59; lexer.advance(false); continue; }
                if lookahead == 62 { state = 82; lexer.advance(false); continue; }
                if lookahead == 64 || lookahead == 94 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            56 => {
                result = true; lexer.set_result_symbol(anon_sym_LT); lexer.mark_end();
                if lookahead == 61 { state = 59; lexer.advance(false); continue; }
                if lookahead == 62 { state = 82; lexer.advance(false); continue; }
                return result;
            }
            57 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                if lookahead == 61 { state = 60; lexer.advance(false); continue; }
                return result;
            }
            58 => {
                result = true; lexer.set_result_symbol(anon_sym_GT); lexer.mark_end();
                if lookahead == 61 { state = 60; lexer.advance(false); continue; }
                if lookahead == 62 { state = 68; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_EQ); lexer.mark_end();
                return result;
            }
            60 => {
                result = true; lexer.set_result_symbol(anon_sym_GT_EQ); lexer.mark_end();
                return result;
            }
            61 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON); lexer.mark_end();
                if lookahead == 58 { state = 54; lexer.advance(false); continue; }
                return result;
            }
            62 => {
                result = true; lexer.set_result_symbol(anon_sym_COLON); lexer.mark_end();
                if lookahead == 58 { state = 54; lexer.advance(false); continue; }
                if lookahead == 61 { state = 49; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                result = true; lexer.set_result_symbol(sym_op_other); lexer.mark_end();
                return result;
            }
            64 => {
                result = true; lexer.set_result_symbol(sym_op_other); lexer.mark_end();
                if lookahead == 38 { state = 20; lexer.advance(false); continue; }
                if lookahead == 62 { state = 20; lexer.advance(false); continue; }
                if lookahead == 124 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            65 => {
                result = true; lexer.set_result_symbol(sym_op_other); lexer.mark_end();
                if lookahead == 42 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            66 => {
                result = true; lexer.set_result_symbol(sym_op_other); lexer.mark_end();
                if lookahead == 45 { state = 72; lexer.advance(false); continue; }
                if lookahead == 124 { state = 72; lexer.advance(false); continue; }
                if lookahead == 35 || lookahead == 38 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            67 => {
                result = true; lexer.set_result_symbol(sym_op_other); lexer.mark_end();
                if lookahead == 60 { state = 72; lexer.advance(false); continue; }
                if lookahead == 62 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            68 => {
                result = true; lexer.set_result_symbol(sym_op_other); lexer.mark_end();
                if lookahead == 61 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            69 => {
                result = true; lexer.set_result_symbol(sym_op_other); lexer.mark_end();
                if lookahead == 62 { state = 70; lexer.advance(false); continue; }
                if lookahead == 35 || lookahead == 45 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            70 => {
                result = true; lexer.set_result_symbol(sym_op_other); lexer.mark_end();
                if lookahead == 62 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            71 => {
                result = true; lexer.set_result_symbol(sym_op_other); lexer.mark_end();
                if lookahead == 64 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            72 => {
                result = true; lexer.set_result_symbol(sym_op_other); lexer.mark_end();
                if lookahead == 124 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            73 => {
                result = true; lexer.set_result_symbol(sym_op_other); lexer.mark_end();
                if lookahead == 61 || lookahead == 124 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            74 => {
                result = true; lexer.set_result_symbol(anon_sym_PLUS); lexer.mark_end();
                return result;
            }
            75 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 46; lexer.advance(false); continue; }
                return result;
            }
            76 => {
                result = true; lexer.set_result_symbol(anon_sym_DASH); lexer.mark_end();
                if lookahead == 45 { state = 46; lexer.advance(false); continue; }
                if lookahead == 62 { state = 70; lexer.advance(false); continue; }
                if lookahead == 124 { state = 15; lexer.advance(false); continue; }
                return result;
            }
            77 => {
                result = true; lexer.set_result_symbol(anon_sym_SLASH); lexer.mark_end();
                if lookahead == 42 { state = 12; lexer.advance(false); continue; }
                return result;
            }
            78 => {
                result = true; lexer.set_result_symbol(anon_sym_PERCENT); lexer.mark_end();
                return result;
            }
            79 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET); lexer.mark_end();
                return result;
            }
            80 => {
                result = true; lexer.set_result_symbol(anon_sym_CARET); lexer.mark_end();
                if lookahead == 62 || lookahead == 64 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            81 => {
                result = true; lexer.set_result_symbol(anon_sym_BANG_EQ); lexer.mark_end();
                return result;
            }
            82 => {
                result = true; lexer.set_result_symbol(anon_sym_LT_GT); lexer.mark_end();
                return result;
            }
            83 => {
                result = true; lexer.set_result_symbol(sym_op_unary_other); lexer.mark_end();
                return result;
            }
            84 => {
                result = true; lexer.set_result_symbol(sym__double_quote_string); lexer.mark_end();
                return result;
            }
            85 => {
                result = true; lexer.set_result_symbol(aux_sym__single_quote_string_token1); lexer.mark_end();
                if lookahead == 39 { state = 5; lexer.advance(false); continue; }
                return result;
            }
            86 => {
                result = true; lexer.set_result_symbol(aux_sym__single_quote_string_token2); lexer.mark_end();
                if lookahead == 39 { state = 9; lexer.advance(false); continue; }
                return result;
            }
            87 => {
                result = true; lexer.set_result_symbol(sym__postgres_escape_string); lexer.mark_end();
                return result;
            }
            88 => {
                result = true; lexer.set_result_symbol(sym__postgres_escape_string); lexer.mark_end();
                if lookahead == 39 { state = 87; lexer.advance(false); continue; }
                if lookahead == 92 { state = 8; lexer.advance(false); continue; }
                if lookahead != 0 { state = 6; lexer.advance(false); continue; }
                return result;
            }
            89 => {
                result = true; lexer.set_result_symbol(sym__natural_number); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 { state = 89; lexer.advance(false); continue; }
                return result;
            }
            90 => {
                result = true; lexer.set_result_symbol(aux_sym__integer_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (46, 100), (95, 29), (101, 22), (66, 24), (98, 24), (79, 25), (111, 25), (88, 32),
                    (120, 32),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 91; lexer.advance(false); continue; }
                return result;
            }
            91 => {
                result = true; lexer.set_result_symbol(aux_sym__integer_token1); lexer.mark_end();
                if lookahead == 46 { state = 100; lexer.advance(false); continue; }
                if lookahead == 95 { state = 29; lexer.advance(false); continue; }
                if lookahead == 101 { state = 22; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 91; lexer.advance(false); continue; }
                return result;
            }
            92 => {
                result = true; lexer.set_result_symbol(aux_sym__integer_token1); lexer.mark_end();
                if lookahead == 95 { state = 24; lexer.advance(false); continue; }
                if lookahead == 48 || lookahead == 49 { state = 92; lexer.advance(false); continue; }
                return result;
            }
            93 => {
                result = true; lexer.set_result_symbol(aux_sym__integer_token1); lexer.mark_end();
                if lookahead == 95 { state = 25; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 55 { state = 93; lexer.advance(false); continue; }
                return result;
            }
            94 => {
                result = true; lexer.set_result_symbol(aux_sym__integer_token1); lexer.mark_end();
                if lookahead == 95 { state = 32; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 70 || 97 <= lookahead && lookahead <= 102 { state = 94; lexer.advance(false); continue; }
                return result;
            }
            95 => {
                result = true; lexer.set_result_symbol(aux_sym__integer_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (95, 27), (101, 22), (66, 24), (98, 24), (79, 25), (111, 25), (88, 32), (120, 32),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 96; lexer.advance(false); continue; }
                return result;
            }
            96 => {
                result = true; lexer.set_result_symbol(aux_sym__integer_token1); lexer.mark_end();
                if lookahead == 95 { state = 27; lexer.advance(false); continue; }
                if lookahead == 101 { state = 22; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 96; lexer.advance(false); continue; }
                return result;
            }
            97 => {
                result = true; lexer.set_result_symbol(aux_sym__integer_token1); lexer.mark_end();
                if lookahead == 95 { state = 28; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 97; lexer.advance(false); continue; }
                return result;
            }
            98 => {
                result = true; lexer.set_result_symbol(aux_sym__decimal_number_token1); lexer.mark_end();
                if lookahead == 95 { state = 30; lexer.advance(false); continue; }
                if lookahead == 101 { state = 23; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 98; lexer.advance(false); continue; }
                return result;
            }
            99 => {
                result = true; lexer.set_result_symbol(aux_sym__decimal_number_token1); lexer.mark_end();
                if lookahead == 95 { state = 31; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 99; lexer.advance(false); continue; }
                return result;
            }
            100 => {
                result = true; lexer.set_result_symbol(aux_sym__decimal_number_token1); lexer.mark_end();
                if lookahead == 101 { state = 23; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 { state = 98; lexer.advance(false); continue; }
                return result;
            }
            101 => {
                result = true; lexer.set_result_symbol(aux_sym__bit_string_token1); lexer.mark_end();
                if lookahead == 39 { state = 7; lexer.advance(false); continue; }
                return result;
            }
            102 => {
                result = true; lexer.set_result_symbol(sym_bang); lexer.mark_end();
                if lookahead == 33 { state = 83; lexer.advance(false); continue; }
                return result;
            }
            103 => {
                result = true; lexer.set_result_symbol(sym_bang); lexer.mark_end();
                if lookahead == 33 { state = 83; lexer.advance(false); continue; }
                if lookahead == 61 { state = 81; lexer.advance(false); continue; }
                return result;
            }
            104 => {
                result = true; lexer.set_result_symbol(anon_sym_BQUOTE); lexer.mark_end();
                return result;
            }
            105 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                return result;
            }
            106 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                if lookahead == 45 { state = 21; lexer.advance(false); continue; }
                if lookahead == 64 { state = 83; lexer.advance(false); continue; }
                return result;
            }
            107 => {
                result = true; lexer.set_result_symbol(anon_sym_AT); lexer.mark_end();
                if lookahead == 64 { state = 71; lexer.advance(false); continue; }
                if lookahead == 62 || lookahead == 63 { state = 63; lexer.advance(false); continue; }
                return result;
            }
            108 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 38 { state = 4; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 || 192 <= lookahead && lookahead <= 383 { state = 112; lexer.advance(false); continue; }
                return result;
            }
            109 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 39 { state = 5; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 || 192 <= lookahead && lookahead <= 383 { state = 112; lexer.advance(false); continue; }
                return result;
            }
            110 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 39 { state = 6; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 || 192 <= lookahead && lookahead <= 383 { state = 112; lexer.advance(false); continue; }
                return result;
            }
            111 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if lookahead == 39 { state = 7; lexer.advance(false); continue; }
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 || 192 <= lookahead && lookahead <= 383 { state = 112; lexer.advance(false); continue; }
                return result;
            }
            112 => {
                result = true; lexer.set_result_symbol(sym__identifier); lexer.mark_end();
                if 48 <= lookahead && lookahead <= 57 || 65 <= lookahead && lookahead <= 90 || lookahead == 95 || 97 <= lookahead && lookahead <= 122 || 192 <= lookahead && lookahead <= 383 { state = 112; lexer.advance(false); continue; }
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
                    (65, 1), (97, 1), (66, 2), (98, 2), (67, 3), (99, 3), (68, 4), (100, 4),
                    (69, 5), (101, 5), (70, 6), (102, 6), (71, 7), (103, 7), (72, 8), (104, 8),
                    (73, 9), (105, 9), (74, 10), (106, 10), (75, 11), (107, 11), (76, 12), (108, 12),
                    (77, 13), (109, 13), (78, 14), (110, 14), (79, 15), (111, 15), (80, 16), (112, 16),
                    (81, 17), (113, 17), (82, 18), (114, 18), (83, 19), (115, 19), (84, 20), (116, 20),
                    (85, 21), (117, 21), (86, 22), (118, 22), (87, 23), (119, 23), (88, 24), (120, 24),
                    (90, 25), (122, 25),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 26; lexer.advance(true); continue; }
                return result;
            }
            1 => {
                if let Some(next) = advance_map(&[
                    (67, 27), (99, 27), (68, 28), (100, 28), (70, 29), (102, 29), (76, 30), (108, 30),
                    (78, 31), (110, 31), (82, 32), (114, 32), (83, 33), (115, 33), (84, 34), (116, 34),
                    (85, 35), (117, 35), (86, 36), (118, 36),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            2 => {
                if let Some(next) = advance_map(&[
                    (69, 37), (101, 37), (73, 38), (105, 38), (79, 39), (111, 39), (82, 40), (114, 40),
                    (84, 41), (116, 41), (89, 42), (121, 42),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            3 => {
                if let Some(next) = advance_map(&[
                    (65, 43), (97, 43), (72, 44), (104, 44), (79, 45), (111, 45), (82, 46), (114, 46),
                    (83, 47), (115, 47), (85, 48), (117, 48), (89, 49), (121, 49),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            4 => {
                if let Some(next) = advance_map(&[
                    (65, 50), (97, 50), (69, 51), (101, 51), (73, 52), (105, 52), (79, 53), (111, 53),
                    (82, 54), (114, 54), (85, 55), (117, 55),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            5 => {
                if let Some(next) = advance_map(&[
                    (65, 56), (97, 56), (76, 57), (108, 57), (78, 58), (110, 58), (83, 59), (115, 59),
                    (88, 60), (120, 60),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            6 => {
                if let Some(next) = advance_map(&[
                    (65, 61), (97, 61), (73, 62), (105, 62), (76, 63), (108, 63), (79, 64), (111, 64),
                    (82, 65), (114, 65), (85, 66), (117, 66),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            7 => {
                if lookahead == 69 || lookahead == 101 { state = 67; lexer.advance(false); continue; }
                if lookahead == 73 || lookahead == 105 { state = 68; lexer.advance(false); continue; }
                if lookahead == 82 || lookahead == 114 { state = 69; lexer.advance(false); continue; }
                return result;
            }
            8 => {
                if lookahead == 65 || lookahead == 97 { state = 70; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 71; lexer.advance(false); continue; }
                if lookahead == 73 || lookahead == 105 { state = 72; lexer.advance(false); continue; }
                return result;
            }
            9 => {
                if let Some(next) = advance_map(&[
                    (70, 73), (102, 73), (71, 74), (103, 74), (76, 75), (108, 75), (77, 76), (109, 76),
                    (78, 77), (110, 77), (83, 78), (115, 78),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            10 => {
                if lookahead == 79 || lookahead == 111 { state = 79; lexer.advance(false); continue; }
                if lookahead == 83 || lookahead == 115 { state = 80; lexer.advance(false); continue; }
                return result;
            }
            11 => {
                if lookahead == 69 || lookahead == 101 { state = 81; lexer.advance(false); continue; }
                return result;
            }
            12 => {
                if let Some(next) = advance_map(&[
                    (65, 82), (97, 82), (69, 83), (101, 83), (73, 84), (105, 84), (79, 85), (111, 85),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            13 => {
                if let Some(next) = advance_map(&[
                    (65, 86), (97, 86), (69, 87), (101, 87), (73, 88), (105, 88), (79, 89), (111, 89),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            14 => {
                if let Some(next) = advance_map(&[
                    (65, 90), (97, 90), (67, 91), (99, 91), (69, 92), (101, 92), (79, 93), (111, 93),
                    (85, 94), (117, 94), (86, 95), (118, 95),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            15 => {
                if let Some(next) = advance_map(&[
                    (66, 96), (98, 96), (70, 97), (102, 97), (73, 98), (105, 98), (76, 99), (108, 99),
                    (78, 100), (110, 100), (80, 101), (112, 101), (82, 102), (114, 102), (84, 103), (116, 103),
                    (85, 104), (117, 104), (86, 105), (118, 105), (87, 106), (119, 106),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            16 => {
                if lookahead == 65 || lookahead == 97 { state = 107; lexer.advance(false); continue; }
                if lookahead == 76 || lookahead == 108 { state = 108; lexer.advance(false); continue; }
                if lookahead == 82 || lookahead == 114 { state = 109; lexer.advance(false); continue; }
                return result;
            }
            17 => {
                if lookahead == 85 || lookahead == 117 { state = 110; lexer.advance(false); continue; }
                return result;
            }
            18 => {
                if let Some(next) = advance_map(&[
                    (65, 111), (97, 111), (67, 112), (99, 112), (69, 113), (101, 113), (73, 114), (105, 114),
                    (79, 115), (111, 115),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            19 => {
                if let Some(next) = advance_map(&[
                    (65, 116), (97, 116), (67, 117), (99, 117), (69, 118), (101, 118), (72, 119), (104, 119),
                    (73, 120), (105, 120), (77, 121), (109, 121), (78, 122), (110, 122), (79, 123), (111, 123),
                    (80, 124), (112, 124), (84, 125), (116, 125), (85, 126), (117, 126),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            20 => {
                if let Some(next) = advance_map(&[
                    (65, 127), (97, 127), (66, 128), (98, 128), (69, 129), (101, 129), (72, 130), (104, 130),
                    (73, 131), (105, 131), (79, 132), (111, 132), (82, 133), (114, 133), (89, 134), (121, 134),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            21 => {
                if let Some(next) = advance_map(&[
                    (78, 135), (110, 135), (80, 136), (112, 136), (83, 137), (115, 137), (85, 138), (117, 138),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            22 => {
                if let Some(next) = advance_map(&[
                    (65, 139), (97, 139), (69, 140), (101, 140), (73, 141), (105, 141), (79, 142), (111, 142),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            23 => {
                if let Some(next) = advance_map(&[
                    (65, 143), (97, 143), (72, 144), (104, 144), (73, 145), (105, 145), (82, 146), (114, 146),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            24 => {
                if lookahead == 77 || lookahead == 109 { state = 147; lexer.advance(false); continue; }
                return result;
            }
            25 => {
                if lookahead == 69 || lookahead == 101 { state = 148; lexer.advance(false); continue; }
                if lookahead == 79 || lookahead == 111 { state = 149; lexer.advance(false); continue; }
                return result;
            }
            26 => {
                if let Some(next) = advance_map(&[
                    (65, 1), (97, 1), (66, 2), (98, 2), (67, 3), (99, 3), (68, 4), (100, 4),
                    (69, 5), (101, 5), (70, 6), (102, 6), (71, 7), (103, 7), (72, 8), (104, 8),
                    (73, 9), (105, 9), (74, 10), (106, 10), (75, 11), (107, 11), (76, 12), (108, 12),
                    (77, 13), (109, 13), (78, 14), (110, 14), (79, 15), (111, 15), (80, 16), (112, 16),
                    (81, 17), (113, 17), (82, 18), (114, 18), (83, 19), (115, 19), (84, 20), (116, 20),
                    (85, 21), (117, 21), (86, 22), (118, 22), (87, 23), (119, 23), (88, 24), (120, 24),
                    (90, 25), (122, 25),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                if 9 <= lookahead && lookahead <= 13 || lookahead == 32 { state = 26; lexer.advance(true); continue; }
                return result;
            }
            27 => {
                if lookahead == 84 || lookahead == 116 { state = 150; lexer.advance(false); continue; }
                return result;
            }
            28 => {
                if lookahead == 68 || lookahead == 100 { state = 151; lexer.advance(false); continue; }
                if lookahead == 77 || lookahead == 109 { state = 152; lexer.advance(false); continue; }
                return result;
            }
            29 => {
                if lookahead == 84 || lookahead == 116 { state = 153; lexer.advance(false); continue; }
                return result;
            }
            30 => {
                if lookahead == 76 || lookahead == 108 { state = 154; lexer.advance(false); continue; }
                if lookahead == 84 || lookahead == 116 { state = 155; lexer.advance(false); continue; }
                if lookahead == 87 || lookahead == 119 { state = 156; lexer.advance(false); continue; }
                return result;
            }
            31 => {
                if lookahead == 65 || lookahead == 97 { state = 157; lexer.advance(false); continue; }
                if lookahead == 68 || lookahead == 100 { state = 158; lexer.advance(false); continue; }
                if lookahead == 89 || lookahead == 121 { state = 159; lexer.advance(false); continue; }
                return result;
            }
            32 => {
                if lookahead == 82 || lookahead == 114 { state = 160; lexer.advance(false); continue; }
                return result;
            }
            33 => {
                result = true; lexer.set_result_symbol(sym_keyword_as); lexer.mark_end();
                if lookahead == 67 || lookahead == 99 { state = 161; lexer.advance(false); continue; }
                return result;
            }
            34 => {
                if lookahead == 79 || lookahead == 111 { state = 162; lexer.advance(false); continue; }
                if lookahead == 84 || lookahead == 116 { state = 163; lexer.advance(false); continue; }
                return result;
            }
            35 => {
                if lookahead == 84 || lookahead == 116 { state = 164; lexer.advance(false); continue; }
                return result;
            }
            36 => {
                if lookahead == 82 || lookahead == 114 { state = 165; lexer.advance(false); continue; }
                return result;
            }
            37 => {
                if lookahead == 70 || lookahead == 102 { state = 166; lexer.advance(false); continue; }
                if lookahead == 71 || lookahead == 103 { state = 167; lexer.advance(false); continue; }
                if lookahead == 84 || lookahead == 116 { state = 168; lexer.advance(false); continue; }
                return result;
            }
            38 => {
                if lookahead == 71 || lookahead == 103 { state = 169; lexer.advance(false); continue; }
                if lookahead == 78 || lookahead == 110 { state = 170; lexer.advance(false); continue; }
                if lookahead == 84 || lookahead == 116 { state = 171; lexer.advance(false); continue; }
                return result;
            }
            39 => {
                if lookahead == 79 || lookahead == 111 { state = 172; lexer.advance(false); continue; }
                if lookahead == 88 || lookahead == 120 { state = 173; lexer.advance(false); continue; }
                return result;
            }
            40 => {
                if lookahead == 73 || lookahead == 105 { state = 174; lexer.advance(false); continue; }
                return result;
            }
            41 => {
                if lookahead == 82 || lookahead == 114 { state = 175; lexer.advance(false); continue; }
                return result;
            }
            42 => {
                result = true; lexer.set_result_symbol(sym_keyword_by); lexer.mark_end();
                if lookahead == 84 || lookahead == 116 { state = 176; lexer.advance(false); continue; }
                return result;
            }
            43 => {
                if lookahead == 67 || lookahead == 99 { state = 177; lexer.advance(false); continue; }
                if lookahead == 76 || lookahead == 108 { state = 178; lexer.advance(false); continue; }
                if lookahead == 83 || lookahead == 115 { state = 179; lexer.advance(false); continue; }
                return result;
            }
            44 => {
                if lookahead == 65 || lookahead == 97 { state = 180; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 181; lexer.advance(false); continue; }
                return result;
            }
            45 => {
                if let Some(next) = advance_map(&[
                    (76, 182), (108, 182), (77, 183), (109, 183), (78, 184), (110, 184), (80, 185), (112, 185),
                    (83, 186), (115, 186),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            46 => {
                if lookahead == 69 || lookahead == 101 { state = 187; lexer.advance(false); continue; }
                if lookahead == 79 || lookahead == 111 { state = 188; lexer.advance(false); continue; }
                return result;
            }
            47 => {
                if lookahead == 86 || lookahead == 118 { state = 189; lexer.advance(false); continue; }
                return result;
            }
            48 => {
                if lookahead == 82 || lookahead == 114 { state = 190; lexer.advance(false); continue; }
                return result;
            }
            49 => {
                if lookahead == 67 || lookahead == 99 { state = 191; lexer.advance(false); continue; }
                return result;
            }
            50 => {
                if lookahead == 84 || lookahead == 116 { state = 192; lexer.advance(false); continue; }
                return result;
            }
            51 => {
                if let Some(next) = advance_map(&[
                    (67, 193), (99, 193), (70, 194), (102, 194), (76, 195), (108, 195), (83, 196), (115, 196),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            52 => {
                if lookahead == 83 || lookahead == 115 { state = 197; lexer.advance(false); continue; }
                return result;
            }
            53 => {
                result = true; lexer.set_result_symbol(sym_keyword_do); lexer.mark_end();
                if lookahead == 85 || lookahead == 117 { state = 198; lexer.advance(false); continue; }
                return result;
            }
            54 => {
                if lookahead == 79 || lookahead == 111 { state = 199; lexer.advance(false); continue; }
                return result;
            }
            55 => {
                if lookahead == 80 || lookahead == 112 { state = 200; lexer.advance(false); continue; }
                return result;
            }
            56 => {
                if lookahead == 67 || lookahead == 99 { state = 201; lexer.advance(false); continue; }
                return result;
            }
            57 => {
                if lookahead == 83 || lookahead == 115 { state = 202; lexer.advance(false); continue; }
                return result;
            }
            58 => {
                if let Some(next) = advance_map(&[
                    (67, 203), (99, 203), (68, 204), (100, 204), (71, 205), (103, 205), (85, 206), (117, 206),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            59 => {
                if lookahead == 67 || lookahead == 99 { state = 207; lexer.advance(false); continue; }
                return result;
            }
            60 => {
                if let Some(next) = advance_map(&[
                    (67, 208), (99, 208), (69, 209), (101, 209), (73, 210), (105, 210), (80, 211), (112, 211),
                    (84, 212), (116, 212),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            61 => {
                if lookahead == 76 || lookahead == 108 { state = 213; lexer.advance(false); continue; }
                return result;
            }
            62 => {
                if lookahead == 69 || lookahead == 101 { state = 214; lexer.advance(false); continue; }
                if lookahead == 76 || lookahead == 108 { state = 215; lexer.advance(false); continue; }
                if lookahead == 82 || lookahead == 114 { state = 216; lexer.advance(false); continue; }
                return result;
            }
            63 => {
                if lookahead == 79 || lookahead == 111 { state = 217; lexer.advance(false); continue; }
                return result;
            }
            64 => {
                if lookahead == 76 || lookahead == 108 { state = 218; lexer.advance(false); continue; }
                if lookahead == 82 || lookahead == 114 { state = 219; lexer.advance(false); continue; }
                return result;
            }
            65 => {
                if lookahead == 69 || lookahead == 101 { state = 220; lexer.advance(false); continue; }
                if lookahead == 79 || lookahead == 111 { state = 221; lexer.advance(false); continue; }
                return result;
            }
            66 => {
                if lookahead == 76 || lookahead == 108 { state = 222; lexer.advance(false); continue; }
                if lookahead == 78 || lookahead == 110 { state = 223; lexer.advance(false); continue; }
                return result;
            }
            67 => {
                if lookahead == 78 || lookahead == 110 { state = 224; lexer.advance(false); continue; }
                if lookahead == 79 || lookahead == 111 { state = 225; lexer.advance(false); continue; }
                return result;
            }
            68 => {
                if lookahead == 78 || lookahead == 110 { state = 226; lexer.advance(false); continue; }
                if lookahead == 83 || lookahead == 115 { state = 227; lexer.advance(false); continue; }
                return result;
            }
            69 => {
                if lookahead == 79 || lookahead == 111 { state = 228; lexer.advance(false); continue; }
                return result;
            }
            70 => {
                if lookahead == 83 || lookahead == 115 { state = 229; lexer.advance(false); continue; }
                if lookahead == 86 || lookahead == 118 { state = 230; lexer.advance(false); continue; }
                return result;
            }
            71 => {
                if lookahead == 65 || lookahead == 97 { state = 231; lexer.advance(false); continue; }
                return result;
            }
            72 => {
                if lookahead == 71 || lookahead == 103 { state = 232; lexer.advance(false); continue; }
                return result;
            }
            73 => {
                result = true; lexer.set_result_symbol(sym_keyword_if); lexer.mark_end();
                return result;
            }
            74 => {
                if lookahead == 78 || lookahead == 110 { state = 233; lexer.advance(false); continue; }
                return result;
            }
            75 => {
                if lookahead == 73 || lookahead == 105 { state = 234; lexer.advance(false); continue; }
                return result;
            }
            76 => {
                if lookahead == 65 || lookahead == 97 { state = 235; lexer.advance(false); continue; }
                if lookahead == 77 || lookahead == 109 { state = 236; lexer.advance(false); continue; }
                return result;
            }
            77 => {
                result = true; lexer.set_result_symbol(sym_keyword_in); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (67, 237), (99, 237), (68, 238), (100, 238), (69, 239), (101, 239), (73, 240), (105, 240),
                    (78, 241), (110, 241), (79, 242), (111, 242), (80, 243), (112, 243), (83, 244), (115, 244),
                    (84, 245), (116, 245), (86, 246), (118, 246),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            78 => {
                result = true; lexer.set_result_symbol(sym_keyword_is); lexer.mark_end();
                if lookahead == 79 || lookahead == 111 { state = 247; lexer.advance(false); continue; }
                return result;
            }
            79 => {
                if lookahead == 73 || lookahead == 105 { state = 248; lexer.advance(false); continue; }
                return result;
            }
            80 => {
                if lookahead == 79 || lookahead == 111 { state = 249; lexer.advance(false); continue; }
                return result;
            }
            81 => {
                if lookahead == 89 || lookahead == 121 { state = 250; lexer.advance(false); continue; }
                return result;
            }
            82 => {
                if lookahead == 78 || lookahead == 110 { state = 251; lexer.advance(false); continue; }
                if lookahead == 83 || lookahead == 115 { state = 252; lexer.advance(false); continue; }
                if lookahead == 84 || lookahead == 116 { state = 253; lexer.advance(false); continue; }
                return result;
            }
            83 => {
                if lookahead == 65 || lookahead == 97 { state = 254; lexer.advance(false); continue; }
                if lookahead == 70 || lookahead == 102 { state = 255; lexer.advance(false); continue; }
                if lookahead == 86 || lookahead == 118 { state = 256; lexer.advance(false); continue; }
                return result;
            }
            84 => {
                if lookahead == 75 || lookahead == 107 { state = 257; lexer.advance(false); continue; }
                if lookahead == 77 || lookahead == 109 { state = 258; lexer.advance(false); continue; }
                if lookahead == 78 || lookahead == 110 { state = 259; lexer.advance(false); continue; }
                return result;
            }
            85 => {
                if lookahead == 67 || lookahead == 99 { state = 260; lexer.advance(false); continue; }
                if lookahead == 71 || lookahead == 103 { state = 261; lexer.advance(false); continue; }
                if lookahead == 87 || lookahead == 119 { state = 262; lexer.advance(false); continue; }
                return result;
            }
            86 => {
                if lookahead == 73 || lookahead == 105 { state = 263; lexer.advance(false); continue; }
                if lookahead == 84 || lookahead == 116 { state = 264; lexer.advance(false); continue; }
                if lookahead == 88 || lookahead == 120 { state = 265; lexer.advance(false); continue; }
                return result;
            }
            87 => {
                if lookahead == 68 || lookahead == 100 { state = 266; lexer.advance(false); continue; }
                if lookahead == 82 || lookahead == 114 { state = 267; lexer.advance(false); continue; }
                if lookahead == 84 || lookahead == 116 { state = 268; lexer.advance(false); continue; }
                return result;
            }
            88 => {
                if lookahead == 78 || lookahead == 110 { state = 269; lexer.advance(false); continue; }
                return result;
            }
            89 => {
                if lookahead == 68 || lookahead == 100 { state = 270; lexer.advance(false); continue; }
                if lookahead == 78 || lookahead == 110 { state = 271; lexer.advance(false); continue; }
                return result;
            }
            90 => {
                if lookahead == 77 || lookahead == 109 { state = 272; lexer.advance(false); continue; }
                if lookahead == 84 || lookahead == 116 { state = 273; lexer.advance(false); continue; }
                return result;
            }
            91 => {
                if lookahead == 72 || lookahead == 104 { state = 274; lexer.advance(false); continue; }
                return result;
            }
            92 => {
                if lookahead == 87 || lookahead == 119 { state = 275; lexer.advance(false); continue; }
                return result;
            }
            93 => {
                result = true; lexer.set_result_symbol(sym_keyword_no); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (78, 276), (110, 276), (83, 277), (115, 277), (84, 278), (116, 278), (87, 279), (119, 279),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            94 => {
                if lookahead == 76 || lookahead == 108 { state = 280; lexer.advance(false); continue; }
                if lookahead == 77 || lookahead == 109 { state = 281; lexer.advance(false); continue; }
                return result;
            }
            95 => {
                if lookahead == 65 || lookahead == 97 { state = 282; lexer.advance(false); continue; }
                return result;
            }
            96 => {
                if lookahead == 74 || lookahead == 106 { state = 283; lexer.advance(false); continue; }
                return result;
            }
            97 => {
                result = true; lexer.set_result_symbol(sym_keyword_of); lexer.mark_end();
                if lookahead == 70 || lookahead == 102 { state = 284; lexer.advance(false); continue; }
                return result;
            }
            98 => {
                if lookahead == 68 || lookahead == 100 { state = 285; lexer.advance(false); continue; }
                return result;
            }
            99 => {
                if lookahead == 68 || lookahead == 100 { state = 286; lexer.advance(false); continue; }
                return result;
            }
            100 => {
                result = true; lexer.set_result_symbol(sym_keyword_on); lexer.mark_end();
                if lookahead == 76 || lookahead == 108 { state = 287; lexer.advance(false); continue; }
                return result;
            }
            101 => {
                if lookahead == 84 || lookahead == 116 { state = 288; lexer.advance(false); continue; }
                return result;
            }
            102 => {
                result = true; lexer.set_result_symbol(sym_keyword_or); lexer.mark_end();
                if lookahead == 67 || lookahead == 99 { state = 289; lexer.advance(false); continue; }
                if lookahead == 68 || lookahead == 100 { state = 290; lexer.advance(false); continue; }
                return result;
            }
            103 => {
                if lookahead == 72 || lookahead == 104 { state = 291; lexer.advance(false); continue; }
                return result;
            }
            104 => {
                if lookahead == 84 || lookahead == 116 { state = 292; lexer.advance(false); continue; }
                return result;
            }
            105 => {
                if lookahead == 69 || lookahead == 101 { state = 293; lexer.advance(false); continue; }
                return result;
            }
            106 => {
                if lookahead == 78 || lookahead == 110 { state = 294; lexer.advance(false); continue; }
                return result;
            }
            107 => {
                if lookahead == 82 || lookahead == 114 { state = 295; lexer.advance(false); continue; }
                if lookahead == 83 || lookahead == 115 { state = 296; lexer.advance(false); continue; }
                return result;
            }
            108 => {
                if lookahead == 65 || lookahead == 97 { state = 297; lexer.advance(false); continue; }
                return result;
            }
            109 => {
                if lookahead == 69 || lookahead == 101 { state = 298; lexer.advance(false); continue; }
                if lookahead == 73 || lookahead == 105 { state = 299; lexer.advance(false); continue; }
                if lookahead == 79 || lookahead == 111 { state = 300; lexer.advance(false); continue; }
                return result;
            }
            110 => {
                if lookahead == 79 || lookahead == 111 { state = 301; lexer.advance(false); continue; }
                return result;
            }
            111 => {
                if lookahead == 78 || lookahead == 110 { state = 302; lexer.advance(false); continue; }
                return result;
            }
            112 => {
                if lookahead == 70 || lookahead == 102 { state = 303; lexer.advance(false); continue; }
                return result;
            }
            113 => {
                if let Some(next) = advance_map(&[
                    (65, 304), (97, 304), (67, 305), (99, 305), (70, 306), (102, 306), (71, 307), (103, 307),
                    (78, 308), (110, 308), (80, 309), (112, 309), (83, 310), (115, 310), (84, 311), (116, 311),
                    (87, 312), (119, 312),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            114 => {
                if lookahead == 71 || lookahead == 103 { state = 313; lexer.advance(false); continue; }
                return result;
            }
            115 => {
                if lookahead == 76 || lookahead == 108 { state = 314; lexer.advance(false); continue; }
                if lookahead == 87 || lookahead == 119 { state = 315; lexer.advance(false); continue; }
                return result;
            }
            116 => {
                if lookahead == 70 || lookahead == 102 { state = 316; lexer.advance(false); continue; }
                return result;
            }
            117 => {
                if lookahead == 72 || lookahead == 104 { state = 317; lexer.advance(false); continue; }
                return result;
            }
            118 => {
                if let Some(next) = advance_map(&[
                    (67, 318), (99, 318), (76, 319), (108, 319), (80, 320), (112, 320), (81, 321), (113, 321),
                    (82, 322), (114, 322), (83, 323), (115, 323), (84, 324), (116, 324),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            119 => {
                if lookahead == 79 || lookahead == 111 { state = 325; lexer.advance(false); continue; }
                return result;
            }
            120 => {
                if lookahead == 77 || lookahead == 109 { state = 326; lexer.advance(false); continue; }
                return result;
            }
            121 => {
                if lookahead == 65 || lookahead == 97 { state = 327; lexer.advance(false); continue; }
                return result;
            }
            122 => {
                if lookahead == 65 || lookahead == 97 { state = 328; lexer.advance(false); continue; }
                return result;
            }
            123 => {
                if lookahead == 77 || lookahead == 109 { state = 329; lexer.advance(false); continue; }
                if lookahead == 82 || lookahead == 114 { state = 330; lexer.advance(false); continue; }
                return result;
            }
            124 => {
                if lookahead == 71 || lookahead == 103 { state = 331; lexer.advance(false); continue; }
                return result;
            }
            125 => {
                if let Some(next) = advance_map(&[
                    (65, 332), (97, 332), (68, 333), (100, 333), (79, 334), (111, 334), (82, 335), (114, 335),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            126 => {
                if lookahead == 80 || lookahead == 112 { state = 336; lexer.advance(false); continue; }
                return result;
            }
            127 => {
                if lookahead == 66 || lookahead == 98 { state = 337; lexer.advance(false); continue; }
                return result;
            }
            128 => {
                if lookahead == 76 || lookahead == 108 { state = 338; lexer.advance(false); continue; }
                return result;
            }
            129 => {
                if lookahead == 77 || lookahead == 109 { state = 339; lexer.advance(false); continue; }
                if lookahead == 82 || lookahead == 114 { state = 340; lexer.advance(false); continue; }
                if lookahead == 88 || lookahead == 120 { state = 341; lexer.advance(false); continue; }
                return result;
            }
            130 => {
                if lookahead == 69 || lookahead == 101 { state = 342; lexer.advance(false); continue; }
                return result;
            }
            131 => {
                if lookahead == 69 || lookahead == 101 { state = 343; lexer.advance(false); continue; }
                if lookahead == 77 || lookahead == 109 { state = 344; lexer.advance(false); continue; }
                if lookahead == 78 || lookahead == 110 { state = 345; lexer.advance(false); continue; }
                return result;
            }
            132 => {
                result = true; lexer.set_result_symbol(sym_keyword_to); lexer.mark_end();
                return result;
            }
            133 => {
                if lookahead == 65 || lookahead == 97 { state = 346; lexer.advance(false); continue; }
                if lookahead == 73 || lookahead == 105 { state = 347; lexer.advance(false); continue; }
                if lookahead == 85 || lookahead == 117 { state = 348; lexer.advance(false); continue; }
                return result;
            }
            134 => {
                if lookahead == 80 || lookahead == 112 { state = 349; lexer.advance(false); continue; }
                return result;
            }
            135 => {
                if let Some(next) = advance_map(&[
                    (66, 350), (98, 350), (67, 351), (99, 351), (73, 352), (105, 352), (76, 353), (108, 353),
                    (83, 354), (115, 354), (84, 355), (116, 355),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            136 => {
                if lookahead == 68 || lookahead == 100 { state = 356; lexer.advance(false); continue; }
                return result;
            }
            137 => {
                if lookahead == 69 || lookahead == 101 { state = 357; lexer.advance(false); continue; }
                if lookahead == 73 || lookahead == 105 { state = 358; lexer.advance(false); continue; }
                return result;
            }
            138 => {
                if lookahead == 73 || lookahead == 105 { state = 359; lexer.advance(false); continue; }
                return result;
            }
            139 => {
                if lookahead == 67 || lookahead == 99 { state = 360; lexer.advance(false); continue; }
                if lookahead == 76 || lookahead == 108 { state = 361; lexer.advance(false); continue; }
                if lookahead == 82 || lookahead == 114 { state = 362; lexer.advance(false); continue; }
                return result;
            }
            140 => {
                if lookahead == 82 || lookahead == 114 { state = 363; lexer.advance(false); continue; }
                return result;
            }
            141 => {
                if lookahead == 69 || lookahead == 101 { state = 364; lexer.advance(false); continue; }
                if lookahead == 82 || lookahead == 114 { state = 365; lexer.advance(false); continue; }
                return result;
            }
            142 => {
                if lookahead == 76 || lookahead == 108 { state = 366; lexer.advance(false); continue; }
                return result;
            }
            143 => {
                if lookahead == 73 || lookahead == 105 { state = 367; lexer.advance(false); continue; }
                return result;
            }
            144 => {
                if lookahead == 69 || lookahead == 101 { state = 368; lexer.advance(false); continue; }
                return result;
            }
            145 => {
                if lookahead == 78 || lookahead == 110 { state = 369; lexer.advance(false); continue; }
                if lookahead == 84 || lookahead == 116 { state = 370; lexer.advance(false); continue; }
                return result;
            }
            146 => {
                if lookahead == 73 || lookahead == 105 { state = 371; lexer.advance(false); continue; }
                return result;
            }
            147 => {
                if lookahead == 76 || lookahead == 108 { state = 372; lexer.advance(false); continue; }
                return result;
            }
            148 => {
                if lookahead == 82 || lookahead == 114 { state = 373; lexer.advance(false); continue; }
                return result;
            }
            149 => {
                if lookahead == 78 || lookahead == 110 { state = 374; lexer.advance(false); continue; }
                return result;
            }
            150 => {
                if lookahead == 73 || lookahead == 105 { state = 375; lexer.advance(false); continue; }
                return result;
            }
            151 => {
                result = true; lexer.set_result_symbol(sym_keyword_add); lexer.mark_end();
                return result;
            }
            152 => {
                if lookahead == 73 || lookahead == 105 { state = 376; lexer.advance(false); continue; }
                return result;
            }
            153 => {
                if lookahead == 69 || lookahead == 101 { state = 377; lexer.advance(false); continue; }
                return result;
            }
            154 => {
                result = true; lexer.set_result_symbol(sym_keyword_all); lexer.mark_end();
                return result;
            }
            155 => {
                if lookahead == 69 || lookahead == 101 { state = 378; lexer.advance(false); continue; }
                return result;
            }
            156 => {
                if lookahead == 65 || lookahead == 97 { state = 379; lexer.advance(false); continue; }
                return result;
            }
            157 => {
                if lookahead == 76 || lookahead == 108 { state = 380; lexer.advance(false); continue; }
                return result;
            }
            158 => {
                result = true; lexer.set_result_symbol(sym_keyword_and); lexer.mark_end();
                return result;
            }
            159 => {
                result = true; lexer.set_result_symbol(sym_keyword_any); lexer.mark_end();
                return result;
            }
            160 => {
                if lookahead == 65 || lookahead == 97 { state = 381; lexer.advance(false); continue; }
                return result;
            }
            161 => {
                result = true; lexer.set_result_symbol(sym_keyword_asc); lexer.mark_end();
                return result;
            }
            162 => {
                if lookahead == 77 || lookahead == 109 { state = 382; lexer.advance(false); continue; }
                return result;
            }
            163 => {
                if lookahead == 82 || lookahead == 114 { state = 383; lexer.advance(false); continue; }
                return result;
            }
            164 => {
                if lookahead == 72 || lookahead == 104 { state = 384; lexer.advance(false); continue; }
                if lookahead == 79 || lookahead == 111 { state = 385; lexer.advance(false); continue; }
                return result;
            }
            165 => {
                if lookahead == 79 || lookahead == 111 { state = 386; lexer.advance(false); continue; }
                return result;
            }
            166 => {
                if lookahead == 79 || lookahead == 111 { state = 387; lexer.advance(false); continue; }
                return result;
            }
            167 => {
                if lookahead == 73 || lookahead == 105 { state = 388; lexer.advance(false); continue; }
                return result;
            }
            168 => {
                if lookahead == 87 || lookahead == 119 { state = 389; lexer.advance(false); continue; }
                return result;
            }
            169 => {
                if lookahead == 73 || lookahead == 105 { state = 390; lexer.advance(false); continue; }
                if lookahead == 83 || lookahead == 115 { state = 391; lexer.advance(false); continue; }
                return result;
            }
            170 => {
                if lookahead == 95 { state = 392; lexer.advance(false); continue; }
                if lookahead == 65 || lookahead == 97 { state = 393; lexer.advance(false); continue; }
                return result;
            }
            171 => {
                result = true; lexer.set_result_symbol(sym_keyword_bit); lexer.mark_end();
                return result;
            }
            172 => {
                if lookahead == 76 || lookahead == 108 { state = 394; lexer.advance(false); continue; }
                return result;
            }
            173 => {
                if lookahead == 50 { state = 395; lexer.advance(false); continue; }
                if lookahead == 51 { state = 396; lexer.advance(false); continue; }
                return result;
            }
            174 => {
                if lookahead == 78 || lookahead == 110 { state = 397; lexer.advance(false); continue; }
                return result;
            }
            175 => {
                if lookahead == 69 || lookahead == 101 { state = 398; lexer.advance(false); continue; }
                return result;
            }
            176 => {
                if lookahead == 69 || lookahead == 101 { state = 399; lexer.advance(false); continue; }
                return result;
            }
            177 => {
                if lookahead == 72 || lookahead == 104 { state = 400; lexer.advance(false); continue; }
                return result;
            }
            178 => {
                if lookahead == 76 || lookahead == 108 { state = 401; lexer.advance(false); continue; }
                return result;
            }
            179 => {
                if lookahead == 67 || lookahead == 99 { state = 402; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 403; lexer.advance(false); continue; }
                if lookahead == 84 || lookahead == 116 { state = 404; lexer.advance(false); continue; }
                return result;
            }
            180 => {
                if lookahead == 78 || lookahead == 110 { state = 405; lexer.advance(false); continue; }
                if lookahead == 82 || lookahead == 114 { state = 406; lexer.advance(false); continue; }
                return result;
            }
            181 => {
                if lookahead == 67 || lookahead == 99 { state = 407; lexer.advance(false); continue; }
                return result;
            }
            182 => {
                if lookahead == 76 || lookahead == 108 { state = 408; lexer.advance(false); continue; }
                if lookahead == 85 || lookahead == 117 { state = 409; lexer.advance(false); continue; }
                return result;
            }
            183 => {
                if lookahead == 77 || lookahead == 109 { state = 410; lexer.advance(false); continue; }
                if lookahead == 80 || lookahead == 112 { state = 411; lexer.advance(false); continue; }
                return result;
            }
            184 => {
                if let Some(next) = advance_map(&[
                    (67, 412), (99, 412), (70, 413), (102, 413), (78, 414), (110, 414), (83, 415), (115, 415),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            185 => {
                if lookahead == 89 || lookahead == 121 { state = 416; lexer.advance(false); continue; }
                return result;
            }
            186 => {
                if lookahead == 84 || lookahead == 116 { state = 417; lexer.advance(false); continue; }
                return result;
            }
            187 => {
                if lookahead == 65 || lookahead == 97 { state = 418; lexer.advance(false); continue; }
                return result;
            }
            188 => {
                if lookahead == 83 || lookahead == 115 { state = 419; lexer.advance(false); continue; }
                return result;
            }
            189 => {
                result = true; lexer.set_result_symbol(sym_keyword_csv); lexer.mark_end();
                return result;
            }
            190 => {
                if lookahead == 82 || lookahead == 114 { state = 420; lexer.advance(false); continue; }
                return result;
            }
            191 => {
                if lookahead == 76 || lookahead == 108 { state = 421; lexer.advance(false); continue; }
                return result;
            }
            192 => {
                if lookahead == 65 || lookahead == 97 { state = 422; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 423; lexer.advance(false); continue; }
                return result;
            }
            193 => {
                if lookahead == 73 || lookahead == 105 { state = 424; lexer.advance(false); continue; }
                if lookahead == 76 || lookahead == 108 { state = 425; lexer.advance(false); continue; }
                return result;
            }
            194 => {
                if lookahead == 65 || lookahead == 97 { state = 426; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 427; lexer.advance(false); continue; }
                if lookahead == 73 || lookahead == 105 { state = 428; lexer.advance(false); continue; }
                return result;
            }
            195 => {
                if lookahead == 65 || lookahead == 97 { state = 429; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 430; lexer.advance(false); continue; }
                if lookahead == 73 || lookahead == 105 { state = 431; lexer.advance(false); continue; }
                return result;
            }
            196 => {
                if lookahead == 67 || lookahead == 99 { state = 432; lexer.advance(false); continue; }
                return result;
            }
            197 => {
                if lookahead == 84 || lookahead == 116 { state = 433; lexer.advance(false); continue; }
                return result;
            }
            198 => {
                if lookahead == 66 || lookahead == 98 { state = 434; lexer.advance(false); continue; }
                return result;
            }
            199 => {
                if lookahead == 80 || lookahead == 112 { state = 435; lexer.advance(false); continue; }
                return result;
            }
            200 => {
                if lookahead == 76 || lookahead == 108 { state = 436; lexer.advance(false); continue; }
                return result;
            }
            201 => {
                if lookahead == 72 || lookahead == 104 { state = 437; lexer.advance(false); continue; }
                return result;
            }
            202 => {
                if lookahead == 69 || lookahead == 101 { state = 438; lexer.advance(false); continue; }
                return result;
            }
            203 => {
                if lookahead == 79 || lookahead == 111 { state = 439; lexer.advance(false); continue; }
                if lookahead == 82 || lookahead == 114 { state = 440; lexer.advance(false); continue; }
                return result;
            }
            204 => {
                result = true; lexer.set_result_symbol(sym_keyword_end); lexer.mark_end();
                return result;
            }
            205 => {
                if lookahead == 73 || lookahead == 105 { state = 441; lexer.advance(false); continue; }
                return result;
            }
            206 => {
                if lookahead == 77 || lookahead == 109 { state = 442; lexer.advance(false); continue; }
                return result;
            }
            207 => {
                if lookahead == 65 || lookahead == 97 { state = 443; lexer.advance(false); continue; }
                return result;
            }
            208 => {
                if lookahead == 69 || lookahead == 101 { state = 444; lexer.advance(false); continue; }
                if lookahead == 76 || lookahead == 108 { state = 445; lexer.advance(false); continue; }
                return result;
            }
            209 => {
                if lookahead == 67 || lookahead == 99 { state = 446; lexer.advance(false); continue; }
                return result;
            }
            210 => {
                if lookahead == 83 || lookahead == 115 { state = 447; lexer.advance(false); continue; }
                return result;
            }
            211 => {
                if lookahead == 76 || lookahead == 108 { state = 448; lexer.advance(false); continue; }
                return result;
            }
            212 => {
                if lookahead == 69 || lookahead == 101 { state = 449; lexer.advance(false); continue; }
                return result;
            }
            213 => {
                if lookahead == 83 || lookahead == 115 { state = 450; lexer.advance(false); continue; }
                return result;
            }
            214 => {
                if lookahead == 76 || lookahead == 108 { state = 451; lexer.advance(false); continue; }
                return result;
            }
            215 => {
                if lookahead == 84 || lookahead == 116 { state = 452; lexer.advance(false); continue; }
                return result;
            }
            216 => {
                if lookahead == 83 || lookahead == 115 { state = 453; lexer.advance(false); continue; }
                return result;
            }
            217 => {
                if lookahead == 65 || lookahead == 97 { state = 454; lexer.advance(false); continue; }
                return result;
            }
            218 => {
                if lookahead == 76 || lookahead == 108 { state = 455; lexer.advance(false); continue; }
                return result;
            }
            219 => {
                result = true; lexer.set_result_symbol(sym_keyword_for); lexer.mark_end();
                if lookahead == 67 || lookahead == 99 { state = 456; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 457; lexer.advance(false); continue; }
                if lookahead == 77 || lookahead == 109 { state = 458; lexer.advance(false); continue; }
                return result;
            }
            220 => {
                if lookahead == 69 || lookahead == 101 { state = 459; lexer.advance(false); continue; }
                return result;
            }
            221 => {
                if lookahead == 77 || lookahead == 109 { state = 460; lexer.advance(false); continue; }
                return result;
            }
            222 => {
                if lookahead == 76 || lookahead == 108 { state = 461; lexer.advance(false); continue; }
                return result;
            }
            223 => {
                if lookahead == 67 || lookahead == 99 { state = 462; lexer.advance(false); continue; }
                return result;
            }
            224 => {
                if lookahead == 69 || lookahead == 101 { state = 463; lexer.advance(false); continue; }
                return result;
            }
            225 => {
                if lookahead == 71 || lookahead == 103 { state = 464; lexer.advance(false); continue; }
                if lookahead == 77 || lookahead == 109 { state = 465; lexer.advance(false); continue; }
                return result;
            }
            226 => {
                result = true; lexer.set_result_symbol(sym_keyword_gin); lexer.mark_end();
                return result;
            }
            227 => {
                if lookahead == 84 || lookahead == 116 { state = 466; lexer.advance(false); continue; }
                return result;
            }
            228 => {
                if lookahead == 85 || lookahead == 117 { state = 467; lexer.advance(false); continue; }
                return result;
            }
            229 => {
                if lookahead == 72 || lookahead == 104 { state = 468; lexer.advance(false); continue; }
                return result;
            }
            230 => {
                if lookahead == 73 || lookahead == 105 { state = 469; lexer.advance(false); continue; }
                return result;
            }
            231 => {
                if lookahead == 68 || lookahead == 100 { state = 470; lexer.advance(false); continue; }
                return result;
            }
            232 => {
                if lookahead == 72 || lookahead == 104 { state = 471; lexer.advance(false); continue; }
                return result;
            }
            233 => {
                if lookahead == 79 || lookahead == 111 { state = 472; lexer.advance(false); continue; }
                return result;
            }
            234 => {
                if lookahead == 75 || lookahead == 107 { state = 473; lexer.advance(false); continue; }
                return result;
            }
            235 => {
                if lookahead == 71 || lookahead == 103 { state = 474; lexer.advance(false); continue; }
                return result;
            }
            236 => {
                if lookahead == 69 || lookahead == 101 { state = 475; lexer.advance(false); continue; }
                if lookahead == 85 || lookahead == 117 { state = 476; lexer.advance(false); continue; }
                return result;
            }
            237 => {
                if lookahead == 82 || lookahead == 114 { state = 477; lexer.advance(false); continue; }
                return result;
            }
            238 => {
                if lookahead == 69 || lookahead == 101 { state = 478; lexer.advance(false); continue; }
                return result;
            }
            239 => {
                if lookahead == 84 || lookahead == 116 { state = 479; lexer.advance(false); continue; }
                return result;
            }
            240 => {
                if lookahead == 84 || lookahead == 116 { state = 480; lexer.advance(false); continue; }
                return result;
            }
            241 => {
                if lookahead == 69 || lookahead == 101 { state = 481; lexer.advance(false); continue; }
                return result;
            }
            242 => {
                if lookahead == 85 || lookahead == 117 { state = 482; lexer.advance(false); continue; }
                return result;
            }
            243 => {
                if lookahead == 85 || lookahead == 117 { state = 483; lexer.advance(false); continue; }
                return result;
            }
            244 => {
                if lookahead == 69 || lookahead == 101 { state = 484; lexer.advance(false); continue; }
                if lookahead == 84 || lookahead == 116 { state = 485; lexer.advance(false); continue; }
                return result;
            }
            245 => {
                result = true; lexer.set_result_symbol(aux_sym_keyword_int_token1); lexer.mark_end();
                if let Some(next) = advance_map(&[
                    (49, 486), (50, 487), (51, 488), (52, 489), (56, 490), (69, 491), (101, 491), (79, 492),
                    (111, 492),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            246 => {
                if lookahead == 79 || lookahead == 111 { state = 493; lexer.advance(false); continue; }
                return result;
            }
            247 => {
                if lookahead == 76 || lookahead == 108 { state = 494; lexer.advance(false); continue; }
                return result;
            }
            248 => {
                if lookahead == 78 || lookahead == 110 { state = 495; lexer.advance(false); continue; }
                return result;
            }
            249 => {
                if lookahead == 78 || lookahead == 110 { state = 496; lexer.advance(false); continue; }
                return result;
            }
            250 => {
                result = true; lexer.set_result_symbol(sym_keyword_key); lexer.mark_end();
                return result;
            }
            251 => {
                if lookahead == 71 || lookahead == 103 { state = 497; lexer.advance(false); continue; }
                return result;
            }
            252 => {
                if lookahead == 84 || lookahead == 116 { state = 498; lexer.advance(false); continue; }
                return result;
            }
            253 => {
                if lookahead == 69 || lookahead == 101 { state = 499; lexer.advance(false); continue; }
                return result;
            }
            254 => {
                if lookahead == 75 || lookahead == 107 { state = 500; lexer.advance(false); continue; }
                return result;
            }
            255 => {
                if lookahead == 84 || lookahead == 116 { state = 501; lexer.advance(false); continue; }
                return result;
            }
            256 => {
                if lookahead == 69 || lookahead == 101 { state = 502; lexer.advance(false); continue; }
                return result;
            }
            257 => {
                if lookahead == 69 || lookahead == 101 { state = 503; lexer.advance(false); continue; }
                return result;
            }
            258 => {
                if lookahead == 73 || lookahead == 105 { state = 504; lexer.advance(false); continue; }
                return result;
            }
            259 => {
                if lookahead == 69 || lookahead == 101 { state = 505; lexer.advance(false); continue; }
                return result;
            }
            260 => {
                if lookahead == 65 || lookahead == 97 { state = 506; lexer.advance(false); continue; }
                return result;
            }
            261 => {
                if lookahead == 71 || lookahead == 103 { state = 507; lexer.advance(false); continue; }
                return result;
            }
            262 => {
                if lookahead == 95 { state = 508; lexer.advance(false); continue; }
                return result;
            }
            263 => {
                if lookahead == 78 || lookahead == 110 { state = 509; lexer.advance(false); continue; }
                return result;
            }
            264 => {
                if lookahead == 67 || lookahead == 99 { state = 510; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 511; lexer.advance(false); continue; }
                return result;
            }
            265 => {
                if lookahead == 86 || lookahead == 118 { state = 512; lexer.advance(false); continue; }
                return result;
            }
            266 => {
                if lookahead == 73 || lookahead == 105 { state = 513; lexer.advance(false); continue; }
                return result;
            }
            267 => {
                if lookahead == 71 || lookahead == 103 { state = 514; lexer.advance(false); continue; }
                return result;
            }
            268 => {
                if lookahead == 65 || lookahead == 97 { state = 515; lexer.advance(false); continue; }
                return result;
            }
            269 => {
                if lookahead == 86 || lookahead == 118 { state = 516; lexer.advance(false); continue; }
                return result;
            }
            270 => {
                if lookahead == 73 || lookahead == 105 { state = 517; lexer.advance(false); continue; }
                return result;
            }
            271 => {
                if lookahead == 69 || lookahead == 101 { state = 518; lexer.advance(false); continue; }
                return result;
            }
            272 => {
                if lookahead == 69 || lookahead == 101 { state = 519; lexer.advance(false); continue; }
                return result;
            }
            273 => {
                if lookahead == 85 || lookahead == 117 { state = 520; lexer.advance(false); continue; }
                return result;
            }
            274 => {
                if lookahead == 65 || lookahead == 97 { state = 521; lexer.advance(false); continue; }
                return result;
            }
            275 => {
                result = true; lexer.set_result_symbol(sym_keyword_new); lexer.mark_end();
                return result;
            }
            276 => {
                if lookahead == 69 || lookahead == 101 { state = 522; lexer.advance(false); continue; }
                return result;
            }
            277 => {
                if lookahead == 67 || lookahead == 99 { state = 523; lexer.advance(false); continue; }
                return result;
            }
            278 => {
                result = true; lexer.set_result_symbol(sym_keyword_not); lexer.mark_end();
                if lookahead == 72 || lookahead == 104 { state = 524; lexer.advance(false); continue; }
                return result;
            }
            279 => {
                if lookahead == 65 || lookahead == 97 { state = 525; lexer.advance(false); continue; }
                return result;
            }
            280 => {
                if lookahead == 76 || lookahead == 108 { state = 526; lexer.advance(false); continue; }
                return result;
            }
            281 => {
                if lookahead == 69 || lookahead == 101 { state = 527; lexer.advance(false); continue; }
                return result;
            }
            282 => {
                if lookahead == 82 || lookahead == 114 { state = 528; lexer.advance(false); continue; }
                return result;
            }
            283 => {
                if lookahead == 69 || lookahead == 101 { state = 529; lexer.advance(false); continue; }
                return result;
            }
            284 => {
                result = true; lexer.set_result_symbol(sym_keyword_off); lexer.mark_end();
                if lookahead == 83 || lookahead == 115 { state = 530; lexer.advance(false); continue; }
                return result;
            }
            285 => {
                result = true; lexer.set_result_symbol(sym_keyword_oid); lexer.mark_end();
                if lookahead == 83 || lookahead == 115 { state = 531; lexer.advance(false); continue; }
                return result;
            }
            286 => {
                result = true; lexer.set_result_symbol(sym_keyword_old); lexer.mark_end();
                return result;
            }
            287 => {
                if lookahead == 89 || lookahead == 121 { state = 532; lexer.advance(false); continue; }
                return result;
            }
            288 => {
                if lookahead == 73 || lookahead == 105 { state = 533; lexer.advance(false); continue; }
                return result;
            }
            289 => {
                result = true; lexer.set_result_symbol(sym_keyword_orc); lexer.mark_end();
                return result;
            }
            290 => {
                if lookahead == 69 || lookahead == 101 { state = 534; lexer.advance(false); continue; }
                if lookahead == 73 || lookahead == 105 { state = 535; lexer.advance(false); continue; }
                return result;
            }
            291 => {
                if lookahead == 69 || lookahead == 101 { state = 536; lexer.advance(false); continue; }
                return result;
            }
            292 => {
                result = true; lexer.set_result_symbol(sym_keyword_out); lexer.mark_end();
                if lookahead == 69 || lookahead == 101 { state = 537; lexer.advance(false); continue; }
                return result;
            }
            293 => {
                if lookahead == 82 || lookahead == 114 { state = 538; lexer.advance(false); continue; }
                return result;
            }
            294 => {
                if lookahead == 69 || lookahead == 101 { state = 539; lexer.advance(false); continue; }
                return result;
            }
            295 => {
                if lookahead == 65 || lookahead == 97 { state = 540; lexer.advance(false); continue; }
                if lookahead == 81 || lookahead == 113 { state = 541; lexer.advance(false); continue; }
                if lookahead == 84 || lookahead == 116 { state = 542; lexer.advance(false); continue; }
                return result;
            }
            296 => {
                if lookahead == 83 || lookahead == 115 { state = 543; lexer.advance(false); continue; }
                return result;
            }
            297 => {
                if lookahead == 73 || lookahead == 105 { state = 544; lexer.advance(false); continue; }
                return result;
            }
            298 => {
                if lookahead == 67 || lookahead == 99 { state = 545; lexer.advance(false); continue; }
                return result;
            }
            299 => {
                if lookahead == 77 || lookahead == 109 { state = 546; lexer.advance(false); continue; }
                return result;
            }
            300 => {
                if lookahead == 67 || lookahead == 99 { state = 547; lexer.advance(false); continue; }
                if lookahead == 71 || lookahead == 103 { state = 548; lexer.advance(false); continue; }
                return result;
            }
            301 => {
                if lookahead == 84 || lookahead == 116 { state = 549; lexer.advance(false); continue; }
                return result;
            }
            302 => {
                if lookahead == 71 || lookahead == 103 { state = 550; lexer.advance(false); continue; }
                return result;
            }
            303 => {
                if lookahead == 73 || lookahead == 105 { state = 551; lexer.advance(false); continue; }
                return result;
            }
            304 => {
                if lookahead == 68 || lookahead == 100 { state = 552; lexer.advance(false); continue; }
                if lookahead == 76 || lookahead == 108 { state = 553; lexer.advance(false); continue; }
                return result;
            }
            305 => {
                if lookahead == 85 || lookahead == 117 { state = 554; lexer.advance(false); continue; }
                return result;
            }
            306 => {
                if lookahead == 69 || lookahead == 101 { state = 555; lexer.advance(false); continue; }
                return result;
            }
            307 => {
                if let Some(next) = advance_map(&[
                    (67, 556), (99, 556), (78, 557), (110, 557), (80, 558), (112, 558), (84, 559), (116, 559),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            308 => {
                if lookahead == 65 || lookahead == 97 { state = 560; lexer.advance(false); continue; }
                return result;
            }
            309 => {
                if lookahead == 69 || lookahead == 101 { state = 561; lexer.advance(false); continue; }
                if lookahead == 76 || lookahead == 108 { state = 562; lexer.advance(false); continue; }
                return result;
            }
            310 => {
                if lookahead == 69 || lookahead == 101 { state = 563; lexer.advance(false); continue; }
                if lookahead == 84 || lookahead == 116 { state = 564; lexer.advance(false); continue; }
                return result;
            }
            311 => {
                if lookahead == 85 || lookahead == 117 { state = 565; lexer.advance(false); continue; }
                return result;
            }
            312 => {
                if lookahead == 82 || lookahead == 114 { state = 566; lexer.advance(false); continue; }
                return result;
            }
            313 => {
                if lookahead == 72 || lookahead == 104 { state = 567; lexer.advance(false); continue; }
                return result;
            }
            314 => {
                if lookahead == 69 || lookahead == 101 { state = 568; lexer.advance(false); continue; }
                if lookahead == 76 || lookahead == 108 { state = 569; lexer.advance(false); continue; }
                return result;
            }
            315 => {
                result = true; lexer.set_result_symbol(sym_keyword_row); lexer.mark_end();
                if lookahead == 83 || lookahead == 115 { state = 570; lexer.advance(false); continue; }
                return result;
            }
            316 => {
                if lookahead == 69 || lookahead == 101 { state = 571; lexer.advance(false); continue; }
                return result;
            }
            317 => {
                if lookahead == 69 || lookahead == 101 { state = 572; lexer.advance(false); continue; }
                return result;
            }
            318 => {
                if lookahead == 85 || lookahead == 117 { state = 573; lexer.advance(false); continue; }
                return result;
            }
            319 => {
                if lookahead == 69 || lookahead == 101 { state = 574; lexer.advance(false); continue; }
                return result;
            }
            320 => {
                if lookahead == 65 || lookahead == 97 { state = 575; lexer.advance(false); continue; }
                return result;
            }
            321 => {
                if lookahead == 85 || lookahead == 117 { state = 576; lexer.advance(false); continue; }
                return result;
            }
            322 => {
                if lookahead == 73 || lookahead == 105 { state = 577; lexer.advance(false); continue; }
                return result;
            }
            323 => {
                if lookahead == 83 || lookahead == 115 { state = 578; lexer.advance(false); continue; }
                return result;
            }
            324 => {
                result = true; lexer.set_result_symbol(sym_keyword_set); lexer.mark_end();
                if lookahead == 79 || lookahead == 111 { state = 579; lexer.advance(false); continue; }
                return result;
            }
            325 => {
                if lookahead == 87 || lookahead == 119 { state = 580; lexer.advance(false); continue; }
                return result;
            }
            326 => {
                if lookahead == 73 || lookahead == 105 { state = 581; lexer.advance(false); continue; }
                return result;
            }
            327 => {
                if lookahead == 76 || lookahead == 108 { state = 582; lexer.advance(false); continue; }
                return result;
            }
            328 => {
                if lookahead == 80 || lookahead == 112 { state = 583; lexer.advance(false); continue; }
                return result;
            }
            329 => {
                if lookahead == 69 || lookahead == 101 { state = 584; lexer.advance(false); continue; }
                return result;
            }
            330 => {
                if lookahead == 84 || lookahead == 116 { state = 585; lexer.advance(false); continue; }
                return result;
            }
            331 => {
                if lookahead == 73 || lookahead == 105 { state = 586; lexer.advance(false); continue; }
                return result;
            }
            332 => {
                if lookahead == 66 || lookahead == 98 { state = 587; lexer.advance(false); continue; }
                if lookahead == 82 || lookahead == 114 { state = 588; lexer.advance(false); continue; }
                if lookahead == 84 || lookahead == 116 { state = 589; lexer.advance(false); continue; }
                return result;
            }
            333 => {
                if lookahead == 73 || lookahead == 105 { state = 590; lexer.advance(false); continue; }
                return result;
            }
            334 => {
                if lookahead == 82 || lookahead == 114 { state = 591; lexer.advance(false); continue; }
                return result;
            }
            335 => {
                if lookahead == 73 || lookahead == 105 { state = 592; lexer.advance(false); continue; }
                return result;
            }
            336 => {
                if lookahead == 80 || lookahead == 112 { state = 593; lexer.advance(false); continue; }
                return result;
            }
            337 => {
                if lookahead == 76 || lookahead == 108 { state = 594; lexer.advance(false); continue; }
                return result;
            }
            338 => {
                if lookahead == 80 || lookahead == 112 { state = 595; lexer.advance(false); continue; }
                return result;
            }
            339 => {
                if lookahead == 80 || lookahead == 112 { state = 596; lexer.advance(false); continue; }
                return result;
            }
            340 => {
                if lookahead == 77 || lookahead == 109 { state = 597; lexer.advance(false); continue; }
                return result;
            }
            341 => {
                if lookahead == 84 || lookahead == 116 { state = 598; lexer.advance(false); continue; }
                return result;
            }
            342 => {
                if lookahead == 78 || lookahead == 110 { state = 599; lexer.advance(false); continue; }
                return result;
            }
            343 => {
                if lookahead == 83 || lookahead == 115 { state = 600; lexer.advance(false); continue; }
                return result;
            }
            344 => {
                if lookahead == 69 || lookahead == 101 { state = 601; lexer.advance(false); continue; }
                return result;
            }
            345 => {
                if lookahead == 89 || lookahead == 121 { state = 602; lexer.advance(false); continue; }
                return result;
            }
            346 => {
                if lookahead == 78 || lookahead == 110 { state = 603; lexer.advance(false); continue; }
                return result;
            }
            347 => {
                if lookahead == 71 || lookahead == 103 { state = 604; lexer.advance(false); continue; }
                return result;
            }
            348 => {
                if lookahead == 69 || lookahead == 101 { state = 605; lexer.advance(false); continue; }
                if lookahead == 78 || lookahead == 110 { state = 606; lexer.advance(false); continue; }
                return result;
            }
            349 => {
                if lookahead == 69 || lookahead == 101 { state = 607; lexer.advance(false); continue; }
                return result;
            }
            350 => {
                if lookahead == 79 || lookahead == 111 { state = 608; lexer.advance(false); continue; }
                return result;
            }
            351 => {
                if lookahead == 65 || lookahead == 97 { state = 609; lexer.advance(false); continue; }
                if lookahead == 79 || lookahead == 111 { state = 610; lexer.advance(false); continue; }
                return result;
            }
            352 => {
                if lookahead == 79 || lookahead == 111 { state = 611; lexer.advance(false); continue; }
                if lookahead == 81 || lookahead == 113 { state = 612; lexer.advance(false); continue; }
                return result;
            }
            353 => {
                if lookahead == 79 || lookahead == 111 { state = 613; lexer.advance(false); continue; }
                return result;
            }
            354 => {
                if lookahead == 65 || lookahead == 97 { state = 614; lexer.advance(false); continue; }
                if lookahead == 73 || lookahead == 105 { state = 615; lexer.advance(false); continue; }
                return result;
            }
            355 => {
                if lookahead == 73 || lookahead == 105 { state = 616; lexer.advance(false); continue; }
                return result;
            }
            356 => {
                if lookahead == 65 || lookahead == 97 { state = 617; lexer.advance(false); continue; }
                return result;
            }
            357 => {
                result = true; lexer.set_result_symbol(sym_keyword_use); lexer.mark_end();
                if lookahead == 82 || lookahead == 114 { state = 618; lexer.advance(false); continue; }
                return result;
            }
            358 => {
                if lookahead == 78 || lookahead == 110 { state = 619; lexer.advance(false); continue; }
                return result;
            }
            359 => {
                if lookahead == 68 || lookahead == 100 { state = 620; lexer.advance(false); continue; }
                return result;
            }
            360 => {
                if lookahead == 85 || lookahead == 117 { state = 621; lexer.advance(false); continue; }
                return result;
            }
            361 => {
                if lookahead == 73 || lookahead == 105 { state = 622; lexer.advance(false); continue; }
                if lookahead == 85 || lookahead == 117 { state = 623; lexer.advance(false); continue; }
                return result;
            }
            362 => {
                if let Some(next) = advance_map(&[
                    (66, 624), (98, 624), (67, 625), (99, 625), (73, 626), (105, 626), (89, 627), (121, 627),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            363 => {
                if lookahead == 66 || lookahead == 98 { state = 628; lexer.advance(false); continue; }
                if lookahead == 83 || lookahead == 115 { state = 629; lexer.advance(false); continue; }
                return result;
            }
            364 => {
                if lookahead == 87 || lookahead == 119 { state = 630; lexer.advance(false); continue; }
                return result;
            }
            365 => {
                if lookahead == 84 || lookahead == 116 { state = 631; lexer.advance(false); continue; }
                return result;
            }
            366 => {
                if lookahead == 65 || lookahead == 97 { state = 632; lexer.advance(false); continue; }
                return result;
            }
            367 => {
                if lookahead == 84 || lookahead == 116 { state = 633; lexer.advance(false); continue; }
                return result;
            }
            368 => {
                if lookahead == 78 || lookahead == 110 { state = 634; lexer.advance(false); continue; }
                if lookahead == 82 || lookahead == 114 { state = 635; lexer.advance(false); continue; }
                return result;
            }
            369 => {
                if lookahead == 68 || lookahead == 100 { state = 636; lexer.advance(false); continue; }
                return result;
            }
            370 => {
                if lookahead == 72 || lookahead == 104 { state = 637; lexer.advance(false); continue; }
                return result;
            }
            371 => {
                if lookahead == 84 || lookahead == 116 { state = 638; lexer.advance(false); continue; }
                return result;
            }
            372 => {
                result = true; lexer.set_result_symbol(sym_keyword_xml); lexer.mark_end();
                return result;
            }
            373 => {
                if lookahead == 79 || lookahead == 111 { state = 639; lexer.advance(false); continue; }
                return result;
            }
            374 => {
                if lookahead == 69 || lookahead == 101 { state = 640; lexer.advance(false); continue; }
                return result;
            }
            375 => {
                if lookahead == 79 || lookahead == 111 { state = 641; lexer.advance(false); continue; }
                return result;
            }
            376 => {
                if lookahead == 78 || lookahead == 110 { state = 642; lexer.advance(false); continue; }
                return result;
            }
            377 => {
                if lookahead == 82 || lookahead == 114 { state = 643; lexer.advance(false); continue; }
                return result;
            }
            378 => {
                if lookahead == 82 || lookahead == 114 { state = 644; lexer.advance(false); continue; }
                return result;
            }
            379 => {
                if lookahead == 89 || lookahead == 121 { state = 645; lexer.advance(false); continue; }
                return result;
            }
            380 => {
                if lookahead == 89 || lookahead == 121 { state = 646; lexer.advance(false); continue; }
                return result;
            }
            381 => {
                if lookahead == 89 || lookahead == 121 { state = 647; lexer.advance(false); continue; }
                return result;
            }
            382 => {
                if lookahead == 73 || lookahead == 105 { state = 648; lexer.advance(false); continue; }
                return result;
            }
            383 => {
                if lookahead == 73 || lookahead == 105 { state = 649; lexer.advance(false); continue; }
                return result;
            }
            384 => {
                if lookahead == 79 || lookahead == 111 { state = 650; lexer.advance(false); continue; }
                return result;
            }
            385 => {
                if lookahead == 95 { state = 651; lexer.advance(false); continue; }
                return result;
            }
            386 => {
                result = true; lexer.set_result_symbol(sym_keyword_avro); lexer.mark_end();
                return result;
            }
            387 => {
                if lookahead == 82 || lookahead == 114 { state = 652; lexer.advance(false); continue; }
                return result;
            }
            388 => {
                if lookahead == 78 || lookahead == 110 { state = 653; lexer.advance(false); continue; }
                return result;
            }
            389 => {
                if lookahead == 69 || lookahead == 101 { state = 654; lexer.advance(false); continue; }
                return result;
            }
            390 => {
                if lookahead == 78 || lookahead == 110 { state = 655; lexer.advance(false); continue; }
                return result;
            }
            391 => {
                if lookahead == 69 || lookahead == 101 { state = 656; lexer.advance(false); continue; }
                return result;
            }
            392 => {
                if lookahead == 80 || lookahead == 112 { state = 657; lexer.advance(false); continue; }
                return result;
            }
            393 => {
                if lookahead == 82 || lookahead == 114 { state = 658; lexer.advance(false); continue; }
                return result;
            }
            394 => {
                if lookahead == 69 || lookahead == 101 { state = 659; lexer.advance(false); continue; }
                return result;
            }
            395 => {
                if lookahead == 68 || lookahead == 100 { state = 660; lexer.advance(false); continue; }
                return result;
            }
            396 => {
                if lookahead == 68 || lookahead == 100 { state = 661; lexer.advance(false); continue; }
                return result;
            }
            397 => {
                result = true; lexer.set_result_symbol(sym_keyword_brin); lexer.mark_end();
                return result;
            }
            398 => {
                if lookahead == 69 || lookahead == 101 { state = 662; lexer.advance(false); continue; }
                return result;
            }
            399 => {
                if lookahead == 65 || lookahead == 97 { state = 663; lexer.advance(false); continue; }
                return result;
            }
            400 => {
                if lookahead == 69 || lookahead == 101 { state = 664; lexer.advance(false); continue; }
                return result;
            }
            401 => {
                if lookahead == 69 || lookahead == 101 { state = 665; lexer.advance(false); continue; }
                return result;
            }
            402 => {
                if lookahead == 65 || lookahead == 97 { state = 666; lexer.advance(false); continue; }
                return result;
            }
            403 => {
                result = true; lexer.set_result_symbol(sym_keyword_case); lexer.mark_end();
                return result;
            }
            404 => {
                result = true; lexer.set_result_symbol(sym_keyword_cast); lexer.mark_end();
                return result;
            }
            405 => {
                if lookahead == 71 || lookahead == 103 { state = 667; lexer.advance(false); continue; }
                return result;
            }
            406 => {
                result = true; lexer.set_result_symbol(aux_sym_keyword_char_token1); lexer.mark_end();
                if lookahead == 65 || lookahead == 97 { state = 668; lexer.advance(false); continue; }
                return result;
            }
            407 => {
                if lookahead == 75 || lookahead == 107 { state = 669; lexer.advance(false); continue; }
                return result;
            }
            408 => {
                if lookahead == 65 || lookahead == 97 { state = 670; lexer.advance(false); continue; }
                return result;
            }
            409 => {
                if lookahead == 77 || lookahead == 109 { state = 671; lexer.advance(false); continue; }
                return result;
            }
            410 => {
                if lookahead == 69 || lookahead == 101 { state = 672; lexer.advance(false); continue; }
                if lookahead == 73 || lookahead == 105 { state = 673; lexer.advance(false); continue; }
                return result;
            }
            411 => {
                if lookahead == 82 || lookahead == 114 { state = 674; lexer.advance(false); continue; }
                if lookahead == 85 || lookahead == 117 { state = 675; lexer.advance(false); continue; }
                return result;
            }
            412 => {
                if lookahead == 85 || lookahead == 117 { state = 676; lexer.advance(false); continue; }
                return result;
            }
            413 => {
                if lookahead == 76 || lookahead == 108 { state = 677; lexer.advance(false); continue; }
                return result;
            }
            414 => {
                if lookahead == 69 || lookahead == 101 { state = 678; lexer.advance(false); continue; }
                return result;
            }
            415 => {
                if lookahead == 84 || lookahead == 116 { state = 679; lexer.advance(false); continue; }
                return result;
            }
            416 => {
                result = true; lexer.set_result_symbol(sym_keyword_copy); lexer.mark_end();
                return result;
            }
            417 => {
                result = true; lexer.set_result_symbol(sym_keyword_cost); lexer.mark_end();
                return result;
            }
            418 => {
                if lookahead == 84 || lookahead == 116 { state = 680; lexer.advance(false); continue; }
                return result;
            }
            419 => {
                if lookahead == 83 || lookahead == 115 { state = 681; lexer.advance(false); continue; }
                return result;
            }
            420 => {
                if lookahead == 69 || lookahead == 101 { state = 682; lexer.advance(false); continue; }
                return result;
            }
            421 => {
                if lookahead == 69 || lookahead == 101 { state = 683; lexer.advance(false); continue; }
                return result;
            }
            422 => {
                result = true; lexer.set_result_symbol(sym_keyword_data); lexer.mark_end();
                if lookahead == 66 || lookahead == 98 { state = 684; lexer.advance(false); continue; }
                return result;
            }
            423 => {
                result = true; lexer.set_result_symbol(sym_keyword_date); lexer.mark_end();
                if lookahead == 84 || lookahead == 116 { state = 685; lexer.advance(false); continue; }
                return result;
            }
            424 => {
                if lookahead == 77 || lookahead == 109 { state = 686; lexer.advance(false); continue; }
                return result;
            }
            425 => {
                if lookahead == 65 || lookahead == 97 { state = 687; lexer.advance(false); continue; }
                return result;
            }
            426 => {
                if lookahead == 85 || lookahead == 117 { state = 688; lexer.advance(false); continue; }
                return result;
            }
            427 => {
                if lookahead == 82 || lookahead == 114 { state = 689; lexer.advance(false); continue; }
                return result;
            }
            428 => {
                if lookahead == 78 || lookahead == 110 { state = 690; lexer.advance(false); continue; }
                return result;
            }
            429 => {
                if lookahead == 89 || lookahead == 121 { state = 691; lexer.advance(false); continue; }
                return result;
            }
            430 => {
                if lookahead == 84 || lookahead == 116 { state = 692; lexer.advance(false); continue; }
                return result;
            }
            431 => {
                if lookahead == 77 || lookahead == 109 { state = 693; lexer.advance(false); continue; }
                return result;
            }
            432 => {
                result = true; lexer.set_result_symbol(sym_keyword_desc); lexer.mark_end();
                return result;
            }
            433 => {
                if lookahead == 73 || lookahead == 105 { state = 694; lexer.advance(false); continue; }
                return result;
            }
            434 => {
                if lookahead == 76 || lookahead == 108 { state = 695; lexer.advance(false); continue; }
                return result;
            }
            435 => {
                result = true; lexer.set_result_symbol(sym_keyword_drop); lexer.mark_end();
                return result;
            }
            436 => {
                if lookahead == 73 || lookahead == 105 { state = 696; lexer.advance(false); continue; }
                return result;
            }
            437 => {
                result = true; lexer.set_result_symbol(sym_keyword_each); lexer.mark_end();
                return result;
            }
            438 => {
                result = true; lexer.set_result_symbol(sym_keyword_else); lexer.mark_end();
                return result;
            }
            439 => {
                if lookahead == 68 || lookahead == 100 { state = 697; lexer.advance(false); continue; }
                return result;
            }
            440 => {
                if lookahead == 89 || lookahead == 121 { state = 698; lexer.advance(false); continue; }
                return result;
            }
            441 => {
                if lookahead == 78 || lookahead == 110 { state = 699; lexer.advance(false); continue; }
                return result;
            }
            442 => {
                result = true; lexer.set_result_symbol(sym_keyword_enum); lexer.mark_end();
                return result;
            }
            443 => {
                if lookahead == 80 || lookahead == 112 { state = 700; lexer.advance(false); continue; }
                return result;
            }
            444 => {
                if lookahead == 80 || lookahead == 112 { state = 701; lexer.advance(false); continue; }
                return result;
            }
            445 => {
                if lookahead == 85 || lookahead == 117 { state = 702; lexer.advance(false); continue; }
                return result;
            }
            446 => {
                if lookahead == 85 || lookahead == 117 { state = 703; lexer.advance(false); continue; }
                return result;
            }
            447 => {
                if lookahead == 84 || lookahead == 116 { state = 704; lexer.advance(false); continue; }
                return result;
            }
            448 => {
                if lookahead == 65 || lookahead == 97 { state = 705; lexer.advance(false); continue; }
                return result;
            }
            449 => {
                if lookahead == 78 || lookahead == 110 { state = 706; lexer.advance(false); continue; }
                if lookahead == 82 || lookahead == 114 { state = 707; lexer.advance(false); continue; }
                return result;
            }
            450 => {
                if lookahead == 69 || lookahead == 101 { state = 708; lexer.advance(false); continue; }
                return result;
            }
            451 => {
                if lookahead == 68 || lookahead == 100 { state = 709; lexer.advance(false); continue; }
                return result;
            }
            452 => {
                if lookahead == 69 || lookahead == 101 { state = 710; lexer.advance(false); continue; }
                return result;
            }
            453 => {
                if lookahead == 84 || lookahead == 116 { state = 711; lexer.advance(false); continue; }
                return result;
            }
            454 => {
                if lookahead == 84 || lookahead == 116 { state = 712; lexer.advance(false); continue; }
                return result;
            }
            455 => {
                if lookahead == 79 || lookahead == 111 { state = 713; lexer.advance(false); continue; }
                return result;
            }
            456 => {
                if lookahead == 69 || lookahead == 101 { state = 714; lexer.advance(false); continue; }
                return result;
            }
            457 => {
                if lookahead == 73 || lookahead == 105 { state = 715; lexer.advance(false); continue; }
                return result;
            }
            458 => {
                if lookahead == 65 || lookahead == 97 { state = 716; lexer.advance(false); continue; }
                return result;
            }
            459 => {
                if lookahead == 90 || lookahead == 122 { state = 717; lexer.advance(false); continue; }
                return result;
            }
            460 => {
                result = true; lexer.set_result_symbol(sym_keyword_from); lexer.mark_end();
                return result;
            }
            461 => {
                result = true; lexer.set_result_symbol(sym_keyword_full); lexer.mark_end();
                return result;
            }
            462 => {
                if lookahead == 84 || lookahead == 116 { state = 718; lexer.advance(false); continue; }
                return result;
            }
            463 => {
                if lookahead == 82 || lookahead == 114 { state = 719; lexer.advance(false); continue; }
                return result;
            }
            464 => {
                if lookahead == 82 || lookahead == 114 { state = 720; lexer.advance(false); continue; }
                return result;
            }
            465 => {
                if lookahead == 69 || lookahead == 101 { state = 721; lexer.advance(false); continue; }
                return result;
            }
            466 => {
                result = true; lexer.set_result_symbol(sym_keyword_gist); lexer.mark_end();
                return result;
            }
            467 => {
                if lookahead == 80 || lookahead == 112 { state = 722; lexer.advance(false); continue; }
                return result;
            }
            468 => {
                result = true; lexer.set_result_symbol(sym_keyword_hash); lexer.mark_end();
                return result;
            }
            469 => {
                if lookahead == 78 || lookahead == 110 { state = 723; lexer.advance(false); continue; }
                return result;
            }
            470 => {
                if lookahead == 69 || lookahead == 101 { state = 724; lexer.advance(false); continue; }
                return result;
            }
            471 => {
                if lookahead == 95 { state = 725; lexer.advance(false); continue; }
                return result;
            }
            472 => {
                if lookahead == 82 || lookahead == 114 { state = 726; lexer.advance(false); continue; }
                return result;
            }
            473 => {
                if lookahead == 69 || lookahead == 101 { state = 727; lexer.advance(false); continue; }
                return result;
            }
            474 => {
                if lookahead == 69 || lookahead == 101 { state = 728; lexer.advance(false); continue; }
                return result;
            }
            475 => {
                if lookahead == 68 || lookahead == 100 { state = 729; lexer.advance(false); continue; }
                return result;
            }
            476 => {
                if lookahead == 84 || lookahead == 116 { state = 730; lexer.advance(false); continue; }
                return result;
            }
            477 => {
                if lookahead == 69 || lookahead == 101 { state = 731; lexer.advance(false); continue; }
                return result;
            }
            478 => {
                if lookahead == 88 || lookahead == 120 { state = 732; lexer.advance(false); continue; }
                return result;
            }
            479 => {
                result = true; lexer.set_result_symbol(sym_keyword_inet); lexer.mark_end();
                return result;
            }
            480 => {
                if lookahead == 73 || lookahead == 105 { state = 733; lexer.advance(false); continue; }
                return result;
            }
            481 => {
                if lookahead == 82 || lookahead == 114 { state = 734; lexer.advance(false); continue; }
                return result;
            }
            482 => {
                if lookahead == 84 || lookahead == 116 { state = 735; lexer.advance(false); continue; }
                return result;
            }
            483 => {
                if lookahead == 84 || lookahead == 116 { state = 736; lexer.advance(false); continue; }
                return result;
            }
            484 => {
                if lookahead == 82 || lookahead == 114 { state = 737; lexer.advance(false); continue; }
                return result;
            }
            485 => {
                if lookahead == 69 || lookahead == 101 { state = 738; lexer.advance(false); continue; }
                return result;
            }
            486 => {
                result = true; lexer.set_result_symbol(aux_sym_keyword_tinyint_token2); lexer.mark_end();
                return result;
            }
            487 => {
                result = true; lexer.set_result_symbol(aux_sym_keyword_smallint_token2); lexer.mark_end();
                return result;
            }
            488 => {
                result = true; lexer.set_result_symbol(aux_sym_keyword_mediumint_token2); lexer.mark_end();
                return result;
            }
            489 => {
                result = true; lexer.set_result_symbol(aux_sym_keyword_int_token3); lexer.mark_end();
                return result;
            }
            490 => {
                result = true; lexer.set_result_symbol(aux_sym_keyword_bigint_token2); lexer.mark_end();
                return result;
            }
            491 => {
                if lookahead == 71 || lookahead == 103 { state = 739; lexer.advance(false); continue; }
                if lookahead == 82 || lookahead == 114 { state = 740; lexer.advance(false); continue; }
                return result;
            }
            492 => {
                result = true; lexer.set_result_symbol(sym_keyword_into); lexer.mark_end();
                return result;
            }
            493 => {
                if lookahead == 75 || lookahead == 107 { state = 741; lexer.advance(false); continue; }
                return result;
            }
            494 => {
                if lookahead == 65 || lookahead == 97 { state = 742; lexer.advance(false); continue; }
                return result;
            }
            495 => {
                result = true; lexer.set_result_symbol(sym_keyword_join); lexer.mark_end();
                return result;
            }
            496 => {
                result = true; lexer.set_result_symbol(sym_keyword_json); lexer.mark_end();
                if lookahead == 66 || lookahead == 98 { state = 743; lexer.advance(false); continue; }
                if lookahead == 70 || lookahead == 102 { state = 744; lexer.advance(false); continue; }
                return result;
            }
            497 => {
                if lookahead == 85 || lookahead == 117 { state = 745; lexer.advance(false); continue; }
                return result;
            }
            498 => {
                result = true; lexer.set_result_symbol(sym_keyword_last); lexer.mark_end();
                return result;
            }
            499 => {
                if lookahead == 82 || lookahead == 114 { state = 746; lexer.advance(false); continue; }
                return result;
            }
            500 => {
                if lookahead == 80 || lookahead == 112 { state = 747; lexer.advance(false); continue; }
                return result;
            }
            501 => {
                result = true; lexer.set_result_symbol(sym_keyword_left); lexer.mark_end();
                return result;
            }
            502 => {
                if lookahead == 76 || lookahead == 108 { state = 748; lexer.advance(false); continue; }
                return result;
            }
            503 => {
                result = true; lexer.set_result_symbol(aux_sym_keyword_like_token1); lexer.mark_end();
                return result;
            }
            504 => {
                if lookahead == 84 || lookahead == 116 { state = 749; lexer.advance(false); continue; }
                return result;
            }
            505 => {
                if lookahead == 83 || lookahead == 115 { state = 750; lexer.advance(false); continue; }
                return result;
            }
            506 => {
                if lookahead == 76 || lookahead == 108 { state = 751; lexer.advance(false); continue; }
                if lookahead == 84 || lookahead == 116 { state = 752; lexer.advance(false); continue; }
                return result;
            }
            507 => {
                if lookahead == 69 || lookahead == 101 { state = 753; lexer.advance(false); continue; }
                return result;
            }
            508 => {
                if lookahead == 80 || lookahead == 112 { state = 754; lexer.advance(false); continue; }
                return result;
            }
            509 => {
                result = true; lexer.set_result_symbol(sym_keyword_main); lexer.mark_end();
                return result;
            }
            510 => {
                if lookahead == 72 || lookahead == 104 { state = 755; lexer.advance(false); continue; }
                return result;
            }
            511 => {
                if lookahead == 82 || lookahead == 114 { state = 756; lexer.advance(false); continue; }
                return result;
            }
            512 => {
                if lookahead == 65 || lookahead == 97 { state = 757; lexer.advance(false); continue; }
                return result;
            }
            513 => {
                if lookahead == 85 || lookahead == 117 { state = 758; lexer.advance(false); continue; }
                return result;
            }
            514 => {
                if lookahead == 69 || lookahead == 101 { state = 759; lexer.advance(false); continue; }
                return result;
            }
            515 => {
                if lookahead == 68 || lookahead == 100 { state = 760; lexer.advance(false); continue; }
                return result;
            }
            516 => {
                if lookahead == 65 || lookahead == 97 { state = 761; lexer.advance(false); continue; }
                return result;
            }
            517 => {
                if lookahead == 70 || lookahead == 102 { state = 762; lexer.advance(false); continue; }
                return result;
            }
            518 => {
                if lookahead == 89 || lookahead == 121 { state = 763; lexer.advance(false); continue; }
                return result;
            }
            519 => {
                result = true; lexer.set_result_symbol(sym_keyword_name); lexer.mark_end();
                if lookahead == 83 || lookahead == 115 { state = 764; lexer.advance(false); continue; }
                return result;
            }
            520 => {
                if lookahead == 82 || lookahead == 114 { state = 765; lexer.advance(false); continue; }
                return result;
            }
            521 => {
                if lookahead == 82 || lookahead == 114 { state = 766; lexer.advance(false); continue; }
                return result;
            }
            522 => {
                result = true; lexer.set_result_symbol(sym_keyword_none); lexer.mark_end();
                return result;
            }
            523 => {
                if lookahead == 65 || lookahead == 97 { state = 767; lexer.advance(false); continue; }
                return result;
            }
            524 => {
                if lookahead == 73 || lookahead == 105 { state = 768; lexer.advance(false); continue; }
                return result;
            }
            525 => {
                if lookahead == 73 || lookahead == 105 { state = 769; lexer.advance(false); continue; }
                return result;
            }
            526 => {
                result = true; lexer.set_result_symbol(sym_keyword_null); lexer.mark_end();
                if lookahead == 83 || lookahead == 115 { state = 770; lexer.advance(false); continue; }
                return result;
            }
            527 => {
                if lookahead == 82 || lookahead == 114 { state = 771; lexer.advance(false); continue; }
                return result;
            }
            528 => {
                if lookahead == 67 || lookahead == 99 { state = 772; lexer.advance(false); continue; }
                return result;
            }
            529 => {
                if lookahead == 67 || lookahead == 99 { state = 773; lexer.advance(false); continue; }
                return result;
            }
            530 => {
                if lookahead == 69 || lookahead == 101 { state = 774; lexer.advance(false); continue; }
                return result;
            }
            531 => {
                result = true; lexer.set_result_symbol(sym_keyword_oids); lexer.mark_end();
                return result;
            }
            532 => {
                result = true; lexer.set_result_symbol(sym_keyword_only); lexer.mark_end();
                return result;
            }
            533 => {
                if lookahead == 77 || lookahead == 109 { state = 775; lexer.advance(false); continue; }
                if lookahead == 79 || lookahead == 111 { state = 776; lexer.advance(false); continue; }
                return result;
            }
            534 => {
                if lookahead == 82 || lookahead == 114 { state = 777; lexer.advance(false); continue; }
                return result;
            }
            535 => {
                if lookahead == 78 || lookahead == 110 { state = 778; lexer.advance(false); continue; }
                return result;
            }
            536 => {
                if lookahead == 82 || lookahead == 114 { state = 779; lexer.advance(false); continue; }
                return result;
            }
            537 => {
                if lookahead == 82 || lookahead == 114 { state = 780; lexer.advance(false); continue; }
                return result;
            }
            538 => {
                result = true; lexer.set_result_symbol(sym_keyword_over); lexer.mark_end();
                if lookahead == 87 || lookahead == 119 { state = 781; lexer.advance(false); continue; }
                return result;
            }
            539 => {
                if lookahead == 68 || lookahead == 100 { state = 782; lexer.advance(false); continue; }
                if lookahead == 82 || lookahead == 114 { state = 783; lexer.advance(false); continue; }
                return result;
            }
            540 => {
                if lookahead == 76 || lookahead == 108 { state = 784; lexer.advance(false); continue; }
                return result;
            }
            541 => {
                if lookahead == 85 || lookahead == 117 { state = 785; lexer.advance(false); continue; }
                return result;
            }
            542 => {
                if lookahead == 73 || lookahead == 105 { state = 786; lexer.advance(false); continue; }
                return result;
            }
            543 => {
                if lookahead == 87 || lookahead == 119 { state = 787; lexer.advance(false); continue; }
                return result;
            }
            544 => {
                if lookahead == 78 || lookahead == 110 { state = 788; lexer.advance(false); continue; }
                return result;
            }
            545 => {
                if lookahead == 69 || lookahead == 101 { state = 789; lexer.advance(false); continue; }
                if lookahead == 73 || lookahead == 105 { state = 790; lexer.advance(false); continue; }
                return result;
            }
            546 => {
                if lookahead == 65 || lookahead == 97 { state = 791; lexer.advance(false); continue; }
                return result;
            }
            547 => {
                if lookahead == 69 || lookahead == 101 { state = 792; lexer.advance(false); continue; }
                return result;
            }
            548 => {
                if lookahead == 82 || lookahead == 114 { state = 793; lexer.advance(false); continue; }
                return result;
            }
            549 => {
                if lookahead == 69 || lookahead == 101 { state = 794; lexer.advance(false); continue; }
                return result;
            }
            550 => {
                if lookahead == 69 || lookahead == 101 { state = 795; lexer.advance(false); continue; }
                return result;
            }
            551 => {
                if lookahead == 76 || lookahead == 108 { state = 796; lexer.advance(false); continue; }
                return result;
            }
            552 => {
                result = true; lexer.set_result_symbol(sym_keyword_read); lexer.mark_end();
                return result;
            }
            553 => {
                result = true; lexer.set_result_symbol(aux_sym_keyword_real_token1); lexer.mark_end();
                return result;
            }
            554 => {
                if lookahead == 82 || lookahead == 114 { state = 797; lexer.advance(false); continue; }
                return result;
            }
            555 => {
                if lookahead == 82 || lookahead == 114 { state = 798; lexer.advance(false); continue; }
                return result;
            }
            556 => {
                if lookahead == 76 || lookahead == 108 { state = 799; lexer.advance(false); continue; }
                return result;
            }
            557 => {
                if lookahead == 65 || lookahead == 97 { state = 800; lexer.advance(false); continue; }
                return result;
            }
            558 => {
                if lookahead == 82 || lookahead == 114 { state = 801; lexer.advance(false); continue; }
                return result;
            }
            559 => {
                if lookahead == 89 || lookahead == 121 { state = 802; lexer.advance(false); continue; }
                return result;
            }
            560 => {
                if lookahead == 77 || lookahead == 109 { state = 803; lexer.advance(false); continue; }
                return result;
            }
            561 => {
                if lookahead == 65 || lookahead == 97 { state = 804; lexer.advance(false); continue; }
                return result;
            }
            562 => {
                if lookahead == 65 || lookahead == 97 { state = 805; lexer.advance(false); continue; }
                if lookahead == 73 || lookahead == 105 { state = 806; lexer.advance(false); continue; }
                return result;
            }
            563 => {
                if lookahead == 84 || lookahead == 116 { state = 807; lexer.advance(false); continue; }
                return result;
            }
            564 => {
                if lookahead == 65 || lookahead == 97 { state = 808; lexer.advance(false); continue; }
                if lookahead == 82 || lookahead == 114 { state = 809; lexer.advance(false); continue; }
                return result;
            }
            565 => {
                if lookahead == 82 || lookahead == 114 { state = 810; lexer.advance(false); continue; }
                return result;
            }
            566 => {
                if lookahead == 73 || lookahead == 105 { state = 811; lexer.advance(false); continue; }
                return result;
            }
            567 => {
                if lookahead == 84 || lookahead == 116 { state = 812; lexer.advance(false); continue; }
                return result;
            }
            568 => {
                result = true; lexer.set_result_symbol(sym_keyword_role); lexer.mark_end();
                return result;
            }
            569 => {
                if lookahead == 66 || lookahead == 98 { state = 813; lexer.advance(false); continue; }
                return result;
            }
            570 => {
                result = true; lexer.set_result_symbol(sym_keyword_rows); lexer.mark_end();
                return result;
            }
            571 => {
                result = true; lexer.set_result_symbol(sym_keyword_safe); lexer.mark_end();
                return result;
            }
            572 => {
                if lookahead == 77 || lookahead == 109 { state = 814; lexer.advance(false); continue; }
                return result;
            }
            573 => {
                if lookahead == 82 || lookahead == 114 { state = 815; lexer.advance(false); continue; }
                return result;
            }
            574 => {
                if lookahead == 67 || lookahead == 99 { state = 816; lexer.advance(false); continue; }
                return result;
            }
            575 => {
                if lookahead == 82 || lookahead == 114 { state = 817; lexer.advance(false); continue; }
                return result;
            }
            576 => {
                if lookahead == 69 || lookahead == 101 { state = 818; lexer.advance(false); continue; }
                return result;
            }
            577 => {
                if lookahead == 65 || lookahead == 97 { state = 819; lexer.advance(false); continue; }
                return result;
            }
            578 => {
                if lookahead == 73 || lookahead == 105 { state = 820; lexer.advance(false); continue; }
                return result;
            }
            579 => {
                if lookahead == 70 || lookahead == 102 { state = 821; lexer.advance(false); continue; }
                return result;
            }
            580 => {
                result = true; lexer.set_result_symbol(sym_keyword_show); lexer.mark_end();
                return result;
            }
            581 => {
                if lookahead == 76 || lookahead == 108 { state = 822; lexer.advance(false); continue; }
                return result;
            }
            582 => {
                if lookahead == 76 || lookahead == 108 { state = 823; lexer.advance(false); continue; }
                return result;
            }
            583 => {
                if lookahead == 83 || lookahead == 115 { state = 824; lexer.advance(false); continue; }
                return result;
            }
            584 => {
                result = true; lexer.set_result_symbol(sym_keyword_some); lexer.mark_end();
                return result;
            }
            585 => {
                result = true; lexer.set_result_symbol(sym_keyword_sort); lexer.mark_end();
                return result;
            }
            586 => {
                if lookahead == 83 || lookahead == 115 { state = 825; lexer.advance(false); continue; }
                return result;
            }
            587 => {
                if lookahead == 76 || lookahead == 108 { state = 826; lexer.advance(false); continue; }
                return result;
            }
            588 => {
                if lookahead == 84 || lookahead == 116 { state = 827; lexer.advance(false); continue; }
                return result;
            }
            589 => {
                if lookahead == 69 || lookahead == 101 { state = 828; lexer.advance(false); continue; }
                if lookahead == 73 || lookahead == 105 { state = 829; lexer.advance(false); continue; }
                if lookahead == 83 || lookahead == 115 { state = 830; lexer.advance(false); continue; }
                return result;
            }
            590 => {
                if lookahead == 78 || lookahead == 110 { state = 831; lexer.advance(false); continue; }
                return result;
            }
            591 => {
                if lookahead == 65 || lookahead == 97 { state = 832; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 833; lexer.advance(false); continue; }
                return result;
            }
            592 => {
                if lookahead == 67 || lookahead == 99 { state = 834; lexer.advance(false); continue; }
                if lookahead == 78 || lookahead == 110 { state = 835; lexer.advance(false); continue; }
                return result;
            }
            593 => {
                if lookahead == 79 || lookahead == 111 { state = 836; lexer.advance(false); continue; }
                return result;
            }
            594 => {
                if lookahead == 69 || lookahead == 101 { state = 837; lexer.advance(false); continue; }
                return result;
            }
            595 => {
                if lookahead == 82 || lookahead == 114 { state = 838; lexer.advance(false); continue; }
                return result;
            }
            596 => {
                result = true; lexer.set_result_symbol(sym_keyword_temp); lexer.mark_end();
                if lookahead == 79 || lookahead == 111 { state = 839; lexer.advance(false); continue; }
                return result;
            }
            597 => {
                if lookahead == 73 || lookahead == 105 { state = 840; lexer.advance(false); continue; }
                return result;
            }
            598 => {
                result = true; lexer.set_result_symbol(sym_keyword_text); lexer.mark_end();
                if lookahead == 70 || lookahead == 102 { state = 841; lexer.advance(false); continue; }
                return result;
            }
            599 => {
                result = true; lexer.set_result_symbol(sym_keyword_then); lexer.mark_end();
                return result;
            }
            600 => {
                result = true; lexer.set_result_symbol(sym_keyword_ties); lexer.mark_end();
                return result;
            }
            601 => {
                result = true; lexer.set_result_symbol(sym_keyword_time); lexer.mark_end();
                if lookahead == 83 || lookahead == 115 { state = 842; lexer.advance(false); continue; }
                return result;
            }
            602 => {
                if lookahead == 73 || lookahead == 105 { state = 843; lexer.advance(false); continue; }
                return result;
            }
            603 => {
                if lookahead == 83 || lookahead == 115 { state = 844; lexer.advance(false); continue; }
                return result;
            }
            604 => {
                if lookahead == 71 || lookahead == 103 { state = 845; lexer.advance(false); continue; }
                return result;
            }
            605 => {
                result = true; lexer.set_result_symbol(sym_keyword_true); lexer.mark_end();
                return result;
            }
            606 => {
                if lookahead == 67 || lookahead == 99 { state = 846; lexer.advance(false); continue; }
                return result;
            }
            607 => {
                result = true; lexer.set_result_symbol(sym_keyword_type); lexer.mark_end();
                return result;
            }
            608 => {
                if lookahead == 85 || lookahead == 117 { state = 847; lexer.advance(false); continue; }
                return result;
            }
            609 => {
                if lookahead == 67 || lookahead == 99 { state = 848; lexer.advance(false); continue; }
                return result;
            }
            610 => {
                if lookahead == 77 || lookahead == 109 { state = 849; lexer.advance(false); continue; }
                return result;
            }
            611 => {
                if lookahead == 78 || lookahead == 110 { state = 850; lexer.advance(false); continue; }
                return result;
            }
            612 => {
                if lookahead == 85 || lookahead == 117 { state = 851; lexer.advance(false); continue; }
                return result;
            }
            613 => {
                if lookahead == 65 || lookahead == 97 { state = 852; lexer.advance(false); continue; }
                if lookahead == 71 || lookahead == 103 { state = 853; lexer.advance(false); continue; }
                return result;
            }
            614 => {
                if lookahead == 70 || lookahead == 102 { state = 854; lexer.advance(false); continue; }
                return result;
            }
            615 => {
                if lookahead == 71 || lookahead == 103 { state = 855; lexer.advance(false); continue; }
                return result;
            }
            616 => {
                if lookahead == 76 || lookahead == 108 { state = 856; lexer.advance(false); continue; }
                return result;
            }
            617 => {
                if lookahead == 84 || lookahead == 116 { state = 857; lexer.advance(false); continue; }
                return result;
            }
            618 => {
                result = true; lexer.set_result_symbol(sym_keyword_user); lexer.mark_end();
                return result;
            }
            619 => {
                if lookahead == 71 || lookahead == 103 { state = 858; lexer.advance(false); continue; }
                return result;
            }
            620 => {
                result = true; lexer.set_result_symbol(sym_keyword_uuid); lexer.mark_end();
                return result;
            }
            621 => {
                if lookahead == 85 || lookahead == 117 { state = 859; lexer.advance(false); continue; }
                return result;
            }
            622 => {
                if lookahead == 68 || lookahead == 100 { state = 860; lexer.advance(false); continue; }
                return result;
            }
            623 => {
                if lookahead == 69 || lookahead == 101 { state = 861; lexer.advance(false); continue; }
                return result;
            }
            624 => {
                if lookahead == 73 || lookahead == 105 { state = 862; lexer.advance(false); continue; }
                return result;
            }
            625 => {
                if lookahead == 72 || lookahead == 104 { state = 863; lexer.advance(false); continue; }
                return result;
            }
            626 => {
                if lookahead == 65 || lookahead == 97 { state = 864; lexer.advance(false); continue; }
                return result;
            }
            627 => {
                if lookahead == 73 || lookahead == 105 { state = 865; lexer.advance(false); continue; }
                return result;
            }
            628 => {
                if lookahead == 79 || lookahead == 111 { state = 866; lexer.advance(false); continue; }
                return result;
            }
            629 => {
                if lookahead == 73 || lookahead == 105 { state = 867; lexer.advance(false); continue; }
                return result;
            }
            630 => {
                result = true; lexer.set_result_symbol(sym_keyword_view); lexer.mark_end();
                return result;
            }
            631 => {
                if lookahead == 85 || lookahead == 117 { state = 868; lexer.advance(false); continue; }
                return result;
            }
            632 => {
                if lookahead == 84 || lookahead == 116 { state = 869; lexer.advance(false); continue; }
                return result;
            }
            633 => {
                result = true; lexer.set_result_symbol(sym_keyword_wait); lexer.mark_end();
                return result;
            }
            634 => {
                result = true; lexer.set_result_symbol(sym_keyword_when); lexer.mark_end();
                return result;
            }
            635 => {
                if lookahead == 69 || lookahead == 101 { state = 870; lexer.advance(false); continue; }
                return result;
            }
            636 => {
                if lookahead == 79 || lookahead == 111 { state = 871; lexer.advance(false); continue; }
                return result;
            }
            637 => {
                result = true; lexer.set_result_symbol(sym_keyword_with); lexer.mark_end();
                if lookahead == 79 || lookahead == 111 { state = 872; lexer.advance(false); continue; }
                return result;
            }
            638 => {
                if lookahead == 69 || lookahead == 101 { state = 873; lexer.advance(false); continue; }
                return result;
            }
            639 => {
                if lookahead == 70 || lookahead == 102 { state = 874; lexer.advance(false); continue; }
                return result;
            }
            640 => {
                result = true; lexer.set_result_symbol(sym_keyword_zone); lexer.mark_end();
                return result;
            }
            641 => {
                if lookahead == 78 || lookahead == 110 { state = 875; lexer.advance(false); continue; }
                return result;
            }
            642 => {
                result = true; lexer.set_result_symbol(sym_keyword_admin); lexer.mark_end();
                return result;
            }
            643 => {
                result = true; lexer.set_result_symbol(sym_keyword_after); lexer.mark_end();
                return result;
            }
            644 => {
                result = true; lexer.set_result_symbol(sym_keyword_alter); lexer.mark_end();
                return result;
            }
            645 => {
                if lookahead == 83 || lookahead == 115 { state = 876; lexer.advance(false); continue; }
                return result;
            }
            646 => {
                if lookahead == 90 || lookahead == 122 { state = 877; lexer.advance(false); continue; }
                return result;
            }
            647 => {
                result = true; lexer.set_result_symbol(sym_keyword_array); lexer.mark_end();
                return result;
            }
            648 => {
                if lookahead == 67 || lookahead == 99 { state = 878; lexer.advance(false); continue; }
                return result;
            }
            649 => {
                if lookahead == 66 || lookahead == 98 { state = 879; lexer.advance(false); continue; }
                return result;
            }
            650 => {
                if lookahead == 82 || lookahead == 114 { state = 880; lexer.advance(false); continue; }
                return result;
            }
            651 => {
                if lookahead == 73 || lookahead == 105 { state = 881; lexer.advance(false); continue; }
                return result;
            }
            652 => {
                if lookahead == 69 || lookahead == 101 { state = 882; lexer.advance(false); continue; }
                return result;
            }
            653 => {
                result = true; lexer.set_result_symbol(sym_keyword_begin); lexer.mark_end();
                return result;
            }
            654 => {
                if lookahead == 69 || lookahead == 101 { state = 883; lexer.advance(false); continue; }
                return result;
            }
            655 => {
                if lookahead == 84 || lookahead == 116 { state = 884; lexer.advance(false); continue; }
                return result;
            }
            656 => {
                if lookahead == 82 || lookahead == 114 { state = 885; lexer.advance(false); continue; }
                return result;
            }
            657 => {
                if lookahead == 65 || lookahead == 97 { state = 886; lexer.advance(false); continue; }
                return result;
            }
            658 => {
                if lookahead == 89 || lookahead == 121 { state = 887; lexer.advance(false); continue; }
                return result;
            }
            659 => {
                if lookahead == 65 || lookahead == 97 { state = 888; lexer.advance(false); continue; }
                return result;
            }
            660 => {
                result = true; lexer.set_result_symbol(sym_keyword_box2d); lexer.mark_end();
                return result;
            }
            661 => {
                result = true; lexer.set_result_symbol(sym_keyword_box3d); lexer.mark_end();
                return result;
            }
            662 => {
                result = true; lexer.set_result_symbol(sym_keyword_btree); lexer.mark_end();
                return result;
            }
            663 => {
                result = true; lexer.set_result_symbol(sym_keyword_bytea); lexer.mark_end();
                return result;
            }
            664 => {
                result = true; lexer.set_result_symbol(sym_keyword_cache); lexer.mark_end();
                if lookahead == 68 || lookahead == 100 { state = 889; lexer.advance(false); continue; }
                return result;
            }
            665 => {
                if lookahead == 68 || lookahead == 100 { state = 890; lexer.advance(false); continue; }
                return result;
            }
            666 => {
                if lookahead == 68 || lookahead == 100 { state = 891; lexer.advance(false); continue; }
                return result;
            }
            667 => {
                if lookahead == 69 || lookahead == 101 { state = 892; lexer.advance(false); continue; }
                return result;
            }
            668 => {
                if lookahead == 67 || lookahead == 99 { state = 893; lexer.advance(false); continue; }
                return result;
            }
            669 => {
                result = true; lexer.set_result_symbol(sym_keyword_check); lexer.mark_end();
                return result;
            }
            670 => {
                if lookahead == 84 || lookahead == 116 { state = 894; lexer.advance(false); continue; }
                return result;
            }
            671 => {
                if lookahead == 78 || lookahead == 110 { state = 895; lexer.advance(false); continue; }
                return result;
            }
            672 => {
                if lookahead == 78 || lookahead == 110 { state = 896; lexer.advance(false); continue; }
                return result;
            }
            673 => {
                if lookahead == 84 || lookahead == 116 { state = 897; lexer.advance(false); continue; }
                return result;
            }
            674 => {
                if lookahead == 69 || lookahead == 101 { state = 898; lexer.advance(false); continue; }
                return result;
            }
            675 => {
                if lookahead == 84 || lookahead == 116 { state = 899; lexer.advance(false); continue; }
                return result;
            }
            676 => {
                if lookahead == 82 || lookahead == 114 { state = 900; lexer.advance(false); continue; }
                return result;
            }
            677 => {
                if lookahead == 73 || lookahead == 105 { state = 901; lexer.advance(false); continue; }
                return result;
            }
            678 => {
                if lookahead == 67 || lookahead == 99 { state = 902; lexer.advance(false); continue; }
                return result;
            }
            679 => {
                if lookahead == 82 || lookahead == 114 { state = 903; lexer.advance(false); continue; }
                return result;
            }
            680 => {
                if lookahead == 69 || lookahead == 101 { state = 904; lexer.advance(false); continue; }
                return result;
            }
            681 => {
                result = true; lexer.set_result_symbol(sym_keyword_cross); lexer.mark_end();
                return result;
            }
            682 => {
                if lookahead == 78 || lookahead == 110 { state = 905; lexer.advance(false); continue; }
                return result;
            }
            683 => {
                result = true; lexer.set_result_symbol(sym_keyword_cycle); lexer.mark_end();
                return result;
            }
            684 => {
                if lookahead == 65 || lookahead == 97 { state = 906; lexer.advance(false); continue; }
                return result;
            }
            685 => {
                if lookahead == 73 || lookahead == 105 { state = 907; lexer.advance(false); continue; }
                return result;
            }
            686 => {
                if lookahead == 65 || lookahead == 97 { state = 908; lexer.advance(false); continue; }
                return result;
            }
            687 => {
                if lookahead == 82 || lookahead == 114 { state = 909; lexer.advance(false); continue; }
                return result;
            }
            688 => {
                if lookahead == 76 || lookahead == 108 { state = 910; lexer.advance(false); continue; }
                return result;
            }
            689 => {
                if lookahead == 82 || lookahead == 114 { state = 911; lexer.advance(false); continue; }
                return result;
            }
            690 => {
                if lookahead == 69 || lookahead == 101 { state = 912; lexer.advance(false); continue; }
                return result;
            }
            691 => {
                if lookahead == 69 || lookahead == 101 { state = 913; lexer.advance(false); continue; }
                return result;
            }
            692 => {
                if lookahead == 69 || lookahead == 101 { state = 914; lexer.advance(false); continue; }
                return result;
            }
            693 => {
                if lookahead == 73 || lookahead == 105 { state = 915; lexer.advance(false); continue; }
                return result;
            }
            694 => {
                if lookahead == 78 || lookahead == 110 { state = 916; lexer.advance(false); continue; }
                return result;
            }
            695 => {
                if lookahead == 69 || lookahead == 101 { state = 917; lexer.advance(false); continue; }
                return result;
            }
            696 => {
                if lookahead == 67 || lookahead == 99 { state = 918; lexer.advance(false); continue; }
                return result;
            }
            697 => {
                if lookahead == 73 || lookahead == 105 { state = 919; lexer.advance(false); continue; }
                return result;
            }
            698 => {
                if lookahead == 80 || lookahead == 112 { state = 920; lexer.advance(false); continue; }
                return result;
            }
            699 => {
                if lookahead == 69 || lookahead == 101 { state = 921; lexer.advance(false); continue; }
                return result;
            }
            700 => {
                if lookahead == 69 || lookahead == 101 { state = 922; lexer.advance(false); continue; }
                return result;
            }
            701 => {
                if lookahead == 84 || lookahead == 116 { state = 923; lexer.advance(false); continue; }
                return result;
            }
            702 => {
                if lookahead == 68 || lookahead == 100 { state = 924; lexer.advance(false); continue; }
                return result;
            }
            703 => {
                if lookahead == 84 || lookahead == 116 { state = 925; lexer.advance(false); continue; }
                return result;
            }
            704 => {
                if lookahead == 83 || lookahead == 115 { state = 926; lexer.advance(false); continue; }
                return result;
            }
            705 => {
                if lookahead == 73 || lookahead == 105 { state = 927; lexer.advance(false); continue; }
                return result;
            }
            706 => {
                if lookahead == 68 || lookahead == 100 { state = 928; lexer.advance(false); continue; }
                if lookahead == 83 || lookahead == 115 { state = 929; lexer.advance(false); continue; }
                return result;
            }
            707 => {
                if lookahead == 78 || lookahead == 110 { state = 930; lexer.advance(false); continue; }
                return result;
            }
            708 => {
                result = true; lexer.set_result_symbol(sym_keyword_false); lexer.mark_end();
                return result;
            }
            709 => {
                if lookahead == 83 || lookahead == 115 { state = 931; lexer.advance(false); continue; }
                return result;
            }
            710 => {
                if lookahead == 82 || lookahead == 114 { state = 932; lexer.advance(false); continue; }
                return result;
            }
            711 => {
                result = true; lexer.set_result_symbol(sym_keyword_first); lexer.mark_end();
                return result;
            }
            712 => {
                result = true; lexer.set_result_symbol(sym_keyword_float); lexer.mark_end();
                if lookahead == 52 { state = 933; lexer.advance(false); continue; }
                if lookahead == 56 { state = 934; lexer.advance(false); continue; }
                return result;
            }
            713 => {
                if lookahead == 87 || lookahead == 119 { state = 935; lexer.advance(false); continue; }
                return result;
            }
            714 => {
                result = true; lexer.set_result_symbol(sym_keyword_force); lexer.mark_end();
                if lookahead == 95 { state = 936; lexer.advance(false); continue; }
                return result;
            }
            715 => {
                if lookahead == 71 || lookahead == 103 { state = 937; lexer.advance(false); continue; }
                return result;
            }
            716 => {
                if lookahead == 84 || lookahead == 116 { state = 938; lexer.advance(false); continue; }
                return result;
            }
            717 => {
                if lookahead == 69 || lookahead == 101 { state = 939; lexer.advance(false); continue; }
                return result;
            }
            718 => {
                if lookahead == 73 || lookahead == 105 { state = 940; lexer.advance(false); continue; }
                return result;
            }
            719 => {
                if lookahead == 65 || lookahead == 97 { state = 941; lexer.advance(false); continue; }
                return result;
            }
            720 => {
                if lookahead == 65 || lookahead == 97 { state = 942; lexer.advance(false); continue; }
                return result;
            }
            721 => {
                if lookahead == 84 || lookahead == 116 { state = 943; lexer.advance(false); continue; }
                return result;
            }
            722 => {
                result = true; lexer.set_result_symbol(sym_keyword_group); lexer.mark_end();
                if lookahead == 83 || lookahead == 115 { state = 944; lexer.advance(false); continue; }
                return result;
            }
            723 => {
                if lookahead == 71 || lookahead == 103 { state = 945; lexer.advance(false); continue; }
                return result;
            }
            724 => {
                if lookahead == 82 || lookahead == 114 { state = 946; lexer.advance(false); continue; }
                return result;
            }
            725 => {
                if lookahead == 80 || lookahead == 112 { state = 947; lexer.advance(false); continue; }
                return result;
            }
            726 => {
                if lookahead == 69 || lookahead == 101 { state = 948; lexer.advance(false); continue; }
                return result;
            }
            727 => {
                result = true; lexer.set_result_symbol(aux_sym_keyword_like_token2); lexer.mark_end();
                return result;
            }
            728 => {
                result = true; lexer.set_result_symbol(sym_keyword_image); lexer.mark_end();
                return result;
            }
            729 => {
                if lookahead == 73 || lookahead == 105 { state = 949; lexer.advance(false); continue; }
                return result;
            }
            730 => {
                if lookahead == 65 || lookahead == 97 { state = 950; lexer.advance(false); continue; }
                return result;
            }
            731 => {
                if lookahead == 77 || lookahead == 109 { state = 951; lexer.advance(false); continue; }
                return result;
            }
            732 => {
                result = true; lexer.set_result_symbol(sym_keyword_index); lexer.mark_end();
                return result;
            }
            733 => {
                if lookahead == 65 || lookahead == 97 { state = 952; lexer.advance(false); continue; }
                return result;
            }
            734 => {
                result = true; lexer.set_result_symbol(sym_keyword_inner); lexer.mark_end();
                return result;
            }
            735 => {
                result = true; lexer.set_result_symbol(sym_keyword_inout); lexer.mark_end();
                return result;
            }
            736 => {
                result = true; lexer.set_result_symbol(sym_keyword_input); lexer.mark_end();
                return result;
            }
            737 => {
                if lookahead == 84 || lookahead == 116 { state = 953; lexer.advance(false); continue; }
                return result;
            }
            738 => {
                if lookahead == 65 || lookahead == 97 { state = 954; lexer.advance(false); continue; }
                return result;
            }
            739 => {
                if lookahead == 69 || lookahead == 101 { state = 955; lexer.advance(false); continue; }
                return result;
            }
            740 => {
                if lookahead == 83 || lookahead == 115 { state = 956; lexer.advance(false); continue; }
                if lookahead == 86 || lookahead == 118 { state = 957; lexer.advance(false); continue; }
                return result;
            }
            741 => {
                if lookahead == 69 || lookahead == 101 { state = 958; lexer.advance(false); continue; }
                return result;
            }
            742 => {
                if lookahead == 84 || lookahead == 116 { state = 959; lexer.advance(false); continue; }
                return result;
            }
            743 => {
                result = true; lexer.set_result_symbol(sym_keyword_jsonb); lexer.mark_end();
                return result;
            }
            744 => {
                if lookahead == 73 || lookahead == 105 { state = 960; lexer.advance(false); continue; }
                return result;
            }
            745 => {
                if lookahead == 65 || lookahead == 97 { state = 961; lexer.advance(false); continue; }
                return result;
            }
            746 => {
                if lookahead == 65 || lookahead == 97 { state = 962; lexer.advance(false); continue; }
                return result;
            }
            747 => {
                if lookahead == 82 || lookahead == 114 { state = 963; lexer.advance(false); continue; }
                return result;
            }
            748 => {
                result = true; lexer.set_result_symbol(sym_keyword_level); lexer.mark_end();
                return result;
            }
            749 => {
                result = true; lexer.set_result_symbol(sym_keyword_limit); lexer.mark_end();
                return result;
            }
            750 => {
                result = true; lexer.set_result_symbol(sym_keyword_lines); lexer.mark_end();
                return result;
            }
            751 => {
                result = true; lexer.set_result_symbol(sym_keyword_local); lexer.mark_end();
                return result;
            }
            752 => {
                if lookahead == 73 || lookahead == 105 { state = 964; lexer.advance(false); continue; }
                return result;
            }
            753 => {
                if lookahead == 68 || lookahead == 100 { state = 965; lexer.advance(false); continue; }
                return result;
            }
            754 => {
                if lookahead == 82 || lookahead == 114 { state = 966; lexer.advance(false); continue; }
                return result;
            }
            755 => {
                result = true; lexer.set_result_symbol(sym_keyword_match); lexer.mark_end();
                if lookahead == 69 || lookahead == 101 { state = 967; lexer.advance(false); continue; }
                return result;
            }
            756 => {
                if lookahead == 73 || lookahead == 105 { state = 968; lexer.advance(false); continue; }
                return result;
            }
            757 => {
                if lookahead == 76 || lookahead == 108 { state = 969; lexer.advance(false); continue; }
                return result;
            }
            758 => {
                if lookahead == 77 || lookahead == 109 { state = 970; lexer.advance(false); continue; }
                return result;
            }
            759 => {
                result = true; lexer.set_result_symbol(sym_keyword_merge); lexer.mark_end();
                return result;
            }
            760 => {
                if lookahead == 65 || lookahead == 97 { state = 971; lexer.advance(false); continue; }
                return result;
            }
            761 => {
                if lookahead == 76 || lookahead == 108 { state = 972; lexer.advance(false); continue; }
                return result;
            }
            762 => {
                if lookahead == 89 || lookahead == 121 { state = 973; lexer.advance(false); continue; }
                return result;
            }
            763 => {
                result = true; lexer.set_result_symbol(sym_keyword_money); lexer.mark_end();
                return result;
            }
            764 => {
                result = true; lexer.set_result_symbol(sym_keyword_names); lexer.mark_end();
                return result;
            }
            765 => {
                if lookahead == 65 || lookahead == 97 { state = 974; lexer.advance(false); continue; }
                return result;
            }
            766 => {
                result = true; lexer.set_result_symbol(sym_keyword_nchar); lexer.mark_end();
                return result;
            }
            767 => {
                if lookahead == 78 || lookahead == 110 { state = 975; lexer.advance(false); continue; }
                return result;
            }
            768 => {
                if lookahead == 78 || lookahead == 110 { state = 976; lexer.advance(false); continue; }
                return result;
            }
            769 => {
                if lookahead == 84 || lookahead == 116 { state = 977; lexer.advance(false); continue; }
                return result;
            }
            770 => {
                result = true; lexer.set_result_symbol(sym_keyword_nulls); lexer.mark_end();
                return result;
            }
            771 => {
                if lookahead == 73 || lookahead == 105 { state = 978; lexer.advance(false); continue; }
                return result;
            }
            772 => {
                if lookahead == 72 || lookahead == 104 { state = 979; lexer.advance(false); continue; }
                return result;
            }
            773 => {
                if lookahead == 84 || lookahead == 116 { state = 980; lexer.advance(false); continue; }
                return result;
            }
            774 => {
                if lookahead == 84 || lookahead == 116 { state = 981; lexer.advance(false); continue; }
                return result;
            }
            775 => {
                if lookahead == 73 || lookahead == 105 { state = 982; lexer.advance(false); continue; }
                return result;
            }
            776 => {
                if lookahead == 78 || lookahead == 110 { state = 983; lexer.advance(false); continue; }
                return result;
            }
            777 => {
                result = true; lexer.set_result_symbol(sym_keyword_order); lexer.mark_end();
                return result;
            }
            778 => {
                if lookahead == 65 || lookahead == 97 { state = 984; lexer.advance(false); continue; }
                return result;
            }
            779 => {
                if lookahead == 83 || lookahead == 115 { state = 985; lexer.advance(false); continue; }
                return result;
            }
            780 => {
                result = true; lexer.set_result_symbol(sym_keyword_outer); lexer.mark_end();
                return result;
            }
            781 => {
                if lookahead == 82 || lookahead == 114 { state = 986; lexer.advance(false); continue; }
                return result;
            }
            782 => {
                result = true; lexer.set_result_symbol(sym_keyword_owned); lexer.mark_end();
                return result;
            }
            783 => {
                result = true; lexer.set_result_symbol(sym_keyword_owner); lexer.mark_end();
                return result;
            }
            784 => {
                if lookahead == 76 || lookahead == 108 { state = 987; lexer.advance(false); continue; }
                return result;
            }
            785 => {
                if lookahead == 69 || lookahead == 101 { state = 988; lexer.advance(false); continue; }
                return result;
            }
            786 => {
                if lookahead == 84 || lookahead == 116 { state = 989; lexer.advance(false); continue; }
                return result;
            }
            787 => {
                if lookahead == 79 || lookahead == 111 { state = 990; lexer.advance(false); continue; }
                return result;
            }
            788 => {
                result = true; lexer.set_result_symbol(sym_keyword_plain); lexer.mark_end();
                return result;
            }
            789 => {
                if lookahead == 68 || lookahead == 100 { state = 991; lexer.advance(false); continue; }
                return result;
            }
            790 => {
                if lookahead == 83 || lookahead == 115 { state = 992; lexer.advance(false); continue; }
                return result;
            }
            791 => {
                if lookahead == 82 || lookahead == 114 { state = 993; lexer.advance(false); continue; }
                return result;
            }
            792 => {
                if lookahead == 68 || lookahead == 100 { state = 994; lexer.advance(false); continue; }
                return result;
            }
            793 => {
                if lookahead == 65 || lookahead == 97 { state = 995; lexer.advance(false); continue; }
                return result;
            }
            794 => {
                result = true; lexer.set_result_symbol(sym_keyword_quote); lexer.mark_end();
                return result;
            }
            795 => {
                result = true; lexer.set_result_symbol(sym_keyword_range); lexer.mark_end();
                return result;
            }
            796 => {
                if lookahead == 69 || lookahead == 101 { state = 996; lexer.advance(false); continue; }
                return result;
            }
            797 => {
                if lookahead == 83 || lookahead == 115 { state = 997; lexer.advance(false); continue; }
                return result;
            }
            798 => {
                if lookahead == 69 || lookahead == 101 { state = 998; lexer.advance(false); continue; }
                return result;
            }
            799 => {
                if lookahead == 65 || lookahead == 97 { state = 999; lexer.advance(false); continue; }
                return result;
            }
            800 => {
                if lookahead == 77 || lookahead == 109 { state = 1000; lexer.advance(false); continue; }
                return result;
            }
            801 => {
                if lookahead == 79 || lookahead == 111 { state = 1001; lexer.advance(false); continue; }
                return result;
            }
            802 => {
                if lookahead == 80 || lookahead == 112 { state = 1002; lexer.advance(false); continue; }
                return result;
            }
            803 => {
                if lookahead == 69 || lookahead == 101 { state = 1003; lexer.advance(false); continue; }
                return result;
            }
            804 => {
                if lookahead == 84 || lookahead == 116 { state = 1004; lexer.advance(false); continue; }
                return result;
            }
            805 => {
                if lookahead == 67 || lookahead == 99 { state = 1005; lexer.advance(false); continue; }
                return result;
            }
            806 => {
                if lookahead == 67 || lookahead == 99 { state = 1006; lexer.advance(false); continue; }
                return result;
            }
            807 => {
                result = true; lexer.set_result_symbol(sym_keyword_reset); lexer.mark_end();
                return result;
            }
            808 => {
                if lookahead == 82 || lookahead == 114 { state = 1007; lexer.advance(false); continue; }
                return result;
            }
            809 => {
                if lookahead == 73 || lookahead == 105 { state = 1008; lexer.advance(false); continue; }
                return result;
            }
            810 => {
                if lookahead == 78 || lookahead == 110 { state = 1009; lexer.advance(false); continue; }
                return result;
            }
            811 => {
                if lookahead == 84 || lookahead == 116 { state = 1010; lexer.advance(false); continue; }
                return result;
            }
            812 => {
                result = true; lexer.set_result_symbol(sym_keyword_right); lexer.mark_end();
                return result;
            }
            813 => {
                if lookahead == 65 || lookahead == 97 { state = 1011; lexer.advance(false); continue; }
                return result;
            }
            814 => {
                if lookahead == 65 || lookahead == 97 { state = 1012; lexer.advance(false); continue; }
                return result;
            }
            815 => {
                if lookahead == 73 || lookahead == 105 { state = 1013; lexer.advance(false); continue; }
                return result;
            }
            816 => {
                if lookahead == 84 || lookahead == 116 { state = 1014; lexer.advance(false); continue; }
                return result;
            }
            817 => {
                if lookahead == 65 || lookahead == 97 { state = 1015; lexer.advance(false); continue; }
                return result;
            }
            818 => {
                if lookahead == 78 || lookahead == 110 { state = 1016; lexer.advance(false); continue; }
                return result;
            }
            819 => {
                if lookahead == 76 || lookahead == 108 { state = 1017; lexer.advance(false); continue; }
                return result;
            }
            820 => {
                if lookahead == 79 || lookahead == 111 { state = 1018; lexer.advance(false); continue; }
                return result;
            }
            821 => {
                result = true; lexer.set_result_symbol(sym_keyword_setof); lexer.mark_end();
                return result;
            }
            822 => {
                if lookahead == 65 || lookahead == 97 { state = 1019; lexer.advance(false); continue; }
                return result;
            }
            823 => {
                if let Some(next) = advance_map(&[
                    (68, 1020), (100, 1020), (73, 1021), (105, 1021), (77, 1022), (109, 1022), (83, 1023), (115, 1023),
                ], lookahead) { state = next; lexer.advance(false); continue; }
                return result;
            }
            824 => {
                if lookahead == 72 || lookahead == 104 { state = 1024; lexer.advance(false); continue; }
                return result;
            }
            825 => {
                if lookahead == 84 || lookahead == 116 { state = 1025; lexer.advance(false); continue; }
                return result;
            }
            826 => {
                if lookahead == 69 || lookahead == 101 { state = 1026; lexer.advance(false); continue; }
                return result;
            }
            827 => {
                result = true; lexer.set_result_symbol(sym_keyword_start); lexer.mark_end();
                return result;
            }
            828 => {
                if lookahead == 77 || lookahead == 109 { state = 1027; lexer.advance(false); continue; }
                return result;
            }
            829 => {
                if lookahead == 83 || lookahead == 115 { state = 1028; lexer.advance(false); continue; }
                return result;
            }
            830 => {
                result = true; lexer.set_result_symbol(sym_keyword_stats); lexer.mark_end();
                return result;
            }
            831 => {
                result = true; lexer.set_result_symbol(sym_keyword_stdin); lexer.mark_end();
                return result;
            }
            832 => {
                if lookahead == 71 || lookahead == 103 { state = 1029; lexer.advance(false); continue; }
                return result;
            }
            833 => {
                if lookahead == 68 || lookahead == 100 { state = 1030; lexer.advance(false); continue; }
                return result;
            }
            834 => {
                if lookahead == 84 || lookahead == 116 { state = 1031; lexer.advance(false); continue; }
                return result;
            }
            835 => {
                if lookahead == 71 || lookahead == 103 { state = 1032; lexer.advance(false); continue; }
                return result;
            }
            836 => {
                if lookahead == 82 || lookahead == 114 { state = 1033; lexer.advance(false); continue; }
                return result;
            }
            837 => {
                result = true; lexer.set_result_symbol(sym_keyword_table); lexer.mark_end();
                if lookahead == 83 || lookahead == 115 { state = 1034; lexer.advance(false); continue; }
                return result;
            }
            838 => {
                if lookahead == 79 || lookahead == 111 { state = 1035; lexer.advance(false); continue; }
                return result;
            }
            839 => {
                if lookahead == 82 || lookahead == 114 { state = 1036; lexer.advance(false); continue; }
                return result;
            }
            840 => {
                if lookahead == 78 || lookahead == 110 { state = 1037; lexer.advance(false); continue; }
                return result;
            }
            841 => {
                if lookahead == 73 || lookahead == 105 { state = 1038; lexer.advance(false); continue; }
                return result;
            }
            842 => {
                if lookahead == 84 || lookahead == 116 { state = 1039; lexer.advance(false); continue; }
                return result;
            }
            843 => {
                if lookahead == 78 || lookahead == 110 { state = 1040; lexer.advance(false); continue; }
                return result;
            }
            844 => {
                if lookahead == 65 || lookahead == 97 { state = 1041; lexer.advance(false); continue; }
                return result;
            }
            845 => {
                if lookahead == 69 || lookahead == 101 { state = 1042; lexer.advance(false); continue; }
                return result;
            }
            846 => {
                if lookahead == 65 || lookahead == 97 { state = 1043; lexer.advance(false); continue; }
                return result;
            }
            847 => {
                if lookahead == 78 || lookahead == 110 { state = 1044; lexer.advance(false); continue; }
                return result;
            }
            848 => {
                if lookahead == 72 || lookahead == 104 { state = 1045; lexer.advance(false); continue; }
                return result;
            }
            849 => {
                if lookahead == 77 || lookahead == 109 { state = 1046; lexer.advance(false); continue; }
                return result;
            }
            850 => {
                result = true; lexer.set_result_symbol(sym_keyword_union); lexer.mark_end();
                return result;
            }
            851 => {
                if lookahead == 69 || lookahead == 101 { state = 1047; lexer.advance(false); continue; }
                return result;
            }
            852 => {
                if lookahead == 68 || lookahead == 100 { state = 1048; lexer.advance(false); continue; }
                return result;
            }
            853 => {
                if lookahead == 71 || lookahead == 103 { state = 1049; lexer.advance(false); continue; }
                return result;
            }
            854 => {
                if lookahead == 69 || lookahead == 101 { state = 1050; lexer.advance(false); continue; }
                return result;
            }
            855 => {
                if lookahead == 78 || lookahead == 110 { state = 1051; lexer.advance(false); continue; }
                return result;
            }
            856 => {
                result = true; lexer.set_result_symbol(sym_keyword_until); lexer.mark_end();
                return result;
            }
            857 => {
                if lookahead == 69 || lookahead == 101 { state = 1052; lexer.advance(false); continue; }
                return result;
            }
            858 => {
                result = true; lexer.set_result_symbol(sym_keyword_using); lexer.mark_end();
                return result;
            }
            859 => {
                if lookahead == 77 || lookahead == 109 { state = 1053; lexer.advance(false); continue; }
                return result;
            }
            860 => {
                result = true; lexer.set_result_symbol(sym_keyword_valid); lexer.mark_end();
                return result;
            }
            861 => {
                result = true; lexer.set_result_symbol(sym_keyword_value); lexer.mark_end();
                if lookahead == 83 || lookahead == 115 { state = 1054; lexer.advance(false); continue; }
                return result;
            }
            862 => {
                if lookahead == 78 || lookahead == 110 { state = 1055; lexer.advance(false); continue; }
                return result;
            }
            863 => {
                if lookahead == 65 || lookahead == 97 { state = 1056; lexer.advance(false); continue; }
                return result;
            }
            864 => {
                if lookahead == 68 || lookahead == 100 { state = 1057; lexer.advance(false); continue; }
                return result;
            }
            865 => {
                if lookahead == 78 || lookahead == 110 { state = 1058; lexer.advance(false); continue; }
                return result;
            }
            866 => {
                if lookahead == 83 || lookahead == 115 { state = 1059; lexer.advance(false); continue; }
                return result;
            }
            867 => {
                if lookahead == 79 || lookahead == 111 { state = 1060; lexer.advance(false); continue; }
                return result;
            }
            868 => {
                if lookahead == 65 || lookahead == 97 { state = 1061; lexer.advance(false); continue; }
                return result;
            }
            869 => {
                if lookahead == 73 || lookahead == 105 { state = 1062; lexer.advance(false); continue; }
                return result;
            }
            870 => {
                result = true; lexer.set_result_symbol(sym_keyword_where); lexer.mark_end();
                return result;
            }
            871 => {
                if lookahead == 87 || lookahead == 119 { state = 1063; lexer.advance(false); continue; }
                return result;
            }
            872 => {
                if lookahead == 85 || lookahead == 117 { state = 1064; lexer.advance(false); continue; }
                return result;
            }
            873 => {
                result = true; lexer.set_result_symbol(sym_keyword_write); lexer.mark_end();
                return result;
            }
            874 => {
                if lookahead == 73 || lookahead == 105 { state = 1065; lexer.advance(false); continue; }
                return result;
            }
            875 => {
                result = true; lexer.set_result_symbol(sym_keyword_action); lexer.mark_end();
                return result;
            }
            876 => {
                result = true; lexer.set_result_symbol(sym_keyword_always); lexer.mark_end();
                return result;
            }
            877 => {
                if lookahead == 69 || lookahead == 101 { state = 1066; lexer.advance(false); continue; }
                return result;
            }
            878 => {
                result = true; lexer.set_result_symbol(sym_keyword_atomic); lexer.mark_end();
                return result;
            }
            879 => {
                if lookahead == 85 || lookahead == 117 { state = 1067; lexer.advance(false); continue; }
                return result;
            }
            880 => {
                if lookahead == 73 || lookahead == 105 { state = 1068; lexer.advance(false); continue; }
                return result;
            }
            881 => {
                if lookahead == 78 || lookahead == 110 { state = 1069; lexer.advance(false); continue; }
                return result;
            }
            882 => {
                result = true; lexer.set_result_symbol(sym_keyword_before); lexer.mark_end();
                return result;
            }
            883 => {
                if lookahead == 78 || lookahead == 110 { state = 1070; lexer.advance(false); continue; }
                return result;
            }
            884 => {
                result = true; lexer.set_result_symbol(aux_sym_keyword_bigint_token1); lexer.mark_end();
                return result;
            }
            885 => {
                if lookahead == 73 || lookahead == 105 { state = 1071; lexer.advance(false); continue; }
                return result;
            }
            886 => {
                if lookahead == 67 || lookahead == 99 { state = 1072; lexer.advance(false); continue; }
                return result;
            }
            887 => {
                result = true; lexer.set_result_symbol(sym_keyword_binary); lexer.mark_end();
                return result;
            }
            888 => {
                if lookahead == 78 || lookahead == 110 { state = 1073; lexer.advance(false); continue; }
                return result;
            }
            889 => {
                result = true; lexer.set_result_symbol(sym_keyword_cached); lexer.mark_end();
                return result;
            }
            890 => {
                result = true; lexer.set_result_symbol(sym_keyword_called); lexer.mark_end();
                return result;
            }
            891 => {
                if lookahead == 69 || lookahead == 101 { state = 1074; lexer.advance(false); continue; }
                return result;
            }
            892 => {
                result = true; lexer.set_result_symbol(sym_keyword_change); lexer.mark_end();
                return result;
            }
            893 => {
                if lookahead == 84 || lookahead == 116 { state = 1075; lexer.advance(false); continue; }
                return result;
            }
            894 => {
                if lookahead == 69 || lookahead == 101 { state = 1076; lexer.advance(false); continue; }
                return result;
            }
            895 => {
                result = true; lexer.set_result_symbol(sym_keyword_column); lexer.mark_end();
                if lookahead == 83 || lookahead == 115 { state = 1077; lexer.advance(false); continue; }
                return result;
            }
            896 => {
                if lookahead == 84 || lookahead == 116 { state = 1078; lexer.advance(false); continue; }
                return result;
            }
            897 => {
                result = true; lexer.set_result_symbol(sym_keyword_commit); lexer.mark_end();
                if lookahead == 84 || lookahead == 116 { state = 1079; lexer.advance(false); continue; }
                return result;
            }
            898 => {
                if lookahead == 83 || lookahead == 115 { state = 1080; lexer.advance(false); continue; }
                return result;
            }
            899 => {
                if lookahead == 69 || lookahead == 101 { state = 1081; lexer.advance(false); continue; }
                return result;
            }
            900 => {
                if lookahead == 82 || lookahead == 114 { state = 1082; lexer.advance(false); continue; }
                return result;
            }
            901 => {
                if lookahead == 67 || lookahead == 99 { state = 1083; lexer.advance(false); continue; }
                return result;
            }
            902 => {
                if lookahead == 84 || lookahead == 116 { state = 1084; lexer.advance(false); continue; }
                return result;
            }
            903 => {
                if lookahead == 65 || lookahead == 97 { state = 1085; lexer.advance(false); continue; }
                return result;
            }
            904 => {
                result = true; lexer.set_result_symbol(sym_keyword_create); lexer.mark_end();
                return result;
            }
            905 => {
                if lookahead == 84 || lookahead == 116 { state = 1086; lexer.advance(false); continue; }
                return result;
            }
            906 => {
                if lookahead == 83 || lookahead == 115 { state = 1087; lexer.advance(false); continue; }
                return result;
            }
            907 => {
                if lookahead == 77 || lookahead == 109 { state = 1088; lexer.advance(false); continue; }
                return result;
            }
            908 => {
                if lookahead == 76 || lookahead == 108 { state = 1089; lexer.advance(false); continue; }
                return result;
            }
            909 => {
                if lookahead == 69 || lookahead == 101 { state = 1090; lexer.advance(false); continue; }
                return result;
            }
            910 => {
                if lookahead == 84 || lookahead == 116 { state = 1091; lexer.advance(false); continue; }
                return result;
            }
            911 => {
                if lookahead == 65 || lookahead == 97 { state = 1092; lexer.advance(false); continue; }
                if lookahead == 69 || lookahead == 101 { state = 1093; lexer.advance(false); continue; }
                return result;
            }
            912 => {
                if lookahead == 82 || lookahead == 114 { state = 1094; lexer.advance(false); continue; }
                return result;
            }
            913 => {
                if lookahead == 68 || lookahead == 100 { state = 1095; lexer.advance(false); continue; }
                return result;
            }
            914 => {
                result = true; lexer.set_result_symbol(sym_keyword_delete); lexer.mark_end();
                return result;
            }
            915 => {
                if lookahead == 84 || lookahead == 116 { state = 1096; lexer.advance(false); continue; }
                return result;
            }
            916 => {
                if lookahead == 67 || lookahead == 99 { state = 1097; lexer.advance(false); continue; }
                return result;
            }
            917 => {
                result = true; lexer.set_result_symbol(sym_keyword_double); lexer.mark_end();
                return result;
            }
            918 => {
                if lookahead == 65 || lookahead == 97 { state = 1098; lexer.advance(false); continue; }
                return result;
            }
            919 => {
                if lookahead == 78 || lookahead == 110 { state = 1099; lexer.advance(false); continue; }
                return result;
            }
            920 => {
                if lookahead == 84 || lookahead == 116 { state = 1100; lexer.advance(false); continue; }
                return result;
            }
            921 => {
                result = true; lexer.set_result_symbol(sym_keyword_engine); lexer.mark_end();
                return result;
            }
            922 => {
                result = true; lexer.set_result_symbol(sym_keyword_escape); lexer.mark_end();
                if lookahead == 68 || lookahead == 100 { state = 1101; lexer.advance(false); continue; }
                return result;
            }
            923 => {
                result = true; lexer.set_result_symbol(sym_keyword_except); lexer.mark_end();
                return result;
            }
            924 => {
                if lookahead == 69 || lookahead == 101 { state = 1102; lexer.advance(false); continue; }
                return result;
            }
            925 => {
                if lookahead == 69 || lookahead == 101 { state = 1103; lexer.advance(false); continue; }
                return result;
            }
            926 => {
                result = true; lexer.set_result_symbol(sym_keyword_exists); lexer.mark_end();
                return result;
            }
            927 => {
                if lookahead == 78 || lookahead == 110 { state = 1104; lexer.advance(false); continue; }
                return result;
            }
            928 => {
                if lookahead == 69 || lookahead == 101 { state = 1105; lexer.advance(false); continue; }
                return result;
            }
            929 => {
                if lookahead == 73 || lookahead == 105 { state = 1106; lexer.advance(false); continue; }
                return result;
            }
            930 => {
                if lookahead == 65 || lookahead == 97 { state = 1107; lexer.advance(false); continue; }
                return result;
            }
            931 => {
                result = true; lexer.set_result_symbol(sym_keyword_fields); lexer.mark_end();
                return result;
            }
            932 => {
                result = true; lexer.set_result_symbol(sym_keyword_filter); lexer.mark_end();
                return result;
            }
            933 => {
                result = true; lexer.set_result_symbol(aux_sym_keyword_real_token2); lexer.mark_end();
                return result;
            }
            934 => {
                result = true; lexer.set_result_symbol(aux_sym_double_token1); lexer.mark_end();
                return result;
            }
            935 => {
                if lookahead == 73 || lookahead == 105 { state = 1108; lexer.advance(false); continue; }
                if lookahead == 83 || lookahead == 115 { state = 1109; lexer.advance(false); continue; }
                return result;
            }
            936 => {
                if lookahead == 78 || lookahead == 110 { state = 1110; lexer.advance(false); continue; }
                if lookahead == 81 || lookahead == 113 { state = 1111; lexer.advance(false); continue; }
                return result;
            }
            937 => {
                if lookahead == 78 || lookahead == 110 { state = 1112; lexer.advance(false); continue; }
                return result;
            }
            938 => {
                result = true; lexer.set_result_symbol(sym_keyword_format); lexer.mark_end();
                return result;
            }
            939 => {
                result = true; lexer.set_result_symbol(sym_keyword_freeze); lexer.mark_end();
                return result;
            }
            940 => {
                if lookahead == 79 || lookahead == 111 { state = 1113; lexer.advance(false); continue; }
                return result;
            }
            941 => {
                if lookahead == 84 || lookahead == 116 { state = 1114; lexer.advance(false); continue; }
                return result;
            }
            942 => {
                if lookahead == 80 || lookahead == 112 { state = 1115; lexer.advance(false); continue; }
                return result;
            }
            943 => {
                if lookahead == 82 || lookahead == 114 { state = 1116; lexer.advance(false); continue; }
                return result;
            }
            944 => {
                result = true; lexer.set_result_symbol(sym_keyword_groups); lexer.mark_end();
                return result;
            }
            945 => {
                result = true; lexer.set_result_symbol(sym_keyword_having); lexer.mark_end();
                return result;
            }
            946 => {
                result = true; lexer.set_result_symbol(sym_keyword_header); lexer.mark_end();
                return result;
            }
            947 => {
                if lookahead == 82 || lookahead == 114 { state = 1117; lexer.advance(false); continue; }
                return result;
            }
            948 => {
                result = true; lexer.set_result_symbol(sym_keyword_ignore); lexer.mark_end();
                return result;
            }
            949 => {
                if lookahead == 65 || lookahead == 97 { state = 1118; lexer.advance(false); continue; }
                return result;
            }
            950 => {
                if lookahead == 66 || lookahead == 98 { state = 1119; lexer.advance(false); continue; }
                return result;
            }
            951 => {
                if lookahead == 69 || lookahead == 101 { state = 1120; lexer.advance(false); continue; }
                return result;
            }
            952 => {
                if lookahead == 76 || lookahead == 108 { state = 1121; lexer.advance(false); continue; }
                return result;
            }
            953 => {
                result = true; lexer.set_result_symbol(sym_keyword_insert); lexer.mark_end();
                return result;
            }
            954 => {
                if lookahead == 68 || lookahead == 100 { state = 1122; lexer.advance(false); continue; }
                return result;
            }
            955 => {
                if lookahead == 82 || lookahead == 114 { state = 1123; lexer.advance(false); continue; }
                return result;
            }
            956 => {
                if lookahead == 69 || lookahead == 101 { state = 1124; lexer.advance(false); continue; }
                return result;
            }
            957 => {
                if lookahead == 65 || lookahead == 97 { state = 1125; lexer.advance(false); continue; }
                return result;
            }
            958 => {
                if lookahead == 82 || lookahead == 114 { state = 1126; lexer.advance(false); continue; }
                return result;
            }
            959 => {
                if lookahead == 73 || lookahead == 105 { state = 1127; lexer.advance(false); continue; }
                return result;
            }
            960 => {
                if lookahead == 76 || lookahead == 108 { state = 1128; lexer.advance(false); continue; }
                return result;
            }
            961 => {
                if lookahead == 71 || lookahead == 103 { state = 1129; lexer.advance(false); continue; }
                return result;
            }
            962 => {
                if lookahead == 76 || lookahead == 108 { state = 1130; lexer.advance(false); continue; }
                return result;
            }
            963 => {
                if lookahead == 79 || lookahead == 111 { state = 1131; lexer.advance(false); continue; }
                return result;
            }
            964 => {
                if lookahead == 79 || lookahead == 111 { state = 1132; lexer.advance(false); continue; }
                return result;
            }
            965 => {
                result = true; lexer.set_result_symbol(sym_keyword_logged); lexer.mark_end();
                return result;
            }
            966 => {
                if lookahead == 73 || lookahead == 105 { state = 1133; lexer.advance(false); continue; }
                return result;
            }
            967 => {
                if lookahead == 68 || lookahead == 100 { state = 1134; lexer.advance(false); continue; }
                return result;
            }
            968 => {
                if lookahead == 65 || lookahead == 97 { state = 1135; lexer.advance(false); continue; }
                return result;
            }
            969 => {
                if lookahead == 85 || lookahead == 117 { state = 1136; lexer.advance(false); continue; }
                return result;
            }
            970 => {
                if lookahead == 73 || lookahead == 105 { state = 1137; lexer.advance(false); continue; }
                return result;
            }
            971 => {
                if lookahead == 84 || lookahead == 116 { state = 1138; lexer.advance(false); continue; }
                return result;
            }
            972 => {
                if lookahead == 85 || lookahead == 117 { state = 1139; lexer.advance(false); continue; }
                return result;
            }
            973 => {
                result = true; lexer.set_result_symbol(sym_keyword_modify); lexer.mark_end();
                return result;
            }
            974 => {
                if lookahead == 76 || lookahead == 108 { state = 1140; lexer.advance(false); continue; }
                return result;
            }
            975 => {
                result = true; lexer.set_result_symbol(sym_keyword_noscan); lexer.mark_end();
                return result;
            }
            976 => {
                if lookahead == 71 || lookahead == 103 { state = 1141; lexer.advance(false); continue; }
                return result;
            }
            977 => {
                result = true; lexer.set_result_symbol(sym_keyword_nowait); lexer.mark_end();
                return result;
            }
            978 => {
                if lookahead == 67 || lookahead == 99 { state = 1142; lexer.advance(false); continue; }
                return result;
            }
            979 => {
                if lookahead == 65 || lookahead == 97 { state = 1143; lexer.advance(false); continue; }
                return result;
            }
            980 => {
                if lookahead == 95 { state = 1144; lexer.advance(false); continue; }
                return result;
            }
            981 => {
                result = true; lexer.set_result_symbol(sym_keyword_offset); lexer.mark_end();
                return result;
            }
            982 => {
                if lookahead == 90 || lookahead == 122 { state = 1145; lexer.advance(false); continue; }
                return result;
            }
            983 => {
                result = true; lexer.set_result_symbol(sym_keyword_option); lexer.mark_end();
                return result;
            }
            984 => {
                if lookahead == 76 || lookahead == 108 { state = 1146; lexer.advance(false); continue; }
                return result;
            }
            985 => {
                result = true; lexer.set_result_symbol(sym_keyword_others); lexer.mark_end();
                return result;
            }
            986 => {
                if lookahead == 73 || lookahead == 105 { state = 1147; lexer.advance(false); continue; }
                return result;
            }
            987 => {
                if lookahead == 69 || lookahead == 101 { state = 1148; lexer.advance(false); continue; }
                return result;
            }
            988 => {
                if lookahead == 84 || lookahead == 116 { state = 1149; lexer.advance(false); continue; }
                return result;
            }
            989 => {
                if lookahead == 73 || lookahead == 105 { state = 1150; lexer.advance(false); continue; }
                return result;
            }
            990 => {
                if lookahead == 82 || lookahead == 114 { state = 1151; lexer.advance(false); continue; }
                return result;
            }
            991 => {
                if lookahead == 69 || lookahead == 101 { state = 1152; lexer.advance(false); continue; }
                if lookahead == 73 || lookahead == 105 { state = 1153; lexer.advance(false); continue; }
                return result;
            }
            992 => {
                if lookahead == 73 || lookahead == 105 { state = 1154; lexer.advance(false); continue; }
                return result;
            }
            993 => {
                if lookahead == 89 || lookahead == 121 { state = 1155; lexer.advance(false); continue; }
                return result;
            }
            994 => {
                if lookahead == 85 || lookahead == 117 { state = 1156; lexer.advance(false); continue; }
                return result;
            }
            995 => {
                if lookahead == 77 || lookahead == 109 { state = 1157; lexer.advance(false); continue; }
                return result;
            }
            996 => {
                result = true; lexer.set_result_symbol(sym_keyword_rcfile); lexer.mark_end();
                return result;
            }
            997 => {
                if lookahead == 73 || lookahead == 105 { state = 1158; lexer.advance(false); continue; }
                return result;
            }
            998 => {
                if lookahead == 78 || lookahead == 110 { state = 1159; lexer.advance(false); continue; }
                return result;
            }
            999 => {
                if lookahead == 83 || lookahead == 115 { state = 1160; lexer.advance(false); continue; }
                return result;
            }
            1000 => {
                if lookahead == 69 || lookahead == 101 { state = 1161; lexer.advance(false); continue; }
                return result;
            }
            1001 => {
                if lookahead == 67 || lookahead == 99 { state = 1162; lexer.advance(false); continue; }
                return result;
            }
            1002 => {
                if lookahead == 69 || lookahead == 101 { state = 1163; lexer.advance(false); continue; }
                return result;
            }
            1003 => {
                result = true; lexer.set_result_symbol(sym_keyword_rename); lexer.mark_end();
                return result;
            }
            1004 => {
                if lookahead == 65 || lookahead == 97 { state = 1164; lexer.advance(false); continue; }
                return result;
            }
            1005 => {
                if lookahead == 69 || lookahead == 101 { state = 1165; lexer.advance(false); continue; }
                return result;
            }
            1006 => {
                if lookahead == 65 || lookahead == 97 { state = 1166; lexer.advance(false); continue; }
                return result;
            }
            1007 => {
                if lookahead == 84 || lookahead == 116 { state = 1167; lexer.advance(false); continue; }
                return result;
            }
            1008 => {
                if lookahead == 67 || lookahead == 99 { state = 1168; lexer.advance(false); continue; }
                return result;
            }
            1009 => {
                result = true; lexer.set_result_symbol(sym_keyword_return); lexer.mark_end();
                if lookahead == 73 || lookahead == 105 { state = 1169; lexer.advance(false); continue; }
                if lookahead == 83 || lookahead == 115 { state = 1170; lexer.advance(false); continue; }
                return result;
            }
            1010 => {
                if lookahead == 69 || lookahead == 101 { state = 1171; lexer.advance(false); continue; }
                return result;
            }
            1011 => {
                if lookahead == 67 || lookahead == 99 { state = 1172; lexer.advance(false); continue; }
                return result;
            }
            1012 => {
                result = true; lexer.set_result_symbol(sym_keyword_schema); lexer.mark_end();
                return result;
            }
            1013 => {
                if lookahead == 84 || lookahead == 116 { state = 1173; lexer.advance(false); continue; }
                return result;
            }
            1014 => {
                result = true; lexer.set_result_symbol(sym_keyword_select); lexer.mark_end();
                return result;
            }
            1015 => {
                if lookahead == 84 || lookahead == 116 { state = 1174; lexer.advance(false); continue; }
                return result;
            }
            1016 => {
                if lookahead == 67 || lookahead == 99 { state = 1175; lexer.advance(false); continue; }
                return result;
            }
            1017 => {
                result = true; lexer.set_result_symbol(aux_sym_keyword_serial_token1); lexer.mark_end();
                if lookahead == 50 { state = 1176; lexer.advance(false); continue; }
                if lookahead == 52 { state = 1177; lexer.advance(false); continue; }
                if lookahead == 56 { state = 1178; lexer.advance(false); continue; }
                if lookahead == 73 || lookahead == 105 { state = 1179; lexer.advance(false); continue; }
                return result;
            }
            1018 => {
                if lookahead == 78 || lookahead == 110 { state = 1180; lexer.advance(false); continue; }
                return result;
            }
            1019 => {
                if lookahead == 82 || lookahead == 114 { state = 1181; lexer.advance(false); continue; }
                return result;
            }
            1020 => {
                if lookahead == 65 || lookahead == 97 { state = 1182; lexer.advance(false); continue; }
                return result;
            }
            1021 => {
                if lookahead == 78 || lookahead == 110 { state = 1183; lexer.advance(false); continue; }
                return result;
            }
            1022 => {
                if lookahead == 79 || lookahead == 111 { state = 1184; lexer.advance(false); continue; }
                return result;
            }
            1023 => {
                if lookahead == 69 || lookahead == 101 { state = 1185; lexer.advance(false); continue; }
                return result;
            }
            1024 => {
                if lookahead == 79 || lookahead == 111 { state = 1186; lexer.advance(false); continue; }
                return result;
            }
            1025 => {
                result = true; lexer.set_result_symbol(sym_keyword_spgist); lexer.mark_end();
                return result;
            }
            1026 => {
                result = true; lexer.set_result_symbol(sym_keyword_stable); lexer.mark_end();
                return result;
            }
            1027 => {
                if lookahead == 69 || lookahead == 101 { state = 1187; lexer.advance(false); continue; }
                return result;
            }
            1028 => {
                if lookahead == 84 || lookahead == 116 { state = 1188; lexer.advance(false); continue; }
                return result;
            }
            1029 => {
                if lookahead == 69 || lookahead == 101 { state = 1189; lexer.advance(false); continue; }
                return result;
            }
            1030 => {
                result = true; lexer.set_result_symbol(sym_keyword_stored); lexer.mark_end();
                return result;
            }
            1031 => {
                result = true; lexer.set_result_symbol(sym_keyword_strict); lexer.mark_end();
                return result;
            }
            1032 => {
                result = true; lexer.set_result_symbol(sym_keyword_string); lexer.mark_end();
                return result;
            }
            1033 => {
                if lookahead == 84 || lookahead == 116 { state = 1190; lexer.advance(false); continue; }
                return result;
            }
            1034 => {
                result = true; lexer.set_result_symbol(sym_keyword_tables); lexer.mark_end();
                if lookahead == 80 || lookahead == 112 { state = 1191; lexer.advance(false); continue; }
                return result;
            }
            1035 => {
                if lookahead == 80 || lookahead == 112 { state = 1192; lexer.advance(false); continue; }
                return result;
            }
            1036 => {
                if lookahead == 65 || lookahead == 97 { state = 1193; lexer.advance(false); continue; }
                return result;
            }
            1037 => {
                if lookahead == 65 || lookahead == 97 { state = 1194; lexer.advance(false); continue; }
                return result;
            }
            1038 => {
                if lookahead == 76 || lookahead == 108 { state = 1195; lexer.advance(false); continue; }
                return result;
            }
            1039 => {
                if lookahead == 65 || lookahead == 97 { state = 1196; lexer.advance(false); continue; }
                return result;
            }
            1040 => {
                if lookahead == 84 || lookahead == 116 { state = 1197; lexer.advance(false); continue; }
                return result;
            }
            1041 => {
                if lookahead == 67 || lookahead == 99 { state = 1198; lexer.advance(false); continue; }
                return result;
            }
            1042 => {
                if lookahead == 82 || lookahead == 114 { state = 1199; lexer.advance(false); continue; }
                return result;
            }
            1043 => {
                if lookahead == 84 || lookahead == 116 { state = 1200; lexer.advance(false); continue; }
                return result;
            }
            1044 => {
                if lookahead == 68 || lookahead == 100 { state = 1201; lexer.advance(false); continue; }
                return result;
            }
            1045 => {
                if lookahead == 69 || lookahead == 101 { state = 1202; lexer.advance(false); continue; }
                return result;
            }
            1046 => {
                if lookahead == 73 || lookahead == 105 { state = 1203; lexer.advance(false); continue; }
                return result;
            }
            1047 => {
                result = true; lexer.set_result_symbol(sym_keyword_unique); lexer.mark_end();
                return result;
            }
            1048 => {
                result = true; lexer.set_result_symbol(sym_keyword_unload); lexer.mark_end();
                return result;
            }
            1049 => {
                if lookahead == 69 || lookahead == 101 { state = 1204; lexer.advance(false); continue; }
                return result;
            }
            1050 => {
                result = true; lexer.set_result_symbol(sym_keyword_unsafe); lexer.mark_end();
                return result;
            }
            1051 => {
                if lookahead == 69 || lookahead == 101 { state = 1205; lexer.advance(false); continue; }
                return result;
            }
            1052 => {
                result = true; lexer.set_result_symbol(sym_keyword_update); lexer.mark_end();
                return result;
            }
            1053 => {
                result = true; lexer.set_result_symbol(sym_keyword_vacuum); lexer.mark_end();
                return result;
            }
            1054 => {
                result = true; lexer.set_result_symbol(sym_keyword_values); lexer.mark_end();
                return result;
            }
            1055 => {
                if lookahead == 65 || lookahead == 97 { state = 1206; lexer.advance(false); continue; }
                return result;
            }
            1056 => {
                if lookahead == 82 || lookahead == 114 { state = 1207; lexer.advance(false); continue; }
                return result;
            }
            1057 => {
                if lookahead == 73 || lookahead == 105 { state = 1208; lexer.advance(false); continue; }
                return result;
            }
            1058 => {
                if lookahead == 71 || lookahead == 103 { state = 1209; lexer.advance(false); continue; }
                return result;
            }
            1059 => {
                if lookahead == 69 || lookahead == 101 { state = 1210; lexer.advance(false); continue; }
                return result;
            }
            1060 => {
                if lookahead == 78 || lookahead == 110 { state = 1211; lexer.advance(false); continue; }
                return result;
            }
            1061 => {
                if lookahead == 76 || lookahead == 108 { state = 1212; lexer.advance(false); continue; }
                return result;
            }
            1062 => {
                if lookahead == 76 || lookahead == 108 { state = 1213; lexer.advance(false); continue; }
                return result;
            }
            1063 => {
                result = true; lexer.set_result_symbol(sym_keyword_window); lexer.mark_end();
                return result;
            }
            1064 => {
                if lookahead == 84 || lookahead == 116 { state = 1214; lexer.advance(false); continue; }
                return result;
            }
            1065 => {
                if lookahead == 76 || lookahead == 108 { state = 1215; lexer.advance(false); continue; }
                return result;
            }
            1066 => {
                result = true; lexer.set_result_symbol(sym_keyword_analyze); lexer.mark_end();
                return result;
            }
            1067 => {
                if lookahead == 84 || lookahead == 116 { state = 1216; lexer.advance(false); continue; }
                return result;
            }
            1068 => {
                if lookahead == 90 || lookahead == 122 { state = 1217; lexer.advance(false); continue; }
                return result;
            }
            1069 => {
                if lookahead == 67 || lookahead == 99 { state = 1218; lexer.advance(false); continue; }
                return result;
            }
            1070 => {
                result = true; lexer.set_result_symbol(sym_keyword_between); lexer.mark_end();
                return result;
            }
            1071 => {
                if lookahead == 65 || lookahead == 97 { state = 1219; lexer.advance(false); continue; }
                return result;
            }
            1072 => {
                if lookahead == 75 || lookahead == 107 { state = 1220; lexer.advance(false); continue; }
                return result;
            }
            1073 => {
                result = true; lexer.set_result_symbol(sym_keyword_boolean); lexer.mark_end();
                return result;
            }
            1074 => {
                result = true; lexer.set_result_symbol(sym_keyword_cascade); lexer.mark_end();
                if lookahead == 68 || lookahead == 100 { state = 1221; lexer.advance(false); continue; }
                return result;
            }
            1075 => {
                if lookahead == 69 || lookahead == 101 { state = 1222; lexer.advance(false); continue; }
                return result;
            }
            1076 => {
                result = true; lexer.set_result_symbol(sym_keyword_collate); lexer.mark_end();
                return result;
            }
            1077 => {
                result = true; lexer.set_result_symbol(sym_keyword_columns); lexer.mark_end();
                return result;
            }
            1078 => {
                result = true; lexer.set_result_symbol(sym_keyword_comment); lexer.mark_end();
                return result;
            }
            1079 => {
                if lookahead == 69 || lookahead == 101 { state = 1223; lexer.advance(false); continue; }
                return result;
            }
            1080 => {
                if lookahead == 83 || lookahead == 115 { state = 1224; lexer.advance(false); continue; }
                return result;
            }
            1081 => {
                result = true; lexer.set_result_symbol(sym_keyword_compute); lexer.mark_end();
                return result;
            }
            1082 => {
                if lookahead == 69 || lookahead == 101 { state = 1225; lexer.advance(false); continue; }
                return result;
            }
            1083 => {
                if lookahead == 84 || lookahead == 116 { state = 1226; lexer.advance(false); continue; }
                return result;
            }
            1084 => {
                if lookahead == 73 || lookahead == 105 { state = 1227; lexer.advance(false); continue; }
                return result;
            }
            1085 => {
                if lookahead == 73 || lookahead == 105 { state = 1228; lexer.advance(false); continue; }
                return result;
            }
            1086 => {
                result = true; lexer.set_result_symbol(sym_keyword_current); lexer.mark_end();
                if lookahead == 95 { state = 1229; lexer.advance(false); continue; }
                return result;
            }
            1087 => {
                if lookahead == 69 || lookahead == 101 { state = 1230; lexer.advance(false); continue; }
                return result;
            }
            1088 => {
                if lookahead == 69 || lookahead == 101 { state = 1231; lexer.advance(false); continue; }
                return result;
            }
            1089 => {
                result = true; lexer.set_result_symbol(sym_keyword_decimal); lexer.mark_end();
                return result;
            }
            1090 => {
                result = true; lexer.set_result_symbol(sym_keyword_declare); lexer.mark_end();
                return result;
            }
            1091 => {
                result = true; lexer.set_result_symbol(sym_keyword_default); lexer.mark_end();
                return result;
            }
            1092 => {
                if lookahead == 66 || lookahead == 98 { state = 1232; lexer.advance(false); continue; }
                return result;
            }
            1093 => {
                if lookahead == 68 || lookahead == 100 { state = 1233; lexer.advance(false); continue; }
                return result;
            }
            1094 => {
                result = true; lexer.set_result_symbol(sym_keyword_definer); lexer.mark_end();
                return result;
            }
            1095 => {
                result = true; lexer.set_result_symbol(sym_keyword_delayed); lexer.mark_end();
                return result;
            }
            1096 => {
                if lookahead == 69 || lookahead == 101 { state = 1234; lexer.advance(false); continue; }
                return result;
            }
            1097 => {
                if lookahead == 84 || lookahead == 116 { state = 1235; lexer.advance(false); continue; }
                return result;
            }
            1098 => {
                if lookahead == 84 || lookahead == 116 { state = 1236; lexer.advance(false); continue; }
                return result;
            }
            1099 => {
                if lookahead == 71 || lookahead == 103 { state = 1237; lexer.advance(false); continue; }
                return result;
            }
            1100 => {
                if lookahead == 69 || lookahead == 101 { state = 1238; lexer.advance(false); continue; }
                return result;
            }
            1101 => {
                result = true; lexer.set_result_symbol(sym_keyword_escaped); lexer.mark_end();
                return result;
            }
            1102 => {
                result = true; lexer.set_result_symbol(sym_keyword_exclude); lexer.mark_end();
                return result;
            }
            1103 => {
                result = true; lexer.set_result_symbol(sym_keyword_execute); lexer.mark_end();
                return result;
            }
            1104 => {
                result = true; lexer.set_result_symbol(sym_keyword_explain); lexer.mark_end();
                return result;
            }
            1105 => {
                if lookahead == 68 || lookahead == 100 { state = 1239; lexer.advance(false); continue; }
                return result;
            }
            1106 => {
                if lookahead == 79 || lookahead == 111 { state = 1240; lexer.advance(false); continue; }
                return result;
            }
            1107 => {
                if lookahead == 76 || lookahead == 108 { state = 1241; lexer.advance(false); continue; }
                return result;
            }
            1108 => {
                if lookahead == 78 || lookahead == 110 { state = 1242; lexer.advance(false); continue; }
                return result;
            }
            1109 => {
                result = true; lexer.set_result_symbol(sym_keyword_follows); lexer.mark_end();
                return result;
            }
            1110 => {
                if lookahead == 79 || lookahead == 111 { state = 1243; lexer.advance(false); continue; }
                if lookahead == 85 || lookahead == 117 { state = 1244; lexer.advance(false); continue; }
                return result;
            }
            1111 => {
                if lookahead == 85 || lookahead == 117 { state = 1245; lexer.advance(false); continue; }
                return result;
            }
            1112 => {
                result = true; lexer.set_result_symbol(sym_keyword_foreign); lexer.mark_end();
                return result;
            }
            1113 => {
                if lookahead == 78 || lookahead == 110 { state = 1246; lexer.advance(false); continue; }
                return result;
            }
            1114 => {
                if lookahead == 69 || lookahead == 101 { state = 1247; lexer.advance(false); continue; }
                return result;
            }
            1115 => {
                if lookahead == 72 || lookahead == 104 { state = 1248; lexer.advance(false); continue; }
                return result;
            }
            1116 => {
                if lookahead == 89 || lookahead == 121 { state = 1249; lexer.advance(false); continue; }
                return result;
            }
            1117 => {
                if lookahead == 73 || lookahead == 105 { state = 1250; lexer.advance(false); continue; }
                return result;
            }
            1118 => {
                if lookahead == 84 || lookahead == 116 { state = 1251; lexer.advance(false); continue; }
                return result;
            }
            1119 => {
                if lookahead == 76 || lookahead == 108 { state = 1252; lexer.advance(false); continue; }
                return result;
            }
            1120 => {
                if lookahead == 78 || lookahead == 110 { state = 1253; lexer.advance(false); continue; }
                return result;
            }
            1121 => {
                if lookahead == 76 || lookahead == 108 { state = 1254; lexer.advance(false); continue; }
                return result;
            }
            1122 => {
                result = true; lexer.set_result_symbol(sym_keyword_instead); lexer.mark_end();
                return result;
            }
            1123 => {
                result = true; lexer.set_result_symbol(aux_sym_keyword_int_token2); lexer.mark_end();
                return result;
            }
            1124 => {
                if lookahead == 67 || lookahead == 99 { state = 1255; lexer.advance(false); continue; }
                return result;
            }
            1125 => {
                if lookahead == 76 || lookahead == 108 { state = 1256; lexer.advance(false); continue; }
                return result;
            }
            1126 => {
                result = true; lexer.set_result_symbol(sym_keyword_invoker); lexer.mark_end();
                return result;
            }
            1127 => {
                if lookahead == 79 || lookahead == 111 { state = 1257; lexer.advance(false); continue; }
                return result;
            }
            1128 => {
                if lookahead == 69 || lookahead == 101 { state = 1258; lexer.advance(false); continue; }
                return result;
            }
            1129 => {
                if lookahead == 69 || lookahead == 101 { state = 1259; lexer.advance(false); continue; }
                return result;
            }
            1130 => {
                result = true; lexer.set_result_symbol(sym_keyword_lateral); lexer.mark_end();
                return result;
            }
            1131 => {
                if lookahead == 79 || lookahead == 111 { state = 1260; lexer.advance(false); continue; }
                return result;
            }
            1132 => {
                if lookahead == 78 || lookahead == 110 { state = 1261; lexer.advance(false); continue; }
                return result;
            }
            1133 => {
                if lookahead == 79 || lookahead == 111 { state = 1262; lexer.advance(false); continue; }
                return result;
            }
            1134 => {
                result = true; lexer.set_result_symbol(sym_keyword_matched); lexer.mark_end();
                return result;
            }
            1135 => {
                if lookahead == 76 || lookahead == 108 { state = 1263; lexer.advance(false); continue; }
                return result;
            }
            1136 => {
                if lookahead == 69 || lookahead == 101 { state = 1264; lexer.advance(false); continue; }
                return result;
            }
            1137 => {
                if lookahead == 78 || lookahead == 110 { state = 1265; lexer.advance(false); continue; }
                return result;
            }
            1138 => {
                if lookahead == 65 || lookahead == 97 { state = 1266; lexer.advance(false); continue; }
                return result;
            }
            1139 => {
                if lookahead == 69 || lookahead == 101 { state = 1267; lexer.advance(false); continue; }
                return result;
            }
            1140 => {
                result = true; lexer.set_result_symbol(sym_keyword_natural); lexer.mark_end();
                return result;
            }
            1141 => {
                result = true; lexer.set_result_symbol(sym_keyword_nothing); lexer.mark_end();
                return result;
            }
            1142 => {
                result = true; lexer.set_result_symbol(sym_keyword_numeric); lexer.mark_end();
                return result;
            }
            1143 => {
                if lookahead == 82 || lookahead == 114 { state = 1268; lexer.advance(false); continue; }
                return result;
            }
            1144 => {
                if lookahead == 73 || lookahead == 105 { state = 1269; lexer.advance(false); continue; }
                return result;
            }
            1145 => {
                if lookahead == 69 || lookahead == 101 { state = 1270; lexer.advance(false); continue; }
                return result;
            }
            1146 => {
                if lookahead == 73 || lookahead == 105 { state = 1271; lexer.advance(false); continue; }
                return result;
            }
            1147 => {
                if lookahead == 84 || lookahead == 116 { state = 1272; lexer.advance(false); continue; }
                return result;
            }
            1148 => {
                if lookahead == 76 || lookahead == 108 { state = 1273; lexer.advance(false); continue; }
                return result;
            }
            1149 => {
                result = true; lexer.set_result_symbol(sym_keyword_parquet); lexer.mark_end();
                return result;
            }
            1150 => {
                if lookahead == 79 || lookahead == 111 { state = 1274; lexer.advance(false); continue; }
                return result;
            }
            1151 => {
                if lookahead == 68 || lookahead == 100 { state = 1275; lexer.advance(false); continue; }
                return result;
            }
            1152 => {
                if lookahead == 83 || lookahead == 115 { state = 1276; lexer.advance(false); continue; }
                return result;
            }
            1153 => {
                if lookahead == 78 || lookahead == 110 { state = 1277; lexer.advance(false); continue; }
                return result;
            }
            1154 => {
                if lookahead == 79 || lookahead == 111 { state = 1278; lexer.advance(false); continue; }
                return result;
            }
            1155 => {
                result = true; lexer.set_result_symbol(sym_keyword_primary); lexer.mark_end();
                return result;
            }
            1156 => {
                if lookahead == 82 || lookahead == 114 { state = 1279; lexer.advance(false); continue; }
                return result;
            }
            1157 => {
                result = true; lexer.set_result_symbol(sym_keyword_program); lexer.mark_end();
                return result;
            }
            1158 => {
                if lookahead == 86 || lookahead == 118 { state = 1280; lexer.advance(false); continue; }
                return result;
            }
            1159 => {
                if lookahead == 67 || lookahead == 99 { state = 1281; lexer.advance(false); continue; }
                return result;
            }
            1160 => {
                if lookahead == 83 || lookahead == 115 { state = 1282; lexer.advance(false); continue; }
                return result;
            }
            1161 => {
                if lookahead == 83 || lookahead == 115 { state = 1283; lexer.advance(false); continue; }
                return result;
            }
            1162 => {
                result = true; lexer.set_result_symbol(sym_keyword_regproc); lexer.mark_end();
                return result;
            }
            1163 => {
                result = true; lexer.set_result_symbol(sym_keyword_regtype); lexer.mark_end();
                return result;
            }
            1164 => {
                if lookahead == 66 || lookahead == 98 { state = 1284; lexer.advance(false); continue; }
                return result;
            }
            1165 => {
                result = true; lexer.set_result_symbol(sym_keyword_replace); lexer.mark_end();
                return result;
            }
            1166 => {
                if lookahead == 84 || lookahead == 116 { state = 1285; lexer.advance(false); continue; }
                return result;
            }
            1167 => {
                result = true; lexer.set_result_symbol(sym_keyword_restart); lexer.mark_end();
                return result;
            }
            1168 => {
                if lookahead == 84 || lookahead == 116 { state = 1286; lexer.advance(false); continue; }
                return result;
            }
            1169 => {
                if lookahead == 78 || lookahead == 110 { state = 1287; lexer.advance(false); continue; }
                return result;
            }
            1170 => {
                result = true; lexer.set_result_symbol(sym_keyword_returns); lexer.mark_end();
                return result;
            }
            1171 => {
                result = true; lexer.set_result_symbol(sym_keyword_rewrite); lexer.mark_end();
                return result;
            }
            1172 => {
                if lookahead == 75 || lookahead == 107 { state = 1288; lexer.advance(false); continue; }
                return result;
            }
            1173 => {
                if lookahead == 89 || lookahead == 121 { state = 1289; lexer.advance(false); continue; }
                return result;
            }
            1174 => {
                if lookahead == 79 || lookahead == 111 { state = 1290; lexer.advance(false); continue; }
                return result;
            }
            1175 => {
                if lookahead == 69 || lookahead == 101 { state = 1291; lexer.advance(false); continue; }
                return result;
            }
            1176 => {
                result = true; lexer.set_result_symbol(aux_sym_keyword_smallserial_token2); lexer.mark_end();
                return result;
            }
            1177 => {
                result = true; lexer.set_result_symbol(aux_sym_keyword_serial_token2); lexer.mark_end();
                return result;
            }
            1178 => {
                result = true; lexer.set_result_symbol(aux_sym_keyword_bigserial_token2); lexer.mark_end();
                return result;
            }
            1179 => {
                if lookahead == 90 || lookahead == 122 { state = 1292; lexer.advance(false); continue; }
                return result;
            }
            1180 => {
                result = true; lexer.set_result_symbol(sym_keyword_session); lexer.mark_end();
                return result;
            }
            1181 => {
                result = true; lexer.set_result_symbol(sym_keyword_similar); lexer.mark_end();
                return result;
            }
            1182 => {
                if lookahead == 84 || lookahead == 116 { state = 1293; lexer.advance(false); continue; }
                return result;
            }
            1183 => {
                if lookahead == 84 || lookahead == 116 { state = 1294; lexer.advance(false); continue; }
                return result;
            }
            1184 => {
                if lookahead == 78 || lookahead == 110 { state = 1295; lexer.advance(false); continue; }
                return result;
            }
            1185 => {
                if lookahead == 82 || lookahead == 114 { state = 1296; lexer.advance(false); continue; }
                return result;
            }
            1186 => {
                if lookahead == 84 || lookahead == 116 { state = 1297; lexer.advance(false); continue; }
                return result;
            }
            1187 => {
                if lookahead == 78 || lookahead == 110 { state = 1298; lexer.advance(false); continue; }
                return result;
            }
            1188 => {
                if lookahead == 73 || lookahead == 105 { state = 1299; lexer.advance(false); continue; }
                return result;
            }
            1189 => {
                result = true; lexer.set_result_symbol(sym_keyword_storage); lexer.mark_end();
                return result;
            }
            1190 => {
                result = true; lexer.set_result_symbol(sym_keyword_support); lexer.mark_end();
                return result;
            }
            1191 => {
                if lookahead == 65 || lookahead == 97 { state = 1300; lexer.advance(false); continue; }
                return result;
            }
            1192 => {
                if lookahead == 69 || lookahead == 101 { state = 1301; lexer.advance(false); continue; }
                return result;
            }
            1193 => {
                if lookahead == 82 || lookahead == 114 { state = 1302; lexer.advance(false); continue; }
                return result;
            }
            1194 => {
                if lookahead == 84 || lookahead == 116 { state = 1303; lexer.advance(false); continue; }
                return result;
            }
            1195 => {
                if lookahead == 69 || lookahead == 101 { state = 1304; lexer.advance(false); continue; }
                return result;
            }
            1196 => {
                if lookahead == 77 || lookahead == 109 { state = 1305; lexer.advance(false); continue; }
                return result;
            }
            1197 => {
                result = true; lexer.set_result_symbol(aux_sym_keyword_tinyint_token1); lexer.mark_end();
                return result;
            }
            1198 => {
                if lookahead == 84 || lookahead == 116 { state = 1306; lexer.advance(false); continue; }
                return result;
            }
            1199 => {
                result = true; lexer.set_result_symbol(sym_keyword_trigger); lexer.mark_end();
                return result;
            }
            1200 => {
                if lookahead == 69 || lookahead == 101 { state = 1307; lexer.advance(false); continue; }
                return result;
            }
            1201 => {
                if lookahead == 69 || lookahead == 101 { state = 1308; lexer.advance(false); continue; }
                return result;
            }
            1202 => {
                if lookahead == 68 || lookahead == 100 { state = 1309; lexer.advance(false); continue; }
                return result;
            }
            1203 => {
                if lookahead == 84 || lookahead == 116 { state = 1310; lexer.advance(false); continue; }
                return result;
            }
            1204 => {
                if lookahead == 68 || lookahead == 100 { state = 1311; lexer.advance(false); continue; }
                return result;
            }
            1205 => {
                if lookahead == 68 || lookahead == 100 { state = 1312; lexer.advance(false); continue; }
                return result;
            }
            1206 => {
                if lookahead == 82 || lookahead == 114 { state = 1313; lexer.advance(false); continue; }
                return result;
            }
            1207 => {
                result = true; lexer.set_result_symbol(aux_sym_keyword_varchar_token1); lexer.mark_end();
                return result;
            }
            1208 => {
                if lookahead == 67 || lookahead == 99 { state = 1314; lexer.advance(false); continue; }
                return result;
            }
            1209 => {
                result = true; lexer.set_result_symbol(sym_keyword_varying); lexer.mark_end();
                return result;
            }
            1210 => {
                result = true; lexer.set_result_symbol(sym_keyword_verbose); lexer.mark_end();
                return result;
            }
            1211 => {
                result = true; lexer.set_result_symbol(sym_keyword_version); lexer.mark_end();
                return result;
            }
            1212 => {
                result = true; lexer.set_result_symbol(sym_keyword_virtual); lexer.mark_end();
                return result;
            }
            1213 => {
                if lookahead == 69 || lookahead == 101 { state = 1315; lexer.advance(false); continue; }
                return result;
            }
            1214 => {
                result = true; lexer.set_result_symbol(sym_keyword_without); lexer.mark_end();
                return result;
            }
            1215 => {
                if lookahead == 76 || lookahead == 108 { state = 1316; lexer.advance(false); continue; }
                return result;
            }
            1216 => {
                if lookahead == 69 || lookahead == 101 { state = 1317; lexer.advance(false); continue; }
                return result;
            }
            1217 => {
                if lookahead == 65 || lookahead == 97 { state = 1318; lexer.advance(false); continue; }
                return result;
            }
            1218 => {
                if lookahead == 82 || lookahead == 114 { state = 1319; lexer.advance(false); continue; }
                return result;
            }
            1219 => {
                if lookahead == 76 || lookahead == 108 { state = 1320; lexer.advance(false); continue; }
                return result;
            }
            1220 => {
                result = true; lexer.set_result_symbol(sym_keyword_bin_pack); lexer.mark_end();
                return result;
            }
            1221 => {
                result = true; lexer.set_result_symbol(sym_keyword_cascaded); lexer.mark_end();
                return result;
            }
            1222 => {
                if lookahead == 82 || lookahead == 114 { state = 1321; lexer.advance(false); continue; }
                return result;
            }
            1223 => {
                if lookahead == 68 || lookahead == 100 { state = 1322; lexer.advance(false); continue; }
                return result;
            }
            1224 => {
                if lookahead == 73 || lookahead == 105 { state = 1323; lexer.advance(false); continue; }
                return result;
            }
            1225 => {
                if lookahead == 78 || lookahead == 110 { state = 1324; lexer.advance(false); continue; }
                return result;
            }
            1226 => {
                result = true; lexer.set_result_symbol(sym_keyword_conflict); lexer.mark_end();
                return result;
            }
            1227 => {
                if lookahead == 79 || lookahead == 111 { state = 1325; lexer.advance(false); continue; }
                return result;
            }
            1228 => {
                if lookahead == 78 || lookahead == 110 { state = 1326; lexer.advance(false); continue; }
                return result;
            }
            1229 => {
                if lookahead == 84 || lookahead == 116 { state = 1327; lexer.advance(false); continue; }
                return result;
            }
            1230 => {
                result = true; lexer.set_result_symbol(sym_keyword_database); lexer.mark_end();
                return result;
            }
            1231 => {
                result = true; lexer.set_result_symbol(sym_keyword_datetime); lexer.mark_end();
                if lookahead == 50 { state = 1328; lexer.advance(false); continue; }
                if lookahead == 79 || lookahead == 111 { state = 1329; lexer.advance(false); continue; }
                return result;
            }
            1232 => {
                if lookahead == 76 || lookahead == 108 { state = 1330; lexer.advance(false); continue; }
                return result;
            }
            1233 => {
                result = true; lexer.set_result_symbol(sym_keyword_deferred); lexer.mark_end();
                return result;
            }
            1234 => {
                if lookahead == 68 || lookahead == 100 { state = 1331; lexer.advance(false); continue; }
                if lookahead == 82 || lookahead == 114 { state = 1332; lexer.advance(false); continue; }
                return result;
            }
            1235 => {
                result = true; lexer.set_result_symbol(sym_keyword_distinct); lexer.mark_end();
                return result;
            }
            1236 => {
                if lookahead == 69 || lookahead == 101 { state = 1333; lexer.advance(false); continue; }
                return result;
            }
            1237 => {
                result = true; lexer.set_result_symbol(sym_keyword_encoding); lexer.mark_end();
                return result;
            }
            1238 => {
                if lookahead == 68 || lookahead == 100 { state = 1334; lexer.advance(false); continue; }
                return result;
            }
            1239 => {
                result = true; lexer.set_result_symbol(sym_keyword_extended); lexer.mark_end();
                return result;
            }
            1240 => {
                if lookahead == 78 || lookahead == 110 { state = 1335; lexer.advance(false); continue; }
                return result;
            }
            1241 => {
                result = true; lexer.set_result_symbol(sym_keyword_external); lexer.mark_end();
                return result;
            }
            1242 => {
                if lookahead == 71 || lookahead == 103 { state = 1336; lexer.advance(false); continue; }
                return result;
            }
            1243 => {
                if lookahead == 84 || lookahead == 116 { state = 1337; lexer.advance(false); continue; }
                return result;
            }
            1244 => {
                if lookahead == 76 || lookahead == 108 { state = 1338; lexer.advance(false); continue; }
                return result;
            }
            1245 => {
                if lookahead == 79 || lookahead == 111 { state = 1339; lexer.advance(false); continue; }
                return result;
            }
            1246 => {
                result = true; lexer.set_result_symbol(sym_keyword_function); lexer.mark_end();
                return result;
            }
            1247 => {
                if lookahead == 68 || lookahead == 100 { state = 1340; lexer.advance(false); continue; }
                return result;
            }
            1248 => {
                if lookahead == 89 || lookahead == 121 { state = 1341; lexer.advance(false); continue; }
                return result;
            }
            1249 => {
                result = true; lexer.set_result_symbol(sym_keyword_geometry); lexer.mark_end();
                return result;
            }
            1250 => {
                if lookahead == 79 || lookahead == 111 { state = 1342; lexer.advance(false); continue; }
                return result;
            }
            1251 => {
                if lookahead == 69 || lookahead == 101 { state = 1343; lexer.advance(false); continue; }
                return result;
            }
            1252 => {
                if lookahead == 69 || lookahead == 101 { state = 1344; lexer.advance(false); continue; }
                return result;
            }
            1253 => {
                if lookahead == 84 || lookahead == 116 { state = 1345; lexer.advance(false); continue; }
                return result;
            }
            1254 => {
                if lookahead == 89 || lookahead == 121 { state = 1346; lexer.advance(false); continue; }
                return result;
            }
            1255 => {
                if lookahead == 84 || lookahead == 116 { state = 1347; lexer.advance(false); continue; }
                return result;
            }
            1256 => {
                result = true; lexer.set_result_symbol(sym_keyword_interval); lexer.mark_end();
                return result;
            }
            1257 => {
                if lookahead == 78 || lookahead == 110 { state = 1348; lexer.advance(false); continue; }
                return result;
            }
            1258 => {
                result = true; lexer.set_result_symbol(sym_keyword_jsonfile); lexer.mark_end();
                return result;
            }
            1259 => {
                result = true; lexer.set_result_symbol(sym_keyword_language); lexer.mark_end();
                return result;
            }
            1260 => {
                if lookahead == 70 || lookahead == 102 { state = 1349; lexer.advance(false); continue; }
                return result;
            }
            1261 => {
                result = true; lexer.set_result_symbol(sym_keyword_location); lexer.mark_end();
                return result;
            }
            1262 => {
                if lookahead == 82 || lookahead == 114 { state = 1350; lexer.advance(false); continue; }
                return result;
            }
            1263 => {
                if lookahead == 73 || lookahead == 105 { state = 1351; lexer.advance(false); continue; }
                return result;
            }
            1264 => {
                result = true; lexer.set_result_symbol(sym_keyword_maxvalue); lexer.mark_end();
                return result;
            }
            1265 => {
                if lookahead == 84 || lookahead == 116 { state = 1352; lexer.advance(false); continue; }
                return result;
            }
            1266 => {
                result = true; lexer.set_result_symbol(sym_keyword_metadata); lexer.mark_end();
                return result;
            }
            1267 => {
                result = true; lexer.set_result_symbol(sym_keyword_minvalue); lexer.mark_end();
                return result;
            }
            1268 => {
                result = true; lexer.set_result_symbol(sym_keyword_nvarchar); lexer.mark_end();
                return result;
            }
            1269 => {
                if lookahead == 68 || lookahead == 100 { state = 1353; lexer.advance(false); continue; }
                return result;
            }
            1270 => {
                result = true; lexer.set_result_symbol(sym_keyword_optimize); lexer.mark_end();
                return result;
            }
            1271 => {
                if lookahead == 84 || lookahead == 116 { state = 1354; lexer.advance(false); continue; }
                return result;
            }
            1272 => {
                if lookahead == 69 || lookahead == 101 { state = 1355; lexer.advance(false); continue; }
                return result;
            }
            1273 => {
                result = true; lexer.set_result_symbol(sym_keyword_parallel); lexer.mark_end();
                return result;
            }
            1274 => {
                if lookahead == 78 || lookahead == 110 { state = 1356; lexer.advance(false); continue; }
                return result;
            }
            1275 => {
                result = true; lexer.set_result_symbol(sym_keyword_password); lexer.mark_end();
                return result;
            }
            1276 => {
                result = true; lexer.set_result_symbol(sym_keyword_precedes); lexer.mark_end();
                return result;
            }
            1277 => {
                if lookahead == 71 || lookahead == 103 { state = 1357; lexer.advance(false); continue; }
                return result;
            }
            1278 => {
                if lookahead == 78 || lookahead == 110 { state = 1358; lexer.advance(false); continue; }
                return result;
            }
            1279 => {
                if lookahead == 69 || lookahead == 101 { state = 1359; lexer.advance(false); continue; }
                return result;
            }
            1280 => {
                if lookahead == 69 || lookahead == 101 { state = 1360; lexer.advance(false); continue; }
                return result;
            }
            1281 => {
                if lookahead == 69 || lookahead == 101 { state = 1361; lexer.advance(false); continue; }
                if lookahead == 73 || lookahead == 105 { state = 1362; lexer.advance(false); continue; }
                return result;
            }
            1282 => {
                result = true; lexer.set_result_symbol(sym_keyword_regclass); lexer.mark_end();
                return result;
            }
            1283 => {
                if lookahead == 80 || lookahead == 112 { state = 1363; lexer.advance(false); continue; }
                return result;
            }
            1284 => {
                if lookahead == 76 || lookahead == 108 { state = 1364; lexer.advance(false); continue; }
                return result;
            }
            1285 => {
                if lookahead == 73 || lookahead == 105 { state = 1365; lexer.advance(false); continue; }
                return result;
            }
            1286 => {
                result = true; lexer.set_result_symbol(sym_keyword_restrict); lexer.mark_end();
                if lookahead == 69 || lookahead == 101 { state = 1366; lexer.advance(false); continue; }
                return result;
            }
            1287 => {
                if lookahead == 71 || lookahead == 103 { state = 1367; lexer.advance(false); continue; }
                return result;
            }
            1288 => {
                result = true; lexer.set_result_symbol(sym_keyword_rollback); lexer.mark_end();
                return result;
            }
            1289 => {
                result = true; lexer.set_result_symbol(sym_keyword_security); lexer.mark_end();
                return result;
            }
            1290 => {
                if lookahead == 82 || lookahead == 114 { state = 1368; lexer.advance(false); continue; }
                return result;
            }
            1291 => {
                result = true; lexer.set_result_symbol(sym_keyword_sequence); lexer.mark_end();
                if lookahead == 70 || lookahead == 102 { state = 1369; lexer.advance(false); continue; }
                return result;
            }
            1292 => {
                if lookahead == 65 || lookahead == 97 { state = 1370; lexer.advance(false); continue; }
                return result;
            }
            1293 => {
                if lookahead == 69 || lookahead == 101 { state = 1371; lexer.advance(false); continue; }
                return result;
            }
            1294 => {
                result = true; lexer.set_result_symbol(aux_sym_keyword_smallint_token1); lexer.mark_end();
                return result;
            }
            1295 => {
                if lookahead == 69 || lookahead == 101 { state = 1372; lexer.advance(false); continue; }
                return result;
            }
            1296 => {
                if lookahead == 73 || lookahead == 105 { state = 1373; lexer.advance(false); continue; }
                return result;
            }
            1297 => {
                result = true; lexer.set_result_symbol(sym_keyword_snapshot); lexer.mark_end();
                return result;
            }
            1298 => {
                if lookahead == 84 || lookahead == 116 { state = 1374; lexer.advance(false); continue; }
                return result;
            }
            1299 => {
                if lookahead == 67 || lookahead == 99 { state = 1375; lexer.advance(false); continue; }
                return result;
            }
            1300 => {
                if lookahead == 67 || lookahead == 99 { state = 1376; lexer.advance(false); continue; }
                return result;
            }
            1301 => {
                if lookahead == 82 || lookahead == 114 { state = 1377; lexer.advance(false); continue; }
                return result;
            }
            1302 => {
                if lookahead == 89 || lookahead == 121 { state = 1378; lexer.advance(false); continue; }
                return result;
            }
            1303 => {
                if lookahead == 69 || lookahead == 101 { state = 1379; lexer.advance(false); continue; }
                return result;
            }
            1304 => {
                result = true; lexer.set_result_symbol(sym_keyword_textfile); lexer.mark_end();
                return result;
            }
            1305 => {
                if lookahead == 80 || lookahead == 112 { state = 1380; lexer.advance(false); continue; }
                return result;
            }
            1306 => {
                if lookahead == 73 || lookahead == 105 { state = 1381; lexer.advance(false); continue; }
                return result;
            }
            1307 => {
                result = true; lexer.set_result_symbol(sym_keyword_truncate); lexer.mark_end();
                return result;
            }
            1308 => {
                if lookahead == 68 || lookahead == 100 { state = 1382; lexer.advance(false); continue; }
                return result;
            }
            1309 => {
                result = true; lexer.set_result_symbol(sym_keyword_uncached); lexer.mark_end();
                return result;
            }
            1310 => {
                if lookahead == 84 || lookahead == 116 { state = 1383; lexer.advance(false); continue; }
                return result;
            }
            1311 => {
                result = true; lexer.set_result_symbol(sym_keyword_unlogged); lexer.mark_end();
                return result;
            }
            1312 => {
                result = true; lexer.set_result_symbol(sym_keyword_unsigned); lexer.mark_end();
                return result;
            }
            1313 => {
                if lookahead == 89 || lookahead == 121 { state = 1384; lexer.advance(false); continue; }
                return result;
            }
            1314 => {
                result = true; lexer.set_result_symbol(sym_keyword_variadic); lexer.mark_end();
                return result;
            }
            1315 => {
                result = true; lexer.set_result_symbol(sym_keyword_volatile); lexer.mark_end();
                return result;
            }
            1316 => {
                result = true; lexer.set_result_symbol(sym_keyword_zerofill); lexer.mark_end();
                return result;
            }
            1317 => {
                result = true; lexer.set_result_symbol(sym_keyword_attribute); lexer.mark_end();
                return result;
            }
            1318 => {
                if lookahead == 84 || lookahead == 116 { state = 1385; lexer.advance(false); continue; }
                return result;
            }
            1319 => {
                if lookahead == 69 || lookahead == 101 { state = 1386; lexer.advance(false); continue; }
                return result;
            }
            1320 => {
                result = true; lexer.set_result_symbol(aux_sym_keyword_bigserial_token1); lexer.mark_end();
                return result;
            }
            1321 => {
                result = true; lexer.set_result_symbol(aux_sym_keyword_character_token1); lexer.mark_end();
                if lookahead == 73 || lookahead == 105 { state = 1387; lexer.advance(false); continue; }
                return result;
            }
            1322 => {
                result = true; lexer.set_result_symbol(sym_keyword_committed); lexer.mark_end();
                return result;
            }
            1323 => {
                if lookahead == 79 || lookahead == 111 { state = 1388; lexer.advance(false); continue; }
                return result;
            }
            1324 => {
                if lookahead == 84 || lookahead == 116 { state = 1389; lexer.advance(false); continue; }
                return result;
            }
            1325 => {
                if lookahead == 78 || lookahead == 110 { state = 1390; lexer.advance(false); continue; }
                return result;
            }
            1326 => {
                if lookahead == 84 || lookahead == 116 { state = 1391; lexer.advance(false); continue; }
                return result;
            }
            1327 => {
                if lookahead == 73 || lookahead == 105 { state = 1392; lexer.advance(false); continue; }
                return result;
            }
            1328 => {
                result = true; lexer.set_result_symbol(sym_keyword_datetime2); lexer.mark_end();
                return result;
            }
            1329 => {
                if lookahead == 70 || lookahead == 102 { state = 1393; lexer.advance(false); continue; }
                return result;
            }
            1330 => {
                if lookahead == 69 || lookahead == 101 { state = 1394; lexer.advance(false); continue; }
                return result;
            }
            1331 => {
                result = true; lexer.set_result_symbol(sym_keyword_delimited); lexer.mark_end();
                return result;
            }
            1332 => {
                result = true; lexer.set_result_symbol(sym_keyword_delimiter); lexer.mark_end();
                return result;
            }
            1333 => {
                result = true; lexer.set_result_symbol(sym_keyword_duplicate); lexer.mark_end();
                return result;
            }
            1334 => {
                result = true; lexer.set_result_symbol(sym_keyword_encrypted); lexer.mark_end();
                return result;
            }
            1335 => {
                result = true; lexer.set_result_symbol(sym_keyword_extension); lexer.mark_end();
                return result;
            }
            1336 => {
                result = true; lexer.set_result_symbol(sym_keyword_following); lexer.mark_end();
                return result;
            }
            1337 => {
                if lookahead == 95 { state = 1395; lexer.advance(false); continue; }
                return result;
            }
            1338 => {
                if lookahead == 76 || lookahead == 108 { state = 1396; lexer.advance(false); continue; }
                return result;
            }
            1339 => {
                if lookahead == 84 || lookahead == 116 { state = 1397; lexer.advance(false); continue; }
                return result;
            }
            1340 => {
                result = true; lexer.set_result_symbol(sym_keyword_generated); lexer.mark_end();
                return result;
            }
            1341 => {
                result = true; lexer.set_result_symbol(sym_keyword_geography); lexer.mark_end();
                return result;
            }
            1342 => {
                if lookahead == 82 || lookahead == 114 { state = 1398; lexer.advance(false); continue; }
                return result;
            }
            1343 => {
                result = true; lexer.set_result_symbol(sym_keyword_immediate); lexer.mark_end();
                return result;
            }
            1344 => {
                result = true; lexer.set_result_symbol(sym_keyword_immutable); lexer.mark_end();
                return result;
            }
            1345 => {
                result = true; lexer.set_result_symbol(sym_keyword_increment); lexer.mark_end();
                if lookahead == 65 || lookahead == 97 { state = 1399; lexer.advance(false); continue; }
                return result;
            }
            1346 => {
                result = true; lexer.set_result_symbol(sym_keyword_initially); lexer.mark_end();
                return result;
            }
            1347 => {
                result = true; lexer.set_result_symbol(sym_keyword_intersect); lexer.mark_end();
                return result;
            }
            1348 => {
                result = true; lexer.set_result_symbol(sym_keyword_isolation); lexer.mark_end();
                return result;
            }
            1349 => {
                result = true; lexer.set_result_symbol(sym_keyword_leakproof); lexer.mark_end();
                return result;
            }
            1350 => {
                if lookahead == 73 || lookahead == 105 { state = 1400; lexer.advance(false); continue; }
                return result;
            }
            1351 => {
                if lookahead == 90 || lookahead == 122 { state = 1401; lexer.advance(false); continue; }
                return result;
            }
            1352 => {
                result = true; lexer.set_result_symbol(aux_sym_keyword_mediumint_token1); lexer.mark_end();
                return result;
            }
            1353 => {
                result = true; lexer.set_result_symbol(sym_keyword_object_id); lexer.mark_end();
                return result;
            }
            1354 => {
                if lookahead == 89 || lookahead == 121 { state = 1402; lexer.advance(false); continue; }
                return result;
            }
            1355 => {
                result = true; lexer.set_result_symbol(sym_keyword_overwrite); lexer.mark_end();
                return result;
            }
            1356 => {
                result = true; lexer.set_result_symbol(sym_keyword_partition); lexer.mark_end();
                if lookahead == 69 || lookahead == 101 { state = 1403; lexer.advance(false); continue; }
                return result;
            }
            1357 => {
                result = true; lexer.set_result_symbol(sym_keyword_preceding); lexer.mark_end();
                return result;
            }
            1358 => {
                result = true; lexer.set_result_symbol(sym_keyword_precision); lexer.mark_end();
                return result;
            }
            1359 => {
                result = true; lexer.set_result_symbol(sym_keyword_procedure); lexer.mark_end();
                return result;
            }
            1360 => {
                result = true; lexer.set_result_symbol(sym_keyword_recursive); lexer.mark_end();
                return result;
            }
            1361 => {
                if lookahead == 83 || lookahead == 115 { state = 1404; lexer.advance(false); continue; }
                return result;
            }
            1362 => {
                if lookahead == 78 || lookahead == 110 { state = 1405; lexer.advance(false); continue; }
                return result;
            }
            1363 => {
                if lookahead == 65 || lookahead == 97 { state = 1406; lexer.advance(false); continue; }
                return result;
            }
            1364 => {
                if lookahead == 69 || lookahead == 101 { state = 1407; lexer.advance(false); continue; }
                return result;
            }
            1365 => {
                if lookahead == 79 || lookahead == 111 { state = 1408; lexer.advance(false); continue; }
                return result;
            }
            1366 => {
                if lookahead == 68 || lookahead == 100 { state = 1409; lexer.advance(false); continue; }
                return result;
            }
            1367 => {
                result = true; lexer.set_result_symbol(sym_keyword_returning); lexer.mark_end();
                return result;
            }
            1368 => {
                result = true; lexer.set_result_symbol(sym_keyword_separator); lexer.mark_end();
                return result;
            }
            1369 => {
                if lookahead == 73 || lookahead == 105 { state = 1410; lexer.advance(false); continue; }
                return result;
            }
            1370 => {
                if lookahead == 66 || lookahead == 98 { state = 1411; lexer.advance(false); continue; }
                return result;
            }
            1371 => {
                if lookahead == 84 || lookahead == 116 { state = 1412; lexer.advance(false); continue; }
                return result;
            }
            1372 => {
                if lookahead == 89 || lookahead == 121 { state = 1413; lexer.advance(false); continue; }
                return result;
            }
            1373 => {
                if lookahead == 65 || lookahead == 97 { state = 1414; lexer.advance(false); continue; }
                return result;
            }
            1374 => {
                result = true; lexer.set_result_symbol(sym_keyword_statement); lexer.mark_end();
                return result;
            }
            1375 => {
                if lookahead == 83 || lookahead == 115 { state = 1415; lexer.advance(false); continue; }
                return result;
            }
            1376 => {
                if lookahead == 69 || lookahead == 101 { state = 1416; lexer.advance(false); continue; }
                return result;
            }
            1377 => {
                if lookahead == 84 || lookahead == 116 { state = 1417; lexer.advance(false); continue; }
                return result;
            }
            1378 => {
                result = true; lexer.set_result_symbol(sym_keyword_temporary); lexer.mark_end();
                return result;
            }
            1379 => {
                if lookahead == 68 || lookahead == 100 { state = 1418; lexer.advance(false); continue; }
                return result;
            }
            1380 => {
                result = true; lexer.set_result_symbol(sym_keyword_timestamp); lexer.mark_end();
                if lookahead == 84 || lookahead == 116 { state = 1419; lexer.advance(false); continue; }
                return result;
            }
            1381 => {
                if lookahead == 79 || lookahead == 111 { state = 1420; lexer.advance(false); continue; }
                return result;
            }
            1382 => {
                result = true; lexer.set_result_symbol(sym_keyword_unbounded); lexer.mark_end();
                return result;
            }
            1383 => {
                if lookahead == 69 || lookahead == 101 { state = 1421; lexer.advance(false); continue; }
                return result;
            }
            1384 => {
                result = true; lexer.set_result_symbol(sym_keyword_varbinary); lexer.mark_end();
                return result;
            }
            1385 => {
                if lookahead == 73 || lookahead == 105 { state = 1422; lexer.advance(false); continue; }
                return result;
            }
            1386 => {
                if lookahead == 77 || lookahead == 109 { state = 1423; lexer.advance(false); continue; }
                return result;
            }
            1387 => {
                if lookahead == 83 || lookahead == 115 { state = 1424; lexer.advance(false); continue; }
                return result;
            }
            1388 => {
                if lookahead == 78 || lookahead == 110 { state = 1425; lexer.advance(false); continue; }
                return result;
            }
            1389 => {
                if lookahead == 76 || lookahead == 108 { state = 1426; lexer.advance(false); continue; }
                return result;
            }
            1390 => {
                result = true; lexer.set_result_symbol(sym_keyword_connection); lexer.mark_end();
                return result;
            }
            1391 => {
                result = true; lexer.set_result_symbol(sym_keyword_constraint); lexer.mark_end();
                if lookahead == 83 || lookahead == 115 { state = 1427; lexer.advance(false); continue; }
                return result;
            }
            1392 => {
                if lookahead == 77 || lookahead == 109 { state = 1428; lexer.advance(false); continue; }
                return result;
            }
            1393 => {
                if lookahead == 70 || lookahead == 102 { state = 1429; lexer.advance(false); continue; }
                return result;
            }
            1394 => {
                result = true; lexer.set_result_symbol(sym_keyword_deferrable); lexer.mark_end();
                return result;
            }
            1395 => {
                if lookahead == 78 || lookahead == 110 { state = 1430; lexer.advance(false); continue; }
                return result;
            }
            1396 => {
                result = true; lexer.set_result_symbol(sym_keyword_force_null); lexer.mark_end();
                return result;
            }
            1397 => {
                if lookahead == 69 || lookahead == 101 { state = 1431; lexer.advance(false); continue; }
                return result;
            }
            1398 => {
                if lookahead == 73 || lookahead == 105 { state = 1432; lexer.advance(false); continue; }
                return result;
            }
            1399 => {
                if lookahead == 76 || lookahead == 108 { state = 1433; lexer.advance(false); continue; }
                return result;
            }
            1400 => {
                if lookahead == 84 || lookahead == 116 { state = 1434; lexer.advance(false); continue; }
                return result;
            }
            1401 => {
                if lookahead == 69 || lookahead == 101 { state = 1435; lexer.advance(false); continue; }
                return result;
            }
            1402 => {
                result = true; lexer.set_result_symbol(sym_keyword_ordinality); lexer.mark_end();
                return result;
            }
            1403 => {
                if lookahead == 68 || lookahead == 100 { state = 1436; lexer.advance(false); continue; }
                return result;
            }
            1404 => {
                result = true; lexer.set_result_symbol(sym_keyword_references); lexer.mark_end();
                return result;
            }
            1405 => {
                if lookahead == 71 || lookahead == 103 { state = 1437; lexer.advance(false); continue; }
                return result;
            }
            1406 => {
                if lookahead == 67 || lookahead == 99 { state = 1438; lexer.advance(false); continue; }
                return result;
            }
            1407 => {
                result = true; lexer.set_result_symbol(sym_keyword_repeatable); lexer.mark_end();
                return result;
            }
            1408 => {
                if lookahead == 78 || lookahead == 110 { state = 1439; lexer.advance(false); continue; }
                return result;
            }
            1409 => {
                result = true; lexer.set_result_symbol(sym_keyword_restricted); lexer.mark_end();
                return result;
            }
            1410 => {
                if lookahead == 76 || lookahead == 108 { state = 1440; lexer.advance(false); continue; }
                return result;
            }
            1411 => {
                if lookahead == 76 || lookahead == 108 { state = 1441; lexer.advance(false); continue; }
                return result;
            }
            1412 => {
                if lookahead == 73 || lookahead == 105 { state = 1442; lexer.advance(false); continue; }
                return result;
            }
            1413 => {
                result = true; lexer.set_result_symbol(sym_keyword_smallmoney); lexer.mark_end();
                return result;
            }
            1414 => {
                if lookahead == 76 || lookahead == 108 { state = 1443; lexer.advance(false); continue; }
                return result;
            }
            1415 => {
                result = true; lexer.set_result_symbol(sym_keyword_statistics); lexer.mark_end();
                return result;
            }
            1416 => {
                result = true; lexer.set_result_symbol(sym_keyword_tablespace); lexer.mark_end();
                return result;
            }
            1417 => {
                if lookahead == 73 || lookahead == 105 { state = 1444; lexer.advance(false); continue; }
                return result;
            }
            1418 => {
                result = true; lexer.set_result_symbol(sym_keyword_terminated); lexer.mark_end();
                return result;
            }
            1419 => {
                if lookahead == 90 || lookahead == 122 { state = 1445; lexer.advance(false); continue; }
                return result;
            }
            1420 => {
                if lookahead == 78 || lookahead == 110 { state = 1446; lexer.advance(false); continue; }
                return result;
            }
            1421 => {
                if lookahead == 68 || lookahead == 100 { state = 1447; lexer.advance(false); continue; }
                return result;
            }
            1422 => {
                if lookahead == 79 || lookahead == 111 { state = 1448; lexer.advance(false); continue; }
                return result;
            }
            1423 => {
                if lookahead == 69 || lookahead == 101 { state = 1449; lexer.advance(false); continue; }
                return result;
            }
            1424 => {
                if lookahead == 84 || lookahead == 116 { state = 1450; lexer.advance(false); continue; }
                return result;
            }
            1425 => {
                result = true; lexer.set_result_symbol(sym_keyword_compression); lexer.mark_end();
                return result;
            }
            1426 => {
                if lookahead == 89 || lookahead == 121 { state = 1451; lexer.advance(false); continue; }
                return result;
            }
            1427 => {
                result = true; lexer.set_result_symbol(sym_keyword_constraints); lexer.mark_end();
                return result;
            }
            1428 => {
                if lookahead == 69 || lookahead == 101 { state = 1452; lexer.advance(false); continue; }
                return result;
            }
            1429 => {
                if lookahead == 83 || lookahead == 115 { state = 1453; lexer.advance(false); continue; }
                return result;
            }
            1430 => {
                if lookahead == 85 || lookahead == 117 { state = 1454; lexer.advance(false); continue; }
                return result;
            }
            1431 => {
                result = true; lexer.set_result_symbol(sym_keyword_force_quote); lexer.mark_end();
                return result;
            }
            1432 => {
                if lookahead == 84 || lookahead == 116 { state = 1455; lexer.advance(false); continue; }
                return result;
            }
            1433 => {
                result = true; lexer.set_result_symbol(sym_keyword_incremental); lexer.mark_end();
                return result;
            }
            1434 => {
                if lookahead == 89 || lookahead == 121 { state = 1456; lexer.advance(false); continue; }
                return result;
            }
            1435 => {
                if lookahead == 68 || lookahead == 100 { state = 1457; lexer.advance(false); continue; }
                return result;
            }
            1436 => {
                result = true; lexer.set_result_symbol(sym_keyword_partitioned); lexer.mark_end();
                return result;
            }
            1437 => {
                result = true; lexer.set_result_symbol(sym_keyword_referencing); lexer.mark_end();
                return result;
            }
            1438 => {
                if lookahead == 69 || lookahead == 101 { state = 1458; lexer.advance(false); continue; }
                return result;
            }
            1439 => {
                result = true; lexer.set_result_symbol(sym_keyword_replication); lexer.mark_end();
                return result;
            }
            1440 => {
                if lookahead == 69 || lookahead == 101 { state = 1459; lexer.advance(false); continue; }
                return result;
            }
            1441 => {
                if lookahead == 69 || lookahead == 101 { state = 1460; lexer.advance(false); continue; }
                return result;
            }
            1442 => {
                if lookahead == 77 || lookahead == 109 { state = 1461; lexer.advance(false); continue; }
                return result;
            }
            1443 => {
                result = true; lexer.set_result_symbol(aux_sym_keyword_smallserial_token1); lexer.mark_end();
                return result;
            }
            1444 => {
                if lookahead == 69 || lookahead == 101 { state = 1462; lexer.advance(false); continue; }
                return result;
            }
            1445 => {
                result = true; lexer.set_result_symbol(sym_keyword_timestamptz); lexer.mark_end();
                return result;
            }
            1446 => {
                result = true; lexer.set_result_symbol(sym_keyword_transaction); lexer.mark_end();
                return result;
            }
            1447 => {
                result = true; lexer.set_result_symbol(sym_keyword_uncommitted); lexer.mark_end();
                return result;
            }
            1448 => {
                if lookahead == 78 || lookahead == 110 { state = 1463; lexer.advance(false); continue; }
                return result;
            }
            1449 => {
                if lookahead == 78 || lookahead == 110 { state = 1464; lexer.advance(false); continue; }
                return result;
            }
            1450 => {
                if lookahead == 73 || lookahead == 105 { state = 1465; lexer.advance(false); continue; }
                return result;
            }
            1451 => {
                result = true; lexer.set_result_symbol(sym_keyword_concurrently); lexer.mark_end();
                return result;
            }
            1452 => {
                if lookahead == 83 || lookahead == 115 { state = 1466; lexer.advance(false); continue; }
                return result;
            }
            1453 => {
                if lookahead == 69 || lookahead == 101 { state = 1467; lexer.advance(false); continue; }
                return result;
            }
            1454 => {
                if lookahead == 76 || lookahead == 108 { state = 1468; lexer.advance(false); continue; }
                return result;
            }
            1455 => {
                if lookahead == 89 || lookahead == 121 { state = 1469; lexer.advance(false); continue; }
                return result;
            }
            1456 => {
                result = true; lexer.set_result_symbol(sym_keyword_low_priority); lexer.mark_end();
                return result;
            }
            1457 => {
                result = true; lexer.set_result_symbol(sym_keyword_materialized); lexer.mark_end();
                return result;
            }
            1458 => {
                result = true; lexer.set_result_symbol(sym_keyword_regnamespace); lexer.mark_end();
                return result;
            }
            1459 => {
                result = true; lexer.set_result_symbol(sym_keyword_sequencefile); lexer.mark_end();
                return result;
            }
            1460 => {
                result = true; lexer.set_result_symbol(sym_keyword_serializable); lexer.mark_end();
                return result;
            }
            1461 => {
                if lookahead == 69 || lookahead == 101 { state = 1470; lexer.advance(false); continue; }
                return result;
            }
            1462 => {
                if lookahead == 83 || lookahead == 115 { state = 1471; lexer.advance(false); continue; }
                return result;
            }
            1463 => {
                result = true; lexer.set_result_symbol(sym_keyword_authorization); lexer.mark_end();
                return result;
            }
            1464 => {
                if lookahead == 84 || lookahead == 116 { state = 1472; lexer.advance(false); continue; }
                return result;
            }
            1465 => {
                if lookahead == 67 || lookahead == 99 { state = 1473; lexer.advance(false); continue; }
                return result;
            }
            1466 => {
                if lookahead == 84 || lookahead == 116 { state = 1474; lexer.advance(false); continue; }
                return result;
            }
            1467 => {
                if lookahead == 84 || lookahead == 116 { state = 1475; lexer.advance(false); continue; }
                return result;
            }
            1468 => {
                if lookahead == 76 || lookahead == 108 { state = 1476; lexer.advance(false); continue; }
                return result;
            }
            1469 => {
                result = true; lexer.set_result_symbol(sym_keyword_high_priority); lexer.mark_end();
                return result;
            }
            1470 => {
                result = true; lexer.set_result_symbol(sym_keyword_smalldatetime); lexer.mark_end();
                return result;
            }
            1471 => {
                result = true; lexer.set_result_symbol(sym_keyword_tblproperties); lexer.mark_end();
                return result;
            }
            1472 => {
                result = true; lexer.set_result_symbol(sym_keyword_auto_increment); lexer.mark_end();
                return result;
            }
            1473 => {
                if lookahead == 83 || lookahead == 115 { state = 1477; lexer.advance(false); continue; }
                return result;
            }
            1474 => {
                if lookahead == 65 || lookahead == 97 { state = 1478; lexer.advance(false); continue; }
                return result;
            }
            1475 => {
                result = true; lexer.set_result_symbol(sym_keyword_datetimeoffset); lexer.mark_end();
                return result;
            }
            1476 => {
                result = true; lexer.set_result_symbol(sym_keyword_force_not_null); lexer.mark_end();
                return result;
            }
            1477 => {
                result = true; lexer.set_result_symbol(sym_keyword_characteristics); lexer.mark_end();
                return result;
            }
            1478 => {
                if lookahead == 77 || lookahead == 109 { state = 1479; lexer.advance(false); continue; }
                return result;
            }
            1479 => {
                if lookahead == 80 || lookahead == 112 { state = 1480; lexer.advance(false); continue; }
                return result;
            }
            1480 => {
                result = true; lexer.set_result_symbol(sym_keyword_current_timestamp); lexer.mark_end();
                return result;
            }
            _ => return false,
        }
    }
}
