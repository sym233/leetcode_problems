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
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fake_head = Some(Box::new(ListNode::new(0)));
        let mut head = head;        
        while head.is_some() {
            let tail = head.as_mut().unwrap().next.take();
            let mut p = &mut fake_head;
            
            loop {
                if  p.as_ref().unwrap().next.is_none()
                    || head.as_ref().unwrap().val < p.as_ref().unwrap().next.as_ref().unwrap().val
                {
                    head.as_mut().unwrap().next = p.as_mut().unwrap().next.take();
                    p.as_mut().unwrap().next = head;
                    head = tail;
                    break;
                } else {
                    p = &mut p.as_mut().unwrap().next;
                }  
            }
        }
        return fake_head.as_mut().unwrap().next.take()
    }
}
