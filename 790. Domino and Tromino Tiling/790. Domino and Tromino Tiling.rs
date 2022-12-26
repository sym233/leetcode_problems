impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        let n = n as usize;
        let m = 1_000_000_007;
        let mut dp = vec![vec![0; 3]; n + 1];
        // dp[i][0]:
        // i - 1   i-th col
        // o        o
        // o        o
        // dp[i][1]:
        // i - 1   i-th col
        // o        o
        // o
        // dp[i][2]:
        // i - 1   i-th col
        // o        
        // o        o
        dp[0][0] = 1;
        for i in 1..=n {
            // vertical domino
            dp[i][0] += dp[i - 1][0];
            dp[i][0] %= m;
            if i >= 2 {
                // horizontal 2 dominos
                dp[i][0] += dp[i - 2][0];
                dp[i][0] %= m;
            }
            // oo +  o
            // o    oo
            dp[i][0] += dp[i - 1][1];
            dp[i][0] %= m;
            // o +  oo
            // oo    o
            dp[i][0] += dp[i - 1][2];
            dp[i][0] %= m;
            // o +  oo
            // oo    
            dp[i][1] += dp[i - 1][2];
            dp[i][1] %= m;
            // oo + 
            // o    oo
            dp[i][2] += dp[i - 1][1];
            dp[i][2] %= m;
            if i >= 2 {
                // o  + oo
                // o    o
                dp[i][1] += dp[i - 2][0];
                dp[i][1] %= m;
                // o  + o
                // o    oo
                dp[i][2] += dp[i - 2][0];
                dp[i][2] %= m;
            }
        }
        return dp[n][0];
    }
}
