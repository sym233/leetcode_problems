use std::collections::{ VecDeque, HashMap };
impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut q: VecDeque<(HashMap<i32, i32>, Vec<i32>)> = VecDeque::new();
        let mut m: HashMap<i32, i32> = HashMap::new();
        for n in candidates {
            if let Some(count) = m.get_mut(&n) {
                *count += 1;
            } else {
                m.insert(n, 1);
            }
        }
        q.push_back((m, Vec::new()));
        let mut res: Vec<Vec<i32>> = Vec::new();
        
        while let Some((m, comb)) = q.pop_front() {
            for (&n, &count) in &m {
                if count == 0 {
                    continue;
                }
                let s = comb.iter().sum::<i32>() + n;
                if s <= target {
                    if let Some(&last) = comb.last() {
                        if n < last {
                            continue;
                        }
                    }
                    let mut comb = comb.clone();
                    comb.push(n);
                    if s == target {
                        res.push(comb);
                    } else {
                        let mut m = m.clone();
                        *m.get_mut(&n).unwrap() -= 1;
                        q.push_back((m, comb));
                    }
                }
            }
        }
        return res;
    }
}
