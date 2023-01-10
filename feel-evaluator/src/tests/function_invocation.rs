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
  let scope = &te_scope(r#"{add: function (x: number, y: number) x + y }"#);
  te_number(false, scope, r#"add(17.82,32.18)"#, 50, 0);
}

#[test]
fn _0002() {
  let scope = &te_scope(
    r#"{
      a: 10,
      add: function (x: number, y: number) x + y + a
    }"#,
  );
  te_number(false, scope, r#"add(17.82,32.18)"#, 60, 0);
}

#[test]
fn _0003() {
  let scope = &te_scope(r#"{ add: function (x: number, y: number) x + y + a }"#);
  te_null(false, scope, r#"add(17.82,32.18)"#, "context has no value for key 'a'");
}

#[test]
fn _0004() {
  let scope = &te_scope(
    r#"{
      a: {
        b: {
          c: 10
        } 
      },
      add: function (x: number, y: number) x + y + a.b.c
    }"#,
  );
  te_number(false, scope, r#"add(17.82,32.18)"#, 60, 0);
}

#[test]
fn _0005() {
  let scope = &te_scope(
    r#"{
      a: @"2023-01-07",
      add: function (x: number, y: number) x + y + a.year
    }"#,
  );
  te_number(false, scope, r#"add(17.82,32.18)"#, 2073, 0);
}
