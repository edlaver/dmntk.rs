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

//! Builder for decision evaluators.

use crate::boxed_expressions::{bring_knowledge_requirements_into_context, build_expression_instance_evaluator};
use crate::errors::err_write_lock_failed;
use crate::model_builder::ModelBuilder;
use crate::model_definitions::{DefDecision, DefDefinitions};
use crate::model_evaluator::ModelEvaluator;
use crate::variable::Variable;
use dmntk_common::Result;
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value;
use dmntk_feel::{value_null, FeelScope, Name};
use std::collections::HashMap;
use std::sync::RwLock;

/// Type alias for closures that evaluate decisions.
/// Fn(input data, model evaluator, output data)
type DecisionEvaluatorFn = Box<dyn Fn(&FeelContext, &ModelEvaluator, &mut FeelContext) -> Name + Send + Sync>;

///
type DecisionEvaluatorEntry = (Variable, DecisionEvaluatorFn);

///
#[derive(Default)]
pub struct DecisionEvaluator {
  evaluators: RwLock<HashMap<String, DecisionEvaluatorEntry>>,
}

impl DecisionEvaluator {
  /// Creates a new decision evaluator.
  pub fn build(&self, definitions: &DefDefinitions, model_builder: &ModelBuilder) -> Result<()> {
    for decision in definitions.decisions() {
      let evaluator_entry = build_decision_evaluator(definitions, decision, model_builder)?;
      let decision_id = decision.id();
      let decision_name = &decision.name().to_string();
      self.evaluators.write().map_err(err_write_lock_failed)?.insert(decision_id.to_owned(), evaluator_entry);
      model_builder.add_invocable_decision(decision_name, decision_id);
    }
    Ok(())
  }

  /// Evaluates a decision with specified identifier.
  pub fn evaluate(&self, decision_id: &str, input_data: &FeelContext, model_evaluator: &ModelEvaluator, evaluated_ctx: &mut FeelContext) -> Option<Name> {
    self
      .evaluators
      .read()
      .ok()?
      .get(decision_id)
      .map(|evaluator_entry| evaluator_entry.1(input_data, model_evaluator, evaluated_ctx))
  }

  /// Returns the name and type of the output variable of a decision with specified identifier.
  pub fn get_output_variable(&self, decision_id: &str) -> Option<Variable> {
    self.evaluators.read().ok()?.get(decision_id).map(|entry| entry.0.clone())
  }
}

///
fn build_decision_evaluator(definitions: &DefDefinitions, decision: &DefDecision, model_builder: &ModelBuilder) -> Result<DecisionEvaluatorEntry> {
  // acquire all needed evaluators
  let item_definition_type_evaluator = model_builder.item_definition_type_evaluator();
  let item_definition_context_evaluator = model_builder.item_definition_context_evaluator();
  let input_data_context_evaluator = model_builder.input_data_context_evaluator();

  // get output variable
  let mut output_variable = Variable::try_from(decision.variable())?;
  output_variable.update_feel_type(item_definition_type_evaluator);

  // prepare output variable name for this decision
  let output_variable_name = output_variable.name().clone();

  // prepare output variable type for this decision
  let output_variable_type = output_variable.feel_type().clone();

  // holds variables for required decisions and required knowledge
  let mut knowledge_requirements_ctx = FeelContext::default();

  // hods variables for required inputs
  let mut input_requirements_ctx = FeelContext::default();

  // bring into context the variables from this decision's knowledge requirements
  bring_knowledge_requirements_into_context(definitions, decision.knowledge_requirements(), &mut knowledge_requirements_ctx)?;

  // bring into context the variables from information requirements
  for information_requirement in decision.information_requirements() {
    // bring into context the variable from required decision
    if let Some(href) = information_requirement.required_decision() {
      if let Some(required_decision) = definitions.decision_by_id(href.into()) {
        let variable_name = required_decision.variable().name();
        //TODO below "Any" type is assumed when the variable has no typeRef property, but typeRef is required - so the models should be corrected
        let variable_type_ref = if required_decision.variable().type_ref().is_some() {
          required_decision.variable().type_ref().as_ref().unwrap().clone()
        } else {
          "Any".to_string()
        };
        let variable_type = item_definition_context_evaluator.eval(&variable_type_ref, variable_name, &mut knowledge_requirements_ctx);
        knowledge_requirements_ctx.set_entry(variable_name, Value::FeelType(variable_type));
        // bring into context the variables from this required decision's knowledge requirements
        bring_knowledge_requirements_into_context(definitions, required_decision.knowledge_requirements(), &mut knowledge_requirements_ctx)?;
      }
    }
    if let Some(href) = information_requirement.required_input() {
      // bring into context the variable from required input
      if let Some(required_input) = definitions.input_data_by_id(href.into()) {
        let variable_name = required_input.variable().name();
        let variable_type = input_data_context_evaluator.eval(href.into(), &mut input_requirements_ctx, item_definition_context_evaluator);
        input_requirements_ctx.set_entry(variable_name, Value::FeelType(variable_type));
      }
    }
  }

  // prepare a scope and build expression instance evaluator
  let scope: FeelScope = knowledge_requirements_ctx.into();
  scope.push(input_requirements_ctx.clone());

  // prepare expression instance for this decision
  let evaluator = if let Some(expression_instance) = decision.decision_logic().as_ref() {
    let (evl, _) = build_expression_instance_evaluator(&scope, expression_instance, model_builder)?;
    evl
  } else {
    Box::new(move |_: &FeelScope| value_null!("no decision logic defined in decision"))
  };

  // prepare references to required knowledge, required decisions and required input data
  let mut required_knowledge_references: Vec<String> = vec![];
  let mut required_decision_references: Vec<String> = vec![];
  let mut required_input_data_references: Vec<String> = vec![];
  for knowledge_requirement in decision.knowledge_requirements() {
    if let Some(href) = knowledge_requirement.required_knowledge() {
      required_knowledge_references.push(href.into());
    }
  }
  for information_requirement in decision.information_requirements() {
    if let Some(href) = information_requirement.required_decision() {
      required_decision_references.push(href.into())
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

    required_knowledge_references.iter().for_each(|id| {
      // evaluate required knowledge as values from business knowledge models
      business_knowledge_model_evaluator.evaluate(id, input_data_ctx, model_evaluator, &mut required_knowledge_ctx)
    });

    required_knowledge_references.iter().for_each(|id| {
      // evaluate required knowledge as decision service function definitions
      decision_service_evaluator.evaluate_fd(id, input_data_ctx, &mut required_knowledge_ctx)
    });

    required_decision_references.iter().for_each(|id| {
      // evaluate required decisions as values from decisions
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
    let coerced_decision_result = output_variable_type.coerced(&decision_result);
    output_data_ctx.set_entry(&output_variable_name, coerced_decision_result);

    // return the name of the output variable
    output_variable_name.clone()
  });
  // return the output variable, and decision evaluator closure
  Ok((output_variable, decision_evaluator))
}