use std::cell::RefCell;
use std::rc::Rc;
use crate::list::ListNode;

pub fn remove_nth_from_end(head: Option<Rc<RefCell<ListNode>>>, n: i32) -> Option<Rc<RefCell<ListNode>>> {
    None
}


#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::list::{Displayable, ListNode};
    use super::remove_nth_from_end;

    #[test]
    fn test_remove_nth_from_end_case1() {
        let head = ListNode::with_next(1,
                                       Some(ListNode::with_next(2,
                                                                Some(ListNode::with_next(3,
                                                                                         Some(ListNode::with_next(4,
                                                                                                                  Some(ListNode::new(5)))))))));


        let exp_list = ListNode::with_next(1,
                                           Some(ListNode::with_next(2,
                                                                    Some(ListNode::with_next(3,
                                                                                             Some(ListNode::new(4)))))));

        let res = remove_nth_from_end(Some(head), 1);

        if let Some(display) = Displayable::from_option(res.as_ref().map(Rc::clone)) {
            println!("{}", display);
        }

        assert_eq!(res, Some(exp_list));
    }

    #[test]
    fn test_remove_nth_from_end_case2() {
        let head = ListNode::with_next(1,
                                       Some(ListNode::with_next(2,
                                                                Some(ListNode::with_next(3,
                                                                                         Some(ListNode::with_next(4,
                                                                                                                  Some(ListNode::new(5)))))))));


        let exp_list = ListNode::with_next(1,
                                           Some(ListNode::with_next(2,
                                                                    Some(ListNode::with_next(3,
                                                                                             Some(ListNode::new(5)))))));

        let res = remove_nth_from_end(Some(head), 2);

        if let Some(display) = Displayable::from_option(res.as_ref().map(Rc::clone)) {
            println!("{}", display);
        }

        assert_eq!(res, Some(exp_list));
    }

    #[test]
    fn test_remove_nth_from_end_case3() {
        let head = ListNode::with_next(1,
                                       Some(ListNode::with_next(2,
                                                                Some(ListNode::with_next(3,
                                                                                         Some(ListNode::with_next(4,
                                                                                                                  Some(ListNode::new(5)))))))));


        let exp_list = ListNode::with_next(1,
                                           Some(ListNode::with_next(2,
                                                                    Some(ListNode::with_next(4,
                                                                                             Some(ListNode::new(5)))))));

        let res = remove_nth_from_end(Some(head), 3);

        if let Some(display) = Displayable::from_option(res.as_ref().map(Rc::clone)) {
            println!("{}", display);
        }

        assert_eq!(res, Some(exp_list));
    }

    #[test]
    fn test_remove_nth_from_end_case4() {
        let head = ListNode::with_next(1,
                                       Some(ListNode::with_next(2,
                                                                Some(ListNode::with_next(3,
                                                                                         Some(ListNode::with_next(4,
                                                                                                                  Some(ListNode::new(5)))))))));


        let exp_list = ListNode::with_next(1,
                                           Some(ListNode::with_next(3,
                                                                    Some(ListNode::with_next(4,
                                                                                             Some(ListNode::new(5)))))));

        let res = remove_nth_from_end(Some(head), 4);

        if let Some(display) = Displayable::from_option(res.as_ref().map(Rc::clone)) {
            println!("{}", display);
        }

        assert_eq!(res, Some(exp_list));
    }

    #[test]
    fn test_remove_nth_from_end_case5() {
        let head = ListNode::with_next(1,
                                       Some(ListNode::with_next(2,
                                                                Some(ListNode::with_next(3,
                                                                                         Some(ListNode::with_next(4,
                                                                                                                  Some(ListNode::new(5)))))))));


        let exp_list = ListNode::with_next(2,
                                           Some(ListNode::with_next(3,
                                                                    Some(ListNode::with_next(4,
                                                                                             Some(ListNode::new(5)))))));

        let res = remove_nth_from_end(Some(head), 5);

        if let Some(display) = Displayable::from_option(res.as_ref().map(Rc::clone)) {
            println!("{}", display);
        }

        assert_eq!(res, Some(exp_list));
    }

    #[test]
    fn test_remove_nth_from_end_case6() {
        let head = ListNode::with_next(1,
                                       Some(ListNode::with_next(2,
                                                                Some(ListNode::with_next(3,
                                                                                         Some(ListNode::with_next(4,
                                                                                                                  Some(ListNode::new(5)))))))));


        let exp_list = ListNode::with_next(1,
                                           Some(ListNode::with_next(2,
                                                                    Some(ListNode::with_next(3,
                                                                                             Some(ListNode::with_next(4,
                                                                                                                      Some(ListNode::new(5)))))))));

        let res = remove_nth_from_end(Some(head), 6);

        if let Some(display) = Displayable::from_option(res.as_ref().map(Rc::clone)) {
            println!("{}", display);
        }

        assert_eq!(res, Some(exp_list));
    }
}