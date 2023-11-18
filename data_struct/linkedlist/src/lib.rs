pub mod util;

use util::linked_list::ListNode;

pub fn has_cycle(head: Option<Box<ListNode>>) -> bool {
    let mut fast_p = &head;
    let mut slow_p = &head;
    while fast_p.is_some() && fast_p.as_ref().unwrap().next.is_some() {
        slow_p = &slow_p.as_ref().unwrap().next;
        fast_p = &fast_p.as_ref().unwrap().next.as_ref().unwrap().next;
        if slow_p == fast_p { return true; }
    }
    false
}

pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match(l1, l2) {
        (Some(node1), None) => Some(node1),
        (None, Some(node2)) => Some(node2),
        (Some(mut node1), Some(mut node2)) => {
            if node1.val < node2.val {
                let n = node1.next.take();
                node1.next = merge_two_lists(n, Some(node2));
                Some(node1)
            } else {
                let n = node2.next.take();
                node2.next = merge_two_lists(Some(node1), n);
                Some(node2)
            }
        },
        _ => None,
    }
}

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut fast_p = &head;
    let mut slow_p = &head;

    while fast_p.is_some() && fast_p.as_ref().unwrap().next.is_some() {
        slow_p = &slow_p.as_ref().unwrap().next;
        fast_p = &fast_p.as_ref().unwrap().next.as_ref().unwrap().next;
    }
    slow_p.clone()
}
