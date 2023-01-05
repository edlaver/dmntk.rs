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

//! `FEEL` or `DMN` function body definition.

use crate::values::Value;
use crate::{Evaluator, Scope};
use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;

/// Type alias of the closure that evaluates `FEEL` or `DMN` function body into [Value].
pub type FunctionBodyEvaluator = Arc<Evaluator>;

/// Function body may be defined in `FEEL` or `DMN` in many ways.
/// This enum is the representation of all of these cases.
#[derive(Clone)]
pub enum FunctionBody {
  /// Function body created from context defined in `DMN` model.
  Context(FunctionBodyEvaluator),
  /// Function body created from `FEEL` textual expression defined in `DMN` model.
  LiteralExpression(FunctionBodyEvaluator),
  /// Function body created from decision table defined in `DMN` model.
  DecisionTable(FunctionBodyEvaluator),
  /// Function body created from decision service defined in `DMN` model.
  DecisionService(FunctionBodyEvaluator),
  /// Function body created from externally defined function (`Java`, `PMML`).
  External(FunctionBodyEvaluator),
}

impl FunctionBody {
  /// Evaluates function body, takes a [Scope] as input and returns evaluated [Value].
  pub fn evaluate(&self, scope: &Scope) -> Value {
    match self {
      FunctionBody::Context(evaluator) => evaluator(scope),
      FunctionBody::LiteralExpression(evaluator) => evaluator(scope),
      FunctionBody::DecisionTable(evaluator) => evaluator(scope),
      FunctionBody::DecisionService(evaluator) => evaluator(scope),
      FunctionBody::External(evaluator) => evaluator(scope),
    }
  }
}

impl Debug for FunctionBody {
  ///
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      FunctionBody::Context(_) => write!(f, "FunctionBodyContext"),
      FunctionBody::LiteralExpression(_) => write!(f, "FunctionBodyLiteralExpression"),
      FunctionBody::DecisionTable(_) => write!(f, "FunctionBodyDecisionTable"),
      FunctionBody::DecisionService(_) => write!(f, "FunctionBodyDecisionService"),
      FunctionBody::External(_) => write!(f, "FunctionBodyExternal"),
    }
  }
}

impl PartialEq for FunctionBody {
  ///
  fn eq(&self, other: &Self) -> bool {
    match self {
      FunctionBody::Context(_) => matches!(other, FunctionBody::Context(_)),
      FunctionBody::LiteralExpression(_) => matches!(other, FunctionBody::LiteralExpression(_)),
      FunctionBody::DecisionTable(_) => matches!(other, FunctionBody::DecisionTable(_)),
      FunctionBody::DecisionService(_) => matches!(other, FunctionBody::DecisionService(_)),
      FunctionBody::External(_) => matches!(other, FunctionBody::External(_)),
    }
  }
}
