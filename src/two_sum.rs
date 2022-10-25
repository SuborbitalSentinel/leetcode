fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return Vec::from([i as i32, j as i32]);
            }
        }
    }
    panic!();
}

#[cfg(test)]
mod tests {
    use crate::two_sum::two_sum;

    #[test]
    fn base_test() {
        let nums = Vec::from([2, 7, 11, 13]);
        let target = 9;
        assert_eq!(two_sum(nums, target), Vec::from([0, 1]));
    }
}
