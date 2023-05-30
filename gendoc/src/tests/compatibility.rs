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

//! # Test fro generating HTML export files from DMN models.

use super::*;

macro_rules! test_export_model {
  ($test_name:tt,$model_name:tt) => {
    #[test]
    fn $test_name() {
      export_model!($model_name);
    }
  };
}

test_export_model!(_2_0001, DMN_2_0001);
test_export_model!(_2_0002, DMN_2_0002);
test_export_model!(_2_0003, DMN_2_0003);
test_export_model!(_2_0004, DMN_2_0004);
test_export_model!(_2_0005, DMN_2_0005);
test_export_model!(_2_0006, DMN_2_0006);
test_export_model!(_2_0007, DMN_2_0007);
test_export_model!(_2_0008, DMN_2_0008);
test_export_model!(_2_0009, DMN_2_0009);
test_export_model!(_2_0010, DMN_2_0010);
test_export_model!(_2_0100, DMN_2_0100);
test_export_model!(_2_0101, DMN_2_0101);
test_export_model!(_2_0102, DMN_2_0102);
test_export_model!(_2_0105, DMN_2_0105);
test_export_model!(_2_0106, DMN_2_0106);
test_export_model!(_2_0107, DMN_2_0107);
test_export_model!(_2_0108, DMN_2_0108);
test_export_model!(_2_0109, DMN_2_0109);
test_export_model!(_2_0110, DMN_2_0110);
test_export_model!(_2_0111, DMN_2_0111);
test_export_model!(_2_0112, DMN_2_0112);
test_export_model!(_2_0113, DMN_2_0113);
test_export_model!(_2_0114, DMN_2_0114);
test_export_model!(_2_0115, DMN_2_0115);
test_export_model!(_2_0116, DMN_2_0116);
test_export_model!(_2_0117, DMN_2_0117);
test_export_model!(_2_0118, DMN_2_0118);
test_export_model!(_2_0119, DMN_2_0119);

test_export_model!(_3_0001, DMN_3_0001);
test_export_model!(_3_0002, DMN_3_0002);
test_export_model!(_3_0003, DMN_3_0003);
test_export_model!(_3_0004, DMN_3_0004);
test_export_model!(_3_0005, DMN_3_0005);
test_export_model!(_3_0006, DMN_3_0006);
test_export_model!(_3_0007, DMN_3_0007);
test_export_model!(_3_0008, DMN_3_0008);
test_export_model!(_3_0009, DMN_3_0009);
test_export_model!(_3_0010, DMN_3_0010);
test_export_model!(_3_0011, DMN_3_0011);
test_export_model!(_3_0012, DMN_3_0012);
test_export_model!(_3_0013, DMN_3_0013);
test_export_model!(_3_0014, DMN_3_0014);
test_export_model!(_3_0016, DMN_3_0016);
test_export_model!(_3_0017, DMN_3_0017);
test_export_model!(_3_0020, DMN_3_0020);
test_export_model!(_3_0021, DMN_3_0021);
test_export_model!(_3_0030, DMN_3_0030);
test_export_model!(_3_0031, DMN_3_0031);
test_export_model!(_3_0032, DMN_3_0032);
test_export_model!(_3_0033, DMN_3_0033);
test_export_model!(_3_0034, DMN_3_0034);
test_export_model!(_3_0035, DMN_3_0035);
test_export_model!(_3_0036, DMN_3_0036);
test_export_model!(_3_0037, DMN_3_0037);
test_export_model!(_3_0038, DMN_3_0038);
test_export_model!(_3_0039, DMN_3_0039);
test_export_model!(_3_0040, DMN_3_0040);
test_export_model!(_3_0041, DMN_3_0041);
test_export_model!(_3_0050, DMN_3_0050);
test_export_model!(_3_0051, DMN_3_0051);
test_export_model!(_3_0052, DMN_3_0052);
test_export_model!(_3_0053, DMN_3_0053);
test_export_model!(_3_0054, DMN_3_0054);
test_export_model!(_3_0055, DMN_3_0055);
test_export_model!(_3_0056, DMN_3_0056);
test_export_model!(_3_0057, DMN_3_0057);
test_export_model!(_3_0058, DMN_3_0058);
test_export_model!(_3_0059, DMN_3_0059);
test_export_model!(_3_0060, DMN_3_0060);
test_export_model!(_3_0061, DMN_3_0061);
test_export_model!(_3_0062, DMN_3_0062);
test_export_model!(_3_0063, DMN_3_0063);
test_export_model!(_3_0064, DMN_3_0064);
test_export_model!(_3_0065, DMN_3_0065);
test_export_model!(_3_0066, DMN_3_0066);
test_export_model!(_3_0067, DMN_3_0067);
test_export_model!(_3_0068, DMN_3_0068);
test_export_model!(_3_0069, DMN_3_0069);
test_export_model!(_3_0070, DMN_3_0070);
test_export_model!(_3_0071, DMN_3_0071);
test_export_model!(_3_0072, DMN_3_0072);
test_export_model!(_3_0073, DMN_3_0073);
test_export_model!(_3_0074, DMN_3_0074);
test_export_model!(_3_0075, DMN_3_0075);
test_export_model!(_3_0076, DMN_3_0076);
test_export_model!(_3_0077, DMN_3_0077);
test_export_model!(_3_0078, DMN_3_0078);
test_export_model!(_3_0080, DMN_3_0080);
test_export_model!(_3_0081, DMN_3_0081);
test_export_model!(_3_0082, DMN_3_0082);
test_export_model!(_3_0083, DMN_3_0083);
test_export_model!(_3_0084, DMN_3_0084);
test_export_model!(_3_0085, DMN_3_0085);
test_export_model!(_3_0086, DMN_3_0086);
test_export_model!(_3_0086_import, DMN_3_0086_IMPORT);
test_export_model!(_3_0087, DMN_3_0087);
test_export_model!(_3_0088, DMN_3_0088);
test_export_model!(_3_0089, DMN_3_0089);
test_export_model!(_3_0089_model_a, DMN_3_0089_MODEL_A);
test_export_model!(_3_0089_model_b, DMN_3_0089_MODEL_B);
test_export_model!(_3_0089_model_b2, DMN_3_0089_MODEL_B2);
test_export_model!(_3_0090, DMN_3_0090);
test_export_model!(_3_0092, DMN_3_0092);
test_export_model!(_3_0100, DMN_3_0100);
test_export_model!(_3_1100, DMN_3_1100);
test_export_model!(_3_1101, DMN_3_1101);
test_export_model!(_3_1102, DMN_3_1102);
test_export_model!(_3_1103, DMN_3_1103);
test_export_model!(_3_1104, DMN_3_1104);
test_export_model!(_3_1105, DMN_3_1105);
test_export_model!(_3_1106, DMN_3_1106);
test_export_model!(_3_1107, DMN_3_1107);
test_export_model!(_3_1108, DMN_3_1108);
test_export_model!(_3_1109, DMN_3_1109);
test_export_model!(_3_1110, DMN_3_1110);
test_export_model!(_3_1115, DMN_3_1115);
test_export_model!(_3_1116, DMN_3_1116);
test_export_model!(_3_1117, DMN_3_1117);
test_export_model!(_3_1120, DMN_3_1120);
test_export_model!(_3_1121, DMN_3_1121);
test_export_model!(_3_1130, DMN_3_1130);
test_export_model!(_3_2891, DMN_3_2891);
test_export_model!(_3_2892, DMN_3_2892);
test_export_model!(_3_2893, DMN_3_2893);
test_export_model!(_3_2894, DMN_3_2894);
test_export_model!(_3_2895, DMN_3_2895);
