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

use crate::Name;
use dmntk_common::Jsonify;
use std::collections::HashMap;

#[test]
fn test_default_name() {
  let name: Name = Default::default();
  assert_eq!("", name.to_string());
  name.assert_receiver_is_total_eq();
}

#[test]
fn test_display() {
  let name: Name = vec!["   x   ".to_string(), " y      \t".to_string(), "  \n  z  \t  ".to_string()].into();
  assert_eq!("x y z", format!("{name}"));
}

#[test]
fn test_debug() {
  let name: Name = vec!["   x   ".to_string(), " y      \t".to_string(), "  \n  z  \t  ".to_string()].into();
  assert_eq!(r#"Name("x y z")"#, format!("{name:?}"));
}

#[test]
fn test_to_string() {
  let name: Name = vec!["a".to_string(), "+".to_string(), "b".to_string()].into();
  let s: String = name.into();
  assert_eq!("a+b", s);
}

#[test]
fn test_jsonify() {
  let name: Name = vec!["a".to_string(), "+".to_string(), "b".to_string()].into();
  let s: String = name.jsonify();
  assert_eq!("a+b", s);
}

#[test]
fn test_from_string_vector() {
  let name: Name = vec!["".to_string(), "".to_string(), "".to_string()].into();
  assert_eq!("", name.to_string());
  let name: Name = vec!["x".to_string(), "y".to_string()].into();
  assert_eq!("x y", name.to_string());
  let name: Name = vec!["x".to_string(), "+".to_string(), "y".to_string()].into();
  assert_eq!("x+y", name.to_string());
  let name: Name = vec!["x".to_string(), "    +    ".to_string(), "y".to_string()].into();
  assert_eq!("x+y", name.to_string());
  let name: Name = vec!["a".to_string(), "b".to_string(), "c".to_string()].into();
  assert_eq!("a b c", name.to_string());
  let name: Name = vec!["   x   ".to_string(), " y      \t".to_string(), "  \n  z  \t  ".to_string()].into();
  assert_eq!("x y z", name.to_string());
}

#[test]
fn test_from_str_vector() {
  let name: Name = vec!["", "", ""].into();
  assert_eq!("", name.to_string());
  let name: Name = vec!["x", "y"].into();
  assert_eq!("x y", name.to_string());
  let name: Name = vec!["x", "+", "y"].into();
  assert_eq!("x+y", name.to_string());
  let name: Name = vec!["a", "b", "c"].into();
  assert_eq!("a b c", name.to_string());
  let name: Name = vec!["   x   ", " y      \t", "  \n  z  \t  "].into();
  assert_eq!("x y z", name.to_string());
}

#[test]
fn additional_symbols() {
  let name: Name = vec!["x", "y"].into();
  assert_eq!("x y", name.to_string());
  let name: Name = vec!["x", ".", "y"].into();
  assert_eq!("x.y", name.to_string());
  let name: Name = vec!["x", "   .    ", "y"].into();
  assert_eq!("x.y", name.to_string());
  let name: Name = vec![".", "x", "y"].into();
  assert_eq!(".x y", name.to_string());
  let name: Name = vec!["x", "y", "."].into();
  assert_eq!("x y.", name.to_string());
  let name: Name = vec!["x", "/", "y"].into();
  assert_eq!("x/y", name.to_string());
  let name: Name = vec!["x", "-", "y"].into();
  assert_eq!("x-y", name.to_string());
  let name: Name = vec!["x", "'", "y"].into();
  assert_eq!("x'y", name.to_string());
  let name: Name = vec!["x", "+", "y"].into();
  assert_eq!("x+y", name.to_string());
  let name: Name = vec!["x", "*", "y"].into();
  assert_eq!("x*y", name.to_string());
}

#[test]
fn test_name_as_hash_map_key() {
  let name_a: Name = "a".into();
  let name_b: Name = "b".into();
  let name_c: Name = "c".into();
  let mut map = HashMap::new();
  map.insert(name_a.clone(), "A".to_string());
  map.insert(name_b.clone(), "B".to_string());
  map.insert(name_c.clone(), "C".to_string());
  assert_eq!(3, map.len());
  assert!(map.contains_key(&name_a));
  assert_eq!("A", map.get(&name_a).unwrap());
  assert!(map.contains_key(&name_b));
  assert_eq!("B", map.get(&name_b).unwrap());
  assert!(map.contains_key(&name_c));
  assert_eq!("C", map.get(&name_c).unwrap());
  assert!(!map.contains_key(&"d".into()));
}

#[test]
#[allow(clippy::nonminimal_bool)]
fn test_equality() {
  let name_a: Name = "a".into();
  let name_b: Name = "b".into();
  let name_x_y: Name = vec!["x", "y"].into();
  let name_m_n: Name = vec!["m", "n"].into();
  let name_xy: Name = vec!["x y"].into();
  let name_mn: Name = vec!["m n"].into();
  assert_eq!("a", name_a.to_string());
  assert_eq!("b", name_b.to_string());
  assert_eq!("x y", name_x_y.to_string());
  assert_eq!("m n", name_m_n.to_string());
  assert_eq!("x y", name_xy.to_string());
  assert_eq!("m n", name_m_n.to_string());
  assert!((name_a == name_a));
  assert!((name_x_y == name_x_y));
  assert!((name_x_y == name_xy));
  assert!((name_m_n == name_m_n));
  assert!((name_m_n == name_mn));
  assert!((name_a != name_b));
  assert!((name_x_y != name_m_n));
  assert!((name_xy != name_mn));
  assert!((name_xy != name_a));
  assert!((name_x_y != name_a));
  assert!((name_mn != name_b));
  assert!(!(name_a != name_a));
  assert!(!(name_x_y != name_x_y));
  assert!(!(name_x_y != name_xy));
  assert!(!(name_m_n != name_m_n));
  assert!(!(name_m_n != name_mn));
  assert!(!(name_a == name_b));
  assert!(!(name_x_y == name_m_n));
  assert!(!(name_xy == name_mn));
  assert!(!(name_x_y == name_a));
  assert!(!(name_xy == name_a));
  assert!(!(name_m_n == name_b));
  assert!((name_m_n != name_b));
  assert!(!(name_mn == name_b));
}

#[test]
fn test_compare() {
  let name_a: Name = "a".into();
  let name_b: Name = "b".into();
  let name_xy: Name = vec!["x y"].into();
  let name_mn: Name = vec!["m n"].into();
  assert_eq!("a", name_a.to_string());
  assert_eq!("b", name_b.to_string());
  assert_eq!("x y", name_xy.to_string());
  assert_eq!("m n", name_mn.to_string());
  assert!((name_a <= name_a));
  assert!((name_a < name_b));
  assert!((name_xy > name_mn));
  assert!((name_xy >= name_a));
}
