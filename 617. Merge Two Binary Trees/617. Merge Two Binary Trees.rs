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
    pub fn merge_trees(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root1) = root1 {
            if let Some(root2) = root2 {
                let mut root1 = root1.borrow_mut();
                let mut root2 = root2.borrow_mut();
                return Some(Rc::new(RefCell::new(TreeNode {
                    val: root1.val + root2.val,
                    left: Self::merge_trees(root1.left.take(), root2.left.take()),
                    right: Self::merge_trees(root1.right.take(), root2.right.take()),
                })));
            } else {
                return Some(root1);
            }
        } else {
            return root2;
        }
    }
}
