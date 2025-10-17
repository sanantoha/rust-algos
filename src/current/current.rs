use std::cell::RefCell;
use std::rc::Rc;
use crate::list::ListNode;

pub fn is_cycle(root: &Option<Rc<RefCell<ListNode>>>) -> bool {
    false
}

pub fn is_cycle2(root: &Rc<RefCell<ListNode>>) -> bool {
    false
}


pub fn is_cycle_ws(root: &Option<Rc<RefCell<ListNode>>>) -> bool {
    false
}


pub fn is_cycle_ws2(root: &Rc<RefCell<ListNode>>) -> bool {
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
    pub fn test_is_not_cycle() {
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