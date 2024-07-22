pub const VERSION: &str = "0.1.0";
pub const STATUS: &str = "STABLE";

#[doc = "Provide basic event utility"]
pub mod game_utils {

    use rand::{distributions::uniform::SampleUniform, Rng};
    use std::{
        cmp::{Ord, PartialOrd},
        io::stdin,
        ops::{
            Add, AddAssign, Div, DivAssign, Mul, MulAssign, RangeInclusive, Rem, RemAssign, Sub,
            SubAssign,
        },
    };

    #[doc = "Get user input from stdin"]
    pub fn input() -> String {
        let mut user_input = String::new();
        match stdin().read_line(&mut user_input) {
            Ok(_) => user_input.trim().to_owned(),
            Err(_) => "".to_owned(),
        }
    }

    #[doc = "Create a random number with type Int"]
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
        rand::thread_rng().gen_range(range)
    }

    #[deprecated]
    #[doc = "Parse &str into usize with delimeter (deprecated)"]
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

#[doc = "Provide basic debugging utility"]
pub mod debug_utils {
    use std::env::args;

    #[derive(Debug, Clone)]
    #[deprecated]
    #[doc = "Types for managing arguments (deprecated)"]
    pub struct Argument {
        list: Vec<String>,
    }

    #[allow(deprecated)]
    impl Argument {
        #[deprecated]
        #[doc = "Create a new instances of argument (deprecated)"]
        pub fn new() -> Self {
            Self { list: Vec::new() }
        }

        #[deprecated]
        #[doc = "Collect arguments from CLI (deprecated)"]
        pub fn get(&mut self, limiter: usize) {
            for (index, args) in args().enumerate() {
                if index != limiter {
                    self.list.push(args);
                } else {
                    break;
                }
            }
        }

        #[deprecated]
        #[doc = "Remove some arguments from list (deprecated)"]
        pub fn remove(&mut self, index: usize) {
            let _ = self.list.remove(index);
        }

        #[deprecated]
        #[doc = "Pops out argument based on index (deprecated)"]
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
