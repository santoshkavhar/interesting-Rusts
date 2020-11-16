mod input;
mod sorting;
mod student;
use crate::sorting::Sort;


fn main() {
    let no_of_students = input::get_no_of_students();

    let mut students = input::create_students(no_of_students);

    //println!("{:?}", students);

    students.menu();
}
