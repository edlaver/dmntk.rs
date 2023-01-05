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

//! `FEEL` qualified names.

use crate::Name;
use std::fmt;
use std::ops::Deref;

/// FEEL `QualifiedName`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualifiedName(Vec<Name>);

impl QualifiedName {
  /// Creates a [QualifiedName] from [Names](Name).
  pub fn new(names: &[&Name]) -> Self {
    Self(names.iter().map(|&v| v.clone()).collect::<Vec<Name>>())
  }
}

impl fmt::Display for QualifiedName {
  /// Implements [Display](std::fmt::Display) trait for [QualifiedName].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0.iter().map(|v| v.to_string()).collect::<Vec<String>>().join("."))
  }
}

impl Deref for QualifiedName {
  type Target = Vec<Name>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl QualifiedName {
  /// Appends this [QualifiedName] with a given [Name].
  pub fn push(&mut self, name: Name) {
    self.0.push(name);
  }
}

impl From<Name> for QualifiedName {
  /// Converts a [Name] into [QualifiedName].
  fn from(name: Name) -> Self {
    Self(name.to_string().split('.').map(Name::from).collect())
  }
}
