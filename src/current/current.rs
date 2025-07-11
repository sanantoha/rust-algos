use std::cell::RefCell;
use std::rc::Rc;
use crate::tree::BinaryTree;

pub fn lowest_common_ancestor(root: &Option<Rc<RefCell<BinaryTree>>>,
                              p: &Option<Rc<RefCell<BinaryTree>>>,
                              q: &Option<Rc<RefCell<BinaryTree>>>) -> Option<Rc<RefCell<BinaryTree>>> {
    None
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::tree::{BinaryTree};
    use super::lowest_common_ancestor;

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