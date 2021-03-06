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

#[test]
fn test_cmd_begin_date() -> Result<(), InterpreterError> {
    let mut exec = Interpreter::default();
    assert!(exec.current_date.is_none());
    let date = NaiveDate::from_ymd(2021, 07, 01);
    exec.next_command(Command::BeginDate(date))?;
    assert_eq!(exec.current_date.unwrap().date, date);
    Ok(())
}

#[test]
fn test_cmd_begin_date_not_contiguous() -> Result<(), InterpreterError> {
    let mut exec = Interpreter::default();
    let date1 = NaiveDate::from_ymd(2021, 07, 01);
    let date2 = NaiveDate::from_ymd(2021, 07, 03);
    exec.next_command(Command::BeginDate(date1))?;
    assert_eq!(
        exec.next_command(Command::BeginDate(date2)).unwrap_err(),
        InterpreterError::DateNotContiguous
    );
    Ok(())
}

#[test]
fn test_cmd_begin_chapter() -> Result<(), InterpreterError> {
    let mut exec = Interpreter::default();
    assert!(exec.current_chapter.is_none());
    let chapter = 5;
    exec.next_command(Command::BeginChapter(chapter))?;
    assert_eq!(exec.current_chapter.unwrap(), chapter);
    Ok(())
}

#[test]
fn test_cmd_assigned_amount() -> Result<(), InterpreterError> {
    let mut exec = Interpreter::default();
    let date = NaiveDate::from_ymd(2021, 07, 01);
    exec.next_command(Command::BeginDate(date))?;

    assert_eq!(exec.current_date.clone().unwrap().assigned, 0);
    exec.next_command(Command::AssignedAmount(5))?;
    assert_eq!(exec.current_date.clone().unwrap().assigned, 5);
    exec.next_command(Command::AssignedAmount(7))?;
    assert_eq!(exec.current_date.unwrap().assigned, 12);
    Ok(())
}

#[test]
fn test_cmd_assigned_amount_no_date() -> Result<(), InterpreterError> {
    let mut exec = Interpreter::default();
    assert_eq!(
        exec.next_command(Command::AssignedAmount(5)).unwrap_err(),
        InterpreterError::AssignProblemsNoDate
    );
    Ok(())
}

fn check_unsolved_numbers(exec: &Interpreter, expected_numbers: Vec<u32>) {
    let unsolved: Vec<u32> = exec.problems.unsolved.iter().map(|p| p.number).collect();
    assert_eq!(unsolved, expected_numbers);
}

#[test]
fn test_cmd_add_problems() -> Result<(), InterpreterError> {
    let mut exec = Interpreter::default();
    exec.next_command(Command::BeginChapter(8))?;
    exec.next_command(Command::AddProblems((1..=6).collect(), None))?;
    check_unsolved_numbers(&exec, vec![1, 2, 3, 4, 5, 6]);
    Ok(())
}

#[test]
fn test_cmd_add_even_problems() -> Result<(), InterpreterError> {
    let mut exec = Interpreter::default();
    exec.next_command(Command::BeginChapter(8))?;
    exec.next_command(Command::AddEvenProblems((1..=6).collect(), None))?;
    check_unsolved_numbers(&exec, vec![2, 4, 6]);
    Ok(())
}

#[test]
fn test_cmd_add_odd_problems() -> Result<(), InterpreterError> {
    let mut exec = Interpreter::default();
    exec.next_command(Command::BeginChapter(8))?;
    exec.next_command(Command::AddOddProblems((1..=6).collect(), None))?;
    check_unsolved_numbers(&exec, vec![1, 3, 5]);
    Ok(())
}

#[test]
fn test_cmd_add_duplicate_problem() -> Result<(), InterpreterError> {
    let mut exec = Interpreter::default();
    exec.next_command(Command::BeginChapter(8))?;
    exec.next_command(Command::AddProblems((1..=6).collect(), None))?;
    assert_eq!(
        exec.next_command(Command::AddProblems(vec![5], None))
            .unwrap_err(),
        InterpreterError::DuplicateProblemAdded(ProblemID {
            number: 5,
            chapter: 8,
        })
    );
    Ok(())
}
