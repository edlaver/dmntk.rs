use dmntk_feel_number::{num, FeelNumber};

#[test]
fn test_round_001() {
  assert_eq!("123.46", num!(123.4567).round(&num!(2)));
}

#[test]
fn test_round_002() {
  assert_eq!("123.45", num!(123.4547).round(&num!(2)));
}

#[test]
fn test_round_003() {
  assert_eq!("100", num!(123.4567).round(&num!(-2)));
}

#[test]
fn test_round_004() {
  assert_eq!("200", num!(163.4567).round(&num!(-2)));
}
