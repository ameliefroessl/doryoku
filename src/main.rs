// bring std packages into scope
use std::env;
// use the puzzle_3 module which is defined in an outside file.
mod puzzle_3;

fn main() {
    println!("Welcome to my puzzle repository!");

    // get input parameters from the command line args
    let args: Vec<String> = env::args().collect();

    // first argument is the input path
    let input_path: &String = &args[1];

    // second argument is the puzzle number
    let puzzle: Result<i32, std::num::ParseIntError> = (*args[2]).parse();
    let puzzle: i32 = match puzzle {
        Ok(puzzle) => puzzle,
        Err(error) => panic!("The second argument should be an int! {:?}", error),
    };
    println!("Running puzzle {puzzle} on file: {input_path}");

    if puzzle == 3 {
        puzzle_3::run(input_path);
    }
}
