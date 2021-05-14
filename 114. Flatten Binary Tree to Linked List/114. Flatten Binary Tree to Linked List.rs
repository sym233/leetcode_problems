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
type Node = Rc<RefCell<TreeNode>>;
impl Solution {
    pub fn flatten(root: &mut Option<Node>) {
        let mut nodes = Vec::<TreeNode>::new();
        let mut frags = Vec::<Option<Node>>::new();
        frags.push(root.take());
        while !frags.is_empty() {
            let frag = frags.pop().unwrap();
            if let Some(frag) = frag {
                let mut node = Rc::try_unwrap(frag).unwrap().into_inner();
                let left = node.left.take();
                let right = node.right.take();
                // println!("val = {}", node.val);
                nodes.push(node);
                // push frag to stack
                frags.push(right);
                frags.push(left);
                
            }
        }
        let mut leaf = Option::<Node>::None;
        while !nodes.is_empty() {
            let mut node = nodes.pop().unwrap();
            node.right = leaf;
            leaf = Some(Rc::new(RefCell::new(node)));
        }
        if let Some(leaf) = leaf {
            root.replace(leaf);
        }
    }
    
}