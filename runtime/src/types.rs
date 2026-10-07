pub(crate) use crate::point::Point;
pub(crate) use ts_port_tables::{FieldId, StateId, Symbol};
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Range {
    pub start_point: Point,
    pub end_point: Point,
    pub start_byte: u32,
    pub end_byte: u32,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct InputEdit {
    pub start_byte: u32,
    pub old_end_byte: u32,
    pub new_end_byte: u32,
    pub start_point: Point,
    pub old_end_point: Point,
    pub new_end_point: Point,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum InputEncoding {
    #[default]
    Utf8,
    Utf16Le,
    Utf16Be,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SymbolType {
    Regular,
    Anonymous,
    Supertype,
    Auxiliary,
}
pub(crate) const BUILTIN_SYM_ERROR_REPEAT: Symbol = ts_port_tables::BUILTIN_SYM_ERROR - 1;
/// The input owns/borrows its latest chunk. `read` replaces it; `chunk` does not
/// call the client. This avoids copying large suffixes returned by callbacks.
pub(crate) trait Input {
    fn read(&mut self, byte: u32, point: Point);
    fn chunk(&self) -> &[u8];
}
pub(crate) struct SliceInput<'a> {
    pub bytes: &'a [u8],
    pub chunk_start: usize,
}
impl Input for SliceInput<'_> {
    fn read(&mut self, byte: u32, _: Point) {
        self.chunk_start = (byte as usize).min(self.bytes.len());
    }
    fn chunk(&self) -> &[u8] {
        &self.bytes[self.chunk_start..]
    }
}
pub(crate) struct CallbackInput<'a, F, T> {
    pub callback: &'a mut F,
    pub chunk: Option<T>,
}
impl<F: FnMut(usize, crate::Point) -> T, T: AsRef<[u8]>> Input for CallbackInput<'_, F, T> {
    fn read(&mut self, byte: u32, point: Point) {
        self.chunk = Some((self.callback)(
            byte as usize,
            crate::Point {
                row: point.row as usize,
                column: point.column as usize,
            },
        ));
    }
    fn chunk(&self) -> &[u8] {
        self.chunk.as_ref().map_or(&[], AsRef::as_ref)
    }
}
