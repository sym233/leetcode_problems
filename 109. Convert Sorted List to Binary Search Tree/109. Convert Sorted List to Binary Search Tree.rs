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
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut l = 0;
        let mut head = head;
        let mut p = &head;
        
        if p.is_none() {
            return None;
        }
        
        while p.is_some() {
            l += 1;
            p = &p.as_ref().unwrap().next;
        }
        
        let mut p = &mut head;
        for _ in 0..(l / 2) {
            p = &mut p.as_mut().unwrap().next;
        }
        
        let right = p.as_mut().unwrap().next.take();
        let mid = p.take();
        let left = head;
        
        let mut tn = TreeNode::new(mid.unwrap().val);
        tn.left = Self::sorted_list_to_bst(left);
        tn.right = Self::sorted_list_to_bst(right);      
        return Some(Rc::new(RefCell::new(tn)));
    }
}
