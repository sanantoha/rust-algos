use std::fmt;

#[derive(Debug, PartialEq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode{ val, next: None }
    }

    pub fn with_next(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode{ val, next }
    }
}

impl fmt::Display for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.next {
            Some(next_node) => write!(f, "{} -> {}", self.val, next_node),
            None => write!(f, "{}", self.val),
        }
    }
}