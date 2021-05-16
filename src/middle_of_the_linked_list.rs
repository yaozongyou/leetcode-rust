// https://leetcode.com/problems/middle-of-the-linked-list/

use crate::singly_linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = &head;
        let mut slow = &head;

        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }

        slow.clone()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::singly_linked_list::new_list;

    #[test]
    fn basics() {
        assert_eq!(Solution::middle_node(None), None);
        assert_eq!(Solution::middle_node(new_list(vec![1, 2, 3, 4, 5])).unwrap().val, 3);
        assert_eq!(Solution::middle_node(new_list(vec![1, 2, 3, 4, 5, 6])).unwrap().val, 4);
    }
}
