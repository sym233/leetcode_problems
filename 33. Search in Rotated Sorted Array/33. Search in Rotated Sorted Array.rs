impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let l = nums.len();
        let first = nums[0];
        let mut s = 0;
        let mut e = l;
        
        return loop {
            let i = (s + e) / 2;
            let n = nums[i];
            if n == target {
                break i as i32;
            }
            if i == s {
                break -1;
            }
            if n < target {
                if n < first && first <= target {
                    e = i;
                } else {
                    s = i;
                }
            } else {
                if target < first && first <= n {
                    s = i;
                } else {
                    e = i;
                }
            }
        };
    }
}
