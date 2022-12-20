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
 * you may not use this file except in compatibility with the License.
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

//! Test for converting decision tables defined as text into HTML format.

use super::*;
use dmntk_examples::decision_tables::*;

/// Utility function for generating HTML file for decision table defined as text.
fn generate_html(dec_tab: &str, output_file_name: &str) {
  let decision_table = dmntk_recognizer::build(dec_tab).expect("building decision table failed");
  let html = crate::decision_table_to_html(&decision_table);
  assert_eq!("<!DOCTYPE html>", &html[0..15]);
  std::fs::create_dir_all(TARGET_DIR).expect("creating target directories failed");
  let mut file = File::create(format!("{TARGET_DIR}/{output_file_name}.html")).expect("creating HTML file failed");
  file.write_all(html.as_bytes()).expect("saving HTML file failed");
}

/// Utility macro for generating HTML file for decision table defined as text.
macro_rules! generate_html {
  ($t:tt) => {{
    generate_html($t, stringify!($t));
  }};
}

#[test]
fn _0001() {
  generate_html!(H_000010);
}

#[test]
fn _0002() {
  generate_html!(H_110010);
}

#[test]
fn _0003() {
  generate_html!(H_000210);
}

#[test]
fn _0004() {
  generate_html!(H_010010);
}

#[test]
fn _0005() {
  generate_html!(H_010210);
}

#[test]
fn _0006() {
  generate_html!(H_011222);
}

#[test]
fn _0007() {
  generate_html!(H_110010);
}
