use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::graph::Node;

// O(E + V) time | O(V) space
pub fn clone_graph(node: Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {

    let mut cache = HashMap::new();
    cache.insert(Rc::as_ptr(&node), Rc::new(RefCell::new(Node::new(node.borrow().val))));

    let mut stack = vec![Rc::clone(&node)];

    while let Some(orig_node) = stack.pop() {
        let copy_node_opt = cache.get(&Rc::as_ptr(&orig_node));
        if let Some(copy_node) = copy_node_opt.map(Rc::clone) {

            for child in orig_node.borrow().neighbors.iter() {
                let copy_child_opt = cache.get(&Rc::as_ptr(child));
                if let Some(copy_child) = copy_child_opt.map(Rc::clone) {
                    copy_node.borrow_mut().neighbors.push(Rc::clone(&copy_child));
                } else {
                    let copy_child = Rc::new(RefCell::new(Node::new(child.borrow().val)));
                    cache.insert(Rc::as_ptr(child), Rc::clone(&copy_child));
                    stack.push(Rc::clone(child));
                    copy_node.borrow_mut().neighbors.push(Rc::clone(&copy_child));
                }
            }
        }
    }

    cache.get(&Rc::as_ptr(&node)).map(Rc::clone)
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::graph::clone_graph::clone_graph;
    use crate::graph::Node;

    #[test]
    fn test_clone_graph() {
        let node1 = Rc::new(RefCell::new(Node::new(1)));
        let node2 = Rc::new(RefCell::new(Node::new(2)));
        let node3 = Rc::new(RefCell::new(Node::new(3)));
        let node4 = Rc::new(RefCell::new(Node::new(4)));

        node1.borrow_mut().neighbors.push(Rc::clone(&node2));
        node1.borrow_mut().neighbors.push(Rc::clone(&node4));

        node2.borrow_mut().neighbors.push(Rc::clone(&node1));
        node2.borrow_mut().neighbors.push(Rc::clone(&node3));

        node3.borrow_mut().neighbors.push(Rc::clone(&node2));
        node3.borrow_mut().neighbors.push(Rc::clone(&node4));

        node4.borrow_mut().neighbors.push(Rc::clone(&node1));
        node4.borrow_mut().neighbors.push(Rc::clone(&node3));

        let cloned_graph = clone_graph(Rc::clone(&node1));

        match &cloned_graph {
            Some(copy_node1) => {
                assert_eq!(copy_node1.borrow().val, 1);
                assert!(!Rc::ptr_eq(copy_node1, &node1));
                assert_eq!(copy_node1.borrow().neighbors.len(), 2);

                let copy_node2 = Rc::clone(copy_node1.borrow().neighbors.get(0).unwrap());
                let copy_node4 = Rc::clone(copy_node1.borrow().neighbors.get(1).unwrap());

                assert_eq!(copy_node2.borrow().val, 2);
                assert!(!Rc::ptr_eq(&copy_node2, &node2));
                assert_eq!(copy_node2.borrow().neighbors.len(), 2);

                assert_eq!(copy_node4.borrow().val, 4);
                assert!(!Rc::ptr_eq(&copy_node4, &node4));
                assert_eq!(copy_node4.borrow().neighbors.len(), 2);

                let copy_node11 = Rc::clone(copy_node2.borrow().neighbors.get(0).unwrap());
                let copy_node3 = Rc::clone(copy_node2.borrow().neighbors.get(1).unwrap());

                assert_eq!(copy_node11.borrow().val, 1);
                assert!(!Rc::ptr_eq(&copy_node11, &node1));
                assert!(Rc::ptr_eq(&copy_node11, &copy_node1));
                assert_eq!(copy_node11.borrow().neighbors.len(), 2);

                assert_eq!(copy_node3.borrow().val, 3);
                assert!(!Rc::ptr_eq(&copy_node3, &node3));
                assert_eq!(copy_node3.borrow().neighbors.len(), 2);

                let copy_node22 = Rc::clone(copy_node3.borrow().neighbors.get(0).unwrap());
                let copy_node44 = Rc::clone(copy_node3.borrow().neighbors.get(1).unwrap());
                assert!(Rc::ptr_eq(&copy_node22, &copy_node2));
                assert!(Rc::ptr_eq(&copy_node44, &copy_node4));

                let copy_node33 = Rc::clone(copy_node4.borrow().neighbors.get(1).unwrap());
                assert!(Rc::ptr_eq(&copy_node33, &copy_node3));
            }
            None => {
                assert!(false);
            }
        }
    }
}