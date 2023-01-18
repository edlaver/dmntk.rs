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

use crate::examples::*;
use crate::{DMNTK_DESCRIPTION, DMNTK_VERSION};
use clap::{arg, command, ArgAction, ArgMatches, Command};
use difference::Changeset;
use dmntk_common::ascii_ctrl::*;
use dmntk_common::{ascii256, ascii_none, Jsonify};
use dmntk_feel::values::Value;
use dmntk_feel::FeelScope;
use dmntk_model::model::{DmnElement, NamedElement, RequiredVariable};
use dmntk_model::ModelDefinitions;

/// Available command-line actions.
enum Action {
  /// Parse `FEEL` expression.
  ParseFeelExpression(String, String),
  /// Evaluate `FEEL` expression.
  EvaluateFeelExpression(String, String),
  /// Test `FEEL` expression.
  TestFeelExpression(
    /// Test file name.
    String,
    /// FEEL file name.
    String,
    /// Flag indicating if only test summary will be printed.
    bool,
    /// Value indicating the requested color mode.
    String,
  ),
  /// Export `FEEL` expression to HTML.
  ExportFeelExpression(String, String, String),
  /// Parse decision table.
  ParseDecisionTable(String),
  /// Evaluate decision table.
  EvaluateDecisionTable(String, String),
  /// Test decision table.
  TestDecisionTable(
    /// Test file name.
    String,
    /// Decision table file name.
    String,
    /// Flag indicating if only test summary will be printed.
    bool,
    /// Value indicating the requested color mode.
    String,
  ),
  /// Export decision table.
  ExportDecisionTable(
    /// Decision table file name.
    String,
    /// HTML file name.
    String,
  ),
  /// Recognize decision table.
  RecognizeDecisionTable(String),
  /// Parse `DMN` model`.
  ParseDmnModel(String, String),
  /// Evaluate `DMN` model`.
  EvaluateDmnModel(String, String, String),
  /// Test `DMN` model`.
  TestDmnModel(
    /// Test file name.
    String,
    /// Decision table file name.
    String,
    /// Invocable name.
    String,
    /// Flag indicating if only test summary will be printed.
    bool,
    /// Value indicating the requested color mode.
    String,
  ),
  /// Export `DMN` model`.
  ExportDmnModel(String, String),
  /// Start `dmntk` as a service.
  StartService(Option<String>, Option<String>, Option<String>),
  /// Generate examples.
  GenerateExamples,
  /// Do nothing, no action was specified.
  DoNothing,
}

/// Executes command-line action.
pub async fn do_action() -> std::io::Result<()> {
  match get_cli_action() {
    Action::ParseFeelExpression(ctx_file_name, feel_file_name) => {
      parse_feel_expression(&ctx_file_name, &feel_file_name);
      Ok(())
    }
    Action::EvaluateFeelExpression(input_file_name, feel_file_name) => {
      evaluate_feel_expression(&input_file_name, &feel_file_name);
      Ok(())
    }
    Action::TestFeelExpression(test_file_name, feel_file_name, summary_only, color) => {
      test_feel_expression(&test_file_name, &feel_file_name, summary_only, &color);
      Ok(())
    }
    Action::ExportFeelExpression(input_file_name, feel_file_name, html_file_name) => {
      export_feel_expression(&input_file_name, &feel_file_name, &html_file_name);
      Ok(())
    }
    Action::ParseDecisionTable(dectab_file_name) => {
      parse_decision_table(&dectab_file_name);
      Ok(())
    }
    Action::EvaluateDecisionTable(input_file_name, dectab_file_name) => {
      evaluate_decision_table(&input_file_name, &dectab_file_name);
      Ok(())
    }
    Action::TestDecisionTable(test_file_name, dectab_file_name, summary_only, color) => {
      test_decision_table(&test_file_name, &dectab_file_name, summary_only, &color);
      Ok(())
    }
    Action::ExportDecisionTable(dectab_file_name, html_file_name) => {
      export_decision_table(&dectab_file_name, &html_file_name);
      Ok(())
    }
    Action::RecognizeDecisionTable(dectab_file_name) => {
      recognize_decision_table(&dectab_file_name);
      Ok(())
    }
    Action::ParseDmnModel(dmn_file_name, color) => {
      parse_dmn_model(&dmn_file_name, &color);
      Ok(())
    }
    Action::EvaluateDmnModel(dmn_file_name, ctx_file_name, invocable_name) => {
      evaluate_dmn_model(&dmn_file_name, &ctx_file_name, &invocable_name);
      Ok(())
    }
    Action::TestDmnModel(test_file_name, dmn_file_name, invocable_name, summary_only, color) => {
      test_dmn_model(&test_file_name, &dmn_file_name, &invocable_name, summary_only, &color);
      Ok(())
    }
    Action::ExportDmnModel(dmn_file_name, html_file_name) => {
      export_dmn_model(&dmn_file_name, &html_file_name);
      Ok(())
    }
    Action::StartService(opt_host, opt_port, opt_dir) => dmntk_server::start_server(opt_host, opt_port, opt_dir).await,
    Action::GenerateExamples => generate_examples(),
    Action::DoNothing => Ok(()),
  }
}

/// Parses CLI argument matches.
#[rustfmt::skip]
fn get_matches() -> ArgMatches {
  command!().name("dmntk").version(DMNTK_VERSION).about(DMNTK_DESCRIPTION)
    // pfe
    .subcommand(Command::new("pfe").about("Parse FEEL Expression").display_order(7)
      .arg(arg!(<CONTEXT_FILE>).help("File containing context for parsed FEEL expression").required(true).index(1))
      .arg(arg!(<FEEL_FILE>).help("File containing FEEL expression to be parsed").required(true).index(2)))
    // efe
    .subcommand(Command::new("efe").about("Evaluate FEEL Expression").display_order(4)
      .arg(arg!(<INPUT_FILE>).help("File containing input data for evaluated FEEL expression").required(true).index(1))
      .arg(arg!(<FEEL_FILE>).help("File containing FEEL expression to be evaluated").required(true).index(2)))
    // tfe
    .subcommand(Command::new("tfe").about("Test FEEL Expression").display_order(10)
      .arg(arg!(-s --summary).help("Display only summary after completing all tests").action(ArgAction::SetTrue).display_order(1))
      .arg(arg!(-c --color).help("Control when colored output is used").action(ArgAction::Set).display_order(2))
      .arg(arg!(<TEST_FILE>).help("File containing test cases for tested FEEL expression").required(true).index(1))
      .arg(arg!(<FEEL_FILE>).help("File containing FEEL expression to be tested").required(true).index(2)))
    // xfe
    .subcommand(Command::new("xfe").about("eXport FEEL Expression").display_order(13)
      .arg(arg!(<INPUT_FILE>).help("File containing input data for expression to be exported to HTML").required(true).index(1))
      .arg(arg!(<FEEL_FILE>).help("File containing FEEL expression to be exported to HTML").required(true).index(2))
      .arg(arg!(<HTML_FILE>).help("Output HTML file").required(true).index(3)))
    // pdm
    .subcommand(Command::new("pdm").about("Parse DMN Model").display_order(5)
      .arg(arg!(-c --color).help("Control when colored output is used").action(ArgAction::Set).display_order(1))
      .arg(arg!(<DMN_FILE>).help("File containing DMN model to be parsed").required(true).index(1)))
    // edm
    .subcommand(Command::new("edm").about("Evaluate DMN Model").display_order(2)
      .arg(arg!(-i --invocable).help("Name of the invocable (decision, bkm, decision service) to be evaluated").action(ArgAction::Set).required(true).display_order(1))
      .arg(arg!(<INPUT_FILE>).help("File containing input data for evaluated DMN model").required(true).index(1))
      .arg(arg!(<DMN_FILE>).help("File containing DMN model to be evaluated").required(true).index(2)))
    // tdm
    .subcommand(Command::new("tdm").about("Test DMN Model").display_order(8)
      .arg(arg!(-i --invocable).help("Name of the invocable to be tested").required(true).action(ArgAction::Set).display_order(1))
      .arg(arg!(-s --summary).help("Display only summary after completing all tests").action(ArgAction::SetTrue).display_order(2))
      .arg(arg!(-c --color).help("Control when colored output is used").action(ArgAction::Set).display_order(3))
      .arg(arg!(<TEST_FILE>).help("File containing test cases for tested DMN model").required(true).index(1))
      .arg(arg!(<DMN_FILE>).help("File containing DMN model to be tested").required(true).index(2)))
    // xdm
    .subcommand(Command::new("xdm").about("eXport DMN Model").display_order(11)
      .arg(arg!(<DMN_FILE>).help("File containing DMN model to be exported to HTML").required(true).index(1))
      .arg(arg!(<HTML_FILE>).help("Output HTML file").required(true).index(2)))
    // pdt
    .subcommand(Command::new("pdt").about("Parse Decision Table").display_order(6)
      .arg(arg!(<DECTAB_FILE>).help("File containing decision table to be parsed").required(true).index(1)))
    // edt
    .subcommand(Command::new("edt").about("Evaluate Decision Table").display_order(3)
      .arg(arg!(<INPUT_FILE>).help("File containing input data for evaluated decision table").required(true).index(1))
      .arg(arg!(<DECTAB_FILE>).help("File containing decision table to be evaluated").required(true).index(2)))
    // tdt
    .subcommand(Command::new("tdt").about("Test Decision Table").display_order(9)
      .arg(arg!(-s --summary).help("Display only summary after completing all tests").action(ArgAction::SetTrue).display_order(1))
      .arg(arg!(-c --color).help("Control when colored output is used").action(ArgAction::Set).display_order(2))
      .arg(arg!(<TEST_FILE>).help("File containing test cases for tested decision table").required(true).index(1))
      .arg(arg!(<DECTAB_FILE>).help("File containing FEEL expression to be tested").required(true).index(2)))
    // xdt
    .subcommand(Command::new("xdt").about("eXport Decision Table").display_order(12)
      .arg(arg!(<DECTAB_FILE>).help("File containing decision table to be exported to HTML").required(true).index(1))
      .arg(arg!(<HTML_FILE>).help("Output HTML file").required(true).index(2)))
    // rdt
    .subcommand(Command::new("rdt").about("Recognize Decision Table").display_order(14)
      .arg(arg!(<DECTAB_FILE>).help("File containing decision table to be recognized").required(true).index(1)))
    // srv
    .subcommand(Command::new("srv").about("Run DMNTK as a service").display_order(1)
      .arg(arg!(-H --host).help("Host name").action(ArgAction::Set).display_order(1))
      .arg(arg!(-P --port).help("Port number").action(ArgAction::Set).display_order(2))
      .arg(arg!(-D --dir).help("Directory where DMN files are searched").action(ArgAction::Set).display_order(3)))
    // exs
    .subcommand(Command::new("exs").about("Generate examples in current directory").display_order(15))
    .get_matches()
}

/// Checks the list of arguments passed from the command line
/// and returns an action related to valid argument.
fn get_cli_action() -> Action {
  let unknown_ctx = &"unknown.ctx".to_string();
  let unknown_feel = &"unknown.feel".to_string();
  let unknown_html = &"unknown.html".to_string();
  let unknown_dtb = &"unknown.dtb".to_string();
  let unknown_dmn = &"unknown.dmn".to_string();
  let unknown = &"unknown".to_string();
  let auto = &"auto".to_string();
  match get_matches().subcommand() {
    // parse FEEL expression subcommand
    Some(("pfe", matches)) => {
      return Action::ParseFeelExpression(
        matches.get_one::<String>("CONTEXT_FILE").unwrap_or(unknown_ctx).to_string(),
        matches.get_one::<String>("FEEL_FILE").unwrap_or(unknown_feel).to_string(),
      );
    }
    // evaluate FEEL expression subcommand
    Some(("efe", matches)) => {
      return Action::EvaluateFeelExpression(
        matches.get_one::<String>("INPUT_FILE").unwrap_or(unknown_ctx).to_string(),
        matches.get_one::<String>("FEEL_FILE").unwrap_or(unknown_feel).to_string(),
      );
    }
    // test FEEL expression subcommand
    Some(("tfe", matches)) => {
      return Action::TestFeelExpression(
        matches.get_one::<String>("TEST_FILE").unwrap_or(unknown_ctx).to_string(),
        matches.get_one::<String>("FEEL_FILE").unwrap_or(unknown_feel).to_string(),
        matches.get_flag("summary"),
        matches.get_one::<String>("color").unwrap_or(auto).to_string(),
      );
    }
    // export FEEL expression subcommand
    Some(("xfe", matches)) => {
      return Action::ExportFeelExpression(
        matches.get_one::<String>("INPUT_FILE").unwrap_or(unknown_ctx).to_string(),
        matches.get_one::<String>("FEEL_FILE").unwrap_or(unknown_feel).to_string(),
        matches.get_one::<String>("HTML_FILE").unwrap_or(unknown_html).to_string(),
      );
    }
    // parse decision table subcommand
    Some(("pdt", matches)) => {
      return Action::ParseDecisionTable(matches.get_one::<String>("DECTAB_FILE").unwrap_or(unknown_dtb).to_string());
    }
    // evaluate decision table subcommand
    Some(("edt", matches)) => {
      return Action::EvaluateDecisionTable(
        matches.get_one::<String>("INPUT_FILE").unwrap_or(unknown_ctx).to_string(),
        matches.get_one::<String>("DECTAB_FILE").unwrap_or(unknown_dtb).to_string(),
      );
    }
    // test decision table subcommand
    Some(("tdt", matches)) => {
      return Action::TestDecisionTable(
        matches.get_one::<String>("TEST_FILE").unwrap_or(unknown_ctx).to_string(),
        matches.get_one::<String>("DECTAB_FILE").unwrap_or(unknown_dtb).to_string(),
        matches.get_flag("summary"),
        matches.get_one::<String>("color").unwrap_or(auto).to_string(),
      );
    }
    // export decision table subcommand
    Some(("xdt", matches)) => {
      return Action::ExportDecisionTable(
        matches.get_one::<String>("DECTAB_FILE").unwrap_or(unknown_dtb).to_string(),
        matches.get_one::<String>("HTML_FILE").unwrap_or(unknown_html).to_string(),
      );
    }
    // recognize decision table subcommand
    Some(("rdt", matches)) => {
      return Action::RecognizeDecisionTable(matches.get_one::<String>("DECTAB_FILE").unwrap_or(unknown_dtb).to_string());
    }
    // parse DMN model subcommand
    Some(("pdm", matches)) => {
      return Action::ParseDmnModel(
        matches.get_one::<String>("DMN_FILE").unwrap_or(unknown_dmn).to_string(),
        matches.get_one::<String>("color").unwrap_or(auto).to_string(),
      );
    }
    // evaluate DMN model subcommand
    Some(("edm", matches)) => {
      return Action::EvaluateDmnModel(
        matches.get_one::<String>("INPUT_FILE").unwrap_or(unknown_ctx).to_string(),
        matches.get_one::<String>("DMN_FILE").unwrap_or(unknown_dmn).to_string(),
        matches.get_one::<String>("invocable").unwrap_or(unknown).to_string(),
      );
    }
    // test DMN model subcommand
    Some(("tdm", matches)) => {
      return Action::TestDmnModel(
        matches.get_one::<String>("TEST_FILE").unwrap_or(unknown_ctx).to_string(),
        matches.get_one::<String>("DMN_FILE").unwrap_or(unknown_dmn).to_string(),
        matches.get_one::<String>("invocable").unwrap_or(unknown).to_string(),
        matches.get_flag("summary"),
        matches.get_one::<String>("color").unwrap_or(auto).to_string(),
      );
    }
    // export DMN model subcommand
    Some(("xdm", matches)) => {
      return Action::ExportDmnModel(
        matches.get_one::<String>("DMN_FILE").unwrap_or(unknown_dmn).to_string(),
        matches.get_one::<String>("HTML_FILE").unwrap_or(unknown_html).to_string(),
      );
    }
    // start server subcommand
    Some(("srv", matches)) => {
      return Action::StartService(
        matches.get_one::<String>("host").map(|host| host.to_string()),
        matches.get_one::<String>("port").map(|port| port.to_string()),
        matches.get_one::<String>("dir").map(|dir| dir.to_string()),
      );
    }
    // generate examples
    Some(("exs", _)) => {
      return Action::GenerateExamples;
    }
    _ => {}
  }
  println!("dmntk {DMNTK_VERSION}");
  println!("{DMNTK_DESCRIPTION}");
  println!("dmntk: missing subcommand");
  println!("Try 'dmntk --help' for more information.");
  Action::DoNothing
}

/// Parses `FEEL` expression loaded from file and prints the parsed `AST` to standard output.
fn parse_feel_expression(ctx_file_name: &str, feel_file_name: &str) {
  match std::fs::read_to_string(feel_file_name) {
    Ok(feel_expression) => match std::fs::read_to_string(ctx_file_name) {
      Ok(context_definition) => match dmntk_evaluator::evaluate_context(&FeelScope::default(), &context_definition) {
        Ok(ctx) => match dmntk_feel_parser::parse_expression(&ctx.into(), &feel_expression, false) {
          Ok(ast_root_node) => {
            println!("    AST:{}", ast_root_node.to_string().trim_end());
          }
          Err(reason) => eprintln!("parsing expression failed with reason: {reason}"),
        },
        Err(reason) => eprintln!("evaluating context failed with reason: {reason}"),
      },
      Err(reason) => eprintln!("loading context file `{ctx_file_name}` failed with reason: {reason}"),
    },
    Err(reason) => eprintln!("loading expression file `{feel_file_name}` failed with reason: {reason}"),
  }
}

/// Evaluates `FEEL` expression loaded from file and prints the result to standard output.
fn evaluate_feel_expression(ctx_file_name: &str, feel_file_name: &str) {
  match std::fs::read_to_string(feel_file_name) {
    Ok(textual_expression) => match std::fs::read_to_string(ctx_file_name) {
      Ok(context_definition) => match dmntk_evaluator::evaluate_context(&FeelScope::default(), &context_definition) {
        Ok(ctx) => match dmntk_feel_parser::parse_expression(&ctx.clone().into(), &textual_expression, false) {
          Ok(ast_root_node) => match dmntk_evaluator::evaluate(&ctx.into(), &ast_root_node) {
            Ok(result) => {
              println!("{result}");
            }
            Err(reason) => eprintln!("evaluating expression failed with reason: {reason}"),
          },
          Err(reason) => eprintln!("parsing expression failed with reason: {reason}"),
        },
        Err(reason) => eprintln!("evaluating context failed with reason: {reason}"),
      },
      Err(reason) => eprintln!("loading context file `{ctx_file_name}` failed with reason: {reason}"),
    },
    Err(reason) => eprintln!("loading expression file `{feel_file_name}` failed with reason: {reason}"),
  }
}

/// Tests `FEEL` expression loaded from file and prints the test result to standard output.
fn test_feel_expression(test_file_name: &str, feel_file_name: &str, summary_only: bool, color: &str) {
  match std::fs::read_to_string(feel_file_name) {
    Ok(feel_file_content) => match std::fs::read_to_string(test_file_name) {
      Ok(test_file_content) => match dmntk_evaluator::evaluate_test_cases(&test_file_content) {
        Ok(test_cases) => {
          let mut passed = 0_usize;
          let mut failed = 0_usize;
          for (test_no, (input_data, expected)) in test_cases.iter().enumerate() {
            let scope = input_data.clone().into();
            match dmntk_feel_parser::parse_expression(&scope, &feel_file_content, false) {
              Ok(node) => match dmntk_evaluator::evaluate(&scope, &node) {
                Ok(actual) => display_test_case_result(&actual, expected, &test_no, &mut passed, &mut failed, summary_only, color),
                Err(reason) => eprintln!("evaluating expression failed with reason: {reason}"),
              },
              Err(reason) => eprintln!("parsing expression failed with reason: {reason}"),
            }
          }
          display_test_summary(passed, failed, summary_only, color);
        }
        Err(reason) => eprintln!("evaluation of test cases failed with reason: {reason}"),
      },
      Err(reason) => eprintln!("loading test file `{test_file_name}` failed with reason: {reason}"),
    },
    Err(reason) => eprintln!("loading expression file `{feel_file_name}` failed with reason: {reason}"),
  }
}

/// Exports `FEEL` expression loaded from file to HTML output file.
fn export_feel_expression(_input_file_name: &str, _feel_file_name: &str, html_file_name: &str) {
  let _ = std::fs::write(html_file_name, "not implemented\n");
}

/// Parses decision table loaded from text file.
fn parse_decision_table(dectab_file_name: &str) {
  match std::fs::read_to_string(dectab_file_name) {
    Ok(text) => match dmntk_recognizer::scan(&text) {
      Ok(mut canvas) => {
        canvas.display_text_layer();
        canvas.display_thin_layer();
        canvas.display_body_layer();
        canvas.display_grid_layer();
        match canvas.plane() {
          Ok(plane) => println!("PLANE\n{plane}"),
          Err(reason) => eprintln!("ERROR: {reason}"),
        };
      }
      Err(reason) => eprintln!("ERROR: {reason}"),
    },
    Err(reason) => eprintln!("loading decision table file `{dectab_file_name}` failed with reason: {reason}"),
  }
}

/// Evaluates context and decision table loaded from files.
fn evaluate_decision_table(input_file_name: &str, dectab_file_name: &str) {
  let input_file_content = match std::fs::read_to_string(input_file_name) {
    Ok(input_file_content) => input_file_content,
    Err(reason) => {
      eprintln!("loading input file `{input_file_name}` failed with reason: {reason}");
      return;
    }
  };
  let input_data = match dmntk_evaluator::evaluate_context(&FeelScope::default(), &input_file_content) {
    Ok(input_data) => input_data,
    Err(reason) => {
      eprintln!("evaluating input data failed with reason: {reason}");
      return;
    }
  };
  let dtb_file_content = match std::fs::read_to_string(dectab_file_name) {
    Ok(dtb_file_content) => dtb_file_content,
    Err(reason) => {
      eprintln!("loading input file `{dectab_file_name}` failed with reason: {reason}");
      return;
    }
  };
  let decision_table = match dmntk_recognizer::build(&dtb_file_content) {
    Ok(decision_table) => decision_table,
    Err(reason) => {
      eprintln!("building decision table failed with reason: {reason}");
      return;
    }
  };
  let scope = input_data.into();
  let evaluator = match dmntk_evaluator::build_decision_table_evaluator(&scope, &decision_table) {
    Ok(evaluator) => evaluator,
    Err(reason) => {
      eprintln!("building decision table evaluator failed with reason: {reason}");
      return;
    }
  };
  let result = evaluator(&scope) as Value;
  println!("{}", result.jsonify());
}

/// Tests decision table loaded from file.
fn test_decision_table(test_file_name: &str, dectab_file_name: &str, summary_only: bool, color: &str) {
  let dtb_file_content = match std::fs::read_to_string(dectab_file_name) {
    Ok(dtb_file_content) => dtb_file_content,
    Err(reason) => {
      eprintln!("loading decision table file `{dectab_file_name}` failed with reason: {reason}");
      return;
    }
  };
  let decision_table = match dmntk_recognizer::build(&dtb_file_content) {
    Ok(decision_table) => decision_table,
    Err(reason) => {
      eprintln!("building decision table failed with reason: {reason}");
      return;
    }
  };
  let test_file_content = match std::fs::read_to_string(test_file_name) {
    Ok(test_file_content) => test_file_content,
    Err(reason) => {
      eprintln!("loading test file `{test_file_name}` failed with reason: {reason}");
      return;
    }
  };
  let test_cases = match dmntk_evaluator::evaluate_test_cases(&test_file_content) {
    Ok(test_cases) => test_cases,
    Err(reason) => {
      eprintln!("evaluating test file failed with reason: {reason}");
      return;
    }
  };
  let mut passed = 0_usize;
  let mut failed = 0_usize;
  for (test_no, (input_data, expected)) in test_cases.iter().enumerate() {
    let scope = input_data.clone().into();
    let evaluator = match dmntk_evaluator::build_decision_table_evaluator(&scope, &decision_table) {
      Ok(evaluator) => evaluator,
      Err(reason) => {
        eprintln!("building decision table evaluator failed with reason: {reason}");
        return;
      }
    };
    let actual = evaluator(&scope) as Value;
    display_test_case_result(&actual, expected, &test_no, &mut passed, &mut failed, summary_only, color);
  }
  display_test_summary(passed, failed, summary_only, color);
}

/// Exports decision table loaded from text file to HTML output file.
fn export_decision_table(dectab_file_name: &str, html_file_name: &str) {
  match std::fs::read_to_string(dectab_file_name) {
    Ok(text) => match dmntk_recognizer::build(&text) {
      Ok(decision_table) => {
        let output = dmntk_gendoc::decision_table_to_html(&decision_table);
        if let Err(reason) = std::fs::write(html_file_name, output) {
          println!("writing output HTML file `{html_file_name}` failed with reason: {reason}")
        }
      }
      Err(reason) => eprintln!("ERROR: {reason}"),
    },
    Err(reason) => eprintln!("loading decision table file `{dectab_file_name}` failed with reason: {reason}"),
  }
}

/// Recognizes the decision table loaded from text file.
fn recognize_decision_table(dtb_file_name: &str) {
  match std::fs::read_to_string(dtb_file_name) {
    Ok(ref text) => match dmntk_recognizer::Recognizer::recognize(text) {
      Ok(recognizer) => {
        recognizer.trace();
      }
      Err(reason) => eprintln!("ERROR: {reason}"),
    },
    Err(reason) => eprintln!("loading decision table file `{dtb_file_name}` failed with reason: {reason}"),
  }
}

/// Parses `DMN` model loaded from XML file.
fn parse_dmn_model(dmn_file_name: &str, color: &str) {
  let use_color = color.to_lowercase() != "never";
  let color_a = if use_color { ascii256!(255) } else { ascii_none!() };
  let color_b = if use_color { ascii256!(82) } else { ascii_none!() };
  let color_c = if use_color { ascii256!(184) } else { ascii_none!() };
  let color_d = if use_color { ascii256!(208) } else { ascii_none!() };
  let color_e = if use_color { ASCII_RED } else { "" };
  let color_r = if use_color { ASCII_RESET } else { "" };
  let none = "(none)".to_string();
  match std::fs::read_to_string(dmn_file_name) {
    Ok(dmn_file_content) => match &dmntk_model::parse(&dmn_file_content) {
      Ok(definitions) => {
        let model_definitions: ModelDefinitions = definitions.into();
        println!("\n{color_a}Model{color_r}");
        println!("{color_a} ├─ name:{color_b} {}{color_r}", definitions.name());
        println!("{color_a} ├─ namespace: {}{color_b}{color_r}", definitions.namespace());
        println!("{color_a} └─ id:{color_b} {}{color_r}", definitions.id().as_ref().unwrap_or(&none));
        // definitions
        if model_definitions.decisions().is_empty() {
          println!("\n{color_a}Decisions{color_c} {none}{color_r}");
        } else {
          println!("\n{color_a}Decisions{color_r}");
          let decision_count = model_definitions.decisions().len();
          for (i, decision) in model_definitions.decisions().iter().enumerate() {
            if i < decision_count - 1 {
              print!(" {color_a}├─{color_r}");
            } else {
              print!(" {color_a}└─{color_r}");
            }
            println!(" {color_c}{}{color_r}", decision.name());
          }
        }
        // item data
        if model_definitions.input_data().is_empty() {
          println!("\n{color_a}Input data{color_c} {none}{color_r}");
        } else {
          println!("\n{color_a}Input data{color_r}");
          let input_data_count = model_definitions.input_data().len();
          for (i, input_data) in model_definitions.input_data().iter().enumerate() {
            if i < input_data_count - 1 {
              print!(" {color_a}├─{color_r}");
            } else {
              print!(" {color_a}└─{color_r}");
            }
            println!(
              " {}{} ({}){}",
              color_d,
              input_data.name(),
              input_data.variable().type_ref().as_ref().unwrap_or(&none),
              color_r
            );
          }
        }
        // more...
        print!("\n{color_e}MORE DETAILS WILL BE IMPLEMENTED...{color_r}\n\n");
      }
      Err(reason) => eprintln!("parsing model file failed with reason: {reason}"),
    },
    Err(reason) => eprintln!("loading model file `{dmn_file_name}` failed with reason: {reason:?}"),
  }
}

/// Evaluates `DMN` model loaded from XML file.
fn evaluate_dmn_model(input_file_name: &str, dmn_file_name: &str, invocable_name: &str) {
  match std::fs::read_to_string(dmn_file_name) {
    Ok(dmn_file_content) => match std::fs::read_to_string(input_file_name) {
      Ok(input_file_content) => match dmntk_evaluator::evaluate_context(&FeelScope::default(), &input_file_content) {
        Ok(input_data) => match dmntk_model::parse(&dmn_file_content) {
          Ok(definitions) => match dmntk_evaluator::ModelEvaluator::new(&definitions) {
            Ok(model_evaluator) => {
              let result = model_evaluator.evaluate_invocable(invocable_name, &input_data);
              println!("{}", result.jsonify())
            }
            Err(reason) => eprintln!("evaluating invocable {invocable_name} failed with reason: {reason}"),
          },
          Err(reason) => eprintln!("parsing model failed with reason: {reason}"),
        },
        Err(reason) => eprintln!("evaluating input data failed with reason: {reason}"),
      },
      Err(reason) => eprintln!("loading input data file `{input_file_name}` failed with reason: {reason}"),
    },
    Err(reason) => eprintln!("loading model file `{dmn_file_name}` failed with reason: {reason}"),
  }
}

/// Tests `DMN` model loaded from XML file.
fn test_dmn_model(test_file_name: &str, dmn_file_name: &str, invocable_name: &str, summary_only: bool, color: &str) {
  let dmn_file_content = match std::fs::read_to_string(dmn_file_name) {
    Ok(dmn_file_content) => dmn_file_content,
    Err(reason) => {
      eprintln!("loading model file `{dmn_file_name}` failed with reason: {reason}");
      return;
    }
  };
  let definitions = match dmntk_model::parse(&dmn_file_content) {
    Ok(definitions) => definitions,
    Err(reason) => {
      eprintln!("parsing model file failed with reason: {reason}");
      return;
    }
  };
  let model_evaluator = match dmntk_evaluator::ModelEvaluator::new(&definitions) {
    Ok(model_evaluator) => model_evaluator,
    Err(reason) => {
      eprintln!("preparing model evaluator failed with reason: {reason}");
      return;
    }
  };
  let test_file_content = match std::fs::read_to_string(test_file_name) {
    Ok(test_file_content) => test_file_content,
    Err(reason) => {
      eprintln!("loading test file `{test_file_name}` failed with reason: {reason}");
      return;
    }
  };
  let test_cases = match dmntk_evaluator::evaluate_test_cases(&test_file_content) {
    Ok(test_cases) => test_cases,
    Err(reason) => {
      eprintln!("evaluating test file failed with reason: {reason}");
      return;
    }
  };
  let mut passed = 0_usize;
  let mut failed = 0_usize;
  for (test_no, (input_data, expected)) in test_cases.iter().enumerate() {
    let actual = model_evaluator.evaluate_invocable(invocable_name, input_data);
    display_test_case_result(&actual, expected, &test_no, &mut passed, &mut failed, summary_only, color);
  }
  display_test_summary(passed, failed, summary_only, color);
}

/// Exports `DMN` model loaded from XML file to HTML output file.
fn export_dmn_model(_dmn_file_name: &str, html_file_name: &str) {
  let _ = std::fs::write(html_file_name, "not implemented\n");
}

/// Generates examples in current directory.
fn generate_examples() -> std::io::Result<()> {
  let create_dir = |path| -> std::io::Result<()> {
    std::fs::create_dir_all(path)?;
    Ok(())
  };
  let write_file = |path, contents| -> std::io::Result<()> {
    std::fs::write(path, contents)?;
    Ok(())
  };
  create_dir("examples")?;
  create_dir("examples/e1")?;
  write_file("./examples/e1/e1.ctx", E1_CTX)?;
  write_file("./examples/e1/e1.feel", E1_FEEL)?;
  create_dir("examples/e2")?;
  write_file("./examples/e2/e2.ctx", E2_CTX)?;
  write_file("./examples/e2/e2.dmn", E2_DMN)?;
  create_dir("examples/e3")?;
  write_file("./examples/e3/e3.ctx", E3_CTX)?;
  write_file("./examples/e3/e3.dtb", E3_DTB)?;
  Ok(())
}

/// Utility function for displaying test case result.
fn display_test_case_result(actual: &Value, expected: &Value, test_no: &usize, passed: &mut usize, failed: &mut usize, summary_only: bool, color: &str) {
  let use_color = color.to_lowercase() != "never";
  let color_red = if use_color { ASCII_RED } else { "" };
  let color_green = if use_color { ASCII_GREEN } else { "" };
  let color_magenta = if use_color { ASCII_MAGENTA } else { "" };
  let color_reset = if use_color { ASCII_RESET } else { "" };
  if dmntk_evaluator::evaluate_equals(actual, expected) {
    *passed += 1;
    if !summary_only {
      println!("test {} ... {}ok{}", test_no + 1, color_green, color_reset);
    }
  } else {
    *failed += 1;
    if !summary_only {
      println!("test {} ... {color_red}FAILED{color_reset}", test_no + 1);
      println!("    {color_green}expected{color_reset}: {expected}");
      println!("      {color_red}actual{color_reset}: {actual}");
      if use_color {
        // showing the difference is reasonable only with colors enabled
        println!(
          "  {}difference{}: {}",
          color_magenta,
          color_reset,
          Changeset::new(&expected.jsonify(), &actual.jsonify(), "")
        );
      }
    }
  }
}

/// Utility function for displaying test summary.
fn display_test_summary(passed: usize, failed: usize, summary_only: bool, color: &str) {
  let use_color = color.to_lowercase() != "never";
  let color_red = if use_color { ASCII_RED } else { "" };
  let color_green = if use_color { ASCII_GREEN } else { "" };
  let color_reset = if use_color { ASCII_RESET } else { "" };
  if failed > 0 {
    if summary_only {
      println!("test result: {color_red}FAILED{color_reset}. {passed} passed; {failed} failed.\n");
    } else {
      println!("\ntest result: {color_red}FAILED{color_reset}. {passed} passed; {failed} failed.\n");
    }
  } else if summary_only {
    println!("test result: {color_green}ok{color_reset}. {passed} passed; {failed} failed.\n");
  } else {
    println!("\ntest result: {color_green}ok{color_reset}. {passed} passed; {failed} failed.\n");
  }
}
