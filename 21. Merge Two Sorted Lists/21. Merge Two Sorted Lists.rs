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
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn get_node(l: Option<Box<ListNode>>) -> (ListNode, Option<Box<ListNode>>) {
            let mut node = *l.unwrap();
            let new_l = node.next;
            node.next = None;
            return (node, new_l);
        }
        match (&l1, &l2) {
            (&None, &None) => return None,
            (Some(_), &None) => {
                let (mut new_node, new_l1) = get_node(l1);
                new_node.next = Solution::merge_two_lists(new_l1, l2);
                return Some(Box::new(new_node));
            },
            (&None, Some(_)) => {
                let (mut new_node, new_l2) = get_node(l2);
                new_node.next = Solution::merge_two_lists(l1, new_l2);
                return Some(Box::new(new_node));
            },
            (Some(b1), Some(b2)) => {
                if b1.val < b2.val {
                    let (mut new_node, new_l1) = get_node(l1);
                    new_node.next = Solution::merge_two_lists(new_l1, l2);
                    return Some(Box::new(new_node));
                } else {
                    let (mut new_node, new_l2) = get_node(l2);
                    new_node.next = Solution::merge_two_lists(l1, new_l2);
                    return Some(Box::new(new_node));
                }
            }
        }
    }
}
