impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let mut k = k;
        s.chars().take_while(|c| {
            if *c == ' ' {
                k -= 1;
                return k > 0;
            }
            return true;
        }).collect()
    }
}
