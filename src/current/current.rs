use std::cell::RefCell;
use std::rc::Rc;
use crate::list::ListNode;

// O(n * log(v)) time | O(1) space
fn insert_greatest_common_divisors(head: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
    None
}

fn gcd(x: i32, y: i32) -> i32 {
    0
}


#[cfg(test)]
mod tests {
    use crate::list::{Displayable, ListNode};
    use super::{gcd, insert_greatest_common_divisors};

    #[test]
    fn test_insert_greatest_common_divisors() {

        let head = ListNode::with_next(18, Some(ListNode::with_next(6, Some(ListNode::with_next(10, Some(ListNode::new(3)))))));
        let exp_head = Some(
            ListNode::with_next(18,
                                Some(ListNode::with_next(6,
                                                         Some(ListNode::with_next(6,
                                                                                  Some(ListNode::with_next(2,
                                                                                                           Some(ListNode::with_next(10,
                                                                                                                                    Some(ListNode::with_next(1,
                                                                                                                                                             Some(ListNode::new(3)))))))))))))
        );

        println!("{:?}", gcd(6, 10));
        let res = insert_greatest_common_divisors(Some(head));
        // 18 -> 6 -> 6 -> 2 -> 10 -> 1 -> 3 -> null
        if let Some(disp_list_node) = Displayable::from_option(res.clone()) {
            println!("{}", disp_list_node);
        }
        assert_eq!(res, exp_head);
    }
}