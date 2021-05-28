use super::commands::Command;
use super::problem::Problem;
use super::report::Report;
use chrono::NaiveDate;
use std::cmp;

struct DailyInformation {
    date: NaiveDate,
    assigned: u32,
    penalty: bool,
}

#[derive(Default)]
struct Problems {
    data: Vec<Problem>,
    solved_used_end_index: usize,
    solved_not_used_end_index: usize,
}

impl Problems {
    pub fn add<I>(&mut self, list: I)
    where
        I: Iterator<Item = Problem>,
    {
        self.data.extend(list);
    }
    pub fn finished(&mut self, n: u32) -> Result<(), &str> {
        let n = n as usize;
        if self.solved_not_used_end_index + n < self.data.len() {
            self.solved_not_used_end_index += n;
            Ok(())
        } else {
            Err("Not enough problems to finish.")
        }
    }
    pub fn solved_iter_mut(&mut self) -> std::iter::Take<std::slice::IterMut<Problem>> {
        self.data.iter_mut().take(self.solved_not_used_end_index)
    }
    pub fn solved_iter(&self) -> std::iter::Take<std::slice::Iter<Problem>> {
        self.data.iter().take(self.solved_not_used_end_index)
    }
    pub fn unsolved_iter(&self) -> std::iter::Skip<std::slice::Iter<Problem>> {
        self.data.iter().skip(self.solved_not_used_end_index)
    }
    pub fn use_problems(&mut self, n: u32) -> bool {
        let n = n as usize;
        if self.solved_used_end_index + n <= self.solved_not_used_end_index {
            self.solved_used_end_index += n;
            true
        } else {
            false
        }
    }
    pub fn total_solved(&self) -> u32 {
        self.solved_not_used_end_index as u32
    }
    pub fn total_solved_not_used(&self) -> u32 {
        (self.solved_not_used_end_index - self.solved_used_end_index) as u32
    }
}

#[derive(Default)]
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
                    .filter(|p| p.need_to_fix)
                    .cloned()
                    .collect();
                let total_need_to_fix = need_to_fix_problems.len() as u32;
                let unsolved_problems: Vec<Problem> =
                    self.problems.unsolved_iter().cloned().collect();
                let total_solved = self.problems.total_solved();
                let total_remaining =
                    problem_goal + total_penalty - total_solved - total_need_to_fix;
                let assigned = current_date.assigned;
                let assigned_problems_completion = (
                    cmp::min(self.problems.total_solved_not_used(), assigned),
                    assigned,
                );
                Ok(Report {
                    total_remaining,
                    total_solved,
                    total_penalty,
                    total_need_to_fix,
                    unsolved_problems,
                    need_to_fix_problems,
                    assigned_problems_completion,
                })
            } else {
                Err("Cannot generate report without beginning date.")
            }
        } else {
            Err("Cannot generate report without setting problem goal.")
        }
    }
    pub fn next_command(&mut self, cmd: Command) -> Result<(), &str> {
        match cmd {
            Command::SetProblemGoal(n) => {
                self.problem_goal = Some(n);
            }
            Command::SetPenalty(n) => {
                self.penalty = Some(n);
            }
            Command::BeginDate(date) => {
                self.process_current_date();
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
                    return Err("Cannot assign problems without setting date.");
                }
            }
            Command::FinishedAmount(n) => {
                self.problems.finished(n);
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
                    return Err("Cannot add problems without beginning chapter.");
                }
            }
            Command::NeedToFix(list) => {
                self.change_need_to_fix_status(list, true);
            }
            Command::Fixed(list) => {
                self.change_need_to_fix_status(list, false);
            }
            Command::Penalty => {
                if let Some(current_date) = &mut self.current_date {
                    current_date.penalty = true;
                } else {
                    return Err("Cannot have penalty without setting date.");
                }
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
    fn process_current_date(&mut self) -> Result<(), String> {
        if let Some(current_date) = &self.current_date {
            let enough_problems_to_use = self.problems.use_problems(current_date.assigned);

            if !enough_problems_to_use {
                if current_date.penalty {
                    if let Some(penalty) = self.penalty {
                        self.total_penalty += penalty;
                    } else {
                        return Err(format!(
                            "Penalty not set; assigned work was not completed on {}",
                            current_date.date
                        ));
                    }
                } else {
                    return Err(format!(
                        "Missing \"penalty\" command; assigned work was not completed on {}",
                        current_date.date
                    ));
                }
            }
        }
        Ok(())
    }
    fn change_need_to_fix_status(
        &mut self,
        list: Vec<u32>,
        new_status: bool,
    ) -> Result<(), String> {
        let new_status_str = if new_status { "need to fix" } else { "fixed" };
        if let Some(current_chapter) = self.current_chapter {
            for problem_number in list {
                if let Some(mut problem) = Interpreter::find_problem(
                    self.problems.solved_iter_mut(),
                    problem_number,
                    current_chapter,
                ) {
                    if problem.need_to_fix != new_status {
                        problem.need_to_fix = new_status;
                    } else {
                        return Err(format!(
                            "{} already has status \"{}\".",
                            problem, new_status_str
                        ));
                    }
                }
            }
        } else {
            return Err(format!(
                "Cannot mark problems as \"{}\" without beginning chapter.",
                new_status_str
            ));
        }
        Ok(())
    }
}
