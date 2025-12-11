use aoc_2025_02::solve;

use aoc_2025_02::file_parser::FileParser;

fn main() {
    let file = FileParser::new("data/input.txt");
    let (part_1, part_2) = solve(file.get_str());
    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
