impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let b1 = word1.as_bytes();
        let b2 = word2.as_bytes();
        let n1 = b1.len();
        let n2 = b2.len();
        let mut dp = vec![vec![usize::MAX; n2 + 1]; n1 + 1];
        // dp[i][j] -> edit dist between w1[..i] and w2[..j]

        for i in 0..=n1 {
            for j in 0..=n2 {
                if i == 0 || j == 0 {
                    dp[i][j] = i + j;
                    continue;
                }
                let diag = if b1[i - 1] == b2[j - 1] {
                    0
                } else {
                    1
                };
                dp[i][j] = *[
                    dp[i - 1][j] + 1,
                    dp[i][j - 1] + 1,
                    dp[i - 1][j - 1] + diag,
                ].into_iter().min().unwrap();
            }
        }
        return dp[n1][n2] as i32;
    }
}
