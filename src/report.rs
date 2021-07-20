use super::problem::ProblemID;
use std::fmt;

#[derive(Debug)]
pub struct Report {
    pub total_remaining: u32,
    pub total_solved: u32,
    pub total_penalty: u32,
    pub total_need_to_fix: u32,
    pub unsolved_problems: Vec<ProblemID>,
    pub need_to_fix_problems: Vec<ProblemID>,
    pub assigned_problems_are_completed: bool,
    pub assigned: u32,
    pub total_solved_not_used: u32,
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Assigned: {} ({})",
            self.assigned,
            if self.assigned_problems_are_completed {
                "COMPLETED"
            } else {
                "NOT COMPLETED"
            }
        )?;
        writeln!(f, "Total Solved Not Used: {}\n", self.total_solved_not_used)?;
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
