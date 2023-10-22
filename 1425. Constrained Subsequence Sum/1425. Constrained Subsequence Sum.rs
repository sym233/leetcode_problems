use std::collections::BTreeMap;

impl Solution {
    pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let l = nums.len();
        let mut m = BTreeMap::<i32, usize>::new();
        let mut dp = vec![nums[0]];

        for i in 1..l {
            m.entry(dp[i - 1])
                .and_modify(|e| *e += 1)
                .or_insert(1);
            if k < i {
                let oldest = dp[i - k - 1];
                if let Some(v) = m.get_mut(&oldest) {
                    if *v == 1 {
                        m.remove(&oldest);
                    } else {
                        *v -= 1;
                    }
                }
            }

            let mut num = nums[i];
            num += (*m.iter().rev().next().unwrap().0).max(0);
            dp.push(num);
        }
        return *dp.iter().max().unwrap();
    }
}
