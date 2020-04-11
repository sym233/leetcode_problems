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
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {        
        fn walk(node: &Option<Rc<RefCell<TreeNode>>>, f: &dyn Fn(i32) -> ()) {
            if node.is_none() {
                return;
            }
            let node_ref = node.as_ref().unwrap().as_ref().borrow();

            walk(&node_ref.left, f);
            f(node_ref.val);
            walk(&node_ref.right, f);
        }

        
        let vr = RefCell::new(Vec::new());
        walk(&root, &|val| vr.borrow_mut().push(val));
        let v: Vec<i32> = vr.into_inner();
        
        let mut min = (v[0] - v[1]).abs();
        for s in v.windows(2) {
            let t = (s[0] - s[1]).abs();
            if t < min {
                min = t;
            }
        }

        return min;
    }
}
