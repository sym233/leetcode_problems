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
use std::cell::{RefCell, Ref};
impl Solution {
    pub fn is_subtree(s: Option<Rc<RefCell<TreeNode>>>, t: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if Self::comp_tree(cp_op_rc(&s), cp_op_rc(&t)) {
            return true;
        }
        
        if s.is_some() && t.is_some() {
            let node_r = br_in(&s);
            return Self::is_subtree(
                cp_op_rc(&node_r.left),
                cp_op_rc(&t)
            ) || Self::is_subtree(
                cp_op_rc(&node_r.right),
                cp_op_rc(&t)
            );
        }
        return false;
    }
    fn comp_tree(a: Option<Rc<RefCell<TreeNode>>>, b: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if a.is_none() && b.is_none() {
            return true;
        }
        if a.is_some() && b.is_some() {
            let a_node = br_in(&a);
            let b_node = br_in(&b);
            
            return a_node.val == b_node.val 
                && Self::comp_tree(
                    cp_op_rc(&a_node.left), 
                    cp_op_rc(&b_node.left)
                )
                && Self::comp_tree(
                    cp_op_rc(&a_node.right), 
                    cp_op_rc(&b_node.right)
                );
        }
        return false;
    }
}

fn cp_op_rc<T>(op_rc: &Option<Rc<T>>) -> Option<Rc<T>> {
    return op_rc.as_ref().map(|rc| rc.clone());
}
fn br_in<T>(op_rf: &Option<Rc<RefCell<T>>>) -> Ref<T>{
    return op_rf.as_ref().unwrap().borrow();
}
