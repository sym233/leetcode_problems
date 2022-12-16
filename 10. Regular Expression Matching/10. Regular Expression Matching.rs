impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        fn f(s: &[u8], p: &[u8]) -> bool {
            if s.is_empty() && p.is_empty() {
                return true;
            }
            if p.is_empty() {
                return false
            }
            if (p.len() > 1 && p[1] == b'*') {
                if p[0] == b'.' {
                    for i in 0..=s.len() {
                        if f(&s[i..], &p[2..]) {
                            return true;
                        }
                    }
                    return false;
                } else {
                    for i in 0..=s.len() {
                        if i == 0 || s[i - 1] == p[0] {
                            if f(&s[i..], &p[2..]) {
                                return true;
                            }
                        } else {
                            break;
                        }
                    } 
                    return false;
                }
            }
            if s.len() > 0 && (s[0] == p[0] || p[0] == b'.') {
                return f(&s[1..], &p[1..]);
            }
            return false;
        }
        f(s.as_bytes(), p.as_bytes())
    }
}
