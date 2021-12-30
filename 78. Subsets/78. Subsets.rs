impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let l = nums.len();
        let mut res = Vec::new();
        for i in 0..(1 << l) {
            let mut ss = Vec::new();
            for j in 0..l {
                if i & (1 << j) > 0 {
                    ss.push(nums[j]);
                }
            }
            res.push(ss);
        }
        return res;
    }
}
