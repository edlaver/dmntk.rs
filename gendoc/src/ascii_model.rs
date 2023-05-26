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
const LABEL_DESCRIPTION: &str = "description";
const LABEL_ID: &str = "id";
const LABEL_INPUT_DATA: &str = "Input data";
const LABEL_LABEL: &str = "label";
const LABEL_IMPACTING_DECISIONS: &str = "impacting decisions";
const LABEL_MODEL: &str = "Model";
const LABEL_NAMESPACE: &str = "namespace";
const LABEL_NONE: &str = "(none)";
const LABEL_PERFORMANCE_INDICATORS: &str = "Performance indicators";

/// Color palette.
struct Colors {
  color_mode: ColorMode,
  color_name: String,
  color_label: String,
  color_id: String,
  color_namespace: String,
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
      color_namespace: color_256!(color_mode, 56),
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
  fn namespace(&self) -> &str {
    &self.color_namespace
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

pub fn print_model(definitions: &Definitions, color_mode: ColorMode) {
  // prepare the color palette
  let colors = &Colors::new(color_mode);

  //-----------------------------------------------------------------------------------------------
  // node: decisions root
  //-----------------------------------------------------------------------------------------------
  let node_decisions_root = if definitions.decisions().is_empty() {
    AsciiNode::leaf_builder()
      .line(AsciiLine::builder().text(LABEL_DECISIONS).space().text(LABEL_NONE).build())
      .build()
  } else {
    let mut decision_node_builder = AsciiNode::node_builder(AsciiLine::builder().text(LABEL_DECISIONS).build());
    for decision in definitions.decisions() {
      decision_node_builder.add_child(
        AsciiNode::leaf_builder()
          .line(AsciiLine::builder().with_color(decision.name(), colors.name()).build())
          .build(),
      );
    }
    decision_node_builder.build()
  };

  //-----------------------------------------------------------------------------------------------
  // node: input data root
  //-----------------------------------------------------------------------------------------------
  let node_input_data_root = if definitions.input_data().is_empty() {
    AsciiNode::leaf_builder()
      .line(AsciiLine::builder().text(LABEL_INPUT_DATA).space().text(LABEL_NONE).build())
      .build()
  } else {
    let mut input_data_node_builder = AsciiNode::node_builder(AsciiLine::builder().text(LABEL_INPUT_DATA).build());
    for input_data in definitions.input_data() {
      let input_data_type = input_data.variable().type_ref().as_ref().cloned().unwrap_or(LABEL_NONE.to_string());
      input_data_node_builder.add_child(
        AsciiNode::leaf_builder()
          .line(
            AsciiLine::builder()
              .with_color(input_data.name(), colors.name())
              .space()
              .l_paren()
              .with_color(&input_data_type, colors.typ())
              .r_paren()
              .build(),
          )
          .build(),
      );
    }
    input_data_node_builder.build()
  };

  let indent = 1;
  let mut output = String::new();
  write_title(&mut output, LABEL_MODEL);
  write_model(&mut output, definitions, colors, indent);
  write_title(&mut output, LABEL_DECISIONS);
  let _ = write_indented(&mut output, &node_decisions_root, &color_mode, indent);
  write_title(&mut output, LABEL_INPUT_DATA);
  let _ = write_indented(&mut output, &node_input_data_root, &color_mode, indent);
  write_title(&mut output, LABEL_PERFORMANCE_INDICATORS);
  write_performance_indicators(&mut output, definitions, colors, indent);
  println!("{}", output);
}

/// Writes a title text.
fn write_title(w: &mut dyn Write, title: &str) {
  let bar = "─".repeat(title.len());
  let _ = writeln!(w, "┌─{}─┐", bar);
  let _ = writeln!(w, "│ {} │", title);
  let _ = writeln!(w, "└─{}─┘", bar);
}

/// Write model properties.
fn write_model(w: &mut dyn Write, definitions: &Definitions, colors: &Colors, indent: usize) {
  let node_model = AsciiNode::node_builder(AsciiLine::builder().with_color(definitions.name(), colors.name()).build())
    .child(build_namespace_leaf(definitions.namespace(), colors.namespace()))
    .opt_child(build_label(definitions.label(), colors.label()))
    .opt_child(build_id(definitions.id(), colors.id()))
    .build();
  let _ = write_indented(w, &node_model, colors.mode(), indent);
}

/// Write performance indicators.
fn write_performance_indicators(w: &mut dyn Write, definitions: &Definitions, colors: &Colors, indent: usize) {
  for performance_indicator in definitions.performance_indicators() {
    // prepare a node with impacting decisions
    let mut impacting_decisions_builder = AsciiNode::node_builder(AsciiLine::builder().text(LABEL_IMPACTING_DECISIONS).colon().build());
    for impacting_decision in performance_indicator.impacting_decisions() {
      impacting_decisions_builder.add_child(build_href(impacting_decision.into(), colors));
    }
    let node_impacting_decisions = impacting_decisions_builder.build();
    let node_performance_indicator = builder_name(performance_indicator.name(), colors.name())
      .opt_child(build_label(performance_indicator.label(), colors.label()))
      .opt_child(build_id(performance_indicator.id(), colors.id()))
      .opt_child(build_description(performance_indicator.description(), colors))
      .child(node_impacting_decisions)
      .build();
    let _ = write_indented(w, &node_performance_indicator, colors.mode(), indent);
  }
}

/// Builds a leaf node containing the value of the identifier.
fn build_id(opt_text: &Option<String>, color: &str) -> Option<AsciiNode> {
  opt_text.as_ref().map(|text| {
    AsciiNode::leaf_builder()
      .line(AsciiLine::builder().text(LABEL_ID).colon_space().with_color(text, color).build())
      .build()
  })
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
fn build_label(opt_text: &Option<String>, color: &str) -> Option<AsciiNode> {
  opt_text.as_ref().map(|text| {
    AsciiNode::leaf_builder()
      .line(AsciiLine::builder().text(LABEL_LABEL).colon_space().with_color(text, color).build())
      .build()
  })
}

/// Builds a leaf node containing the value of the label.
fn build_href(text: &str, colors: &Colors) -> AsciiNode {
  AsciiNode::leaf_builder()
    .line(AsciiLine::builder().with_color("#", colors.href()).with_color(text, colors.href()).build())
    .build()
}

/// Prepares a node builder containing a name as a root element.
fn builder_name(text: &str, color: &str) -> AsciiNodeBuilder {
  AsciiNode::node_builder(AsciiLine::builder().with_color(text, color).build())
}

/// Builds a leaf node containing a namespace.
fn build_namespace_leaf(text: &str, color: &str) -> AsciiNode {
  build_label_value_line_leaf(LABEL_NAMESPACE, text, color)
}

/// Builds a leaf with a single line containing label and coloured value.
fn build_label_value_line_leaf(label: &str, value: &str, color: &str) -> AsciiNode {
  AsciiNode::leaf_builder()
    .line(AsciiLine::builder().text(label).colon_space().with_color(value, color).build())
    .build()
}
