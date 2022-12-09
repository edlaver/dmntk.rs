/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2018-2022 Dariusz Depta Engos Software
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * Apache license, Version 2.0
 *
 * Copyright (c) 2018-2022 Dariusz Depta Engos Software
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! `FEEL` context.

use self::errors::*;
use crate::names::Name;
use crate::qualified_names::QualifiedName;
use crate::strings::ToFeelString;
use crate::types::FeelType;
use crate::value_null;
use crate::values::Value;
use dmntk_common::{DmntkError, Jsonify};
use std::collections::{BTreeMap, HashSet};
use std::convert::TryFrom;
use std::ops::Deref;

/// Type alias for context entries.
type FeelContextEntries = BTreeMap<Name, Value>;

/// The `FEEL` context.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FeelContext(FeelContextEntries);

impl Deref for FeelContext {
  type Target = FeelContextEntries;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl TryFrom<Value> for FeelContext {
  type Error = DmntkError;
  /// Tries to convert a [Value] to its [FeelContext] representation.
  fn try_from(value: Value) -> Result<Self, Self::Error> {
    if let Value::Context(ctx) = value {
      Ok(ctx)
    } else {
      Err(err_value_is_not_a_context(&value))
    }
  }
}

impl From<FeelContext> for Value {
  /// Converts this [FeelContext] to its [Value] representation.
  fn from(ctx: FeelContext) -> Self {
    Value::Context(ctx)
  }
}

impl std::fmt::Display for FeelContext {
  ///
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{{{}}}",
      self
        .0
        .iter()
        .map(|(name, value)| { format!(r#"{name}: {value}"#) })
        .collect::<Vec<String>>()
        .join(", ")
    )
  }
}

impl ToFeelString for FeelContext {
  /// Converts [FeelContext] into `FEEL` string.
  fn to_feel_string(&self) -> String {
    format!(
      "{{{}}}",
      self
        .0
        .iter()
        .map(|(name, value)| {
          let name_str = format!("{name}");
          let padded_name_str = match name_str.as_str() {
            "{" | "}" | ":" | "," => format!("\"{name_str}\""),
            "\"" => "\"\\\"\"".to_string(),
            _ => name_str,
          };
          format!(r#"{padded_name_str}: {value}"#)
        })
        .collect::<Vec<String>>()
        .join(", ")
    )
  }
}

impl Jsonify for FeelContext {
  /// Converts this [FeelContext] into its `JSON` representation.
  fn jsonify(&self) -> String {
    format!(
      "{{{}}}",
      self
        .0
        .iter()
        .map(|(name, value)| format!(r#""{}": {}"#, name, value.jsonify()))
        .collect::<Vec<String>>()
        .join(", ")
    )
  }
}

impl FeelContext {
  /// Checks if this [FeelContext] contains an entry pointed by [Name].
  pub fn contains_entry(&self, name: &Name) -> bool {
    self.0.contains_key(name)
  }
  /// Checks if this [FeelContext] contains an entry pointed by [QualifiedName].
  pub fn contains_entries(&self, qname: &QualifiedName) -> bool {
    self.contains_deep(qname.as_slice())
  }
  /// Sets a single value for specified entry name in this [FeelContext].
  pub fn set_entry(&mut self, name: &Name, value: Value) {
    self.0.insert(name.clone(), value);
  }
  /// Sets a null value for specified entry name in this [FeelContext].
  pub fn set_null(&mut self, name: Name) {
    self.0.insert(name, value_null!());
  }
  /// Returns a value of an entry specified by name.
  pub fn get_entry(&self, name: &Name) -> Option<&Value> {
    self.0.get(name)
  }
  /// Returns a list of key-value pairs.
  pub fn get_entries(&self) -> Vec<(&Name, &Value)> {
    self.0.iter().collect::<Vec<(&Name, &Value)>>()
  }
  /// Returns a first value contained by context.
  pub fn get_first(&self) -> Option<&Value> {
    self.0.values().take(1).next()
  }
  /// Returns the number of entries in this context.
  pub fn len(&self) -> usize {
    self.0.len()
  }
  /// Returns `true` when this context is empty.
  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }
  ///
  pub fn zip(&mut self, other: &FeelContext) {
    for (name, value) in &other.0 {
      self.0.insert(name.clone(), value.clone());
    }
  }
  ///
  pub fn overwrite(&mut self, other: &FeelContext) {
    for (name, value) in &other.0 {
      if self.0.contains_key(name) {
        self.0.insert(name.clone(), value.clone());
      }
    }
  }
  /// Creates an entry with a value for specified [QualifiedName].
  /// All non existing intermediary contexts will be created.
  pub fn create_entry(&mut self, qname: &QualifiedName, value: Value) {
    self.create_deep(qname.as_slice(), value);
  }
  /// Returns a list of flattened keys for this [FeelContext].
  pub fn flatten_keys(&self) -> HashSet<String> {
    let mut keys: HashSet<String> = HashSet::new();
    for (key, value) in self.0.iter() {
      keys.insert(key.into());
      if let Value::Context(sub_ctx) = value {
        let sub_keys = sub_ctx.flatten_keys();
        if !sub_keys.is_empty() {
          for sub_key in sub_keys {
            keys.insert(sub_key.clone());
            keys.insert(format!("{key} . {sub_key}"));
          }
        }
      }
      if let Value::List(items) = value {
        for item in items.as_vec() {
          if let Value::Context(item_ctx) = item {
            let sub_keys = item_ctx.flatten_keys();
            if !sub_keys.is_empty() {
              for sub_key in sub_keys {
                keys.insert(sub_key.clone());
                keys.insert(format!("{key} . {sub_key}"));
              }
            }
          }
        }
      }
      if let Value::FeelType(FeelType::Context(type_entries)) = value {
        for name in type_entries.keys() {
          let sub_key = name.to_string();
          keys.insert(sub_key.clone());
          keys.insert(format!("{key} . {sub_key}"));
        }
      }
    }
    keys.iter().cloned().collect()
  }
  /// Searches for a value of an entry pointed by specified qualified name.
  pub fn search_entry<'search>(&'search self, qname: &'search QualifiedName) -> Option<&'search Value> {
    self.search_deep(qname.as_slice())
  }
  /// Deep check for a value pointed by slice of names.
  pub fn contains_deep(&self, names: &[Name]) -> bool {
    if names.is_empty() {
      return false;
    }
    let tail = &names[1..];
    if let Some(value) = self.0.get(&names[0]) {
      if tail.is_empty() {
        return true;
      }
      if let Value::Context(context) = value {
        return context.contains_deep(tail);
      }
    }
    false
  }
  /// Creates intermediary contexts when needed.
  pub fn create_deep(&mut self, names: &[Name], value: Value) {
    // if there are no names, then return
    if names.is_empty() {
      return;
    }
    let key = names[0].clone();
    let tail = &names[1..];
    // if tail is empty, then insert the value under
    // specified key in current context and return
    if tail.is_empty() {
      self.0.insert(key, value);
      return;
    }
    // if there is a context under the specified key,
    // then insert value to this context and return
    if let Some(Value::Context(sub_ctx)) = self.0.get_mut(&key) {
      sub_ctx.create_deep(tail, value);
      return;
    }
    // finally, when got to this point, insert a value
    // to newly created context
    let mut sub_ctx = FeelContext::default();
    sub_ctx.create_deep(tail, value);
    self.0.insert(key, sub_ctx.into());
  }
  /// Deep search for a value pointed by names.
  pub fn search_deep(&self, names: &[Name]) -> Option<&Value> {
    if !names.is_empty() {
      let tail = &names[1..];
      if let Some(value) = self.0.get(&names[0]) {
        if let Value::Context(ctx) = value {
          return if tail.is_empty() { Some(value) } else { ctx.search_deep(tail) };
        } else if tail.is_empty() {
          return Some(value);
        }
      }
    }
    None
  }
}

/// Definitions of context errors.
mod errors {
  use crate::values::Value;
  use dmntk_common::DmntkError;

  /// Context errors.
  struct ContextError(String);

  impl From<ContextError> for DmntkError {
    fn from(e: ContextError) -> Self {
      DmntkError::new("ContextError", &e.0)
    }
  }

  /// Creates an instance of `value is not a context` error.
  pub fn err_value_is_not_a_context(value: &Value) -> DmntkError {
    ContextError(format!("'{value}' is not a value containing context")).into()
  }
}
