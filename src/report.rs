use super::problem::Problem;

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
