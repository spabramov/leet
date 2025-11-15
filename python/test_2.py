# Definition for singly-linked list.
from typing import Optional, override

import pytest


class ListNode:
    def __init__(self, val: int = 0, next: Optional["ListNode"] = None):
        self.val = val
        self.next = next

    @override
    def __repr__(self) -> str:
        return str(to_seq(self))


class Solution:
    def addTwoNumbers(
        self, l1: ListNode | None, l2: ListNode | None
    ) -> ListNode | None:
        def val(node: ListNode | None) -> int:
            return 0 if node is None else node.val

        def nxt(node: ListNode | None) -> ListNode | None:
            return None if node is None else node.next

        def add(
            l1: ListNode | None, l2: ListNode | None, overflow: int
        ) -> ListNode | None:
            if l1 is None and l2 is None:
                return ListNode(overflow) if overflow else None

            sum = val(l1) + val(l2) + overflow

            node = ListNode(val=sum % 10)
            node.next = add(nxt(l1), nxt(l2), overflow=sum // 10)

            return node

        return add(l1, l2, 0)


def to_list(nums: list[int]) -> ListNode:
    root = ListNode(nums[0])

    node = root
    for num in nums[1:]:
        node.next = ListNode(num)
        node = node.next

    return root


def to_seq(node: ListNode | None) -> list[int]:
    if node is None:
        return []

    seq = [node.val]
    node = node.next
    while node is not None:
        seq.append(node.val)
        node = node.next

    return seq


@pytest.mark.parametrize(
    ("l1", "l2", "expected"),
    [
        ([2, 4, 3], [5, 6, 4], [7, 0, 8]),
        ([0], [0], [0]),
        ([9, 9, 9, 9, 9, 9, 9], [9, 9, 9, 9], [8, 9, 9, 9, 0, 0, 0, 1]),
    ],
)
def test_2(l1: list[int], l2: list[int], expected: list[int]) -> None:
    assert to_seq(Solution().addTwoNumbers(to_list(l1), to_list(l2))) == expected
