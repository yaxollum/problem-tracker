use std::fmt;

#[derive(Clone, Debug)]
pub struct Problem {
    pub number: u32,
    pub chapter: u32,
    pub need_to_fix: bool,
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "problem {} of chapter {}", self.number, self.chapter)
    }
}
