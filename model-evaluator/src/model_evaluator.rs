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

use crate::builders::*;
use crate::errors::*;
use dmntk_common::Result;
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value;
use dmntk_feel::{value_null, Name};
use dmntk_model::model::Definitions;
use dmntk_model::ModelDefinitions;
use std::collections::HashMap;
use std::sync::{Arc, RwLock, RwLockReadGuard};

/// Types of invocable artefacts.
pub enum InvocableType {
  Decision(String),
  BusinessKnowledgeModel(String, Name),
  DecisionService(String),
}

/// Model evaluator.
#[derive(Default)]
pub struct ModelEvaluator {
  /// Input data evaluator.
  input_data_evaluator: RwLock<InputDataEvaluator>,
  /// Input data context evaluator.
  input_data_context_evaluator: RwLock<InputDataContextEvaluator>,
  /// Item definition evaluator.
  item_definition_evaluator: RwLock<ItemDefinitionEvaluator>,
  /// Item definition context evaluator.
  item_definition_context_evaluator: RwLock<ItemDefinitionContextEvaluator>,
  ///Item definition type evaluator.
  item_definition_type_evaluator: RwLock<ItemDefinitionTypeEvaluator>,
  /// Business knowledge model evaluator.
  business_knowledge_model_evaluator: RwLock<BusinessKnowledgeModelEvaluator>,
  /// Decision evaluator.
  decision_evaluator: RwLock<DecisionEvaluator>,
  /// Decision service evaluator.
  decision_service_evaluator: RwLock<DecisionServiceEvaluator>,
  /// Map of [InvocableType] indexed by invocable (decision, business knowledge model or decision service) name.
  invocable_by_name: RwLock<HashMap<String, InvocableType>>,
}

impl ModelEvaluator {
  /// Creates an instance of [ModelEvaluator].
  pub fn new(definitions: &Definitions) -> Result<Arc<Self>> {
    let model_definitions: ModelDefinitions = definitions.into();
    let model_evaluator = Arc::new(ModelEvaluator::default());
    model_evaluator.input_data_evaluator.write().unwrap().build(&model_definitions)?;
    model_evaluator.input_data_context_evaluator.write().unwrap().build(&model_definitions)?;
    model_evaluator.item_definition_evaluator.write().unwrap().build(&model_definitions)?;
    model_evaluator.item_definition_context_evaluator.write().unwrap().build(&model_definitions)?;
    model_evaluator.item_definition_type_evaluator.write().unwrap().build(&model_definitions)?;
    model_evaluator
      .business_knowledge_model_evaluator
      .write()
      .unwrap()
      .build(&model_definitions, &model_evaluator)?;
    model_evaluator.decision_evaluator.write().unwrap().build(&model_definitions, &model_evaluator)?;
    model_evaluator
      .decision_service_evaluator
      .write()
      .unwrap()
      .build(&model_definitions, Arc::clone(&model_evaluator))?;
    model_evaluator.decision_service_evaluator.write().unwrap().build_function_definitions(&model_evaluator);
    Ok(model_evaluator)
  }

  /// Creates an instance of [ModelEvaluator] from multiple definitions.
  pub fn new_1(defs: &Vec<(Option<Name>, &Definitions)>) -> Result<Arc<Self>> {
    let model_evaluator = Arc::new(ModelEvaluator::default());
    let model_definitions: ModelDefinitions = defs.into();
    model_evaluator.input_data_evaluator.write().unwrap().build(&model_definitions)?;
    model_evaluator.input_data_context_evaluator.write().unwrap().build(&model_definitions)?;
    model_evaluator.item_definition_evaluator.write().unwrap().build(&model_definitions)?;
    model_evaluator.item_definition_context_evaluator.write().unwrap().build(&model_definitions)?;
    model_evaluator.item_definition_type_evaluator.write().unwrap().build(&model_definitions)?;
    model_evaluator
      .business_knowledge_model_evaluator
      .write()
      .unwrap()
      .build(&model_definitions, &model_evaluator)?;
    model_evaluator.decision_evaluator.write().unwrap().build(&model_definitions, &model_evaluator)?;
    model_evaluator
      .decision_service_evaluator
      .write()
      .unwrap()
      .build(&model_definitions, Arc::clone(&model_evaluator))?;
    model_evaluator.decision_service_evaluator.write().unwrap().build_function_definitions(&model_evaluator);
    Ok(model_evaluator)
  }

  ///
  pub fn input_data_evaluator(&self) -> Result<RwLockReadGuard<InputDataEvaluator>> {
    self.input_data_evaluator.read().map_err(err_read_lock_failed)
  }

  ///
  pub fn input_data_context_evaluator(&self) -> Result<RwLockReadGuard<InputDataContextEvaluator>> {
    self.input_data_context_evaluator.read().map_err(err_read_lock_failed)
  }

  ///
  pub fn item_definition_context_evaluator(&self) -> Result<RwLockReadGuard<ItemDefinitionContextEvaluator>> {
    self.item_definition_context_evaluator.read().map_err(err_read_lock_failed)
  }

  ///
  pub fn item_definition_evaluator(&self) -> Result<RwLockReadGuard<ItemDefinitionEvaluator>> {
    self.item_definition_evaluator.read().map_err(err_read_lock_failed)
  }

  ///
  pub fn item_definition_type_evaluator(&self) -> Result<RwLockReadGuard<ItemDefinitionTypeEvaluator>> {
    self.item_definition_type_evaluator.read().map_err(err_read_lock_failed)
  }

  ///
  pub fn business_knowledge_model_evaluator(&self) -> Result<RwLockReadGuard<BusinessKnowledgeModelEvaluator>> {
    self.business_knowledge_model_evaluator.read().map_err(err_read_lock_failed)
  }

  ///
  pub fn decision_service_evaluator(&self) -> Result<RwLockReadGuard<DecisionServiceEvaluator>> {
    self.decision_service_evaluator.read().map_err(err_read_lock_failed)
  }

  ///
  pub fn decision_evaluator(&self) -> Result<RwLockReadGuard<DecisionEvaluator>> {
    self.decision_evaluator.read().map_err(err_read_lock_failed)
  }

  /// Evaluates an invocable with specified name.
  pub fn evaluate_invocable(&self, invocable_name: &str, input_data: &FeelContext) -> Value {
    if let Ok(invocable_by_name) = self.invocable_by_name.read() {
      let invocable = invocable_by_name.get(invocable_name);
      match invocable {
        Some(InvocableType::Decision(id)) => {
          // evaluate decision
          self.evaluate_decision(id, input_data)
        }
        Some(InvocableType::BusinessKnowledgeModel(id, output_variable_name)) => {
          // evaluate business knowledge model
          self.evaluate_business_knowledge_model(id, input_data, output_variable_name)
        }
        Some(InvocableType::DecisionService(id)) => {
          // evaluate decision service
          self.evaluate_decision_service(id, input_data)
        }
        None => value_null!("invocable with name '{}' not found", invocable_name),
      }
    } else {
      value_null!("write lock failed when acquiring invocable_by_name map")
    }
  }

  ///
  pub fn add_invocable_decision(&self, name: &str, id: &str) {
    if let Ok(mut invocable_by_name) = self.invocable_by_name.write() {
      invocable_by_name.insert(name.to_string(), InvocableType::Decision(id.to_string()));
    }
  }

  ///
  pub fn add_invocable_business_knowledge_model(&self, name: &str, id: &str, output_variable_name: Name) {
    if let Ok(mut invocable_by_name) = self.invocable_by_name.write() {
      invocable_by_name.insert(name.to_string(), InvocableType::BusinessKnowledgeModel(id.to_string(), output_variable_name));
    }
  }

  ///
  pub fn add_invocable_decision_service(&self, name: &str, id: &str) {
    if let Ok(mut invocable_by_name) = self.invocable_by_name.write() {
      invocable_by_name.insert(name.to_string(), InvocableType::DecisionService(id.to_string()));
    }
  }

  /// Evaluates a business knowledge model.
  pub fn evaluate_business_knowledge_model(&self, id: &str, input_data: &FeelContext, output_variable_name: &Name) -> Value {
    if let Ok(business_knowledge_model_evaluator) = self.business_knowledge_model_evaluator() {
      let mut evaluated_ctx = FeelContext::default();
      business_knowledge_model_evaluator.evaluate(id, input_data, self, &mut evaluated_ctx);
      if let Some(Value::FunctionDefinition(parameters, body, _external, _, closure_ctx, result_type)) = evaluated_ctx.get_entry(output_variable_name) {
        //TODO handle external
        let mut parameters_ctx = FeelContext::default();
        parameters_ctx.zip(closure_ctx);
        for (name, _) in parameters {
          if let Some(value) = input_data.get_entry(name) {
            parameters_ctx.set_entry(name, value.to_owned());
          }
        }
        parameters_ctx.zip(&evaluated_ctx);
        let result = body.evaluate(&parameters_ctx.into());
        result_type.coerced(&result)
      } else {
        value_null!()
      }
    } else {
      value_null!()
    }
  }

  /// Evaluates a decision.
  pub fn evaluate_decision(&self, id: &str, input_data: &FeelContext) -> Value {
    if let Ok(decision_evaluator) = self.decision_evaluator() {
      let mut evaluated_ctx = FeelContext::default();
      if let Some(output_variable_name) = decision_evaluator.evaluate(id, input_data, self, &mut evaluated_ctx) {
        if let Some(output_value) = evaluated_ctx.get_entry(&output_variable_name) {
          output_value.clone()
        } else {
          value_null!()
        }
      } else {
        value_null!()
      }
    } else {
      value_null!()
    }
  }

  /// Evaluates a decision service.
  pub fn evaluate_decision_service(&self, id: &str, input_data: &FeelContext) -> Value {
    if let Ok(decision_service_evaluator) = self.decision_service_evaluator() {
      let mut evaluated_ctx = FeelContext::default();
      if let Some(output_variable_name) = decision_service_evaluator.evaluate(id, input_data, self, &mut evaluated_ctx) {
        if let Some(output_value) = evaluated_ctx.get_entry(&output_variable_name) {
          output_value.clone()
        } else {
          value_null!()
        }
      } else {
        value_null!()
      }
    } else {
      value_null!()
    }
  }
}
