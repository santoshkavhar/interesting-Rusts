use crate::student::Students;
use std::io;
use std::process;

pub trait Sort {
    fn menu(&mut self);
    fn sort_(&mut self, sort_kind : SortBy);
}

pub enum SortBy{
    Name,
    Total,
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
                1 => self.sort_(SortBy::Name),
                2 => self.sort_(SortBy::Total),
                3 => {
                    println!("Ok, Quitting...");
                    process::exit(0);
                }
                _ => println!("Unexpected choice! Try Again!!"),
            };
        }
    }

    fn sort_(&mut self, sort_kind : SortBy) {
        match sort_kind{
            SortBy::Name =>  self.sort(),
            SortBy::Total => self.sort_by(|a, b| b.total.cmp(&a.total)),
        }
        println!("{:#?}", self);
    }
}
