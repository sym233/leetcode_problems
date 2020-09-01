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
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        const M: i64 = 1_000_000_007;
        
        let mut sub_sums: Vec<i32> = Vec::new();
        
        fn s(node: &Option<Rc<RefCell<TreeNode>>>, mut sub_sums: &mut Vec<i32>) -> i32 {
            if node.is_none() {
                return 0;
            }
            let node = node.as_ref().unwrap().as_ref().borrow();
            let sum = s(&node.left, &mut sub_sums) + s(&node.right, &mut sub_sums) + node.val;
            
            sub_sums.push(sum);
            
            return sum;
        };
        
        let root_sum = s(&root, &mut sub_sums);
        let mut d = root_sum;
        let mut closest_sub_sum = 0;
        
        
        for sum in sub_sums {
            let td = (root_sum - sum * 2).abs();
            if td < d {
                closest_sub_sum = sum;
                d = td;
            }
        }
        
        let root_sum = root_sum as i64;
        let closest_sub_sum = closest_sub_sum as i64;
        
        return ((root_sum - closest_sub_sum) * closest_sub_sum % M) as i32;
    }
}
