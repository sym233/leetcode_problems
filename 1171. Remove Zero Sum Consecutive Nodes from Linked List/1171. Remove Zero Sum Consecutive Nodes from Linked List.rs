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
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut seq: Vec<i32> = Vec::new();
        let mut head = head;
        while head.is_some() {
            let tail = head.as_mut().unwrap().next.take();
            seq.push(head.unwrap().val);
            let l = seq.len();
            let mut sum = 0;
            for i in (0..l).rev() {
                sum += seq[i];
                if sum == 0 {
                    seq.truncate(i as usize);
                    break;
                }
            }
            head = tail;
        }
        return Self::gen_list(&seq[..], None);
    }
    
    fn gen_list(v: &[i32], tail: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let l = v.len();
        if l == 0 {
            return tail;
        }
        let node = ListNode {
            next: tail,
            val: *v.last().unwrap(),
        };
        return Self::gen_list(&v[..(l - 1)], Some(Box::new(node)));
    }
}
