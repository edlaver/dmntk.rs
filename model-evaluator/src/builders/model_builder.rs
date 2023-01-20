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

use crate::builders::model_definitions::DefDefinitions;
use crate::evaluators::model_evaluator::InvocableType;
use crate::evaluators::*;
use dmntk_common::Result;
use dmntk_feel::Name;
use dmntk_model::model::Definitions;
use std::cell::RefCell;
use std::collections::HashMap;

/// Model builder.
#[derive(Default)]
pub struct ModelBuilder {
  /// Input data evaluator builder.
  input_data_evaluator_builder: InputDataEvaluator,
  /// Input data context evaluator builder.
  input_data_context_evaluator_builder: InputDataContextEvaluator,
  /// Item definition evaluator builder.
  item_definition_evaluator_builder: ItemDefinitionEvaluator,
  /// Item definition context evaluator builder.
  item_definition_context_evaluator_builder: ItemDefinitionContextEvaluator,
  /// Item definition type evaluator builder.
  item_definition_type_evaluator_builder: ItemDefinitionTypeEvaluator,
  /// Business knowledge model evaluator builder.
  business_knowledge_model_evaluator_builder: BusinessKnowledgeModelEvaluator,
  /// Decision evaluator builder
  decision_evaluator_builder: DecisionEvaluator,
  /// Decision service evaluator builder.
  decision_service_evaluator_builder: DecisionServiceEvaluator,
  /// Map of invocables indexed by invocable name.
  invocable_by_name: RefCell<HashMap<String, InvocableType>>,
}

impl ModelBuilder {
  ///
  pub fn new(defs: &Definitions) -> Result<Self> {
    let definitions: DefDefinitions = defs.into();
    let model_builder = ModelBuilder::default();
    model_builder.input_data_evaluator_builder.build(&definitions)?;
    model_builder.input_data_context_evaluator_builder.build(&definitions)?;
    model_builder.item_definition_evaluator_builder.build(&definitions)?;
    model_builder.item_definition_context_evaluator_builder.build(&definitions)?;
    model_builder.item_definition_type_evaluator_builder.build(&definitions)?;
    model_builder.business_knowledge_model_evaluator_builder.build(&definitions, &model_builder)?;
    model_builder.decision_evaluator_builder.build(&definitions, &model_builder)?;
    model_builder.decision_service_evaluator_builder.build(&definitions, &model_builder)?;
    Ok(model_builder)
  }

  ///
  pub fn input_data_evaluator(&self) -> &InputDataEvaluator {
    &self.input_data_evaluator_builder
  }

  ///
  pub fn input_data_context_evaluator(&self) -> &InputDataContextEvaluator {
    &self.input_data_context_evaluator_builder
  }

  ///
  pub fn item_definition_context_evaluator(&self) -> &ItemDefinitionContextEvaluator {
    &self.item_definition_context_evaluator_builder
  }

  ///
  pub fn item_definition_evaluator(&self) -> &ItemDefinitionEvaluator {
    &self.item_definition_evaluator_builder
  }

  ///
  pub fn item_definition_type_evaluator(&self) -> &ItemDefinitionTypeEvaluator {
    &self.item_definition_type_evaluator_builder
  }

  ///
  pub fn business_knowledge_model_evaluator(&self) -> &BusinessKnowledgeModelEvaluator {
    &self.business_knowledge_model_evaluator_builder
  }

  ///
  pub fn decision_evaluator(&self) -> &DecisionEvaluator {
    &self.decision_evaluator_builder
  }

  ///
  pub fn decision_service_evaluator(&self) -> &DecisionServiceEvaluator {
    &self.decision_service_evaluator_builder
  }

  ///
  pub fn add_invocable_decision(&self, name: &str, id: &str) {
    self.invocable_by_name.borrow_mut().insert(name.to_string(), InvocableType::Decision(id.to_string()));
  }

  ///
  pub fn add_invocable_business_knowledge_model(&self, name: &str, id: &str, output_variable_name: Name) {
    self
      .invocable_by_name
      .borrow_mut()
      .insert(name.to_string(), InvocableType::BusinessKnowledgeModel(id.to_string(), output_variable_name));
  }

  ///
  pub fn add_invocable_decision_service(&self, name: &str, id: &str) {
    self.invocable_by_name.borrow_mut().insert(name.to_string(), InvocableType::DecisionService(id.to_string()));
  }
}
