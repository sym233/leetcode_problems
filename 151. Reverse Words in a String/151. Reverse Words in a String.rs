impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.trim()
            .split_whitespace()
            .rev()
            .collect::<Vec<&str>>()
            .join(" ")
    }
}
