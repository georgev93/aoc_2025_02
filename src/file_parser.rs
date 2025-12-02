use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_file(path: &str) -> Vec<String> {
    FileParser.parse(path)
}

pub trait FileParserTrait {
    fn parse(&self, path: &str) -> Vec<String>;
}

pub struct FileParser;

impl FileParser {}

impl FileParserTrait for FileParser {
    fn parse(&self, path: &str) -> Vec<String> {
        let file = File::open(path).unwrap_or_else(|_| {
            panic!("Could not find file \"{path}\"");
        });

        let reader = BufReader::new(file);

        let mut lines: Vec<String> = Vec::new();
        for line in reader.lines() {
            lines.push(line.unwrap().clone());
        }
        lines
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
            fn parse(&self, _path: &str) -> Vec<String> {
                self.mock_data.clone()
            }
        }
    }

    #[test]
    fn mock_file_opener() {
        let mock_parser = mocks::MockFileParser {
            mock_data: vec!["one".to_string(), "two".to_string(), "three".to_string()],
        };
        let result_array = mock_parser.parse("my_path");

        assert_eq!(result_array[0], "one");
        assert_eq!(result_array[1], "two");
        assert_eq!(result_array[2], "three");
    }

    #[test]
    #[should_panic(expected = "Could not find file \"not a path\"")]
    fn file_opener_bad_file() {
        FileParser.parse("not a path");
    }

    #[test]
    fn file_opener() {
        let result_vec = FileParser.parse("tests/file_opening_test.txt");

        assert_eq!(result_vec[0], "Here is a file");
        assert_eq!(result_vec[1], "It has stuff");
        assert_eq!(result_vec[2], "and");
        assert_eq!(result_vec[3], "Many Lines");
    }

    #[test]
    fn file_opener_single() {
        let result_vec = FileParser.parse("tests/single_line_file.txt");

        assert_eq!(result_vec[0], "This file has one line");
    }

    #[test]
    #[should_panic(expected = "Could not find file \"tests/non_open_permission.txt\"")]
    fn file_permission_issue() {
        FileParser.parse("tests/non_open_permission.txt");
    }
}
