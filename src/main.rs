use std::io;

type Marks = u16;

#[derive(Debug)]
struct Subjects{
    maths: Marks,
    physics: Marks,
    english: Marks,
}

#[derive(Debug)]
struct Student{
    name:String,
    subjects:Subjects,
    total: Marks,
}

type Students = Vec<Student>;

fn get_input_no_of_students() -> u8 {
    
    // Not Allowed here
    // let mut no_of_students = String::new();

    loop{
        let mut no_of_students = String::new();

        println!("Hello there, please enter the number of students(1 to 100 only):");

        io::stdin().read_line(&mut no_of_students).expect("Failed to read number of students.");

        let no_of_students: u8 = match no_of_students.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if no_of_students > 0 && no_of_students < 101{
            return no_of_students
        }
    }
}

fn create_students(no_of_students : u8) -> Students {
    let mut students = Students::new(); 
    for i in 0..no_of_students{
        let student = get_ith_student_details(i + 1);
        println!("{:?}", student );
        students.push(student);
    }
    students
}

fn get_ith_student_details(student_number: u8) -> Student {
    
    println!("Enter name for Student number {}: ", student_number);
    let mut name = String::new();

    io::stdin().read_line(&mut name).expect("Failed to read name for Student!!");
    let name: String = String::from(name.trim());

    let mut subjects = Subjects{maths:0, physics:0, english:0};
    let mut student = Student{name:String::from(name), total:0, subjects};

    println!("-----------" );
    println!("{:?}", student);
    println!("-----------" );

    loop {
        println!("Enter Maths marks for Student number {}: ", student_number);//, Physics and English : ");
        let mut maths_marks = String::new();
        io::stdin().read_line(&mut maths_marks).expect("Failed to read maths marks!!");
    
        let maths_marks: u16 = match maths_marks.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        student.subjects.maths = maths_marks;
        break;

    }


    println!("-----------" );
    println!("{:?}", student);
    println!("-----------" );

    student

}

fn main() {
    
    let no_of_students = get_input_no_of_students();
    
    //println!("{}", no_of_students );

    let students = create_students(no_of_students);

    println!("{:?}", students );
    
    

}
