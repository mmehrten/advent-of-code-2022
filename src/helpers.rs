use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Open an input path and return a buffered reader over the contents.
/// # Arguments
///
/// * `input_path` - The string with the file to open
///
/// # Returns
///
/// A BufReader over the file contents.
fn get_buf_reader(input_path: String) -> BufReader<File> {
    // Create a buffer to read the file line by line
    let contents =
        File::open(&input_path).expect(format!("Error reading file: {}", input_path).as_str());
    let reader = BufReader::new(contents);
    reader
}

#[derive(Debug)]
pub enum InputType {
    Example,
    Challenge,
}

impl fmt::Display for InputType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Get the input array as a Vec<String> for a given AoC day and input type.
/// # Arguments
///
/// * `day` - The AoC day to run
/// * `itype` - The InputType to get
///
/// # Returns
///
/// A Vec<String> containing the test data.
pub fn get_input(day: usize, itype: InputType) -> Vec<String> {
    let input_path = format!("inputs/{}-{}.txt", itype, day);
    get_buf_reader(input_path)
        .lines()
        .map(|line| line.expect("Failed to read data from input file."))
        .collect()
}

#[cfg(test)]
mod test_input_parsing {
    use crate::get_input;
    use crate::InputType;

    #[test]
    #[should_panic]
    fn error_file_handled() {
        get_input(0, InputType::Challenge);
    }

    #[test]
    fn example_file_parsed() {
        assert_eq!(get_input(0, InputType::Example), ["OK"]);
    }
}
