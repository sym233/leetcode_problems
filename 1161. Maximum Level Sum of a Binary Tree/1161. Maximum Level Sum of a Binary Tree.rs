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
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = i32::MIN;
        let mut min_level = i32::MAX;
        let mut nodes: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
        if root.is_some() {
            nodes.push(root);
        }

        for curr_level in 1.. {
            if nodes.is_empty() {
                break;
            }
            let mut sum = 0;
            let mut next_nodes: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
            for node in nodes {
                if let Some(node) = node {
                    let node = node.borrow();
                    sum += node.val;
                    if node.left.is_some() {
                        next_nodes.push(node.left.clone());
                    }
                    if node.right.is_some() {
                        next_nodes.push(node.right.clone());
                    }
                }
            }
            if sum > max_sum {
                max_sum = sum;
                min_level = curr_level;
            }
            nodes = next_nodes;
        }
        min_level
    }
}
