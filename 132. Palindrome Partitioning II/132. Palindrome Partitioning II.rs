impl Solution {
    pub fn min_cut(s: String) -> i32 {
        fn is_pal(s: &[u8]) -> bool {
            let l = s.len();
            if l <= 1 {
                return true;
            }
            for i in 0..l / 2 {
                if s[i] != s[l - i - 1] {
                    return false;
                }
            }
            true
        }

        let s = s.as_bytes();
        let l = s.len();
        let mut dp = vec![0; l + 1];
        for i in 1..=l {
            dp[i] = i;
            for j in 0..i {
                if is_pal(&s[j..i]) {
                    let first_cut = if j == 0 { 0 } else { 1 };
                    dp[i] = dp[i].min(dp[j] + first_cut);
                }
            }
        }
        // println!("{:?}", dp);
        dp[l] as i32
    }
}
