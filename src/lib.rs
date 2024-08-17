// utitility functions for the puzzles package
pub mod utils {

    use std::fs::read_to_string;

    pub fn read_file(file_path: &str) -> String {
        let contents: String =
            read_to_string(file_path).expect("Should have been able to read the file");

        // this gives ownership of the file contents to the calling function
        contents
    }

    use std::fs::File;

    // rust prelude and traits are imporant to understand here
    // https://doc.rust-lang.org/std/prelude/
    // https://doc.rust-lang.org/book/ch10-02-traits.html#defining-a-trait
    use std::io::prelude::*;

    pub fn write_matrix_to_file(matrix: &Vec<Vec<char>>, file_name: String) {
        let mut file = File::create(file_name).expect("Unable to open file.");
        for i in matrix {
            for j in i {
                write!(file, "{}", j).expect("Could not write to file.");
            }
            write!(file, "\n").expect("Could not write to file.");
        }
    }
}
