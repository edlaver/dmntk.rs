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

//! Builder for decision service evaluators.

use crate::model_builder::ModelBuilder;
use crate::model_definitions::{DefDecisionService, DefDefinitions, DefKey};
use crate::model_evaluator::ModelEvaluator;
use crate::variable::Variable;
use dmntk_common::Result;
use dmntk_feel::closure::Closure;
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value;
use dmntk_feel::{value_null, Evaluator, FeelScope, FeelType, Name};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Type of closure that evaluates a decision service.
/// Fn(input_data, model evaluator, output data)
type DecisionServiceEvaluatorFn = Box<dyn Fn(&FeelContext, &ModelEvaluator, &mut FeelContext) -> Name + Send + Sync>;

///
type DecisionServiceEvaluatorEntry = (Variable, Vec<(Name, FeelType)>, DecisionServiceEvaluatorFn, Option<Evaluator>);

/// Decision service evaluator.
pub struct DecisionServiceEvaluator {
  evaluators: RwLock<HashMap<DefKey, DecisionServiceEvaluatorEntry>>,
}

impl DecisionServiceEvaluator {
  /// Creates an empty decision service evaluator.
  pub fn empty() -> Self {
    Self {
      evaluators: RwLock::new(HashMap::new()),
    }
  }

  /// Creates a new decision service evaluator.
  pub fn new(definitions: &DefDefinitions, model_builder: &ModelBuilder) -> Result<Self> {
    let mut evaluators = HashMap::new();
    for decision_service in definitions.decision_services() {
      let decision_service_namespace = decision_service.namespace();
      let decision_service_id = decision_service.id();
      let decision_service_name = decision_service.name().to_string();
      let evaluator = build_decision_service_evaluator(decision_service, model_builder)?;
      let def_key = DefKey::new(decision_service_namespace, decision_service_id);
      evaluators.insert(def_key.clone(), evaluator);
      model_builder.add_decision_service_invocable(decision_service_name, def_key);
    }
    Ok(Self {
      evaluators: RwLock::new(evaluators),
    })
  }

  /// Creates function definition evaluators for all decision service evaluators.
  pub fn build_function_definitions(&self, model_evaluator: &Arc<ModelEvaluator>) {
    if let Ok(mut evaluators) = self.evaluators.write() {
      let identifiers = evaluators.keys().cloned().collect::<Vec<DefKey>>();
      for decision_service_id in identifiers {
        let evaluator = Arc::clone(model_evaluator);
        evaluators.entry(decision_service_id.clone()).and_modify(|entry| {
          let output_variable_type = entry.0.feel_type().clone();
          let body_evaluator = Box::new(move |scope: &FeelScope| {
            let input_data = scope.peek().unwrap_or_default();
            let mut output_data = FeelContext::default();
            let decision_service_evaluator = evaluator.decision_service_evaluator();
            let opt_out_variable_name = decision_service_evaluator.evaluate(&decision_service_id, &input_data, &evaluator, &mut output_data);
            if let Some(out_variable_name) = opt_out_variable_name {
              if let Some(result_value) = output_data.get_entry(&out_variable_name) {
                return result_value.clone();
              }
            }
            value_null!()
          });
          let function_body = dmntk_feel::FunctionBody::DecisionService(Arc::new(body_evaluator));
          let function_definition = Value::FunctionDefinition(entry.1.clone(), function_body, false, Closure::default(), FeelContext::default(), output_variable_type);
          let decision_service_as_function_definition_evaluator = Box::new(move |_: &FeelScope| function_definition.clone());
          entry.3 = Some(decision_service_as_function_definition_evaluator);
        });
      }
    }
  }

  /// Evaluates a decision service with specified identifier.
  pub fn evaluate(&self, def_key: &DefKey, input_data: &FeelContext, model_evaluator: &ModelEvaluator, output_data: &mut FeelContext) -> Option<Name> {
    self.evaluators.read().ok()?.get(def_key).map(|entry| entry.2(input_data, model_evaluator, output_data))
  }

  /// Returns a decision service as function definition with specified identifier.
  pub fn evaluate_fd(&self, def_key: &DefKey, input_data: &FeelContext, output_data: &mut FeelContext) {
    if let Ok(evaluators) = self.evaluators.read() {
      if let Some((variable, _, _, Some(evaluator))) = evaluators.get(def_key) {
        let scope: FeelScope = input_data.clone().into();
        let function_definition = evaluator(&scope) as Value;
        let output_variable_name = variable.name().clone();
        output_data.set_entry(&output_variable_name, function_definition);
      }
    }
  }
}

///
fn build_decision_service_evaluator(decision_service: &DefDecisionService, model_builder: &ModelBuilder) -> Result<DecisionServiceEvaluatorEntry> {
  let item_definition_type_evaluator = model_builder.item_definition_type_evaluator();
  let input_data_evaluator = model_builder.input_data_evaluator();
  let decision_evaluator = model_builder.decision_evaluator();

  let mut output_variable = Variable::try_from(decision_service.variable())?;
  output_variable.update_feel_type(item_definition_type_evaluator);

  // prepare output variable name for this decision
  let output_variable_name = output_variable.name().clone();

  // prepare output variable type for this decision
  let output_variable_type = output_variable.feel_type().clone();

  // prepare references to required input data
  let input_data_references: Vec<String> = decision_service.input_data().iter().map(|href| href.id().to_string()).collect();

  // prepare references to input decisions
  let input_decisions: Vec<DefKey> = decision_service.input_decisions().iter().map(|href| href.into()).collect();

  // prepare references to encapsulated decisions
  let encapsulated_decisions: Vec<DefKey> = decision_service.encapsulated_decisions().iter().map(|href| href.into()).collect();

  // prepare references to output decisions
  let output_decisions: Vec<DefKey> = decision_service.output_decisions().iter().map(|href| href.into()).collect();

  // prepare a container for formal parameters accepted by this decision service
  let mut formal_parameters: Vec<(Name, FeelType)> = vec![];

  // fills the list of formal parameters based on required input data
  // these parameters are placed before input parameters defined by input decisions
  for input_data_id in &input_data_references {
    if let Some(input_data_variable) = input_data_evaluator.get_variable(input_data_id) {
      let parameter_name = input_data_variable.name().clone();
      let parameter_type = input_data_variable.resolve_feel_type(item_definition_type_evaluator);
      formal_parameters.push((parameter_name, parameter_type));
    }
  }

  // prepare evaluators from output variables of input decisions
  // these evaluators will evaluate results from input decisions and values from input data
  // simultaneously, fills the list of formal parameters based on output variables of input decisions
  let mut input_decision_results_evaluators = vec![];
  for def_key in &input_decisions {
    if let Some(decision_output_variable) = decision_evaluator.get_variable(def_key) {
      let parameter_name = decision_output_variable.name().clone();
      let parameter_type = decision_output_variable.resolve_feel_type(item_definition_type_evaluator);
      formal_parameters.push((parameter_name, parameter_type));
      let evaluator = decision_output_variable.build_evaluator();
      input_decision_results_evaluators.push(evaluator);
    }
  }

  // build decision service evaluator closure
  let decision_service_evaluator = Box::new(move |input_data: &FeelContext, model_evaluator: &ModelEvaluator, output_data: &mut FeelContext| {
    // acquire all evaluators needed
    let item_definition_evaluator = model_evaluator.item_definition_evaluator();
    let input_data_evaluator = model_evaluator.input_data_evaluator();
    let decision_evaluator = model_evaluator.decision_evaluator();
    // evaluate input decisions and store the results in separate context
    let mut input_decisions_results = FeelContext::default();
    input_decisions.iter().for_each(|def_key| {
      decision_evaluator.evaluate(def_key, input_data, model_evaluator, &mut input_decisions_results);
    });
    // now evaluate input data for encapsulated and output decisions and store them in separate context
    let mut evaluated_input_data = FeelContext::default();
    // first take values from evaluated input decisions...
    let input_decision_results_value = Value::Context(input_decisions_results);
    for evaluator in &input_decision_results_evaluators {
      let (name, value) = evaluator(&input_decision_results_value, item_definition_evaluator);
      evaluated_input_data.set_entry(&name, value);
    }
    // ...and then take values from provided input data
    let input_data_values = Value::Context(input_data.clone());
    for evaluator in &input_decision_results_evaluators {
      let (name, value) = evaluator(&input_data_values, item_definition_evaluator);
      evaluated_input_data.set_entry(&name, value);
    }
    // evaluate required inputs (from required input data references)
    input_data_references.iter().for_each(|input_data_id| {
      if let Some((name, value)) = input_data_evaluator.evaluate(input_data_id, &input_data_values, item_definition_evaluator) {
        evaluated_input_data.set_entry(&name, value);
      }
    });
    // prepare context for evaluated result data for this decision service
    let mut evaluated_ctx = FeelContext::default();
    // evaluate encapsulated decisions
    encapsulated_decisions.iter().for_each(|def_key| {
      decision_evaluator.evaluate(def_key, &evaluated_input_data, model_evaluator, &mut evaluated_ctx);
    });
    // evaluate output decisions
    let mut output_names = vec![];
    output_decisions.iter().for_each(|def_key| {
      if let Some(output_name) = decision_evaluator.evaluate(def_key, &evaluated_input_data, model_evaluator, &mut evaluated_ctx) {
        output_names.push(output_name);
      }
    });
    // prepare the result from this decision service
    if output_names.len() == 1 {
      if let Some(value) = evaluated_ctx.get_entry(&output_names[0]) {
        let single_result = value.to_owned();
        let coerced_single_result = single_result.coerced(&output_variable_type);
        output_data.set_entry(&output_variable_name, coerced_single_result);
      }
    } else {
      let mut output_ctx = FeelContext::default();
      output_names.iter().for_each(|output_name| {
        if let Some(value) = evaluated_ctx.get_entry(output_name) {
          output_ctx.set_entry(output_name, value.to_owned());
        }
      });
      let complex_result = Value::Context(output_ctx);
      let coerced_complex_result = complex_result.coerced(&output_variable_type);
      output_data.set_entry(&output_variable_name, coerced_complex_result);
    }
    output_variable_name.clone()
  });
  Ok((output_variable, formal_parameters, decision_service_evaluator, None))
}
