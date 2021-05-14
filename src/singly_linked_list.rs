// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// for tests
pub fn new_list(items: &Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;

    items.iter().rev().for_each(|item| {
        let node = Box::new(ListNode {
            val: *item,
            next: head.take(),
        });
        head = Some(node);
    });

    head
}

pub fn print_list(head: &Option<Box<ListNode>>) -> String {
    let mut head = head;
    let mut s = String::new();
    s.push('[');
    if let Some(node) = head {
        s.push_str(&node.val.to_string());
        head = &node.next;
    }
    while let Some(node) = head {
        s.push(',');
        s.push_str(&node.val.to_string());
        head = &node.next;
    }
    s.push(']');
    s
}
