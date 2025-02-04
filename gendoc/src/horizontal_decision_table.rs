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

//! Builder for `HTML` representation of the horizontal decision table.

use crate::defs::*;
use dmntk_model::*;
use domrs::HtmlElement;

pub fn create_horizontal_decision_table_elements(decision_table: &DecisionTable) -> HtmlElement {
  // prepare an element containing the decision table
  let mut decision_table_element = HtmlElement::new_div(CLASS_DECISION_TABLE);

  // prepare information item name
  if let Some(information_item_name) = decision_table.information_item_name() {
    let mut html_information_item_name = HtmlElement::new_div(CLASS_INFORMATION_ITEM_NAME);
    html_information_item_name.set_content(information_item_name);
    decision_table_element.add_child(html_information_item_name);
  }

  // prepare grid container
  let mut html_grid_container = HtmlElement::new_div(CLASS_GRID_CONTAINER);

  // prepare grid body
  let mut html_grid_body = HtmlElement::new_div(CLASS_GRID_BODY);
  html_grid_body.set_style(&prepare_style_grid_template_columns(decision_table));

  // prepare horizontal double line
  let mut html_horizontal_double_line = HtmlElement::new_div(CLASS_HORIZONTAL_DOUBLE_LINE);
  html_horizontal_double_line.set_style(&prepare_style_horizontal_double_line(decision_table));
  html_grid_body.add_child(html_horizontal_double_line);

  // prepare first vertical double line
  let mut html_first_vertical_double_line = HtmlElement::new_div(CLASS_VERTICAL_DOUBLE_LINE);
  html_first_vertical_double_line.set_style(&prepare_style_first_vertical_double_line(decision_table));
  html_grid_body.add_child(html_first_vertical_double_line);

  // prepare second vertical line when needed
  if decision_table.annotations().len() > 0 {
    let mut html_second_vertical_double_line = HtmlElement::new_div(CLASS_VERTICAL_DOUBLE_LINE);
    html_second_vertical_double_line.set_style(&prepare_style_second_vertical_double_line(decision_table));
    html_grid_body.add_child(html_second_vertical_double_line);
  }

  // add hit policy
  let mut html_hit_policy = HtmlElement::new_div(CLASS_HIT_POLICY);
  html_hit_policy.set_style(&prepare_style_hit_policy(decision_table));
  html_hit_policy.set_content(&decision_table.hit_policy().to_string());
  html_grid_body.add_child(html_hit_policy);

  // add input clauses
  for input_clause in decision_table.input_clauses() {
    let mut html_input_expression = HtmlElement::new_div(CLASS_INPUT_EXPRESSION);
    html_input_expression.set_style(&prepare_style_input_expression(decision_table));
    html_input_expression.set_content(&input_clause.input_expression);
    html_grid_body.add_child(html_input_expression);
  }

  // add output clauses
  if decision_table.output_clauses().len() > 1 {
    if let Some(output_label) = &decision_table.output_label() {
      let mut html_output_label = HtmlElement::new_div(CLASS_OUTPUT_LABEL);
      html_output_label.set_style(&prepare_style_output_label(decision_table.output_clauses().len()));
      html_output_label.set_content(output_label);
      html_grid_body.add_child(html_output_label);
    } else {
      for output_clause in decision_table.output_clauses() {
        let mut html_output_component = HtmlElement::new_div(CLASS_OUTPUT_COMPONENT);
        if let Some(output_clause_name) = &output_clause.name {
          html_output_component.set_content(output_clause_name);
        } else {
          html_output_component.set_content("&nbsp;");
        }
        html_grid_body.add_child(html_output_component);
      }
    }
  } else {
    let mut html_output_label = HtmlElement::new_div(CLASS_OUTPUT_LABEL);
    if let Some(output_label) = &decision_table.output_label() {
      html_output_label.set_content(output_label);
    } else {
      html_output_label.set_content("&nbsp;");
    }
    html_grid_body.add_child(html_output_label);
  }

  // add annotation clauses
  for annotation in decision_table.annotations() {
    let mut html_annotation_label = HtmlElement::new_div(CLASS_ANNOTATION_LABEL);
    html_annotation_label.set_style(&prepare_style_annotation_label(decision_table));
    html_annotation_label.set_content(&annotation.name);
    html_grid_body.add_child(html_annotation_label);
  }

  if decision_table.output_label().is_some() && decision_table.output_clauses().len() > 1 {
    for output_clause in decision_table.output_clauses() {
      let mut html_output_component = HtmlElement::new_div(CLASS_OUTPUT_COMPONENT);
      if let Some(output_clause_name) = &output_clause.name {
        html_output_component.set_content(output_clause_name);
      } else {
        html_output_component.set_content("&nbsp;");
      }
      html_grid_body.add_child(html_output_component);
    }
  }

  if decision_table.allowed_values_present() {
    for input_clause in decision_table.input_clauses() {
      let mut html_input_allowed_values = HtmlElement::new_div(CLASS_INPUT_ALLOWED_VALUES);
      if let Some(input_allowed_values) = &input_clause.allowed_input_values {
        html_input_allowed_values.set_content(input_allowed_values);
      } else {
        html_input_allowed_values.set_content("&nbsp;");
      }
      html_grid_body.add_child(html_input_allowed_values);
    }
    for output_clause in decision_table.output_clauses() {
      let mut html_output_allowed_values = HtmlElement::new_div(CLASS_OUTPUT_ALLOWED_VALUES);
      if let Some(output_allowed_values) = &output_clause.allowed_output_values {
        html_output_allowed_values.set_content(output_allowed_values);
      } else {
        html_output_allowed_values.set_content("&nbsp;");
      }
      html_grid_body.add_child(html_output_allowed_values);
    }
    for _ in decision_table.annotations() {
      html_grid_body.add_child(HtmlElement::new_div(CLASS_ANNOTATION_ALLOWED_VALUES));
    }
  }

  // add rules
  for (rule_number, rule) in decision_table.rules().enumerate() {
    let mut html_rule_number = HtmlElement::new_div(CLASS_RULE_NUMBER);
    html_rule_number.set_content(&format!("{}", rule_number + 1));
    html_grid_body.add_child(html_rule_number);
    for input_entry in &rule.input_entries {
      let mut html_input_entry = HtmlElement::new_div(CLASS_INPUT_ENTRY);
      html_input_entry.set_content(&input_entry.text);
      html_grid_body.add_child(html_input_entry);
    }
    for output_entry in &rule.output_entries {
      let mut html_output_entry = HtmlElement::new_div(CLASS_OUTPUT_ENTRY);
      html_output_entry.set_content(&output_entry.text);
      html_grid_body.add_child(html_output_entry);
    }
    for annotation_entry in &rule.annotation_entries {
      let mut html_annotation_entry = HtmlElement::new_div(CLASS_ANNOTATION_ENTRY);
      html_annotation_entry.set_content(&annotation_entry.text);
      html_grid_body.add_child(html_annotation_entry);
    }
  }

  // finalize the decision table
  html_grid_container.add_child(html_grid_body);
  decision_table_element.add_child(html_grid_container);
  decision_table_element
}

fn prepare_style_grid_template_columns(decision_table: &DecisionTable) -> String {
  let column_count = get_column_count(decision_table);
  let column_widths = " auto".to_string().repeat(column_count);
  format!("grid-template-columns:{column_widths};")
}

fn prepare_style_horizontal_double_line(decision_table: &DecisionTable) -> String {
  let column_count = get_column_count(decision_table);
  let row_start = get_header_row_count(decision_table) + 1;
  format!("grid-column: 1 / span {}; grid-row: {} / span 1;", column_count, row_start)
}

fn prepare_style_first_vertical_double_line(decision_table: &DecisionTable) -> String {
  let row_count = get_row_count(decision_table) + decision_table.rules().len();
  let col_start = 2 + decision_table.input_clauses().len();
  format!("grid-column: {} / span 1; grid-row: 1 / span {};", col_start, row_count)
}

fn prepare_style_second_vertical_double_line(decision_table: &DecisionTable) -> String {
  let row_count = get_row_count(decision_table) + decision_table.rules().len();
  let col_start = 3 + decision_table.input_clauses().len() + decision_table.output_clauses().len();
  format!("grid-column: {} / span 1; grid-row: 1 / span {};", col_start, row_count)
}

fn prepare_style_hit_policy(decision_table: &DecisionTable) -> String {
  let row_span = get_header_row_count(decision_table);
  format!("grid-row: span {};", row_span)
}

fn prepare_style_input_expression(decision_table: &DecisionTable) -> String {
  format!("grid-row: span {};", get_row_span(decision_table))
}

fn prepare_style_output_label(output_clauses: usize) -> String {
  format!("grid-column: span {};", output_clauses)
}

fn prepare_style_annotation_label(decision_table: &DecisionTable) -> String {
  format!("grid-row: span {};", get_row_span(decision_table))
}

fn get_column_count(decision_table: &DecisionTable) -> usize {
  let mut column_count = 2 + decision_table.input_clauses().len() + decision_table.output_clauses().len() + decision_table.annotations().len();
  if decision_table.annotations().len() > 0 {
    column_count += 1;
  }
  column_count
}

fn get_header_row_count(decision_table: &DecisionTable) -> usize {
  match (
    decision_table.output_label().is_some(),
    decision_table.allowed_values_present(),
    decision_table.output_clauses().len() > 1,
  ) {
    (false, false, false) => 1,
    (false, false, true) => 1,
    (false, true, false) => 2,
    (false, true, true) => 2,
    (true, false, false) => 1,
    (true, false, true) => 2,
    (true, true, false) => 2,
    (true, true, true) => 3,
  }
}

fn get_row_count(decision_table: &DecisionTable) -> usize {
  match (
    decision_table.output_label().is_some(),
    decision_table.allowed_values_present(),
    decision_table.output_clauses().len() > 1,
  ) {
    (false, false, false) => 2,
    (false, false, true) => 2,
    (false, true, false) => 3,
    (false, true, true) => 3,
    (true, false, false) => 2,
    (true, false, true) => 3,
    (true, true, false) => 3,
    (true, true, true) => 4,
  }
}

fn get_row_span(decision_table: &DecisionTable) -> usize {
  match (
    decision_table.output_label().is_some(),
    decision_table.allowed_values_present(),
    decision_table.output_clauses().len() > 1,
  ) {
    (false, false, false) => 1,
    (false, false, true) => 1,
    (false, true, false) => 1,
    (false, true, true) => 1,
    (true, false, false) => 1,
    (true, false, true) => 2,
    (true, true, false) => 1,
    (true, true, true) => 2,
  }
}
