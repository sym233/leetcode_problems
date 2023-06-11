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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
          match (list1, list2) {
              (Some(mut n1), Some(mut n2)) if n1.val < n2.val => {
                  n1.next = Self::merge_two_lists(n1.next.take(), Some(n2));
                  Some(n1)
              },
              (Some(mut n1), Some(mut n2)) => {
                  n2.next = Self::merge_two_lists(n2.next.take(), Some(n1));
                  Some(n2)
              },
              (l1 @ Some(_), None) => l1,
              (None, l2 @ Some(_)) => l2,
              (None, None) => None,
          }
    }
}
