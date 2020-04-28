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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut level = vec![root];
        let mut next_lv: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
        
        while !level.is_empty() {
            for back in level.iter().rev() {
                if back.is_some() {
                    res.push(back.as_ref().unwrap().borrow().val);
                    break;
                }
            }
            
            for node in level.iter_mut() {
                let mut node = node.take();
                if node.is_some() {
                    let mut node = Rc::try_unwrap(node.unwrap()).unwrap().into_inner();
                    next_lv.push(node.left.take());
                    next_lv.push(node.right.take());
                }
            }
            level = next_lv;
            next_lv = Vec::new();
        }
        return res;
    }
}
