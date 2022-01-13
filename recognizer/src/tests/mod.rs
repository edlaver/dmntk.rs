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

mod builder;
mod canvas;
mod plane;
mod point;
mod recognizer;
mod rect;

pub const EX_01: &str = r#"
  ┌───┬────────────┬───────╥──────┐
  │ U │  Customer  │ Order ║      │
  ╞═══╪════════════╪═══════╬══════╡
  │ 1 │ "Business" │  <10  ║ 0.10 │
  ├───┼────────────┼───────╫──────┤
  │ 2 │ "Business" │ >=10  ║ 0.15 │
  ├───┼────────────┼───────╫──────┤
  │ 3 │ "Private"  │   -   ║ 0.05 │
  └───┴────────────┴───────╨──────┘
"#;

pub const EX_02: &str = r#"
  ┌───┬───────────┬───────╥──────┐
  │ U │ Customer  │ Order ║      │
  │   ├───────────┼───────╫──────┤
  │   │"Business",│  <10, ║ 0.05,│
  │   │"Private"  │ >=10  ║ 0.10,│
  │   │           │       ║ 0.15 │
  ╞═══╪═══════════╪═══════╬══════╡
  │ 1 │"Business" │  <10  ║ 0.10 │
  ├───┼───────────┼───────╫──────┤
  │ 2 │"Business" │ >=10  ║ 0.15 │
  ├───┼───────────┼───────╫──────┤
  │ 3 │"Private"  │   -   ║ 0.05 │
  └───┴───────────┴───────╨──────┘
"#;

pub const EX_03: &str = r#"
  ┌───┬───────────┬───────╥─────────────────────╥─────────────┬───────────┐
  │ U │           │       ║    Order options    ║             │           │
  │   │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
  │   │   type    │ size  ║ Discount │ Priority ║             │           │
  │   ├───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
  │   │"Business",│  <10, ║   0.10,  │"Normal", ║             │           │
  │   │"Private"  │ >=10  ║   0.15,  │ "High",  ║             │           │
  │   │           │       ║   0.05   │ "Low"    ║             │           │
  ╞═══╪═══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
  │ 1 │"Business" │  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
  ├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
  │ 2 │"Business" │ >=10  ║   0.15   │  "High"  ║ Large order │   Ref 2   │
  ├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
  │ 3 │"Private"  │   -   ║   0.05   │  "Low"   ║ All orders  │   Ref 3   │
  └───┴───────────┴───────╨──────────┴──────────╨─────────────┴───────────┘
"#;

pub const EX_04: &str = r#"
  ┌─────────────────────────────────────┐
  │ Order options                       │
  ├───┬───────────┬───────╥─────────────┴───────╥─────────────┬───────────┐
  │ U │           │       ║    Order options    ║             │           │
  │   │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
  │   │   type    │ size  ║ Discount │ Priority ║             │           │
  │   ├───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
  │   │"Business",│  <10, ║   0.10,  │"Normal", ║             │           │
  │   │"Private"  │ >=10  ║   0.15,  │ "High",  ║             │           │
  │   │           │       ║   0.05   │ "Low"    ║             │           │
  ╞═══╪═══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
  │ 1 │"Business" │  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
  ├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
  │ 2 │"Business" │ >=10  ║   0.15   │ "High"   ║ Large order │   Ref 2   │
  ├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
  │ 3 │"Private"  │   -   ║   0.05   │ "Low"    ║ All orders  │   Ref 3   │
  └───┴───────────┴───────╨──────────┴──────────╨─────────────┴───────────┘
"#;

pub const EX_05: &str = r#"
  ┌─────────────────╥───────────────┬──────────┬───────────────┐
  │ Applicant age   ║     <25       │ [25..60] │      >60      │
  ├─────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │                 ║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  ├─────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │        U        ║  1   │    2   │     3    │   4    │   5  │
  └─────────────────╨──────┴────────┴──────────┴────────┴──────┘
"#;

pub const EX_06: &str = r#"
  ┌─────────────────────────────────╥───────────────┬──────────┬───────────────┐
  │ Applicant age                   ║     <25       │ [25..60] │      >60      │
  ├─────────────────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history                 ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═════════╤═══════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Sell    │ Applicant risk rating ║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  │         ├───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ options │ Special Discount      ║  10  │    5   │     5    │    5   │  0   │
  ╞═════════╧═══════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Additional acceptance           ║ No   │   No   │    No    │   No   │ Yes  │
  ├─────────────────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ Reference                       ║ Rf 0 │  Rf 1  │   Rf 2   │  Rf 3  │ Rf 4 │
  ├─────────────────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ U                               ║  1   │    2   │     3    │   4    │   5  │
  └─────────────────────────────────╨──────┴────────┴──────────┴────────┴──────┘
"#;

pub const EX_07: &str = r#"
  ┌──────────────────────────────────────────────────────────────────────────────────┐
  │ Sell options                                                                     │
  ├─────────────────────────────────┬─────────────────────╥───────────────┬──────────┼───────────────┐
  │ Applicant age                   │ <25,[25..60],>60    ║     <25       │ [25..60] │      >60      │
  ├─────────────────────────────────┼─────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history                 │   "good","bad"      ║"good"│ "bad"  │     -    │ "good" │"bad" │
  ╞═════════╤═══════════════════════╪═════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Sell    │ Applicant risk rating │"Low","Medium","High"║"Low" │"Medium"│ "Medium" │"Medium"│"High"│
  │         ├───────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ options │ Special Discount      │     0, 5, 10        ║  10  │    5   │     5    │    5   │  0   │
  ╞═════════╧═══════════════════════╪═════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Additional acceptance           │                     ║ No   │   No   │    No    │   No   │ Yes  │
  ├─────────────────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ Reference                       │                     ║ Rf 0 │  Rf 1  │   Rf 2   │  Rf 3  │ Rf 4 │
  ├─────────────────────────────────┼─────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ U                               │                     ║  1   │    2   │     3    │   4    │   5  │
  └─────────────────────────────────┴─────────────────────╨──────┴────────┴──────────┴────────┴──────┘
"#;

pub const EX_08: &str = r#"
  ┌───────────────────────────┐
  │ information item name     │
  ├───┬────────────────────┬──┴─────────────────╥────────────────────┐
  │ C │ input expression 1 │ input expression 2 ║    output label    │
  │   ├────────────────────┼────────────────────╫────────────────────┤
  │   │ input value 1a,    │ input value 2a,    ║ output value 1a,   │
  │   │   input value 1b   │   input value 2b   ║   output value 1b  │
  ╞═══╪════════════════════╪════════════════════╬════════════════════╡
  │ 1 │                    │  input entry 2.1   ║  output entry 1.1  │
  ├───┤  input entry 1.1   ├────────────────────╫────────────────────┤
  │ 2 │                    │  input entry 2.2   ║  output entry 1.2  │
  ├───┼────────────────────┼────────────────────╫────────────────────┤
  │ 3 │  input entry 1.2   │         -          ║  output entry 1.3  │
  ├───┼────────────────────┼────────────────────╫────────────────────┤
  │ 4 │  input entry 1.3   │  input entry 2.3   ║  output entry 1.4  │
  └───┴────────────────────┴────────────────────╨────────────────────┘
"#;

pub const EX_09: &str = r#"
  ┌───────────────────────────┐
  │   information item name   │
  ├────────────────────┬──────┴─────────────╥─────────────────────────────────────┬──────────────────┐
  │ input expression 1 │ input value 1a,    ║            input entry 1.1          │ input entry 1.2  │
  │                    │ input value 1b     ║                                     │                  │
  ├────────────────────┼────────────────────╫──────────────────┬──────────────────┼──────────────────┤
  │ input expression 2 │ input value 2a,    ║ input entry 2.1  │ input entry 2.2  │         -        │
  │                    │ input value 2b     ║                  │                  │                  │
  ╞════════════════════╪════════════════════╬══════════════════╪══════════════════╪══════════════════╡
  │ output label       │ output value 1a,   ║ output entry 1.1 │ output entry 1.2 │ output entry 1.3 │
  │                    │ output value 1b    ║                  │                  │                  │
  ├────────────────────┼────────────────────╫──────────────────┼──────────────────┼──────────────────┤
  │         U          │                    ║         1        │        2         │         3        │
  └────────────────────┴────────────────────╨──────────────────┴──────────────────┴──────────────────┘
"#;

pub const EX_10: &str = r#"
  ┌────────────────────────────────────────────────────────────────┐
  │ information item name                                          │
  ├──────────────────────────────────╥─────────────────────────────┤
  │                                  ║      input expression 1     │
  │           output label           ╟──────────────┬──────────────┤
  │                                  ║ input entry  │ input entry  │
  │                                  ║      1.1     │      1.2     │
  ╞════════════════════╤═════════════╬══════════════╪══════════════╡
  │                    │ input entry ║ output entry │ output entry │
  │                    │     2.1     ║      1.1     │      1.3     │
  │ input expression 2 ├─────────────╫──────────────┼──────────────┤
  │                    │ input entry ║ output entry │ output entry │
  │                    │     2.2     ║      1.2     │      1.4     │
  └────────────────────┴─────────────╨──────────────┴──────────────┘
"#;

fn eq_vectors(actual: &[String], expected: &[&str]) {
  assert_eq!(actual.len(), expected.len());
  for (index, value) in actual.iter().enumerate() {
    assert_eq!(value, expected[index]);
  }
}

fn eq_matrices(actual: &[Vec<String>], expected: &[&[&str]]) {
  assert_eq!(actual.len(), expected.len());
  for (r, row) in actual.iter().enumerate() {
    assert_eq!(row.len(), expected[r].len());
    for (c, col) in row.iter().enumerate() {
      assert_eq!(col, expected[r][c]);
    }
  }
}
