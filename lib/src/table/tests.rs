use super::Table;

const TEST_VALUES: [&str; 20] = [
    include_str!("../../../test_data/lib/simple/value_single.json"),
    include_str!("../../../test_data/lib/simple/value_multiple.json"),
    include_str!("../../../test_data/lib/simple/ranges_single.json"),
    include_str!("../../../test_data/lib/simple/ranges_multiple.json"),
    include_str!("../../../test_data/lib/simple/mixed_single.json"),
    include_str!("../../../test_data/lib/simple/mixed_multiple.json"),
    include_str!("../../../test_data/lib/complex/value_single.json"),
    include_str!("../../../test_data/lib/complex/value_multiple.json"),
    include_str!("../../../test_data/lib/complex/ranges_single.json"),
    include_str!("../../../test_data/lib/complex/ranges_multiple.json"),
    include_str!("../../../test_data/lib/complex/mixed_single.json"),
    include_str!("../../../test_data/lib/complex/mixed_multiple.json"),
    include_str!("../../../test_data/lib/errors/range/duplicate.json"),
    include_str!("../../../test_data/lib/errors/range/oob_less.json"),
    include_str!("../../../test_data/lib/errors/range/oob_more.json"),
    include_str!("../../../test_data/lib/errors/range/unsorted.json"),
    include_str!("../../../test_data/lib/errors/single/duplicate.json"),
    include_str!("../../../test_data/lib/errors/single/oob_less.json"),
    include_str!("../../../test_data/lib/errors/single/oob_more.json"),
    include_str!("../../../test_data/lib/errors/single/unsorted.json"),
];

#[test]
fn simple_value_single_is_valid() {
    assert!(serde_json::from_str::<Table>(TEST_VALUES[0]).is_ok());
}

#[test]
fn simple_value_multiple_is_valid() {
    assert!(serde_json::from_str::<Table>(TEST_VALUES[1]).is_ok());
}

#[test]
fn simple_ranges_single_is_valid() {
    assert!(serde_json::from_str::<Table>(TEST_VALUES[2]).is_ok());
}

#[test]
fn simple_ranges_multiple_is_valid() {
    assert!(serde_json::from_str::<Table>(TEST_VALUES[3]).is_ok());
}

#[test]
fn simple_mixed_single_is_valid() {
    assert!(serde_json::from_str::<Table>(TEST_VALUES[4]).is_ok());
}

#[test]
fn simple_mixed_multiple_is_valid() {
    assert!(serde_json::from_str::<Table>(TEST_VALUES[5]).is_ok());
}

#[test]
fn complex_value_single_is_valid() {
    assert!(serde_json::from_str::<Table>(TEST_VALUES[6]).is_ok());
}

#[test]
fn complex_value_multiple_is_valid() {
    assert!(serde_json::from_str::<Table>(TEST_VALUES[7]).is_ok());
}

#[test]
fn complex_ranges_single_is_valid() {
    assert!(serde_json::from_str::<Table>(TEST_VALUES[8]).is_ok());
}

#[test]
fn complex_ranges_multiple_is_valid() {
    assert!(serde_json::from_str::<Table>(TEST_VALUES[9]).is_ok());
}

#[test]
fn complex_mixed_single_is_valid() {
    assert!(serde_json::from_str::<Table>(TEST_VALUES[10]).is_ok());
}

#[test]
fn complex_mixed_multiple_is_valid() {
    assert!(serde_json::from_str::<Table>(TEST_VALUES[11]).is_ok());
}

// TODO Test Errors
#[test]
fn errors_range_duplicate_is_error() {
    assert!(serde_json::from_str::<Table>(TEST_VALUES[12]).is_err());
}

#[test]
fn errors_range_oob_less_is_error() {
    assert!(serde_json::from_str::<Table>(TEST_VALUES[13]).is_err());
}

#[test]
fn errors_range_oob_more_is_error() {
    assert!(serde_json::from_str::<Table>(TEST_VALUES[14]).is_err());
}

#[test]
fn errors_range_unsorted_is_error() {
    assert!(serde_json::from_str::<Table>(TEST_VALUES[15]).is_err());
}

#[test]
fn errors_single_duplicate_is_error() {
    assert!(serde_json::from_str::<Table>(TEST_VALUES[16]).is_err());
}

#[test]
fn errors_single_oob_less_is_error() {
    assert!(serde_json::from_str::<Table>(TEST_VALUES[17]).is_err());
}

#[test]
fn errors_single_oob_more_is_error() {
    assert!(serde_json::from_str::<Table>(TEST_VALUES[18]).is_err());
}

#[test]
fn errors_single_unsorted_is_error() {
    assert!(serde_json::from_str::<Table>(TEST_VALUES[19]).is_err());
}
