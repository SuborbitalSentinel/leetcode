use std::collections::HashSet;

pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let expected_total: i32 = (1..(nums.len() + 1) as i32).sum();
    let mut duplicate = -1;
    let mut aggregator = 0;
    let mut set = HashSet::new();
    for value in nums {
        if set.contains(&value) {
            duplicate = value;
        } else {
            set.insert(value);
            aggregator += value;
        }
    }
    return Vec::from([duplicate, (expected_total - aggregator)]);
}

#[cfg(test)]
mod tests {
    use crate::set_mismatch::find_error_nums;

    #[test]
    fn example_test() {
        let input = Vec::from([1, 2, 2, 4]);
        let output = Vec::from([2, 3]);
        assert_eq!(find_error_nums(input), output);
    }

    #[test]
    fn failed_test1() {
        let input = Vec::from([3, 2, 2]);
        let output = Vec::from([2, 1]);
        assert_eq!(find_error_nums(input), output);
    }

    #[test]
    fn failed_test2() {
        let input = Vec::from([1, 3, 3]);
        let output = Vec::from([3, 2]);
        assert_eq!(find_error_nums(input), output);
    }
}
