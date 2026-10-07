//! `ts_assert` evaluates its expression even in release C builds. Do not replace
//! it with `debug_assert!` if the expression has side effects.
macro_rules! ts_assert {
    ($expression:expr) => {{
        let result = $expression;
        debug_assert!(result);
    }};
}
pub(crate) use ts_assert;
