use std::cell::RefCell;
use std::rc::Rc;
use crate::list::ListNode;

// O(n) time | O(1) space
pub fn remove_nth_from_end(head: Option<Rc<RefCell<ListNode>>>, n: i32) -> Option<Rc<RefCell<ListNode>>> {
    if head.is_none() || n < 0 {
        return None;
    }

    let mut curr_opt = head.as_ref().map(Rc::clone);

    let mut k = n;

    while let Some(curr) = curr_opt.take() {
        k -= 1;
        curr_opt = curr.borrow().next.as_ref().map(Rc::clone);
        if k == 0 {
            break;
        }
    }

    if k > 0 {
        return head;
    }
    if curr_opt.is_none() {
        if k == 0 {
            return head.as_ref().and_then(|x| x.borrow().next.as_ref().map(Rc::clone));
        }
        return head;
    }

    let mut fst_opt = curr_opt;
    let mut snd_opt = head.as_ref().map(Rc::clone);

    while let Some(fst) = fst_opt.take() {
        if fst.borrow().next.is_none() {
            break;
        }
        fst_opt = fst.borrow().next.as_ref().map(Rc::clone);
        if let Some(snd) = snd_opt.take() {
            snd_opt = snd.borrow().next.as_ref().map(Rc::clone);
        }
    }

    let next_opt = snd_opt.as_ref().and_then(|x| x.borrow().next.as_ref().map(Rc::clone));

    if let Some(next) = next_opt {
        if let Some(snd) = snd_opt {
            snd.borrow_mut().next = next.borrow().next.as_ref().map(Rc::clone);
        }
    }
    head
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::list::{Displayable, ListNode};
    use crate::list::remove_nth_node_from_end_of_list::remove_nth_from_end;

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