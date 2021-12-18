impl Solution {
    pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
        let row = rows as usize;
        let chars: Vec<char> = encoded_text.chars().collect();
        let mut res: Vec<char> = Vec::new();
        let l = chars.len();
        let col = l / row;
        for i in 0..col {
            let mut p = i;
            while p < l {
                res.push(chars[p]);
                p += col + 1;
            }
        }
        let res: String = res.into_iter().collect();
        return res.trim_end().to_string();
    }
}
