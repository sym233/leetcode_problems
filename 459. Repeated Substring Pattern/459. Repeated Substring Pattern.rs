impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let l = s.len();
        if l < 2 {
            return false;
        }
        for i in 1..=(l / 2) {
            if l % i == 0 {
                let mut eq = true;
                for j in 1..(l / i) {
                    if s[0..i] != s[(j * i)..(j * i + i)] {
                        eq = false; 
                        break;
                    }
                }
                if eq {
                    return true;
                }
            }
        }
        return false;
    }
}
