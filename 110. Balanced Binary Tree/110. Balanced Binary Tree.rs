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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn d(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
            // some depth if balanced
            if let Some(node) = root {
                let TreeNode { left, right, ..} = &*node.borrow();
                if let Some(dl) = d(left) {
                    if let Some(dr) = d(right) {
                        if  (dr - dl).abs() <= 1 {
                            return Some(dl.max(dr) + 1)
                        }
                    }
                }
            } else {
              return Some(0);
            }
            None
        }
        d(&root).is_some()
    }
}
