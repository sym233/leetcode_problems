use std::collections::HashSet;
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let words: HashSet<String> = word_dict.into_iter().collect();
        let l = s.len();
        let mut prefix_included: Vec<bool> = vec![false; l + 1];
        prefix_included[0] = true;
        for i in 1..=l {
            for j in 0..i {
                if prefix_included[j] && words.contains(&s[j..i]) {
                    prefix_included[i] = true;
                    break;
                }
            }
        }
        return prefix_included[l];
    }
}
