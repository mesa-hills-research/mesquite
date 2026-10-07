//! The runtime uses C's 32-bit coordinates; the public API uses `usize`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Point {
    pub row: u32,
    pub column: u32,
}
pub(crate) const POINT_ZERO: Point = Point { row: 0, column: 0 };
pub(crate) const POINT_MAX: Point = Point {
    row: u32::MAX,
    column: u32::MAX,
};
pub(crate) const fn point_new(row: u32, column: u32) -> Point {
    Point { row, column }
}
pub(crate) fn point_add(a: Point, b: Point) -> Point {
    if b.row > 0 {
        point_new(a.row.wrapping_add(b.row), b.column)
    } else {
        point_new(a.row, a.column.wrapping_add(b.column))
    }
}
pub(crate) fn point_sub(a: Point, b: Point) -> Point {
    if a.row > b.row {
        point_new(a.row - b.row, a.column)
    } else {
        point_new(0, a.column.saturating_sub(b.column))
    }
}
pub(crate) fn point_lte(a: Point, b: Point) -> bool {
    a <= b
}
pub(crate) fn point_lt(a: Point, b: Point) -> bool {
    a < b
}
pub(crate) fn point_gt(a: Point, b: Point) -> bool {
    a > b
}
pub(crate) fn point_gte(a: Point, b: Point) -> bool {
    a >= b
}
pub(crate) fn point_eq(a: Point, b: Point) -> bool {
    a == b
}
