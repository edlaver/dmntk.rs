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

mod compatibility;
pub mod decision_logic;
pub mod decision_tables;
mod examples;
pub mod input_data;
pub mod item_definition;

pub use compatibility::*;
pub use examples::valid::*;
pub use examples::*;

use std::fmt::Write;

/// Generates possible decision table variants.
#[rustfmt::skip]
pub fn generate_decision_table_variants() -> String {
  let mut buffer = String::new();
  let orientation = ["horizontal", "vertical", "crosstab"];
  let information_item = ["present", "absent"];
  let output_label = ["present", "absent"];
  let allowed_values = ["absent", "present"];
  let inputs = ["absent", "single", "double", "multiple"];
  let outputs = ["single", "double", "multiple"];
  let annotations = ["absent", "single", "double", "multiple"];
  let total_variants = orientation.len() * information_item.len() * output_label.len() * allowed_values.len() * inputs.len() * outputs.len() * annotations.len();
  assert_eq!(1152, total_variants);
  let _ = writeln!(&mut buffer, "┌─────────────┬─────────────┬─────────┬─────────┬──────────┬──────────┬─────────────┬─────────┬────────┐");
  let _ = writeln!(&mut buffer, "│  Preferred  │ Information │ Output  │ Allowed │  Inputs  │ Outputs  │ Annotations │ Example │ Status │");
  let _ = writeln!(&mut buffer, "│ orientation │  item name  │  label  │ values  │          │          │             │         │        │");
  let _ = writeln!(&mut buffer, "├─────────────┼─────────────┼─────────┼─────────┼──────────┼──────────┼─────────────┼─────────┼────────┤");
  let mut counter = 1;
  for v_decision_table_orientation in orientation {
    for v_information_item_name in information_item {
      for v_output_label in output_label {
        for v_allowed_values in allowed_values {
          for v_inputs in inputs {
            for v_outputs in outputs {
              for v_annotations in annotations {
                let _ = writeln!(&mut buffer, "│{v_decision_table_orientation:^13}│{v_information_item_name:^13}│{v_output_label:^9}│{v_allowed_values:^9}│{v_inputs:^10}│{v_outputs:^10}│{v_annotations:^13}│ DT_{counter:04} │        │");
                counter += 1;
              }
            }
          }
        }
      }
    }
  }
  let _ = writeln!(&mut buffer, "└─────────────┴─────────────┴─────────┴─────────┴──────────┴──────────┴─────────────┴─────────┴────────┘");
  buffer
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_generate_decision_table_variants() {
    let text = generate_decision_table_variants();
    assert_eq!(1157, text.lines().count());
  }
}
