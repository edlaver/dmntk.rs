use dmntk_feel_number::{num, FeelNumber};

#[test]
fn test_neg_001() {
  assert_eq!("-1.23", -num!(1.23));
}

#[test]
fn test_neg_002() {
  assert_eq!("1.23", -num!(-1.23));
}
