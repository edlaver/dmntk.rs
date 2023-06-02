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

//! # DMN model evaluator

use crate::business_knowledge_model::BusinessKnowledgeModelEvaluator;
use crate::decision::DecisionEvaluator;
use crate::decision_service::DecisionServiceEvaluator;
use crate::input_data::InputDataEvaluator;
use crate::item_definition::ItemDefinitionEvaluator;
use crate::model_builder::ModelBuilder;
use crate::model_definitions::{DefKey, InvocableType, Invocables};
use dmntk_common::Result;
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value;
use dmntk_feel::{value_null, Name};
use dmntk_model::Definitions;
use std::collections::hash_map::Keys;
use std::sync::Arc;

/// Model evaluator.
pub struct ModelEvaluator {
  /// Input data evaluator.
  input_data_evaluator: InputDataEvaluator,
  /// Item definition evaluator.
  item_definition_evaluator: ItemDefinitionEvaluator,
  /// Business knowledge model evaluator.
  business_knowledge_model_evaluator: BusinessKnowledgeModelEvaluator,
  /// Decision evaluator.
  decision_evaluator: DecisionEvaluator,
  /// Decision service evaluator.
  decision_service_evaluator: DecisionServiceEvaluator,
  /// Map of invocables indexed by invocable name.
  invocables: Invocables,
}

impl From<ModelBuilder> for ModelEvaluator {
  /// Creates [ModelEvaluator] from provided [ModelBuilder].
  fn from(model_builder: ModelBuilder) -> Self {
    let (input_data_evaluator, item_definition_evaluator, business_knowledge_model_evaluator, decision_evaluator, decision_service_evaluator, invocables) = model_builder.into();
    Self {
      input_data_evaluator,
      item_definition_evaluator,
      business_knowledge_model_evaluator,
      decision_evaluator,
      decision_service_evaluator,
      invocables,
    }
  }
}

impl ModelEvaluator {
  /// Creates an instance of [ModelEvaluator] from parsed [Definitions].
  pub fn new(definitions: &Definitions) -> Result<Arc<Self>> {
    let mut model_builder = ModelBuilder::default();
    model_builder.add_model(definitions);
    model_builder.build()?;
    let model_evaluator: Arc<ModelEvaluator> = Arc::new(model_builder.into());
    model_evaluator.decision_service_evaluator.build_function_definitions(&Arc::clone(&model_evaluator));
    Ok(model_evaluator)
  }

  /// Returns a reference to input data evaluator.
  pub fn input_data_evaluator(&self) -> &InputDataEvaluator {
    &self.input_data_evaluator
  }

  /// Returns a reference to item definition evaluator.
  pub fn item_definition_evaluator(&self) -> &ItemDefinitionEvaluator {
    &self.item_definition_evaluator
  }

  /// Returns a reference to business knowledge model evaluator.
  pub fn business_knowledge_model_evaluator(&self) -> &BusinessKnowledgeModelEvaluator {
    &self.business_knowledge_model_evaluator
  }

  /// Returns a reference to decision evaluator.
  pub fn decision_evaluator(&self) -> &DecisionEvaluator {
    &self.decision_evaluator
  }

  /// Returns a reference to decision service evaluator.
  pub fn decision_service_evaluator(&self) -> &DecisionServiceEvaluator {
    &self.decision_service_evaluator
  }

  /// Returns invocable names available in this model.
  pub fn invocable_names(&self) -> Keys<'_, String, InvocableType> {
    self.invocables.keys()
  }

  /// Evaluates an invocable identified by specified `invocable_name`.
  pub fn evaluate_invocable(&self, invocable_name: &str, input_data: &FeelContext) -> Value {
    let invocable = self.invocables.get(invocable_name);
    match invocable {
      Some(InvocableType::Decision(def_key)) => {
        // evaluate decision
        self.evaluate_decision(def_key, input_data)
      }
      Some(InvocableType::BusinessKnowledgeModel(def_key, output_variable_name)) => {
        // evaluate business knowledge model
        self.evaluate_bkm(def_key, input_data, output_variable_name)
      }
      Some(InvocableType::DecisionService(def_key)) => {
        // evaluate decision service
        self.evaluate_decision_service(def_key, input_data)
      }
      None => value_null!("invocable with name '{}' not found", invocable_name),
    }
  }

  /// Evaluates a decision.
  fn evaluate_decision(&self, def_key: &DefKey, input_data: &FeelContext) -> Value {
    let mut evaluated_ctx = FeelContext::default();
    if let Some(output_variable_name) = self.decision_evaluator.evaluate(def_key, input_data, self, &mut evaluated_ctx) {
      if let Some(output_value) = evaluated_ctx.get_entry(&output_variable_name) {
        output_value.clone()
      } else {
        value_null!()
      }
    } else {
      value_null!()
    }
  }

  /// Evaluates a business knowledge model.
  fn evaluate_bkm(&self, def_key: &DefKey, input_data: &FeelContext, output_variable_name: &Name) -> Value {
    let mut evaluated_ctx = FeelContext::default();
    self.business_knowledge_model_evaluator.evaluate(def_key, input_data, self, &mut evaluated_ctx);
    if let Some(Value::FunctionDefinition(parameters, body, _external, _, closure_ctx, result_type)) = evaluated_ctx.get_entry(output_variable_name) {
      //TODO Handle external functions.
      let mut parameters_ctx = FeelContext::default();
      parameters_ctx.zip(closure_ctx);
      for (name, _) in parameters {
        if let Some(value) = input_data.get_entry(name) {
          parameters_ctx.set_entry(name, value.to_owned());
        }
      }
      parameters_ctx.zip(&evaluated_ctx);
      let result = body.evaluate(&parameters_ctx.into());
      result.coerced(result_type)
    } else {
      value_null!()
    }
  }

  /// Evaluates a decision service.
  fn evaluate_decision_service(&self, def_key: &DefKey, input_data: &FeelContext) -> Value {
    let mut evaluated_ctx = FeelContext::default();
    if let Some(output_variable_name) = self.decision_service_evaluator.evaluate(def_key, input_data, self, &mut evaluated_ctx) {
      if let Some(output_value) = evaluated_ctx.get_entry(&output_variable_name) {
        output_value.clone()
      } else {
        value_null!()
      }
    } else {
      value_null!()
    }
  }
}
