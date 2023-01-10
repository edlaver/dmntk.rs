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
use crate::context::{ParsingContext, ParsingContextEntry};
use crate::lalr::TokenType::StartTextualExpression;

#[test]
fn _0001() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    "[1..10]",
    r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ Numeric
       │     └─ `1.`
       └─ IntervalEnd (closed)
          └─ Numeric
             └─ `10.`
    "#,
    false,
  );
}

#[test]
fn _0002() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    "(1..10]",
    r#"
       Range
       ├─ IntervalStart (opened)
       │  └─ Numeric
       │     └─ `1.`
       └─ IntervalEnd (closed)
          └─ Numeric
             └─ `10.`
    "#,
    false,
  );
}

#[test]
fn _0003() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    "[1..10)",
    r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ Numeric
       │     └─ `1.`
       └─ IntervalEnd (opened)
          └─ Numeric
             └─ `10.`
    "#,
    false,
  );
}

#[test]
fn _0004() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    "(1..10)",
    r#"
       Range
       ├─ IntervalStart (opened)
       │  └─ Numeric
       │     └─ `1.`
       └─ IntervalEnd (opened)
          └─ Numeric
             └─ `10.`
    "#,
    false,
  );
}

#[test]
fn _0005() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    "]1..10]",
    r#"
       Range
       ├─ IntervalStart (opened)
       │  └─ Numeric
       │     └─ `1.`
       └─ IntervalEnd (closed)
          └─ Numeric
             └─ `10.`
    "#,
    false,
  );
}

#[test]
fn _0006() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    "[1..10[",
    r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ Numeric
       │     └─ `1.`
       └─ IntervalEnd (opened)
          └─ Numeric
             └─ `10.`
    "#,
    false,
  );
}

#[test]
fn _0007() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    "]1..10[",
    r#"
       Range
       ├─ IntervalStart (opened)
       │  └─ Numeric
       │     └─ `1.`
       └─ IntervalEnd (opened)
          └─ Numeric
             └─ `10.`
    "#,
    false,
  );
}

#[test]
fn _0008() {
  let scope = scope!();
  scope.set_entry("a".into());
  scope.set_entry("b".into());
  accept(
    &scope,
    StartTextualExpression,
    "[a..b]",
    r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ QualifiedName
       │     └─ Name
       │        └─ `a`
       └─ IntervalEnd (closed)
          └─ QualifiedName
             └─ Name
                └─ `b`
    "#,
    false,
  );
}

#[test]
fn _0009() {
  let scope = scope!();
  let mut ctx = ParsingContext::default();
  ctx.set_entry("start".into());
  ctx.set_entry("end".into());
  scope.set_value("r".into(), ParsingContextEntry::Context(ctx));
  accept(
    &scope,
    StartTextualExpression,
    "[r.start..r.end]",
    r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ QualifiedName
       │     ├─ Name
       │     │  └─ `r`
       │     └─ Name
       │        └─ `start`
       └─ IntervalEnd (closed)
          └─ QualifiedName
             ├─ Name
             │  └─ `r`
             └─ Name
                └─ `end`
    "#,
    false,
  );
}

#[test]
fn _00010() {
  let scope = scope!();
  scope.set_entry("r".into());
  scope.set_entry("start".into());
  scope.set_entry("end".into());
  accept(
    &scope,
    StartTextualExpression,
    "[r.start..r.end]",
    r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ QualifiedName
       │     ├─ Name
       │     │  └─ `r`
       │     └─ Name
       │        └─ `start`
       └─ IntervalEnd (closed)
          └─ QualifiedName
             ├─ Name
             │  └─ `r`
             └─ Name
                └─ `end`
    "#,
    false,
  );
}

#[test]
fn _00011() {
  let scope = scope!();
  scope.set_entry("r".into());
  scope.set_entry("s".into());
  scope.set_entry("start".into());
  scope.set_entry("end".into());
  accept(
    &scope,
    StartTextualExpression,
    "[r.start..r.s.end]",
    r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ QualifiedName
       │     ├─ Name
       │     │  └─ `r`
       │     └─ Name
       │        └─ `start`
       └─ IntervalEnd (closed)
          └─ QualifiedName
             ├─ Name
             │  └─ `r`
             ├─ Name
             │  └─ `s`
             └─ Name
                └─ `end`
    "#,
    false,
  );
}
