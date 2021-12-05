impl Solution {
    pub fn count_valid_words(sentence: String) -> i32 {
        let mut res = 0;
        
        for word in sentence.split(' ') {
            let l = word.len();
            if l == 0 {
                continue;
            }
            if word.contains(char::is_numeric) {
                continue;
            }
            if let Some(i) = word.find('-') {
                if i == 0 || i == l - 1 {
                    continue;
                }
                if word[i+1..].contains('-') {
                    continue;
                }
                if !word[i+1..].starts_with(|c: char| c.is_ascii_lowercase()) {
                    continue;
                }
            }
            if let Some(i) = word.find(|c: char| [',', '.', '!'].contains(&c)) {
                if i != l - 1 {
                    continue;
                }
            }
            // println!("{}", word);
            res += 1;
        }
        return res;
    }
}
