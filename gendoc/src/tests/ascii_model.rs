use crate::ascii_model::print_model;
use dmntk_common::ColorMode;

#[test]
fn a() {
  let definitions = dmntk_model::parse(dmntk_examples::DMN_2_0001).expect("parsing model failed");
  print_model(&definitions, ColorMode::On);
}
