mod input;
mod sorting;
mod student;
use crate::sorting::Sort;
use std::process;

// TODO: Better Error handling
fn main() {
    let no_of_students = student::Student::get_no_of_students().unwrap_or_else(|err| {
        // TODO: Use eprintln!
        println!("Problem getting number of students: {}", err);
        process::exit(1);
    });

    let mut students = student::Student::create_students(no_of_students).unwrap_or_else(|err| {
        // TODO: Use eprintln!
        println!("Problem creating students: {}", err);
        process::exit(1);
    });

    if let Err(e) = students.sorting_menu() {
        // TODO: Use eprintln!
        println!("Sorting Menu Error! {}", e);
        process::exit(1);
    };
}
