// utitility functions for the puzzles package
pub mod utils {

    use std::fs;

    pub fn read_file(file_path: &str) -> String {
        let contents: String =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        // this gives ownership of the file contents to the calling function
        contents
    }
}
