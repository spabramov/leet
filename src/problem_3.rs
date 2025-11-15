use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut cache: HashMap<char, usize> = HashMap::new();
        let mut max = 0;
        let mut cur = 0;
        for (idx, c) in s.chars().enumerate() {
            let pos = cache.insert(c, idx);
            if let Some(pos) = pos {
                if idx - pos > cur {
                    // if current charactes has its duplicate way back at the begining
                    cur += 1;
                }
                max = max.max(cur);
                cur = cur.min(idx - pos);
            } else {
                cur += 1;
            }
        }
        max.max(cur) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("abcabcbb", 3)]
    #[case("bbbbb", 1)]
    #[case("pwwkew", 3)]
    #[case("abc", 3)]
    #[case("aab", 2)]
    #[case("cdd", 2)]
    #[case("abba", 2)]
    #[case("ohomm", 3)]
    #[case("tmmzuxt", 5)]
    #[case("swwkww", 2)]
    fn test_solution(#[case] s: &str, #[case] expected: i32) {
        assert_eq!(
            Solution::length_of_longest_substring(s.to_string()),
            expected
        )
    }
}
