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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let n = n as usize;
        let mut fake_head = ListNode::new(0);
        fake_head.next = head;
        let mut fake_head = Some(Box::new(fake_head));
        let mut t: Vec<*mut Option<Box<ListNode>>> = Vec::new();
        let mut p = &mut fake_head as *mut Option<Box<ListNode>>;
        
        unsafe {
            while (p.as_ref().unwrap().is_some()) {
                t.push(p);
                p = &mut p.as_mut().unwrap().as_mut().unwrap().next as *mut Option<Box<ListNode>>;
            }

            // including fake_head
            let l = t.len();
        
            // access node t[l - n - 1] and delete node t[l - n - 2]
            let mut removed_node = t[l - n - 1].as_mut().unwrap().as_mut().unwrap().next.take();
            let tail = removed_node.as_mut().unwrap().next.take();
            t[l - n - 1].as_mut().unwrap().as_mut().unwrap().next = tail; 
        }
        return fake_head.as_mut().unwrap().next.take();
    }
}
