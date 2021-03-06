use chrono::NaiveDate;

#[derive(Debug)]
pub enum Command {
    SetProblemGoal(u32),
    SetPenalty(u32),
    BeginDate(NaiveDate),
    BeginChapter(u32),
    AssignedAmount(u32),
    FinishedAmount(u32),
    AddProblems(Vec<u32>, Option<Vec<u32>>),
    AddEvenProblems(Vec<u32>, Option<Vec<u32>>),
    AddOddProblems(Vec<u32>, Option<Vec<u32>>),
    NeedToFix(Vec<u32>),
    Fixed(Vec<u32>),
    Penalty,
    ResetRemaining,
    NOP,
}
