impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let mut maxl = 0;
        let mut count = 0;

        // (len, count)
        let mut dp = vec![(1, 1); l];
        // dp[i] -> num[..=i] have LIS and its length and count

        for i in 0..l {
            for j in 0..i {
                if nums[i] > nums[j] {
                    if dp[i].0 == dp[j].0 + 1 {
                        dp[i].1 += dp[j].1;
                    }
                    if dp[i].0 < dp[j].0 + 1 {
                        dp[i].0 = dp[j].0 + 1;
                        dp[i].1 = dp[j].1;
                    }
                }
            }
            if dp[i].0 == maxl {
                count += dp[i].1;
            }
            if dp[i].0 > maxl {
                maxl = dp[i].0;
                count = dp[i].1;
            }
        }
        count
    }
}
