use std::fs::read_dir;

use colored::Colorize;
use swc_common::{BytePos, comments::SingleThreadedComments};
use swc_experimental_ecma_ast::Program;
use swc_experimental_ecma_parser::{EsSyntax, Lexer, Parser, StringInput, Syntax};

use crate::{
    AppArgs,
    suite::{Case, TestResult},
    util::crate_root,
};

const TEST_PATH: &str = "fixtures/parser-misc";

pub struct MiscParserSuite {
    cases: Vec<Case>,
}

impl MiscParserSuite {
    pub fn new() -> Self {
        let test_path = crate_root().join(TEST_PATH);
        let pass_cases = read_dir(test_path.join("pass")).unwrap();
        let fail_cases = read_dir(test_path.join("fail")).unwrap();

        let mut cases = Vec::new();
        for pass_case in pass_cases {
            let pass_case = pass_case.unwrap();
            let path = pass_case.path();
            let code = std::fs::read_to_string(&path).unwrap();
            cases.push(Case {
                path,
                code,
                should_fail: false,
            });
        }

        for fail_case in fail_cases {
            let fail_case = fail_case.unwrap();
            let path = fail_case.path();
            let code = std::fs::read_to_string(&path).unwrap();
            cases.push(Case {
                path,
                code,
                should_fail: true,
            });
        }

        Self { cases }
    }

    pub fn run(&mut self, args: &AppArgs) -> Vec<TestResult> {
        let mut results = Vec::with_capacity(self.cases.len());
        for case in self.cases.iter_mut() {
            if args.debug {
                println!("[{}] {:?}", "Debug".green(), case.relative_path());
            }

            let ext = case.path.extension().unwrap().to_string_lossy().to_string();
            let syntax = match ext.as_str() {
                "js" | "cjs" | "mjs" | "jsx" => {
                    let mut es = EsSyntax::default();
                    if ext == "jsx" {
                        es.jsx = true;
                    }
                    if case.code.contains("// @decorators") {
                        es.decorators = true;
                    }
                    if case.code.contains("// @decorators_before_export") {
                        es.decorators_before_export = true;
                    }
                    if case.code.contains("// @export_default_from") {
                        es.export_default_from = true;
                    }
                    if case.code.contains("// @allow_super_outside_method") {
                        es.allow_super_outside_method = true;
                    }
                    Syntax::Es(es)
                }
                "ts" | "cts" | "mts" | "tsx" => Syntax::Es(EsSyntax::default()),
                _ => unreachable!(),
            };

            let input = StringInput::new(&case.code, BytePos(0), BytePos(case.code.len() as u32));
            let comments = SingleThreadedComments::default();
            let lexer = Lexer::new(syntax, Default::default(), input, Some(&comments));
            let mut parser = Parser::new_from(lexer);
            let ret = match ext.as_str() {
                "js" => parser.parse_program(),
                "cjs" => parser.parse_script().map(Program::Script),
                "mjs" => parser.parse_module().map(Program::Module),
                "ts" | "jsx" | "tsx" => {
                    results.push(TestResult::Ignored {
                        path: case.path.clone(),
                    });
                    continue;
                }
                _ => unreachable!(),
            };

            let errors = parser.take_errors();
            let failed = ret.is_err() || !errors.is_empty();
            match (case.should_fail, failed) {
                (true, false) => results.push(TestResult::Failed {
                    path: case.path.clone(),
                    error: "Expected failure, but parsed successfully".to_string(),
                }),
                (true, true) => results.push(TestResult::Passed {
                    path: case.path.clone(),
                }),
                (false, false) => results.push(TestResult::Passed {
                    path: case.path.clone(),
                }),
                (false, true) => results.push(TestResult::Failed {
                    path: case.path.clone(),
                    error: format!("{:?}", errors),
                }),
            }
        }
        results
    }
}
