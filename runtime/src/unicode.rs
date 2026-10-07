//! Unicode decoding helpers belong to the lexer unit. Invalid input must retain
//! C's byte consumption and -1 lookahead semantics; do not use lossy UTF-8.
pub(crate) const DECODE_ERROR: i32 = -1;

pub(crate) fn ts_decode_utf8(input: &[u8]) -> (u32, i32) {
    todo!("lexer: ts_decode_utf8")
}

pub(crate) fn ts_decode_utf16_le(input: &[u8]) -> (u32, i32) {
    todo!("lexer: ts_decode_utf16_le")
}

pub(crate) fn ts_decode_utf16_be(input: &[u8]) -> (u32, i32) {
    todo!("lexer: ts_decode_utf16_be")
}
