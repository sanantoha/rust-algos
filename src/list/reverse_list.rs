use crate::list::list::ListNode;

// O(n) time | O(1) space
pub fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

    let mut prev = None;
    let mut curr = head;

    while let Some(mut node) = curr {
        let next = node.next;
        node.next = prev;
        prev = Some(node);
        curr = next;
    }

    return prev;
}