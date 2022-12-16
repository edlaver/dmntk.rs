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

use crate::recognizer::Recognizer;
use crate::tests::{eq_matrices, eq_vectors, EX_01, EX_02, EX_03, EX_05, EX_06, EX_07, EX_08, EX_09, EX_10, EX_ERR_01, EX_ERR_02, EX_ERR_03};
use dmntk_examples::decision_tables::H_110010;
use dmntk_model::model::{BuiltinAggregator, DecisionTableOrientation, HitPolicy};

const EMPTY_VECTOR: &[&str] = &[];
const EMPTY_MATRIX: &[&[&str]] = &[];

fn no_information_item_name(recognizer: &Recognizer) {
  assert!(recognizer.information_item_name.is_none());
}

fn eq_information_item_name(recognizer: &Recognizer, expected: &str) {
  assert!(recognizer.information_item_name.is_some());
  assert_eq!(recognizer.information_item_name.as_ref().unwrap(), expected);
}

fn eq_hit_policy(recognizer: &Recognizer, expected: HitPolicy) {
  assert_eq!(recognizer.hit_policy, expected);
}

fn eq_orientation(recognizer: &Recognizer, expected: DecisionTableOrientation) {
  assert_eq!(recognizer.orientation, expected);
}

fn eq_output_label(recognizer: &Recognizer, expected: &str) {
  assert!(recognizer.output_label.is_some());
  assert_eq!(recognizer.output_label.as_ref().unwrap(), expected);
}

fn eq_input_expressions(recognizer: &Recognizer, expected: &[&str]) {
  eq_vectors(&recognizer.input_expressions, expected);
}

fn eq_input_values(recognizer: &Recognizer, expected: &[&str]) {
  eq_vectors(&recognizer.input_values, expected);
}

fn eq_input_entries(recognizer: &Recognizer, expected: &[&[&str]]) {
  eq_matrices(&recognizer.input_entries, expected);
}

fn eq_output_components(recognizer: &Recognizer, expected: &[&str]) {
  eq_vectors(&recognizer.output_components, expected);
}

fn eq_output_values(recognizer: &Recognizer, expected: &[&str]) {
  eq_vectors(&recognizer.output_values, expected);
}

fn eq_output_entries(recognizer: &Recognizer, expected: &[&[&str]]) {
  eq_matrices(&recognizer.output_entries, expected);
}

fn eq_annotations(recognizer: &Recognizer, expected: &[&str]) {
  eq_vectors(&recognizer.annotations, expected);
}

fn eq_annotation_entries(recognizer: &Recognizer, expected: &[&[&str]]) {
  eq_matrices(&recognizer.annotation_entries, expected);
}

#[test]
fn test_dt_0001() {
  let recognizer = &Recognizer::recognize(&String::from(H_110010)).unwrap();
  eq_orientation(recognizer, DecisionTableOrientation::RuleAsRow);
  eq_information_item_name(recognizer, " Weekdays ");
  eq_hit_policy(recognizer, HitPolicy::Collect(BuiltinAggregator::List));
  eq_input_expressions(recognizer, EMPTY_VECTOR);
  eq_input_values(recognizer, EMPTY_VECTOR);
  eq_input_entries(
    recognizer,
    &[EMPTY_VECTOR, EMPTY_VECTOR, EMPTY_VECTOR, EMPTY_VECTOR, EMPTY_VECTOR, EMPTY_VECTOR, EMPTY_VECTOR],
  );
  eq_output_label(recognizer, "   Weekday   ");
  eq_output_components(recognizer, EMPTY_VECTOR);
  eq_output_values(recognizer, EMPTY_VECTOR);
  eq_output_entries(
    recognizer,
    &[
      &[r#"  "Monday"   "#],
      &[r#"  "Tuesday"  "#],
      &[r#" "Wednesday" "#],
      &[r#" "Thursday"  "#],
      &[r#"  "Friday"   "#],
      &[r#" "Saturday"  "#],
      &[r#"  "Sunday"   "#],
    ],
  );
  eq_annotations(recognizer, EMPTY_VECTOR);
  eq_annotation_entries(recognizer, EMPTY_MATRIX);
}

#[test]
fn test_example_01() {
  let rec = &Recognizer::recognize(&String::from(EX_01)).unwrap();
  no_information_item_name(rec);
  eq_hit_policy(rec, HitPolicy::Unique);
  eq_orientation(rec, DecisionTableOrientation::RuleAsRow);
  eq_input_expressions(rec, &["  Customer  ", " Order "]);
  eq_input_values(rec, EMPTY_VECTOR);
  eq_input_entries(rec, &[&[r#" "Business" "#, "  <10  "], &[r#" "Business" "#, " >=10  "], &[r#" "Private"  "#, "   -   "]]);
  eq_output_label(rec, "      ");
  eq_output_components(rec, EMPTY_VECTOR);
  eq_output_values(rec, EMPTY_VECTOR);
  eq_output_entries(rec, &[&[" 0.10 "], &[" 0.15 "], &[" 0.05 "]]);
  eq_annotations(rec, EMPTY_VECTOR);
  eq_annotation_entries(rec, EMPTY_MATRIX);
}

#[test]
fn test_example_02() {
  let rec = &Recognizer::recognize(&String::from(EX_02)).unwrap();
  no_information_item_name(rec);
  eq_hit_policy(rec, HitPolicy::Unique);
  eq_orientation(rec, DecisionTableOrientation::RuleAsRow);
}

#[test]
fn test_example_03() {
  let rec = &Recognizer::recognize(&String::from(EX_03)).unwrap();
  no_information_item_name(rec);
  eq_hit_policy(rec, HitPolicy::Unique);
  eq_orientation(rec, DecisionTableOrientation::RuleAsRow);
}

#[test]
fn ex_0033() {
  let rec = &Recognizer::recognize(&String::from(EX_05)).unwrap();
  no_information_item_name(rec);
  eq_hit_policy(rec, HitPolicy::Unique);
  eq_orientation(rec, DecisionTableOrientation::RuleAsColumn);
}

#[test]
fn ex_0044() {
  let rec = &Recognizer::recognize(&String::from(EX_06)).unwrap();
  no_information_item_name(rec);
  eq_hit_policy(rec, HitPolicy::Unique);
  eq_orientation(rec, DecisionTableOrientation::RuleAsColumn);
  eq_input_expressions(rec, &[" Applicant age                   ", " Medical history                 "]);
  eq_input_values(rec, EMPTY_VECTOR);
  eq_input_entries(
    rec,
    &[
      &["     <25       ", r#""good""#],
      &["     <25       ", r#" "bad"  "#],
      &[" [25..60] ", "     -    "],
      &["      >60      ", r#" "good" "#],
      &["      >60      ", r#""bad" "#],
    ],
  );
  eq_output_label(rec, " Sell    \n         \n options ");
  eq_output_components(rec, &[" Applicant risk rating ", " Special Discount      "]);
  eq_output_values(rec, EMPTY_VECTOR);
  eq_output_entries(
    rec,
    &[
      &[r#""Low" "#, "  10  "],
      &[r#""Medium""#, "    5   "],
      &[r#" "Medium" "#, "     5    "],
      &[r#""Medium""#, "    5   "],
      &[r#""High""#, "  0   "],
    ],
  );
  eq_annotations(rec, &[" Additional acceptance           ", " Reference                       "]);
  eq_annotation_entries(
    rec,
    &[
      &[" No   ", " Rf 0 "],
      &["   No   ", "  Rf 1  "],
      &["    No    ", "   Rf 2   "],
      &["   No   ", "  Rf 3  "],
      &[" Yes  ", " Rf 4 "],
    ],
  );
}

#[test]
fn ex_0064() {
  let rec = &Recognizer::recognize(&String::from(EX_07)).unwrap();
  eq_information_item_name(rec, " Sell options                                                                     ");
  eq_hit_policy(rec, HitPolicy::Unique);
  eq_orientation(rec, DecisionTableOrientation::RuleAsColumn);
  eq_input_expressions(rec, &[" Applicant age                   ", " Medical history                 "]);
  eq_input_values(rec, &[" <25,[25..60],>60    ", r#"   "good","bad"      "#]);
  eq_input_entries(
    rec,
    &[
      &["     <25       ", r#""good""#],
      &["     <25       ", r#" "bad"  "#],
      &[" [25..60] ", "     -    "],
      &["      >60      ", r#" "good" "#],
      &["      >60      ", r#""bad" "#],
    ],
  );
  eq_output_label(rec, " Sell    \n         \n options ");
  eq_output_components(rec, &[" Applicant risk rating ", " Special Discount      "]);
  eq_output_entries(
    rec,
    &[
      &[r#""Low" "#, "  10  "],
      &[r#""Medium""#, "    5   "],
      &[r#" "Medium" "#, "     5    "],
      &[r#""Medium""#, "    5   "],
      &[r#""High""#, "  0   "],
    ],
  );
  eq_annotations(rec, &[" Additional acceptance           ", " Reference                       "]);
  eq_annotation_entries(
    rec,
    &[
      &[" No   ", " Rf 0 "],
      &["   No   ", "  Rf 1  "],
      &["    No    ", "   Rf 2   "],
      &["   No   ", "  Rf 3  "],
      &[" Yes  ", " Rf 4 "],
    ],
  );
}

#[test]
fn general_horizontal() {
  let rec = &Recognizer::recognize(&String::from(EX_08)).unwrap();
  eq_information_item_name(rec, " information item name     ");
  eq_hit_policy(rec, HitPolicy::Collect(BuiltinAggregator::List));
  eq_orientation(rec, DecisionTableOrientation::RuleAsRow);
  eq_input_expressions(rec, &[" input expression 1 ", " input expression 2 "]);
  eq_input_values(rec, &[" input value 1a,    \n   input value 1b   ", " input value 2a,    \n   input value 2b   "]);
  eq_input_entries(
    rec,
    &[
      &["                    \n  input entry 1.1   \n                    ", "  input entry 2.1   "],
      &["                    \n  input entry 1.1   \n                    ", "  input entry 2.2   "],
      &["  input entry 1.2   ", "         -          "],
      &["  input entry 1.3   ", "  input entry 2.3   "],
    ],
  );
  eq_output_label(rec, "    output label    ");
  eq_output_components(rec, EMPTY_VECTOR);
  eq_output_values(rec, &[" output value 1a,   \n   output value 1b  "]);
  eq_output_entries(
    rec,
    &[&["  output entry 1.1  "], &["  output entry 1.2  "], &["  output entry 1.3  "], &["  output entry 1.4  "]],
  );
  eq_annotations(rec, EMPTY_VECTOR);
  eq_annotation_entries(rec, EMPTY_MATRIX);
}

#[test]
fn general_vertical() {
  let rec = &Recognizer::recognize(&String::from(EX_09)).unwrap();
  eq_information_item_name(rec, "   information item name   ");
  eq_hit_policy(rec, HitPolicy::Unique);
  eq_orientation(rec, DecisionTableOrientation::RuleAsColumn);
}

#[test]
fn general_cross_tab() {
  assert!(&Recognizer::recognize(&String::from(EX_10)).is_err());
  // eq_information_item_name(rec, " information item name                                          ");
  // eq_hit_policy(rec, HitPolicy::Unique);
  // eq_orientation(rec, DecisionTableOrientation::Crosstab);
  // eq_input_expressions(rec, EMPTY_VECTOR);
  // eq_input_values(rec, EMPTY_VECTOR);
  // eq_input_entries(rec, EMPTY_MATRIX);
  // no_output_label(rec);
  // eq_output_components(rec, EMPTY_VECTOR);
  // eq_output_values(rec, EMPTY_VECTOR);
  // eq_output_entries(rec, EMPTY_MATRIX);
  // eq_annotations(rec, EMPTY_VECTOR);
  // eq_annotation_entries(rec, EMPTY_MATRIX);
}

#[test]
fn test_err_01() {
  assert_eq!(
    "RecognizerError: expected characters not found: ['╬']",
    Recognizer::recognize(&String::from(EX_ERR_01)).err().unwrap().to_string()
  );
}

#[test]
fn test_err_02() {
  assert_eq!(
    "RecognizerError: character ' ' is not allowed in ['─', '┴']",
    Recognizer::recognize(&String::from(EX_ERR_02)).err().unwrap().to_string()
  );
}

#[test]
fn test_err_03() {
  assert_eq!(
    "RecognizerError: rectangle is not closed, start point: (0,1), end point: (0,0)",
    Recognizer::recognize(&String::from(EX_ERR_03)).err().unwrap().to_string()
  );
}
