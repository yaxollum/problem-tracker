mod commands;
mod error;
mod interpreter;
mod problem;
mod report;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser); // synthesized by LALRPOP

use interpreter::Interpreter;
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut exec = Interpreter::default();

    for (line, line_num) in stdin.lock().lines().zip(1..) {
        let line = line.unwrap();
        if let Ok(cmd) = parser::FullCommandParser::new().parse(&line) {
            if let Err(msg) = exec.next_command(cmd) {
                eprintln!("Line {}: {}", line_num, msg);
                std::process::exit(1);
            }
        } else {
            eprintln!("Could not parse line {}: \"{}\"", line_num, line);
            std::process::exit(1);
        }
    }
    match exec.report() {
        Ok(report) => println!("{}", report),
        Err(msg) => {
            eprintln!("{}", msg);
            std::process::exit(1);
        }
    }
}
