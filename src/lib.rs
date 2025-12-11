pub mod file_parser;

use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use crate::id_range::IdRange;

mod id_range;

pub fn solve(input_file: &str) -> (u64, u64) {
    let file_contents: Vec<&str> = input_file
        .trim_ascii()
        .split(',')
        .map(|s| s.trim_ascii())
        .collect();

    let mut id_range_vector: Vec<IdRange> = Vec::with_capacity(file_contents.len());

    for range_string in file_contents {
        id_range_vector.push(IdRange::new(range_string))
    }

    id_range_vector
        .par_iter()
        .map(|id_range| {
            let mut result1 = 0u64;
            let mut result2 = 0u64;
            let (invalid_ids_1, invalid_ids_2) = id_range.get_invalid_ids();
            for invalid_id in invalid_ids_1 {
                result1 += invalid_id;
            }
            for invalid_id in invalid_ids_2 {
                result2 += invalid_id;
            }
            (result1, result2)
        })
        .reduce(|| (0, 0), |(ax, ay), (bx, by)| (ax + bx, ay + by))
}

pub fn solve_pt1(input_file: &str) -> u64 {
    let (result1, _) = solve(input_file);
    result1
}
pub fn solve_pt2(input_file: &str) -> u64 {
    let (_, result2) = solve(input_file);
    result2
}

// pub fn solve_pt1(input_file: &str) -> u64 {
//     let file_contents: Vec<&str> = input_file
//         .trim_ascii()
//         .split(',')
//         .map(|s| s.trim_ascii())
//         .collect();
//
//     let mut id_range_vector: Vec<IdRange> = Vec::with_capacity(file_contents.len());
//
//     for range_string in file_contents {
//         id_range_vector.push(IdRange::new(range_string))
//     }
//
//     let result1 = Arc::new(AtomicU64::new(0));
//
//     let mut handles: Vec<std::thread::JoinHandle<()>> = Vec::with_capacity(id_range_vector.len());
//     for id_range in id_range_vector {
//         let result1_clone = Arc::clone(&result1);
//         let handle = thread::spawn(move || {
//             let (invalid_ids_1, _invalid_ids_2) = id_range.get_invalid_ids();
//             for invalid_id in invalid_ids_1 {
//                 result1_clone.fetch_add(invalid_id, Ordering::SeqCst);
//             }
//         });
//         handles.push(handle);
//     }
//
//     for handle in handles {
//         handle.join().expect("Thread panicked!");
//     }
//
//     result1.load(Ordering::Acquire)
// }
//
// pub fn solve_pt2(input_file: &str) -> u64 {
//     let file_contents: Vec<&str> = input_file
//         .trim_ascii()
//         .split(',')
//         .map(|s| s.trim_ascii())
//         .collect();
//
//     let mut id_range_vector: Vec<IdRange> = Vec::with_capacity(file_contents.len());
//
//     for range_string in file_contents {
//         id_range_vector.push(IdRange::new(range_string))
//     }
//
//     let result2 = Arc::new(AtomicU64::new(0));
//
//     let mut handles: Vec<std::thread::JoinHandle<()>> = Vec::with_capacity(id_range_vector.len());
//     for id_range in id_range_vector {
//         let result2_clone = Arc::clone(&result2);
//         let handle = thread::spawn(move || {
//             let (_invalid_ids_1, invalid_ids_2) = id_range.get_invalid_ids();
//             for invalid_id in invalid_ids_2 {
//                 result2_clone.fetch_add(invalid_id, Ordering::SeqCst);
//             }
//         });
//         handles.push(handle);
//     }
//
//     for handle in handles {
//         handle.join().expect("Thread panicked!");
//     }
//
//     result2.load(Ordering::Acquire)
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_parser::FileParser;

    #[test]
    fn example() {
        let file = FileParser::new("data/example.txt");
        let (part_1, part_2) = solve(file.get_str());
        assert_eq!(part_1, 1227775554);
        assert_eq!(part_2, 4174379265);
    }

    #[test]
    fn example_pts() {
        let my_file = FileParser::new("data/example.txt");
        assert_eq!(solve_pt1(my_file.get_str()), 1227775554);
        assert_eq!(solve_pt2(my_file.get_str()), 4174379265);
    }

    #[test]
    fn actual() {
        let file = FileParser::new("data/input.txt");
        let (part_1, part_2) = solve(file.get_str());
        assert_eq!(part_1, 13919717792);
        assert_eq!(part_2, 14582313461);
    }

    #[test]
    fn actual_pts() {
        let my_file = FileParser::new("data/input.txt");
        assert_eq!(solve_pt1(my_file.get_str()), 13919717792);
        assert_eq!(solve_pt2(my_file.get_str()), 14582313461);
    }
}
