use std::fmt;

#[derive(Debug)]
pub enum StudentError {
    ParsingStudentNumbersError,
}

impl std::error::Error for StudentError {}

impl fmt::Display for StudentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            StudentError::ParsingStudentNumbersError => write!(f, "Parsing number of students failed!")
        }
    }
}