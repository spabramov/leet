import pytest


class Solution:
    def longestPalindrome(self, s: str) -> str:
        def is_palindrome(i: int, j: int) -> bool:
            while i < j:
                if s[i] != s[j]:
                    return False
                i += 1
                j -= 1
            return True

        result = s[0]

        for i in range(len(s)):
            for j in range(i + len(result), len(s)):
                if is_palindrome(i, j):
                    result = s[i : j + 1]
        return result


@pytest.mark.parametrize(
    ("s", "expected"),
    [
        ("babad", "bab"),
        ("cbbd", "bb"),
        ("bb", "bb"),
    ],
)
def test_solution(s: str, expected: str) -> None:
    assert Solution().longestPalindrome(s) == expected
