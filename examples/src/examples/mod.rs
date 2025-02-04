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

//! Examples of decision tables in text form.
//!
//! Decision tables defined in this module are used to test
//! the functionality of **decision tables** crate.
//!
//! ### Estimation of the number of all possible decision table types
//!
//! ```ORI 3``` *ORIENTATION*            **horizontal**, **vertical**, **crosstab**
//!
//! ```ITN 2``` *INFORMATION-ITEM-NAME*  **present**, **not present**
//!
//! ```OLB 2``` *OUTPUT-LABEL*           **present**, **not present**
//!
//! ```IOV 2``` *INPUT-OUTPUT-VALUES*    **present**, **not present**
//!
//! ```OUT 2``` *OUTPUTS*                **single**, **multiple**
//!
//! ```ANN 2``` *ANNOTATIONS*            **present**, **not present**
//!
//! ```text
//! ┌─────┬─────┬─────┬─────┬─────┬─────┬────────────────────┬─────────┐
//! │ ORI │ ITN │ OLB │ IOV │ OUT │ ANN │      Remarks       │ Example │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ NO  │ NO  │ NO  │ SGL │ NO  │                    │ EX_0001 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ NO  │ NO  │ NO  │ SGL │ YES │                    │ EX_0002 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ NO  │ NO  │ NO  │ MUL │ NO  │                    │ EX_0003 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ NO  │ NO  │ NO  │ MUL │ YES │                    │ EX_0004 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ NO  │ NO  │ YES │ SGL │ NO  │                    │ EX_0005 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ NO  │ NO  │ YES │ SGL │ YES │                    │ EX_0006 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ NO  │ NO  │ YES │ MUL │ NO  │                    │ EX_0007 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ NO  │ NO  │ YES │ MUL │ YES │                    │ EX_0008 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ NO  │ YES │ NO  │ SGL │ NO  │                    │ EX_0009 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ NO  │ YES │ NO  │ SGL │ YES │                    │ EX_0010 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ NO  │ YES │ NO  │ MUL │ NO  │                    │ EX_0011 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ NO  │ YES │ NO  │ MUL │ YES │                    │ EX_0012 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ NO  │ YES │ YES │ SGL │ NO  │                    │ EX_0013 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ NO  │ YES │ YES │ SGL │ YES │                    │ EX_0014 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ NO  │ YES │ YES │ MUL │ NO  │                    │ EX_0015 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ NO  │ YES │ YES │ MUL │ YES │                    │ EX_0016 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ YES │ NO  │ NO  │ SGL │ NO  │                    │ EX_0017 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ YES │ NO  │ NO  │ SGL │ YES │                    │ EX_0018 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ YES │ NO  │ NO  │ MUL │ NO  │                    │ EX_0019 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ YES │ NO  │ NO  │ MUL │ YES │                    │ EX_0020 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ YES │ NO  │ YES │ SGL │ NO  │                    │ EX_0021 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ YES │ NO  │ YES │ SGL │ YES │                    │ EX_0022 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ YES │ NO  │ YES │ MUL │ NO  │                    │ EX_0023 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ YES │ NO  │ YES │ MUL │ YES │                    │ EX_0024 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ YES │ YES │ NO  │ SGL │ NO  │                    │ EX_0025 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ YES │ YES │ NO  │ SGL │ YES │                    │ EX_0026 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ YES │ YES │ NO  │ MUL │ NO  │                    │ EX_0027 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ YES │ YES │ NO  │ MUL │ YES │                    │ EX_0028 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ YES │ YES │ YES │ SGL │ NO  │                    │ EX_0029 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ YES │ YES │ YES │ SGL │ YES │                    │ EX_0030 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ YES │ YES │ YES │ MUL │ NO  │                    │ EX_0031 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ HT  │ YES │ YES │ YES │ MUL │ YES │                    │ EX_0032 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ NO  │ NO  │ NO  │ SGL │ NO  │                    │ EX_0033 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ NO  │ NO  │ NO  │ SGL │ YES │                    │ EX_0034 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ NO  │ NO  │ NO  │ MUL │ NO  │                    │ EX_0035 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ NO  │ NO  │ NO  │ MUL │ YES │                    │ EX_0036 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ NO  │ NO  │ YES │ SGL │ NO  │                    │ EX_0037 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ NO  │ NO  │ YES │ SGL │ YES │                    │ EX_0038 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ NO  │ NO  │ YES │ MUL │ NO  │                    │ EX_0039 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ NO  │ NO  │ YES │ MUL │ YES │                    │ EX_0040 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ NO  │ YES │ NO  │ SGL │ NO  │                    │ EX_0041 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ NO  │ YES │ NO  │ SGL │ YES │                    │ EX_0042 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ NO  │ YES │ NO  │ MUL │ NO  │                    │ EX_0043 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ NO  │ YES │ NO  │ MUL │ YES │                    │ EX_0044 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ NO  │ YES │ YES │ SGL │ NO  │                    │ EX_0045 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ NO  │ YES │ YES │ SGL │ YES │                    │ EX_0046 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ NO  │ YES │ YES │ MUL │ NO  │                    │ EX_0047 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ NO  │ YES │ YES │ MUL │ YES │                    │ EX_0048 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ YES │ NO  │ NO  │ SGL │ NO  │                    │ EX_0049 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ YES │ NO  │ NO  │ SGL │ YES │                    │ EX_0050 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ YES │ NO  │ NO  │ MUL │ NO  │                    │ EX_0051 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ YES │ NO  │ NO  │ MUL │ YES │                    │ EX_0052 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ YES │ NO  │ YES │ SGL │ NO  │                    │ EX_0053 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ YES │ NO  │ YES │ SGL │ YES │                    │ EX_0054 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ YES │ NO  │ YES │ MUL │ NO  │                    │ EX_0055 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ YES │ NO  │ YES │ MUL │ YES │                    │ EX_0056 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ YES │ YES │ NO  │ SGL │ NO  │                    │ EX_0057 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ YES │ YES │ NO  │ SGL │ YES │                    │ EX_0058 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ YES │ YES │ NO  │ MUL │ NO  │                    │ EX_0059 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ YES │ YES │ NO  │ MUL │ YES │                    │ EX_0060 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ YES │ YES │ YES │ SGL │ NO  │                    │ EX_0061 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ YES │ YES │ YES │ SGL │ YES │                    │ EX_0062 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ YES │ YES │ YES │ MUL │ NO  │                    │ EX_0063 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ VT  │ YES │ YES │ YES │ MUL │ YES │                    │ EX_0064 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ NO  │ NO  │ NO  │ SGL │ NO  │                    │ EX_0065 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ NO  │ NO  │ NO  │ SGL │ YES │ postponed          │ EX_0066 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ NO  │ NO  │ NO  │ MUL │ NO  │                    │ EX_0067 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ NO  │ NO  │ NO  │ MUL │ YES │ postponed          │ EX_0068 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ NO  │ NO  │ YES │ SGL │ NO  │                    │ EX_0069 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ NO  │ NO  │ YES │ SGL │ YES │ postponed          │ EX_0070 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ NO  │ NO  │ YES │ MUL │ NO  │                    │ EX_0071 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ NO  │ NO  │ YES │ MUL │ YES │ postponed          │ EX_0072 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ NO  │ YES │ NO  │ SGL │ NO  │                    │ EX_0073 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ NO  │ YES │ NO  │ SGL │ YES │ postponed          │ EX_0074 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ NO  │ YES │ NO  │ MUL │ NO  │                    │ EX_0075 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ NO  │ YES │ NO  │ MUL │ YES │ postponed          │ EX_0076 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ NO  │ YES │ YES │ SGL │ NO  │                    │ EX_0077 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ NO  │ YES │ YES │ SGL │ YES │ postponed          │ EX_0078 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ NO  │ YES │ YES │ MUL │ NO  │                    │ EX_0079 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ NO  │ YES │ YES │ MUL │ YES │ postponed          │ EX_0080 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ YES │ NO  │ NO  │ SGL │ NO  │                    │ EX_0081 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ YES │ NO  │ NO  │ SGL │ YES │ postponed          │ EX_0082 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ YES │ NO  │ NO  │ MUL │ NO  │                    │ EX_0083 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ YES │ NO  │ NO  │ MUL │ YES │ postponed          │ EX_0084 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ YES │ NO  │ YES │ SGL │ NO  │                    │ EX_0085 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ YES │ NO  │ YES │ SGL │ YES │ postponed          │ EX_0086 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ YES │ NO  │ YES │ MUL │ NO  │                    │ EX_0087 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ YES │ NO  │ YES │ MUL │ YES │ postponed          │ EX_0088 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ YES │ YES │ NO  │ SGL │ NO  │                    │ EX_0089 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ YES │ YES │ NO  │ SGL │ YES │ postponed          │ EX_0090 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ YES │ YES │ NO  │ MUL │ NO  │                    │ EX_0091 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ YES │ YES │ NO  │ MUL │ YES │ postponed          │ EX_0092 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ YES │ YES │ YES │ SGL │ NO  │                    │ EX_0093 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ YES │ YES │ YES │ SGL │ YES │ postponed          │ EX_0094 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ YES │ YES │ YES │ MUL │ NO  │                    │ EX_0095 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ YES │ YES │ YES │ MUL │ YES │ postponed          │ EX_0096 │
//! ├─────┼─────┼─────┼─────┼─────┼─────┼────────────────────┼─────────┤
//! │ CT  │ YES │ YES │ NO  │ SGL │ NO  │ Three inputs.      │ EX_0097 │
//! └─────┴─────┴─────┴─────┴─────┴─────┴────────────────────┴─────────┘
//! ```

pub mod valid;

pub const EXAMPLE_0001_DTB: &str = r#"
  ┌───┬──────────┬───────╥──────┐
  │ U │ Customer │ Order ║      │
  ╞═══╪══════════╪═══════╬══════╡
  │ 1 │ Business │  <10  ║ 0.10 │
  ├───┼──────────┼───────╫──────┤
  │ 2 │ Business │ >=10  ║ 0.15 │
  ├───┼──────────┼───────╫──────┤
  │ 3 │ Private  │   -   ║ 0.05 │
  └───┴──────────┴───────╨──────┘
"#;

pub const EXAMPLE_0001_CTX: &str = r#"
{
  Customer: "Business",
     Order:  10.00
}
"#;

pub const EXAMPLE_0002_DTB: &str = r#"
  ┌───────────────────────╥───────────────┬──────────┬───────────────┐
  │ Applicant age         ║     <25       │ [25..60] │      >60      │
  ├───────────────────────╫──────┬────────┼──────────┼────────┬──────┤
  │ Medical history       ║ good │  bad   │     -    │  good  │ bad  │
  ╞═══════════════════════╬══════╪════════╪══════════╪════════╪══════╡
  │ Applicant risk rating ║ Low  │ Medium │  Medium  │ Medium │ High │
  ├───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │ Special Discount      ║  10  │    7   │     6    │    4   │  0   │
  ├───────────────────────╫──────┼────────┼──────────┼────────┼──────┤
  │           U           ║  1   │    2   │     3    │   4    │   5  │
  └───────────────────────╨──────┴────────┴──────────┴────────┴──────┘
% { Applicant age: 20, Medical history: "good" }, {Applicant risk rating: "Low",    Special Discount: 10}
% { Applicant age: 24, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 7}
% { Applicant age: 25, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 6}
% { Applicant age: 25, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 6}
% { Applicant age: 60, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 6}
% { Applicant age: 60, Medical history: "bad"  }, {Applicant risk rating: "Medium", Special Discount: 6}
% { Applicant age: 61, Medical history: "good" }, {Applicant risk rating: "Medium", Special Discount: 4}
% { Applicant age: 61, Medical history: "bad"  }, {Applicant risk rating: "High",   Special Discount: 0}
% { Applicant age: 61, Medical history: "well" }, null
"#;

pub const EXAMPLE_0002_CTX: &str = r#"
{
    Applicant age: 24,
  Medical history: "good"
}
"#;
