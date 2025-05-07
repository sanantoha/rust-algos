use std::cell::RefCell;
use std::rc::Rc;
use crate::list::ListNode;

pub fn get_intersection_node(l: &Option<Rc<RefCell<ListNode>>>, r: &Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
    None
}

pub fn get_intersection_node1(l: &Option<Rc<RefCell<ListNode>>>, r: &Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
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

    #[test]
    fn it_get_intersection_node2() {
        let l = ListNode::with_next(1, Some(ListNode::new(2)));
        let r = ListNode::with_next(4, Some(ListNode::new(5)));

        let res = get_intersection_node1(&Some(l), &Some(r));

        if let Some(disp) = Displayable::from_option(res.as_ref().map(Rc::clone)) {
            println!("{}", disp);
        }

        assert_eq!(res, None);
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