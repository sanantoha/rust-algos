use std::cell::RefCell;
use std::rc::Rc;
use crate::list::ListNode;

// O(n * log(n)) time | O(1) space
pub fn sort(head: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
    if head.is_none() || head.as_ref().and_then(|x| x.borrow().next.as_ref().map(Rc::clone)).is_none() {
        return head;
    }

    let mid = get_mid(head.as_ref().map(Rc::clone));
    let right = sort(mid);
    let left = sort(head);

    merge(left, right)
}

fn merge(left: Option<Rc<RefCell<ListNode>>>, right: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {

    let dummy = Some(ListNode::new(0));

    let mut curr_opt = dummy.as_ref().map(Rc::clone);
    let mut left_opt = left.as_ref().map(Rc::clone);
    let mut right_opt = right.as_ref().map(Rc::clone);

    while left_opt.is_some() && right_opt.is_some() {
        if let Some(curr) = curr_opt.as_ref().map(Rc::clone) {
            if let (Some(l), Some(r)) = (left_opt.as_ref().map(Rc::clone), right_opt.as_ref().map(Rc::clone)) {
                if l.borrow().val <= r.borrow().val {
                    curr.borrow_mut().next = Some(Rc::clone(&l));
                    left_opt = l.borrow().next.as_ref().map(Rc::clone);
                } else {
                    curr.borrow_mut().next = Some(Rc::clone(&r));
                    right_opt = r.borrow().next.as_ref().map(Rc::clone);
                }
            }
            curr_opt = curr.borrow().next.as_ref().map(Rc::clone);
        }
    }

    if let Some(curr) = curr_opt {
        if let Some(l) = left_opt {
            curr.borrow_mut().next = Some(Rc::clone(&l));
        }
        if let Some(r) = right_opt {
            curr.borrow_mut().next = Some(Rc::clone(&r));
        }
    }

    dummy.as_ref().and_then(|x| x.borrow().next.as_ref().map(Rc::clone))
}

fn get_mid(head: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
    let mut fast_opt = head.as_ref().and_then(|x| x.borrow().next.as_ref().map(Rc::clone));
    let mut slow_opt = head.as_ref().map(Rc::clone);

    while let Some(curr) = fast_opt.take() {
        let next_opt = curr.borrow().next.as_ref().map(Rc::clone);
        if let Some(next) = next_opt {
            if next.borrow().next.is_none() {
                break;
            }

            fast_opt = next.borrow().next.as_ref().map(Rc::clone);
            if let Some(slow) = slow_opt.take() {
                slow_opt = slow.borrow().next.as_ref().map(Rc::clone);
            }
        }
    }

    let next = slow_opt.as_ref().and_then(|x| x.borrow().next.as_ref().map(Rc::clone));
    if let Some(slow) = slow_opt {
        slow.borrow_mut().next = None;
    }
    next
}

#[cfg(test)]
mod tests {
    use crate::list::{Displayable, ListNode};
    use crate::list::sort_list::{merge, sort};

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
}