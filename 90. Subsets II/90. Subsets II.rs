use std::collections::HashMap;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut m: HashMap<i32, i32> = HashMap::new();
        for n in nums {
            if let Some(count) = m.get_mut(&n) {
                *count += 1;
            } else {
                m.insert(n, 1);
            }
        }
        let mut res: Vec<Vec<i32>> = vec![vec![]];
        let mut next: Vec<Vec<i32>> = Vec::new();
        
        for (n, count) in m {
            for ss in res {
                for i in 0..=count {
                    let mut ss = ss.clone();
                    for j in 0..i {
                        ss.push(n);                        
                    }
                    next.push(ss);
                }
            }
            res = next;
            next = Vec::new();
        }
        return res;
    }
}