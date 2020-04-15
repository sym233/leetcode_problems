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

use std::cmp::Ordering;
impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut fake_head = Some(Box::new(ListNode::new(0)));
        let mut p = &mut fake_head;
        let mut fake_head_2 = Some(Box::new(ListNode::new(0)));
        let mut p_2 = &mut fake_head_2;
        let mut head = head;
        while head.is_some() {
            let tail = head.as_mut().unwrap().next.take();
            let val = head.as_ref().unwrap().val;
            if val.cmp(&x) == Ordering::Less{
                p.as_mut().unwrap().next = head;
                p = &mut p.as_mut().unwrap().next;
            } else {
                p_2.as_mut().unwrap().next = head;
                p_2 = &mut p_2.as_mut().unwrap().next;
            }
            head = tail;
        }
        p.as_mut().unwrap().next = fake_head_2.as_mut().unwrap().next.take();
        return fake_head.as_mut().unwrap().next.take();
    }
}
