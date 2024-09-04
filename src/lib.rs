/*
The module tree of this project looks like:

crate
|-puzzle_3
|-puzzle_4
|-puzzle_5
|-utils


By using `mod <module_name>;` the rust compiler knows that the code
of the module is in a seperate file called <module_name>.rs
*/

// puzzle logic
pub mod puzzle_3;
pub mod puzzle_4;
pub mod puzzle_5;

// utitility functions for the puzzles package
pub mod utils;

// library to handle Error types
use std::error::Error;

struct Input {
    puzzle_number: u32,
    input_path: String,
}

impl Input {
    // convention to name constructor functions `new`
    // Result type either returns an Input type in the success case
    // or an error message in the error case
    fn new(args: &[String]) -> Result<Input, String> {
        match args.len() {
            2 => {}
            // TODO return a custom error not a string?
            _ => return Err("Too few arguments passed!".to_string()),
        }

        // Cloning is not ideal since it copies the value
        // However for now it's the easiest without defining lifetimes.
        let input_path: String = args[0].clone();
        let puzzle_number = args[1]
            .clone()
            .parse::<u32>()
            .map_err(|err| format!("Error parsing puzzle number: {err}"))?;

        Ok(Input {
            puzzle_number: puzzle_number,
            input_path: input_path,
        })
    }
}

// can use anyhow to return different types of errors.
// use anyhow::{Result, Error};

pub fn run(args: Vec<String>) -> Result<u64, Box<dyn Error>> {
    let input = Input::new(&args)?;

    println!(
        "Running puzzle {:?} on file: {:?}",
        input.puzzle_number, input.input_path
    );

    let output = match input.puzzle_number {
        3 => Ok(puzzle_3::run(&input.input_path)?),
        4 => Ok(puzzle_4::run(&input.input_path)?),
        5 => Ok(puzzle_5::run(&input.input_path)?),
        _ => Err("Puzzle does not exist".to_string().into()),
    };

    output
}

#[cfg(test)]
mod tests {
    // Import the parent module methods.
    use super::*;

    #[test]
    fn test_puzzle_3() -> Result<(), Box<dyn Error>> {
        let input_args = vec!["data/input_day_3.txt".to_string(), "3".to_string()];
        assert_eq!(run(input_args)?, 69527306);
        Ok(())
    }

    #[test]
    fn test_puzzle_4() -> Result<(), Box<dyn Error>> {
        let input_args = vec!["data/input_day_4.txt".to_string(), "4".to_string()];
        assert_eq!(run(input_args)?, 23806951);
        Ok(())
    }

    #[test]
    fn test_puzzle_5() -> Result<(), Box<dyn Error>> {
        let input_args = vec!["data/input_day_5.txt".to_string(), "5".to_string()];
        assert_eq!(run(input_args)?, 346433842);
        Ok(())
    }
}
