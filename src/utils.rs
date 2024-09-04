use std::fs::File;

// rust prelude and traits are imporant to understand here
// https://doc.rust-lang.org/std/prelude/
// https://doc.rust-lang.org/book/ch10-02-traits.html#defining-a-trait
use std::io;
use std::io::prelude::*;

pub fn read_file(path: &str) -> Result<String, io::Error> {
    let mut contents = String::new();

    File::open(path)?.read_to_string(&mut contents)?;

    // this gives ownership of the file contents to the calling function
    Ok(contents)
}

// convert an input string to a vector of vectors
pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in input.split("\n") {
        let mut line_characters: Vec<char> = Vec::new();
        for character in line.chars() {
            line_characters.push(character);
        }
        matrix.push(line_characters);
    }
    return matrix;
}

// write a character matrix to file
pub fn write_matrix_to_file(matrix: &Vec<Vec<char>>, file_name: String) {
    let mut file = File::create(file_name).expect("Unable to open file.");
    for i in matrix {
        for j in i {
            write!(file, "{}", j).expect("Could not write to file.");
        }
        write!(file, "\n").expect("Could not write to file.");
    }
}

// calculate the numerical value of a list of numbers
// the position of the number in the vector represents its place in the acutal number
pub fn numerical_value(characters: &Vec<u32>) -> u32 {
    let mut numerical_value: u32 = 0;
    for (index, digit) in characters.iter().enumerate() {
        let invers_index = (characters.len() as u32 - 1) - (index as u32);
        let power = 10_u32.pow(invers_index);
        let mul = power * digit;
        numerical_value += mul;
    }
    return numerical_value;
}
