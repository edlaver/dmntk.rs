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

use super::*;

#[test]
fn _0001() {
  let scope = te_scope(r#"{ cos: function(x) external { java:{class:"java.lang.Math",method signature:"cos(double)"}} }"#);
  te_number(false, &scope, "cos(123)", -88796890, 8);
}

#[test]
fn _0002() {
  let scope = te_scope(r#"{ cos: function(x) external { java:{class:"java.lang.Math",method signature:"cos(double)"}} }"#);
  te_null(false, &scope, "cos(1,2)", r#"invalid number of arguments"#);
}

#[test]
fn _0003() {
  let scope = te_scope(r#"{ cos: function(arg: number) external { java:{class:"java.lang.Math",method signature:"cos(double)"}} }"#);
  te_number(false, &scope, "cos(arg: 123)", -88796890, 8);
}

#[test]
fn _0004() {
  let scope = te_scope(r#"{ cos: function(arg: number) external { java:{class:"java.lang.Math",method signature:"cos(double)"}} }"#);
  te_null(false, &scope, "cos(x: 123)", "parameter with name arg not found in arguments");
}

#[test]
fn _0005() {
  let scope = te_scope(r#"{ cos: function(arg: number) external { java:{class:"java.lang.Math",method signature:"cos(double)"}} }"#);
  te_null(false, &scope, "cos(x: 1, y: 2)", r#"invalid number of arguments"#);
}

#[test]
fn _0006() {
  let scope = te_scope(r#"{ mathFoo: function(x) external {java:{class:"java.lang.Math",method signature:"foo(double)"}} }"#);
  te_null(false, &scope, "mathFoo(123)", "");
}

#[test]
fn _0007() {
  let scope = te_scope(r#"{ Weather on Ῥόδος: function() external { pmml:{document: "https://en.wiktionary.org/wiki/%E1%BF%AC%CF%8C%CE%B4%CE%BF%CF%82", model: "weather"}} }"#);
  te_string(
    false,
    &scope,
    "Weather on Ῥόδος()",
    r#"PMML, document = https://en.wiktionary.org/wiki/%E1%BF%AC%CF%8C%CE%B4%CE%BF%CF%82, model name = weather"#,
  );
}

#[test]
fn _0008() {
  let scope = te_scope(r#"{ run: function() external { java:{class:"io.dmntk.Runner",method signature:"run()"}} }"#);
  te_string(false, &scope, "run()", "JAVA, class = io.dmntk.Runner, method signature = run()");
}

#[test]
fn _0009() {
  let scope = te_scope(r#"{ Weather on Ῥόδος: function() external { pmml:{document: "", model: "weather"}} }"#);
  te_null(false, &scope, "Weather on Ῥόδος()", r#"PMML document not specified"#);
}

#[test]
fn _0010() {
  let scope = te_scope(r#"{ Weather on Ῥόδος: function() external { pmml:{document: "https://en.wiktionary.org/wiki/%E1%BF%AC%CF%8C%CE%B4%CE%BF%CF%82", model: ""}} }"#);
  te_null(false, &scope, "Weather on Ῥόδος()", r#"PMML model name not specified"#);
}

#[test]
fn _0011() {
  let scope = te_scope(r#"{ fair value: function() external { rust:{ fun: ""} } }"#);
  te_null(
    false,
    &scope,
    "fair value()",
    r#"expected built-in function name or function definition, actual is null(invalid body in function definition)"#,
  );
}

#[test]
fn _0012() {
  let scope = te_scope(r#"{ valueOf: function(c) external {java:{class:"java.lang.String",method signature:"valueOf(char)"}} }"#);
  te_null(false, &scope, "valueOf(1)", "");
}

#[test]
fn _0013() {
  let scope = te_scope(r#"{ valueOf: function(n) external {java:{class:"java.lang.Integer",method signature:"valueOf(java.lang.String)"}} }"#);
  te_null(false, &scope, "valueOf(1)", "");
}

#[test]
fn _0014() {
  let scope = te_scope(r#"{ valueOf: function(n) external {java:{class:"java.lang.Integer",method signature:"valueOf(java.lang.String)"}} }"#);
  te_null(false, &scope, r#"valueOf("394857340958730495873204598374503")"#, "");
}

#[test]
fn _0015() {
  let scope = te_scope(r#"{ valueOf: function(n) external {java:{class:"java.lang.Float",method signature:"valueOf(java.lang.String)"}} }"#);
  te_null(false, &scope, r#"valueOf(1)"#, "");
}

#[test]
fn _0016() {
  let scope = te_scope(r#"{ valueOf: function(n) external {java:{class:"java.lang.Float",method signature:"valueOf(java.lang.String)"}} }"#);
  te_null(false, &scope, r#"valueOf("99,9")"#, "");
}
