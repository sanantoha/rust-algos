pub mod reverse_list;
pub mod cycle_linked_list;
pub mod add_two_numbers;
pub mod merged_two_sorted_lists;
pub mod middle_node;
pub mod deep_copy_arbitrary_pointer;

use std::cell::RefCell;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    pub fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ListNode{ val, next: None }))
    }

    pub fn with_next(val: i32, next: Option<Rc<RefCell<ListNode>>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ListNode{ val, next }))
    }
}

// Newtype wrapper for Rc<RefCell<ListNode>>
#[derive(Debug, Clone)]
pub struct ListNodeWrapper(pub Rc<RefCell<ListNode>>);

impl PartialEq for ListNodeWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().val == other.0.borrow().val
    }
}

impl Eq for ListNodeWrapper {}

impl Hash for ListNodeWrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.borrow().val.hash(state);
    }
}

impl fmt::Display for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.next {
            Some(next_node) => write!(f, "{} -> {}", self.val, next_node.borrow()),
            None => write!(f, "{}", self.val),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ArbListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<ArbListNode>>>,
    pub arb: Option<Rc<RefCell<ArbListNode>>>
}

impl ArbListNode {
    pub fn new(x: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(
            ArbListNode {val: x, next: None, arb: None }
        ))
    }
}

impl fmt::Display for ArbListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.next, &self.arb) {
            (Some(next_node), Some(arb_node)) => write!(f, "{} a:{} -> {}", self.val, arb_node.borrow().val, next_node.borrow()),
            (Some(next_node), None) => write!(f, "{} a:none -> {}", self.val, next_node.borrow()),
            (None, Some(arb_node)) => write!(f, "{} a:{} -> ", self.val, arb_node.borrow().val),
            (None, None) => write!(f, "{}", self.val),
        }
    }
}


#[derive(Debug)]
pub struct Displayable<T>(pub Rc<RefCell<T>>) where T: std::fmt::Display;

impl <T> Displayable<T> where T: std::fmt::Display {
    pub fn new(node: Rc<RefCell<T>>) -> Self {
        Displayable(node)
    }
    // Helper function to handle Option<Rc<RefCell<ListNode>>>
    pub fn from_option(option: Option<Rc<RefCell<T>>>) -> Option<Self> {
        option.map(Displayable::new)
    }
}

// Implement Display for DisplayableListNode
impl <T> fmt::Display for Displayable<T>  where T: std::fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Borrow the ListNode and use its Display implementation
        let node = self.0.borrow();
        write!(f, "{}", node)
    }
}