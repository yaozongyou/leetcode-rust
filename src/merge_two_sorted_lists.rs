pub struct Solution;

use crate::singly_linked_list::ListNode;

impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(node1), None) => Some(node1),
            (None, Some(node2)) => Some(node2),
            (Some(mut node1), Some(mut node2)) => {
                if node1.val < node2.val {
                    let n = node1.next.take();
                    node1.next = Solution::merge_two_lists(n, Some(node2));
                    Some(node1)
                } else {
                    let n = node2.next.take();
                    node2.next = Solution::merge_two_lists(Some(node1), n);
                    Some(node2)
                }
            }
            _ => None,
        }
    }

    pub fn merge_two_lists_2(mut _l1: Option<Box<ListNode>>, mut _l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // v1.next = v1.next.and_then(|n| merge_two_lists(Some(n), Some(v2)));
        unimplemented!()
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
            print_list(&Solution::merge_two_lists(new_list(vec![1, 2, 4]), new_list(vec![1, 3, 4]))),
            "[1,1,2,3,4,4]"
        );
        assert_eq!(
            print_list(&Solution::merge_two_lists(new_list(vec![]), new_list(vec![]))),
            "[]"
        );
        assert_eq!(
            print_list(&Solution::merge_two_lists(new_list(vec![]), new_list(vec![0]))),
            "[0]"
        );

        /*
        assert_eq!(
            print_list(&Solution::merge_two_lists_2(new_list(vec![1, 2, 4]), new_list(vec![1, 3, 4]))),
            "[1,1,2,3,4,4]"
        );
        assert_eq!(
            print_list(&Solution::merge_two_lists_2(new_list(vec![]), new_list(vec![]))),
            "[]"
        );
        assert_eq!(
            print_list(&Solution::merge_two_lists_2(new_list(vec![]), new_list(vec![0]))),
            "[0]"
        );
        */
    }
}
