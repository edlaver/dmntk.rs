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
fn test_0001() {
  let scope = &te_scope("{}");
  te_number(
    false,
    scope,
    r#"  1  // eol comment
         + 1"#,
    2,
    0,
  );
}

#[test]
fn test_0002() {
  let scope = &te_scope("{}");
  te_number(
    false,
    scope,
    r#" 1
          /*
          some intro waffle
          */
          + 1"#,
    2,
    0,
  );
}

#[test]
fn test_0003() {
  let scope = &te_scope("{}");
  te_number(false, scope, r#"1 + /* 1 + */ 1"#, 2, 0);
}

#[test]
fn test_0004() {
  let scope = &te_scope("{}");
  te_number(
    false,
    scope,
    r#" 1
          /*
          some intro waffle
          */
          + 1 // and stuff
          + 2"#,
    4,
    0,
  );
}
