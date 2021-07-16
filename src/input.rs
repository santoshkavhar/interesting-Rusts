use crate::student;
mod report;
use std::{io, process};

// Associated functions
impl student::Student {
    pub fn get_no_of_students() -> u8 {
        // Not Allowed here
        // let mut no_of_students = String::new();

        loop {
            let mut no_of_students = String::new();

            println!("Hello there, please enter the number of students(1 to 100 only):");

            io::stdin()
                .read_line(&mut no_of_students)
                .expect("Failed to read number of students.");

            let no_of_students: u8 = match no_of_students.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            if no_of_students > 0 && no_of_students < 101 {
                return no_of_students;
            } else if no_of_students == 0 {
                println!("You entered 0 !!! quitting...");
                process::exit(0);
            }
        }
    }

    pub fn create_students(no_of_students: u8) -> student::Students {
        let mut students = student::Students::new();
        for i in 0..no_of_students {
            let student = report::report_generation::get_ith_student_details(i + 1);
            //println!("{:?}", student);
            students.push(student);
        }
        students
    }
}
