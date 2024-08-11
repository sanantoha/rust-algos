use rust_algos::tree::tree;
use tree::TreeNode;
use rust_algos::list::list;
use list::ListNode;

fn main() {

    let mut root = TreeNode::single(1);

    // Replace the empty children with real nodes
    root.left = Some(Box::new(TreeNode::single(2)));
    root.right = Some(Box::new(TreeNode::single(3)));

    println!("Hello, world1! {:?}", root);

    let third = Box::new(ListNode::new(3));
    let second = Box::new(ListNode::with_next(2, Some(third)));
    let first = Box::new(ListNode::with_next(1, Some(second)));

    let head = ListNode::with_next(1, Some(Box::new(ListNode::with_next(2, Some(Box::new(ListNode::with_next(3, Some(Box::new(ListNode::with_next(4, Some(Box::new(ListNode::new(5)))))))))))));

    println!("{}", first);
    println!("{}", head);
}
