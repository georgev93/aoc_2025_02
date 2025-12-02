mod file_parser;
use crate::file_parser::parse_file;

pub fn solve(input_file: &str) -> (u32, u32) {
    let file_contents = parse_file(input_file);
    (0, 0)
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn example() {
    //     let (part_1, _part_2) = solve("data/example.txt");
    //     assert_eq!(part_1, 1227775554);
    // }
}
