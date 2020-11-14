use std::io;
use std::process;
mod subject;

pub type Students = Vec<Student>;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Student {
    pub name: String,
    subjects: subject::Subjects,
    pub total: subject::Marks,
}

impl Student {
    fn new(name: String) -> Self {
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
}

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

pub fn create_students(no_of_students: u8) -> Students {
    let mut students = Students::new();
    for i in 0..no_of_students {
        let student = get_ith_student_details(i + 1);
        //println!("{:?}", student);
        students.push(student);
    }
    students
}

fn get_ith_student_details(student_number: u8) -> Student {
    println!("Enter name for Student number {}: ", student_number);
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name for Student!!");
    let name: String = String::from(name.trim());

    let mut student = Student::new(name);

    //println!("-----------");
    //println!("{:?}", student);
    //println!("-----------");

    let mut subject_number = 1;

    loop {
        let subject_name = match subject_number {
            1 => "maths",
            2 => "physics",
            3 => "english",
            _ => "Unknown Subject",
        };

        println!(
            "Enter {} marks for Student number {}(range 1 to 100): ",
            subject_name, student_number
        ); //, physics and english : ");
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

        match subject_number {
            1 => student.subjects.maths = subject_mark,
            2 => student.subjects.physics = subject_mark,
            3 => student.subjects.english = subject_mark,
            _ => println!("Unknown Subject"),
        };

        subject_number += 1;
        if subject_number == 4 {
            break;
        }
    }

    student.total = subject::calculate_total_marks(&student.subjects);

    student
}
