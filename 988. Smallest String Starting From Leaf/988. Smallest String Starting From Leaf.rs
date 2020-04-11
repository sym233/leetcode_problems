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
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn dfs(node: Rc<RefCell<TreeNode>>, s: String, mut res: &mut String) {
            let node_r = node.borrow();
            let mut s = s;
            s.push((b'a' + node_r.val as u8) as char);
            let mut is_leaf = true;
            if node_r.left.is_some() {
                dfs(node_r.left.as_ref().unwrap().clone(), s.clone(), &mut res);
                is_leaf = false;
            }
            if node_r.right.is_some() {
                dfs(node_r.right.as_ref().unwrap().clone(), s.clone(), &mut res);
                is_leaf = false;
            }
            if is_leaf {
                let s: String = s.chars().rev().collect();
                if res.is_empty() || s < *res {
                    *res = s;
                }
            }
        }
        let mut res = String::new();
        if root.is_some() {
            dfs(root.as_ref().unwrap().clone(), String::new(), &mut res);
        }
        return res;
    }
}
