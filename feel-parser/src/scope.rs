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
use crate::AstNode;
use dmntk_feel::Name;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt;

/// Parsing scope.
pub struct ParsingScope {
  /// The stack of parsing contexts.
  stack: RefCell<Vec<ParsingContext>>,
}

impl From<&dmntk_feel::FeelScope> for ParsingScope {
  /// Temporary - remove
  fn from(scope: &dmntk_feel::FeelScope) -> Self {
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

impl fmt::Display for ParsingScope {
  /// Converts parsing scope to text representation.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{}]", self.stack.borrow_mut().iter().map(|ctx| ctx.to_string()).collect::<Vec<String>>().join(", "))
  }
}

impl From<ParsingContext> for ParsingScope {
  /// Creates a [ParsingScope] from [ParsingContext].
  fn from(ctx: ParsingContext) -> Self {
    Self { stack: RefCell::new(vec![ctx]) }
  }
}

impl ParsingScope {
  /// Returns a context removed from the top of the stack.
  pub fn pop(&self) -> Option<ParsingContext> {
    self.stack.borrow_mut().pop()
  }

  /// Puts a context on the top of the stack.
  pub fn push(&self, ctx: ParsingContext) {
    self.stack.borrow_mut().push(ctx);
  }

  /// Puts a default context on the top of the stack.
  pub fn push_default(&self) {
    self.stack.borrow_mut().push(ParsingContext::default());
  }

  /// Sets a specified name in context placed on the top of the stack.
  pub fn set_name(&self, name: Name) {
    if let Some(last_ctx) = self.stack.borrow_mut().last_mut() {
      last_ctx.set_name(name);
    }
  }

  /// Sets a context under a specified name in the context placed on the top of the stack.
  pub fn set_context(&self, name: Name, ctx: ParsingContext) {
    if let Some(last_ctx) = self.stack.borrow_mut().last_mut() {
      last_ctx.set_context(name, ctx);
    }
  }

  /// Returns a set of flattened keys from all contexts in scope.
  pub fn flattened_keys(&self) -> HashSet<String> {
    self.stack.borrow().iter().flat_map(|ctx| ctx.flattened_keys()).collect::<HashSet<String>>()
  }

  pub fn prepare_closed_names(&self, node: &AstNode) {
    self.visit1(node, 0);
  }

  fn visit0(&self, _: usize) -> Vec<Name> {
    vec![]
  }

  fn visit1(&self, node: &AstNode, level: usize) -> Vec<Name> {
    match node {
      AstNode::Add(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::And(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::At(_) => self.visit0(level),
      AstNode::Between(lhs, mhs, rhs) => self.visit3(lhs, mhs, rhs, level),
      AstNode::Boolean(_) => self.visit0(level),
      AstNode::CommaList(lhs) => self.visit4(lhs, level),
      AstNode::Context(lhs) => self.visit4(lhs, level),
      AstNode::ContextEntry(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::ContextEntryKey(_) => self.visit0(level),
      AstNode::ContextType(lhs) => self.visit4(lhs, level),
      AstNode::ContextTypeEntry(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::ContextTypeEntryKey(_) => self.visit0(level),
      AstNode::Div(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::Eq(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::EvaluatedExpression(lhs) => self.visit1(lhs, level),
      AstNode::Every(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::Exp(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::ExpressionList(lhs) => self.visit4(lhs, level),
      AstNode::FeelType(_) => self.visit0(level),
      AstNode::Filter(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::For(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::FormalParameter(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::FormalParameters(lhs) => self.visit4(lhs, level),
      AstNode::FunctionBody(lhs, _) => self.visit1(lhs, level),
      AstNode::FunctionDefinition(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::FunctionInvocation(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::FunctionType(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::Ge(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::Gt(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::If(lhs, mhs, rhs) => self.visit3(lhs, mhs, rhs, level),
      AstNode::In(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::InstanceOf(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::IntervalEnd(lhs, _) => self.visit1(lhs, level),
      AstNode::IntervalStart(lhs, _) => self.visit1(lhs, level),
      AstNode::Irrelevant => self.visit0(level),
      AstNode::IterationContexts(lhs) => self.visit4(lhs, level),
      AstNode::IterationContextSingle(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::IterationContextRange(lhs, mhs, rhs) => self.visit3(lhs, mhs, rhs, level),
      AstNode::Le(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::List(lhs) => self.visit4(lhs, level),
      AstNode::ListType(lhs) => self.visit1(lhs, level),
      AstNode::Lt(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::Mul(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::Name(name) => {
        if level == 0 {
          // println!("name = {:?}", name);

          //TODO here collect the name for closure
        }
        vec![name.clone()]
      }
      AstNode::NamedParameter(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::NamedParameters(lhs) => self.visit4(lhs, level),
      AstNode::Neg(lhs) => self.visit1(lhs, level),
      AstNode::NegatedList(lhs) => self.visit4(lhs, level),
      AstNode::Null => self.visit0(level),
      AstNode::Numeric(_, _) => self.visit0(level),
      AstNode::Nq(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::Or(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::Out(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::ParameterName(_) => self.visit0(level),
      AstNode::ParameterTypes(lhs) => self.visit4(lhs, level),
      AstNode::Path(lhs, rhs) => {
        let mut parts = self.visit1(rhs, level + 1);
        let mut name = self.visit1(lhs, level + 1);
        parts.append(&mut name);
        let mut a = parts.clone();
        a.reverse();
        if level == 0 {
          // println!("path = {:?}", a);

          //TODO here collect the qualified name for closure
        }
        parts
      }
      AstNode::PositionalParameters(lhs) => self.visit4(lhs, level),
      AstNode::QualifiedName(lhs) => self.visit4(lhs, level),
      AstNode::QualifiedNameSegment(_name) => self.visit0(level), //TODO verify if also must be used
      AstNode::QuantifiedContext(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::QuantifiedContexts(lhs) => self.visit4(lhs, level),
      AstNode::Range(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::RangeType(lhs) => self.visit1(lhs, level),
      AstNode::Some(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::String(_) => self.visit0(level),
      AstNode::Sub(lhs, rhs) => self.visit2(lhs, rhs, level),
      AstNode::UnaryGe(lhs) => self.visit1(lhs, level),
      AstNode::UnaryGt(lhs) => self.visit1(lhs, level),
      AstNode::UnaryLe(lhs) => self.visit1(lhs, level),
      AstNode::UnaryLt(lhs) => self.visit1(lhs, level),
      AstNode::Satisfies(lhs) => self.visit1(lhs, level),
    }
  }

  fn visit2(&self, lhs: &AstNode, rhs: &AstNode, level: usize) -> Vec<Name> {
    self.visit1(lhs, level);
    self.visit1(rhs, level);
    vec![]
  }

  fn visit3(&self, lhs: &AstNode, mhs: &AstNode, rhs: &AstNode, level: usize) -> Vec<Name> {
    self.visit1(lhs, level);
    self.visit1(mhs, level);
    self.visit1(rhs, level);
    vec![]
  }

  fn visit4(&self, lhs: &Vec<AstNode>, level: usize) -> Vec<Name> {
    for item in lhs {
      self.visit1(item, level);
    }
    vec![]
  }
}
