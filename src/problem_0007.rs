struct Solution;
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let sign = x.signum();
        let mut rest = x.abs();
        let mut out: i32 = 0;

        while rest > 0 {
            if let Some(res) = out.checked_mul(10).and_then(|v| v.checked_add(rest % 10)) {
                out = res;
            } else {
                return 0;
            }
            rest /= 10;
        }
        sign * out
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(123, 321)]
    #[case(-123, -321)]
    #[case(120, 21)]
    #[case(120, 21)]
    #[case(2063847413, 0)]
    fn test_solution(#[case] x: i32, #[case] expected: i32) {
        assert_eq!(Solution::reverse(x), expected)
    }
}
