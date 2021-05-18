pub struct Solution;

use crate::singly_linked_list::ListNode;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut slow_pointer = &mut dummy;
        let mut fast_pointer = &mut slow_pointer.clone();

        for _ in 0..=n {
            fast_pointer = &mut fast_pointer.as_mut().unwrap().next;
        }

        while fast_pointer.is_some() {
            fast_pointer = &mut fast_pointer.as_mut().unwrap().next;
            slow_pointer = &mut slow_pointer.as_mut().unwrap().next;
        }

        slow_pointer.as_mut().unwrap().next = slow_pointer.as_mut().unwrap().next.as_mut().unwrap().next.clone();

        dummy.unwrap().next
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::singly_linked_list::new_list;
    use crate::singly_linked_list::print_list;

    #[test]
    fn basics() {
        assert_eq!(
            print_list(&Solution::remove_nth_from_end(new_list(vec![1, 2, 3, 4, 5]), 2)),
            "[1,2,3,5]"
        );
        assert_eq!(print_list(&Solution::remove_nth_from_end(new_list(vec![1]), 1)), "[]");
        assert_eq!(print_list(&Solution::remove_nth_from_end(new_list(vec![1, 2]), 1)), "[1]");
    }
}
