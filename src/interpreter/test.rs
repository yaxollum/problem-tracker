use super::*;

#[test]
fn test_cmd_set_problem_goal() -> Result<(), String> {
    let mut exec = Interpreter::default();
    assert_eq!(exec.problem_goal, None);
    let goal = 200;
    exec.next_command(Command::SetProblemGoal(goal))?;
    assert_eq!(exec.problem_goal, Some(goal));
    Ok(())
}

#[test]
fn test_cmd_set_penalty() -> Result<(), String> {
    let mut exec = Interpreter::default();
    assert_eq!(exec.penalty, None);
    let penalty = 5;
    exec.next_command(Command::SetPenalty(penalty))?;
    assert_eq!(exec.penalty, Some(penalty));
    Ok(())
}
/*
Command::SetPenalty(n) => {
    self.penalty = Some(n);
}
Command::BeginDate(date) => {
    self.process_current_date()?;
    if !self.check_next_date_contiguous(&date) {
        return Err("Date is not contiguous.".to_owned());
    }

    self.current_date = Some(DailyInformation {
        date: date,
        assigned: 0,
        penalty: false,
    });
}
Command::BeginChapter(n) => {
    self.current_chapter = Some(n);
}
Command::AssignedAmount(n) => {
    if let Some(current_date) = &mut self.current_date {
        current_date.assigned += n;
    } else {
        return Err("Cannot assign problems without setting date.".to_owned());
    }
}
Command::FinishedAmount(n) => {
    self.problems.finished(n)?;
}
Command::AddProblems(list) => {
    if let Some(current_chapter) = self.current_chapter {
        self.problems
            .add(list.iter().map(|&problem_number| Problem {
                number: problem_number,
                chapter: current_chapter,
                need_to_fix: false,
            }));
    } else {
        return Err("Cannot add problems without beginning chapter.".to_owned());
    }
}
Command::NeedToFix(list) => {
    self.change_need_to_fix_status(list, true)?;
}
Command::Fixed(list) => {
    self.change_need_to_fix_status(list, false)?;
}
Command::Penalty => {
    if let Some(current_date) = &mut self.current_date {
        current_date.penalty = true;
    } else {
        return Err("Cannot have penalty without setting date.".to_owned());
    }
}
Command::ResetRemaining => {}
Command::NOP => {}
*/
