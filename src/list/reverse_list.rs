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

#[test]
fn test_reverse() {

    let head: ListNode = ListNode::with_next(1,
                                             Some(Box::new(ListNode::with_next(2,
                                                                               Some(Box::new(ListNode::with_next(3,
                                                                                                                 Some(Box::new(ListNode::with_next(4,
                                                                                                                                                   Some(Box::new(ListNode::new(5)))))))))))));


    let exp_list: ListNode = ListNode::with_next(5,
                                                 Some(Box::new(ListNode::with_next(4,
                                                                                   Some(Box::new(ListNode::with_next(3,
                                                                                                                     Some(Box::new(ListNode::with_next(2,
                                                                                                                                                       Some(Box::new(ListNode::new(1)))))))))))));

    assert_eq!(reverse(Some(Box::new(head))), Some(Box::new(exp_list)));
}