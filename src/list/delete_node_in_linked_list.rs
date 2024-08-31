use std::rc::Rc;
use std::cell::RefCell;
use super::ListNode;

// O(n) time | O(1) space
pub fn delete_node(node: Rc<RefCell<ListNode>>) {

    let mut curr = Some(node.clone());

    while let Some(node) = curr.take() {
        let next_opt = node.borrow().next.as_ref().map(Rc::clone);
        if let Some(next) = next_opt {
            node.borrow_mut().val = next.borrow().val;

            if next.borrow().next.is_none() {
                node.borrow_mut().next = None;
            }

            curr = Some(Rc::clone(&next));
        };        
    }
}

#[cfg(test)]
mod tests {

    use super::ListNode;
    use crate::list::{delete_node_in_linked_list::delete_node, Displayable};

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