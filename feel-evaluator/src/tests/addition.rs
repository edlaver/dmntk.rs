/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2018-2022 Dariusz Depta Engos Software
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
 * Copyright (c) 2018-2022 Dariusz Depta Engos Software
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
fn test_0001() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "1+1", 2, 0);
}

#[test]
fn test_0002() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, " 1 + 2 ", 3, 0);
}

#[test]
fn test_0003() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, " 5 +2 +1 ", 8, 0);
}

#[test]
fn test_0004() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "20+200+2", 222, 0);
}

#[test]
fn test_0005() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "( 1 + 2 ) + ( 3 + 4 )", 10, 0);
}

#[test]
fn test_0006() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "( ( ( 1 + 2 ) ) )", 3, 0);
}

#[test]
fn test_0007() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "(1+2)*(3+2)", 15, 0);
}

#[test]
fn test_0008() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "1+2*3+2", 9, 0);
}

#[test]
fn test_0009() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, ".25 + .2", 45, 2);
}

#[test]
fn test_0010() {
  let scope = &te_scope(r#"{Full Name:"John Doe"}"#);
  te_string(false, scope, r#""Hello " + Full Name"#, "Hello John Doe");
}

#[test]
fn test_0011() {
  let scope = &te_scope(r#"{Employment Status:"EMPLOYED"}"#);
  te_string(false, scope, r#""You are " + Employment Status"#, "You are EMPLOYED");
}

#[test]
fn test_0013() {
  let scope = &te_scope(r#"{Full Name:"John Doe", Employment Status:"EMPLOYED"}"#);
  te_string(
    false,
    scope,
    r#""Hello " + Full Name + ", you are " + Employment Status"#,
    "Hello John Doe, you are EMPLOYED",
  );
}

#[test]
fn test_0014() {
  let scope = &te_scope(r#"{}"#);
  te_null(
    false,
    scope,
    r#"1.25 + "alfa""#,
    r#"incompatible types in addition: 1.25(number) + "alfa"(string)"#,
  );
}
