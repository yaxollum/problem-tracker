use problem_tracker::{run, RunResult};
use std::io;

fn main() {
    let stdin = io::stdin();
    match run(stdin.lock()) {
        RunResult::Report(report) => println!("{}", report),
        RunResult::ParseError { line_num, line } => {
            eprintln!("Could not parse line {}: \"{}\"", line_num, line);
            std::process::exit(1);
        }
        RunResult::RuntimeError { line_num, error } => {
            eprintln!("Line {}: {}", line_num, error);
            std::process::exit(1);
        }
        RunResult::ReportError(msg) => {
            eprintln!("{}", msg);
            std::process::exit(1);
        }
    }
}
