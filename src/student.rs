mod subject;
use std::collections::HashMap;

pub type Students = Vec<Student>;

// TODO: implement Ord and PartialOrd for HashMap
#[derive(Debug, Eq, PartialEq)]
pub struct Student {
    pub name: String,
    pub subjects: subject::Subjects,
    pub total: subject::Marks,
}

impl Student {
    pub fn new(name: String) -> Self {
        Student {
            name,
            subjects:HashMap::new(),
            total: 0,
        }
    }

    pub fn calculate_total_marks(&mut self) {
            for (_, value) in &self.subjects {
                self.total += value;
            }
    }
}
