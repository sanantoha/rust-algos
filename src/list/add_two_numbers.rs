use std::cell::RefCell;
use std::rc::Rc;
use crate::list::ListNode;

// O(max(l1, l2)) time | O(max(l1, l2)) space
pub fn add_two_numbers(l1: &Option<Rc<RefCell<ListNode>>>, l2: &Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
    if l1.is_none() {
        return l2.clone();
    }
    if l2.is_none() {
        return l1.clone();
    }

    let dummy = ListNode::new(0);
    let mut curr = Rc::clone(&dummy);
    let mut carry = 0;

    let mut c1 = l1.as_ref().map(Rc::clone);
    let mut c2 = l2.as_ref().map(Rc::clone);

    while c1.is_some() || c2.is_some() {
        let val1 = c1.as_ref().map_or(0, |x| x.borrow().val);
        let val2 = c2.as_ref().map_or(0, |x| x.borrow().val);

        let sum = val1 + val2 + carry;
        let new_node = ListNode::new(sum % 10);
        carry = sum / 10;

        curr.borrow_mut().next = Some(Rc::clone(&new_node));
        curr = new_node;

        c1 = c1.and_then(|x| x.borrow().next.as_ref().map(Rc::clone));
        c2 = c2.and_then(|x| x.borrow().next.as_ref().map(Rc::clone));
    }

    if carry > 0 {
        curr.borrow_mut().next = Some(ListNode::new(carry));
    }


    return dummy.borrow().next.as_ref().map(Rc::clone);
}


#[cfg(test)]
mod tests {

    use crate::list::Displayable;
    use crate::list::ListNode;
    use super::add_two_numbers;

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