use rust_algos::tree::tree;
use tree::TreeNode;

fn main() {

    let mut root = TreeNode::single(1);

    // Replace the empty children with real nodes
    root.left = Some(Box::new(TreeNode::single(2)));
    root.right = Some(Box::new(TreeNode::single(3)));

    println!("Hello, world1! {:?}", root);
}
