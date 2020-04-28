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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }
        if p.is_some() && q.is_some() {
            let pp = p.as_ref().unwrap().borrow();
            let qq = q.as_ref().unwrap().borrow();
            if pp.val == qq.val {
                return Self::is_same_tree(
                    pp.left.as_ref().map(|r| r.clone()),
                    qq.left.as_ref().map(|r| r.clone())
                ) && Self::is_same_tree(
                    pp.right.as_ref().map(|r| r.clone()),
                    qq.right.as_ref().map(|r| r.clone())
                );
            }
        }
        return false;
    }
}
