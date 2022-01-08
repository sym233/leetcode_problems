impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let l = digits.len();
        let mut letters: Vec<String> = vec![String::new()];
        let mut next_letters: Vec<String> = Vec::new();
        let dtl: [&str; 10] = [
            "",
            "",
            "abc",
            "def",
            "ghi",
            "jkl",
            "mno",
            "pqrs",
            "tuv",
            "wxyz",            
        ];
        for d in digits.chars() {
            let d = (d as u8 - '0' as u8) as usize;
            for c in dtl[d].chars() {
                for s in &letters {
                    let mut s = s.clone();
                    s.push(c);
                    next_letters.push(s);
                }
            }
            letters = next_letters;
            next_letters = Vec::new();
        }
        return letters.into_iter().filter(|s| !s.is_empty()).collect();
    }
}
