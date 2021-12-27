use std::collections::HashSet;
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let codes: HashSet<String> = (1..=26).map(|n| n.to_string()).collect();
        let l = s.len();
        let mut dp = vec![0; l + 1];
        dp[0] = 1;
        for i in 0..l {
            if i < l && codes.contains(&s[i..(i + 1)]) {
                dp[i + 1] += dp[i];
            }
            if i + 1 < l && codes.contains(&s[i..(i + 2)]) {
                dp[i + 2] += dp[i];
            }
        }
        return dp[l];
    }
}
