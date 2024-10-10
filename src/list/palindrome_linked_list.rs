use crate::list::ListNode;
use std::cell::RefCell;
use std::rc::Rc;

// O(n) time | O(1) space
pub fn is_palindrome(head: Option<Rc<RefCell<ListNode>>>) -> bool {
    if head.is_none() {
        return false;
    }

    let mut fst_opt = head.as_ref().map(Rc::clone);
    let mut snd_opt = reverse(get_middle(head));

    while let Some(snd) = snd_opt.take() {
        if let Some(fst) = fst_opt.take() {
            if fst.borrow().val != snd.borrow().val {
                return false;
            }

            fst_opt = fst.borrow().next.as_ref().map(Rc::clone);
            snd_opt = snd.borrow().next.as_ref().map(Rc::clone);
        } else {
            return false;
        }
    }

    true
}

fn get_middle(head: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {

    let mut fast = head.as_ref().map(Rc::clone);
    let mut slow = head.as_ref().map(Rc::clone);

    while fast.is_some() && fast.as_ref().and_then(|x| x.borrow().next.as_ref().map(Rc::clone)).is_some() {

        slow = slow.as_ref().and_then(|x| x.borrow().next.as_ref().map(Rc::clone));

        fast = fast.as_ref().and_then(|x| x.borrow().next.as_ref().map(Rc::clone))
                    .as_ref().and_then(|x| x.borrow().next.as_ref().map(Rc::clone));
    }

    slow
}

fn reverse(head: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {

    let mut curr_opt = head.as_ref().map(Rc::clone);
    let mut prev = None;

    while let Some(curr) = curr_opt.take() {
        let next = curr.borrow().next.as_ref().map(Rc::clone);
        curr.borrow_mut().next = prev;
        prev = Some(curr);
        curr_opt = next;
    }

    prev
}

#[cfg(test)]
mod tests {
    use crate::list::palindrome_linked_list::is_palindrome;
    use crate::list::ListNode;

    #[test]
    fn test_is_palindrome() {

        let head = ListNode::with_next(1,
                                            Some(ListNode::with_next(2,
                                                Some(ListNode::with_next(3,
                                                Some(ListNode::with_next(3,
                                            Some(ListNode::with_next(2,
                                       Some(ListNode::new(1)))))))))));

        assert!(is_palindrome(Some(head)));
    }

    #[test]
    fn test_is_palindrome_case1() {

        let head = ListNode::with_next(1,
                                            Some(ListNode::with_next(2,
                                                Some(ListNode::with_next(3,
                                            Some(ListNode::with_next(2,
                                       Some(ListNode::new(1)))))))));

        assert!(is_palindrome(Some(head)));
    }

    #[test]
    fn test_is_palindrome_case2() {
        assert!(!is_palindrome(None))
    }
}