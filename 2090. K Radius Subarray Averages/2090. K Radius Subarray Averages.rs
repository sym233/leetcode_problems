impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let l = nums.len();
        let mut s: i64 = 0;
        let mut c = 0;
        let mut res: Vec<i32> = Vec::new();
        for i in 0..l {
            s += nums[i] as i64;
            c += 1;
            if c <= k {
                // pass
            } else if c < 2 * k + 1 {
                res.push(-1);
            } else if c == 2 * k + 1 {
                res.push((s / (2 * k + 1) as i64) as i32);
            } else {
                s -= nums[i - 2 * k as usize - 1] as i64;
                c -= 1;
                res.push((s / (2 * k + 1) as i64) as i32);
            }
        }
        while res.len() < l {
            res.push(-1);
        }
        return res;
    }
}
