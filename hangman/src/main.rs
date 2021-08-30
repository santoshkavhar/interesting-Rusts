use std::io;

struct Hangman {
    chances: u8,
    word: String,
    placeholder: String,
}

impl Hangman {
    fn initialize_placeholder(&mut self) {
        for _ in 0..self.word.len() {
            self.placeholder += "_";
        }
    }

    fn check_if_present(&mut self, user_input: char) {
        let mut count = 0;

        for (i, c) in self.word.chars().enumerate() {
            let placeholder_ith = self.placeholder.as_bytes()[i];
            let p: char = placeholder_ith as char;
            if c == user_input && p == '_' {
                let mut new_placeholder = String::from("");
                new_placeholder.push_str(&self.placeholder[0..i]);
                new_placeholder.push_str(&user_input.to_string());
                new_placeholder.push_str(&self.placeholder[i+1..]);

                self.placeholder = new_placeholder;
                count += 1;
            }
        }

        if count == 0 {
            if self.chances == 1 {
                println!("You lost! The word was : {}", self.word);
            }
            self.chances -= 1;
            return;
        }
        self.check_if_won();
    }

    fn check_if_won(&mut self) {
        let bytes = &self.placeholder[..].as_bytes();
        for &character in bytes.iter() {
            if character == b'_' {
                return;
            } 
        }
        println!("You Won!!");
        self.chances = 0;
    }
}

fn main() {
    println!("Hello, guess this word, you've 6 chances!");
    let mut hangman = Hangman {
        chances: 6,
        word: "castaway".to_string(),
        placeholder: String::from(""),
    };
    hangman.initialize_placeholder();

    while hangman.chances > 0 {
        println!(
            "You have {} chances left!!!
                {}",
            hangman.chances, hangman.placeholder
        );

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read user input.");
        let user_input: String = user_input.trim().to_string();
        //println!("{:?}", user_input);

        if user_input.len() == 1 {
            hangman.check_if_present(user_input.chars().nth(0).unwrap_or_default());
        } else {
            println!("Check your input, its length should be only 1");
        }
    }
}
