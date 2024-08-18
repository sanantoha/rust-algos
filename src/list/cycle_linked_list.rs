use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use crate::list::{ListNode, ListNodeWrapper};

// O(n) time | O(n) space
pub fn is_cycle(head: &Option<Rc<RefCell<ListNode>>>) -> bool {
    let mut set = HashSet::new();

    // if let Some(curr) = head {

        let mut current = head.as_ref().map(Rc::clone);

        while let Some(node) = current {
            if set.contains(&ListNodeWrapper(Rc::clone(&node))) {
                return true;
            }
            set.insert(ListNodeWrapper(Rc::clone(&node)));

            current = node.borrow().next.as_ref().map(Rc::clone);
        }
    // }

    false
}

// O(n) time | O(1) space
pub fn is_cycle_ws(head: &Option<Rc<RefCell<ListNode>>>) -> bool {

    if let Some(curr) = head {
        let mut fast = curr.borrow().next.as_ref().map(Rc::clone);
        let mut slow = Some(Rc::clone(curr));

        while let (Some(fnode), Some(snode)) = (fast, slow) {
            if Rc::ptr_eq(&fnode, &snode) {
                return true;
            }
            slow = snode.borrow().next.as_ref().map(Rc::clone);
            if let Some(ffnode) = fnode.borrow().next.as_ref() {
                fast = ffnode.borrow().next.as_ref().map(Rc::clone);
            } else {
                return false;
            }
        }
    }
    false
}

#[test]
pub fn test_is_cycle() {

    let node0 = ListNode::with_next(0, Some(ListNode::with_next(1, Some(ListNode::new(3)))));
    let node4 = ListNode::with_next(4, Some(ListNode::with_next(5, Some(ListNode::with_next(6, Some(ListNode::with_next(7, Some(Rc::clone(&node0)))))))));

    node0.borrow().next.as_ref().unwrap().borrow().next.as_ref().unwrap().borrow_mut().next = Some(Rc::clone(&node4));

    let root = Some(Rc::clone(&node0));
    let node10 = ListNode::with_next(0, Some(ListNode::with_next(1, Some(ListNode::new(2)))));
    let root1 = Some(Rc::clone(&node10));

    assert!(is_cycle(&root));
    assert!(!is_cycle(&root1));

    assert!(is_cycle_ws(&root));
    assert!(!is_cycle_ws(&root1));
}