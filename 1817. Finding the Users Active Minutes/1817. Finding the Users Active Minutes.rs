use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut map = HashMap::<i32, HashSet<i32>>::new();
        for log in logs {
            let u = log[0];
            let m = log[1];
            map.entry(u)
                .and_modify(|s| { s.insert(m); })
                .or_insert(HashSet::from([m]));
        }
        let mut res = vec![0; k as usize];
        for s in map.values() {
            res[s.len() - 1] += 1;
        }
        return res;
    }
}
