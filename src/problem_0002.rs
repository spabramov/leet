// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry: i32 = 0;
        let mut result = ListNode::new(0);
        let mut node = &mut result;
        let mut l1 = l1;
        let mut l2 = l2;
        while carry > 0 || l1.is_some() || l2.is_some() {
            let sum = carry
                + l1.as_ref().map_or(0, |node| node.val)
                + l2.as_ref().map_or(0, |node| node.val);

            carry = sum / 10;
            node.next = Some(Box::new(ListNode::new(sum % 10)));
            node = node.next.as_mut().unwrap();

            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
        }
        result.next
    }
}

fn to_list(nums: &[i32]) -> Option<Box<ListNode>> {
    nums.iter().rev().fold(None, |acc, e| {
        Some(Box::new(ListNode { val: *e, next: acc }))
    })
}

fn to_seq(mut list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut seq = vec![];

    while let Some(node) = list {
        seq.push(node.val);
        list = node.next;
    }
    seq
}

#[cfg(test)]
mod test {

    use super::*;
    use rstest::*;

    #[rstest]
    #[case(        vec![2,4,3],   vec![5,6,4], vec![7,0,8])]
    #[case(            vec![0],       vec![0], vec![0])]
    #[case(vec![9,9,9,9,9,9,9], vec![9,9,9,9], vec![8,9,9,9,0,0,0,1])]
    fn test_solution(#[case] l1: Vec<i32>, #[case] l2: Vec<i32>, #[case] expected: Vec<i32>) {
        let solution = Solution::add_two_numbers(to_list(&l1), to_list(&l2));
        assert_eq!(to_seq(solution), expected)
    }
}
