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

#[test]
fn _0001() {
  let scope = &te_scope("{}");
  te_bool(false, scope, r#"some n in [1,2,3] satisfies n > 1.5"#, true);
}

#[test]
fn _0002() {
  let scope = &te_scope("{}");
  te_bool(false, scope, r#"some n in [1,2,3] satisfies n > 10.5"#, false);
}

#[test]
fn _0003() {
  let scope = &te_scope("{}");
  te_bool(false, scope, r#"some n in 1 satisfies n > 10.5"#, false);
}

#[test]
fn _0004() {
  let scope = &te_scope("{}");
  te_bool(false, scope, r#"some n in 12.58 satisfies n > 10.5"#, true);
}

#[test]
fn _0005() {
  let scope = FeelScope::default();
  let node = AstNode::Some(Box::new(AstNode::Name("n".into())), Box::new(AstNode::Name("n".into())));
  assert_eq!(
    r#"<FeelEvaluatorError> expected AST node AstNode::QuantifiedContexts, actual AST node is Name(Name("n"))"#,
    crate::evaluate(&scope, &node).err().unwrap().to_string()
  );
}

#[test]
fn _0006() {
  let scope = FeelScope::default();
  let node = AstNode::Some(Box::new(AstNode::QuantifiedContexts(vec![])), Box::new(AstNode::Name("n".into())));
  assert_eq!(
    r#"<FeelEvaluatorError> expected AST node AstNode::Satisfies, actual AST node is Name(Name("n"))"#,
    crate::evaluate(&scope, &node).err().unwrap().to_string()
  );
}
