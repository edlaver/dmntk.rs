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

//! # TCK handler

use super::dto::{InputNodeDto, OutputNodeDto, WrappedValue};
use super::errors::err_missing_parameter;
use crate::data::ApplicationData;
use actix_web::web::Json;
use actix_web::{post, web};
use dmntk_common::DmntkError;
use dmntk_feel::context::FeelContext;
use dmntk_workspace::Workspace;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::{fmt, io};

/// Data transfer object for an error.
#[derive(Serialize)]
pub struct TckErrorDto {
  /// Error details.
  #[serde(rename = "detail")]
  detail: String,
}

/// Data transfer object for a result.
#[derive(Serialize)]
pub struct TckResultDto<T> {
  /// Result containing data.
  #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
  data: Option<T>,
  /// Result containing errors.
  #[serde(rename = "errors", skip_serializing_if = "Vec::is_empty")]
  errors: Vec<TckErrorDto>,
}

impl<T> Default for TckResultDto<T> {
  /// Creates default result structure.
  fn default() -> Self {
    Self { data: None, errors: vec![] }
  }
}

impl<T: Serialize> ToString for TckResultDto<T> {
  /// Converts result to JSON string.
  fn to_string(&self) -> String {
    serde_json::to_string(self).unwrap_or("conversion to JSON failed for ResultDto".to_string())
  }
}

impl<T> TckResultDto<T> {
  /// Utility function for creating a result with some data inside.
  pub fn data(d: T) -> TckResultDto<T> {
    TckResultDto {
      data: Some(d),
      ..Default::default()
    }
  }

  /// Utility function for creating a result with a single error inside.
  pub fn error(err: impl fmt::Display) -> TckResultDto<T> {
    TckResultDto {
      errors: vec![TckErrorDto { detail: format!("{err}") }],
      ..Default::default()
    }
  }
}

/// Parameters for evaluating invocable in DMN™ model definitions.
/// The format of input data is compatible with test cases
/// defined in [Technology Compatibility Kit for DMN standard](https://github.com/dmn-tck/tck).
#[derive(Deserialize)]
pub struct TckEvaluateParams {
  /// Name of the namespace where the model will be searched.
  #[serde(rename = "namespace")]
  model_namespace: Option<String>,
  /// Name of the model in which the invocable will be searched.
  #[serde(rename = "model")]
  model_name: Option<String>,
  /// Name of the invocable to be evaluated.
  #[serde(rename = "invocable")]
  invocable_name: Option<String>,
  /// Collection of input values.
  #[serde(rename = "input")]
  input_values: Option<Vec<InputNodeDto>>,
}

/// Handler for evaluating models with input data in the format compatible with test cases
/// defined in [Technology Compatibility Kit for DMN standard](https://github.com/dmn-tck/tck).
#[post("/tck/evaluate")]
pub async fn post_tck_evaluate(params: Json<TckEvaluateParams>, data: web::Data<ApplicationData>) -> io::Result<Json<TckResultDto<OutputNodeDto>>> {
  let workspace: &Workspace = data.workspace.borrow();
  match do_evaluate_tck(workspace, &params.into_inner()) {
    Ok(response) => Ok(Json(TckResultDto::data(response))),
    Err(reason) => Ok(Json(TckResultDto::error(reason))),
  }
}

/// Evaluates the invocable in model and returns the result.
/// Input and output data format is compatible with
/// [Technology Compatibility Kit for DMN standard](https://github.com/dmn-tck/tck).
fn do_evaluate_tck(workspace: &Workspace, params: &TckEvaluateParams) -> Result<OutputNodeDto, DmntkError> {
  if let Some(model_namespace) = &params.model_namespace {
    if let Some(model_name) = &params.model_name {
      if let Some(invocable_name) = &params.invocable_name {
        if let Some(input_values) = &params.input_values {
          // convert input values into FEEL context
          let input_data = FeelContext::try_from(WrappedValue::try_from(input_values)?.0)?;
          // evaluate artifact with specified name
          workspace.evaluate_invocable_by_name(model_namespace, model_name, invocable_name, &input_data)?.try_into()
        } else {
          Err(err_missing_parameter("input"))
        }
      } else {
        Err(err_missing_parameter("invocable"))
      }
    } else {
      Err(err_missing_parameter("model"))
    }
  } else {
    Err(err_missing_parameter("namespace"))
  }
}
