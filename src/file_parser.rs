use std::fs::File;
use std::io::{BufRead, BufReader};

// pub fn parse_lines(path: &str) -> Vec<String> {
//     FileParser.parse_lines(path)
// }

pub trait FileParserTrait {
    fn parse_lines(&self) -> Vec<String>;
    fn parse_delimeted(&self) -> Vec<String>;
}

pub struct FileParser {
    file: File,
}

impl FileParser {
    pub fn new(path: &str) -> Self {
        let file = File::open(path).unwrap_or_else(|_| {
            panic!("Could not find file \"{path}\"");
        });

        Self { file }
    }
}

impl FileParserTrait for FileParser {
    fn parse_lines(&self) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        for line in BufReader::new(&self.file).lines() {
            lines.push(line.unwrap().clone());
        }
        lines
    }

    fn parse_delimeted(&self) -> Vec<String> {
        let mut items: Vec<String> = Vec::new();
        for item in BufReader::new(&self.file).split(b',') {
            items.push(String::from_utf8(item.unwrap().trim_ascii().to_vec()).unwrap());
        }
        items
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;

    pub mod mocks {
        use super::*;

        pub struct MockFileParser {
            pub mock_data: Vec<String>,
        }

        impl FileParserTrait for MockFileParser {
            fn parse_lines(&self) -> Vec<String> {
                self.mock_data.clone()
            }

            fn parse_delimeted(&self) -> Vec<String> {
                self.mock_data.clone()
            }
        }
    }

    #[test]
    fn mock_file_opener() {
        let mock_parser = mocks::MockFileParser {
            mock_data: vec!["one".to_string(), "two".to_string(), "three".to_string()],
        };
        let result_array = mock_parser.parse_lines();

        assert_eq!(result_array[0], "one");
        assert_eq!(result_array[1], "two");
        assert_eq!(result_array[2], "three");
    }

    #[test]
    #[should_panic(expected = "Could not find file \"not a path\"")]
    fn file_opener_bad_file() {
        FileParser::new("not a path").parse_lines();
    }

    #[test]
    fn file_opener() {
        let result_vec = FileParser::new("tests/file_opening_test.txt").parse_lines();

        assert_eq!(result_vec[0], "Here is a file");
        assert_eq!(result_vec[1], "It has stuff");
        assert_eq!(result_vec[2], "and");
        assert_eq!(result_vec[3], "Many Lines");
    }

    #[test]
    fn file_opener_single() {
        let result_vec = FileParser::new("tests/single_line_file.txt").parse_lines();

        assert_eq!(result_vec[0], "This file has one line");
    }

    #[test]
    #[should_panic(expected = "Could not find file \"tests/non_open_permission.txt\"")]
    fn file_permission_issue() {
        FileParser::new("tests/non_open_permission.txt").parse_lines();
    }

    #[test]
    fn single_line_comma() {
        let result_vec = FileParser::new("tests/comma_delimited.txt").parse_delimeted();
        assert_eq!(result_vec[0], "one");
        assert_eq!(result_vec[1], "two");
        assert_eq!(result_vec[2], "three");
        assert_eq!(result_vec[3], "four");
    }
}
