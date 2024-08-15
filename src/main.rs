use std::rc::Rc;
use rust_algos::tree::tree;
use tree::TreeNode;
use rust_algos::list::list;
use list::ListNode;
use rust_algos::list::list::DisplayableListNode;

fn main() {

    let mut root = TreeNode::single(1);

    // Replace the empty children with real nodes
    root.left = Some(Box::new(TreeNode::single(2)));
    root.right = Some(Box::new(TreeNode::single(3)));

    println!("Tree {:?}", root);

    let third = ListNode::new(3);
    let second = ListNode::with_next(2, Some(third));
    let first = ListNode::with_next(1, Some(second));

    let head = ListNode::with_next(1, Some(ListNode::with_next(2, Some(ListNode::with_next(3, Some(ListNode::with_next(4, Some(ListNode::new(5)))))))));

    let ffirst = Rc::clone(&first);
    let fffirst = Rc::clone(&first);

    println!("{}", DisplayableListNode::new(Rc::clone(&ffirst)));
    println!("{}", DisplayableListNode::new(head));

    println!("{}", Rc::strong_count(&ffirst));

    let s = first.borrow().next.as_ref().map(Rc::clone);

    println!("{}", Rc::strong_count(&first));

    if let Some(sss) = DisplayableListNode::from_option(s) {
        println!("{}", sss);
    }
}
