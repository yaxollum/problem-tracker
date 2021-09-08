use super::problem::{FixStatus, ProblemID};
use chrono::NaiveDate;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum InterpreterError {
    DateNotContiguous,
    AssignProblemsNoDate,
    NotEnoughProblems,
    AddProblemsWithoutChapter,
    DuplicateProblemAdded(ProblemID),
    PenaltyNoDate,
    PenaltyNotSet(NaiveDate),
    MissingPenalty(NaiveDate),
    UnexpectedPenalty(NaiveDate),
    FixStatusNotChanged(ProblemID, FixStatus),
    FixStatusProblemNotFound(ProblemID, FixStatus),
    FixStatusWithoutChapter(FixStatus),
    UnexpectedSkipProblem(u32),
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::DateNotContiguous => "Date is not contiguous.".to_owned(),
                Self::AssignProblemsNoDate =>
                    "Cannot assign problems without setting date.".to_owned(),
                Self::NotEnoughProblems => "Not enough problems to finish.".to_owned(),
                Self::AddProblemsWithoutChapter =>
                    "Cannot add problems without beginning chapter.".to_owned(),
                Self::DuplicateProblemAdded(problem) =>
                    format!("{} has already been added previously.", problem),
                Self::PenaltyNoDate => "Cannot have penalty without setting date.".to_owned(),
                Self::PenaltyNotSet(date) => format!(
                    "Penalty not set; assigned work was not completed on {}",
                    date
                ),
                Self::MissingPenalty(date) => format!(
                    "Missing \"penalty\" command; assigned work was not completed on {}",
                    date
                ),
                Self::UnexpectedPenalty(date) => format!(
                    "Unexpected \"penalty\" command; assigned work was completed on {}",
                    date
                ),
                Self::FixStatusNotChanged(problem, status) =>
                    format!("{} already has status {}.", problem, status),
                Self::FixStatusProblemNotFound(problem, status) => format!(
                    "{} not found in solved problems; cannot mark as {}",
                    problem, status
                ),
                Self::FixStatusWithoutChapter(status) => format!(
                    "Cannot mark problems as {} without beginning chapter.",
                    status
                ),
                Self::UnexpectedSkipProblem(num) => {
                    format!("Unexpected skip: problem {}. Only problems within the same \"add\" statement can be skipped.",num)
                }
            }
        )?;
        Ok(())
    }
}
