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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut fake_head = Some(Box::new(ListNode::new(0)));
        fake_head.as_mut().unwrap().next = head;
        
        let mut s: &mut Option<Box<ListNode>> = &mut fake_head;
        let mut e: &mut Option<Box<ListNode>> = s;
        
        let mut count = 0;
        while e.as_ref().unwrap().next.is_some() {
            count += 1;
            e = &mut e.as_mut().unwrap().next;
            
            if count == k {
                let tail = e.as_mut().unwrap().next.take();
                let to_rev = s.as_mut().unwrap().next.take();
                s.as_mut().unwrap().next = Self::reverse_list(to_rev);
                let mut e2 = s;
                while e2.as_ref().unwrap().next.is_some() {
                    e2 = &mut e2.as_mut().unwrap().next;
                }
                e2.as_mut().unwrap().next = tail;
                s = e2;
                e = s;
                count = 0;
            }
        }
        return fake_head.as_mut().unwrap().next.take()
    }
    
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
