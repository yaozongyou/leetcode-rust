// https://leetcode.com/problems/reverse-linked-list/

use leetcode::singly_linked_list::new_list;
use leetcode::singly_linked_list::print_list;
use leetcode::singly_linked_list::ListNode;

struct Solution;

impl Solution {
    pub fn reverse_list_1(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head = None;

        while let Some(mut node) = head {
            head = node.next;
            node.next = new_head;
            new_head = Some(node);
        }

        new_head
    }

    pub fn reverse_list_2(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;

        while let Some(mut curr) = head.take() {
            let next = curr.next.take();
            curr.next = prev.take();
            prev = Some(curr);
            head = next;
        }

        prev
    }
}

fn main() {
    let head = new_list(&vec![1, 2, 3]);
    print!("{}\n", print_list(&head));
    let result = Solution::reverse_list_1(head);
    print!("{}\n", print_list(&result));

    let head = new_list(&vec![1, 2, 3]);
    print!("{}\n", print_list(&head));
    let result = Solution::reverse_list_2(head);
    print!("{}\n", print_list(&result));
}
