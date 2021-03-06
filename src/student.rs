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
        self.class = compute_class(self.total, (self.subjects.len() * 100) as u16)
    }

}

fn compute_class(total: subject::Marks, total_marks: subject::Marks) -> Option<class::Class>{
    let percent: f64 = (total as f64/total_marks as f64) * 100.0;

    if percent >= 80.0 {
        return Some(class::Class::Distinction)
    } else if percent >= 60.0 && percent < 80.0 {
        return Some(class::Class::FirstClass)
    } else if percent >= 40.0 && percent < 60.0 {
        return Some(class::Class::SecondClass)
    }
    None
}