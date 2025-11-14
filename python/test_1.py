import pytest


class Solution:
    def twoSum(self, nums: list[int], target: int) -> list[int]:
        cache: dict[int, int] = {}
        for i, n in enumerate(nums):
            if (j := cache.get(target - n)) is not None:
                return [j, i]
            cache[n] = i
        assert False


solution = Solution()


@pytest.mark.parametrize(
    ("nums", "target", "expected"),
    [
        ([2, 7, 11, 15], 9, {0, 1}),
        ([3, 2, 4], 6, {1, 2}),
        ([3, 3], 6, {0, 1}),
    ],
)
def test_1(nums: list[int], target: int, expected: list[int]) -> None:
    assert set(solution.twoSum(nums, target)) == expected
