struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter().sum::<i32>() % k
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(vec![3,9,7], 5, 4)]
    #[case(vec![4,1,3], 4, 0)]
    fn test_solution(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        assert_eq!(Solution::min_operations(nums, k), expected)
    }
}
