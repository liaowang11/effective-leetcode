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

struct Solution {}
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (&l1, &l2);
        let mut sum = 0;
        let mut result = None;
        let mut p: &mut Option<Box<ListNode>> = &mut result;
        loop {
            sum = match (l1, l2) {
                (Some(node1), Some(node2)) => {
                    l1 = &node1.next;
                    l2 = &node2.next;
                    node1.val + node2.val + sum
                }
                (Some(node1), None) => {
                    l1 = &node1.next;
                    node1.val + sum
                }
                (None, Some(node2)) => {
                    l2 = &node2.next;
                    node2.val + sum
                }
                (None, None) => {
                    break;
                }
            };
            *p = Some(Box::new(ListNode::new(sum % 10)));
            p = &mut ((*p).as_mut().unwrap()).next;
            sum /= 10;
        }
        if sum != 0 {
            *p = Some(Box::new(ListNode::new(sum)));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {}
}
