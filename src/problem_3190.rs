struct Solution;
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        nums.into_iter().filter(|v| *v % 3 > 0).count() as i32
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use rstest::*;

    #[rstest]
    #[case(vec![1,2,3,4], 3)]
    #[case(vec![3,6,9], 0)]
    fn test_solution(#[case] nums: Vec<i32>, #[case] expected: i32) {
        assert_eq!(Solution::minimum_operations(nums), expected)
    }
}
