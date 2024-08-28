use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::{fmt, io};

#[allow(dead_code)]
#[derive(Debug)]
struct LRUCache {
    capacity: usize,
    cache: HashMap<i32, Rc<RefCell<Node>>>,
    head: Rc<RefCell<Node>>,
    tail: Rc<RefCell<Node>>
}

#[allow(dead_code)]
impl LRUCache {
    
    pub fn new(capacity: usize) -> Result<LRUCache, Box<dyn std::error::Error>> {
        if capacity == 0 {
            let err = std::io::Error::new(io::ErrorKind::InvalidInput, "capacity should be greater that zero");
            Err(Box::new(err))
        } else {
            let cache = HashMap::with_capacity(capacity);
            let head = Rc::new(RefCell::new(Node {key: 0, val: 0, next: None, prev: None}));
            let tail = Rc::new(RefCell::new(Node {key: 0, val: 0, next: None, prev: None}));
            head.borrow_mut().next = Some(Rc::clone(&tail));
            tail.borrow_mut().prev = Some(Rc::clone(&head));

            Ok(LRUCache{
                capacity,
                cache,
                head,
                tail
            })
        }
    }

    pub fn get(&self, key: i32) -> Option<i32> {
        self.cache.get(&key).map(|node| {
            LRUCache::remove(node);
            self.add(node);
            node.borrow().val
        })
    }

    pub fn put(&mut self, key: i32, val: i32) {
        if let Some(node) = self.cache.get(&key) {
            LRUCache::remove(node);
            self.add(node);
            node.borrow_mut().val = val;
        } else {
            if self.cache.len() == self.capacity {
                let latest = self.tail.borrow().prev.as_ref().map(Rc::clone);
                if let Some(latest_node) = latest {
                    self.cache.remove(&latest_node.borrow().val);
                    LRUCache::remove(&latest_node);
                }                
            }
            let node = Rc::new(RefCell::new(Node { key, val, next: None, prev: None }));
            self.add(&node);
            self.cache.insert(key, Rc::clone(&node));
        }
    }

    fn add(&self, node: &Rc<RefCell<Node>>) {
        if let Some(head_next) = self.head.borrow().next.as_ref() {
            node.borrow_mut().next = Some(Rc::clone(head_next));
            node.borrow_mut().prev = Some(Rc::clone(&self.head));

            head_next.borrow_mut().prev = Some(Rc::clone(node));                        
        }
        self.head.borrow_mut().next = Some(Rc::clone(node));
    }

    fn remove(node: &Rc<RefCell<Node>>) {
        if let Some(next_node) = node.borrow().next.as_ref() {
            next_node.borrow_mut().prev = node.borrow().prev.as_ref().map(Rc::clone);
        }

        if let Some(prev_node) = node.borrow().prev.as_ref() {
            prev_node.borrow_mut().next = node.borrow().next.as_ref().map(Rc::clone);
        }       
    }
}

impl fmt::Display for LRUCache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let head = &self.head.borrow().next.as_ref().map(|x| format!("{}", x.borrow())).unwrap_or("".to_owned());
        let tail = &self.tail.borrow().prev.as_ref().map(|x| format!("{}", x.borrow())).unwrap_or("".to_owned());
        write!(f, "LRUCache {{ 
    capacity: {},
    head: {},
    tail: {}
}}", &self.capacity, head, tail)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Node {
    key: i32,
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Rc<RefCell<Node>>>
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.next {
            Some(next_node) => write!(f, "key:{},val:{} -> {}", self.key, self.val, next_node.borrow()),
            None => write!(f, "key:{},val:{} -> ", self.key, self.val),
        }
    }
}


#[cfg(test)]
mod tests {

    use super::LRUCache;

    #[test]
    fn it_lru_cache() {

        if let Ok(mut lru_cache) = LRUCache::new(2) {            
            lru_cache.put(1, 1);            
            lru_cache.put(2, 2);
            println!("{}", lru_cache);            

            let r = lru_cache.get(1);
            println!("{:?}", r); // 1
            assert_eq!(r, Some(1));
            println!("{}", lru_cache);
            

            lru_cache.put(3, 3);
            println!("{}", lru_cache);

            let r = lru_cache.get(2);
            println!("{:?}", r); // None
            assert_eq!(r, None);

            lru_cache.put(4, 4);
            println!("{}", lru_cache);

            let r = lru_cache.get(1);
            println!("{:?}", r); // None
            assert_eq!(r, None);
            let r = lru_cache.get(3);
            println!("{:?}", r); // Some(3)
            assert_eq!(r, Some(3));
            let r = lru_cache.get(4);
            println!("{:?}", r); // Some(4)
            assert_eq!(r, Some(4));
            println!("{}", lru_cache);
        }        
    }
}