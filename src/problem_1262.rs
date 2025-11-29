struct Solution;
impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let total: i32 = nums.iter().sum();
        let mut nums = nums;
        nums.sort();

        match total % 3 {
            1 => {
                let opt1 = total - nums.iter().filter(|&v| v % 3 == 2).take(2).sum::<i32>();
                let opt2 = total - nums.iter().find(|&v| v % 3 == 1).unwrap_or(&total);

                if opt1 % 3 == 0 && opt1 > opt2 {
                    opt1
                } else {
                    opt2
                }
            }
            2 => {
                let opt1 = total - nums.iter().filter(|&v| v % 3 == 1).take(2).sum::<i32>();
                let opt2 = total - nums.iter().find(|&v| v % 3 == 2).unwrap_or(&total);

                if opt1 % 3 == 0 && opt1 > opt2 {
                    opt1
                } else {
                    opt2
                }
            }
            _ => total,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(vec![3,6,5,1,8], 18)]
    #[case(vec![4], 0)]
    #[case(vec![1,2,3,4,4], 12)]
    #[case(vec![2,6,2,2,7], 15)]
    fn test_solution(#[case] nums: Vec<i32>, #[case] expected: i32) {
        assert_eq!(Solution::max_sum_div_three(nums), expected)
    }
}
