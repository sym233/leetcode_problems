impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s: Vec<char> = s.chars().collect();
        let l = s.len();
        let mut res: &[char] = &[];
        for i in 0..l {
            let mut is_p_even = true;
            let mut is_p_odd = true;
            for hl in 0..l {
                if is_p_odd && hl <= i && i + hl + 1 <= l {
                    if s[i - hl] == s[i + hl] {
                        if res.len() < 2 * hl + 1 {
                            res = &s[(i - hl)..(i + hl + 1)];                            
                        }
                    } else {
                        is_p_odd = false;
                    }
                }
                if is_p_even && hl <= i && i + hl + 2 <= l {
                    if s[i - hl] == s[i + 1 + hl] {
                        if res.len() < 2 * hl + 2 {
                            res = &s[(i - hl)..(i + hl + 2)];
                        }
                    } else {
                        is_p_even = false;
                    }
                }
                if i < hl || l < i + hl + 2 || (!is_p_even && !is_p_odd) {
                    break;
                }
            }
        }
        return res.into_iter().collect();
    }
}
