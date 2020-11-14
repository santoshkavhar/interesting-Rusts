mod student;


fn main() {
    
    let no_of_students = student::get_input_no_of_students();

    let students = student::create_students(no_of_students);

    println!("{:?}", students );    
    

}
