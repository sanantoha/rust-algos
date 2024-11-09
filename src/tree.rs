use std::cell::RefCell;
use std::rc::Rc;

pub mod balanced_binary_tree;
pub mod dfs_tree_traverse_rec;
pub mod dfs_tree_traverse;
pub mod bfs_tree_traverse;
pub mod kth_smallest_element_in_bst;
pub mod left_view_binary_tree;
pub mod level_order_binary_tree_traverse;
pub mod binary_tree_tilt;
mod all_elements_in_two_binary_search_trees;
mod binary_tree_diameter;
mod binary_tree_zigzag_level_order_traverse;
mod brunch_sums;
mod bst_successor_search;
mod construct_binary_tree_from_preorder_and_inorder_traversal;
mod convert_sorted_array_to_bst;
mod evaluate_expression_tree;
mod find_closest_value_in_bst;
mod find_mode_in_bst;
mod find_node_distance_k;
mod lower_common_ancestor_for_bst;
mod lowest_common_ancestor_of_binary_tree;
mod max_depth_of_bst;
mod max_path_sum_in_binary_tree;
mod merge_binary_trees;
mod minimum_absolute_difference;
mod node_depths;
mod populating_next_right_pointer_in_each_node;
mod reconstruct_bst;
mod reverse_binary_tree;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn leaf(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn new(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Self {
        TreeNode {
            val,
            left,
            right,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Node {
    pub val: i32,
    pub left: Option<Rc<RefCell<Node>>>,
    pub right: Option<Rc<RefCell<Node>>>,
    pub parent: Option<Rc<RefCell<Node>>>,
}

impl Node {

    pub fn new(val: i32) -> Self {
        Node {
            val,
            left: None,
            right: None,
            parent: None,
        }
    }

    pub fn new_with(val: i32, left: Option<Rc<RefCell<Node>>>, right: Option<Rc<RefCell<Node>>>, parent: Option<Rc<RefCell<Node>>>) -> Self {
        Node {
            val,
            left,
            right,
            parent
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct BinaryTree {
    pub val: i32,
    pub left: Option<Rc<RefCell<BinaryTree>>>,
    pub right: Option<Rc<RefCell<BinaryTree>>>,
}

impl BinaryTree {
    pub fn new(val: i32, left: Option<Rc<RefCell<BinaryTree>>>, right: Option<Rc<RefCell<BinaryTree>>>) -> Self {
        BinaryTree {
            val,
            left,
            right
        }
    }

    pub fn leaf(val: i32) -> Self {
        BinaryTree {
            val,
            left: None,
            right: None
        }
    }
}