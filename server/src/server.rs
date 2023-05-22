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

use crate::data::ApplicationData;
use crate::errors::*;
use actix_web::web::Json;
use actix_web::{post, web, App, HttpResponse, HttpServer};
use dmntk_common::{DmntkError, Jsonify, Result};
use dmntk_feel::values::Value;
use dmntk_feel::FeelScope;
use dmntk_workspace::Workspace;
use serde::Serialize;
use std::net::IpAddr;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::RwLock;
use std::{env, fmt, io};

const DMNTK_DEFAULT_PORT: u16 = 22022;
const DMNTK_DEFAULT_HOST: &str = "0.0.0.0";
const DMNTK_HOST_VARIABLE: &str = "DMNTK_HOST";
const DMNTK_PORT_VARIABLE: &str = "DMNTK_PORT";
const DMNTK_DIR_VARIABLE: &str = "DMNTK_DIR";

const CONTENT_TYPE: &str = "application/json";

/// Data transfer object for an error.
#[derive(Serialize)]
pub struct ErrorDto {
  /// Error details.
  #[serde(rename = "detail")]
  detail: String,
}

/// Data transfer object for a result.
#[derive(Serialize)]
pub struct ResultDto<T> {
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
  /// Utility function for creating [ResultDto] with a single error inside.
  pub fn error(err: impl fmt::Display) -> ResultDto<T> {
    ResultDto {
      errors: vec![ErrorDto { detail: format!("{err}") }],
      ..Default::default()
    }
  }
}

/// Handler for evaluating invocable in model.
///
/// Input data may be provided in `JSON` format or `FEEL` context format.
/// The result value is always converted to JSON format.
#[post("/evaluate/{namespace}/{model}/{invocable}")]
async fn post_evaluate(params: web::Path<(String, String, String)>, request_body: String, data: web::Data<ApplicationData>) -> HttpResponse {
  let workspace = data.workspace.read().unwrap();
  let (namespace, model, invocable) = params.into_inner();
  let result = do_evaluate(&workspace, &namespace, &model, &invocable, &request_body);
  match result {
    Ok(value) => HttpResponse::Ok().content_type(CONTENT_TYPE).body(format!("{{\"data\":{}}}", value.jsonify())),
    Err(reason) => HttpResponse::Ok().content_type(CONTENT_TYPE).body(ResultDto::<String>::error(reason).to_string()),
  }
}

/// Handler for 404 errors.
async fn not_found() -> io::Result<Json<ResultDto<()>>> {
  Ok(Json(ResultDto::error(err_endpoint_not_found())))
}

#[cfg(feature = "tck")]
fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(crate::tck::post_tck_evaluate);
}

#[cfg(not(feature = "tck"))]
fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(post_evaluate);
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
      .app_data(web::PayloadConfig::new(4 * 1024 * 1024))
      .configure(config)
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

/// Evaluates the artifact specified in parameters and returns the result.
fn do_evaluate(workspace: &Workspace, model_rdnn: &str, model_name: &str, invocable_name: &str, input: &str) -> Result<Value, DmntkError> {
  let input_data = dmntk_evaluator::evaluate_context(&FeelScope::default(), input)?;
  let value = workspace.evaluate_invocable(model_rdnn, model_name, invocable_name, &input_data)?;
  Ok(value)
}
