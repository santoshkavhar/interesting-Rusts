use crate::student;
mod report;
use std::{io, process};

// Associated functions
impl student::Student {
    pub fn get_no_of_students() -> Result<u8, String> {
        // Not Allowed here
        // let mut no_of_students = String::new();

        let mut no_of_students = String::new();

        println!("Hello there, please enter the number of students(1 to 100 only):");

        io::stdin()
            .read_line(&mut no_of_students)
            .expect("Failed to read number of students.");

        let no_of_students: u8 = match no_of_students.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                return Err(format!("Parsing number of students failed! Err: {}", err).to_string())
            }
        };

        // TODO: set values from a config.yml
        if no_of_students > 0 && no_of_students < 101 {
            return Ok(no_of_students);
        } else if no_of_students == 0 {
            println!("You entered 0 !!! quitting...");
            process::exit(0);
        }
        return Err(format!("Number of students is invalid! {}", no_of_students));
    }

    pub fn create_students(no_of_students: u8) -> Result<student::Students, String> {
        let mut students = student::Students::new();
        for i in 0..no_of_students {
            // TOD0: Get from CSV
            let student = match report::report_generation::get_ith_student_details(i + 1) {
                Ok(stu) => stu,
                // TODO: Use eprintln!
                Err(error) => {
                    return Err(format!(
                        "Error fetching {} student's details. Err: {}",
                        i + 1,
                        error
                    ))
                }
            };
            //println!("{:?}", student);
            students.push(student);
        }
        Ok(students)
    }
}
