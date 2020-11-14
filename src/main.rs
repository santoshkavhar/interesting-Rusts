mod sorting;
mod student;

fn main() {
    let no_of_students = student::get_no_of_students();

    let mut students = student::create_students(no_of_students);

    //println!("{:?}", students);

    sorting::menu(&mut students);
}
