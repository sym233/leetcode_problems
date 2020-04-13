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
impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut l = 0;
        let mut p = &head;
        
        loop {
            if p.is_some() {
                p = &p.as_ref().unwrap().next;
                l += 1;
            } else {
                break;
            }
        }
        
        if l <= 1 {
            return head;
        }
        
        let mut p = &mut head;
        for _ in 0..(l / 2) {
            p = &mut p.as_mut().unwrap().next;
        }
        let tail = p.take();
        
        return Self::merge(Self::sort_list(head), Self::sort_list(tail));
    }
    
    fn merge(h1: Option<Box<ListNode>>, h2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut h1 = h1;
        let mut h2 = h2;
        
        match (h1.is_none(), h2.is_none()) {
            (true, true) => return None,
            (true, false) => return h2,
            (false, true) => return h1,
            (false, false) => {
                if h1.as_ref().unwrap().val < h2.as_ref().unwrap().val {
                    let tail = h1.as_mut().unwrap().next.take();
                    h1.as_mut().unwrap().next = Self::merge(tail, h2);
                    return h1;
                } else {
                    let tail = h2.as_mut().unwrap().next.take();
                    h2.as_mut().unwrap().next = Self::merge(h1, tail);
                    return h2;
                }
            }
        }
    }
}
