/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2018-2023 Dariusz Depta, Engos Software
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
 * Copyright (c) 2018-2023 Dariusz Depta, Engos Software
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

//! # Builder for decision evaluators

use crate::boxed_expressions::*;
use crate::model_builder::ModelBuilder;
use crate::model_definitions::*;
use crate::model_evaluator::ModelEvaluator;
use crate::variable::Variable;
use dmntk_common::Result;
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value;
use dmntk_feel::{value_null, FeelScope, Name};
use std::collections::HashMap;
use std::sync::Arc;

/// Type alias for closures that evaluate decisions.
///
/// (input data, model evaluator, output data) -> Output variable name
///
type DecisionEvaluatorFn = Box<dyn Fn(&FeelContext, &ModelEvaluator, &mut FeelContext) -> Name + Send + Sync>;

/// Type alias for decision's output variable combined with decision's evaluator function.
///
/// (variable, decision evaluator function)
///
type DecisionEvaluatorEntry = (Variable, DecisionEvaluatorFn);

/// Decision evaluator.
pub struct DecisionEvaluator {
  evaluators: Arc<HashMap<DefKey, DecisionEvaluatorEntry>>,
}

impl DecisionEvaluator {
  /// Creates an empty decision evaluator.
  pub fn empty() -> Self {
    Self {
      evaluators: Arc::new(HashMap::new()),
    }
  }

  /// Creates a new decision evaluator.
  pub fn new(definitions: &DefDefinitions, model_builder: &ModelBuilder) -> Result<Self> {
    let mut evaluators = HashMap::new();
    for decision in definitions.decisions() {
      let evaluator_entry = build_decision_evaluator(definitions, decision, model_builder)?;
      let decision_namespace = decision.namespace();
      let decision_id = decision.id();
      let decision_name = decision.name().to_string();
      let def_key = DefKey::new(decision_namespace, decision_id);
      evaluators.insert(def_key.clone(), evaluator_entry);
      model_builder.add_decision_invocable(decision_name, def_key);
    }
    Ok(Self { evaluators: Arc::new(evaluators) })
  }

  /// Evaluates a decision identified by specified `decision_id`.
  pub fn evaluate(&self, def_key: &DefKey, input_data: &FeelContext, model_evaluator: &ModelEvaluator, evaluated_ctx: &mut FeelContext) -> Option<Name> {
    self
      .evaluators
      .get(def_key)
      .map(|evaluator_entry| evaluator_entry.1(input_data, model_evaluator, evaluated_ctx))
  }

  /// Returns the variable for specified decision.
  pub fn get_variable(&self, def_key: &DefKey) -> Option<&Variable> {
    self.evaluators.get(def_key).map(|entry| &entry.0)
  }
}

/// Builds and returns decision evaluator.
fn build_decision_evaluator(definitions: &DefDefinitions, decision: &DefDecision, model_builder: &ModelBuilder) -> Result<DecisionEvaluatorEntry> {
  // acquire all needed intermediary evaluators
  let item_definition_type_evaluator = model_builder.item_definition_type_evaluator();
  let item_definition_context_evaluator = model_builder.item_definition_context_evaluator();
  let input_data_context_evaluator = model_builder.input_data_context_evaluator();

  // get the output variable properties
  let mut output_variable = Variable::try_from(decision.variable())?;
  output_variable.update_feel_type(item_definition_type_evaluator);

  // prepare output variable name for processed decision
  let output_variable_name = output_variable.name().clone();

  // prepare output variable type for processed decision
  let output_variable_type = output_variable.feel_type().clone();

  // holds variables for required decisions and required knowledge
  let mut requirements_ctx = FeelContext::default();

  // hods variables for required inputs
  let mut input_requirements_ctx = FeelContext::default();

  // bring into context the variables from this decision's knowledge requirements
  bring_knowledge_requirements_into_context(definitions, decision.knowledge_requirements(), &mut requirements_ctx)?;

  // bring into context the variables from information requirements
  for information_requirement in decision.information_requirements() {
    // bring into context the variable from required decision
    if let Some(href) = information_requirement.required_decision() {
      if let Some(required_decision) = definitions.decision_by_key(href.namespace(), href.id()) {
        let variable_name = required_decision.variable().name();
        let variable_type_ref = required_decision.variable().type_ref();
        let variable_type = item_definition_context_evaluator.eval(variable_type_ref, variable_name, &mut requirements_ctx);
        requirements_ctx.set_entry(variable_name, Value::FeelType(variable_type));
      }
    }
    // bring into context the variable from required input
    if let Some(href) = information_requirement.required_input() {
      if let Some(required_input) = definitions.input_data_by_key(href.namespace(), href.id()) {
        let variable_name = required_input.variable().name();
        let variable_type = input_data_context_evaluator.eval(href.id(), &mut input_requirements_ctx, item_definition_context_evaluator);
        input_requirements_ctx.set_entry(variable_name, Value::FeelType(variable_type));
      }
    }
  }

  // prepare a scope and build expression instance evaluator
  let scope: FeelScope = requirements_ctx.into();
  scope.push(input_requirements_ctx.clone());

  // prepare expression instance for this decision
  let evaluator = if let Some(expression_instance) = decision.decision_logic().as_ref() {
    let (evaluator, _) = build_expression_instance_evaluator(&scope, expression_instance, model_builder)?;
    evaluator
  } else {
    Box::new(move |_: &FeelScope| value_null!("no decision logic defined in decision"))
  };

  // prepare references to required knowledge, required decisions and required input data
  let mut required_knowledge_references: Vec<DefKey> = vec![];
  let mut required_decision_references: Vec<DefKey> = vec![];
  let mut required_input_data_references: Vec<DefKey> = vec![];
  for knowledge_requirement in decision.knowledge_requirements() {
    required_knowledge_references.push(knowledge_requirement.required_knowledge().into());
  }
  for information_requirement in decision.information_requirements() {
    if let Some(href) = information_requirement.required_decision() {
      required_decision_references.push(DefKey::new(href.namespace(), href.id()))
    }
    if let Some(href) = information_requirement.required_input() {
      required_input_data_references.push(href.into())
    }
  }

  // build decision evaluator closure
  let decision_evaluator = Box::new(move |input_data_ctx: &FeelContext, model_evaluator: &ModelEvaluator, output_data_ctx: &mut FeelContext| {
    let business_knowledge_model_evaluator = model_evaluator.business_knowledge_model_evaluator();
    let decision_service_evaluator = model_evaluator.decision_service_evaluator();
    let decision_evaluator = model_evaluator.decision_evaluator();
    let input_data_evaluator = model_evaluator.input_data_evaluator();
    let item_definition_evaluator = model_evaluator.item_definition_evaluator();

    // prepare context containing values from required knowledge and required decisions
    let mut required_knowledge_ctx: FeelContext = Default::default();

    // evaluate required knowledge as values from business knowledge models
    required_knowledge_references.iter().for_each(|id| {
      business_knowledge_model_evaluator.evaluate(id, input_data_ctx, model_evaluator, &mut required_knowledge_ctx);
    });

    // evaluate required knowledge as decision service function definitions
    required_knowledge_references.iter().for_each(|def_key| {
      decision_service_evaluator.evaluate_fd(def_key, input_data_ctx, &mut required_knowledge_ctx);
    });

    // evaluate required decisions as values from decisions
    required_decision_references.iter().for_each(|id| {
      decision_evaluator.evaluate(id, input_data_ctx, model_evaluator, &mut required_knowledge_ctx);
    });

    // values from required knowledge may be overridden by input data
    required_knowledge_ctx.overwrite(input_data_ctx);

    // prepare context containing values from required input data
    let mut required_input_ctx: FeelContext = Default::default();
    let input_data = Value::Context(input_data_ctx.clone());
    required_input_data_references.iter().for_each(|input_data_id| {
      if let Some((name, value)) = input_data_evaluator.evaluate(input_data_id, &input_data, item_definition_evaluator) {
        required_input_ctx.set_entry(&name, value);
      }
    });
    required_input_ctx.zip(&required_knowledge_ctx);

    // place the result under the name of the output variable
    let scope: FeelScope = required_input_ctx.into();
    let decision_result = evaluator(&scope) as Value;
    let coerced_decision_result = decision_result.coerced(&output_variable_type);
    output_data_ctx.set_entry(&output_variable_name, coerced_decision_result);

    // return the name of the output variable
    output_variable_name.clone()
  });
  // return the output variable and decision evaluator function
  Ok((output_variable, decision_evaluator))
}
