struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let text: Vec<char> = s.chars().collect();
        let expand = |mut i: usize, mut j: usize| -> usize {
            if j >= s.len() || text[i] != text[j] {
                return 0;
            }
            while text[i] == text[j] {
                if i == 0 || j == s.len() - 1 {
                    j += 2;
                    break;
                };
                i -= 1;
                j += 1;
            }
            j.saturating_sub(i).saturating_sub(1)
        };

        let mut start = 0;
        let mut len = 1.min(s.len());
        for i in 0..(s.len() - 1) {
            for j in i..=(i + 1) {
                let width = expand(i, j);
                if width > len {
                    start = j - width / 2;
                    len = width
                }
            }
        }
        text[start..(start + len)].iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("babad", "bab")]
    #[case("cbbd", "bb")]
    #[case("bb", "bb")]
    #[case("racecar", "racecar")]
    #[case("abba", "abba")]
    #[case("tattarrattat", "tattarrattat")]
    fn test_solution(#[case] s: &str, #[case] expected: &str) {
        assert_eq!(Solution::longest_palindrome(s.to_string()), expected)
    }
}
