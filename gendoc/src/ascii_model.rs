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
use dmntk_model::model::{Definitions, DmnElement, NamedElement, RequiredVariable};
use std::fmt::Write;

const LABEL_DECISIONS: &str = "Decisions";
const LABEL_ID: &str = "id";
const LABEL_INPUT_DATA: &str = "Input data";
const LABEL_MODEL: &str = "Model";
const LABEL_NAME: &str = "name";
const LABEL_NAMESPACE: &str = "namespace";
const LABEL_NONE: &str = "(none)";

pub fn print_model(definitions: &Definitions, color_mode: ColorMode) {
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

  // node: model root for definitions with top level properties
  let node_model_root = AsciiNode::node_builder(AsciiLine::builder().text(LABEL_MODEL).build())
    .child(node_definitions_name)
    .child(node_definitions_namespace)
    .child(node_definitions_id)
    .build();

  // node: decisions root
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

  // node: input data root
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
