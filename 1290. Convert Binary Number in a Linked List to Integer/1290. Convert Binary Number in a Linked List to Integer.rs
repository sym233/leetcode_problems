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
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut n = 0;
        let mut head = head;
        
        while head.is_some() {
            let tail = head.as_mut().unwrap().next.take();

            n <<= 1;
            n |= head.unwrap().val;
            head = tail;
        }
        return n;
    }
}
