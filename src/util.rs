pub const VERSION: &str = "0.1.0";
pub const STATUS: &str = "STABLE";

pub mod game_utils {

    use rand::{distributions::uniform::SampleUniform, Rng};
    use std::{
        cmp::{Ord, PartialOrd},
        io,
        ops::{
            Add, AddAssign, Div, DivAssign, Mul, MulAssign, RangeInclusive, Rem, RemAssign, Sub,
            SubAssign,
        },
    };

    pub fn input() -> String {
        let mut user_input = String::new();
        if let Ok(_) = io::stdin().read_line(&mut user_input) {
            return user_input.trim().to_string();
        }

        return String::from("");
    }

    pub fn random<Int>(range: RangeInclusive<Int>) -> Int
    where
        Int: Ord
            + Add
            + Sub
            + Mul
            + Div
            + Rem
            + AddAssign
            + SubAssign
            + MulAssign
            + DivAssign
            + RemAssign
            + PartialOrd
            + SampleUniform,
    {
        let random_from_range: Int = rand::thread_rng().gen_range(range);
        return random_from_range;
    }

    pub fn parser_usize(text: &str) -> usize {
        let mut values: usize = 0;

        for (index, bytes) in text.as_bytes().iter().enumerate() {
            match bytes {
                &b'=' => {
                    let pivot = &text[index + 1..text.len()];
                    values = match pivot.parse::<usize>() {
                        Ok(parseint_success) => parseint_success,
                        Err(_) => 0,
                    };
                }

                _ => continue,
            }
        }

        values
    }
}

pub mod debug_utils {
    use std::env;

    #[derive(Debug, Clone)]
    pub struct Argument {
        list: Vec<String>,
    }

    impl Argument {
        pub fn new() -> Self {
            Self { list: Vec::new() }
        }

        pub fn get(&mut self, limiter: usize) {
            for (index, args) in env::args().enumerate() {
                if index != limiter {
                    self.list.push(args);
                } else {
                    break;
                }
            }
        }

        pub fn remove(&mut self, index: usize) {
            let _ = self.list.remove(index);
        }

        pub fn expose(&self, index: usize) -> String {
            match self.list.get(index) {
                Some(string) => string.clone(),
                None => String::from(""),
            }
        }
    }

    pub fn helper() -> String {
        "Guessing Game: How to\n".to_string()
            + "  --help | -h: display this help message\n"
            + "  --play | -p: play the game. limit: 100\n"
            + "  --limit=<n>: set the number limiter to <n>\n"
    }

    pub fn start_default() -> String {
        "Play mode: Default\n".to_string()
            + "  objective: guess the secret random number\n"
            + "  limit:     100 (Default)\n"
    }

    pub fn start_usize(opt: usize) -> String {
        format!("Play mode: Custom\n  objective: guess the secret random number\n  limit:     {opt} (Custom)\n")
    }

    pub fn parse_error(inputted: &str) -> String {
        format!("Error:  {} is not a valid number\n", inputted) + "Action: Continue\n"
    }
}
