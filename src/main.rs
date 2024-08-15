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

    println!("Hello, world1! {:?}", root);

    let third = ListNode::new(3);
    let second = ListNode::with_next(2, Some(third));
    let first = ListNode::with_next(1, Some(second));

    let head = ListNode::with_next(1, Some(ListNode::with_next(2, Some(ListNode::with_next(3, Some(ListNode::with_next(4, Some(ListNode::new(5)))))))));

    println!("{}", DisplayableListNode::new(first));
    println!("{}", DisplayableListNode::new(head));
}
