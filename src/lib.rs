mod commands;
mod error;
mod interpreter;
mod problem;
mod report;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser); // synthesized by LALRPOP

use error::InterpreterError;
use interpreter::Interpreter;
use report::Report;
use std::io;

pub enum RunResult {
    Report(Report),
    ParseError {
        line_num: u32,
        line: String,
    },
    RuntimeError {
        line_num: u32,
        error: InterpreterError,
    },
    ReportError(String),
}

pub fn run<T: io::BufRead>(input: T) -> RunResult {
    let mut exec = Interpreter::default();

    for (line, line_num) in input.lines().zip(1..) {
        let line = line.unwrap();
        if let Ok(cmd) = parser::FullCommandParser::new().parse(&line) {
            if let Err(error) = exec.next_command(cmd) {
                return RunResult::RuntimeError { line_num, error };
            }
        } else {
            return RunResult::ParseError { line_num, line };
        }
    }
    match exec.report() {
        Ok(report) => RunResult::Report(report),
        Err(msg) => RunResult::ReportError(msg.to_owned()),
    }
}
