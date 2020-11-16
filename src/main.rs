mod input;
mod sorting;
mod student;
use crate::sorting::Sort;
use crate::student::Student;

fn main() {
    let no_of_students = Student::get_no_of_students();

    let mut students = Student::create_students(no_of_students);

    //println!("{:?}", students);

    students.menu();
}
