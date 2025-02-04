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
use crate::lalr::TokenType::StartExpression;

#[test]
fn _0001() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#"date and time(date("2017-01-01"),time("23:59:01"))"#,
    r#"
       FunctionInvocation
       ├─ Name
       │  └─ `date and time`
       └─ PositionalParameters
          ├─ FunctionInvocation
          │  ├─ Name
          │  │  └─ `date`
          │  └─ PositionalParameters
          │     └─ String
          │        └─ `2017-01-01`
          └─ FunctionInvocation
             ├─ Name
             │  └─ `time`
             └─ PositionalParameters
                └─ String
                   └─ `23:59:01`
    "#,
    false,
  );
}

#[test]
fn _0002() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#"date and time(d:date("2017-01-01"),t:time("23:59:01"))"#,
    r#"
       FunctionInvocation
       ├─ Name
       │  └─ `date and time`
       └─ NamedParameters
          ├─ NamedParameter
          │  ├─ ParameterName
          │  │  └─ `d`
          │  └─ FunctionInvocation
          │     ├─ Name
          │     │  └─ `date`
          │     └─ PositionalParameters
          │        └─ String
          │           └─ `2017-01-01`
          └─ NamedParameter
             ├─ ParameterName
             │  └─ `t`
             └─ FunctionInvocation
                ├─ Name
                │  └─ `time`
                └─ PositionalParameters
                   └─ String
                      └─ `23:59:01`
    "#,
    false,
  );
}

#[test]
fn _0003() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#"date and time(date:date("2017-01-01"),time:time("23:59:01"))"#,
    r#"
       FunctionInvocation
       ├─ Name
       │  └─ `date and time`
       └─ NamedParameters
          ├─ NamedParameter
          │  ├─ ParameterName
          │  │  └─ `date`
          │  └─ FunctionInvocation
          │     ├─ Name
          │     │  └─ `date`
          │     └─ PositionalParameters
          │        └─ String
          │           └─ `2017-01-01`
          └─ NamedParameter
             ├─ ParameterName
             │  └─ `time`
             └─ FunctionInvocation
                ├─ Name
                │  └─ `time`
                └─ PositionalParameters
                   └─ String
                      └─ `23:59:01`
    "#,
    false,
  );
}

#[test]
fn _0004() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#"date and time("2018-12-10T10:30:00.0001+05:00:01")"#,
    r#"
       FunctionInvocation
       ├─ Name
       │  └─ `date and time`
       └─ PositionalParameters
          └─ String
             └─ `2018-12-10T10:30:00.0001+05:00:01`
    "#,
    false,
  );
}
