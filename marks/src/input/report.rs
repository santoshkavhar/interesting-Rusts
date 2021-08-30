pub mod report_generation {
    use crate::student;
    use std::io;

    pub fn get_ith_student_details(student_number: u8) -> Result<student::Student, String> {
        println!("Enter name for Student number {}: ", student_number);
        let mut name = String::new();

        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read name for Student!!");
        let name: String = String::from(name.trim());

        let mut student = student::Student::new(name);

        // println!("-----------
        //          {:?}
        //          -----------, student);

        let mut subject_number = 1;

        loop {
            // TODO: Get subjects from config
            let subject_name = match subject_number {
                1 => "maths",
                2 => "physics",
                3 => "english",
                _ => "Unknown Subject",
            };

            println!(
                "Enter {} marks for Student number {}(range 0 to 100): ",
                subject_name, student_number
            );
            let mut subject_mark = String::new();
            io::stdin()
                .read_line(&mut subject_mark)
                .expect("Failed to read marks!!");

            let subject_mark: u16 = match subject_mark.trim().parse() {
                Ok(num) => {
                    if num <= 100 {
                        num
                    } else {
                        println!("Please check your range...");
                        continue;
                    }
                }
                Err(_) => {
                    println!("Please enter valid number...");
                    continue;
                }
            };

            student
                .subjects
                .insert(subject_name.to_string(), subject_mark);

            subject_number += 1;
            if subject_number == 4 {
                break;
            }
        }

        student.calculate_total_marks_and_class();

        Ok(student)
    }
}
