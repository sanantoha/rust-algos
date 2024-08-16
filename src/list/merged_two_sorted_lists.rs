use std::cell::RefCell;
use std::rc::Rc;
use crate::list::list::ListNode;

// O(l1 + l2) time | O(1) space
pub fn merge(l1: &Option<Rc<RefCell<ListNode>>>, l2: &Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {

    let mut c1 = l1.as_ref().map(Rc::clone);
    let mut c2 = l2.as_ref().map(Rc::clone);

    let dummy = ListNode::new(0);
    let mut curr = Rc::clone(&dummy);

    while c1.is_some() && c2.is_some() {
        if c1.as_ref().unwrap().borrow().val <= c2.as_ref().unwrap().borrow().val {
            let new_node = c1.as_ref().unwrap();
            curr.as_ref().borrow_mut().next = Some(Rc::clone(new_node));

            curr = Rc::clone(new_node);
            c1 = c1.and_then(|x| x.borrow().next.as_ref().map(Rc::clone))
        } else {
            let new_node = c2.as_ref().unwrap();
            curr.as_ref().borrow_mut().next = Some(Rc::clone(new_node));

            curr = Rc::clone(new_node);
            c2 = c2.and_then(|x| x.borrow().next.as_ref().map(Rc::clone))
        }
    }

    curr.as_ref().borrow_mut().next = c1.or(c2).as_ref().map(Rc::clone);

    return dummy.borrow().next.as_ref().map(Rc::clone);
}

#[test]
pub fn test_merge() {

    use crate::list::list::DisplayableListNode;

    let head1 = Some(ListNode::with_next(4, Some(ListNode::with_next(8, Some(ListNode::with_next(15, Some(ListNode::new(19))))))));
    let head2 = Some(ListNode::with_next(7, Some(ListNode::with_next(9, Some(ListNode::with_next(10, Some(ListNode::new(16))))))));

    let exp = Some(ListNode::with_next(4,
                Some(ListNode::with_next(7,
                    Some(ListNode::with_next(8,
                        Some(ListNode::with_next(9,
                            Some(ListNode::with_next(10,
                                Some(ListNode::with_next(15,
                                    Some(ListNode::with_next(16,
                                        Some(ListNode::new(19))))))))))))))));

    let res = merge(&head1, &head2);
    if let Some(display_res) = DisplayableListNode::from_option(res.clone()) {
        println!("{}", display_res);
    }
    assert_eq!(res, exp);
}