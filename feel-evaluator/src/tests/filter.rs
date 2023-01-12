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

use super::*;

#[test]
fn _0001() {
  let scope = &te_scope(r#"{A:[1,3,5]}"#);
  te_null(false, scope, "A[0]", "index in filter is out of range [1..3], actual index is 0");
  te_null(false, scope, "A[4]", "index in filter is out of range [1..3], actual index is 4");
  te_null(false, scope, "A[-4]", "index in filter is out of range [-3..-1], actual index is -4");
}

#[test]
fn _0002() {
  let scope = &te_scope(r#"{A:[1,3,5]}"#);
  te_number(false, scope, "A[1]", 1, 0);
  te_number(false, scope, "A[-3]", 1, 0);
}

#[test]
fn _0003() {
  let scope = &te_scope(r#"{A:[1,3,5]}"#);
  te_number(false, scope, "A[2]", 3, 0);
  te_number(false, scope, "A[-2]", 3, 0);
}

#[test]
fn _0004() {
  let scope = &te_scope(r#"{A:[1,3,5]}"#);
  te_number(false, scope, "A[3]", 5, 0);
  te_number(false, scope, "A[-1]", 5, 0);
}

#[test]
fn _0005() {
  let scope = &te_scope(r#"{B:["A","B","C","D","E"]}"#);
  te_null(false, scope, "B[0]", "index in filter is out of range [1..5], actual index is 0");
  te_null(false, scope, "B[6]", "index in filter is out of range [1..5], actual index is 6");
  te_null(false, scope, "B[-6]", "index in filter is out of range [-5..-1], actual index is -6");
}

#[test]
fn _0006() {
  let scope = &te_scope(r#"{B:["A","B","C","D","E"]}"#);
  te_string(false, scope, "B[1]", "A");
  te_string(false, scope, "B[-5]", "A");
}

#[test]
fn _0007() {
  let scope = &te_scope(r#"{B:["A","B","C","D","E"]}"#);
  te_string(false, scope, "B[2]", "B");
  te_string(false, scope, "B[-4]", "B");
}

#[test]
fn _0008() {
  let scope = &te_scope(r#"{B:["A","B","C","D","E"]}"#);
  te_string(false, scope, "B[3]", "C");
  te_string(false, scope, "B[-3]", "C");
}

#[test]
fn _0009() {
  let scope = &te_scope(r#"{B:["A","B","C","D","E"]}"#);
  te_string(false, scope, "B[4]", "D");
  te_string(false, scope, "B[-2]", "D");
}

#[test]
fn _0010() {
  let scope = &te_scope(r#"{B:["A","B","C","D","E"]}"#);
  te_string(false, scope, "B[5]", "E");
  te_string(false, scope, "B[-1]", "E");
}

#[test]
fn _0011() {
  let scope = &te_scope(r#"{A:[1,3,5],B:["A","B","C","D","E"]}"#);
  te_string(false, scope, "B[A[1]]", "A");
}

#[test]
fn _0012() {
  let scope = &te_scope(r#"{A:[1,3,5],B:["A","B","C","D","E"]}"#);
  te_string(false, scope, "B[A[2]]", "C");
}

#[test]
fn _0013() {
  let scope = &te_scope(r#"{A:[1,3,5],B:["A","B","C","D","E"]}"#);
  te_string(false, scope, "B[A[3]]", "E");
}

#[test]
fn _0014() {
  let scope = &te_scope(r#"{l:[{x:1,y:2},{x:null,y:3}]}"#);
  te_be_value(false, scope, "l[x<2]", "{x:1,y:2}");
}

#[test]
fn _0015() {
  let scope = &te_scope(r#"{}"#);
  te_be_value(false, scope, "[{x:1,y:2},{x:null,y:3}][x<2]", "{x:1,y:2}");
}

#[test]
fn _0016() {
  let scope = &te_scope(r#"{}"#);
  te_be_value(false, scope, "[{x:1,y:2},{x:null,y:3}][y>2]", "{x:null,y:3}");
}

#[test]
fn _0017() {
  let scope = &te_scope(r#"{}"#);
  te_be_value(false, scope, "[1,2,3,4,5,6,7,8,9,10][item>=5]", "[5,6,7,8,9,10]");
}

#[test]
fn _0018() {
  let scope = &te_scope(r#"{l:[{x:1,y:2},{x:2,y:4},{x:3,y:6}]}"#);
  te_be_value(false, scope, "l[2]", "{x:2,y:4}");
}

#[test]
fn _0019() {
  let scope = &te_scope(r#"{l:[{x:1,y:2},{x:2,y:4},{x:3,y:6}]}"#);
  te_be_value(false, scope, "l[x=2]", "{x:2,y:4}");
}

#[test]
fn _0020() {
  let scope = &te_scope(r#"{l:[{x:1,y:2},{x:2,y:4},{x:3,y:6}]}"#);
  te_value(false, scope, "l[x=2].y", "4");
}

#[test]
fn _0021() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "[1,2,3,4,5,6][1]", 1, 0);
}

#[test]
fn _0022() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "[1,2,3,4,5,6][item=4]", 4, 0);
}

#[test]
fn _0023() {
  let scope = &te_scope(r#"{}"#);
  te_be_value(false, scope, "[1,2,3,4,5,6][item>=4]", "[4,5,6]");
}

#[test]
fn _0024() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "[1,2,3,4,5,6][1][1]", 1, 0);
}

#[test]
fn _0025() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "[1,2,3,4,5,6][2]", 2, 0);
}

#[test]
fn _0026() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "[1,2,3,4,5,6][3]", 3, 0);
}

#[test]
fn _0027() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "[1,2,3,4,5,6][4]", 4, 0);
}

#[test]
fn _0028() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "[1,2,3,4,5,6][5]", 5, 0);
}

#[test]
fn _0029() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "[1,2,3,4,5,6][6]", 6, 0);
}

#[test]
fn _0030() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "[1,2,3,4,5,6][-1]", 6, 0);
}

#[test]
fn _0031() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "[1,2,3,4,5,6][-2]", 5, 0);
}

#[test]
fn _0032() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "[1,2,3,4,5,6][-3]", 4, 0);
}

#[test]
fn _0033() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "[1,2,3,4,5,6][-4]", 3, 0);
}

#[test]
fn _0034() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "[1,2,3,4,5,6][-5]", 2, 0);
}

#[test]
fn _0035() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "[1,2,3,4,5,6][-6]", 1, 0);
}

#[test]
fn _0036() {
  let scope = &te_scope(r#"{}"#);
  te_null(false, scope, "[1,2,3,4,5,6][0]", "index in filter is out of range [1..6], actual index is 0");
}

#[test]
fn _0037() {
  let scope = &te_scope(r#"{}"#);
  te_null(false, scope, "[1,2,3,4,5,6][7]", "index in filter is out of range [1..6], actual index is 7");
}

#[test]
fn _0038() {
  let scope = &te_scope(r#"{}"#);
  te_null(false, scope, "[1,2,3,4,5,6][-7]", "index in filter is out of range [-6..-1], actual index is -7");
}

#[test]
fn _0039() {
  let scope = &te_scope(r#"{}"#);
  te_be_value(false, scope, r#"true[true]"#, r#"[true]"#);
}

#[test]
fn _0040() {
  let scope = &te_scope(r#"{}"#);
  be_be_value(false, scope, r#"[{a: 1}, {a: 2}, {a: 3}]"#, r#"[{a:1},{a:2},{a:3}]"#);
}

#[test]
fn _0041() {
  let scope = &te_scope(r#"{}"#);
  te_be_value(false, scope, r#"[{a: 1}, {a: 2}, {a: 3}][1]"#, r#"{a:1}"#);
}

#[test]
fn _0042() {
  let scope = &te_scope(r#"{}"#);
  te_be_value(false, scope, r#"[{item: 1}, {item: 2}, {item: 3}][item >= 2]"#, r#"[{item:2},{item:3}]"#);
}

#[test]
fn _0043() {
  let scope = &te_scope(r#"{}"#);
  te_be_value(false, scope, r#"[{a: 1}, {a: 2}, {a: 3}][item.a=2]"#, r#"{a:2}"#);
}

#[test]
fn _0044() {
  let scope = &te_scope(r#"{}"#);
  te_be_value(false, scope, r#"[{a: 1}, {a: 2}, {a: 3}][item.a >= 2]"#, r#"[{a:2},{a:3}]"#);
}

#[test]
fn _0045() {
  let scope = &te_scope(
    r#"{
    DeptTable:    [ {manager:"Smith",name:"Sales",number:10},
                    {manager:"Jones",name:"Finance",number:20},
                    {manager:"King",name:"Engineering",number:30} ],
    EmployeeTable:[ {deptNum:10,id:"7792",name:"Clark"},
                    {deptNum:10,id:"7934",name:"Miller"},
                    {deptNum:20,id:"7976",name:"Adams"},
                    {deptNum:20,id:"7902",name:"Ford"},
                    {deptNum:30,id:"7900",name:"James"} ],
    LastName:"Clark"}"#,
  );
  te_string(false, scope, "DeptTable[number=EmployeeTable[name=LastName].deptNum[1]].manager[1]", "Smith");
}

#[test]
fn _0046() {
  let scope = &te_scope(
    r#"
    {
       EmployeeTable:[ 
         {deptNum:10,id:"7792",name:"Clark"},
         {deptNum:10,id:"7934",name:"Miller"},
         {deptNum:20,id:"7976",name:"Adams"},
         {deptNum:20,id:"7902",name:"Ford"},
         {deptNum:30,id:"7900",name:"James"} 
       ],
       LastName:"Clark"
    }
    "#,
  );
  te_number(false, scope, "EmployeeTable[name=LastName].deptNum[1]", 10, 0);
}

#[test]
fn _0047() {
  let scope = &te_scope(
    r#"
    {
       EmployeeTable:[ 
         {deptNum:10,id:"7792",name:"Clark"},
         {deptNum:10,id:"7934",name:"Miller"},
         {deptNum:20,id:"7976",name:"Adams"},
         {deptNum:20,id:"7902",name:"Ford"},
         {deptNum:30,id:"7900",name:"James"} 
       ],
       LastName:"Clark"
    }
    "#,
  );
  te_number(false, scope, "EmployeeTable[name=LastName].deptNum[1]", 10, 0);
}

#[test]
fn _0048() {
  let scope = &te_scope(
    r#"{
    DeptTable:    [ {manager:"Smith",name:"Sales",number:10},
                    {manager:"Jones",name:"Finance",number:20},
                    {manager:"King",name:"Engineering",number:30} ],
    EmployeeTable:[ {deptNum:10,id:"7792",name:"Clark"},
                    {deptNum:10,id:"7934",name:"Miller"},
                    {deptNum:20,id:"7976",name:"Adams"},
                    {deptNum:20,id:"7902",name:"Ford"},
                    {deptNum:30,id:"7900",name:"James"} ],
    LastName:"Clark"}"#,
  );
  te_string(false, scope, "DeptTable[number=10].manager[1]", "Smith");
}

#[test]
fn _0049() {
  let scope = &te_scope(
    r#"{
    DeptTable:    [ {manager:"Smith",name:"Sales",number:10},
                    {manager:"Jones",name:"Finance",number:20},
                    {manager:"King",name:"Engineering",number:30} ],
    EmployeeTable:[ {deptNum:10,id:"7792",name:"Clark"},
                    {deptNum:10,id:"7934",name:"Miller"},
                    {deptNum:20,id:"7976",name:"Adams"},
                    {deptNum:20,id:"7902",name:"Ford"},
                    {deptNum:30,id:"7900",name:"James"} ],
    LastName:"Clark"}"#,
  );
  te_be_value(false, scope, "DeptTable[number=10]", r#"{manager:"Smith",name:"Sales",number:10}"#);
}

#[test]
fn _0050() {
  let scope = &te_scope(
    r#"{
     check_eq: function(x,y) x = y,
     check_neq: function(x,y) x != y,
     num: 1 
   }"#,
  );
  te_number(false, scope, "[1,2,3,4,5,6][check_eq(item,4)]", 4, 0);
  te_number(false, scope, "[1,2,3,4,5,6][check_eq(item,5)]", 5, 0);
  te_number(false, scope, "[1,2,3,4,5,6][check_eq(item,num)]", 1, 0);
  te_be_value(false, scope, "[1,2,3,4,5,6][check_neq(item,num)]", "[2,3,4,5,6]");
}

#[test]
fn _0051() {
  let scope = &te_scope(
    r#"{
      "Flight List": [
        { "Flight Number": "UA456",  "From": "SFO", "To": "SNA", "Departure": @"2017-01-01T19:00:00", "Arrival": @"2017-01-01T20:00:00", "Capacity": 2, "Status": "scheduled" },
        { "Flight Number": "UA123",  "From": "SFO", "To": "SNA", "Departure": @"2017-01-01T18:00:00", "Arrival": @"2017-01-01T19:00:00", "Capacity": 5, "Status": "cancelled" },
        { "Flight Number": "UA789",  "From": "SFO", "To": "SNA", "Departure": @"2017-01-01T21:00:00", "Arrival": @"2017-01-01T23:00:00", "Capacity": 2, "Status": "scheduled" },
        { "Flight Number": "UA1001", "From": "SFO", "To": "SNA", "Departure": @"2017-01-01T23:00:00", "Arrival": @"2017-01-02T05:00:00", "Capacity": 0, "Status": "scheduled" },
        { "Flight Number": "UA1111", "From": "SFO", "To": "LAX", "Departure": @"2017-01-01T23:00:00", "Arrival": @"2017-01-02T05:00:00", "Capacity": 2, "Status": "scheduled" }]
    }"#,
  );
  te_be_value(
    false,
    scope,
    "Flight List[1]",
    r#"{"Arrival": @"2017-01-01T20:00:00", "Capacity": 2, "Departure": @"2017-01-01T19:00:00", "Flight Number": "UA456", "From": "SFO", "Status": "scheduled", "To": "SNA"}"#,
  );
}

#[test]
fn _0052() {
  let scope = &te_scope(
    r#"{
      "Flight List": [
        { "Flight Number": "UA456",  "From": "SFO", "To": "SNA", "Departure": @"2017-01-01T19:00:00", "Arrival": @"2017-01-01T20:00:00", "Capacity": 2, "Status": "scheduled" },
        { "Flight Number": "UA123",  "From": "SFO", "To": "SNA", "Departure": @"2017-01-01T18:00:00", "Arrival": @"2017-01-01T19:00:00", "Capacity": 5, "Status": "cancelled" },
        { "Flight Number": "UA789",  "From": "SFO", "To": "SNA", "Departure": @"2017-01-01T21:00:00", "Arrival": @"2017-01-01T23:00:00", "Capacity": 2, "Status": "scheduled" },
        { "Flight Number": "UA1001", "From": "SFO", "To": "SNA", "Departure": @"2017-01-01T23:00:00", "Arrival": @"2017-01-02T05:00:00", "Capacity": 0, "Status": "scheduled" },
        { "Flight Number": "UA1111", "From": "SFO", "To": "LAX", "Departure": @"2017-01-01T23:00:00", "Arrival": @"2017-01-02T05:00:00", "Capacity": 2, "Status": "scheduled" }]
    }"#,
  );
  te_be_value(
    false,
    scope,
    r#"Flight List[Status = "cancelled"]"#,
    r#"{"Arrival": @"2017-01-01T19:00:00", "Capacity": 5, "Departure": @"2017-01-01T18:00:00", "Flight Number": "UA123", "From": "SFO", "Status": "cancelled", "To": "SNA"}"#,
  );
  te_be_value(
    false,
    scope,
    r#"Flight List[Status = "cancelled"][1]"#,
    r#"{"Arrival": @"2017-01-01T19:00:00", "Capacity": 5, "Departure": @"2017-01-01T18:00:00", "Flight Number": "UA123", "From": "SFO", "Status": "cancelled", "To": "SNA"}"#,
  );
}
