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
    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return Self::walk(None, None, root);
    }
    
    fn walk(grandpa_val: Option<i32>, father_val: Option<i32>, node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if node.is_none() {
            return 0;
        }
        
        let mut res = 0;
        let node_r = node.as_ref().unwrap().borrow();
        
        if let Some(n) = grandpa_val {
            if n % 2 == 0 {
                res += node_r.val;
            }
        }
        
        let left = node_r.left.as_ref().map(|r| r.clone());
        let right = node_r.right.as_ref().map(|r| r.clone());
        
        return res 
            + Self::walk(father_val.clone(), Some(node_r.val), left)
            + Self::walk(father_val.clone(), Some(node_r.val), right);
        
    }
}
