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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fake_head = ListNode::new(0);
        fake_head.next = head;
        let mut fake_rev_head = ListNode::new(0);
        while fake_head.next.is_some() {
            let mut node = fake_head.next.take();
            fake_head.next = node.as_mut().unwrap().next.take();
            
            node.as_mut().unwrap().next = fake_rev_head.next.take();
            fake_rev_head.next = node;
        }
        return fake_rev_head.next.take();
    }
}
