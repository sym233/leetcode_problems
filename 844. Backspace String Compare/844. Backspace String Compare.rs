impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        return Self::parse(s) == Self::parse(t);
    }
    fn parse(s: String) -> String {
        let mut res = String::new();
        for c in s.chars() {
            if c == '#' {
                res.pop();
            } else {
                res.push(c);
            }
        }
        return res;
    }
}
