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
use std::collections::{BTreeMap, VecDeque};
impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn walk(node: Option<Rc<RefCell<TreeNode>>>, f: &dyn Fn(i32, i32, i32) -> ()) {
            let mut q: VecDeque<(Option<Rc<RefCell<TreeNode>>>, i32, i32)> = VecDeque::new();
            q.push_back((node, 0, 0));
            
            while !q.is_empty() {
                if let Some((node, x, y)) = q.pop_front() {
                    if node.is_some() {
                        let node_ref = node.as_ref().unwrap().as_ref().borrow();
                        f(x, y, node_ref.val);
                        q.push_back((node_ref.left.as_ref().map(|n| n.clone()), x - 1, y - 1));
                        q.push_back((node_ref.right.as_ref().map(|n| n.clone()), x + 1, y - 1));
                    }
                }
            }
        }
    
        let mr: RefCell<BTreeMap<i32, BTreeMap<i32, Vec<i32>>>> = RefCell::new(BTreeMap::new());

        walk(root.map(|n| n.clone()), &|x, y, val| {
            let mut tree = mr.borrow_mut();
            if let Some(t2) = tree.get_mut(&x) {
                if let Some(v) = t2.get_mut(&y) {
                    v.push(val);
                } else {
                    t2.insert(y, vec![val]);
                }
            } else {
                let mut t2 = BTreeMap::new();
                t2.insert(y, vec![val]);
                tree.insert(x, t2);
            }
        });

        let mut res: Vec<Vec<i32>> = Vec::new();

        for (_, t2) in mr.borrow().iter() {
            let mut v1: Vec<i32> = Vec::new();
            for (_, v) in t2.iter().rev() {
                let mut v2 = v.clone();
                v2.sort();
                v1.append(&mut v2);
            }
            res.push(v1);
        }
        return res;
    }
}
