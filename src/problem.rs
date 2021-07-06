use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Problem {
    pub number: u32,
    pub chapter: u32,
    pub fix_status: FixStatus,
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "problem {} of chapter {}", self.number, self.chapter)
    }
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
