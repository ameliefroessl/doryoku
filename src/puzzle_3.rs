use std::collections::HashSet;

use crate::utils::{numerical_value, parse_input, read_file};
// used to hold the data from the input.
struct CharNumber {
    row: usize,
    cols: Vec<usize>,
    numerical_value: u32,
}

pub fn run(input_path: &str) -> Result<u64, String> {
    let answer = solve(input_path);
    println!("The answer for puzzle 3 part 1 is: {answer}.");

    let answer = solve_part_2(input_path);
    println!("The answer for puzzle 3 part 2 is: {answer}.");

    // TODO find a way to return u32 and both answers
    Ok(u64::from(answer))
}

fn solve_part_2(input_path: &str) -> u32 {
    let input: String = read_file(input_path).expect("Could not read input file! ");

    let char_matrix = parse_input(&input);

    let mut char_numbers: Vec<CharNumber> = Vec::new();

    // let mut sum: u32 = 0;
    for (row_i, row) in char_matrix.iter().enumerate() {
        let mut current_number: Vec<u32> = Vec::new();
        let mut current_cols: Vec<usize> = Vec::new();

        for (col_i, col) in row.iter().enumerate() {
            // if we found a numeric number, we add it to our current number
            if col.is_numeric() {
                current_number.push(col.to_string().parse().unwrap());
                current_cols.push(col_i);
            }
            // if we hit the end of a number or the end of the row and the current number
            // is a part number, then we calculate the numerical value from it and pass it to the sum.
            if (!col.is_numeric() || col_i == row.len() - 1) && current_number.len() > 0 {
                let numerical_value = numerical_value(&current_number);
                char_numbers.push(CharNumber {
                    row: row_i,
                    cols: current_cols,
                    numerical_value: numerical_value,
                });
                // clear
                current_number = Vec::new();
                current_cols = Vec::new();
            }
        }
    }

    // fill the digital matrix
    let mut digital_matrix: Vec<Vec<u32>> = vec![vec![0; char_matrix[0].len()]; char_matrix.len()];
    fill_digital_matrix(&char_numbers, &mut digital_matrix);

    let gears = get_gears(&digital_matrix, &char_matrix);

    gears
}

fn get_gears(digital_matrix: &Vec<Vec<u32>>, char_matrix: &Vec<Vec<char>>) -> u32 {
    let mut sum = 0;

    for (row_i, row) in char_matrix.iter().enumerate() {
        for (col_i, col) in row.iter().enumerate() {
            // if the element is a symbol, check all the numbers
            if is_symbol(col) {
                //check for numers
                let gear = get_gear(digital_matrix, row_i, col_i);

                sum += gear;
            }
        }
    }
    return sum;
}

fn get_gear(matrix: &Vec<Vec<u32>>, row: usize, col: usize) -> u32 {
    let mut neighbors: HashSet<u32> = HashSet::new();
    let rows = matrix.len();
    let cols = matrix[0].len();

    // top
    if row != 0 {
        //middle
        let value = matrix[row - 1][col];
        update_neighbors(value, &mut neighbors);

        //left
        if col != 0 {
            let value = matrix[row - 1][col - 1];
            update_neighbors(value, &mut neighbors);
        }
        //right
        if col + 1 != cols {
            let value = matrix[row - 1][col + 1];
            update_neighbors(value, &mut neighbors);
        }
    }

    // left
    if col != 0 {
        let value = matrix[row][col - 1];
        update_neighbors(value, &mut neighbors);
    }

    //right
    if col + 1 != cols {
        let value = matrix[row][col + 1];
        update_neighbors(value, &mut neighbors);
    }

    // bottom
    if row + 1 != rows {
        //middle
        let value = matrix[row + 1][col];
        update_neighbors(value, &mut neighbors);
        //left
        if col != 0 {
            let value = matrix[row + 1][col - 1];
            update_neighbors(value, &mut neighbors);
        }
        //right
        if col + 1 != cols {
            let value = matrix[row + 1][col + 1];
            update_neighbors(value, &mut neighbors);
        }
    }

    if neighbors.len() == 2 {
        let half_gears: Vec<&u32> = neighbors.iter().collect();
        return half_gears[0] * half_gears[1];
    } else {
        return 0;
    }
}

fn update_neighbors(value: u32, neighbors: &mut HashSet<u32>) {
    if value > 0 && !neighbors.contains(&value) {
        neighbors.insert(value);
    }
}

fn fill_digital_matrix(char_numbers: &Vec<CharNumber>, digital_matrix: &mut Vec<Vec<u32>>) {
    for char_number in char_numbers.iter() {
        for column in char_number.cols.iter() {
            digital_matrix[char_number.row][*column] = char_number.numerical_value;
        }
    }
}

// solve the puzzle
// this puzzle tries to solve the puzzle outlined in AOC 2023 day 3
// the idea is to iterate over each element in the matrix. As soon as we hit a number char,
// we start testing to see if the char is a part number (one of its neighbors is a symbol),
// we continue testing for precending char in the row until we've reached the end of a number.
// if the number (possible stretching across multiple indexes) is a part_number we add it to
// the sum, otherwise we drop it.
fn solve(input_path: &str) -> u32 {
    let input: String = read_file(input_path).expect("Could not read input file!");
    let char_matrix = parse_input(&input);

    let mut sum: u32 = 0;
    for (row_i, row) in char_matrix.iter().enumerate() {
        let mut current_number: Vec<u32> = Vec::new();
        let mut is_part: bool = false;
        for (col_i, col) in row.iter().enumerate() {
            // if we found a numeric number, we add it to our current number
            // and check if this char is a part number
            if col.is_numeric() {
                current_number.push(col.to_string().parse().unwrap());
                is_part = is_part || is_part_number(row_i, col_i, &char_matrix);
            }
            // if we hit the end of a number or the end of the row and the current number
            // is a part number, then we calculate a gear ration.
            if (!col.is_numeric() || col_i == row.len() - 1) && current_number.len() > 0 {
                if is_part {
                    sum += numerical_value(&current_number);
                }
                // clear
                current_number = Vec::new();
                is_part = false;
            }
        }
    }
    sum
}

// check if the element at row/col index is a part number
// a part number is a char where one of the 8 touching squares around it is a symbol
fn is_part_number(row: usize, col: usize, matrix: &Vec<Vec<char>>) -> bool {
    let rows = matrix.len();
    let cols = matrix[0].len();

    // top
    if row != 0 {
        //middle
        if is_symbol(&matrix[row - 1][col]) {
            return true;
        }

        //left
        if col != 0 {
            if is_symbol(&matrix[row - 1][col - 1]) {
                return true;
            }
        }
        //right
        if col + 1 != cols {
            if is_symbol(&matrix[row - 1][col + 1]) {
                return true;
            }
        }
    }

    // left
    if col != 0 {
        if is_symbol(&matrix[row][col - 1]) {
            return true;
        }
    }

    //right
    if col + 1 != cols {
        if is_symbol(&matrix[row][col + 1]) {
            return true;
        }
    }

    // bottom
    if row + 1 != rows {
        //middle
        if is_symbol(&matrix[row + 1][col]) {
            return true;
        }
        //left
        if col != 0 {
            if is_symbol(&matrix[row + 1][col - 1]) {
                return true;
            }
        }
        //right
        if col + 1 != cols {
            if is_symbol(&matrix[row + 1][col + 1]) {
                return true;
            }
        }
    }

    return false;
}

// the charachter is considered a symbol if it is
// not numeric or a period.
fn is_symbol(character: &char) -> bool {
    if character.is_numeric() {
        return false;
    }
    if character.to_string() == "." {
        return false;
    }
    return true;
}
