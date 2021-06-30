use std::fmt;

pub enum InterpreterError {
    DateNotContiguous,
    AssignProblemsNoDate,
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{}",
            match self {
                DateNotContiguous => "Date is not contiguous.",
                AssignProblemsNoDate=>"Cannot assign problems without setting date."
            }
        )?;
        Ok(())
    }
}
