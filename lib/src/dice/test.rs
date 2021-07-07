// TODO add more tests
use super::Dice;

#[test]
fn max_combination_min_eq_1() {
    let d = Dice::maybe_new(u8::max_value(), u16::max_value()).expect("max values where negative?");
    let min = d.min_val();
    assert_eq!(1.0, d.possible_combinations(min));
}

#[test]
fn combination_2d6_mid() {
    let d = Dice::maybe_new(2, 6).unwrap();
    assert_eq!(6.0, d.possible_combinations(7));
}

#[test]
fn prob_2d6_mid() {
    let d = Dice::maybe_new(2, 6).unwrap();
    assert_eq!(0.16666666666666666, d.probability(7));
}

#[test]
fn prob_3d6_mid() {
    let d = Dice::maybe_new(3, 6).unwrap();
    assert_eq!(d.probability(10), d.probability(11));
}
