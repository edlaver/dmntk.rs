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

//! # URI

use self::errors::*;
use crate::DmntkError;
use std::convert::TryFrom;
use uriparse::{URIReference, URI};

/// URI.
#[derive(Debug, Clone)]
pub struct Uri(String);

impl TryFrom<&str> for Uri {
  type Error = DmntkError;
  /// Converts [Uri] from string.
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    if let Ok(uri_reference) = URIReference::try_from(value) {
      if let Ok(uri) = URI::try_from(uri_reference) {
        if uri.has_query() || uri.has_fragment() {
          return Err(err_invalid_uri(value));
        }
        return Ok(Self(uri.to_string()));
      }
    }
    Err(err_invalid_uri(value))
  }
}

impl<'a> From<&'a Uri> for &'a str {
  /// Converts a reference to [Uri] into reference to string.
  fn from(value: &'a Uri) -> Self {
    &value.0
  }
}

impl From<Uri> for String {
  /// Converts [Uri] into string.
  fn from(value: Uri) -> Self {
    value.0
  }
}

mod errors {
  use crate::{DmntkError, ToErrorMessage};

  /// Errors reported by [Uri](crate::uri::Uri).
  #[derive(ToErrorMessage)]
  struct UriError(String);

  /// Creates an error indicating an invalid URI.
  pub fn err_invalid_uri(s: &str) -> DmntkError {
    UriError(format!("invalid reference: '{s}'")).into()
  }
}
