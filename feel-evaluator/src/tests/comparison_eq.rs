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
 * you may not use this file except in compatibility with the License.
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
use dmntk_feel::scope;

#[test]
fn test_0001() {
  te_bool(false, &scope!(), "1=1", true);
}

#[test]
fn test_0002() {
  te_bool(false, &scope!(), "100 = null", false);
}

#[test]
fn test_0003() {
  te_bool(false, &scope!(), "1 = 1.000", true);
}

#[test]
fn test_0004() {
  te_bool(false, &scope!(), "1.276=1.276", true);
}

#[test]
fn test_0005() {
  te_bool(false, &scope!(), ".54635=.54635", true);
}

#[test]
fn test_0006() {
  te_bool(false, &scope!(), "(1+1)=2", true);
}

#[test]
fn test_0007() {
  te_bool(false, &scope!(), "(1+2)=2", false);
}

#[test]
fn test_0008() {
  te_bool(false, &scope!(), " ( 1 + 1 ) = 2", true);
}

#[test]
fn test_0009() {
  te_bool(false, &scope!(), " ( 1 + 3 ) = 2", false);
}

#[test]
fn test_0010() {
  te_bool(false, &scope!(), "(1+1)=(5-3)", true);
}

#[test]
fn test_0011() {
  te_bool(false, &scope!(), "(1*2)=(10/5)", true);
}

#[test]
fn test_0012() {
  te_bool(false, &scope!(), "true=true", true);
}

#[test]
fn test_0013() {
  te_bool(false, &scope!(), "true=false", false);
}

#[test]
fn test_0014() {
  te_bool(false, &scope!(), "false=true", false);
}

#[test]
fn test_0015() {
  te_bool(false, &scope!(), "true=true", true);
}

#[test]
fn test_0016() {
  te_bool(false, &scope!(), "true=null", false);
}

#[test]
fn test_0017() {
  te_bool(false, &scope!(), r#"(10 = 10)"#, true);
}

#[test]
fn test_0018() {
  te_null(
    false,
    &scope!(),
    r#"10 = ({c1: {a: {c: "bar", b: "foo"}}})"#,
    r#"equal err '10' =?= '{c1: {a: {b: "foo", c: "bar"}}}'"#,
  );
}

#[test]
fn test_0019() {
  te_bool(false, &scope!(), r#"{a: {c: "bar", b: "foo"}} = {a: {b: "foo", c: "bar"}}"#, true);
}

#[test]
fn test_0020() {
  te_bool(false, &scope!(), r#"{a: {c: "bar", b: "foo"}} = {a: {b: "foo", c: "bars"}}"#, false);
}

#[test]
fn test_0021() {
  te_bool(false, &scope!(), r#"[1,2,3] = [1,2,3]"#, true);
}

#[test]
fn test_0022() {
  te_bool(false, &scope!(), r#" @"P0D" = @"-P0D" "#, true);
}

#[test]
fn test_0023() {
  te_bool(false, &scope!(), r#" @"2002-04-02T23:00:00-04:00" = @"2002-04-03T02:00:00-01:00" "#, true);
}

#[test]
fn test_0024() {
  te_bool(
    false,
    &scope!(),
    r#" date and time("2018-12-08T00:00:00+00:00") = date and time("2018-12-08T00:00:00@Etc/UTC") "#,
    true,
  );
}
