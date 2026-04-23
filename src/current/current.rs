use std::cell::RefCell;
use std::rc::Rc;
use std::{fmt, io};

#[allow(dead_code)]
#[derive(Debug)]
struct LRUCache {
    capacity: usize,
}

#[allow(dead_code)]
impl LRUCache {

    pub fn new(capacity: usize) -> Result<LRUCache, Box<dyn std::error::Error>> {
        let err = std::io::Error::new(io::ErrorKind::InvalidInput, "capacity should be greater that zero");
        Err(Box::new(err))
    }

    pub fn get(&self, key: i32) -> Option<i32> {
        None
    }

    pub fn put(&mut self, key: i32, val: i32) {

    }
}

impl fmt::Display for LRUCache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let head = &self.head.borrow().next.as_ref().map(|x| format!("{}", x.borrow())).unwrap_or("".to_owned());
//         let tail = &self.tail.borrow().prev.as_ref().map(|x| format!("{}", x.borrow())).unwrap_or("".to_owned());
//         write!(f, "LRUCache {{
//     capacity: {},
//     head: {},
//     tail: {}
// }}", &self.capacity, head, tail)
        Ok(())
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