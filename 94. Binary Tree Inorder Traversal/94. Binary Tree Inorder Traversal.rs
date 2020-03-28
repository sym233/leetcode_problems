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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if let Some(r_node) = root {
            let ref_node = Rc::try_unwrap(r_node).unwrap();
            let node = ref_node.into_inner();
            return [
                Self::inorder_traversal(node.left),
                vec![node.val],
                Self::inorder_traversal(node.right)
            ].concat();
        } else {
            return Vec::new();
        }
    }
}
