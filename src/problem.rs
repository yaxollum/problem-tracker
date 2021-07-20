use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ProblemID {
    pub number: u32,
    pub chapter: u32,
}

impl fmt::Display for ProblemID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "problem {} of chapter {}", self.number, self.chapter)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Problem {
    pub id: ProblemID,
    pub fix_status: FixStatus,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FixStatus {
    Fixed,
    NeedToFix,
}

impl fmt::Display for FixStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\"{}\"",
            match self {
                Self::Fixed => "fixed",
                Self::NeedToFix => "need to fix",
            }
        )
    }
}
