struct Solution;
/*

0 1 2 3 4

0 2 4 6 8
1 3 5 7 9

0   4   8
1 3 5 7 9
2   6   10

0     6
1   5 7
2 4   8 10
3     9

n = 5
0       8   i + 2*(n-1-r)       + 2*(n-1-r) + ...
1     7 9   i + 2*(n-1-r) + 2*r + 2*(n-1-r) + ...
2   6   10  i + 2*(n-1-r) + 2*r + 2*(n-1-r) + ...
3 5     11  i + 2*(n-1-r) + 2*r + 2*(n-1-r) + ...
4       12  i             + 2*r + 2*(n-1-r) + ...

*/
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 || num_rows >= (s.len() as i32) {
            return s;
        }

        let text: Vec<_> = s.chars().collect();
        let mut output = String::with_capacity(s.len());

        let n: usize = num_rows as usize;
        for row in 0..n {
            let mut idx = row;

            let steps = [2 * (n - (1 + row)), 2 * row];
            for step in steps.into_iter().filter(|&v| v > 0).cycle() {
                output.push(text[idx]);
                idx += step;
                if idx >= text.len() {
                    break;
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("PAYPALISHIRING", 3, "PAHNAPLSIIGYIR")]
    #[case("PAYPALISHIRING", 4, "PINALSIGYAHRPI")]
    #[case("A", 1, "A")]
    #[case("A", 2, "A")]
    fn test_solution(#[case] s: &str, #[case] num_rows: i32, #[case] expected: &str) {
        assert_eq!(Solution::convert(s.into(), num_rows), expected)
    }
}
