use std::cell::RefCell;
use std::rc::Rc;
use crate::list::ListNode;

pub fn is_palindrome(root: Option<Rc<RefCell<ListNode>>>) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::is_palindrome;
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