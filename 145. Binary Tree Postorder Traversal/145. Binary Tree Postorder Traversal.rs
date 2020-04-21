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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![root];
        let mut res: Vec<i32> = Vec::new();
        
        while !stack.is_empty() {
            let mut top = stack.last_mut().unwrap();
            let mut has_child = false;
            
            if top.is_some() {
                let mut r = top.as_ref().unwrap().clone();
                let mut node = r.borrow_mut();

                let mut right = None;
                let mut left = None;
                if node.right.is_some() {
                    right = node.right.take();
                    has_child = true;
                }
                if node.left.is_some() {
                    left = node.left.take();
                    has_child = true;
                }

                if has_child {
                    stack.push(right);
                    stack.push(left);
                } else {
                    res.push(node.val);
                    stack.pop();
                }
            } else {
                stack.pop();
            }
        }
        return res;
    }
}
