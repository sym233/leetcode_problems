// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

use rand::{thread_rng, Rng};

struct Solution {
    head: Option<Box<ListNode>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    /** @param head The linked list's head.
        Note that the head is guaranteed to be not null, so it contains at least one node. */
    fn new(head: Option<Box<ListNode>>) -> Self {
        Self {
            head,
        }
    }
    
    /** Returns a random node's value. */
    fn get_random(&self) -> i32 {
        let mut selected = self.head.as_ref().unwrap().val;
        
        let mut count = 2;
        let mut p = &self.head.as_ref().unwrap().next;
        let mut rng = thread_rng();
        
        while p.is_some() {
            let r: i32 = rng.gen_range(0, count);
            if r == 0 {
                selected = p.as_ref().unwrap().val;
            }
            count += 1;
            p = &p.as_ref().unwrap().next;
        }
        
        return selected;
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(head);
 * let ret_1: i32 = obj.get_random();
 */
