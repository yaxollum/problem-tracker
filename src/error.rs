use std::fmt;

pub enum InterpreterError {
    DateNotContiguous,
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{}",
            match self {
                DateNotContiguous => "Date is not contiguous.",
            }
        )?;
        Ok(())
    }
}
