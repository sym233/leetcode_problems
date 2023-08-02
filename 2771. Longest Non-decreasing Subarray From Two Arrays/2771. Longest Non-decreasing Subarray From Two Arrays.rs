impl Solution {
    pub fn max_non_decreasing_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let nums = vec![&nums1, &nums2];
        let n = nums1.len();
        let mut dp = vec![[1; 2]; n];
        let mut res = 1;
        for i in 1..n {
            for k1 in 0..2 {
                for k2 in 0..2 {
                    if nums[k1][i] >= nums[k2][i - 1] && dp[i][k1] < dp[i - 1][k2] + 1{
                        dp[i][k1] = dp[i - 1][k2] + 1;
                        res = res.max(dp[i][k1]);
                    }
                }
            }
        }
        res
    }
}
