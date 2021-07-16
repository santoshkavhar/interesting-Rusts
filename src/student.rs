mod subject;
mod class;
use std::collections::HashMap;

pub type Students = Vec<Student>;


// TODO: implement Ord and PartialOrd for HashMap
#[derive(Debug, Eq, PartialEq)]
pub struct Student {
    pub name: String,
    pub subjects: subject::Subjects,
    pub total: subject::Marks,
    pub class: Option<class::Class>,
}

impl Student {
    pub fn new(name: String) -> Self {
        Student {
            name,
            subjects:HashMap::new(),
            total: 0,
            class: None,
        }
    }

    pub fn calculate_total_marks_and_class(&mut self) {
        // Calculate total marks
        for (_, value) in &self.subjects {
            self.total += value;
        }
        // Compute class
        self.class = class::compute_class(self.total, (self.subjects.len() * 100) as u16)
    }

}

