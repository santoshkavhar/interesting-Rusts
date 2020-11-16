mod subject;

pub type Students = Vec<Student>;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Student {
    pub name: String,
    pub subjects: subject::Subjects,
    pub total: subject::Marks,
}

impl Student {
    pub fn new(name: String) -> Self {
        let subjects = subject::Subjects {
            maths: 0,
            physics: 0,
            english: 0,
        };
        Student {
            name,
            subjects,
            total: 0,
        }
    }

    pub fn calculate_total_marks(&mut self) {
        for subject_mark in (self.subjects).marks() {
            self.total += subject_mark;
        }
    }
}
