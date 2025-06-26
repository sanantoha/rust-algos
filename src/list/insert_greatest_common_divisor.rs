use std::cell::RefCell;
use std::rc::Rc;
use crate::list::ListNode;

// O(n * log(k)) time | O(1) space - where k is max sum with two numbers
pub fn insert_greatest_common_divisors(head: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {

    let mut curr_opt = head.as_ref().map(Rc::clone);

    while let Some(curr) = curr_opt.take() {
        let mut next_opt = curr.borrow().next.as_ref().map(Rc::clone);
        if let Some(next) = next_opt.take() {
            let val = curr.borrow().val;
            let next_val = next.borrow().val;
            let new_val = gcd(val, next_val);
            let node = ListNode::new(new_val);
            curr.borrow_mut().next = Some(Rc::clone(&node));
            node.borrow_mut().next = Some(Rc::clone(&next));
            curr_opt = Some(next);
        }
    }

    head
}

fn gcd(mut x: i32, mut y: i32) -> i32 {
    while y != 0 {
        let tmp = y;
        y = x % y;
        x = tmp;
    }
    x
}

#[cfg(test)]
mod tests {
    use crate::list::Displayable;
    use super::*;

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

        // println!("{:?}", gcd(6, 10));
        let res = insert_greatest_common_divisors(Some(head));
        // 18 -> 6 -> 6 -> 2 -> 10 -> 1 -> 3 -> null
        if let Some(disp_list_node) = Displayable::from_option(res.clone()) {
            println!("{}", disp_list_node);
        }
        assert_eq!(res, exp_head);
    }
}