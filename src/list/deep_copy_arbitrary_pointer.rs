use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::ArbListNode;


// O(n) time | O(n) space
pub fn deep_copy(head: &Rc<RefCell<ArbListNode>>) -> Option<Rc<RefCell<ArbListNode>>> {

    let mut cache = HashMap::new();

    let mut curr = Some(Rc::clone(head));

    let dummy = ArbListNode::new(0);
    let mut copy_curr = Rc::clone(&dummy);

    while let Some(node) = curr.take() {
        let copy = cache.entry(Rc::as_ptr(&node)).or_insert(ArbListNode::new(node.borrow().val));
        copy.borrow_mut().arb = node.borrow().arb.as_ref().map(Rc::clone);
        copy_curr.borrow_mut().next = Some(Rc::clone(copy));
        

        copy_curr = Rc::clone(copy);
        curr = node.borrow().next.as_ref().map(Rc::clone);
    }


    let mut curr_copy = dummy.borrow().next.as_ref().map(Rc::clone);

    while let Some(copy) = curr_copy.take() {
        let arb_clone = copy.borrow().arb.as_ref().map(Rc::clone);
        if let Some(arb) = arb_clone {
            let copy_arb = cache.entry(Rc::as_ptr(&arb)).or_insert(ArbListNode::new(arb.borrow().val));
            copy.borrow_mut().arb = Some(Rc::clone(copy_arb));
        }

        curr_copy = copy.borrow().next.as_ref().map(Rc::clone);
    }

    return dummy.borrow().next.as_ref().map(Rc::clone);
}

#[cfg(test)]
mod tests {

    use super::deep_copy;
    use crate::list::ArbListNode;
    use std::{cell::RefCell, rc::Rc};
    use crate::list::Displayable;

    #[test]
    fn it_deep_copy() {
        let root = ArbListNode::new(1);
        let second = ArbListNode::new(2);
        let third = ArbListNode::new(3);
        let four = ArbListNode::new(4);
        let five = ArbListNode::new(5);

        root.borrow_mut().next = Some(Rc::clone(&second));
        second.borrow_mut().next = Some(Rc::clone(&third));
        third.borrow_mut().next = Some(Rc::clone(&four));
        four.borrow_mut().next = Some(Rc::clone(&five));
        
        second.borrow_mut().arb = Some(Rc::clone(&five));
        third.borrow_mut().arb = Some(Rc::clone(&root));
        five.borrow_mut().arb = Some(Rc::clone(&second));


        // ArbitraryListNode copy = deepCopy(root);
        let copy = deep_copy(&root);
        assert!(copy.is_some());
        if let Some(node) = copy {            
            println!("root: {}", Displayable::new(Rc::clone(&root)));
            println!("copy: {}", Displayable::new(Rc::clone(&node)));
            assert_arb_list_node(&root, &node);
        }
        
    }

    fn assert_arb_list_node(root: &Rc<RefCell<ArbListNode>>, copy: &Rc<RefCell<ArbListNode>>) {

        let mut curr = Some(Rc::clone(root));
        let mut curr_copy = Some(Rc::clone(copy));

        while let (Some(node), Some(node_copy)) = (curr.take(), curr_copy.take()) {

            assert_eq!(node.borrow().val, node_copy.borrow().val); // copied value should be the same
            assert!(!Rc::ptr_eq(&node, &node_copy)); // failed if the same pointer means copy was incorrect

            let arb = node.borrow().arb.as_ref().map(Rc::clone);
            let arb_copy = node_copy.borrow().arb.as_ref().map(Rc::clone);
            assert_eq!(arb.is_none(), arb_copy.is_none()); // if arbitrary pointer is not present it should be the same in the copy
            assert_eq!(arb.is_some(), arb_copy.is_some()); // if artibrary pointer is present it should be the same in the copy

            if let (Some(arb), Some(arb_copy)) = (arb, arb_copy) {
                assert_eq!(arb.borrow().val, arb_copy.borrow().val); // arbitrary value should be the same
                assert!(!Rc::ptr_eq(&arb, &arb_copy)); // failed if copy arbitrary pointer is the same as original
            }

            curr = node.borrow().next.as_ref().map(Rc::clone);
            curr_copy = node_copy.borrow().next.as_ref().map(Rc::clone);
        }

        assert!(curr.is_none()); // list should have the same length and ended after iterate all node
        assert!(curr_copy.is_none()); // copy list should also ended after iteration because main list and copy should have the same amount of nodes

    }
}