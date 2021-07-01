use super::*;

#[test]
fn test_cmd_set_problem_goal() -> Result<(), InterpreterError> {
    let mut exec = Interpreter::default();
    assert_eq!(exec.problem_goal, None);
    let goal = 200;
    exec.next_command(Command::SetProblemGoal(goal))?;
    assert_eq!(exec.problem_goal, Some(goal));
    Ok(())
}

#[test]
fn test_cmd_set_penalty() -> Result<(), InterpreterError> {
    let mut exec = Interpreter::default();
    assert_eq!(exec.penalty, None);
    let penalty = 5;
    exec.next_command(Command::SetPenalty(penalty))?;
    assert_eq!(exec.penalty, Some(penalty));
    Ok(())
}
