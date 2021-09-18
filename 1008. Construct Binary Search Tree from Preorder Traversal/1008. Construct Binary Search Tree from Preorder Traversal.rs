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
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(preorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            let l = preorder.len();
            if l == 0 {
                return None;
            }
            let val = preorder[0];
            let mut first_larger = l;
            for i in 1..l {
                if preorder[i] > val {
                    first_larger = i;
                    break;
                }
            }
            let left = build(&preorder[1..first_larger]);
            let right = build(&preorder[first_larger..]);
            return Some(Rc::new(RefCell::new(TreeNode {
                val,
                left,
                right,
            })));
        }
        return build(&preorder[..]);
    }
}
