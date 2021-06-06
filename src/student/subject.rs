use std::collections::HashMap;

pub type Marks = u16;
pub type SubjectName = String;
pub type Subjects = HashMap<SubjectName, Marks>;

#[derive(Debug, Eq, PartialEq)]
pub enum Class{
    Distinction,
    FirstClass,
    SecondClass,
}
