use crate::ascii_model::print_model;
use dmntk_common::ColorMode;

#[test]
fn _0001() {
  let definitions = dmntk_model::parse(dmntk_examples::DMN_2_0001).expect("parsing model failed");
  print_model(&definitions, ColorMode::On);
}

#[test]
fn _0002() {
  let definitions = dmntk_model::parse(dmntk_examples::DMN_2_0002).expect("parsing model failed");
  print_model(&definitions, ColorMode::On);
}

#[test]
fn test_full_model() {
  let definitions = dmntk_model::parse(dmntk_examples::DMN_FULL).expect("parsing model failed");
  print_model(&definitions, ColorMode::On);
}
