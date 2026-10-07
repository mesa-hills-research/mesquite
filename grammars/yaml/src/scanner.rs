//! YAML's external scanner and core-schema resolver, translated from `scanner.c`
//! and `schema.core.c`. Token order and speculative advances follow the C scanner.

use ts_port_tables::{ExternalScanner, Lexer, SERIALIZATION_BUFFER_SIZE};

const END_OF_FILE: usize = 0;
const S_DIR_YML_BGN: usize = 1;
const R_DIR_YML_VER: usize = 2;
const S_DIR_TAG_BGN: usize = 3;
const R_DIR_TAG_HDL: usize = 4;
const R_DIR_TAG_PFX: usize = 5;
const S_DIR_RSV_BGN: usize = 6;
const R_DIR_RSV_PRM: usize = 7;
const S_DRS_END: usize = 8;
const S_DOC_END: usize = 9;
const R_BLK_SEQ_BGN: usize = 10;
const BR_BLK_SEQ_BGN: usize = 11;
const B_BLK_SEQ_BGN: usize = 12;
const R_BLK_KEY_BGN: usize = 13;
const BR_BLK_KEY_BGN: usize = 14;
const B_BLK_KEY_BGN: usize = 15;
const R_BLK_VAL_BGN: usize = 16;
const BR_BLK_VAL_BGN: usize = 17;
const B_BLK_VAL_BGN: usize = 18;
const R_BLK_IMP_BGN: usize = 19;
const R_BLK_LIT_BGN: usize = 20;
const BR_BLK_LIT_BGN: usize = 21;
const R_BLK_FLD_BGN: usize = 22;
const BR_BLK_FLD_BGN: usize = 23;
const BR_BLK_STR_CTN: usize = 24;
const R_FLW_SEQ_BGN: usize = 25;
const BR_FLW_SEQ_BGN: usize = 26;
const B_FLW_SEQ_BGN: usize = 27;
const R_FLW_SEQ_END: usize = 28;
const BR_FLW_SEQ_END: usize = 29;
const B_FLW_SEQ_END: usize = 30;
const R_FLW_MAP_BGN: usize = 31;
const BR_FLW_MAP_BGN: usize = 32;
const B_FLW_MAP_BGN: usize = 33;
const R_FLW_MAP_END: usize = 34;
const BR_FLW_MAP_END: usize = 35;
const B_FLW_MAP_END: usize = 36;
const R_FLW_SEP_BGN: usize = 37;
const BR_FLW_SEP_BGN: usize = 38;
const R_FLW_KEY_BGN: usize = 39;
const BR_FLW_KEY_BGN: usize = 40;
const R_FLW_JSV_BGN: usize = 41;
const BR_FLW_JSV_BGN: usize = 42;
const R_FLW_NJV_BGN: usize = 43;
const BR_FLW_NJV_BGN: usize = 44;
const R_DQT_STR_BGN: usize = 45;
const BR_DQT_STR_BGN: usize = 46;
const B_DQT_STR_BGN: usize = 47;
const R_DQT_STR_CTN: usize = 48;
const BR_DQT_STR_CTN: usize = 49;
const R_DQT_ESC_NWL: usize = 50;
const BR_DQT_ESC_NWL: usize = 51;
const R_DQT_ESC_SEQ: usize = 52;
const BR_DQT_ESC_SEQ: usize = 53;
const R_DQT_STR_END: usize = 54;
const BR_DQT_STR_END: usize = 55;
const R_SQT_STR_BGN: usize = 56;
const BR_SQT_STR_BGN: usize = 57;
const B_SQT_STR_BGN: usize = 58;
const R_SQT_STR_CTN: usize = 59;
const BR_SQT_STR_CTN: usize = 60;
const R_SQT_ESC_SQT: usize = 61;
const BR_SQT_ESC_SQT: usize = 62;
const R_SQT_STR_END: usize = 63;
const BR_SQT_STR_END: usize = 64;
const R_SGL_PLN_NUL_BLK: usize = 65;
const R_SGL_PLN_BOL_BLK: usize = 70;
const R_SGL_PLN_INT_BLK: usize = 75;
const R_SGL_PLN_FLT_BLK: usize = 80;
const R_SGL_PLN_STR_BLK: usize = 90;
const BR_SGL_PLN_STR_BLK: usize = 91;
const B_SGL_PLN_STR_BLK: usize = 92;
const R_SGL_PLN_STR_FLW: usize = 93;
const BR_SGL_PLN_STR_FLW: usize = 94;
const R_MTL_PLN_STR_BLK: usize = 95;
const BR_MTL_PLN_STR_BLK: usize = 96;
const R_MTL_PLN_STR_FLW: usize = 97;
const BR_MTL_PLN_STR_FLW: usize = 98;
const R_TAG: usize = 99;
const BR_TAG: usize = 100;
const B_TAG: usize = 101;
const R_ACR_BGN: usize = 102;
const BR_ACR_BGN: usize = 103;
const B_ACR_BGN: usize = 104;
const R_ACR_CTN: usize = 105;
const R_ALS_BGN: usize = 106;
const BR_ALS_BGN: usize = 107;
const B_ALS_BGN: usize = 108;
const R_ALS_CTN: usize = 109;
const BL: usize = 110;
const COMMENT: usize = 111;
const ERR_REC: usize = 112;

const IND_ROT: i16 = b'r' as i16;
const IND_MAP: i16 = b'm' as i16;
const IND_SEQ: i16 = b'q' as i16;
const IND_STR: i16 = b's' as i16;
const SCH_STT_FRZ: i8 = -1;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum ResultSchema {
    #[default]
    String,
    Int,
    Null,
    Bool,
    Float,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ScanResult {
    Success,
    Stop,
    Fail,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Indent {
    kind: i16,
    length: i16,
}

/// The two C indentation arrays are kept together; their push/pop order is identical.
#[derive(Default)]
pub(crate) struct Scanner {
    row: i16,
    col: i16,
    blk_imp_row: i16,
    blk_imp_col: i16,
    blk_imp_tab: i16,
    indents: Vec<Indent>,
    // Temporary state, not serialized.
    end_row: i16,
    end_col: i16,
    cur_row: i16,
    cur_col: i16,
    cur_chr: i32,
    // Cached until the next advance; unlike C field reads, Lexer calls are dynamic.
    lookahead: i32,
    sch_stt: i8,
    rlt_sch: ResultSchema,
}

impl Scanner {
    fn adv(&mut self, lexer: &mut dyn Lexer) {
        self.cur_col = self.cur_col.wrapping_add(1);
        self.cur_chr = self.lookahead;
        lexer.advance(false);
        self.lookahead = lexer.lookahead();
    }

    fn adv_nwl(&mut self, lexer: &mut dyn Lexer) {
        self.cur_row = self.cur_row.wrapping_add(1);
        self.cur_col = 0;
        self.cur_chr = self.lookahead;
        lexer.advance(false);
        self.lookahead = lexer.lookahead();
    }

    fn skp(&mut self, lexer: &mut dyn Lexer) {
        self.cur_col = self.cur_col.wrapping_add(1);
        self.cur_chr = self.lookahead;
        lexer.advance(true);
        self.lookahead = lexer.lookahead();
    }

    fn skp_nwl(&mut self, lexer: &mut dyn Lexer) {
        self.cur_row = self.cur_row.wrapping_add(1);
        self.cur_col = 0;
        self.cur_chr = self.lookahead;
        lexer.advance(true);
        self.lookahead = lexer.lookahead();
    }

    fn mrk_end(&mut self, lexer: &mut dyn Lexer) {
        self.end_row = self.cur_row;
        self.end_col = self.cur_col;
        lexer.mark_end();
    }

    fn init(&mut self) {
        self.cur_row = self.row;
        self.cur_col = self.col;
        self.cur_chr = 0;
        self.sch_stt = 0;
        self.rlt_sch = ResultSchema::String;
    }

    fn finish(&mut self, lexer: &mut dyn Lexer, symbol: usize) -> bool {
        self.row = self.end_row;
        self.col = self.end_col;
        lexer.set_result_symbol(symbol as u16);
        true
    }

    fn pop_ind(&mut self) -> bool {
        // Error recovery can attempt to pop the root indentation.
        if self.indents.len() == 1 {
            return false;
        }
        self.indents.pop();
        true
    }

    fn push_ind(&mut self, kind: i16, length: i16) {
        self.indents.push(Indent { kind, length });
    }

    fn may_upd_imp_col(&mut self, row: i16, col: i16, has_tab: bool) {
        if self.blk_imp_row != row {
            self.blk_imp_row = row;
            self.blk_imp_col = col;
            self.blk_imp_tab = i16::from(has_tab);
        }
    }

    // Each schema has five positional/contextual variants, with timestamp slots
    // reserved between float and string even though the core schema has no timestamps.
    fn plain_symbol(&self, position: usize) -> usize {
        let base = match self.rlt_sch {
            ResultSchema::Null => R_SGL_PLN_NUL_BLK,
            ResultSchema::Bool => R_SGL_PLN_BOL_BLK,
            ResultSchema::Int => R_SGL_PLN_INT_BLK,
            ResultSchema::Float => R_SGL_PLN_FLT_BLK,
            ResultSchema::String => R_SGL_PLN_STR_BLK,
        };
        base + position
    }

    fn scn_uri_esc(&mut self, lexer: &mut dyn Lexer) -> ScanResult {
        if self.lookahead != i32::from(b'%') {
            return ScanResult::Stop;
        }
        self.mrk_end(lexer);
        self.adv(lexer);
        if !is_ns_hex_digit(self.lookahead) {
            return ScanResult::Fail;
        }
        self.adv(lexer);
        if !is_ns_hex_digit(self.lookahead) {
            return ScanResult::Fail;
        }
        self.adv(lexer);
        ScanResult::Success
    }

    fn scn_ns_uri_char(&mut self, lexer: &mut dyn Lexer) -> ScanResult {
        if is_ns_uri_char(self.lookahead) {
            self.adv(lexer);
            return ScanResult::Success;
        }
        self.scn_uri_esc(lexer)
    }

    fn scn_ns_tag_char(&mut self, lexer: &mut dyn Lexer) -> ScanResult {
        if is_ns_tag_char(self.lookahead) {
            self.adv(lexer);
            return ScanResult::Success;
        }
        self.scn_uri_esc(lexer)
    }

    fn scn_dir_bgn(&mut self, lexer: &mut dyn Lexer) -> bool {
        self.adv(lexer);
        if self.lookahead == i32::from(b'Y') {
            self.adv(lexer);
            if self.lookahead == i32::from(b'A') {
                self.adv(lexer);
                if self.lookahead == i32::from(b'M') {
                    self.adv(lexer);
                    if self.lookahead == i32::from(b'L') {
                        self.adv(lexer);
                        if is_wht(self.lookahead) {
                            self.mrk_end(lexer);
                            return self.finish(lexer, S_DIR_YML_BGN);
                        }
                    }
                }
            }
        } else if self.lookahead == i32::from(b'T') {
            self.adv(lexer);
            if self.lookahead == i32::from(b'A') {
                self.adv(lexer);
                if self.lookahead == i32::from(b'G') {
                    self.adv(lexer);
                    if is_wht(self.lookahead) {
                        self.mrk_end(lexer);
                        return self.finish(lexer, S_DIR_TAG_BGN);
                    }
                }
            }
        }
        loop {
            if !is_ns_char(self.lookahead) {
                break;
            }
            self.adv(lexer);
        }
        if self.cur_col > 1 && is_wht(self.lookahead) {
            self.mrk_end(lexer);
            return self.finish(lexer, S_DIR_RSV_BGN);
        }
        false
    }

    fn scn_dir_yml_ver(&mut self, lexer: &mut dyn Lexer, result_symbol: usize) -> bool {
        let mut n1: u16 = 0;
        let mut n2: u16 = 0;
        while is_ns_dec_digit(self.lookahead) {
            self.adv(lexer);
            n1 = n1.wrapping_add(1);
        }
        if self.lookahead != i32::from(b'.') {
            return false;
        }
        self.adv(lexer);
        while is_ns_dec_digit(self.lookahead) {
            self.adv(lexer);
            n2 = n2.wrapping_add(1);
        }
        if n1 == 0 || n2 == 0 {
            return false;
        }
        self.mrk_end(lexer);
        self.finish(lexer, result_symbol)
    }

    fn scn_tag_hdl_tal(&mut self, lexer: &mut dyn Lexer) -> bool {
        if self.lookahead == i32::from(b'!') {
            self.adv(lexer);
            return true;
        }
        let mut n: u16 = 0;
        while is_ns_word_char(self.lookahead) {
            self.adv(lexer);
            n = n.wrapping_add(1);
        }
        if n == 0 {
            return true;
        }
        if self.lookahead == i32::from(b'!') {
            self.adv(lexer);
            return true;
        }
        false
    }

    fn scn_dir_tag_hdl(&mut self, lexer: &mut dyn Lexer, result_symbol: usize) -> bool {
        if self.lookahead == i32::from(b'!') {
            self.adv(lexer);
            if self.scn_tag_hdl_tal(lexer) {
                self.mrk_end(lexer);
                return self.finish(lexer, result_symbol);
            }
        }
        false
    }

    fn scn_dir_rsv_prm(&mut self, lexer: &mut dyn Lexer, result_symbol: usize) -> bool {
        if !is_ns_char(self.lookahead) {
            return false;
        }
        self.adv(lexer);
        while is_ns_char(self.lookahead) {
            self.adv(lexer);
        }
        self.mrk_end(lexer);
        self.finish(lexer, result_symbol)
    }

    fn scn_acr_bgn(&mut self, lexer: &mut dyn Lexer, result_symbol: usize) -> bool {
        if self.lookahead != i32::from(b'&') {
            return false;
        }
        self.adv(lexer);
        if !is_ns_anchor_char(self.lookahead) {
            return false;
        }
        self.mrk_end(lexer);
        self.finish(lexer, result_symbol)
    }

    fn scn_acr_ctn(&mut self, lexer: &mut dyn Lexer, result_symbol: usize) -> bool {
        while is_ns_anchor_char(self.lookahead) {
            self.adv(lexer);
        }
        self.mrk_end(lexer);
        self.finish(lexer, result_symbol)
    }

    fn scn_als_bgn(&mut self, lexer: &mut dyn Lexer, result_symbol: usize) -> bool {
        if self.lookahead != i32::from(b'*') {
            return false;
        }
        self.adv(lexer);
        if !is_ns_anchor_char(self.lookahead) {
            return false;
        }
        self.mrk_end(lexer);
        self.finish(lexer, result_symbol)
    }

    fn scn_als_ctn(&mut self, lexer: &mut dyn Lexer, result_symbol: usize) -> bool {
        while is_ns_anchor_char(self.lookahead) {
            self.adv(lexer);
        }
        self.mrk_end(lexer);
        self.finish(lexer, result_symbol)
    }

    fn scn_drs_doc_end(&mut self, lexer: &mut dyn Lexer) -> bool {
        if self.lookahead != i32::from(b'-') && self.lookahead != i32::from(b'.') {
            return false;
        }
        let delimeter: i32 = self.lookahead;
        self.adv(lexer);
        if self.lookahead == delimeter {
            self.adv(lexer);
            if self.lookahead == delimeter {
                self.adv(lexer);
                if is_wht(self.lookahead) {
                    return true;
                }
            }
        }
        self.mrk_end(lexer);
        false
    }

    fn scn_dqt_str_cnt(&mut self, lexer: &mut dyn Lexer, result_symbol: usize) -> bool {
        if !is_nb_double_char(self.lookahead) {
            return false;
        }
        if self.cur_col == 0 && self.scn_drs_doc_end(lexer) {
            self.mrk_end(lexer);
            return self.finish(
                lexer,
                if self.cur_chr == i32::from(b'-') {
                    S_DRS_END
                } else {
                    S_DOC_END
                },
            );
        } else {
            self.adv(lexer);
        }
        while is_nb_double_char(self.lookahead) {
            self.adv(lexer);
        }
        self.mrk_end(lexer);
        self.finish(lexer, result_symbol)
    }

    fn scn_sqt_str_cnt(&mut self, lexer: &mut dyn Lexer, result_symbol: usize) -> bool {
        if !is_nb_single_char(self.lookahead) {
            return false;
        }
        if self.cur_col == 0 && self.scn_drs_doc_end(lexer) {
            self.mrk_end(lexer);
            return self.finish(
                lexer,
                if self.cur_chr == i32::from(b'-') {
                    S_DRS_END
                } else {
                    S_DOC_END
                },
            );
        } else {
            self.adv(lexer);
        }
        while is_nb_single_char(self.lookahead) {
            self.adv(lexer);
        }
        self.mrk_end(lexer);
        self.finish(lexer, result_symbol)
    }

    fn scn_blk_str_bgn(&mut self, lexer: &mut dyn Lexer, result_symbol: usize) -> bool {
        if self.lookahead != i32::from(b'|') && self.lookahead != i32::from(b'>') {
            return false;
        }
        self.adv(lexer);
        let cur_ind: i16 = self.indents.last().unwrap().length;
        let mut ind: i16 = -1;
        if self.lookahead >= i32::from(b'1') && self.lookahead <= i32::from(b'9') {
            ind = (self.lookahead - i32::from(b'1')) as i16;
            self.adv(lexer);
            if self.lookahead == i32::from(b'+') || self.lookahead == i32::from(b'-') {
                self.adv(lexer);
            }
        } else if self.lookahead == i32::from(b'+') || self.lookahead == i32::from(b'-') {
            self.adv(lexer);
            if self.lookahead >= i32::from(b'1') && self.lookahead <= i32::from(b'9') {
                ind = (self.lookahead - i32::from(b'1')) as i16;
                self.adv(lexer);
            }
        }
        if !is_wht(self.lookahead) {
            return false;
        }
        self.mrk_end(lexer);
        if ind != -1 {
            ind = ind.wrapping_add(cur_ind);
        } else {
            ind = cur_ind;
            while is_wsp(self.lookahead) {
                self.adv(lexer);
            }
            if self.lookahead == i32::from(b'#') {
                self.adv(lexer);
                while !is_nwl(self.lookahead) && self.lookahead != 0 {
                    self.adv(lexer);
                }
            }
            if is_nwl(self.lookahead) {
                self.adv_nwl(lexer);
            }
            while self.lookahead != 0 {
                if self.lookahead == i32::from(b' ') {
                    self.adv(lexer);
                } else if is_nwl(self.lookahead) {
                    if i32::from(self.cur_col) - 1 < i32::from(ind) {
                        break;
                    }
                    ind = self.cur_col.wrapping_sub(1);
                    self.adv_nwl(lexer);
                } else {
                    if i32::from(self.cur_col) - 1 > i32::from(ind) {
                        ind = self.cur_col.wrapping_sub(1);
                    }
                    break;
                }
            }
        }
        self.push_ind(IND_STR, ind);
        self.finish(lexer, result_symbol)
    }

    fn scn_blk_str_cnt(&mut self, lexer: &mut dyn Lexer, result_symbol: usize) -> bool {
        if !is_ns_char(self.lookahead) {
            return false;
        }
        if self.cur_col == 0 && self.scn_drs_doc_end(lexer) {
            if !self.pop_ind() {
                return false;
            }
            return self.finish(lexer, BL);
        } else {
            self.adv(lexer);
        }
        self.mrk_end(lexer);
        loop {
            if is_ns_char(self.lookahead) {
                self.adv(lexer);
                while is_ns_char(self.lookahead) {
                    self.adv(lexer);
                }
                self.mrk_end(lexer);
            }
            if is_wsp(self.lookahead) {
                self.adv(lexer);
                while is_wsp(self.lookahead) {
                    self.adv(lexer);
                }
            } else {
                break;
            }
        }
        self.finish(lexer, result_symbol)
    }

    fn scn_pln_cnt(&mut self, lexer: &mut dyn Lexer, is_in_blk: bool) -> ScanResult {
        let is_plain_safe = |c| is_plain_safe(c, is_in_blk);
        let mut is_cur_saf: bool = is_plain_safe(self.cur_chr);
        let mut is_lka_wsp: bool = is_wsp(self.lookahead);
        let mut is_lka_saf: bool = is_plain_safe(self.lookahead);
        if is_lka_saf || is_lka_wsp {
            loop {
                if (is_lka_saf
                    && self.lookahead != i32::from(b'#')
                    && self.lookahead != i32::from(b':'))
                    || (is_cur_saf && self.lookahead == i32::from(b'#'))
                {
                    self.adv(lexer);
                    self.mrk_end(lexer);
                    self.sch_stt = advance_schema(self.sch_stt, self.cur_chr, &mut self.rlt_sch);
                } else if is_lka_wsp {
                    self.adv(lexer);
                    self.sch_stt = advance_schema(self.sch_stt, self.cur_chr, &mut self.rlt_sch);
                } else if self.lookahead == i32::from(b':') {
                    self.adv(lexer); // check later
                } else {
                    break;
                }
                is_cur_saf = is_lka_saf;
                is_lka_wsp = is_wsp(self.lookahead);
                is_lka_saf = is_plain_safe(self.lookahead);

                if self.cur_chr == i32::from(b':') {
                    if is_lka_saf {
                        self.mrk_end(lexer);
                        self.sch_stt =
                            advance_schema(self.sch_stt, self.cur_chr, &mut self.rlt_sch);
                    } else {
                        return ScanResult::Fail;
                    }
                }
            }
        } else {
            return ScanResult::Stop;
        }
        ScanResult::Success
    }
}

fn is_wsp(c: i32) -> bool {
    c == i32::from(b' ') || c == i32::from(b'\t')
}

fn is_nwl(c: i32) -> bool {
    c == i32::from(b'\r') || c == i32::from(b'\n')
}

fn is_wht(c: i32) -> bool {
    is_wsp(c) || is_nwl(c) || c == 0
}

fn is_ns_dec_digit(c: i32) -> bool {
    c >= i32::from(b'0') && c <= i32::from(b'9')
}

fn is_ns_hex_digit(c: i32) -> bool {
    is_ns_dec_digit(c)
        || (c >= i32::from(b'a') && c <= i32::from(b'f'))
        || (c >= i32::from(b'A') && c <= i32::from(b'F'))
}

fn is_ns_word_char(c: i32) -> bool {
    c == i32::from(b'-')
        || (c >= i32::from(b'0') && c <= i32::from(b'9'))
        || (c >= i32::from(b'a') && c <= i32::from(b'z'))
        || (c >= i32::from(b'A') && c <= i32::from(b'Z'))
}

fn is_nb_json(c: i32) -> bool {
    c == 0x09 || (0x20..=0x10ffff).contains(&c)
}

fn is_nb_double_char(c: i32) -> bool {
    is_nb_json(c) && c != i32::from(b'\\') && c != i32::from(b'"')
}

fn is_nb_single_char(c: i32) -> bool {
    is_nb_json(c) && c != i32::from(b'\'')
}

fn is_ns_char(c: i32) -> bool {
    (0x21..=0x7e).contains(&c)
        || c == 0x85
        || (0xa0..=0xd7ff).contains(&c)
        || (0xe000..=0xfefe).contains(&c)
        || (0xff00..=0xfffd).contains(&c)
        || (0x10000..=0x10ffff).contains(&c)
}

fn is_c_indicator(c: i32) -> bool {
    c == i32::from(b'-')
        || c == i32::from(b'?')
        || c == i32::from(b':')
        || c == i32::from(b',')
        || c == i32::from(b'[')
        || c == i32::from(b']')
        || c == i32::from(b'{')
        || c == i32::from(b'}')
        || c == i32::from(b'#')
        || c == i32::from(b'&')
        || c == i32::from(b'*')
        || c == i32::from(b'!')
        || c == i32::from(b'|')
        || c == i32::from(b'>')
        || c == i32::from(b'\'')
        || c == i32::from(b'"')
        || c == i32::from(b'%')
        || c == i32::from(b'@')
        || c == i32::from(b'`')
}

fn is_c_flow_indicator(c: i32) -> bool {
    c == i32::from(b',')
        || c == i32::from(b'[')
        || c == i32::from(b']')
        || c == i32::from(b'{')
        || c == i32::from(b'}')
}

fn is_plain_safe(c: i32, is_in_blk: bool) -> bool {
    is_ns_char(c) && (is_in_blk || !is_c_flow_indicator(c))
}

fn is_ns_uri_char(c: i32) -> bool {
    is_ns_word_char(c)
        || c == i32::from(b'#')
        || c == i32::from(b';')
        || c == i32::from(b'/')
        || c == i32::from(b'?')
        || c == i32::from(b':')
        || c == i32::from(b'@')
        || c == i32::from(b'&')
        || c == i32::from(b'=')
        || c == i32::from(b'+')
        || c == i32::from(b'$')
        || c == i32::from(b',')
        || c == i32::from(b'_')
        || c == i32::from(b'.')
        || c == i32::from(b'!')
        || c == i32::from(b'~')
        || c == i32::from(b'*')
        || c == i32::from(b'\'')
        || c == i32::from(b'(')
        || c == i32::from(b')')
        || c == i32::from(b'[')
        || c == i32::from(b']')
}

fn is_ns_tag_char(c: i32) -> bool {
    is_ns_word_char(c)
        || c == i32::from(b'#')
        || c == i32::from(b';')
        || c == i32::from(b'/')
        || c == i32::from(b'?')
        || c == i32::from(b':')
        || c == i32::from(b'@')
        || c == i32::from(b'&')
        || c == i32::from(b'=')
        || c == i32::from(b'+')
        || c == i32::from(b'$')
        || c == i32::from(b'_')
        || c == i32::from(b'.')
        || c == i32::from(b'~')
        || c == i32::from(b'*')
        || c == i32::from(b'\'')
        || c == i32::from(b'(')
        || c == i32::from(b')')
}

fn is_ns_anchor_char(c: i32) -> bool {
    is_ns_char(c) && !is_c_flow_indicator(c)
}

impl ExternalScanner for Scanner {
    fn scan(&mut self, lexer: &mut dyn Lexer, valid_symbols: &[bool]) -> bool {
        self.init();
        // The runtime can rewind between scans. Only cache within this scan;
        // the advance helpers refresh the value at every subsequent position.
        self.lookahead = lexer.lookahead();
        self.mrk_end(lexer);

        let allow_comment: bool = !(valid_symbols[R_DQT_STR_CTN]
            || valid_symbols[BR_DQT_STR_CTN]
            || valid_symbols[R_SQT_STR_CTN]
            || valid_symbols[BR_SQT_STR_CTN]);
        let current = *self.indents.last().unwrap();
        let cur_ind = current.length;
        let prt_ind = self
            .indents
            .iter()
            .rev()
            .nth(1)
            .map_or(-1, |indent| indent.length);
        let cur_ind_typ = current.kind;

        let mut has_tab_ind: bool = false;
        let mut leading_spaces: i16 = 0;

        loop {
            if self.lookahead == i32::from(b' ') {
                if !has_tab_ind {
                    leading_spaces = leading_spaces.wrapping_add(1);
                }
                self.skp(lexer);
            } else if self.lookahead == i32::from(b'\t') {
                has_tab_ind = true;
                self.skp(lexer);
            } else if is_nwl(self.lookahead) {
                has_tab_ind = false;
                leading_spaces = 0;
                self.skp_nwl(lexer);
            } else if allow_comment && self.lookahead == i32::from(b'#') {
                if valid_symbols[BR_BLK_STR_CTN] && valid_symbols[BL] && self.cur_col <= cur_ind {
                    if !self.pop_ind() {
                        return false;
                    }
                    return self.finish(lexer, BL);
                }
                if if valid_symbols[BR_BLK_STR_CTN] {
                    self.cur_row == self.row
                } else {
                    self.cur_col == 0 || self.cur_row != self.row || self.cur_col > self.col
                } {
                    self.adv(lexer);
                    while !is_nwl(self.lookahead) && self.lookahead != 0 {
                        self.adv(lexer);
                    }
                    self.mrk_end(lexer);
                    return self.finish(lexer, COMMENT);
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if self.lookahead == 0 {
            if valid_symbols[BL] {
                self.mrk_end(lexer);
                if !self.pop_ind() {
                    return false;
                }
                return self.finish(lexer, BL);
            }
            if valid_symbols[END_OF_FILE] {
                self.mrk_end(lexer);
                return self.finish(lexer, END_OF_FILE);
            }
            return false;
        }

        let bgn_row: i16 = self.cur_row;
        let bgn_col: i16 = self.cur_col;
        let bgn_chr: i32 = self.lookahead;

        if valid_symbols[BL]
            && bgn_col <= cur_ind
            && !has_tab_ind
            && if cur_ind == prt_ind && cur_ind_typ == IND_SEQ {
                bgn_col < cur_ind || self.lookahead != i32::from(b'-')
            } else {
                bgn_col <= prt_ind || cur_ind_typ == IND_STR
            }
        {
            if !self.pop_ind() {
                return false;
            }
            return self.finish(lexer, BL);
        }

        let has_nwl: bool = self.cur_row > self.row;
        let is_r: bool = !has_nwl;
        let is_br: bool = has_nwl && leading_spaces > cur_ind;
        let is_b: bool = has_nwl && leading_spaces == cur_ind && !has_tab_ind;
        let is_s: bool = bgn_col == 0;

        if valid_symbols[R_DIR_YML_VER] && is_r {
            return self.scn_dir_yml_ver(lexer, R_DIR_YML_VER);
        }
        if valid_symbols[R_DIR_TAG_HDL] && is_r {
            return self.scn_dir_tag_hdl(lexer, R_DIR_TAG_HDL);
        }
        if valid_symbols[R_DIR_TAG_PFX] && is_r {
            return self.scn_dir_tag_pfx(lexer, R_DIR_TAG_PFX);
        }
        if valid_symbols[R_DIR_RSV_PRM] && is_r {
            return self.scn_dir_rsv_prm(lexer, R_DIR_RSV_PRM);
        }
        if valid_symbols[BR_BLK_STR_CTN] && is_br && self.scn_blk_str_cnt(lexer, BR_BLK_STR_CTN) {
            return true;
        }

        if (valid_symbols[R_DQT_STR_CTN] && is_r && self.scn_dqt_str_cnt(lexer, R_DQT_STR_CTN))
            || (valid_symbols[BR_DQT_STR_CTN]
                && is_br
                && self.scn_dqt_str_cnt(lexer, BR_DQT_STR_CTN))
        {
            return true;
        }

        if (valid_symbols[R_SQT_STR_CTN] && is_r && self.scn_sqt_str_cnt(lexer, R_SQT_STR_CTN))
            || (valid_symbols[BR_SQT_STR_CTN]
                && is_br
                && self.scn_sqt_str_cnt(lexer, BR_SQT_STR_CTN))
        {
            return true;
        }

        if valid_symbols[R_ACR_CTN] && is_r {
            return self.scn_acr_ctn(lexer, R_ACR_CTN);
        }
        if valid_symbols[R_ALS_CTN] && is_r {
            return self.scn_als_ctn(lexer, R_ALS_CTN);
        }

        if self.lookahead == i32::from(b'%') {
            if valid_symbols[S_DIR_YML_BGN] && is_s {
                return self.scn_dir_bgn(lexer);
            }
        } else if self.lookahead == i32::from(b'*') {
            if valid_symbols[R_ALS_BGN] && is_r {
                self.may_upd_imp_col(bgn_row, bgn_col, has_tab_ind);
                return self.scn_als_bgn(lexer, R_ALS_BGN);
            }
            if valid_symbols[BR_ALS_BGN] && is_br {
                self.may_upd_imp_col(bgn_row, bgn_col, has_tab_ind);
                return self.scn_als_bgn(lexer, BR_ALS_BGN);
            }
            if valid_symbols[B_ALS_BGN] && is_b {
                self.may_upd_imp_col(bgn_row, bgn_col, has_tab_ind);
                return self.scn_als_bgn(lexer, B_ALS_BGN);
            }
        } else if self.lookahead == i32::from(b'&') {
            if valid_symbols[R_ACR_BGN] && is_r {
                self.may_upd_imp_col(bgn_row, bgn_col, has_tab_ind);
                return self.scn_acr_bgn(lexer, R_ACR_BGN);
            }
            if valid_symbols[BR_ACR_BGN] && is_br {
                self.may_upd_imp_col(bgn_row, bgn_col, has_tab_ind);
                return self.scn_acr_bgn(lexer, BR_ACR_BGN);
            }
            if valid_symbols[B_ACR_BGN] && is_b {
                self.may_upd_imp_col(bgn_row, bgn_col, has_tab_ind);
                return self.scn_acr_bgn(lexer, B_ACR_BGN);
            }
        } else if self.lookahead == i32::from(b'!') {
            if valid_symbols[R_TAG] && is_r {
                self.may_upd_imp_col(bgn_row, bgn_col, has_tab_ind);
                return self.scn_tag(lexer, R_TAG);
            }
            if valid_symbols[BR_TAG] && is_br {
                self.may_upd_imp_col(bgn_row, bgn_col, has_tab_ind);
                return self.scn_tag(lexer, BR_TAG);
            }
            if valid_symbols[B_TAG] && is_b {
                self.may_upd_imp_col(bgn_row, bgn_col, has_tab_ind);
                return self.scn_tag(lexer, B_TAG);
            }
        } else if self.lookahead == i32::from(b'[') {
            if valid_symbols[R_FLW_SEQ_BGN] && is_r {
                self.may_upd_imp_col(bgn_row, bgn_col, has_tab_ind);
                self.adv(lexer);
                self.mrk_end(lexer);
                return self.finish(lexer, R_FLW_SEQ_BGN);
            }
            if valid_symbols[BR_FLW_SEQ_BGN] && is_br {
                self.may_upd_imp_col(bgn_row, bgn_col, has_tab_ind);
                self.adv(lexer);
                self.mrk_end(lexer);
                return self.finish(lexer, BR_FLW_SEQ_BGN);
            }
            if valid_symbols[B_FLW_SEQ_BGN] && is_b {
                self.may_upd_imp_col(bgn_row, bgn_col, has_tab_ind);
                self.adv(lexer);
                self.mrk_end(lexer);
                return self.finish(lexer, B_FLW_SEQ_BGN);
            }
        } else if self.lookahead == i32::from(b']') {
            if valid_symbols[R_FLW_SEQ_END] && is_r {
                self.adv(lexer);
                self.mrk_end(lexer);
                return self.finish(lexer, R_FLW_SEQ_END);
            }
            if valid_symbols[BR_FLW_SEQ_END] && is_br {
                self.adv(lexer);
                self.mrk_end(lexer);
                return self.finish(lexer, BR_FLW_SEQ_END);
            }
            if valid_symbols[B_FLW_SEQ_END] && is_b {
                self.adv(lexer);
                self.mrk_end(lexer);
                return self.finish(lexer, BR_FLW_SEQ_END);
            }
        } else if self.lookahead == i32::from(b'{') {
            if valid_symbols[R_FLW_MAP_BGN] && is_r {
                self.may_upd_imp_col(bgn_row, bgn_col, has_tab_ind);
                self.adv(lexer);
                self.mrk_end(lexer);
                return self.finish(lexer, R_FLW_MAP_BGN);
            }
            if valid_symbols[BR_FLW_MAP_BGN] && is_br {
                self.may_upd_imp_col(bgn_row, bgn_col, has_tab_ind);
                self.adv(lexer);
                self.mrk_end(lexer);
                return self.finish(lexer, BR_FLW_MAP_BGN);
            }
            if valid_symbols[B_FLW_MAP_BGN] && is_b {
                self.may_upd_imp_col(bgn_row, bgn_col, has_tab_ind);
                self.adv(lexer);
                self.mrk_end(lexer);
                return self.finish(lexer, B_FLW_MAP_BGN);
            }
        } else if self.lookahead == i32::from(b'}') {
            if valid_symbols[R_FLW_MAP_END] && is_r {
                self.adv(lexer);
                self.mrk_end(lexer);
                return self.finish(lexer, R_FLW_MAP_END);
            }
            if valid_symbols[BR_FLW_MAP_END] && is_br {
                self.adv(lexer);
                self.mrk_end(lexer);
                return self.finish(lexer, BR_FLW_MAP_END);
            }
            if valid_symbols[B_FLW_MAP_END] && is_b {
                self.adv(lexer);
                self.mrk_end(lexer);
                return self.finish(lexer, BR_FLW_MAP_END);
            }
        } else if self.lookahead == i32::from(b',') {
            if valid_symbols[R_FLW_SEP_BGN] && is_r {
                self.adv(lexer);
                self.mrk_end(lexer);
                return self.finish(lexer, R_FLW_SEP_BGN);
            }
            if valid_symbols[BR_FLW_SEP_BGN] && is_br {
                self.adv(lexer);
                self.mrk_end(lexer);
                return self.finish(lexer, BR_FLW_SEP_BGN);
            }
        } else if self.lookahead == i32::from(b'"') {
            if valid_symbols[R_DQT_STR_BGN] && is_r {
                self.may_upd_imp_col(bgn_row, bgn_col, has_tab_ind);
                self.adv(lexer);
                self.mrk_end(lexer);
                return self.finish(lexer, R_DQT_STR_BGN);
            }
            if valid_symbols[BR_DQT_STR_BGN] && is_br {
                self.may_upd_imp_col(bgn_row, bgn_col, has_tab_ind);
                self.adv(lexer);
                self.mrk_end(lexer);
                return self.finish(lexer, BR_DQT_STR_BGN);
            }
            if valid_symbols[B_DQT_STR_BGN] && is_b {
                self.may_upd_imp_col(bgn_row, bgn_col, has_tab_ind);
                self.adv(lexer);
                self.mrk_end(lexer);
                return self.finish(lexer, B_DQT_STR_BGN);
            }
            if valid_symbols[R_DQT_STR_END] && is_r {
                self.adv(lexer);
                self.mrk_end(lexer);
                return self.finish(lexer, R_DQT_STR_END);
            }
            if valid_symbols[BR_DQT_STR_END] && is_br {
                self.adv(lexer);
                self.mrk_end(lexer);
                return self.finish(lexer, BR_DQT_STR_END);
            }
        } else if self.lookahead == i32::from(b'\'') {
            if valid_symbols[R_SQT_STR_BGN] && is_r {
                self.may_upd_imp_col(bgn_row, bgn_col, has_tab_ind);
                self.adv(lexer);
                self.mrk_end(lexer);
                return self.finish(lexer, R_SQT_STR_BGN);
            }
            if valid_symbols[BR_SQT_STR_BGN] && is_br {
                self.may_upd_imp_col(bgn_row, bgn_col, has_tab_ind);
                self.adv(lexer);
                self.mrk_end(lexer);
                return self.finish(lexer, BR_SQT_STR_BGN);
            }
            if valid_symbols[B_SQT_STR_BGN] && is_b {
                self.may_upd_imp_col(bgn_row, bgn_col, has_tab_ind);
                self.adv(lexer);
                self.mrk_end(lexer);
                return self.finish(lexer, B_SQT_STR_BGN);
            }
            if valid_symbols[R_SQT_STR_END] && is_r {
                self.adv(lexer);
                if self.lookahead == i32::from(b'\'') {
                    self.adv(lexer);
                    self.mrk_end(lexer);
                    return self.finish(lexer, R_SQT_ESC_SQT);
                } else {
                    self.mrk_end(lexer);
                    return self.finish(lexer, R_SQT_STR_END);
                }
            }
            if valid_symbols[BR_SQT_STR_END] && is_br {
                self.adv(lexer);
                if self.lookahead == i32::from(b'\'') {
                    self.adv(lexer);
                    self.mrk_end(lexer);
                    return self.finish(lexer, BR_SQT_ESC_SQT);
                } else {
                    self.mrk_end(lexer);
                    return self.finish(lexer, BR_SQT_STR_END);
                }
            }
        } else if self.lookahead == i32::from(b'?') {
            let is_r_blk_key_bgn: bool = valid_symbols[R_BLK_KEY_BGN] && is_r;
            let is_br_blk_key_bgn: bool = valid_symbols[BR_BLK_KEY_BGN] && is_br;
            let is_b_blk_key_bgn: bool = valid_symbols[B_BLK_KEY_BGN] && is_b;
            let is_r_flw_key_bgn: bool = valid_symbols[R_FLW_KEY_BGN] && is_r;
            let is_br_flw_key_bgn: bool = valid_symbols[BR_FLW_KEY_BGN] && is_br;
            if is_r_blk_key_bgn
                || is_br_blk_key_bgn
                || is_b_blk_key_bgn
                || is_r_flw_key_bgn
                || is_br_flw_key_bgn
            {
                self.adv(lexer);
                if is_wht(self.lookahead) {
                    self.mrk_end(lexer);
                    if is_r_blk_key_bgn {
                        if has_tab_ind {
                            return false;
                        }
                        self.push_ind(IND_MAP, bgn_col);
                        return self.finish(lexer, R_BLK_KEY_BGN);
                    }
                    if is_br_blk_key_bgn {
                        if has_tab_ind {
                            return false;
                        }
                        self.push_ind(IND_MAP, bgn_col);
                        return self.finish(lexer, BR_BLK_KEY_BGN);
                    }
                    if is_b_blk_key_bgn {
                        return self.finish(lexer, B_BLK_KEY_BGN);
                    }
                    if is_r_flw_key_bgn {
                        return self.finish(lexer, R_FLW_KEY_BGN);
                    }
                    if is_br_flw_key_bgn {
                        return self.finish(lexer, BR_FLW_KEY_BGN);
                    }
                }
            }
        } else if self.lookahead == i32::from(b':') {
            if valid_symbols[R_FLW_JSV_BGN] && is_r {
                self.adv(lexer);
                self.mrk_end(lexer);
                return self.finish(lexer, R_FLW_JSV_BGN);
            }
            if valid_symbols[BR_FLW_JSV_BGN] && is_br {
                self.adv(lexer);
                self.mrk_end(lexer);
                return self.finish(lexer, BR_FLW_JSV_BGN);
            }
            let is_r_blk_val_bgn: bool = valid_symbols[R_BLK_VAL_BGN] && is_r;
            let is_br_blk_val_bgn: bool = valid_symbols[BR_BLK_VAL_BGN] && is_br;
            let is_b_blk_val_bgn: bool = valid_symbols[B_BLK_VAL_BGN] && is_b;
            let is_r_blk_imp_bgn: bool = valid_symbols[R_BLK_IMP_BGN] && is_r;
            let is_r_flw_njv_bgn: bool = valid_symbols[R_FLW_NJV_BGN] && is_r;
            let is_br_flw_njv_bgn: bool = valid_symbols[BR_FLW_NJV_BGN] && is_br;
            if is_r_blk_val_bgn
                || is_br_blk_val_bgn
                || is_b_blk_val_bgn
                || is_r_blk_imp_bgn
                || is_r_flw_njv_bgn
                || is_br_flw_njv_bgn
            {
                self.adv(lexer);
                let is_lka_wht: bool = is_wht(self.lookahead);
                if is_lka_wht {
                    if is_r_blk_val_bgn {
                        if has_tab_ind {
                            return false;
                        }
                        self.push_ind(IND_MAP, bgn_col);
                        self.mrk_end(lexer);
                        return self.finish(lexer, R_BLK_VAL_BGN);
                    }
                    if is_br_blk_val_bgn {
                        if has_tab_ind {
                            return false;
                        }
                        self.push_ind(IND_MAP, bgn_col);
                        self.mrk_end(lexer);
                        return self.finish(lexer, BR_BLK_VAL_BGN);
                    }
                    if is_b_blk_val_bgn {
                        self.mrk_end(lexer);
                        return self.finish(lexer, B_BLK_VAL_BGN);
                    }
                    if is_r_blk_imp_bgn {
                        if cur_ind != self.blk_imp_col {
                            if self.blk_imp_tab != 0 {
                                return false;
                            }
                            self.push_ind(IND_MAP, self.blk_imp_col);
                        }
                        self.mrk_end(lexer);
                        return self.finish(lexer, R_BLK_IMP_BGN);
                    }
                }
                if is_lka_wht
                    || self.lookahead == i32::from(b',')
                    || self.lookahead == i32::from(b']')
                    || self.lookahead == i32::from(b'}')
                {
                    if is_r_flw_njv_bgn {
                        self.mrk_end(lexer);
                        return self.finish(lexer, R_FLW_NJV_BGN);
                    }
                    if is_br_flw_njv_bgn {
                        self.mrk_end(lexer);
                        return self.finish(lexer, BR_FLW_NJV_BGN);
                    }
                }
            }
        } else if self.lookahead == i32::from(b'-') {
            let is_r_blk_seq_bgn: bool = valid_symbols[R_BLK_SEQ_BGN] && is_r;
            let is_br_blk_seq_bgn: bool = valid_symbols[BR_BLK_SEQ_BGN] && is_br;
            let is_b_blk_seq_bgn: bool = valid_symbols[B_BLK_SEQ_BGN] && is_b;
            let is_s_drs_end: bool = is_s;
            if is_r_blk_seq_bgn || is_br_blk_seq_bgn || is_b_blk_seq_bgn || is_s_drs_end {
                self.adv(lexer);
                if is_wht(self.lookahead) {
                    if is_r_blk_seq_bgn {
                        if has_tab_ind {
                            return false;
                        }
                        self.push_ind(IND_SEQ, bgn_col);
                        self.mrk_end(lexer);
                        return self.finish(lexer, R_BLK_SEQ_BGN);
                    }
                    if is_br_blk_seq_bgn {
                        if has_tab_ind {
                            return false;
                        }
                        self.push_ind(IND_SEQ, bgn_col);
                        self.mrk_end(lexer);
                        return self.finish(lexer, BR_BLK_SEQ_BGN);
                    }
                    if is_b_blk_seq_bgn {
                        if cur_ind_typ == IND_MAP {
                            self.push_ind(IND_SEQ, bgn_col);
                        }
                        self.mrk_end(lexer);
                        return self.finish(lexer, B_BLK_SEQ_BGN);
                    }
                } else if self.lookahead == i32::from(b'-') && is_s_drs_end {
                    self.adv(lexer);
                    if self.lookahead == i32::from(b'-') {
                        self.adv(lexer);
                        if is_wht(self.lookahead) {
                            if valid_symbols[BL] {
                                if !self.pop_ind() {
                                    return false;
                                }
                                return self.finish(lexer, BL);
                            }
                            self.mrk_end(lexer);
                            return self.finish(lexer, S_DRS_END);
                        }
                    }
                }
            }
        } else if self.lookahead == i32::from(b'.') {
            if is_s {
                self.adv(lexer);
                if self.lookahead == i32::from(b'.') {
                    self.adv(lexer);
                    if self.lookahead == i32::from(b'.') {
                        self.adv(lexer);
                        if is_wht(self.lookahead) {
                            if valid_symbols[BL] {
                                if !self.pop_ind() {
                                    return false;
                                }
                                return self.finish(lexer, BL);
                            }
                            self.mrk_end(lexer);
                            return self.finish(lexer, S_DOC_END);
                        }
                    }
                }
            }
        } else if self.lookahead == i32::from(b'\\') {
            let is_r_dqt_esc_nwl: bool = valid_symbols[R_DQT_ESC_NWL] && is_r;
            let is_br_dqt_esc_nwl: bool = valid_symbols[BR_DQT_ESC_NWL] && is_br;
            let is_r_dqt_esc_seq: bool = valid_symbols[R_DQT_ESC_SEQ] && is_r;
            let is_br_dqt_esc_seq: bool = valid_symbols[BR_DQT_ESC_SEQ] && is_br;
            if is_r_dqt_esc_nwl || is_br_dqt_esc_nwl || is_r_dqt_esc_seq || is_br_dqt_esc_seq {
                self.adv(lexer);
                if is_nwl(self.lookahead) {
                    if is_r_dqt_esc_nwl {
                        self.mrk_end(lexer);
                        return self.finish(lexer, R_DQT_ESC_NWL);
                    }
                    if is_br_dqt_esc_nwl {
                        self.mrk_end(lexer);
                        return self.finish(lexer, BR_DQT_ESC_NWL);
                    }
                }
                if is_r_dqt_esc_seq {
                    return self.scn_dqt_esc_seq(lexer, R_DQT_ESC_SEQ);
                }
                if is_br_dqt_esc_seq {
                    return self.scn_dqt_esc_seq(lexer, BR_DQT_ESC_SEQ);
                }
                return false;
            }
        } else if self.lookahead == i32::from(b'|') {
            if valid_symbols[R_BLK_LIT_BGN] && is_r {
                return self.scn_blk_str_bgn(lexer, R_BLK_LIT_BGN);
            }
            if valid_symbols[BR_BLK_LIT_BGN] && is_br {
                return self.scn_blk_str_bgn(lexer, BR_BLK_LIT_BGN);
            }
        } else if self.lookahead == i32::from(b'>') {
            if valid_symbols[R_BLK_FLD_BGN] && is_r {
                return self.scn_blk_str_bgn(lexer, R_BLK_FLD_BGN);
            }
            if valid_symbols[BR_BLK_FLD_BGN] && is_br {
                return self.scn_blk_str_bgn(lexer, BR_BLK_FLD_BGN);
            }
        }

        let maybe_sgl_pln_blk: bool = (valid_symbols[R_SGL_PLN_STR_BLK] && is_r)
            || (valid_symbols[BR_SGL_PLN_STR_BLK] && is_br)
            || (valid_symbols[B_SGL_PLN_STR_BLK] && is_b);
        let maybe_sgl_pln_flw: bool = (valid_symbols[R_SGL_PLN_STR_FLW] && is_r)
            || (valid_symbols[BR_SGL_PLN_STR_FLW] && is_br);
        let maybe_mtl_pln_blk: bool = (valid_symbols[R_MTL_PLN_STR_BLK] && is_r)
            || (valid_symbols[BR_MTL_PLN_STR_BLK] && is_br);
        let maybe_mtl_pln_flw: bool = (valid_symbols[R_MTL_PLN_STR_FLW] && is_r)
            || (valid_symbols[BR_MTL_PLN_STR_FLW] && is_br);

        if maybe_sgl_pln_blk || maybe_sgl_pln_flw || maybe_mtl_pln_blk || maybe_mtl_pln_flw {
            let is_in_blk: bool = maybe_sgl_pln_blk || maybe_mtl_pln_blk;
            let is_plain_safe = |c| is_plain_safe(c, is_in_blk);
            if (i32::from(self.cur_col) - i32::from(bgn_col)) == 0 {
                self.adv(lexer);
            }
            if (i32::from(self.cur_col) - i32::from(bgn_col)) == 1 {
                let is_plain_first: bool = (is_ns_char(bgn_chr) && !is_c_indicator(bgn_chr))
                    || ((bgn_chr == i32::from(b'-')
                        || bgn_chr == i32::from(b'?')
                        || bgn_chr == i32::from(b':'))
                        && is_plain_safe(self.lookahead));
                if !is_plain_first {
                    return false;
                }
                self.sch_stt = advance_schema(self.sch_stt, self.cur_chr, &mut self.rlt_sch);
            } else {
                // no need to check the following cases:
                // ..X
                // ...X
                // --X
                // ---X
                // X: lookahead
                self.sch_stt = SCH_STT_FRZ; // must be ResultSchema::String
            }

            self.mrk_end(lexer);

            loop {
                if !is_nwl(self.lookahead)
                    && self.scn_pln_cnt(lexer, is_in_blk) != ScanResult::Success
                {
                    break;
                }
                if self.lookahead == 0 || !is_nwl(self.lookahead) {
                    break;
                }
                loop {
                    if is_nwl(self.lookahead) {
                        self.adv_nwl(lexer);
                    } else if is_wsp(self.lookahead) {
                        self.adv(lexer);
                    } else {
                        break;
                    }
                }
                if self.lookahead == 0 || self.cur_col <= cur_ind {
                    break;
                }
                if self.cur_col == 0 && self.scn_drs_doc_end(lexer) {
                    break;
                }
            }

            if self.end_row == bgn_row {
                if maybe_sgl_pln_blk {
                    self.may_upd_imp_col(bgn_row, bgn_col, has_tab_ind);
                    return self.finish(
                        lexer,
                        self.plain_symbol(if is_r {
                            0
                        } else if is_br {
                            1
                        } else {
                            2
                        }),
                    );
                }
                if maybe_sgl_pln_flw {
                    return self.finish(lexer, self.plain_symbol(if is_r { 3 } else { 4 }));
                }
            } else {
                if maybe_mtl_pln_blk {
                    self.may_upd_imp_col(bgn_row, bgn_col, has_tab_ind);
                    return self.finish(
                        lexer,
                        if is_r {
                            R_MTL_PLN_STR_BLK
                        } else {
                            BR_MTL_PLN_STR_BLK
                        },
                    );
                }
                if maybe_mtl_pln_flw {
                    return self.finish(
                        lexer,
                        if is_r {
                            R_MTL_PLN_STR_FLW
                        } else {
                            BR_MTL_PLN_STR_FLW
                        },
                    );
                }
            }

            return false;
        }

        !valid_symbols[ERR_REC]
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        let capacity = buffer.len().min(SERIALIZATION_BUFFER_SIZE);
        if capacity < 10 {
            return 0;
        }
        let mut size = 0;
        for value in [
            self.row,
            self.col,
            self.blk_imp_row,
            self.blk_imp_col,
            self.blk_imp_tab,
        ] {
            buffer[size..size + 2].copy_from_slice(&value.to_ne_bytes());
            size += 2;
        }
        for indent in self.indents.iter().skip(1) {
            // C checks only size < 1024 and can write a final pair at 1022,
            // overrunning its buffer by two bytes. Keep complete pairs within
            // the supplied buffer; all defined C serializations are identical.
            if size + 4 > capacity {
                break;
            }
            buffer[size..size + 2].copy_from_slice(&indent.kind.to_ne_bytes());
            buffer[size + 2..size + 4].copy_from_slice(&indent.length.to_ne_bytes());
            size += 4;
        }
        size
    }

    fn deserialize(&mut self, buffer: &[u8]) {
        self.row = 0;
        self.col = 0;
        self.blk_imp_row = -1;
        self.blk_imp_col = -1;
        self.blk_imp_tab = 0;
        self.indents.clear();
        self.push_ind(IND_ROT, -1);
        if !buffer.is_empty() {
            assert!(buffer.len() >= 10 && (buffer.len() - 10).is_multiple_of(4));
            let mut values = buffer
                .as_chunks::<2>()
                .0
                .iter()
                .map(|bytes| i16::from_ne_bytes([bytes[0], bytes[1]]));
            self.row = values.next().unwrap();
            self.col = values.next().unwrap();
            self.blk_imp_row = values.next().unwrap();
            self.blk_imp_col = values.next().unwrap();
            self.blk_imp_tab = values.next().unwrap();
            while let Some(kind) = values.next() {
                self.push_ind(kind, values.next().unwrap());
            }
        }
    }
}

impl Scanner {
    fn scn_dir_tag_pfx(&mut self, lexer: &mut dyn Lexer, result_symbol: usize) -> bool {
        if self.lookahead == i32::from(b'!') {
            self.adv(lexer);
        } else if self.scn_ns_tag_char(lexer) != ScanResult::Success {
            return false;
        }
        loop {
            match self.scn_ns_uri_char(lexer) {
                ScanResult::Stop => {
                    self.mrk_end(lexer);
                    return self.finish(lexer, result_symbol);
                }
                ScanResult::Fail => return self.finish(lexer, result_symbol),
                ScanResult::Success => {}
            }
        }
    }

    fn scn_tag(&mut self, lexer: &mut dyn Lexer, result_symbol: usize) -> bool {
        if self.lookahead != i32::from(b'!') {
            return false;
        }
        self.adv(lexer);
        if is_wht(self.lookahead) {
            self.mrk_end(lexer);
            return self.finish(lexer, result_symbol);
        }
        if self.lookahead == i32::from(b'<') {
            self.adv(lexer);
            if self.scn_ns_uri_char(lexer) != ScanResult::Success {
                return false;
            }
            loop {
                match self.scn_ns_uri_char(lexer) {
                    ScanResult::Stop => {
                        if self.lookahead == i32::from(b'>') {
                            self.adv(lexer);
                            self.mrk_end(lexer);
                            return self.finish(lexer, result_symbol);
                        }
                        return false;
                    }
                    ScanResult::Fail => return false,
                    ScanResult::Success => {}
                }
            }
        } else {
            if self.scn_tag_hdl_tal(lexer) && self.scn_ns_tag_char(lexer) != ScanResult::Success {
                return false;
            }
            loop {
                match self.scn_ns_tag_char(lexer) {
                    ScanResult::Stop => {
                        self.mrk_end(lexer);
                        return self.finish(lexer, result_symbol);
                    }
                    ScanResult::Fail => return self.finish(lexer, result_symbol),
                    ScanResult::Success => {}
                }
            }
        }
    }

    fn scn_dqt_esc_seq(&mut self, lexer: &mut dyn Lexer, result_symbol: usize) -> bool {
        let digits = match char::from_u32(self.lookahead as u32) {
            Some(
                '0' | 'a' | 'b' | 't' | '\t' | 'n' | 'v' | 'r' | 'e' | 'f' | ' ' | '"' | '/' | '\\'
                | 'N' | '_' | 'L' | 'P',
            ) => 0,
            Some('U') => 8,
            Some('u') => 4,
            Some('x') => 2,
            _ => return false,
        };
        self.adv(lexer);
        for _ in 0..digits {
            if !is_ns_hex_digit(self.lookahead) {
                return false;
            }
            self.adv(lexer);
        }
        self.mrk_end(lexer);
        self.finish(lexer, result_symbol)
    }
}

/// Creates the scanner, with the same reset as C's calloc + deserialize(NULL, 0).
pub(crate) fn create() -> Box<dyn ExternalScanner> {
    let mut scanner = Scanner::default();
    scanner.deserialize(&[]);
    Box::new(scanner)
}

// This is the core schema's incremental DFA, not a post-token string parser:
// trailing spaces, tabs, and schema freezing must be handled at the same points.
#[inline]
fn advance_schema(state: i8, c: i32, result: &mut ResultSchema) -> i8 {
    // Most plain scalars freeze as strings at their first character. Keep this
    // absorbing state in the scalar loop instead of dispatching the whole DFA
    // for every remaining character. A frozen non-string retains its type only
    // through C's four schema terminators (not through tabs).
    if state == SCH_STT_FRZ {
        if !matches!(c, 0 | 0x0d | 0x0a | 0x20) {
            *result = ResultSchema::String;
        }
        SCH_STT_FRZ
    } else {
        advance_schema_dfa(state, c, result)
    }
}

fn advance_schema_dfa(state: i8, c: i32, result: &mut ResultSchema) -> i8 {
    match state {
        SCH_STT_FRZ => {}
        0 => {
            if c == i32::from(b'.') {
                *result = ResultSchema::String;
                return 6;
            }
            if c == i32::from(b'0') {
                *result = ResultSchema::Int;
                return 37;
            }
            if c == i32::from(b'F') {
                *result = ResultSchema::String;
                return 2;
            }
            if c == i32::from(b'N') {
                *result = ResultSchema::String;
                return 16;
            }
            if c == i32::from(b'T') {
                *result = ResultSchema::String;
                return 13;
            }
            if c == i32::from(b'f') {
                *result = ResultSchema::String;
                return 17;
            }
            if c == i32::from(b'n') {
                *result = ResultSchema::String;
                return 29;
            }
            if c == i32::from(b't') {
                *result = ResultSchema::String;
                return 26;
            }
            if c == i32::from(b'~') {
                *result = ResultSchema::Null;
                return 35;
            }
            if c == i32::from(b'+') {
                *result = ResultSchema::String;
                return 1;
            }
            if c == i32::from(b'-') {
                *result = ResultSchema::String;
                return 1;
            }
            if i32::from(b'1') <= c && c <= i32::from(b'9') {
                *result = ResultSchema::Int;
                return 38;
            }
        }
        1 => {
            if c == i32::from(b'.') {
                *result = ResultSchema::String;
                return 7;
            }
            if i32::from(b'0') <= c && c <= i32::from(b'9') {
                *result = ResultSchema::Int;
                return 38;
            }
        }
        2 => {
            if c == i32::from(b'A') {
                *result = ResultSchema::String;
                return 9;
            }
            if c == i32::from(b'a') {
                *result = ResultSchema::String;
                return 22;
            }
        }
        3 => {
            if c == i32::from(b'A') {
                *result = ResultSchema::String;
                return 12;
            }
            if c == i32::from(b'a') {
                *result = ResultSchema::String;
                return 12;
            }
        }
        4 => {
            if c == i32::from(b'E') {
                *result = ResultSchema::Bool;
                return 36;
            }
        }
        5 => {
            if c == i32::from(b'F') {
                *result = ResultSchema::Float;
                return 41;
            }
        }
        6 => {
            if c == i32::from(b'I') {
                *result = ResultSchema::String;
                return 11;
            }
            if c == i32::from(b'N') {
                *result = ResultSchema::String;
                return 3;
            }
            if c == i32::from(b'i') {
                *result = ResultSchema::String;
                return 24;
            }
            if c == i32::from(b'n') {
                *result = ResultSchema::String;
                return 18;
            }
            if i32::from(b'0') <= c && c <= i32::from(b'9') {
                *result = ResultSchema::Float;
                return 42;
            }
        }
        7 => {
            if c == i32::from(b'I') {
                *result = ResultSchema::String;
                return 11;
            }
            if c == i32::from(b'i') {
                *result = ResultSchema::String;
                return 24;
            }
            if i32::from(b'0') <= c && c <= i32::from(b'9') {
                *result = ResultSchema::Float;
                return 42;
            }
        }
        8 => {
            if c == i32::from(b'L') {
                *result = ResultSchema::Null;
                return 35;
            }
        }
        9 => {
            if c == i32::from(b'L') {
                *result = ResultSchema::String;
                return 14;
            }
        }
        10 => {
            if c == i32::from(b'L') {
                *result = ResultSchema::String;
                return 8;
            }
        }
        11 => {
            if c == i32::from(b'N') {
                *result = ResultSchema::String;
                return 5;
            }
            if c == i32::from(b'n') {
                *result = ResultSchema::String;
                return 20;
            }
        }
        12 => {
            if c == i32::from(b'N') {
                *result = ResultSchema::Float;
                return 41;
            }
        }
        13 => {
            if c == i32::from(b'R') {
                *result = ResultSchema::String;
                return 15;
            }
            if c == i32::from(b'r') {
                *result = ResultSchema::String;
                return 28;
            }
        }
        14 => {
            if c == i32::from(b'S') {
                *result = ResultSchema::String;
                return 4;
            }
        }
        15 => {
            if c == i32::from(b'U') {
                *result = ResultSchema::String;
                return 4;
            }
        }
        16 => {
            if c == i32::from(b'U') {
                *result = ResultSchema::String;
                return 10;
            }
            if c == i32::from(b'u') {
                *result = ResultSchema::String;
                return 23;
            }
        }
        17 => {
            if c == i32::from(b'a') {
                *result = ResultSchema::String;
                return 22;
            }
        }
        18 => {
            if c == i32::from(b'a') {
                *result = ResultSchema::String;
                return 25;
            }
        }
        19 => {
            if c == i32::from(b'e') {
                *result = ResultSchema::Bool;
                return 36;
            }
        }
        20 => {
            if c == i32::from(b'f') {
                *result = ResultSchema::Float;
                return 41;
            }
        }
        21 => {
            if c == i32::from(b'l') {
                *result = ResultSchema::Null;
                return 35;
            }
        }
        22 => {
            if c == i32::from(b'l') {
                *result = ResultSchema::String;
                return 27;
            }
        }
        23 => {
            if c == i32::from(b'l') {
                *result = ResultSchema::String;
                return 21;
            }
        }
        24 => {
            if c == i32::from(b'n') {
                *result = ResultSchema::String;
                return 20;
            }
        }
        25 => {
            if c == i32::from(b'n') {
                *result = ResultSchema::Float;
                return 41;
            }
        }
        26 => {
            if c == i32::from(b'r') {
                *result = ResultSchema::String;
                return 28;
            }
        }
        27 => {
            if c == i32::from(b's') {
                *result = ResultSchema::String;
                return 19;
            }
        }
        28 => {
            if c == i32::from(b'u') {
                *result = ResultSchema::String;
                return 19;
            }
        }
        29 => {
            if c == i32::from(b'u') {
                *result = ResultSchema::String;
                return 23;
            }
        }
        30 => {
            if c == i32::from(b'+') || c == i32::from(b'-') {
                *result = ResultSchema::String;
                return 32;
            }
            if i32::from(b'0') <= c && c <= i32::from(b'9') {
                *result = ResultSchema::Float;
                return 43;
            }
        }
        31 => {
            if i32::from(b'0') <= c && c <= i32::from(b'7') {
                *result = ResultSchema::Int;
                return 39;
            }
        }
        32 => {
            if i32::from(b'0') <= c && c <= i32::from(b'9') {
                *result = ResultSchema::Float;
                return 43;
            }
        }
        33 => {
            if (i32::from(b'0') <= c && c <= i32::from(b'9'))
                || (i32::from(b'A') <= c && c <= i32::from(b'F'))
                || (i32::from(b'a') <= c && c <= i32::from(b'f'))
            {
                *result = ResultSchema::Int;
                return 40;
            }
        }
        34 => {
            unreachable!("unreachable core-schema DFA state");
        }
        35 => {
            *result = ResultSchema::Null;
        }
        36 => {
            *result = ResultSchema::Bool;
        }
        37 => {
            *result = ResultSchema::Int;
            if c == i32::from(b'.') {
                *result = ResultSchema::Float;
                return 42;
            }
            if c == i32::from(b'o') {
                *result = ResultSchema::String;
                return 31;
            }
            if c == i32::from(b'x') {
                *result = ResultSchema::String;
                return 33;
            }
            if c == i32::from(b'E') || c == i32::from(b'e') {
                *result = ResultSchema::String;
                return 30;
            }
            if i32::from(b'0') <= c && c <= i32::from(b'9') {
                *result = ResultSchema::Int;
                return 38;
            }
        }
        38 => {
            *result = ResultSchema::Int;
            if c == i32::from(b'.') {
                *result = ResultSchema::Float;
                return 42;
            }
            if c == i32::from(b'E') || c == i32::from(b'e') {
                *result = ResultSchema::String;
                return 30;
            }
            if i32::from(b'0') <= c && c <= i32::from(b'9') {
                *result = ResultSchema::Int;
                return 38;
            }
        }
        39 => {
            *result = ResultSchema::Int;
            if i32::from(b'0') <= c && c <= i32::from(b'7') {
                *result = ResultSchema::Int;
                return 39;
            }
        }
        40 => {
            *result = ResultSchema::Int;
            if (i32::from(b'0') <= c && c <= i32::from(b'9'))
                || (i32::from(b'A') <= c && c <= i32::from(b'F'))
                || (i32::from(b'a') <= c && c <= i32::from(b'f'))
            {
                *result = ResultSchema::Int;
                return 40;
            }
        }
        41 => {
            *result = ResultSchema::Float;
        }
        42 => {
            *result = ResultSchema::Float;
            if c == i32::from(b'E') || c == i32::from(b'e') {
                *result = ResultSchema::String;
                return 30;
            }
            if i32::from(b'0') <= c && c <= i32::from(b'9') {
                *result = ResultSchema::Float;
                return 42;
            }
        }
        43 => {
            *result = ResultSchema::Float;
            if i32::from(b'0') <= c && c <= i32::from(b'9') {
                *result = ResultSchema::Float;
                return 43;
            }
        }
        _ => {
            *result = ResultSchema::String;
            return SCH_STT_FRZ;
        }
    }
    if !matches!(c, 0 | 0x0d | 0x0a | 0x20) {
        *result = ResultSchema::String;
    }
    SCH_STT_FRZ
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[derive(Debug, PartialEq, Eq)]
    enum Event {
        Advance(usize, bool),
        MarkEnd(usize),
        Symbol(u16),
    }

    struct TestLexer {
        input: Vec<i32>,
        position: usize,
        end: usize,
        symbol: u16,
        events: Vec<Event>,
        lookahead_calls: Cell<usize>,
    }

    impl TestLexer {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().map(|c| c as i32).collect(),
                position: 0,
                end: 0,
                symbol: u16::MAX,
                events: Vec::new(),
                lookahead_calls: Cell::new(0),
            }
        }
    }

    impl Lexer for TestLexer {
        fn lookahead(&self) -> i32 {
            self.lookahead_calls.set(self.lookahead_calls.get() + 1);
            self.input.get(self.position).copied().unwrap_or(0)
        }
        fn result_symbol(&self) -> u16 {
            self.symbol
        }
        fn set_result_symbol(&mut self, symbol: u16) {
            self.symbol = symbol;
            self.events.push(Event::Symbol(symbol));
        }
        fn advance(&mut self, skip: bool) {
            self.events.push(Event::Advance(self.position, skip));
            self.position = (self.position + 1).min(self.input.len());
        }
        fn mark_end(&mut self) {
            self.end = self.position;
            self.events.push(Event::MarkEnd(self.position));
        }
        fn get_column(&mut self) -> u32 {
            panic!("YAML tracks columns itself")
        }
        fn is_at_included_range_start(&self) -> bool {
            panic!("the C YAML scanner does not query included ranges")
        }
        fn eof(&self) -> bool {
            panic!("the C YAML scanner uses lookahead == 0, not eof()")
        }
    }

    fn scanner() -> Scanner {
        let mut scanner = Scanner::default();
        scanner.deserialize(&[]);
        scanner
    }

    fn valid(symbols: &[usize]) -> [bool; ERR_REC + 1] {
        let mut valid = [false; ERR_REC + 1];
        for &symbol in symbols {
            valid[symbol] = true;
        }
        valid
    }

    #[test]
    fn lookahead_is_read_once_at_entry_and_after_each_advance() {
        let cases: &[(&str, &[usize], usize)] = &[
            ("- value", &[R_BLK_SEQ_BGN], R_BLK_SEQ_BGN),
            ("\r\n# comment", &[], COMMENT),
            ("| # comment\n    content", &[R_BLK_LIT_BGN], R_BLK_LIT_BGN),
            (
                "café 中 text: value",
                &[R_SGL_PLN_STR_BLK],
                R_SGL_PLN_STR_BLK,
            ),
            ("true,", &[R_SGL_PLN_STR_FLW], R_SGL_PLN_BOL_BLK + 3),
            ("12 # comment", &[R_SGL_PLN_STR_BLK], R_SGL_PLN_INT_BLK),
            ("''", &[R_SQT_STR_END], R_SQT_ESC_SQT),
            ("", &[END_OF_FILE], END_OF_FILE),
        ];
        let mut scanner = scanner();
        for &(input, symbols, expected) in cases {
            // Reusing the scanner must not reuse the preceding scan's lookahead.
            scanner.deserialize(&[]);
            let mut lexer = TestLexer::new(input);
            assert!(scanner.scan(&mut lexer, &valid(symbols)), "{input:?}");
            assert_eq!(lexer.symbol, expected as u16, "{input:?}");
            let advances = lexer
                .events
                .iter()
                .filter(|event| matches!(event, Event::Advance(..)))
                .count();
            assert_eq!(lexer.lookahead_calls.get(), advances + 1, "{input:?}");
        }
    }

    #[test]
    fn lookahead_refreshes_after_speculative_rewind() {
        let mut scanner = scanner();
        let mut lexer = TestLexer::new("12: x");
        assert!(scanner.scan(&mut lexer, &valid(&[R_SGL_PLN_STR_BLK])));
        assert_eq!(lexer.position, 3);
        assert_eq!(lexer.end, 2);
        assert_eq!(scanner.lookahead, i32::from(b' '));

        // The parser resumes at mark_end, not at the speculative lexer position.
        let mut snapshot = [0; SERIALIZATION_BUFFER_SIZE];
        let size = scanner.serialize(&mut snapshot);
        scanner.deserialize(&snapshot[..size]);
        lexer.position = lexer.end;
        assert!(scanner.scan(&mut lexer, &valid(&[R_BLK_IMP_BGN])));
        assert_eq!(lexer.symbol, R_BLK_IMP_BGN as u16);
        assert_eq!(lexer.end, 3);
        assert_eq!(scanner.col, 3);
    }

    #[test]
    fn core_schema_types_and_case_sensitive_dfa() {
        let cases: &[(ResultSchema, &[&str])] = &[
            (ResultSchema::Null, &["~", "null", "Null", "NULL"]),
            (
                ResultSchema::Bool,
                &["true", "True", "TRUE", "false", "False", "FALSE"],
            ),
            (
                ResultSchema::Int,
                &["0", "01", "-12", "+12", "0o70", "0xDeadBEEF"],
            ),
            (
                ResultSchema::Float,
                &[
                    ".5", "1.", "1e+2", "1E-2", ".inf", "+.Inf", "-.INF", ".nan", ".NaN", ".NAN",
                ],
            ),
            (
                ResultSchema::String,
                &[
                    "yes",
                    "TrueX",
                    "nUlL",
                    "0o8",
                    "0x",
                    "-0x1",
                    "1e",
                    "1_000",
                    "+.nan",
                    ".Nan",
                    "2026-10-07",
                ],
            ),
        ];
        for &(expected, inputs) in cases {
            for &input in inputs {
                let mut state = 0;
                let mut result = ResultSchema::String;
                for c in input.chars() {
                    state = advance_schema(state, c as i32, &mut result);
                }
                assert_eq!(result, expected, "{input:?}");
            }
        }
    }

    #[test]
    fn schema_freezes_at_spaces_but_tabs_force_strings() {
        let mut result = ResultSchema::String;
        let state = advance_schema(0, i32::from(b'1'), &mut result);
        assert_eq!(result, ResultSchema::Int);
        let state = advance_schema(state, i32::from(b' '), &mut result);
        assert_eq!(state, SCH_STT_FRZ);
        assert_eq!(result, ResultSchema::Int);
        assert_eq!(
            advance_schema(state, i32::from(b'2'), &mut result),
            SCH_STT_FRZ
        );
        assert_eq!(result, ResultSchema::String);

        let state = advance_schema(0, i32::from(b'1'), &mut result);
        assert_eq!(
            advance_schema(state, i32::from(b'\t'), &mut result),
            SCH_STT_FRZ
        );
        assert_eq!(result, ResultSchema::String);
    }

    #[test]
    fn frozen_schema_fast_path_matches_dfa_for_every_codepoint() {
        for schema in [
            ResultSchema::String,
            ResultSchema::Int,
            ResultSchema::Null,
            ResultSchema::Bool,
            ResultSchema::Float,
        ] {
            // Include invalid scalars and negative lookahead as well: the C
            // resolver compares int32_t values without Unicode conversion.
            for c in (0..=0x10ffff).chain([-1, 0x110000, i32::MIN, i32::MAX]) {
                let mut expected = schema;
                let expected_state = advance_schema_dfa(SCH_STT_FRZ, c, &mut expected);
                let mut actual = schema;
                let actual_state = advance_schema(SCH_STT_FRZ, c, &mut actual);
                assert_eq!(
                    (actual_state, actual),
                    (expected_state, expected),
                    "{schema:?} {c:#x}"
                );
            }
        }
    }

    #[test]
    fn frozen_schema_preserves_types_only_through_spaces() {
        for (input, symbol, end) in [
            ("123   # comment", R_SGL_PLN_INT_BLK, 3),
            ("123   456", R_SGL_PLN_STR_BLK, 9),
            ("123\t # comment", R_SGL_PLN_STR_BLK, 3),
            ("null   # comment", R_SGL_PLN_NUL_BLK, 4),
            ("null   tail", R_SGL_PLN_STR_BLK, 11),
            ("true   # comment", R_SGL_PLN_BOL_BLK, 4),
            ("true \t# comment", R_SGL_PLN_STR_BLK, 4),
            (".inf   # comment", R_SGL_PLN_FLT_BLK, 4),
            (".inf   tail", R_SGL_PLN_STR_BLK, 11),
            ("ordinary text", R_SGL_PLN_STR_BLK, 13),
        ] {
            let mut scanner = scanner();
            let mut lexer = TestLexer::new(input);
            assert!(
                scanner.scan(&mut lexer, &valid(&[R_SGL_PLN_STR_BLK])),
                "{input:?}"
            );
            assert_eq!(lexer.symbol, symbol as u16, "{input:?}");
            assert_eq!(lexer.end, end, "{input:?}");
        }
    }

    #[test]
    fn serialization_is_native_i16_pairs_without_the_root() {
        let mut scanner = scanner();
        scanner.row = -32_760;
        scanner.col = 17;
        scanner.blk_imp_row = -1;
        scanner.blk_imp_col = 12;
        scanner.blk_imp_tab = 1;
        scanner.push_ind(IND_MAP, 12);
        scanner.push_ind(IND_SEQ, 12);
        let mut buffer = [0; SERIALIZATION_BUFFER_SIZE];
        let length = scanner.serialize(&mut buffer);
        let expected: Vec<_> = [-32_760i16, 17, -1, 12, 1, IND_MAP, 12, IND_SEQ, 12]
            .into_iter()
            .flat_map(i16::to_ne_bytes)
            .collect();
        assert_eq!(&buffer[..length], expected);

        let mut restored = super::Scanner::default();
        restored.deserialize(&buffer[..length]);
        assert_eq!(restored.indents, scanner.indents);
        let mut again = [0; SERIALIZATION_BUFFER_SIZE];
        assert_eq!(restored.serialize(&mut again), length);
        assert_eq!(again, buffer);
        restored.deserialize(&[]);
        assert_eq!(
            restored.indents,
            [Indent {
                kind: IND_ROT,
                length: -1
            }]
        );
        assert_eq!(
            (
                restored.row,
                restored.col,
                restored.blk_imp_row,
                restored.blk_imp_col,
                restored.blk_imp_tab
            ),
            (0, 0, -1, -1, 0)
        );
    }

    #[test]
    fn serialization_never_splits_an_indent_or_overruns() {
        let mut scanner = scanner();
        for i in 0..300 {
            scanner.push_ind(IND_MAP, i);
        }
        let mut buffer = [0xa5; SERIALIZATION_BUFFER_SIZE];
        assert_eq!(scanner.serialize(&mut buffer), 1022);
        assert_eq!(&buffer[1022..], &[0xa5, 0xa5]);
        let mut restored = super::Scanner::default();
        restored.deserialize(&buffer[..1022]);
        assert_eq!(restored.indents, scanner.indents[..254]);
    }

    #[test]
    fn sequence_indicator_lexer_call_order() {
        let mut scanner = scanner();
        let mut lexer = TestLexer::new("- value");
        assert!(scanner.scan(&mut lexer, &valid(&[R_BLK_SEQ_BGN])));
        assert_eq!(
            lexer.events,
            [
                Event::MarkEnd(0),
                Event::Advance(0, false),
                Event::MarkEnd(1),
                Event::Symbol(R_BLK_SEQ_BGN as u16)
            ]
        );
        assert_eq!(
            scanner.indents.last(),
            Some(&Indent {
                kind: IND_SEQ,
                length: 0
            })
        );
    }

    #[test]
    fn document_marker_pops_indent_without_consuming_marker() {
        let mut scanner = scanner();
        scanner.push_ind(IND_MAP, 0);
        let mut lexer = TestLexer::new("--- \n");
        assert!(scanner.scan(&mut lexer, &valid(&[BL])));
        assert_eq!(
            lexer.events,
            [
                Event::MarkEnd(0),
                Event::Advance(0, false),
                Event::Advance(1, false),
                Event::Advance(2, false),
                Event::Symbol(BL as u16)
            ]
        );
        assert_eq!(lexer.end, 0);
        assert_eq!((scanner.row, scanner.col), (0, 0));
        assert_eq!(scanner.indents.len(), 1);
    }

    #[test]
    fn block_header_speculates_beyond_its_marked_end() {
        let mut scanner = scanner();
        let input = "| # comment\n    content";
        let mut lexer = TestLexer::new(input);
        assert!(scanner.scan(&mut lexer, &valid(&[R_BLK_LIT_BGN])));
        assert_eq!(lexer.end, 1);
        assert_eq!(lexer.position, input.find("content").unwrap());
        assert_eq!((scanner.row, scanner.col), (0, 1));
        assert_eq!((scanner.cur_row, scanner.cur_col), (1, 4));
        assert_eq!(
            scanner.indents.last(),
            Some(&Indent {
                kind: IND_STR,
                length: 3
            })
        );
    }

    #[test]
    fn plain_scalars_stop_before_colons_and_comments() {
        for input in ["12: x", "12 # comment"] {
            let mut scanner = scanner();
            let mut lexer = TestLexer::new(input);
            assert!(scanner.scan(&mut lexer, &valid(&[R_SGL_PLN_STR_BLK])));
            assert_eq!(lexer.symbol, R_SGL_PLN_INT_BLK as u16);
            assert_eq!(lexer.end, 2);
            assert_eq!(lexer.position, 3);
            assert_eq!(scanner.col, 2);
            assert_eq!((scanner.blk_imp_row, scanner.blk_imp_col), (0, 0));
        }
    }

    #[test]
    fn quote_escapes_preserve_success_and_failure_positions() {
        let mut scanner = scanner();
        let mut lexer = TestLexer::new("''");
        assert!(scanner.scan(&mut lexer, &valid(&[R_SQT_STR_END])));
        assert_eq!(lexer.symbol, R_SQT_ESC_SQT as u16);
        assert_eq!(lexer.end, 2);
        scanner.deserialize(&[]);
        let mut lexer = TestLexer::new("\\u12xz");
        assert!(!scanner.scan(&mut lexer, &valid(&[R_DQT_ESC_SEQ])));
        assert_eq!(lexer.position, 4);
        assert_eq!(lexer.end, 0);
        assert_eq!(lexer.symbol, u16::MAX);
    }

    #[test]
    fn crlf_and_i16_coordinates_follow_c_not_lexer_columns() {
        let mut scanner = scanner();
        let mut lexer = TestLexer::new("\r\n#x");
        assert!(scanner.scan(&mut lexer, &valid(&[])));
        // Unlike the runtime's points, the scanner counts CR and LF separately.
        assert_eq!((scanner.row, scanner.col), (2, 2));
        scanner.cur_col = i16::MAX;
        let mut lexer = TestLexer::new("a\n");
        scanner.lookahead = lexer.lookahead();
        scanner.adv(&mut lexer);
        assert_eq!(scanner.cur_chr, i32::from(b'a'));
        assert_eq!(scanner.cur_col, i16::MIN);
        scanner.cur_row = i16::MAX;
        scanner.adv_nwl(&mut lexer);
        assert_eq!(scanner.cur_chr, i32::from(b'\n'));
        assert_eq!((scanner.cur_row, scanner.cur_col), (i16::MIN, 0));
    }
}
