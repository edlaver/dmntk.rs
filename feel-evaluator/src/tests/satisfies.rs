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
use dmntk_feel::scope;

#[test]
fn _0001() {
  satisfies(false, &scope!(), "1", "", r#"[]"#, false);
}

#[test]
fn _0002() {
  satisfies(false, &scope!(), "1", "", r#"[1]"#, true);
}

#[test]
fn _0003() {
  satisfies(false, &scope!(), "10", "", r#"[9,10,11]"#, true);
}

#[test]
fn _0004() {
  satisfies(false, &scope!(), "8", "", r#"[9,10,11]"#, false);
}

#[test]
fn _0005() {
  satisfies(false, &scope!(), "12", "", r#"[9,10,11]"#, false);
}

#[test]
fn _0006() {
  satisfies(false, &scope!(), "10", "", r#"[9],[10],[11]"#, true);
}

#[test]
fn _0007() {
  satisfies(false, &scope!(), "8", "", r#"[9],[10],[11]"#, false);
}

#[test]
fn _0008() {
  satisfies(false, &scope!(), "12", "", r#"[9],[10],[11]"#, false);
}

#[test]
fn _0009() {
  satisfies(false, &scope!(), "3", "", r#"not([2,3,4])"#, false);
}

#[test]
fn _0010() {
  satisfies(false, &scope!(), "1", "", r#"not([2,3,4])"#, true);
}

#[test]
fn _0011() {
  satisfies(false, &scope!(), "3", "", r#"not([2..4])"#, false);
}

#[test]
fn _0012() {
  satisfies(false, &scope!(), "1", "", r#"not([2..4])"#, true);
}

#[test]
fn _0013() {
  satisfies(false, &scope!(), "1", "", r#"not(2,3,4)"#, true);
}

#[test]
fn _0014() {
  satisfies(false, &scope!(), "3", "", r#"not(2,3,4)"#, false);
}

#[test]
fn _0015() {
  satisfies(false, &scope!(), "null", "", r#"not(2,"a",null,3,4)"#, false);
}

#[test]
fn _0016() {
  satisfies(false, &scope!(), "null", "", r#"not(2,"a",3,4)"#, true);
}

#[test]
fn _0017() {
  satisfies(false, &scope!(), "true", "", r#"not(2,"a",null,true,3,4)"#, false);
}

#[test]
fn _0018() {
  satisfies(false, &scope!(), "false", "", r#"not(2,"a",true,3,4)"#, true);
}

#[test]
fn _0019() {
  satisfies(false, &scope!(), "10", "", r#"not(<20)"#, false);
}

#[test]
fn _0020() {
  satisfies(false, &scope!(), "10", "", r#"not(<10)"#, true);
}

#[test]
fn _0021() {
  satisfies(false, &scope!(), "10", "", r#"not(<=20)"#, false);
}

#[test]
fn _0022() {
  satisfies(false, &scope!(), "10", "", r#"not(<=9)"#, true);
}

#[test]
fn _0023() {
  satisfies(false, &scope!(), "10", "", r#"not(>9)"#, false);
}

#[test]
fn _0024() {
  satisfies(false, &scope!(), "10", "", r#"not(>11)"#, true);
}

#[test]
fn _0025() {
  satisfies(false, &scope!(), "10", "", r#"not(>=10)"#, false);
}

#[test]
fn _0026() {
  satisfies(false, &scope!(), "10", "", r#"not(>=11)"#, true);
}

#[test]
fn _0027() {
  satisfies_null(
    false,
    &scope!(),
    "1",
    "",
    r#"not(2,3,function() 1.23)"#,
    r#"unexpected type in negated list: function<>->Any"#,
  );
}
