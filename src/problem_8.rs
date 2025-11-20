struct Solution;
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut iter = s.trim_start().chars().peekable();

        let sign: i32 = iter
            .next_if(|c| ['-', '+'].contains(c))
            .map(|c| if c == '-' { -1 } else { 1 })
            .unwrap_or(1);

        iter.map_while(|c| c.to_digit(10).map(|c| c as i32))
            .try_fold(0i32, |acc, d| {
                acc.checked_mul(10).and_then(|v| v.checked_add(d))
            })
            .and_then(|v| v.checked_mul(sign))
            .unwrap_or(if sign > 0 { i32::MAX } else { i32::MIN })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("42", 42)]
    #[case("   -042", -42)]
    #[case("1337c0d3", 1337)]
    #[case("0-1", 0)]
    #[case("words and 987", 0)]
    #[case("-91283472332", i32::MIN)]
    fn test_solution(#[case] s: &str, #[case] expected: i32) {
        assert_eq!(Solution::my_atoi(s.into()), expected)
    }
}
