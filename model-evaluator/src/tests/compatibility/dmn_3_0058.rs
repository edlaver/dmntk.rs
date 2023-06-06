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

use super::super::*;

from_examples!(DMN_3_0058);

#[test]
fn _0001() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "decision001", &ctx, r#"1000000.01"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "decision002", &ctx, r#"1000000.01"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "decision003", &ctx, r#"1000000.01"#);
}

#[test]
fn _0004() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "decision003_a",
    &ctx,
    r#"null([core::number] <FeelNumberError> invalid number literal '1,000,000.01')"#,
  );
}

#[test]
fn _0005() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "decision004",
    &ctx,
    r#"null([core::number] grouping separator must be space, period, comma or null)"#,
  );
}

#[test]
fn _0006() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "decision004_a",
    &ctx,
    r#"null([core::number] grouping separator must be space, period, comma or null)"#,
  );
}

#[test]
fn _0007() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "decision004_b",
    &ctx,
    r#"null([core::number] decimal separator must be period, comma or null)"#,
  );
}

#[test]
fn _0008() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "decision004_c",
    &ctx,
    r#"null([core::number] decimal separator must be period, comma or null)"#,
  );
}

#[test]
fn _0009() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "decision005", &ctx, r#"1000000.01"#);
}

#[test]
fn _0010() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "decision006", &ctx, r#"1000000.01"#);
}

#[test]
fn _0011() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "decision007", &ctx, r#"1000000"#);
}

#[test]
fn _0012() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "decision008", &ctx, r#"1000000.00"#);
}

#[test]
fn _0013() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "decision009",
    &ctx,
    r#"null([core::number] decimal separator must be different from grouping separator)"#,
  );
}

#[test]
fn _0014() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "decision010",
    &ctx,
    r#"null([core::number] decimal separator must be different from grouping separator)"#,
  );
}

#[test]
fn _0015() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "decision011",
    &ctx,
    r#"null([core::number] invalid argument type, expected string, actual type is Null)"#,
  );
}

#[test]
fn _0016() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "decision012",
    &ctx,
    r#"null([core::number] invalid argument type, expected string, actual type is number)"#,
  );
}

#[test]
fn _0017() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "decision013", &ctx, r#"1000000.01"#);
}

#[test]
fn _0018() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "decision014", &ctx, r#"null(parameter 'grouping separator' not found)"#);
}

#[test]
fn _0019() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "decision015",
    &ctx,
    r#"null([core::number] <FeelNumberError> invalid number literal 'foo.bar001')"#,
  );
}

#[test]
fn _0020() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "decision016",
    &ctx,
    r#"null(expected 3 parameters, actual number of parameters is 2)"#,
  );
}

#[test]
fn _0021() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    "decision017",
    &ctx,
    r#"null(expected 3 parameters, actual number of parameters is 4)"#,
  );
}
