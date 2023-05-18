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

use crate::dto::{InputNodeDto, OutputNodeDto, WrappedValue};
use crate::errors::*;
use actix_web::web::Json;
use actix_web::{error, get, post, web, App, HttpResponse, HttpServer};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use dmntk_common::{DmntkError, Jsonify, Result};
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value;
use dmntk_feel::FeelScope;
use dmntk_workspace::{EvaluatorStatus, Workspace};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::RwLock;
use std::{env, io};

const DMNTK_NAME: &str = env!("CARGO_PKG_NAME");
const DMNTK_VERSION: &str = env!("CARGO_PKG_VERSION");
const DMNTK_COPYRIGHT: &str = env!("CARGO_PKG_AUTHORS");
const DMNTK_DEFAULT_PORT: u16 = 22022;
const DMNTK_DEFAULT_HOST: &str = "0.0.0.0";
const DMNTK_HOST_VARIABLE: &str = "DMNTK_HOST";
const DMNTK_PORT_VARIABLE: &str = "DMNTK_PORT";
const DMNTK_DIR_VARIABLE: &str = "DMNTK_DIR";

/// Shared workspace with decision model definitions.
struct ApplicationData {
  workspace: RwLock<Workspace>,
}

/// Data transfer object for an error.
#[derive(Serialize)]
struct ErrorDto {
  /// Error details.
  #[serde(rename = "details")]
  details: String,
}

/// Data transfer object for a result.
#[derive(Serialize)]
struct ResultDto<T> {
  /// Result containing data.
  #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
  data: Option<T>,
  /// Result containing errors.
  #[serde(rename = "errors", skip_serializing_if = "Vec::is_empty")]
  errors: Vec<ErrorDto>,
}

impl<T> Default for ResultDto<T> {
  /// Creates default result structure.
  fn default() -> Self {
    Self { data: None, errors: vec![] }
  }
}

impl<T: Serialize> ToString for ResultDto<T> {
  /// Converts [ResultDto] to JSON string.
  fn to_string(&self) -> String {
    serde_json::to_string(self).unwrap_or("conversion to JSON failed for ResultDto".to_string())
  }
}

impl<T> ResultDto<T> {
  /// Creates [ResultDto] with some data inside.
  fn data(d: T) -> ResultDto<T> {
    ResultDto {
      data: Some(d),
      ..Default::default()
    }
  }
  /// Creates [ResultDto] with single error inside.
  fn error(err: DmntkError) -> ResultDto<T> {
    ResultDto {
      errors: vec![ErrorDto { details: format!("{err}") }],
      ..Default::default()
    }
  }
}

/// System information structure.
#[derive(Serialize)]
struct SystemInfoDto {
  /// System name.
  #[serde(rename = "name")]
  name: String,
  /// System version.
  #[serde(rename = "version")]
  version: String,
  /// Legal notice.
  #[serde(rename = "copyright")]
  copyright: String,
}

impl Default for SystemInfoDto {
  /// Creates default system information structure.
  fn default() -> Self {
    Self {
      name: DMNTK_NAME.to_string(),
      version: DMNTK_VERSION.to_string(),
      copyright: DMNTK_COPYRIGHT.to_string(),
    }
  }
}

/// Parameters for adding model definitions to workspace.
#[derive(Deserialize)]
struct AddDefinitionsParams {
  /// Content of the model, encoded in `Base64`.
  #[serde(rename = "content")]
  content: Option<String>,
}

/// Result data sent back to caller after adding definitions.
#[derive(Serialize)]
struct AddDefinitionsResult {
  /// RDNN of the added definitions.
  #[serde(rename = "rdnn")]
  model_rdnn: String,
  /// Namespace of the added definitions.
  #[serde(rename = "namespace")]
  model_namespace: String,
  /// Name of added definitions.
  #[serde(rename = "name")]
  model_name: String,
}

/// Result data sent back to caller after replacing definitions.
#[derive(Serialize)]
struct ReplaceDefinitionsResult {
  /// RDNN of the replaced definitions.
  #[serde(rename = "rdnn")]
  model_rdnn: String,
  /// Namespace of the replaced definitions.
  #[serde(rename = "namespace")]
  model_namespace: String,
  /// Name of replaced definitions.
  #[serde(rename = "name")]
  model_name: String,
}

/// Parameters for replacing DMN™ model definitions in workspace.
#[derive(Deserialize)]
struct ReplaceDefinitionsParams {
  /// Content of the DMN™ model, encoded in `Base64`.
  #[serde(rename = "content")]
  content: Option<String>,
}

/// Parameters for removing DMN™ model definitions from workspace.
#[derive(Deserialize)]
struct RemoveDefinitionsParams {
  /// Namespace of the definitions to be removed.
  #[serde(rename = "namespace")]
  namespace: Option<String>,
  /// Name of the definitions to be removed.
  #[serde(rename = "name")]
  name: Option<String>,
}

/// Operation status sent back to caller after request completion.
#[derive(Serialize)]
struct StatusResult {
  /// Operation status.
  #[serde(rename = "status")]
  status: String,
}

/// Parameters for evaluating invocable in DMN™ model definitions.
/// The format of input data is compatible with test cases
/// defined in [Technology Compatibility Kit for DMN standard](https://github.com/dmn-tck/tck).
#[derive(Deserialize)]
struct TckEvaluateParams {
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

/// Parameters for evaluating invocable in DMN™ model definitions.
#[derive(Deserialize)]
struct EvaluateParams {
  /// Name of the namespace where the model will be searches.
  #[serde(rename = "namespace")]
  model_namespace: String,
  /// Name of the model in which the invocable will be searched.
  #[serde(rename = "model")]
  model_name: String,
  /// Name of the invocable to be evaluated.
  #[serde(rename = "invocable")]
  invocable_name: String,
}

/// Handler for retrieving system information.
#[get("/system/info")]
async fn get_system_info() -> io::Result<Json<ResultDto<SystemInfoDto>>> {
  Ok(Json(ResultDto::data(SystemInfoDto::default())))
}

/// Handler for deleting all model definitions from workspace.
#[post("/definitions/clear")]
async fn post_definitions_clear(data: web::Data<ApplicationData>) -> io::Result<Json<ResultDto<StatusResult>>> {
  let mut workspace = data.workspace.write().unwrap();
  Ok(Json(ResultDto::data(do_clear_definitions(&mut workspace))))
}

/// Handler for adding model definitions to workspace.
#[post("/definitions/add")]
async fn post_definitions_add(params: Json<AddDefinitionsParams>, data: web::Data<ApplicationData>) -> io::Result<Json<ResultDto<AddDefinitionsResult>>> {
  let mut workspace = data.workspace.write().unwrap();
  let result = do_add_definitions(&mut workspace, &params.into_inner());
  match result {
    Ok(result) => Ok(Json(ResultDto::data(result))),
    Err(reason) => Ok(Json(ResultDto::error(reason))),
  }
}

/// Handler for replacing model definitions in workspace.
#[post("/definitions/replace")]
async fn post_definitions_replace(params: Json<ReplaceDefinitionsParams>, data: web::Data<ApplicationData>) -> io::Result<Json<ResultDto<ReplaceDefinitionsResult>>> {
  let mut workspace = data.workspace.write().unwrap();
  let result = do_replace_definitions(&mut workspace, &params.into_inner());
  match result {
    Ok(result) => Ok(Json(ResultDto::data(result))),
    Err(reason) => Ok(Json(ResultDto::error(reason))),
  }
}

/// Handler for removing model definitions from workspace.
#[post("/definitions/remove")]
async fn post_definitions_remove(params: Json<RemoveDefinitionsParams>, data: web::Data<ApplicationData>) -> io::Result<Json<ResultDto<StatusResult>>> {
  let mut workspace = data.workspace.write().unwrap();
  let result = do_remove_definitions(&mut workspace, &params.into_inner());
  match result {
    Ok(result) => Ok(Json(ResultDto::data(result))),
    Err(reason) => Ok(Json(ResultDto::error(reason))),
  }
}

/// Handler for deploying model definitions stashed in workspace.
#[post("/definitions/deploy")]
async fn post_definitions_deploy(data: web::Data<ApplicationData>) -> io::Result<Json<ResultDto<Vec<EvaluatorStatus>>>> {
  let mut workspace = data.workspace.write().unwrap();
  let result = do_deploy_definitions(&mut workspace);
  match result {
    Ok(result) => Ok(Json(ResultDto::data(result))),
    Err(reason) => Ok(Json(ResultDto::error(reason))),
  }
}

/// Handler for evaluating models with input values in format compatible with test cases
/// defined in [Technology Compatibility Kit for DMN standard](https://github.com/dmn-tck/tck).
#[post("/tck/evaluate")]
async fn post_tck_evaluate(params: Json<TckEvaluateParams>, data: web::Data<ApplicationData>) -> io::Result<Json<ResultDto<OutputNodeDto>>> {
  let workspace = data.workspace.read().unwrap();
  match do_evaluate_tck(&workspace, &params.into_inner()) {
    Ok(response) => Ok(Json(ResultDto::data(response))),
    Err(reason) => Ok(Json(ResultDto::error(reason))),
  }
}

/// Handler for evaluating invocable in model.
///
/// Input values may be defined in `JSON` or `FEEL` context format.
/// Result is always in JSON format.
#[post("/evaluate/{namespace}/{model}/{invocable}")]
async fn post_evaluate(params: web::Path<EvaluateParams>, request_body: String, data: web::Data<ApplicationData>) -> HttpResponse {
  let workspace = data.workspace.read().unwrap();
  let result = do_evaluate(&workspace, &params.into_inner(), &request_body);
  match result {
    Ok(value) => HttpResponse::Ok().content_type("application/json").body(format!("{{\"data\":{}}}", value.jsonify())),
    Err(reason) => HttpResponse::Ok().content_type("application/json").body(ResultDto::<String>::error(reason).to_string()),
  }
}

/// Handler for 404 errors.
async fn not_found() -> io::Result<Json<ResultDto<()>>> {
  Ok(Json(ResultDto::error(err_endpoint_not_found())))
}

/// Starts the server.
pub async fn start_server(opt_host: Option<String>, opt_port: Option<String>, opt_dir: Option<String>) -> io::Result<()> {
  let workspace = Workspace::new(get_workspace_dir(opt_dir));
  let application_data = web::Data::new(ApplicationData {
    workspace: RwLock::new(workspace),
  });
  let address = get_server_address(opt_host, opt_port);
  println!("dmntk {address}");
  HttpServer::new(move || {
    App::new()
      .app_data(application_data.clone())
      .app_data(web::JsonConfig::default().limit(4 * 1024 * 1024).error_handler(|err, _| {
        error::InternalError::from_response(
          "",
          HttpResponse::BadRequest()
            .content_type("application/json")
            .body(ResultDto::<String>::error(err_internal_error(&format!("{err:?}"))).to_string()),
        )
        .into()
      }))
      .service(get_system_info)
      .service(post_definitions_clear)
      .service(post_definitions_add)
      .service(post_definitions_replace)
      .service(post_definitions_remove)
      .service(post_definitions_deploy)
      .service(post_tck_evaluate)
      .service(post_evaluate)
      .default_service(web::route().to(not_found))
  })
  .bind(address)?
  .run()
  .await
}

/// Returns the host address and the port number, the server will start to listen on.
///
/// The default host and port are defined by `DMNTK_DEFAULT_HOST` and `DMNTK_DEFAULT_PORT` constants.
/// When other values are given as parameters to this function, these will be the actual host and port.
/// Host and port may be also controlled using environment variables:
/// - `HOST` or `DMNTK_HOST` for the host name,
/// - `PORT` or `DMNTK_PORT` for the port name.
///
/// Priorities (from highest to lowest):
/// - `opt_host` an `opt_port` parameters,
/// - `DMNTK_HOST` and `DMNTK_PORT` environment variables
/// - `HOST` and `PORT` environment variables
/// - `DMNTK_DEFAULT_HOST` and `DMNTK_DEFAULT_PORT` constants.
///
fn get_server_address(opt_host: Option<String>, opt_port: Option<String>) -> String {
  // resolve IP address
  let mut host = DMNTK_DEFAULT_HOST.to_string();
  if let Ok(host_ip_address) = env::var(DMNTK_HOST_VARIABLE) {
    if is_valid_ip_address(&host_ip_address) {
      host = host_ip_address;
    } else {
      eprintln!("invalid host address specified in environment variable {}: {}", DMNTK_HOST_VARIABLE, host_ip_address);
    }
  }
  if let Some(host_ip_address) = opt_host {
    if is_valid_ip_address(&host_ip_address) {
      host = host_ip_address;
    } else {
      eprintln!("invalid host address given as command option: {}", host_ip_address);
    }
  }
  // resolve IP port
  let mut port: u16 = DMNTK_DEFAULT_PORT;
  if let Ok(p_str) = env::var(DMNTK_PORT_VARIABLE) {
    if let Ok(p) = u16::from_str(&p_str) {
      port = p;
    } else {
      eprintln!("invalid port number specified in environment variable {}: {}", DMNTK_PORT_VARIABLE, p_str);
    }
  }
  if let Some(p_str) = opt_port {
    if let Ok(p) = u16::from_str(&p_str) {
      port = p;
    } else {
      eprintln!("invalid port number specified as command option: {}", p_str);
    }
  }
  format!("{host}:{port}")
}

/// Checks if the specified IP address is correct.
///
/// This function may provide more detailed checks
/// when the [Ipv4Addr](std::net::Ipv4Addr)
/// and [Ipv6Addr](std::net::Ipv6Addr) stabilize.
fn is_valid_ip_address(ip: &str) -> bool {
  ip == "localhost" || ip.parse::<IpAddr>().is_ok()
}

/// Returns the root directory for workspace.
fn get_workspace_dir(opt_dir: Option<String>) -> Option<PathBuf> {
  let mut dir: Option<String> = None;
  if let Ok(d) = env::var(DMNTK_DIR_VARIABLE) {
    let dir_path = Path::new(&d);
    if dir_path.exists() && dir_path.is_dir() {
      dir = Some(d);
    } else {
      eprintln!("invalid directory specified in environment variable {}: {}", DMNTK_DIR_VARIABLE, d);
    }
  }
  if let Some(d) = opt_dir {
    let dir_path = Path::new(&d);
    if dir_path.exists() && dir_path.is_dir() {
      dir = Some(d);
    } else {
      eprintln!("invalid directory specified as command option: {}", d);
    }
  }
  dir.map(|d| PathBuf::from(&d))
}

/// Deletes all model definitions stored in specified workspace.
fn do_clear_definitions(workspace: &mut Workspace) -> StatusResult {
  workspace.clear();
  StatusResult {
    status: "definitions cleared".to_string(),
  }
}

/// Add model definition to workspace.
fn do_add_definitions(workspace: &mut Workspace, params: &AddDefinitionsParams) -> Result<AddDefinitionsResult> {
  if let Some(content) = &params.content {
    if let Ok(bytes) = STANDARD.decode(content) {
      if let Ok(xml) = String::from_utf8(bytes) {
        match dmntk_model::parse(&xml) {
          Ok(definitions) => {
            let (rdnn, namespace, name) = workspace.add(definitions)?;
            Ok(AddDefinitionsResult {
              model_rdnn: rdnn,
              model_namespace: namespace,
              model_name: name,
            })
          }
          Err(reason) => Err(reason),
        }
      } else {
        Err(err_invalid_utf8_content())
      }
    } else {
      Err(err_invalid_base64_encoding())
    }
  } else {
    Err(err_missing_parameter("content"))
  }
}

/// Replace model definition in workspace.
fn do_replace_definitions(workspace: &mut Workspace, params: &ReplaceDefinitionsParams) -> Result<ReplaceDefinitionsResult> {
  if let Some(content) = &params.content {
    if let Ok(bytes) = STANDARD.decode(content) {
      if let Ok(xml) = String::from_utf8(bytes) {
        match dmntk_model::parse(&xml) {
          Ok(definitions) => {
            let (rdnn, namespace, name) = workspace.replace(definitions)?;
            Ok(ReplaceDefinitionsResult {
              model_rdnn: rdnn,
              model_namespace: namespace,
              model_name: name,
            })
          }
          Err(reason) => Err(reason),
        }
      } else {
        Err(err_invalid_utf8_content())
      }
    } else {
      Err(err_invalid_base64_encoding())
    }
  } else {
    Err(err_missing_parameter("content"))
  }
}

/// Removes model definition from workspace.
fn do_remove_definitions(workspace: &mut Workspace, params: &RemoveDefinitionsParams) -> Result<StatusResult> {
  if let Some(namespace) = &params.namespace {
    if let Some(name) = &params.name {
      workspace.remove(namespace, name);
      Ok(StatusResult {
        status: "definitions removed".to_string(),
      })
    } else {
      Err(err_missing_parameter("name"))
    }
  } else {
    Err(err_missing_parameter("namespace"))
  }
}

/// Deploy all definitions in workspace.
fn do_deploy_definitions(workspace: &mut Workspace) -> Result<Vec<EvaluatorStatus>> {
  Ok(workspace.deploy())
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
          workspace.evaluate_invocable(model_namespace, model_name, invocable_name, &input_data)?.try_into()
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

/// Evaluates the artifact specified in parameters and returns the result.
fn do_evaluate(workspace: &Workspace, params: &EvaluateParams, input: &str) -> Result<Value, DmntkError> {
  let model_namespace = &params.model_namespace;
  let model_name = &params.model_name;
  let invocable_name = &params.invocable_name;
  let input_data = dmntk_evaluator::evaluate_context(&FeelScope::default(), input)?;
  let value = workspace.evaluate_invocable(model_namespace, model_name, invocable_name, &input_data)?;
  Ok(value)
}

/*
 All tests below are only for maximizing dead code coverage.
 Remove all these tests when better coverage method is used.
*/
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_deserialize_add_definitions_params() {
    assert!(serde_json::from_str::<Vec<AddDefinitionsParams>>(r#"{"content": "a"}"#).is_err());
    assert!(serde_json::from_str::<AddDefinitionsParams>(r#"{"content":"a"}"#).is_ok());
    assert!(serde_json::from_str::<AddDefinitionsParams>(r#"{ {"inner": 10} }"#).is_err());
    assert!(serde_json::from_str::<AddDefinitionsParams>(r#"[1]"#).is_err());
    assert!(serde_json::from_str::<AddDefinitionsParams>(r#"1"#).is_err());
  }

  #[test]
  fn test_deserialize_replace_definitions_params() {
    assert!(serde_json::from_str::<Vec<ReplaceDefinitionsParams>>(r#"{"content": "a"}"#).is_err());
    assert!(serde_json::from_str::<ReplaceDefinitionsParams>(r#"{"content":"a"}"#).is_ok());
    assert!(serde_json::from_str::<ReplaceDefinitionsParams>(r#"{ {"inner": 10} }"#).is_err());
    assert!(serde_json::from_str::<ReplaceDefinitionsParams>(r#"[1]"#).is_err());
    assert!(serde_json::from_str::<ReplaceDefinitionsParams>(r#"1"#).is_err());
  }

  #[test]
  fn test_deserialize_remove_definitions_params() {
    assert!(serde_json::from_str::<Vec<RemoveDefinitionsParams>>(r#"{"content": "a"}"#).is_err());
    assert!(serde_json::from_str::<RemoveDefinitionsParams>(r#"{"content":"a"}"#).is_ok());
    assert!(serde_json::from_str::<RemoveDefinitionsParams>(r#"{ {"inner": 10} }"#).is_err());
    assert!(serde_json::from_str::<RemoveDefinitionsParams>(r#"[1]"#).is_err());
    assert!(serde_json::from_str::<RemoveDefinitionsParams>(r#"1"#).is_err());
  }

  #[test]
  fn test_deserialize_tck_evaluate_params() {
    assert!(serde_json::from_str::<Vec<TckEvaluateParams>>(r#"{"namespace": "a"}"#).is_err());
    assert!(serde_json::from_str::<TckEvaluateParams>(r#"{"namespace":"a"}"#).is_ok());
    assert!(serde_json::from_str::<TckEvaluateParams>(r#"{ {"inner": 10} }"#).is_err());
    assert!(serde_json::from_str::<TckEvaluateParams>(r#"[1]"#).is_err());
    assert!(serde_json::from_str::<TckEvaluateParams>(r#"1"#).is_err());
  }

  #[test]
  fn test_deserialize_evaluate_params() {
    assert!(serde_json::from_str::<Vec<EvaluateParams>>(r#"{"namespace": "a"}"#).is_err());
    assert!(serde_json::from_str::<EvaluateParams>(r#"{"namespace":"a","model":"b","invocable":"c"}"#).is_ok());
    assert!(serde_json::from_str::<EvaluateParams>(r#"{ {"inner": 10} }"#).is_err());
    assert!(serde_json::from_str::<EvaluateParams>(r#"[1]"#).is_err());
    assert!(serde_json::from_str::<EvaluateParams>(r#"1"#).is_err());
  }
}
