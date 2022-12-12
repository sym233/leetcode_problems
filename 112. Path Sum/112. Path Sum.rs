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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        root.map_or(false, |root| {
            match &*root.borrow() {
                &TreeNode {val, left: None, right: None} => val == target_sum,
                &TreeNode {val, ref left, ref right} => 
                    Self::has_path_sum(left.clone(), target_sum - val) || Self::has_path_sum(right.clone(), target_sum - val),
            }
        })
    }
}
