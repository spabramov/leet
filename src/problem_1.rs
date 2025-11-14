use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut cache: HashMap<i32, i32> = HashMap::new();

        for (i, num) in nums.into_iter().enumerate() {
            let comp = target - num;
            if let Some(comp) = cache.get(&comp) {
                return vec![*comp, i as i32];
            } else {
                cache.insert(num, i as i32);
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod test {
    use crate::problem_1::Solution;
    use rstest::*;

    #[rstest]
    #[case(vec![2, 7, 11, 15], 9, vec![0, 1])]
    #[case(       vec![3,2,4], 6, vec![1, 2])]
    #[case(         vec![3,3], 6, vec![0, 1])]
    fn test_1(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: Vec<i32>) {
        assert_eq!(Solution::two_sum(nums, target), expected);
    }
}
