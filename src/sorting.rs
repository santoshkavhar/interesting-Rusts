use crate::student::Students;
use std::io;
use std::process;

pub trait Sort {
    fn menu(&mut self);
    fn sort_by_name(&mut self);
    fn sort_by_total(&mut self);
}

impl Sort for Students {
    fn menu(&mut self) {
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
                1 => self.sort_by_name(),
                2 => self.sort_by_total(),
                3 => {
                    println!("Ok, Quitting...");
                    process::exit(0);
                }
                _ => println!("Unexpected choice! Try Again!!"),
            };
        }
    }

    fn sort_by_name(&mut self) {
        self.sort();
        println!("{:#?}", self);
    }

    fn sort_by_total(&mut self) {
        self.sort_by(|a, b| b.total.cmp(&a.total));
        println!("{:#?}", self);
    }
}
