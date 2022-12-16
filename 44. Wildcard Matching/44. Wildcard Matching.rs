impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        
        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
        let mut prev_match = vec![0usize];
        dp[0][0] = true;

        for j in 0..p.len() {
            let mut next_match = vec![];
            if p[j] == b'*' {
                if let Some(&i) = prev_match.first() {
                    for i in i..=s.len() {
                        dp[i][j + 1] = true;
                        next_match.push(i);
                    }
                }
            } else {
                for i in prev_match {
                    if i < s.len() && dp[i][j] && (s[i] == p[j] || p[j] == b'?') {
                        dp[i + 1][j + 1] = true;
                        next_match.push(i + 1);
                    }
                }
            }
            prev_match = next_match;
        }
        return dp[s.len()][p.len()];
    }
}
