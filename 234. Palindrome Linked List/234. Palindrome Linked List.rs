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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        fn list_to_vec(list: &Option<Box<ListNode>>) -> Vec<i32> {
            let mut p = list;
            let mut v: Vec<i32> = Vec::new();
            while p.is_some() {
                let node = p.as_ref().unwrap();
                v.push(node.val);
                p = &node.next;
            }
            return v;
        }
        
        let v = list_to_vec(&head);
        let l = v.len();
        
        for i in 0..(l / 2) {
            if v[i] != v[l - i - 1] {
                return false;
            }
        }
        return true;
    }
}
