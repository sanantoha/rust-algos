use crate::list::ListNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn add_two_numbers(l1: &Option<Rc<RefCell<ListNode>>>, l2: &Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
    None
}


#[cfg(test)]
mod tests {
    use super::add_two_numbers;
    use crate::list::Displayable;
    use crate::list::ListNode;

    #[test]
    pub fn test_add_two_numbers() {


        let l1 = Some(ListNode::with_next(1, Some(ListNode::with_next(0, Some(ListNode::with_next(9, Some(ListNode::new(9))))))));
        let l2 = Some(ListNode::with_next(7, Some(ListNode::with_next(3, Some(ListNode::new(2))))));

        let exp = Some(ListNode::with_next(8, Some(ListNode::with_next(3, Some(ListNode::with_next(1, Some(ListNode::with_next(0, Some(ListNode::new(1))))))))));

        let res = add_two_numbers(&l1, &l2);
        if let Some(disp_list_node) = Displayable::from_option(res.clone()) {
            println!("{}", disp_list_node);
        }
        assert_eq!(res, exp);
    }
}