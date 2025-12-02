mod file_parser;
use crate::{file_parser::{FileParser, FileParserTrait}, id_range::IdRange};

mod id_range;
// use crate::id_range::id_range;

pub fn solve(input_file: &str) -> (u64, u64) {
    let file_contents = FileParser::new(input_file).parse_delimeted();

    let mut id_range_vector: Vec<IdRange> = Vec::new();

    for range_string in file_contents {
        id_range_vector.push(IdRange::new(range_string.as_str()))
    }

    let mut result = 0;
    for id_range in id_range_vector {
        for invalid_id in id_range.get_invalid_ids() {
            result += invalid_id
        }
    }

    (result, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let (part_1, _part_2) = solve("data/example.txt");
        assert_eq!(part_1, 1227775554);
    }

    #[test]
    fn actual() {
        let (part_1, _part_2) = solve("data/input.txt");
        assert_eq!(part_1, 13919717792);
    }
}
