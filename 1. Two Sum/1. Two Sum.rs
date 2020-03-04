use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let l = nums.len();
        let mut map: HashMap<i32, usize> = HashMap::new();
        for i in 0..l {
            let n = nums[i];
            let c = target - n;
            if let Some(&j) = map.get(&c) {
                return vec![j as i32, i as i32];
            }
            map.insert(n, i);
        }
        return vec![-1, -1];
    }
}
