use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use crate::tree::Node;

// O(n) time | O(n) space
pub fn connect(root: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {

    if let Some(node) = root {

        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&node));

        while !queue.is_empty() {
            let mut size = queue.len();

            let mut prev: Option<Rc<RefCell<Node>>> = None;

            while size > 0 {
                size -= 1;
                if let Some(curr) = queue.pop_front() {
                    if let Some(prev_) = prev {
                        prev_.borrow_mut().parent = Some(Rc::clone(&curr));
                    }

                    if let Some(left) = &curr.borrow().left {
                        queue.push_back(Rc::clone(left));
                    }
                    if let Some(right) = &curr.borrow().right {
                        queue.push_back(Rc::clone(right));
                    }
                    prev = Some(Rc::clone(&curr));
                }
            }
        }

        return Some(Rc::clone(&node));
    }

    None
}

#[cfg(test)]
mod tests {

    use super::connect;
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::tree::Node;

    #[test]
    fn test_connect() {

        let root = Some(Rc::new(RefCell::new(Node::new_with(1,
                             Some(Rc::new(RefCell::new(Node::new_with(2,
                                  Some(Rc::new(RefCell::new(Node::new(4)))),
                                  Some(Rc::new(RefCell::new(Node::new(5)))),
                                  None
                             )))),
                             Some(Rc::new(RefCell::new(Node::new_with(3,
                                  Some(Rc::new(RefCell::new(Node::new(6)))),
                                  Some(Rc::new(RefCell::new(Node::new(7)))),
                                  None
                             )))),
                            None
        ))));


        let exp = create_exp_tree();

        let res = connect(root.as_ref().map(Rc::clone));
        assert_eq!(res, exp);
    }

    fn create_exp_tree() -> Option<Rc<RefCell<Node>>> {
        let node2 = Some(Rc::new(RefCell::new(Node::new(2))));
        let node4 = Some(Rc::new(RefCell::new(Node::new(4))));
        let node5 = Some(Rc::new(RefCell::new(Node::new(5))));

        let node3 = Some(Rc::new(RefCell::new(Node::new(3))));
        let node6 = Some(Rc::new(RefCell::new(Node::new(6))));
        let node7 = Some(Rc::new(RefCell::new(Node::new(7))));

        if let Some(node2_) = node2.as_ref() {
            node2_.borrow_mut().parent = node3.as_ref().map(Rc::clone);
            node2_.borrow_mut().left = node4.as_ref().map(Rc::clone);
            node2_.borrow_mut().right = node5.as_ref().map(Rc::clone);
        }

        if let Some(node3_) = node3.as_ref() {
            node3_.borrow_mut().left = node6.as_ref().map(Rc::clone);
            node3_.borrow_mut().right = node7.as_ref().map(Rc::clone);
        }

        if let Some(node4_) = node4.as_ref() {
            node4_.borrow_mut().parent = node5.as_ref().map(Rc::clone);
        }

        if let Some(node5_) = node5.as_ref() {
            node5_.borrow_mut().parent = node6.as_ref().map(Rc::clone);
        }

        if let Some(node6_) = node6.as_ref() {
            node6_.borrow_mut().parent = node7.as_ref().map(Rc::clone);
        }

        let exp = Some(Rc::new(RefCell::new(Node::new_with(1,
                                                           node2.as_ref().map(Rc::clone),
                                                           node3.as_ref().map(Rc::clone),
                                                           None
        ))));

        exp
    }
}