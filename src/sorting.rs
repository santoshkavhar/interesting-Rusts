use crate::student::Students;
use std::io;
use std::process;

pub fn menu(students: &mut Students) {
    // Ask for which order to sort.

    // Ask if want to continue
    loop {
        println!(
            "Please enter your choice of sorting:
        1.Sort by name
        2.Sort by total marks.
        3.Quit."
        );

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Couldn't get user choice of sorting!!");

        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => sort_by_name(students),
            2 => sort_by_total(students),
            3 => {
                println!("Ok, Quitting...");
                process::exit(0);
            }
            _ => println!("Unexpected choice! Try Again!!"),
        };
    }
}

fn sort_by_name(students: &mut Students) {
    students.sort();
    println!("{:#?}", students);
}

fn sort_by_total(students: &mut Students) {
    students.sort_by(|a, b| b.total.cmp(&a.total));
    println!("{:#?}", students);
}
