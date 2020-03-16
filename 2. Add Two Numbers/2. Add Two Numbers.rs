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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn list_to_vec(list: &Option<Box<ListNode>>) -> Vec<i32> {
            let mut v: Vec<i32> = Vec::new();
            let mut p = list;
            while let Some(b) = p {
                v.push(b.val);
                p = &b.next;
            }
            return v;
        }

        let mut n1 = list_to_vec(&l1);
        let mut n2 = list_to_vec(&l2);
        let l = 1 + if n1.len() > n2.len() {
            n1.len()
        } else {
            n2.len()
        };

        let mut n3: Vec<i32> = vec![0; l];

        for i in 0..l {
            if n1.len() > i {
                n3[i] += n1[i];
            }
            if n2.len() > i {
                n3[i] += n2[i];
            }
            if n3[i] >= 10 {
                n3[i + 1] += n3[i] / 10;
                n3[i] %= 10;
            }
        }

        if n3.len() > 1 && n3.last().unwrap() == &0 {
            n3.pop();
        }
        let mut list: Option<Box<ListNode>> = None;
        
        for &n in n3.iter().rev() {
            let mut node = ListNode::new(n);
            node.next = list;
            list = Some(Box::new(node));
        }        
        return list;
    }
}
