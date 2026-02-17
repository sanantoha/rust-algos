use std::cell::RefCell;
use std::rc::Rc;
use crate::tree::Node;

// O(h) time | O(1) space
pub fn find_in_order_successor(node: &Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
    None
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::tree::Node;
    use super::find_in_order_successor;

    #[test]
    fn test_find_in_order_successor() {
        let (node5, node9) = create_tree(5, 9);

        assert_eq!(find_in_order_successor(&node5), node9)
    }

    #[test]
    fn test_find_in_order_successor_case1() {
        let (node9, node11) = create_tree(9, 11);

        assert_eq!(find_in_order_successor(&node9), node11)
    }

    #[test]
    fn test_find_in_order_successor_case2() {
        let (node11, node12) = create_tree(11, 12);

        assert_eq!(find_in_order_successor(&node11), node12)
    }

    #[test]
    fn test_find_in_order_successor_case3() {
        let (node12, node14) = create_tree(12, 14);

        assert_eq!(find_in_order_successor(&node12), node14)
    }

    #[test]
    fn test_find_in_order_successor_case4() {
        let (node14, node20) = create_tree(14, 20);

        assert_eq!(find_in_order_successor(&node14), node20)
    }

    #[test]
    fn test_find_in_order_successor_case5() {
        let (node20, node25) = create_tree(20, 25);

        assert_eq!(find_in_order_successor(&node20), node25)
    }

    #[test]
    fn test_find_in_order_successor_case6() {
        let (node25, _) = create_tree(25, i32::MAX);

        assert_eq!(find_in_order_successor(&node25), None)
    }

    fn create_tree(input: i32, result: i32) -> (Option<Rc<RefCell<Node>>>, Option<Rc<RefCell<Node>>>) {
        /*
                    20
                  /   \
                9     25
              /   \
            5     12
                 /  \
               11   14

         */

        let node5 = Rc::new(RefCell::new(Node::new(5)));
        let node9 = Rc::new(RefCell::new(Node::new(9)));
        let node11 = Rc::new(RefCell::new(Node::new(11)));
        let node12 = Rc::new(RefCell::new(Node::new(12)));
        let node14 = Rc::new(RefCell::new(Node::new(14)));
        let node20 = Rc::new(RefCell::new(Node::new(20)));
        let node25 = Rc::new(RefCell::new(Node::new(25)));


        node5.borrow_mut().parent = Some(Rc::clone(&node9));

        node9.borrow_mut().left = Some(Rc::clone(&node5));
        node9.borrow_mut().right = Some(Rc::clone(&node12));
        node9.borrow_mut().parent = Some(Rc::clone(&node20));

        node11.borrow_mut().parent = Some(Rc::clone(&node12));

        node12.borrow_mut().parent = Some(Rc::clone(&node9));
        node12.borrow_mut().left = Some(Rc::clone(&node11));
        node12.borrow_mut().right = Some(Rc::clone(&node14));

        node14.borrow_mut().parent = Some(Rc::clone(&node12));

        node20.borrow_mut().left = Some(Rc::clone(&node9));
        node20.borrow_mut().right = Some(Rc::clone(&node25));

        node25.borrow_mut().parent = Some(Rc::clone(&node20));

        let vec = vec![
            Rc::clone(&node5),
            Rc::clone(&node9),
            Rc::clone(&node11),
            Rc::clone(&node12),
            Rc::clone(&node14),
            Rc::clone(&node20),
            Rc::clone(&node25)
        ];

        let mut input_node = None;
        let mut result_node = None;

        for v in vec.iter() {
            if v.borrow().val == input {
                input_node = Some(Rc::clone(&v));
            }
            if v.borrow().val == result {
                result_node = Some(Rc::clone(&v));
            }
        }

        (input_node, result_node)
    }
}