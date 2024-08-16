use core::cell::RefCell;
use std::rc::Rc;
use super::list::ListNode;

// O(n) time | O(1) space
pub fn middle_node(head: &Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {

    let mut fast = head.as_ref().and_then(|x| x.borrow().next.as_ref().map(Rc::clone));
    let mut slow = head.as_ref().map(Rc::clone);

    while fast.is_some() && fast.as_ref().map(|x| x.borrow().next.as_ref().map(Rc::clone)).is_some() {

        fast = fast.and_then(|x| x.borrow().next.as_ref().map(Rc::clone))
            .and_then(|x| x.borrow().next.as_ref().map(Rc::clone));

        slow = slow.and_then(|x| x.borrow().next.as_ref().map(Rc::clone));
    }

    slow
}

#[test]
pub fn test_middle_node() {
    use crate::list::list::DisplayableListNode;

    let head = Some(ListNode::with_next(4, Some(ListNode::with_next(8, Some(ListNode::with_next(15, Some(ListNode::new(19))))))));
    let exp = Some(ListNode::with_next(15, Some(ListNode::new(19))));

    let res = middle_node(&head);
    if let Some(display_res) = DisplayableListNode::from_option(res.clone()) {
        println!("{}", display_res);
    }
    assert_eq!(middle_node(&head), exp);



    let head1 = Some(ListNode::with_next(1, Some(ListNode::with_next(2, Some(ListNode::with_next(3, Some(ListNode::with_next(4, Some(ListNode::new(5))))))))));
    let exp1 = Some(ListNode::with_next(3, Some(ListNode::with_next(4, Some(ListNode::new(5))))));

    let res1 = middle_node(&head1);
    if let Some(display_res) = DisplayableListNode::from_option(res1.clone()) {
        println!("{}", display_res);
    }
    assert_eq!(middle_node(&head1), exp1);
}