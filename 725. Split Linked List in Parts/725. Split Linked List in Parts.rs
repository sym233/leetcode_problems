// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn split_list_to_parts(root: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let k = k as usize;
        let mut root = root;
        let mut res: Vec<Option<Box<ListNode>>> = vec![None; k];
        let mut l = 0usize;
        let mut p = &root;
        while(p.is_some()) {
            p = &p.as_ref().unwrap().next;
            l += 1;
        }
        let count = l / k;
        let rem = l % k;
        for i in 0..k {
            let c = if i < rem {
                count + 1
            } else {
                count
            };
            let mut p = &mut root;
            for _ in 0..c {
                p = &mut p.as_mut().unwrap().next;
            }
            let tail = p.take();
            res[i] = root;
            root = tail;
        }
        return res;
    }
}
