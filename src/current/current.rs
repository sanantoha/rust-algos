use crate::list::ListNode;
use std::cell::RefCell;
use std::rc::Rc;


pub fn delete_node(node: Rc<RefCell<ListNode>>) {

}

#[cfg(test)]
mod tests {
    use crate::list::ListNode;
    use crate::list::{Displayable};
    use super::delete_node;

    #[test]
    fn it_delete_node() {

        let node0 = ListNode::with_next(3, Some(ListNode::with_next(4, Some(ListNode::new(5)))));
        let node1 = ListNode::with_next(2, Some(node0.clone()));
        let lst = ListNode::with_next(1, Some(node1));

        let exp_lst = ListNode::with_next(1, Some(ListNode::with_next(2, Some(ListNode::with_next(4, Some(ListNode::new(5)))))));

        println!("{}", Displayable::new(lst.clone()));
        delete_node(node0.clone());
        println!("{}", Displayable::new(lst.clone()));

        assert_eq!(lst, exp_lst);
    }
}