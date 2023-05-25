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

use dmntk_common::{color_256, write_ascii_tree, AsciiLine, AsciiNode, ColorMode};
use dmntk_model::model::*;
use std::fmt::Write;

const LABEL_DECISIONS: &str = "Decisions";
const LABEL_ID: &str = "id";
const LABEL_INPUT_DATA: &str = "Input data";
const LABEL_MODEL: &str = "Model";
const LABEL_NAME: &str = "name";
const LABEL_NAMESPACE: &str = "namespace";
const LABEL_NONE: &str = "(none)";
const LABEL_PERFORMANCE_INDICATORS: &str = "Performance indicators";

pub fn print_model(definitions: &Definitions, color_mode: ColorMode) {
  // color for unique identifiers
  let color_id = color_256!(color_mode, 82);
  let color_a = color_256!(color_mode, 82);
  let color_b = color_256!(color_mode, 184);
  let color_c = color_256!(color_mode, 208);

  // definitions.name
  let node_definitions_name = AsciiNode::leaf_builder()
    .line(AsciiLine::builder().text(LABEL_NAME).colon_space().with_color(definitions.name(), &color_a).build())
    .build();

  // definitions.namespace
  let node_definitions_namespace = AsciiNode::leaf_builder()
    .line(
      AsciiLine::builder()
        .text(LABEL_NAMESPACE)
        .colon_space()
        .with_color(definitions.namespace(), &color_a)
        .build(),
    )
    .build();

  // definitions.id
  let definitions_id = definitions.id().as_ref().cloned().unwrap_or(LABEL_NONE.to_string());
  let node_definitions_id = AsciiNode::leaf_builder()
    .line(AsciiLine::builder().text(LABEL_ID).colon_space().with_color(&definitions_id, &color_a).build())
    .build();

  //-----------------------------------------------------------------------------------------------
  // node: model root for definitions with top level properties
  //-----------------------------------------------------------------------------------------------
  let mut node_model_root_builder = AsciiNode::node_builder(AsciiLine::builder().text(LABEL_MODEL).build());
  node_model_root_builder.add_child(node_definitions_name);
  node_model_root_builder.add_child(node_definitions_namespace);
  node_model_root_builder.add_child(node_definitions_id);
  if let Some(performance_indicators_node) = build_performance_indicators_node(definitions, &color_id) {
    node_model_root_builder.add_child(performance_indicators_node)
  }
  let node_model_root = node_model_root_builder.build();

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
      decision_node_builder.add_child(AsciiNode::leaf_builder().line(AsciiLine::builder().with_color(decision.name(), &color_b).build()).build());
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
              .with_color(input_data.name(), &color_c)
              .space()
              .l_paren()
              .with_color(&input_data_type, &color_c)
              .r_paren()
              .build(),
          )
          .build(),
      );
    }
    input_data_node_builder.build()
  };

  // display the report
  let mut output = String::new();
  let _ = write_ascii_tree(&mut output, &node_model_root, &color_mode);
  let _ = writeln!(&mut output);
  let _ = write_ascii_tree(&mut output, &node_decisions_root, &color_mode);
  let _ = writeln!(&mut output);
  let _ = write_ascii_tree(&mut output, &node_input_data_root, &color_mode);
  println!("{}", output);
}

/// Builds a tree node containing performance indicators.
fn build_performance_indicators_node(definitions: &Definitions, color_id: &str) -> Option<AsciiNode> {
  let performance_indicators: Vec<&PerformanceIndicator> = definitions
    .business_context_elements()
    .iter()
    .filter_map(|item| match item {
      BusinessContextElementInstance::PerformanceIndicator(performance_indicator) => Some(performance_indicator),
      _ => None,
    })
    .collect();
  if !performance_indicators.is_empty() {
    let mut performance_indicators_node_builder = AsciiNode::node_builder(AsciiLine::builder().text(LABEL_PERFORMANCE_INDICATORS).build());
    for performance_indicator in performance_indicators {
      let name = performance_indicator.name();
      let mut performance_indicator_node_builder = AsciiNode::node_builder(AsciiLine::builder().text(name).build());

      if let Some(description) = performance_indicator.description() {
        performance_indicator_node_builder.add_child(build_description_leaf(description));
      }

      if let Some(id) = performance_indicator.id() {
        performance_indicator_node_builder.add_child(build_id_leaf(id, color_id));
      }

      let performance_indicator_node = performance_indicator_node_builder.build();
      performance_indicators_node_builder.add_child(performance_indicator_node);
    }
    let performance_indicators_node = performance_indicators_node_builder.build();
    return Some(performance_indicators_node);
  }
  None
}

/// Builds a leaf node containing the value of the identifier.
fn build_id_leaf(text: &str, color: &str) -> AsciiNode {
  AsciiNode::leaf_builder()
    .line(AsciiLine::builder().text(LABEL_ID).colon_space().with_color(text, color).build())
    .build()
}

/// Builds a leaf node containing description.
fn build_description_leaf(text: &str) -> AsciiNode {
  let mut leaf_builder = AsciiNode::leaf_builder();
  for line in text.lines().map(|s| s.trim()).filter(|s| !s.is_empty()) {
    leaf_builder.add_line(AsciiLine::builder().text(line).build());
  }
  leaf_builder.build()
}
