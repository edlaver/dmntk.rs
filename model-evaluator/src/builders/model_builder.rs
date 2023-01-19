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

use crate::builders::item_definition_type::ItemDefinitionTypeEvaluatorBuilder;
use crate::builders::model_definitions::DefDefinitions;
use dmntk_common::Result;
use dmntk_model::model::Definitions;

/// Model builder.
#[derive(Default)]
pub struct ModelBuilder {
  // /// Input data evaluator builder.
  // input_data_evaluator_builder: InputDataEvaluatorBuilder,
  // /// Input data context evaluator builder.
  // input_data_context_evaluator_builder: InputDataContextEvaluatorBuilder,
  // /// Item definition evaluator builder.
  // item_definition_evaluator_builder: ItemDefinitionEvaluatorBuilder,
  // /// Item definition context evaluator builder.
  // item_definition_context_evaluator_builder: ItemDefinitionContextEvaluatorBuilder,
  /// Item definition type evaluator builder.
  item_definition_type_evaluator_builder: ItemDefinitionTypeEvaluatorBuilder,
  // /// Business knowledge model evaluator builder.
  // business_knowledge_model_evaluator_builder: BusinessKnowledgeModelEvaluatorBuilder,
  // /// Decision evaluator builder
  // decision_evaluator_builder: DecisionEvaluatorBuilder,
  // /// Decision service evaluator builder.
  // decision_service_evaluator_builder: DecisionServiceEvaluatorBuilder,
  // /// Map of [InvocableType] indexed by invocable (decision, business knowledge model or decision service) name.
  // invocable_by_name: HashMap<String, InvocableType>,
}

impl ModelBuilder {
  ///
  pub fn new(defs: &Definitions) -> Result<Self> {
    let definitions: DefDefinitions = defs.into();
    let mut model_builder = ModelBuilder::default();
    // model_builder.input_data_evaluator_builder.build(definitions)?;
    // model_builder.input_data_context_evaluator_builder.build(definitions)?;
    // model_builder.item_definition_evaluator_builder.build(definitions)?;
    // model_builder.item_definition_context_evaluator_builder.build(definitions)?;
    model_builder.item_definition_type_evaluator_builder.build(&definitions)?;
    Ok(model_builder)
  }
  /*

   ///
   pub fn input_data_evaluator(&self) -> &InputDataEvaluatorBuilder {
     &self.input_data_evaluator_builder
   }

   ///
   pub fn input_data_context_evaluator(&self) -> &InputDataContextEvaluatorBuilder {
     &self.input_data_context_evaluator_builder
   }

   ///
   pub fn item_definition_context_evaluator(&self) -> &ItemDefinitionContextEvaluatorBuilder {
     &self.item_definition_context_evaluator_builder
   }

   ///
   pub fn item_definition_evaluator(&self) -> &ItemDefinitionEvaluatorBuilder {
     &self.item_definition_evaluator_builder
   }

   ///
   pub fn item_definition_type_evaluator(&self) -> &ItemDefinitionTypeEvaluatorBuilder {
     &self.item_definition_type_evaluator_builder
   }

   ///
   pub fn business_knowledge_model_evaluator(&self) -> &BusinessKnowledgeModelEvaluatorBuilder {
     &self.business_knowledge_model_evaluator_builder
   }

   ///
   pub fn decision_evaluator(&self) -> &DecisionEvaluatorBuilder {
     &self.decision_evaluator_builder
   }

   ///
   pub fn decision_service_evaluator(&self) -> &DecisionServiceEvaluatorBuilder {
     &self.decision_service_evaluator_builder
   }

   ///
   pub fn add_invocable_decision(&mut self, name: &str, id: &str) {
     self.invocable_by_name.insert(name.to_string(), InvocableType::Decision(id.to_string()));
   }

   ///
   pub fn add_invocable_business_knowledge_model(&mut self, name: &str, id: &str, output_variable_name: Name) {
     self
       .invocable_by_name
       .insert(name.to_string(), InvocableType::BusinessKnowledgeModel(id.to_string(), output_variable_name));
   }

   ///
   pub fn add_invocable_decision_service(&mut self, name: &str, id: &str) {
     self.invocable_by_name.insert(name.to_string(), InvocableType::DecisionService(id.to_string()));
   }

  */
}
