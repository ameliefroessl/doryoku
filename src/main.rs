// library to help us exit the program without panicing
use std::process;

// library to handle everything env related
use std::env;

// out run function is defined in our library crate
use puzzles::run;

fn main() {
    println!("Welcome to my puzzle repository!");

    // get input parameters from the command line args
    let args: Vec<String> = env::args().collect();

    // Using the let Err() syntax, we are able to cleanly handle
    // any errors being passed from the run fucntion.
    if let Err(e) = run(args[1..].to_vec()) {
        // print to standard error
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
