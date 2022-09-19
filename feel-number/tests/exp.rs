use dmntk_feel_number::{num, FeelNumber};

#[test]
fn test_exp_001() {
  assert_eq!("2.718281828459045235360287471352662", num!(1).exp());
}

#[test]
fn test_exp_002() {
  assert_eq!("54.59815003314423907811026120286088", num!(4).exp());
}
