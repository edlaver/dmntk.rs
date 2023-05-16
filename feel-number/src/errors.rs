use dmntk_common::DmntkError;

/// `FEEL` number errors.
struct FeelNumberError(String);

impl From<FeelNumberError> for DmntkError {
  /// Converts `FeelNumberError` into [DmntkError].
  fn from(e: FeelNumberError) -> Self {
    DmntkError::new("FeelNumberError", &e.0)
  }
}

/// Creates invalid number literal error.
pub fn err_invalid_number_literal(s: &str) -> DmntkError {
  FeelNumberError(format!("invalid number literal '{s}'")).into()
}

/// Creates number conversion error.
pub fn err_number_conversion_failed() -> DmntkError {
  FeelNumberError("number conversion failed".to_string()).into()
}
