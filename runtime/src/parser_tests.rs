use super::*;

fn status(cost: u32, node_count: u32, dynamic_precedence: i32, is_in_error: bool) -> ErrorStatus {
    ErrorStatus {
        cost,
        node_count,
        dynamic_precedence,
        is_in_error,
    }
}

fn assert_comparison(a: ErrorStatus, b: ErrorStatus, expected: ErrorComparison) {
    assert_eq!(compare_error_statuses(a, b), expected);
    let reverse = match expected {
        ErrorComparison::TakeLeft => ErrorComparison::TakeRight,
        ErrorComparison::PreferLeft => ErrorComparison::PreferRight,
        ErrorComparison::None => ErrorComparison::None,
        ErrorComparison::PreferRight => ErrorComparison::PreferLeft,
        ErrorComparison::TakeRight => ErrorComparison::TakeLeft,
    };
    assert_eq!(compare_error_statuses(b, a), reverse);
}

#[test]
fn string_input_is_a_byte_suffix_and_ignores_point() {
    let input = "aé\0b".as_bytes();
    let point = Point {
        row: 200,
        column: 30,
    };
    assert_eq!(ts_string_input_read(input, 0, point), input);
    // Input offsets are bytes, even in the middle of a UTF-8 character.
    assert_eq!(ts_string_input_read(input, 2, point), &input[2..]);
    assert!(ts_string_input_read(input, input.len() as u32, point).is_empty());
    assert!(ts_string_input_read(input, u32::MAX, point).is_empty());
    assert!(ts_string_input_read(&[], 0, point).is_empty());
}

#[test]
fn versions_prefer_out_of_error_before_cost_or_precedence() {
    use ErrorComparison::*;
    assert_comparison(status(5, 0, -100, false), status(6, 0, 100, true), TakeLeft);
    assert_comparison(
        status(6, 0, -100, false),
        status(6, 0, 100, true),
        PreferLeft,
    );
    assert_comparison(
        status(5000, 0, -100, false),
        status(0, 0, 100, true),
        PreferLeft,
    );
}

#[test]
fn versions_use_strict_cost_threshold_and_cheaper_node_count() {
    use ErrorComparison::*;
    for in_error in [false, true] {
        assert_comparison(
            status(0, 0, 0, in_error),
            status(MAX_COST_DIFFERENCE, 1000, 100, in_error),
            PreferLeft,
        );
        assert_comparison(
            status(0, 0, 0, in_error),
            status(MAX_COST_DIFFERENCE + 1, 0, 100, in_error),
            TakeLeft,
        );
        assert_comparison(
            status(0, 17, 0, in_error),
            status(100, 999, 100, in_error),
            PreferLeft,
        );
        assert_comparison(
            status(0, 18, 0, in_error),
            status(100, 0, 100, in_error),
            TakeLeft,
        );
    }
}

#[test]
fn version_cost_math_wraps_like_unsigned_c() {
    use ErrorComparison::*;
    assert_comparison(
        status(0, u32::MAX, 0, false),
        status(u32::MAX, 0, 0, false),
        PreferLeft,
    );
    assert_comparison(
        status(0, 1, 0, false),
        status(1 << 31, 0, 0, false),
        PreferLeft,
    );
}

#[test]
fn version_precedence_only_breaks_equal_cost_ties() {
    use ErrorComparison::*;
    assert_comparison(
        status(2, 0, -1, false),
        status(2, 100, 1, false),
        PreferRight,
    );
    assert_comparison(status(2, 0, 1, true), status(2, 100, 1, true), None);
    assert_comparison(
        status(1, 0, -1, false),
        status(2, 100, 1, false),
        PreferLeft,
    );
}

#[test]
fn lookahead_log_escapes_c_control_characters_only() {
    assert_eq!(
        parser_escape_symbol("\t\n\u{b}\u{c}\r\\\"é\0ignored"),
        "\\t\\n\\v\\f\\r\\\\\"é"
    );
    assert_eq!(parser_log_buffer("hello\0ignored".into()), "hello");
    assert_eq!(
        parser_log_buffer("x".repeat(2000)).len(),
        SERIALIZATION_BUFFER_SIZE - 1
    );
    let message = parser_log_buffer(format!("{}é", "x".repeat(SERIALIZATION_BUFFER_SIZE - 2)));
    assert_eq!(message.len(), SERIALIZATION_BUFFER_SIZE - 2);
}
