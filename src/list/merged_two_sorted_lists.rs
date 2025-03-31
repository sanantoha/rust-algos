use std::cell::RefCell;
use std::rc::Rc;
use crate::list::ListNode;

// O(l + r) time | O(1) space
pub fn merge(l: &Option<Rc<RefCell<ListNode>>>, r: &Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {

    let dummy = ListNode::new(-1);
    let mut curr_opt = Some(Rc::clone(&dummy));

    let mut curr_l_opt = l.as_ref().map(Rc::clone);
    let mut curr_r_opt = r.as_ref().map(Rc::clone);

    while let (Some(curr_l), Some(curr_r)) = (curr_l_opt.as_ref().map(Rc::clone), curr_r_opt.as_ref().map(Rc::clone)) {
        if let Some(curr) = curr_opt.take() {
            if curr_l.borrow().val <= curr_r.borrow().val {
                curr.borrow_mut().next = Some(Rc::clone(&curr_l));
                curr_l_opt = curr_l.borrow().next.as_ref().map(Rc::clone);
            } else {
                curr.borrow_mut().next = Some(Rc::clone(&curr_r));
                curr_r_opt = curr_r.borrow().next.as_ref().map(Rc::clone);
            }
            curr_opt = curr.borrow().next.as_ref().map(Rc::clone)
        }
    }

    if let Some(curr) = curr_opt.take() {
        curr.borrow_mut().next = curr_l_opt.or(curr_r_opt)
    }

    let res = dummy.borrow().next.as_ref().map(Rc::clone);
    res
}


#[cfg(test)]
mod tests {

    use crate::list::Displayable;
    use crate::list::ListNode;
    use super::merge;

    #[test]
    pub fn test_merge() {

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
        if let Some(display_res) = Displayable::from_option(res.clone()) {
            println!("{}", display_res);
        }
        assert_eq!(res, exp);
    }
}