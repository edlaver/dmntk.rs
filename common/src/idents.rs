use uuid::Uuid;

/// Returns newly generated UUID version 4 as a string.
pub fn gen_id() -> String {
  Uuid::new_v4().to_string()
}
