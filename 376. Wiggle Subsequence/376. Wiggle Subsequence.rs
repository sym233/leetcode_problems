impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let l = nums.len();        
        let mut dp: Vec<Vec<i32>> = vec![vec![0; 2]; l];
        // dp[i][0] prev diff is positive
        // dp[i][1] prev diff is negative       
        
        for i in 0..l {
            if i == 0 {
                dp[i][0] = 1;
                dp[i][1] = 1;
            }
            for j in 0..i {
                let sign = (nums[i] - nums[j]).signum();
                match sign {
                    1 => {
                        dp[i][0] = dp[i][0].max(dp[j][1] + 1);
                    },
                    -1 => {
                        dp[i][1] = dp[i][1].max(dp[j][0] + 1);
                    },
                    _ => continue,
                }
            }
            dp[i][0] = dp[i][0].max(1);
            dp[i][1] = dp[i][1].max(1);
        }
        return dp[l - 1][0].max(dp[l - 1][1]);
    }
}
