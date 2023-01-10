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

//! Implementation of the scope used while parsing FEEL expressions.

use crate::context::ParsingContext;
use dmntk_feel::Name;
use std::cell::RefCell;
use std::collections::HashSet;

/// Parsing scope.
#[derive(Debug)]
pub struct ParsingScope {
  /// The stack of parsing contexts.
  stack: RefCell<Vec<ParsingContext>>,
}

impl From<&dmntk_feel::Scope> for ParsingScope {
  /// Temporary - remove
  fn from(scope: &dmntk_feel::Scope) -> Self {
    let stack = RefCell::new(vec![]);
    for feel_context in scope.contexts() {
      stack.borrow_mut().push(feel_context.into());
    }
    Self { stack }
  }
}

impl Default for ParsingScope {
  /// Creates a default parsing scope containing single parsing context.
  fn default() -> Self {
    Self {
      stack: RefCell::new(vec![ParsingContext::default()]),
    }
  }
}

impl ParsingScope {
  /// Returns a context removed from the top of the stack.
  pub fn pop(&self) -> Option<ParsingContext> {
    self.stack.borrow_mut().pop()
  }

  /// Pushes a default parsing context on top of the stack.
  pub fn push_default(&self) {
    self.stack.borrow_mut().push(ParsingContext::default())
  }

  /// Sets a specified name in context placed on the top of the stack.
  pub fn set_name(&self, name: Name) {
    if let Some(last_ctx) = self.stack.borrow_mut().last_mut() {
      last_ctx.set_name(name);
    }
  }

  /// Sets parsing context under a specified name in context placed on the top of the stack.
  pub fn set_context(&self, name: Name, ctx: ParsingContext) {
    if let Some(last_ctx) = self.stack.borrow_mut().last_mut() {
      last_ctx.set_context(name, ctx);
    }
  }

  /// Returns a vector of flattened keys in all contexts in scope.
  pub fn flatten_keys(&self) -> HashSet<String> {
    self.stack.borrow().iter().flat_map(|ctx| ctx.flatten_keys()).collect::<HashSet<String>>()
  }
}
