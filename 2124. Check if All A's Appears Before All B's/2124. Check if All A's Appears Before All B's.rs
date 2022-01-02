impl Solution {
    pub fn check_string(s: String) -> bool {
        return !s.chars().skip_while(|&c| c == 'a').any(|c| c == 'a');
    }
}
