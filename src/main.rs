mod commands;
mod interpreter;
mod problem;
mod report;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser); // synthesized by LALRPOP

use commands::Command;
use interpreter::Interpreter;
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut exec = Interpreter::default();

    let mut terminated = false;
    for (line, line_num) in stdin.lock().lines().zip(1..) {
        let line = line.unwrap();
        if let Ok(cmd) = parser::FullCommandParser::new().parse(&line) {
            if let Err(msg) = exec.next_command(cmd) {
                println!("Line {}: {}", line_num, msg);
                terminated = true;
                break;
            }
        } else {
            println!("Could not parse line {}: \"{}\"", line_num, line);
            terminated = true;
            break;
        }
    }
    if !terminated {
        match exec.report() {
            Ok(report) => println!("{:?}", report),
            Err(msg) => println!("{}", msg),
        }
    } else {
        println!("Terminated.");
    }
}
