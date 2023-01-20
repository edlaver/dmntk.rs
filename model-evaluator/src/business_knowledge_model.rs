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

//! Builder for business knowledge model evaluators.

use crate::boxed_expressions::*;
use crate::errors::*;
use crate::model_builder::ModelBuilder;
use crate::model_definitions::{DefBusinessKnowledgeModel, DefDefinitions};
use crate::model_evaluator::ModelEvaluator;
use dmntk_common::Result;
use dmntk_feel::closure::Closure;
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value;
use dmntk_feel::{FeelScope, FeelType, FunctionBody, Name};
use dmntk_model::model::*;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Type of closure that evaluates business knowledge model.
/// Fn(input data, model evaluator, output data)
type BusinessKnowledgeModelEvaluatorFn = Box<dyn Fn(&FeelContext, &ModelEvaluator, &mut FeelContext) + Send + Sync>;

/// Business knowledge model evaluator.
#[derive(Default)]
pub struct BusinessKnowledgeModelEvaluator {
  evaluators: RwLock<HashMap<String, BusinessKnowledgeModelEvaluatorFn>>,
}

impl BusinessKnowledgeModelEvaluator {
  /// Creates a new business knowledge model evaluator.
  pub fn build(&self, definitions: &DefDefinitions, model_builder: &ModelBuilder) -> Result<()> {
    for business_knowledge_model in definitions.business_knowledge_models() {
      let function_definition = business_knowledge_model.encapsulated_logic().as_ref().ok_or_else(err_empty_encapsulated_logic)?;
      let evaluator = build_bkm_evaluator(definitions, business_knowledge_model, function_definition, model_builder)?;
      let business_knowledge_model_id = business_knowledge_model.id();
      let business_knowledge_model_name = &business_knowledge_model.name().to_string();
      let output_variable_name = business_knowledge_model.variable().name();
      self
        .evaluators
        .write()
        .map_err(err_write_lock_failed)?
        .insert(business_knowledge_model_id.to_owned(), evaluator);
      model_builder.add_invocable_business_knowledge_model(business_knowledge_model_name, business_knowledge_model_id, output_variable_name.to_owned());
    }
    Ok(())
  }
  /// Evaluates a business knowledge model with specified identifier.
  /// When a required business knowledge model is found, then its evaluator
  /// is executed, and the result is stored in `evaluated_ctx`.
  pub fn evaluate(&self, business_knowledge_model_id: &str, input_data: &FeelContext, model_evaluator: &ModelEvaluator, output_data: &mut FeelContext) {
    if let Ok(evaluators) = self.evaluators.read() {
      if let Some(evaluator) = evaluators.get(business_knowledge_model_id) {
        evaluator(input_data, model_evaluator, output_data);
      }
    }
  }
}

///
fn build_bkm_evaluator(
  definitions: &DefDefinitions,
  business_knowledge_model: &DefBusinessKnowledgeModel,
  function_definition: &FunctionDefinition,
  model_builder: &ModelBuilder,
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  let item_definition_type_evaluator = model_builder.item_definition_type_evaluator();
  let mut local_context = FeelContext::default();
  let mut formal_parameters = vec![];
  for information_item in function_definition.formal_parameters() {
    let feel_type = if let Some(type_ref) = information_item.type_ref() {
      item_definition_type_evaluator.information_item_type(type_ref).ok_or_else(err_empty_feel_type)?
    } else {
      FeelType::Any
    };
    let feel_name = information_item.feel_name();
    formal_parameters.push((feel_name.clone(), feel_type.clone()));
    local_context.set_entry(feel_name, Value::FeelType(feel_type));
  }
  let output_variable_name = business_knowledge_model.variable().name().clone();
  let output_variable_type = if let Some(output_variable_type_ref) = business_knowledge_model.variable().type_ref().as_ref() {
    item_definition_type_evaluator.information_item_type(output_variable_type_ref).unwrap_or(FeelType::Any)
  } else {
    FeelType::Any
  };
  let mut knowledge_requirements = vec![];
  for knowledge_requirement in business_knowledge_model.knowledge_requirements() {
    let href = knowledge_requirement.required_knowledge().as_ref().ok_or_else(err_empty_reference)?;
    knowledge_requirements.push(href.into());
  }
  // bring into context the variables from knowledge requirements
  bring_knowledge_requirements_into_context(definitions, business_knowledge_model.knowledge_requirements(), &mut local_context)?;
  //TODO verify the above line - there was no such example in models
  if let Some(expression_instance) = function_definition.body() {
    let scope: FeelScope = local_context.into();
    build_bkm_expression_instance_evaluator(
      &scope,
      formal_parameters,
      expression_instance,
      output_variable_name,
      output_variable_type,
      knowledge_requirements,
      model_builder,
    )
  } else {
    Ok(Box::new(move |_: &FeelContext, _: &ModelEvaluator, _: &mut FeelContext| ()))
  }
}

///
fn build_bkm_expression_instance_evaluator(
  scope: &FeelScope,
  formal_parameters: Vec<(Name, FeelType)>,
  expression_instance: &ExpressionInstance,
  output_variable_name: Name,
  output_variable_type: FeelType,
  knowledge_requirements: Vec<String>,
  model_builder: &ModelBuilder,
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  match expression_instance {
    ExpressionInstance::Context(context) => {
      //
      build_bkm_context_evaluator(
        scope,                  //
        formal_parameters,      //
        context,                //
        output_variable_name,   //
        output_variable_type,   //
        knowledge_requirements, //
        model_builder,          //
      )
    }
    ExpressionInstance::DecisionTable(decision_table) => {
      //
      build_bkm_decision_table_evaluator(
        scope,                  //
        formal_parameters,      //
        decision_table,         //
        output_variable_name,   //
        output_variable_type,   //
        knowledge_requirements, //
      )
    }
    ExpressionInstance::FunctionDefinition(function_definition) => {
      //
      build_bkm_function_definition_evaluator(
        scope,                  //
        formal_parameters,      //
        function_definition,    //
        output_variable_name,   //
        output_variable_type,   //
        knowledge_requirements, //
        model_builder,          //
      )
    }
    ExpressionInstance::Invocation(invocation) => {
      //
      build_bkm_invocation_evaluator(
        scope,                  //
        formal_parameters,      //
        invocation,             //
        output_variable_name,   //
        output_variable_type,   //
        knowledge_requirements, //
        model_builder,          //
      )
    }
    ExpressionInstance::LiteralExpression(literal_expression) => {
      //
      build_bkm_literal_expression_evaluator(
        scope,                  //
        formal_parameters,      //
        literal_expression,     //
        output_variable_name,   //
        output_variable_type,   //
        knowledge_requirements, //
      )
    }
    ExpressionInstance::Relation(relation) => {
      //
      build_bkm_relation_evaluator(
        scope,                  //
        formal_parameters,      //
        relation,               //
        output_variable_name,   //
        output_variable_type,   //
        knowledge_requirements, //
        model_builder,          //
      )
    }
  }
}

///
fn build_bkm_context_evaluator(
  scope: &FeelScope,
  formal_parameters: Vec<(Name, FeelType)>,
  context: &Context,
  output_variable_name: Name,
  output_variable_type: FeelType,
  knowledge_requirements: Vec<String>,
  model_builder: &ModelBuilder,
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  let (evaluator, _) = build_context_evaluator(scope, context, model_builder)?;
  let closure = Closure::default();
  let closure_ctx = FeelContext::default();
  let function = Value::FunctionDefinition(
    formal_parameters,
    FunctionBody::Context(Arc::new(evaluator)),
    false,
    closure,
    closure_ctx,
    output_variable_type,
  );
  build_bkm_evaluator_from_function_definition(output_variable_name, function, knowledge_requirements)
}

///
fn build_bkm_decision_table_evaluator(
  scope: &FeelScope,
  formal_parameters: Vec<(Name, FeelType)>,
  decision_table: &DecisionTable,
  output_variable_name: Name,
  output_variable_type: FeelType,
  knowledge_requirements: Vec<String>,
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  let (evaluator, _) = build_decision_table_evaluator(scope, decision_table)?;
  let closure = Closure::default();
  let closure_ctx = FeelContext::default();
  let function = Value::FunctionDefinition(
    formal_parameters,
    FunctionBody::DecisionTable(Arc::new(evaluator)),
    false,
    closure,
    closure_ctx,
    output_variable_type,
  );
  build_bkm_evaluator_from_function_definition(output_variable_name, function, knowledge_requirements)
}

///
fn build_bkm_function_definition_evaluator(
  scope: &FeelScope,
  formal_parameters: Vec<(Name, FeelType)>,
  function_definition: &FunctionDefinition,
  output_variable_name: Name,
  output_variable_type: FeelType,
  knowledge_requirements: Vec<String>,
  model_builder: &ModelBuilder,
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  let (evaluator, _) = build_function_definition_evaluator(scope, function_definition, model_builder)?;
  let closure = Closure::default();
  let closure_ctx = FeelContext::default();
  let function = Value::FunctionDefinition(
    formal_parameters,
    FunctionBody::FunctionDefinition(Arc::new(evaluator)),
    false,
    closure,
    closure_ctx,
    output_variable_type,
  );
  build_bkm_evaluator_from_function_definition(output_variable_name, function, knowledge_requirements)
}

///
fn build_bkm_invocation_evaluator(
  scope: &FeelScope,
  formal_parameters: Vec<(Name, FeelType)>,
  invocation: &Invocation,
  output_variable_name: Name,
  output_variable_type: FeelType,
  knowledge_requirements: Vec<String>,
  model_builder: &ModelBuilder,
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  let (evaluator, _) = build_invocation_evaluator(scope, invocation, model_builder)?;
  let closure = Closure::default();
  let closure_ctx = FeelContext::default();
  let function = Value::FunctionDefinition(
    formal_parameters,
    FunctionBody::Invocation(Arc::new(evaluator)),
    false,
    closure,
    closure_ctx,
    output_variable_type,
  );
  build_bkm_evaluator_from_function_definition(output_variable_name, function, knowledge_requirements)
}

///
fn build_bkm_literal_expression_evaluator(
  scope: &FeelScope,
  formal_parameters: Vec<(Name, FeelType)>,
  literal_expression: &LiteralExpression,
  output_variable_name: Name,
  output_variable_type: FeelType,
  knowledge_requirements: Vec<String>,
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  let (evaluator, _) = build_literal_expression_evaluator(scope, literal_expression)?;
  let closure = Closure::default();
  let closure_ctx = FeelContext::default();
  let function = Value::FunctionDefinition(
    formal_parameters,
    FunctionBody::LiteralExpression(Arc::new(evaluator)),
    false,
    closure,
    closure_ctx,
    output_variable_type,
  );
  build_bkm_evaluator_from_function_definition(output_variable_name, function, knowledge_requirements)
}

///
fn build_bkm_relation_evaluator(
  scope: &FeelScope,
  formal_parameters: Vec<(Name, FeelType)>,
  relation: &Relation,
  output_variable_name: Name,
  output_variable_type: FeelType,
  knowledge_requirements: Vec<String>,
  model_builder: &ModelBuilder,
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  let (evaluator, _) = build_relation_evaluator(scope, relation, model_builder)?;
  let closure = Closure::default();
  let closure_ctx = FeelContext::default();
  let function_definition = Value::FunctionDefinition(
    formal_parameters,
    FunctionBody::Relation(Arc::new(evaluator)),
    false,
    closure,
    closure_ctx,
    output_variable_type,
  );
  build_bkm_evaluator_from_function_definition(output_variable_name, function_definition, knowledge_requirements)
}

///
fn build_bkm_evaluator_from_function_definition(
  output_variable_name: Name,
  function_definition: Value,
  knowledge_requirements: Vec<String>,
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  Ok(Box::new(
    move |input_data: &FeelContext, model_evaluator: &ModelEvaluator, output_data: &mut FeelContext| {
      let business_knowledge_model_evaluator = model_evaluator.business_knowledge_model_evaluator();
      let decision_service_evaluator = model_evaluator.decision_service_evaluator();
      knowledge_requirements.iter().for_each(|id| {
        // TODO refactor: call either business knowledge model or decision service, but not both!
        business_knowledge_model_evaluator.evaluate(id, input_data, model_evaluator, output_data);
        decision_service_evaluator.evaluate(id, input_data, model_evaluator, output_data);
      });
      output_data.set_entry(&output_variable_name, function_definition.clone())
    },
  ))
}