use std::collections::{ HashMap, VecDeque };
impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let l = nums.len();
        let mut m: HashMap<i32, i32> = HashMap::new();
        for n in nums {
            if let Some(v) = m.get_mut(&n) {
                *v += 1;
            } else {
                m.insert(n, 1);
            }
        }
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut q: VecDeque<(Vec<i32>, HashMap<i32, i32>)> = VecDeque::from(vec![(Vec::new(), m)]);
        while let Some((p, m)) = q.pop_front() {
            for (&n, &count) in m.iter() {
                if count > 0 {
                    let mut p = p.clone();
                    p.push(n);
                    if p.len() == l {
                        res.push(p);
                        continue;
                    }
                    let mut m = m.clone();
                    *m.get_mut(&n).unwrap() -= 1;
                    q.push_back((p, m));
                }
            }
        }
        return res;
    }
}
