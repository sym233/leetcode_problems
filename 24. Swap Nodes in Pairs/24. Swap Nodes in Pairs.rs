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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        if let Some(ref mut fst) = head {
            if let Some(mut snd) = fst.next.take() {
                std::mem::swap(fst, &mut snd);
                snd.next = Self::swap_pairs(fst.next.take());
                fst.next = Some(snd);
            }
        }
        head
    }
}
