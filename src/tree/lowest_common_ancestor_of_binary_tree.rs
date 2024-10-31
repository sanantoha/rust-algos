use std::cell::RefCell;
use std::rc::Rc;
use crate::tree::BinaryTree;

// O(n) time | O(n) space
pub fn lowest_common_ancestor<'a>(root: &'a Option<Rc<RefCell<BinaryTree>>>,
                                  p: &'a Option<Rc<RefCell<BinaryTree>>>,
                                  q: &'a Option<Rc<RefCell<BinaryTree>>>) -> Option<Rc<RefCell<BinaryTree>>> {

    if let Some(node) = root {
        if let Some(pt) = p {
            if Rc::ptr_eq(&node, &pt) {
                return root.as_ref().map(Rc::clone);
            }
        }
        if let Some(qt) = q {
            if Rc::ptr_eq(&node, &qt) {
                return root.as_ref().map(Rc::clone);
            }
        }

        let left = node.borrow().left.as_ref().map(Rc::clone);
        let right = node.borrow().right.as_ref().map(Rc::clone);

        let left_res = lowest_common_ancestor(&left, p, q);
        let right_res = lowest_common_ancestor(&right, p, q);

        if left_res.is_some() && right_res.is_some() {
            return root.as_ref().map(Rc::clone);
        }

        if left_res.is_some() {
            left_res
        } else {
            right_res
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::tree::{BinaryTree};
    use crate::tree::lowest_common_ancestor_of_binary_tree::lowest_common_ancestor;

    #[test]
    fn test_lowest_common_ancessor() {

        let node7 = Rc::new(RefCell::new(BinaryTree::leaf(7)));

        let node5 = Rc::new(RefCell::new(BinaryTree::new(5,
                Some(Rc::new(RefCell::new(BinaryTree::leaf(6)))),
                Some(Rc::new(RefCell::new(BinaryTree::new(2,
                    Some(Rc::clone(&node7)),
                    Some(Rc::new(RefCell::new(BinaryTree::leaf(4))))
                ))
            ))
        )));

        let node1 = Rc::new(RefCell::new(BinaryTree::new(1,
                 Some(Rc::new(RefCell::new(BinaryTree::leaf(0)))),
                 Some(Rc::new(RefCell::new(BinaryTree::leaf(8)))),
        )));

        let root = Some(Rc::new(RefCell::new(BinaryTree::new(3,
            Some(Rc::clone(&node5)),
            Some(Rc::clone(&node1)),
        ))));


        let res = lowest_common_ancestor(&root, &Some(Rc::clone(&node7)), &Some(Rc::clone(&node5)));
        println!("{:?}", res);

        assert_eq!(res, Some(Rc::clone(&node5)));
    }
}