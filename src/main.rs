use std::io;
use std::iter;

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

    let mut subject_number = 1;

    loop {

        let subject_name = match subject_number{
            1 => "Maths",
            2 => "Physics",
            3 => "English",
            _ => "Unknown Subject"
        };

        println!("Enter {} marks for Student number {}(range 1 to 100): ", subject_name, student_number);//, Physics and English : ");
        let mut subject_mark = String::new();
        io::stdin().read_line(&mut subject_mark).expect("Failed to read marks!!");
    
        let subject_mark: u16 = match subject_mark.trim().parse() {
            Ok(num) => if num >= 0 && num <= 100{
                num
            } else {
                println!("Please check your range..." );
                continue
            },
            Err(_) => continue,
        };

        match subject_number{
            1 => student.subjects.maths =  subject_mark,
            2 => student.subjects.physics =  subject_mark,
            3 => student.subjects.english =  subject_mark,
            _ => println!("Unknown Subject"),
        };

        subject_number += 1;
        if subject_number == 4 {
            break;
        }
    }

    student.total = calculate_total_marks(&student.subjects);

    student

}

fn calculate_total_marks(subjects: &Subjects) -> u16 {
    let mut total_marks : u16 = 0;
    for subject_mark in (*subjects).marks(){
        total_marks += subject_mark;
    }

    println!("total: {}",total_marks );
    
    //total_marks = subjects.maths + subjects.physics + subjects.english;
    total_marks
}


impl Subjects {
    fn marks(&self) -> impl Iterator<Item = u16> {
        let m = iter::once(self.maths);
        let p = iter::once(self.physics);
        let e = iter::once(self.english);
        m.chain(p).chain(e)
    }
}



fn main() {
    
    let no_of_students = get_input_no_of_students();

    let students = create_students(no_of_students);

    println!("{:?}", students );
    
    

}
