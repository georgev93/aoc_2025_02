pub struct IdRange {
    min: u64,
    max: u64,
}

struct IdDecomposition {
    input: String,
    left: u64,
    right: u64,
    length: usize,
}

impl IdDecomposition {
    fn new(input: u64) -> Self {
        let input = input.to_string();
        let length = input.len();
        let (left_str, right_str) = input.split_at(length.div_ceil(2));
        let left = left_str.parse::<u64>().unwrap();
        let mut right = 0;
        if length > 1 {
            right = right_str.parse::<u64>().unwrap();
        }

        Self {
            input,
            left,
            right,
            length,
        }
    }

    fn is_invalid_pt1(&self) -> bool {
        (self.length.is_multiple_of(2)) && (self.left == self.right)
    }

    fn is_invalid_pt2(&self) -> bool {
        if self.length == 1 {
            return false;
        }

        for possible_split in 0..=self.length / 2 {
            if self.length.is_multiple_of(possible_split) {
                let chunks: Vec<&str> = self
                    .input
                    .as_bytes()
                    .chunks(possible_split)
                    .map(|b| std::str::from_utf8(b).expect("Non UTF-8 chars detected"))
                    .collect();
                if chunks
                    .first()
                    .is_none_or(|first| chunks.iter().all(|s| s == first))
                {
                    return true;
                }
            }
        }
        false
    }
}

impl IdRange {
    pub fn new(input_string: &str) -> Self {
        let (input_min, input_max) = input_string.split_once("-").unwrap();
        IdRange {
            min: input_min.parse::<u64>().unwrap(),
            max: input_max.parse::<u64>().unwrap(),
        }
    }

    pub fn get_invalid_ids(&self) -> (Vec<u64>, Vec<u64>) {
        let mut ret_vec1 = vec![];
        let mut ret_vec2 = vec![];
        for val in self.min..=self.max {
            let decomp = IdDecomposition::new(val);
            if decomp.is_invalid_pt1() {
                ret_vec1.push(val);
            }
            if decomp.is_invalid_pt2() {
                ret_vec2.push(val);
                println!("{val}");
            }
        }
        (ret_vec1, ret_vec2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_string() {
        let mut my_range = IdRange::new("10-20");
        assert_eq!(my_range.min, 10);
        assert_eq!(my_range.max, 20);

        my_range = IdRange::new("0-1000");
        assert_eq!(my_range.min, 0);
        assert_eq!(my_range.max, 1000);
    }

    #[test]
    fn get_invalid_ids_example() {
        let mut my_range = IdRange { min: 11, max: 22 };
        assert_eq!(my_range.get_invalid_ids(), (vec![11, 22], vec![11, 22]));

        my_range = IdRange { min: 95, max: 115 };
        assert_eq!(my_range.get_invalid_ids(), (vec![99], vec![99, 111]));

        my_range = IdRange {
            min: 998,
            max: 1012,
        };
        assert_eq!(my_range.get_invalid_ids(), (vec![1010], vec![999, 1010]));

        my_range = IdRange {
            min: 1188511880,
            max: 1188511890,
        };
        assert_eq!(
            my_range.get_invalid_ids(),
            (vec![1188511885], vec![1188511885])
        );

        my_range = IdRange {
            min: 222220,
            max: 222224,
        };
        assert_eq!(my_range.get_invalid_ids(), (vec![222222], vec![222222]));

        my_range = IdRange {
            min: 1698522,
            max: 1698528,
        };
        assert_eq!(my_range.get_invalid_ids(), (vec![], vec![]));

        my_range = IdRange {
            min: 446443,
            max: 446449,
        };
        assert_eq!(my_range.get_invalid_ids(), (vec![446446], vec![446446]));

        my_range = IdRange {
            min: 38593856,
            max: 38593862,
        };
        assert_eq!(my_range.get_invalid_ids(), (vec![38593859], vec![38593859]));

        my_range = IdRange {
            min: 565653,
            max: 565659,
        };
        assert_eq!(my_range.get_invalid_ids(), (vec![], vec![565656]));

        my_range = IdRange {
            min: 824824821,
            max: 824824827,
        };
        assert_eq!(my_range.get_invalid_ids(), (vec![], vec![824824824]));

        my_range = IdRange {
            min: 2121212118,
            max: 2121212124,
        };
        assert_eq!(my_range.get_invalid_ids(), (vec![], vec![2121212121]));
    }

    #[test]
    fn id_decomposition() {
        let mut my_decomp = IdDecomposition::new(1001);
        assert_eq!(10, my_decomp.left);
        assert_eq!(1, my_decomp.right);
        assert_eq!(4, my_decomp.length);

        my_decomp = IdDecomposition::new(1);
        assert_eq!(1, my_decomp.left);
        assert_eq!(0, my_decomp.right);
        assert_eq!(1, my_decomp.length);
    }
}
