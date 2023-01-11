/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2018-2023 Dariusz Depta Engos Software
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
 * Copyright (c) 2018-2023 Dariusz Depta Engos Software
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

//! Implementation of the parsing context.

use dmntk_feel::values::Value;
use dmntk_feel::{FeelType, Name};
use std::collections::btree_map::Iter;
use std::collections::{BTreeMap, HashSet};

/// Attributes of the element in parsing context.
#[derive(Debug, Clone)]
pub enum ParsingContextEntry {
  ///
  Context(ParsingContext),
  ///
  Attributes,
}

/// Parsing context.
#[derive(Debug, Default, Clone)]
pub struct ParsingContext(BTreeMap<Name, ParsingContextEntry>);

impl From<dmntk_feel::context::FeelContext> for ParsingContext {
  /// Temporary - remove.
  fn from(context: dmntk_feel::context::FeelContext) -> Self {
    let mut entries = BTreeMap::new();
    for (name, value) in context.iter() {
      match value {
        Value::Context(inner_ctx) => {
          entries.insert(name.to_owned(), ParsingContextEntry::Context(inner_ctx.clone().into()));
        }
        Value::FeelType(feel_type) => {
          if let FeelType::Context(a) = feel_type {
            for name in a.keys() {
              entries.insert(name.to_owned(), ParsingContextEntry::Attributes);
            }
          }
        }
        _ => {
          entries.insert(name.to_owned(), ParsingContextEntry::Attributes);
        }
      }
      if let Value::Context(inner_ctx) = value {
        entries.insert(name.to_owned(), ParsingContextEntry::Context(inner_ctx.clone().into()));
      } else {
        entries.insert(name.to_owned(), ParsingContextEntry::Attributes);
      }
    }
    Self(entries)
  }
}

impl ParsingContext {
  /// Places a specified name in this parsing context.
  pub fn set_name(&mut self, name: Name) {
    self.0.insert(name, ParsingContextEntry::Attributes);
  }

  /// Places parsing context under specified name.
  pub fn set_context(&mut self, name: Name, ctx: ParsingContext) {
    self.0.insert(name, ParsingContextEntry::Context(ctx));
  }

  /// Returns an iterator over the entries.
  pub fn get_entries(&self) -> Iter<Name, ParsingContextEntry> {
    self.0.iter()
  }

  /// Returns a list of flattened keys for this parsing context.
  pub fn flatten_keys(&self) -> HashSet<String> {
    let mut keys: HashSet<String> = HashSet::new();
    for (key, value) in self.0.iter() {
      keys.insert(key.into());
      if let ParsingContextEntry::Context(sub_ctx) = value {
        let sub_keys = sub_ctx.flatten_keys();
        if !sub_keys.is_empty() {
          for sub_key in sub_keys {
            keys.insert(sub_key.clone());
            keys.insert(format!("{key} . {sub_key}"));
          }
        }
      }
    }
    keys
  }
}
