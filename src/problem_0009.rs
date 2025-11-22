struct Solution;
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        if x == 0 {
            return true;
        }

        let len = x.ilog10();
        let digit_at = |pos: u32| x / 10_i32.pow(len - pos) % 10;

        for i in 0..(len / 2 + 1) {
            let left = digit_at(i);
            let right = digit_at(len - i);

            if left != right {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(121, true)]
    #[case(-121, false)]
    #[case(0, true)]
    #[case(10, false)]
    #[case(1000021, false)]
    fn test_solution(#[case] x: i32, #[case] expected: bool) {
        assert_eq!(Solution::is_palindrome(x), expected)
    }
}
