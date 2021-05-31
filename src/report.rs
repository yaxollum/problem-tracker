use super::problem::Problem;
use std::fmt;

#[derive(Debug)]
pub struct Report {
    pub total_remaining: u32,
    pub total_solved: u32,
    pub total_penalty: u32,
    pub total_need_to_fix: u32,
    pub unsolved_problems: Vec<Problem>,
    pub need_to_fix_problems: Vec<Problem>,
    pub assigned_problems_completion: (u32, u32),
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Assigned Problems Completion: {} out of {}\n",
            self.assigned_problems_completion.0, self.assigned_problems_completion.1
        )?;
        writeln!(f, "Total Remaining: {}", self.total_remaining)?;
        writeln!(f, "Total Solved: {}", self.total_solved)?;
        writeln!(f, "Total Penalty: {}", self.total_penalty)?;
        writeln!(f, "Total Need to Fix: {}", self.total_need_to_fix)?;

        writeln!(f, "\nNeed to Fix Problems:")?;
        for p in &self.need_to_fix_problems {
            writeln!(f, "{}", p)?;
        }

        writeln!(f, "\nUnsolved Problems:")?;
        for p in &self.unsolved_problems {
            writeln!(f, "{}", p)?;
        }
        Ok(())
    }
}
