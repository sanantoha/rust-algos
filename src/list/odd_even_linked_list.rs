use std::rc::Rc;
use std::cell::RefCell;
use crate::list::ListNode;

// O(n) time | O(1) space
pub fn odd_even_list(root: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {

    let mut curr = root.as_ref().map(Rc::clone);

    let even_dummy = ListNode::new(0);
    let mut even_curr = Some(Rc::clone(&even_dummy));
    let odd_dummy = ListNode::new(0);
    let mut odd_curr = Some(Rc::clone(&odd_dummy));    

    let mut idx = 1;

    while let Some(node) = curr.take() {
        if idx % 2 == 0 {
            if let Some(ec) = even_curr.take() {
                ec.borrow_mut().next = Some(Rc::clone(&node));                
                even_curr = ec.borrow().next.as_ref().map(Rc::clone);
            }            
        } else if let Some(oc) = odd_curr.take() {
            oc.borrow_mut().next = Some(Rc::clone(&node));
            odd_curr = oc.borrow().next.as_ref().map(Rc::clone);
        }
        idx += 1;
        curr = node.borrow().next.as_ref().map(Rc::clone);
    }

    if let Some(ec) = even_curr.take() {
        ec.borrow_mut().next = None;   
    }

    if let Some(oc) = odd_curr.take() {
        oc.borrow_mut().next = even_dummy.borrow().next.as_ref().map(Rc::clone);
    }

    return odd_dummy.borrow().next.as_ref().map(Rc::clone);
}

// O(n) time | O(1) space
pub fn odd_even_list1(root: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
    let res = root.as_ref().map(Rc::clone);
    let mut odd = root.as_ref().map(Rc::clone);
    let mut even = root.and_then(|x| x.borrow().next.as_ref().map(Rc::clone));
    let even_head = even.as_ref().map(Rc::clone);

    while even.is_some() && even.as_ref().and_then(|x| x.borrow().next.as_ref().map(Rc::clone)).is_some() {        
        if let Some(even_node) = even.take() {
            if let Some(odd_node) = odd.take() {
                odd_node.borrow_mut().next = even_node.borrow().next.as_ref().map(Rc::clone);
                if let Some(odd_next_node) = odd_node.borrow().next.as_ref().map(Rc::clone) {
                    odd = Some(Rc::clone(&odd_next_node));            
                    even_node.borrow_mut().next = odd_next_node.borrow().next.as_ref().map(Rc::clone);
                }            
            }
    
            even = even_node.borrow().next.as_ref().map(Rc::clone);        
        }        
    }

    if let Some(odd_node) = odd.take() {
        odd_node.borrow_mut().next = even_head;
    }

    res
}

#[cfg(test)]
mod tests {

    use crate::list::{Displayable, ListNode};
    use super::odd_even_list;
    use super::odd_even_list1;

    #[test]
    fn it_odd_even_list() {
        let lst = Some(ListNode::with_next(1, Some(ListNode::with_next(2, 
                Some(ListNode::with_next(3, Some(ListNode::with_next(4, Some(ListNode::new(5))))))))));

        let exp = Some(ListNode::with_next(1, Some(ListNode::with_next(3, 
                    Some(ListNode::with_next(5, Some(ListNode::with_next(2, Some(ListNode::new(4))))))))));                

        let res = odd_even_list(lst);
        if let Some(display_res) = Displayable::from_option(res.clone()) {
            println!("{}", display_res);
        }

        assert_eq!(res, exp);
    }

    #[test]
    fn it_odd_even_list_case2() {
        let lst = Some(ListNode::with_next(2, Some(ListNode::with_next(1, 
            Some(ListNode::with_next(3, Some(ListNode::with_next(5, 
                Some(ListNode::with_next(6, Some(ListNode::with_next(4, Some(ListNode::new(7))))))))))))));

        let exp: Option<std::rc::Rc<std::cell::RefCell<ListNode>>> = Some(ListNode::with_next(2, Some(ListNode::with_next(3, 
                Some(ListNode::with_next(6, Some(ListNode::with_next(7, 
                    Some(ListNode::with_next(1, Some(ListNode::with_next(5, Some(ListNode::new(4))))))))))))));

        let res = odd_even_list(lst);
        if let Some(display_res) = Displayable::from_option(res.clone()) {
            println!("{}", display_res);
        }

        assert_eq!(res, exp);
    }

    #[test]
    fn it_odd_even_list1() {
        let lst = Some(ListNode::with_next(1, Some(ListNode::with_next(2, 
                Some(ListNode::with_next(3, Some(ListNode::with_next(4, Some(ListNode::new(5))))))))));

        let exp = Some(ListNode::with_next(1, Some(ListNode::with_next(3, 
                    Some(ListNode::with_next(5, Some(ListNode::with_next(2, Some(ListNode::new(4))))))))));                

        let res = odd_even_list1(lst);
        if let Some(display_res) = Displayable::from_option(res.clone()) {
            println!("{}", display_res);
        }

        assert_eq!(res, exp);
    }

    #[test]
    fn it_odd_even_list1_case2() {
        let lst = Some(ListNode::with_next(2, Some(ListNode::with_next(1, 
            Some(ListNode::with_next(3, Some(ListNode::with_next(5, 
                Some(ListNode::with_next(6, Some(ListNode::with_next(4, Some(ListNode::new(7))))))))))))));

        let exp: Option<std::rc::Rc<std::cell::RefCell<ListNode>>> = Some(ListNode::with_next(2, Some(ListNode::with_next(3, 
                Some(ListNode::with_next(6, Some(ListNode::with_next(7, 
                    Some(ListNode::with_next(1, Some(ListNode::with_next(5, Some(ListNode::new(4))))))))))))));

        let res = odd_even_list1(lst);
        if let Some(display_res) = Displayable::from_option(res.clone()) {
            println!("{}", display_res);
        }

        assert_eq!(res, exp);
    }
}