use super::Table;

const CHOICES: [&str; 12] = [
    include_str!("../../../test_data/simple/value_single.json"),
    include_str!("../../../test_data/simple/value_multiple.json"),
    include_str!("../../../test_data/simple/ranges_single.json"),
    include_str!("../../../test_data/simple/ranges_multiple.json"),
    include_str!("../../../test_data/simple/mixed_single.json"),
    include_str!("../../../test_data/simple/mixed_multiple.json"),
    include_str!("../../../test_data/complex/value_single.json"),
    include_str!("../../../test_data/complex/value_multiple.json"),
    include_str!("../../../test_data/complex/ranges_single.json"),
    include_str!("../../../test_data/complex/ranges_multiple.json"),
    include_str!("../../../test_data/complex/mixed_single.json"),
    include_str!("../../../test_data/complex/mixed_multiple.json"),
];

#[test]
fn simple_value_single_is_valid() {
    assert!(serde_json::from_str::<Table>(CHOICES[0]).is_ok());
}

#[test]
fn simple_value_multiple_is_valid() {
    assert!(serde_json::from_str::<Table>(CHOICES[1]).is_ok());
}

#[test]
fn simple_ranges_single_is_valid() {
    assert!(serde_json::from_str::<Table>(CHOICES[2]).is_ok());
}

#[test]
fn simple_ranges_multiple_is_valid() {
    assert!(serde_json::from_str::<Table>(CHOICES[3]).is_ok());
}

#[test]
fn simple_mixed_single_is_valid() {
    assert!(serde_json::from_str::<Table>(CHOICES[4]).is_ok());
}

#[test]
fn simple_mixed_multiple_is_valid() {
    assert!(serde_json::from_str::<Table>(CHOICES[5]).is_ok());
}

#[test]
fn complex_value_single_is_valid() {
    assert!(serde_json::from_str::<Table>(CHOICES[6]).is_ok());
}

#[test]
fn complex_value_multiple_is_valid() {
    assert!(serde_json::from_str::<Table>(CHOICES[7]).is_ok());
}

#[test]
fn complex_ranges_single_is_valid() {
    assert!(serde_json::from_str::<Table>(CHOICES[8]).is_ok());
}

#[test]
fn complex_ranges_multiple_is_valid() {
    assert!(serde_json::from_str::<Table>(CHOICES[9]).is_ok());
}

#[test]
fn complex_mixed_single_is_valid() {
    assert!(serde_json::from_str::<Table>(CHOICES[10]).is_ok());
}

#[test]
fn complex_mixed_multiple_is_valid() {
    assert!(serde_json::from_str::<Table>(CHOICES[11]).is_ok());
}
