/// Builds null value with invalid argument type message.
macro_rules! invalid_argument_type {
  ($function:literal, $expected:expr, $actual:expr) => {{
    use std::file;
    use std::path::Path;
    let f = Path::new(file!()).file_stem().unwrap().to_string_lossy();
    value_null!(f, $function, "{}", format!("invalid argument type, expected {}, actual type is {}", $expected, $actual))
  }};
}

pub(crate) use invalid_argument_type;
