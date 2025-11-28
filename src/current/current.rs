use std::cell::RefCell;
use std::rc::Rc;
use crate::list::ListNode;

pub fn sort(head: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
    None
}

fn merge(left: Option<Rc<RefCell<ListNode>>>, right: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
    None
}

fn get_mid(head: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
    None
}

#[cfg(test)]
mod tests {
    use crate::list::{Displayable, ListNode};
    use super::{merge, sort, get_mid};

    #[test]
    fn test_sort() {
        let head = ListNode::with_next(4,
                                       Some(ListNode::with_next(7,
                                                                Some(ListNode::with_next(1,
                                                                                         Some(ListNode::with_next(5,
                                                                                                                  Some(ListNode::with_next(3,
                                                                                                                                           Some(ListNode::new(2)))))))))));

        let exp_head = ListNode::with_next(1,
                                           Some(ListNode::with_next(2,
                                                                    Some(ListNode::with_next(3,
                                                                                             Some(ListNode::with_next(4,
                                                                                                                      Some(ListNode::with_next(5,
                                                                                                                                               Some(ListNode::new(7)))))))))));

        let res = sort(Some(head));

        if let Some(display) = Displayable::from_option(res.clone()) {
            println!("{}", display);
        }

        assert_eq!(res, Some(exp_head));
    }

    #[test]
    fn test_merge() {
        let one = ListNode::with_next(10,
                                      Some(ListNode::with_next(20,
                                                               Some(ListNode::new(30)))));

        let two = ListNode::with_next(13,
                                      Some(ListNode::with_next(15,
                                                               Some(ListNode::with_next(33,
                                                                                        Some(ListNode::new(34)))))));


        let exp = ListNode::with_next(10,
                                      Some(ListNode::with_next(13,
                                                               Some(ListNode::with_next(15,
                                                                                        Some(ListNode::with_next(20,
                                                                                                                 Some(ListNode::with_next(30,
                                                                                                                                          Some(ListNode::with_next(33,
                                                                                                                                                                   Some(ListNode::new(34)))))))))))));

        let res = merge(Some(one), Some(two));

        if let Some(display) = Displayable::from_option(res.clone()) {
            println!("{}", display);
        }

        assert_eq!(res, Some(exp));
    }

    #[test]
    fn test_get_mid() {
        let lst = ListNode::with_next(10,
                                      Some(ListNode::with_next(13,
                                                               Some(ListNode::with_next(15,
                                                                                        Some(ListNode::with_next(20,
                                                                                                                 Some(ListNode::with_next(30,
                                                                                                                                          Some(ListNode::with_next(33,
                                                                                                                                                                   Some(ListNode::new(34)))))))))))));

        let exp = ListNode::with_next(20,
                                      Some(ListNode::with_next(30,
                                                               Some(ListNode::with_next(33,
                                                                                        Some(ListNode::new(34)))))));

        let res = get_mid(Some(lst));

        if let Some(display) = Displayable::from_option(res.clone()) {
            println!("{}", display);
        }

        assert_eq!(res, Some(exp));
    }
}