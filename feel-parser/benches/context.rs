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

#![feature(test)]

extern crate test;

use dmntk_feel::{scope, FeelScope};
use dmntk_feel_parser::parse_context;
use test::Bencher;

#[bench]
fn feel_parser_context_0001(b: &mut Bencher) {
  let scope = scope!();
  let input = r#"{}"#;
  b.iter(|| parse_context(&scope, input, false));
}

#[bench]
fn feel_parser_context_0002(b: &mut Bencher) {
  let scope = scope!();
  let input = " \n { \t } \r ";
  b.iter(|| parse_context(&scope, input, false));
}

#[bench]
fn feel_parser_context_0003(b: &mut Bencher) {
  let scope = scope!();
  let input = r#"{age:49}"#;
  b.iter(|| parse_context(&scope, input, false));
}

#[bench]
fn feel_parser_context_0004(b: &mut Bencher) {
  let scope = scope!();
  let input = r#"{"name":"John","age":27,"car":{"model":"{Porsche}","production year":2021}}"#;
  b.iter(|| parse_context(&scope, input, false));
}
