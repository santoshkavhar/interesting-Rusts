use crate::student;
use std::{io, process};

pub trait Sort {
    fn sorting_menu(&mut self) -> Result<(), String>;
    fn sort_(&mut self, sort_kind: SortBy);
}

pub enum SortBy {
    Name,
    Total,
}

impl Sort for student::Students {
    fn sorting_menu(&mut self) -> Result<(), String> {
        loop {
            println!(
                "Please enter your choice of sorting:
            1.Sort by name
            2.Sort by total marks.
            3.Quit."
            );

            let mut choice = String::new();
            match io::stdin().read_line(&mut choice) {
                Ok(_) => {}
                Err(error) => {
                    return Err(
                        format!("Couldn't get user choice of sorting! Err: {}", error).to_string(),
                    )
                }
            }

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

    fn sort_(&mut self, sort_kind: SortBy) {
        match sort_kind {
            SortBy::Name => self.sort_by(|a, b| a.name.cmp(&b.name)),
            SortBy::Total => self.sort_by(|a, b| b.total.cmp(&a.total)),
        }
        println!("{:#?}", self);
    }
}
