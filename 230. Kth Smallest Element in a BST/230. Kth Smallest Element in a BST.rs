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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let v = Self::deserialize(root);
        let k = k as usize;
        return v[k - 1];
    }
    
    fn deserialize(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if node.is_some() {
            let mut node = Rc::try_unwrap(node.unwrap()).unwrap().into_inner();
            
            let left = node.left.take();
            let right = node.right.take();
            
            let mut res = Self::deserialize(left);
            res.push(node.val);
            res.append(&mut Self::deserialize(right));
            return res;
        } else {
            return Vec::new();
        }
    }
}
