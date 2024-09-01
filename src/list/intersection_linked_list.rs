use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;
use super::ListNode;

// O(l + r) time | O(l) space
pub fn get_intersection_node(l: &Option<Rc<RefCell<ListNode>>>, r: &Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
    if l.is_none() || r.is_none() {
        return None;
    }

    let mut curr_l = l.as_ref().map(Rc::clone);
    let mut set = HashSet::new();

    while let Some(left) = curr_l.take() {
        set.insert(Rc::as_ptr(&left));

        curr_l = left.borrow().next.as_ref().map(Rc::clone);
    }

    let mut curr_r = r.as_ref().map(Rc::clone);

    while let Some(right) = curr_r.take() {
        if set.contains(&Rc::as_ptr(&right)) {
            return Some(right);
        }

        curr_r = right.borrow().next.as_ref().map(Rc::clone);
    }

    None
}

// O(l + r) time | O(1) space
pub fn get_intersection_node1(l: &Option<Rc<RefCell<ListNode>>>, r: &Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
    if l.is_none() || r.is_none() {
        return None;
    }

    let mut curr_l = l.as_ref().map(Rc::clone);
    let mut curr_r = r.as_ref().map(Rc::clone);

    while let (Some(left), Some(right)) = (curr_l.take(), curr_r.take()) {
        if Rc::ptr_eq(&left, &right) {
            return Some(left);
        }

        curr_l = left.borrow().next.as_ref().map(Rc::clone).or(r.as_ref().map(Rc::clone));
        curr_r = right.borrow().next.as_ref().map(Rc::clone).or(l.as_ref().map(Rc::clone));
    }
    
    None
}

#[cfg(test)]
mod tests {

    use super::get_intersection_node;
    use super::get_intersection_node1;
    use crate::list::ListNode;
    use crate::list::Displayable;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn it_get_intersection_node() {
        let (l, r) = create_lists();
        let exp_common = create_common();

        let res = get_intersection_node(&Some(l), &Some(r));

        if let Some(disp) = Displayable::from_option(res.as_ref().map(Rc::clone)) {
            println!("{}", disp);
        }
        
        assert_eq!(res, Some(exp_common));
    }

    #[test]
    fn it_get_intersection_node1() {
        let (l, r) = create_lists();
        let exp_common = create_common();

        let res = get_intersection_node1(&Some(l), &Some(r));
        
        if let Some(disp) = Displayable::from_option(res.as_ref().map(Rc::clone)) {
            println!("{}", disp);
        }
        
        assert_eq!(res, Some(exp_common));
    }

    fn create_lists() -> (Rc<RefCell<ListNode>>, Rc<RefCell<ListNode>>) {    
        let common = create_common();    
        let l = ListNode::with_next(4, Some(ListNode::with_next(1, Some(common.clone()))));
        let r = ListNode::with_next(5, Some(ListNode::with_next(6, Some(ListNode::with_next(1, Some(common.clone()))))));
        (l, r)
    }

    fn create_common() -> Rc<RefCell<ListNode>> {
        ListNode::with_next(8, Some(ListNode::with_next(4, Some(ListNode::new(5)))))
    }
}