use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use crate::list::ListNode;

// O(n) time | O(n) space
pub fn is_cycle(head: &Option<Rc<RefCell<ListNode>>>) -> bool {
    let mut set = HashSet::new();

    let mut current = head.as_ref().map(Rc::clone);

    while let Some(node) = current.take() {
        if set.contains(&Rc::as_ptr(&node)) {
            return true;
        }
        set.insert(Rc::as_ptr(&node));

        current = node.borrow().next.as_ref().map(Rc::clone);
    }

    false
}

// O(n) time | O(n) space
pub fn is_cycle2(head: &Rc<RefCell<ListNode>>) -> bool {
    let mut set = HashSet::new();

    let mut curr = Some(Rc::clone(head));

    while let Some(node) = curr.take() {
        if set.contains(&Rc::as_ptr(&node)) {
            return true;
        }
        set.insert(Rc::as_ptr(&node));

        if let Some(next) = node.borrow().next.as_ref() {
            curr = Some(Rc::clone(next))
        }
    }
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

// O(n) time | O(1) space
pub fn is_cycle_ws2(head: &Rc<RefCell<ListNode>>) -> bool {

    let mut fast = head.borrow().next.as_ref().map(Rc::clone);
    let mut slow = Some(Rc::clone(head));

    while let (Some(fnode), Some(snode)) = (fast.take(), slow.take()) {
        if Rc::ptr_eq(&fnode, &snode) {
            return true;
        }

        fast = fnode.borrow().next.as_ref().map(Rc::clone)
                    .and_then(|x| x.borrow().next.as_ref().map(Rc::clone));
        slow = snode.borrow().next.as_ref().map(Rc::clone);        
    }

    false
}

#[cfg(test)]
mod tests {

    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::list::ListNode;
    use super::{is_cycle, is_cycle2, is_cycle_ws, is_cycle_ws2};
    
    #[test]
    pub fn test_is_cycle_if_circle() {
        let root = Some(create_circle_list());                

        assert!(is_cycle(&root));                        
    }

    #[test]
    pub fn test_is_cycle_if_circle2() {
        let root = create_circle_list();                

        assert!(is_cycle2(&root));                        
    }

    #[test]
    pub fn test_is_cycle_ws_if_circle() {
        let root = Some(create_circle_list());

        assert!(is_cycle_ws(&root));
    }

    #[test]
    pub fn test_is_cycle_ws_if_circle2() {
        let root = create_circle_list();

        assert!(is_cycle_ws2(&root));
    }

    #[test]
    pub fn test_is_cycle_() {
        let root = Some(create_list());
        assert!(!is_cycle(&root));        
    }

    #[test]
    pub fn test_ic_cycle_ws() {
        let root = Some(create_list());
        assert!(!is_cycle_ws(&root));
    }

    pub fn create_circle_list() -> Rc<RefCell<ListNode>> {
        let node0 = ListNode::with_next(0, Some(ListNode::with_next(1, Some(ListNode::new(3)))));
        let node4 = ListNode::with_next(4, Some(ListNode::with_next(5, Some(ListNode::with_next(6, Some(ListNode::with_next(7, Some(Rc::clone(&node0)))))))));

        node0.borrow().next.as_ref().unwrap().borrow().next.as_ref().unwrap().borrow_mut().next = Some(Rc::clone(&node4));

        node0
    }

    pub fn create_list() -> Rc<RefCell<ListNode>> {
        ListNode::with_next(0, Some(ListNode::with_next(1, Some(ListNode::new(2)))))
    }
}