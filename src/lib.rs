mod file_parser;
use std::{sync::{Arc, atomic::{AtomicU64, Ordering}}, thread};

use crate::{file_parser::{FileParser, FileParserTrait}, id_range::IdRange};

mod id_range;
// use crate::id_range::id_range;

pub fn solve(input_file: &str) -> (u64, u64) {
    let file_contents = FileParser::new(input_file).parse_delimeted();

    let mut id_range_vector: Vec<IdRange> = Vec::new();

    for range_string in file_contents {
        id_range_vector.push(IdRange::new(range_string.as_str()))
    }

    let result = Arc::new(AtomicU64::new(0));

    let mut handles: Vec<std::thread::JoinHandle<()>> = Vec::with_capacity(id_range_vector.len());
    for id_range in id_range_vector {
        let result_clone = Arc::clone(&result);
        let handle = thread::spawn(move || {
            for invalid_id in id_range.get_invalid_ids() {
                result_clone.fetch_add(invalid_id, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked!");
    }

    (result.load(Ordering::Acquire), 0)
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
