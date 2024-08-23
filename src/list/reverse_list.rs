use std::cell::RefCell;
use std::rc::Rc;
use crate::list::ListNode;

// O(n) time | O(1) space
pub fn reverse(head: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {

    let mut prev = None;
    let mut curr = head;

    while let Some(node) = curr {
        let next = node.borrow().next.as_ref().map(Rc::clone);
        node.borrow_mut().next = prev;
        prev = Some(node);
        curr = next;
    }

    prev
}

#[cfg(test)]
mod tests {

    use crate::list::Displayable;
    use crate::list::ListNode;
    use super::reverse;

    #[test]
    fn test_reverse() {        
        let head = ListNode::with_next(1, Some(ListNode::with_next(2, Some(ListNode::with_next(3, Some(ListNode::with_next(4, Some(ListNode::new(5)))))))));


        let exp_list = ListNode::with_next(5, Some(ListNode::with_next(4, Some(ListNode::with_next(3, Some(ListNode::with_next(2, Some(ListNode::new(1)))))))));

        let res = reverse(Some(head));
        if let Some(disp_list_node) = Displayable::from_option(res.clone()) {
            println!("{}", disp_list_node);
        }
        assert_eq!(res, Some(exp_list));
    }
}