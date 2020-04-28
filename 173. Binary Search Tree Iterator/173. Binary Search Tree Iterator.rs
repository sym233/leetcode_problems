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
struct BSTIterator {
    val: Option<i32>,
    val_iterated: bool,
    left_iter: Option<Box<BSTIterator>>,
    right_iter: Option<Box<BSTIterator>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        if root.is_none() {
            return Self {
                val: None,
                val_iterated: true,
                left_iter: None,
                right_iter: None,
            }
        }
        
        let node_r = root.as_ref().unwrap().borrow();
        let left_iter = node_r.left.as_ref().map(
            |r| Box::new(Self::new(Some(r.clone())))
        );
        let right_iter = node_r.right.as_ref().map(
            |r| Box::new(Self::new(Some(r.clone())))
        );
        return Self {
            val: Some(node_r.val),
            val_iterated: false,
            left_iter,
            right_iter,
        };
    }
    
    /** @return the next smallest number */
    fn next(&mut self) -> i32 {
        if !self.val_iterated {
            if self.left_iter.is_some() 
                && self.left_iter.as_ref().unwrap().has_next() 
            {
                // left
                return self.left_iter.as_mut().unwrap().next();
            } else {
                // node val
                self.val_iterated = true;
                return self.val.unwrap();
            }
        } else {
            // right
            return self.right_iter.as_mut().unwrap().next();
        }
    }
    
    /** @return whether we have a next smallest number */
    fn has_next(&self) -> bool {
        if self.val.is_none() {
            return false;
        }
        if self.val_iterated {
            // if can iter right tree
            return self.right_iter.is_some() 
                && self.right_iter.as_ref().unwrap().has_next();
        }
        
        // can iter left subtree or at least node value
        return true;
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
