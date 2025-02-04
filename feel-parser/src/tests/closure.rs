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

use crate::{parse_context, parse_textual_expression, AstNode, ClosureBuilder};
use dmntk_feel::{scope, FeelScope};

#[test]
fn _0001() {
  let scope = FeelScope::default();
  let node = parse_context(&scope, r#" { x: 10, a: function(a) function(b) a * b + x } "#, false).unwrap();
  assert_eq!("[x]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0002() {
  let scope = FeelScope::default();
  let node = parse_context(&scope, r#" { x: 10, a: function(a) function(b) a * b + x } "#, false).unwrap();
  assert_eq!(r#"Closure({QualifiedName([Name("x")])})"#, format!("{:?}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0003() {
  let node = parse_textual_expression(&scope!(), r#" 10 between 2 and 34 "#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0004() {
  let node = parse_textual_expression(&scope!(), r#" 1 + 2 "#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0005() {
  let node = parse_textual_expression(&scope!(), r#" 1 < 2"#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0006() {
  let node = parse_textual_expression(&scope!(), r#" 1 <= 2"#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0007() {
  let node = parse_textual_expression(&scope!(), r#" 1 > 2"#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0008() {
  let node = parse_textual_expression(&scope!(), r#" 1 >= 2"#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0009() {
  let node = parse_textual_expression(&scope!(), r#" @"2022-02-08" "#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0010() {
  let node = parse_textual_expression(&scope!(), r#" @"2022-02-08T10:11:12" "#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0011() {
  let node = parse_textual_expression(&scope!(), r#" @"P1D" "#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0012() {
  let node = parse_textual_expression(&scope!(), r#" @"P1Y" "#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0013() {
  let node = parse_textual_expression(&scope!(), r#" true and false "#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0014() {
  let node = parse_textual_expression(&scope!(), r#" [1..10] "#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0015() {
  let node = AstNode::CommaList(vec![AstNode::Numeric("1".into(), "2".into())]);
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0016() {
  let node = AstNode::NegatedList(vec![AstNode::Numeric("1".into(), "2".into())]);
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0017() {
  let node = parse_textual_expression(&scope!(), r#" "a" instance of function<string,string>->string "#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0018() {
  let node = parse_textual_expression(&scope!(), r#" ["a","b","c"] instance of list<string> "#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0019() {
  let node = parse_textual_expression(&scope!(), r#" [1..10] instance of range<number> "#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0020() {
  let node = AstNode::Irrelevant;
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0021() {
  let node = AstNode::Out(Box::new(AstNode::Numeric("1".into(), "2".into())), Box::new(AstNode::Numeric("1".into(), "2".into())));
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0022() {
  let scope = FeelScope::default();
  let node = parse_context(&scope, r#" { a: { b: 1, c: "heute" } } "#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}
