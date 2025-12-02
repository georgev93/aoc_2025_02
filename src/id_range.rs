pub struct IdRange {
    min: u64,
    max: u64,
}

struct IdDecomposition {
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
            left,
            right,
            length,
        }
    }

    fn is_invalid(&self) -> bool {
        (self.length.is_multiple_of(2)) && (self.left == self.right)
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

    pub fn get_invalid_ids(&self) -> Vec<u64> {
        // let min = IdDecomposition::new(self.min);
        // let max = IdDecomposition::new(self.max);

        let mut ret_vec = vec![];
        for val in self.min..=self.max {
            if IdDecomposition::new(val).is_invalid() {
                ret_vec.push(val)
            }
        }
        ret_vec
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
        assert_eq!(my_range.get_invalid_ids(), vec![11, 22]);

        my_range = IdRange { min: 95, max: 115 };
        assert_eq!(my_range.get_invalid_ids(), vec![99]);

        my_range = IdRange {
            min: 998,
            max: 1012,
        };
        assert_eq!(my_range.get_invalid_ids(), vec![1010]);

        my_range = IdRange {
            min: 1188511880,
            max: 1188511890,
        };
        assert_eq!(my_range.get_invalid_ids(), vec![1188511885]);

        my_range = IdRange {
            min: 222220,
            max: 222224,
        };
        assert_eq!(my_range.get_invalid_ids(), vec![222222]);

        my_range = IdRange {
            min: 1698522,
            max: 1698528,
        };
        assert_eq!(my_range.get_invalid_ids(), vec![]);

        my_range = IdRange {
            min: 446443,
            max: 446449,
        };
        assert_eq!(my_range.get_invalid_ids(), vec![446446]);

        my_range = IdRange {
            min: 38593856,
            max: 38593862,
        };
        assert_eq!(my_range.get_invalid_ids(), vec![38593859]);
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
