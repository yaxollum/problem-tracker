#[cfg(test)]
mod test;

use super::commands::Command;
use super::error::InterpreterError;
use super::problem::{FixStatus, Problem};
use super::report::Report;
use chrono::NaiveDate;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct DailyInformation {
    date: NaiveDate,
    assigned: u32,
    penalty: bool,
}

#[derive(Default, Debug)]
struct Problems {
    unsolved: VecDeque<Problem>,
    solved_not_used: VecDeque<Problem>,
    solved_used: VecDeque<Problem>,
    solved_before_reset: Vec<Problem>,
}

impl Problems {
    pub fn add<I>(&mut self, list: I)
    where
        I: Iterator<Item = Problem>,
    {
        self.unsolved.extend(list);
    }
    pub fn finished(&mut self, n: u32) -> Result<(), InterpreterError> {
        let n = n as usize;
        if n <= self.unsolved.len() {
            self.solved_not_used.extend(self.unsolved.drain(..n));
            Ok(())
        } else {
            Err(InterpreterError::NotEnoughProblems)
        }
    }
    pub fn solved_iter_mut(
        &mut self,
    ) -> std::iter::Chain<
        std::collections::vec_deque::IterMut<Problem>,
        std::collections::vec_deque::IterMut<Problem>,
    > {
        self.solved_used
            .iter_mut()
            .chain(self.solved_not_used.iter_mut())
    }

    pub fn solved_iter(
        &self,
    ) -> std::iter::Chain<
        std::collections::vec_deque::Iter<Problem>,
        std::collections::vec_deque::Iter<Problem>,
    > {
        self.solved_used.iter().chain(self.solved_not_used.iter())
    }
    pub fn use_problems(&mut self, n: u32) -> bool {
        let n = n as usize;
        if n <= self.solved_not_used.len() {
            self.solved_used.extend(self.solved_not_used.drain(..n));
            true
        } else {
            false
        }
    }
    pub fn total_solved(&self) -> u32 {
        (self.solved_not_used.len() + self.solved_used.len()) as u32
    }
    pub fn total_solved_not_used(&self) -> u32 {
        self.solved_not_used.len() as u32
    }
    pub fn reset_remaining(&mut self) {
        self.solved_before_reset.extend(self.solved_used.drain(..));
        self.solved_before_reset
            .extend(self.solved_not_used.drain(..));
    }
}

#[derive(Default, Debug)]
pub struct Interpreter {
    problem_goal: Option<u32>,
    penalty: Option<u32>,
    current_date: Option<DailyInformation>,
    current_chapter: Option<u32>,

    problems: Problems,

    total_penalty: u32,
}

impl Interpreter {
    pub fn report(&self) -> Result<Report, &str> {
        if let Some(problem_goal) = self.problem_goal {
            if let Some(current_date) = &self.current_date {
                let total_penalty = self.total_penalty;

                let need_to_fix_problems: Vec<Problem> = self
                    .problems
                    .solved_iter()
                    .filter(|p| p.fix_status == FixStatus::NeedToFix)
                    .cloned()
                    .collect();
                let total_need_to_fix = need_to_fix_problems.len() as u32;
                let unsolved_problems: Vec<Problem> =
                    self.problems.unsolved.iter().cloned().collect();
                let total_solved = self.problems.total_solved();
                let total_remaining =
                    problem_goal + total_penalty - total_solved + total_need_to_fix;
                let assigned = current_date.assigned;
                let total_solved_not_used = self.problems.total_solved_not_used();
                let assigned_problems_are_completed = assigned <= total_solved_not_used;
                Ok(Report {
                    total_remaining,
                    total_solved,
                    total_penalty,
                    total_need_to_fix,
                    unsolved_problems,
                    need_to_fix_problems,
                    assigned,
                    assigned_problems_are_completed,
                    total_solved_not_used,
                })
            } else {
                Err("Cannot generate report without beginning date.")
            }
        } else {
            Err("Cannot generate report without setting problem goal.")
        }
    }
    pub fn next_command(&mut self, cmd: Command) -> Result<(), InterpreterError> {
        match cmd {
            Command::SetProblemGoal(n) => {
                self.problem_goal = Some(n);
            }
            Command::SetPenalty(n) => {
                self.penalty = Some(n);
            }
            Command::BeginDate(date) => {
                self.process_current_date()?;
                if !self.check_next_date_contiguous(&date) {
                    return Err(InterpreterError::DateNotContiguous);
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
                    return Err(InterpreterError::AssignProblemsNoDate);
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
                            fix_status: FixStatus::Fixed,
                        }));
                } else {
                    return Err(InterpreterError::AddProblemsWithoutChapter);
                }
            }
            Command::AddEvenProblems(list) => {
                self.next_command(Command::AddProblems(
                    list.iter().cloned().filter(|x| x % 2 == 0).collect(),
                ))?;
            }
            Command::AddOddProblems(list) => {
                self.next_command(Command::AddProblems(
                    list.iter().cloned().filter(|x| x % 2 == 1).collect(),
                ))?;
            }
            Command::NeedToFix(list) => {
                self.change_need_to_fix_status(list, FixStatus::NeedToFix)?;
            }
            Command::Fixed(list) => {
                self.change_need_to_fix_status(list, FixStatus::Fixed)?;
            }
            Command::Penalty => {
                if let Some(current_date) = &mut self.current_date {
                    current_date.penalty = true;
                } else {
                    return Err(InterpreterError::PenaltyNoDate);
                }
            }
            Command::ResetRemaining => {
                self.total_penalty = 0;
                self.problems.reset_remaining();
            }
            Command::NOP => {}
        }
        Ok(())
    }
    fn find_problem<'a, I>(
        mut problem_list: I,
        problem_number: u32,
        chapter: u32,
    ) -> Option<&'a mut Problem>
    where
        I: Iterator<Item = &'a mut Problem>,
    {
        problem_list.find(|p| p.number == problem_number && p.chapter == chapter)
    }
    fn check_next_date_contiguous(&self, date: &NaiveDate) -> bool {
        if let Some(current_date) = &self.current_date {
            date == &current_date.date.succ()
        } else {
            true
        }
    }
    fn process_current_date(&mut self) -> Result<(), InterpreterError> {
        if let Some(current_date) = &self.current_date {
            let enough_problems_to_use = self.problems.use_problems(current_date.assigned);

            if !enough_problems_to_use {
                if current_date.penalty {
                    if let Some(penalty) = self.penalty {
                        self.total_penalty += penalty;
                    } else {
                        return Err(InterpreterError::PenaltyNotSet(current_date.date));
                    }
                } else {
                    return Err(InterpreterError::MissingPenalty(current_date.date));
                }
            } else {
                if current_date.penalty {
                    return Err(InterpreterError::UnexpectedPenalty(current_date.date));
                }
            }
        }
        Ok(())
    }
    fn change_need_to_fix_status(
        &mut self,
        list: Vec<u32>,
        new_status: FixStatus,
    ) -> Result<(), InterpreterError> {
        if let Some(current_chapter) = self.current_chapter {
            for problem_number in list {
                if let Some(mut problem) = Interpreter::find_problem(
                    self.problems.solved_iter_mut(),
                    problem_number,
                    current_chapter,
                ) {
                    if problem.fix_status != new_status {
                        problem.fix_status = new_status;
                    } else {
                        return Err(InterpreterError::FixStatusNotChanged(
                            problem.clone(),
                            new_status,
                        ));
                    }
                } else {
                    return Err(InterpreterError::FixStatusProblemNotFound(
                        Problem {
                            number: problem_number,
                            chapter: current_chapter,
                            fix_status: FixStatus::Fixed,
                        },
                        new_status,
                    ));
                }
            }
        } else {
            return Err(InterpreterError::FixStatusWithoutChapter(new_status));
        }
        Ok(())
    }
}
