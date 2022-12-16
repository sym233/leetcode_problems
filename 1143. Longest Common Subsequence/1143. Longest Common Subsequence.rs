impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let s1 = text1.as_bytes();
        let l1 = s1.len();
        let s2 = text2.as_bytes();
        let l2 = s2.len();
        let mut dp = vec![vec![0; l2 + 1]; l1 + 1];

        for i in 0..l1 {
            for j in 0..l2 {
                let add = if s1[i] == s2[j] {
                    1
                } else {
                    0
                };
                dp[i + 1][j + 1] = *[
                    dp[i + 1][j + 1],
                    dp[i][j + 1],
                    dp[i + 1][j],
                    dp[i][j] + add
                ].into_iter().max().unwrap();
            }
        }
        return dp[l1][l2];
    }
}
