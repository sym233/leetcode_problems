use std::collections::HashMap;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut m = HashMap::<i32,i32>::new();
        let mut res = 0;
        for ans in answers {
            if let Some(v) = m.get_mut(&ans) {
                *v += 1;
            } else {
                m.insert(ans, 1);
            }
        }

        for (k, v) in m {
            let mut v = v;
            while v > k + 1 {
                res += k + 1;
                v -= k + 1;
            }
            res += k + 1;
        }
        res
    }
}
