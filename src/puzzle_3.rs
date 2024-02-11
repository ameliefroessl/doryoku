use puzzles::utils::read_file;

// main entrypoint to puzzle_3
pub fn run(input_path: &str) {
    let answer = solve(input_path);
    print!("The answer for puzzle 3 is: {answer}.");
}

// solve the puzzle
// this puzzle tries to solve the puzzle outlined in AOC 2023 day 3

// the idea is to iterate over each element in the matrix. As soon as we hit a number char,
// we start testing to see if the char is a part number (one of its neighbors is a symbol),
// we continue testing for precending char in the row until we've reached the end of a number.
// if the number (possible stretching across multiple indexes) is a part_number we add it to
// the sum, otherwise we drop it.
fn solve(input_path: &str) -> u32 {
    let input: String = read_file(input_path);
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
                    let mut numerical_value: u32 = 0;
                    for (index, digit) in current_number.iter().enumerate() {
                        let invers_index = (current_number.len() as u32 - 1) - (index as u32);
                        let power = 10_u32.pow(invers_index);
                        let mul = power * digit;
                        numerical_value += mul;
                    }

                    sum += numerical_value;
                }
                // clear
                current_number = Vec::new();
                is_part = false;
            }
        }
    }
    sum
}

// convert the input string to a sparse matrix of the input char
fn parse_input(input: &str) -> Vec<Vec<char>> {
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
