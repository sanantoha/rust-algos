#[cfg(test)]
mod tests {
    use crate::list::list::ListNode;
    use super::super::super::super::list::reverse_list;

    #[test]
    fn test_reverse() {

        let head: ListNode = ListNode::with_next(1,
                                 Some(Box::new(ListNode::with_next(2,
                                       Some(Box::new(ListNode::with_next(3,
                                             Some(Box::new(ListNode::with_next(4,
                                                   Some(Box::new(ListNode::new(5)))))))))))));


        let exp_list: ListNode = ListNode::with_next(5,
                                 Some(Box::new(ListNode::with_next(4,
                                       Some(Box::new(ListNode::with_next(3,
                                             Some(Box::new(ListNode::with_next(2,
                                                  Some(Box::new(ListNode::new(1)))))))))))));

        assert_eq!(reverse_list::reverse(Some(Box::new(head))), Some(Box::new(exp_list)));
    }
}