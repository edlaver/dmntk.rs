mod common;

use dmntk_feel_number::FeelNumber;

#[test]
fn test_sub_001() {
  eqs!("1.00", num!(1.23) - num!(0.23));
}

#[test]
fn test_sub_002() {
  let mut x = num!(1.23);
  x -= num!(0.23);
  eqs!("1.00", x);
}
