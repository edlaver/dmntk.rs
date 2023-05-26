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

//! # ASCII report of the DMN model

use dmntk_common::{color_256, write_indented, AsciiLine, AsciiNode, AsciiNodeBuilder, ColorMode};
use dmntk_model::model::*;
use std::fmt::Write;

const LABEL_DECISIONS: &str = "Decisions";
const LABEL_DECISIONS_MADE: &str = "decisions made";
const LABEL_DECISIONS_OWNED: &str = "decisions owned";
const LABEL_DESCRIPTION: &str = "description";
const LABEL_ID: &str = "id";
const LABEL_INPUT_DATA: &str = "Input data";
const LABEL_LABEL: &str = "label";
const LABEL_IMPACTING_DECISIONS: &str = "impacting decisions";
const LABEL_MODEL: &str = "Model";
const LABEL_NAME: &str = "name";
const LABEL_NAMESPACE: &str = "namespace";
const LABEL_ORGANISATION_UNITS: &str = "Organisation units";
const LABEL_PERFORMANCE_INDICATORS: &str = "Performance indicators";
const LABEL_TYPE: &str = "type";
const LABEL_URI: &str = "URI";
const LABEL_VARIABLE: &str = "variable (output)";

/// Color palette.
struct Colors {
  color_mode: ColorMode,
  color_name: String,
  color_label: String,
  color_id: String,
  color_uri: String,
  color_description: String,
  color_type: String,
  color_href: String,
}

impl Colors {
  /// Creates a new color palette based on color mode.
  fn new(color_mode: ColorMode) -> Self {
    Self {
      color_mode,
      color_name: color_256!(color_mode, 184),
      color_label: color_256!(color_mode, 209),
      color_id: color_256!(color_mode, 82),
      color_uri: color_256!(color_mode, 56),
      color_description: color_256!(color_mode, 255),
      color_type: color_256!(color_mode, 74),
      color_href: color_256!(color_mode, 61),
    }
  }
  fn mode(&self) -> &ColorMode {
    &self.color_mode
  }
  fn name(&self) -> &str {
    &self.color_name
  }
  fn label(&self) -> &str {
    &self.color_label
  }
  fn id(&self) -> &str {
    &self.color_id
  }
  fn uri(&self) -> &str {
    &self.color_uri
  }
  fn description(&self) -> &str {
    &self.color_description
  }
  fn typ(&self) -> &str {
    &self.color_type
  }
  fn href(&self) -> &str {
    &self.color_href
  }
}

/// Prints the model report to standard output.
pub fn print_model(definitions: &Definitions, color_mode: ColorMode) {
  let colors = &Colors::new(color_mode);
  let indent = 2;
  let mut output = String::new();
  write_model(&mut output, definitions, colors, indent);
  write_decisions(&mut output, definitions, colors, indent);
  write_input_data(&mut output, definitions, colors, indent);
  write_performance_indicators(&mut output, definitions, colors, indent);
  write_organisation_units(&mut output, definitions, colors, indent);
  println!("{}", output);
}

/// Writes a title text.
fn write_title(w: &mut dyn Write, title: &str) {
  let bar = "─".repeat(title.len());
  let _ = writeln!(w);
  let _ = writeln!(w, "┌─{}─┐", bar);
  let _ = writeln!(w, "│ {} │", title);
  let _ = writeln!(w, "└─{}─┘", bar);
}

/// Write model properties.
fn write_model(w: &mut dyn Write, definitions: &Definitions, colors: &Colors, indent: usize) {
  write_title(w, LABEL_MODEL);
  let node_model = AsciiNode::node_builder(AsciiLine::builder().with_color(definitions.name(), colors.name()).build())
    .child(build_namespace_leaf(definitions.namespace(), colors))
    .opt_child(build_label(definitions.label(), colors))
    .opt_child(build_id(definitions.id(), colors))
    .opt_child(build_description(definitions.description(), colors))
    .build();
  let _ = write_indented(w, &node_model, colors.mode(), indent);
}

/// Write decisions.
fn write_decisions(w: &mut dyn Write, definitions: &Definitions, colors: &Colors, indent: usize) {
  let decisions = definitions.decisions();
  if !decisions.is_empty() {
    write_title(w, LABEL_DECISIONS);
  }
  for decision in decisions {
    let node_decision = builder_name(decision.name(), colors)
      .opt_child(build_label(decision.label(), colors))
      .opt_child(build_id(decision.id(), colors))
      .opt_child(build_description(decision.description(), colors))
      .child(build_variable(decision.variable(), colors))
      .build();
    let _ = write_indented(w, &node_decision, colors.mode(), indent);
  }
}

/// Write input data.
fn write_input_data(w: &mut dyn Write, definitions: &Definitions, colors: &Colors, indent: usize) {
  let input_data = definitions.input_data();
  if !input_data.is_empty() {
    write_title(w, LABEL_INPUT_DATA);
  }
  for input in input_data {
    let node_input = builder_name(input.name(), colors)
      .opt_child(build_label(input.label(), colors))
      .opt_child(build_id(input.id(), colors))
      .opt_child(build_description(input.description(), colors))
      .child(build_variable(input.variable(), colors))
      .build();
    let _ = write_indented(w, &node_input, colors.mode(), indent);
  }
}

/// Write performance indicators.
fn write_performance_indicators(w: &mut dyn Write, definitions: &Definitions, colors: &Colors, indent: usize) {
  let performance_indicators = definitions.performance_indicators();
  if !performance_indicators.is_empty() {
    write_title(w, LABEL_PERFORMANCE_INDICATORS);
  }
  for performance_indicator in performance_indicators {
    // prepare a node with impacting decisions
    let mut impacting_decisions_builder = AsciiNode::node_builder(AsciiLine::builder().text(LABEL_IMPACTING_DECISIONS).colon().build());
    for impacting_decision in performance_indicator.impacting_decisions() {
      impacting_decisions_builder.add_child(build_href(impacting_decision.into(), colors));
    }
    let node_impacting_decisions = impacting_decisions_builder.build();
    let node_performance_indicator = builder_name(performance_indicator.name(), colors)
      .opt_child(build_label(performance_indicator.label(), colors))
      .opt_child(build_id(performance_indicator.id(), colors))
      .opt_child(build_description(performance_indicator.description(), colors))
      .opt_child(build_uri(performance_indicator.uri(), colors))
      .child(node_impacting_decisions)
      .build();
    let _ = write_indented(w, &node_performance_indicator, colors.mode(), indent);
  }
}

/// Write organisation units.
fn write_organisation_units(w: &mut dyn Write, definitions: &Definitions, colors: &Colors, indent: usize) {
  let organisation_units = definitions.organisation_units();
  if !organisation_units.is_empty() {
    write_title(w, LABEL_ORGANISATION_UNITS);
  }
  for organisation_units in definitions.organisation_units() {
    // prepare a node with decisions made
    let mut decisions_made_builder = AsciiNode::node_builder(AsciiLine::builder().text(LABEL_DECISIONS_MADE).colon().build());
    for decision_made in organisation_units.decisions_made() {
      decisions_made_builder.add_child(build_href(decision_made.into(), colors));
    }
    let node_decisions_made = decisions_made_builder.build();
    // prepare a node with decisions owned
    let mut decisions_made_builder = AsciiNode::node_builder(AsciiLine::builder().text(LABEL_DECISIONS_OWNED).colon().build());
    for decision_owned in organisation_units.decisions_owned() {
      decisions_made_builder.add_child(build_href(decision_owned.into(), colors));
    }
    let node_decisions_owned = decisions_made_builder.build();
    let node_organisation_unit = builder_name(organisation_units.name(), colors)
      .opt_child(build_label(organisation_units.label(), colors))
      .opt_child(build_id(organisation_units.id(), colors))
      .opt_child(build_description(organisation_units.description(), colors))
      .opt_child(build_uri(organisation_units.uri(), colors))
      .child(node_decisions_made)
      .child(node_decisions_owned)
      .build();
    let _ = write_indented(w, &node_organisation_unit, colors.mode(), indent);
  }
}

/// Prepares a node builder containing a name as a root element.
fn builder_name(text: &str, colors: &Colors) -> AsciiNodeBuilder {
  AsciiNode::node_builder(AsciiLine::builder().with_color(text, colors.name()).build())
}

/// Builds a leaf node containing a name.
fn build_name(text: &str, colors: &Colors) -> AsciiNode {
  build_labeled_text_leaf(LABEL_NAME, text, colors.name())
}

/// Builds a leaf node containing the value of the identifier.
fn build_id(opt_text: &Option<String>, colors: &Colors) -> Option<AsciiNode> {
  build_opt_labeled_text_leaf(LABEL_ID, opt_text, colors.id())
}

/// Builds a leaf node containing description.
fn build_description(opt_text: &Option<String>, colors: &Colors) -> Option<AsciiNode> {
  opt_text.as_ref().map(|text| {
    let mut leaf_builder = AsciiNode::leaf_builder();
    leaf_builder.add_line(AsciiLine::builder().text(LABEL_DESCRIPTION).colon().build());
    for line in text.lines().map(|s| s.trim()).filter(|s| !s.is_empty()) {
      leaf_builder.add_line(AsciiLine::builder().indent().with_color(line, colors.description()).build());
    }
    leaf_builder.build()
  })
}

/// Builds a leaf node containing the value of the label.
fn build_label(opt_text: &Option<String>, colors: &Colors) -> Option<AsciiNode> {
  build_opt_labeled_text_leaf(LABEL_LABEL, opt_text, colors.label())
}

/// Builds a leaf node containing a label.
fn build_href(text: &str, colors: &Colors) -> AsciiNode {
  AsciiNode::leaf_builder()
    .line(AsciiLine::builder().with_color("#", colors.href()).with_color(text, colors.href()).build())
    .build()
}

/// Builds a leaf node containing an URI.
fn build_uri(opt_text: &Option<String>, colors: &Colors) -> Option<AsciiNode> {
  build_opt_labeled_text_leaf(LABEL_URI, opt_text, colors.uri())
}

/// Builds a leaf node containing a namespace.
fn build_namespace_leaf(text: &str, colors: &Colors) -> AsciiNode {
  build_labeled_text_leaf(LABEL_NAMESPACE, text, colors.uri())
}

/// Builds a leaf node containing a type.
fn build_type(opt_text: &Option<String>, colors: &Colors) -> Option<AsciiNode> {
  build_opt_labeled_text_leaf(LABEL_TYPE, opt_text, colors.typ())
}

/// Builds a node containing output variable properties.
fn build_variable(variable: &InformationItem, colors: &Colors) -> AsciiNode {
  AsciiNode::node_builder(AsciiLine::builder().text(LABEL_VARIABLE).build())
    .child(build_name(variable.name(), colors))
    .opt_child(build_label(variable.label(), colors))
    .opt_child(build_type(variable.type_ref(), colors))
    .opt_child(build_description(variable.description(), colors))
    .build()
}

/// Builds a leaf with a single line containing label and coloured value.
fn build_labeled_text_leaf(label: &str, text: &str, color: &str) -> AsciiNode {
  AsciiNode::leaf_builder()
    .line(AsciiLine::builder().text(label).colon_space().with_color(text, color).build())
    .build()
}

/// Builds optional a leaf with a single line containing label and coloured value.
fn build_opt_labeled_text_leaf(label: &str, opt_text: &Option<String>, color: &str) -> Option<AsciiNode> {
  opt_text.as_ref().map(|text| {
    AsciiNode::leaf_builder()
      .line(AsciiLine::builder().text(label).colon_space().with_color(text, color).build())
      .build()
  })
}
