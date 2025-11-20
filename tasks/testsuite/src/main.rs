use crate::{misc::parser::MiscParserSuite, suite::TestResult, test262_parser::Test262ParserSuite};
use colored::*;
use pico_args::Arguments;

mod misc;
mod suite;
mod test262_parser;
mod util;

pub struct AppArgs {
    pub debug: bool,
}

pub fn main() {
    let mut args = Arguments::from_env();
    let args = AppArgs {
        debug: args.contains("--debug"),
    };

    let mut results = Vec::new();
    results.extend(MiscParserSuite::new().run(&args));
    results.extend(Test262ParserSuite::new().run(&args));

    let mut passed = 0;
    let mut failed = 0;
    let mut ignored = 0;

    for result in results {
        match result {
            TestResult::Passed { .. } => {
                passed += 1;
            }
            TestResult::Failed { path, error } => {
                failed += 1;
                println!("Failed: {} - {}", path.display(), error.red());
            }
            TestResult::Ignored { .. } => {
                ignored += 1;
            }
        }
    }

    println!("\n{}", "Test Summary:".bold());
    println!("Passed: {}", passed.to_string().green());
    println!("Failed: {}", failed.to_string().red());
    println!("Ignored: {}", ignored.to_string().yellow());

    if failed > 0 {
        std::process::exit(1);
    }
}
