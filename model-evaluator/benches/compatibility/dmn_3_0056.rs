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

use super::*;

from_examples!(DMN_3_0056);

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision001";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"2"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision001_a";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"-2"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0003(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision002";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"2"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0004(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision003";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"0"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0005(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision003_a";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"null([core::modulo] division by zero)"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0006(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision004";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null(expected 2 parameters, actual number of parameters is 0)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0007(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision005";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null(expected 2 parameters, actual number of parameters is 1)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0008(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision005_a";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null(expected 2 parameters, actual number of parameters is 3)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0009(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision006";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"2"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0010(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision007";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"null(parameter 'divisor' not found)"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0011(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision008";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::modulo] invalid argument type, expected number, actual type is Null)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0012(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision008_a";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::modulo] invalid argument type, expected number, actual type is Null)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0013(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision008_b";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::modulo] invalid argument type, expected number, actual type is Null)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0014(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision009";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::modulo] invalid argument type, expected number, actual type is string)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0015(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision010";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::modulo] invalid argument type, expected number, actual type is boolean)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0016(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision011";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::modulo] invalid argument type, expected number, actual type is days and time duration)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0017(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision012";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::modulo] invalid argument type, expected number, actual type is years and months duration)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0018(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision013";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::modulo] invalid argument type, expected number, actual type is date)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0019(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision014";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::modulo] invalid argument type, expected number, actual type is time)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0020(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision015";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::modulo] invalid argument type, expected number, actual type is date and time)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0021(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision016a";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"2"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0022(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision016b";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"3"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0023(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision016c";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"-3"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0024(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision016d";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"-2"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0025(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision017a";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"1.1"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0026(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision017b";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"3.4"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0027(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision017c";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"-3.4"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0028(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision017d";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"-1.1"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}
