extern crate problem_tracker;

use problem_tracker::{run, RunResult};

#[test]
fn test_parse_error() {
    let program = b"set problem goal 400\nbad command";
    if let RunResult::ParseError { line_num, .. } = run(&program[..]) {
        assert_eq!(line_num, 2);
    } else {
        panic!();
    }
}
