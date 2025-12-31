use std::cell::RefCell;
use std::rc::Rc;
use crate::list::ListNode;

// O(l + r) time | O(1) space
pub fn merge(l: &Option<Rc<RefCell<ListNode>>>, r: &Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {

    None
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