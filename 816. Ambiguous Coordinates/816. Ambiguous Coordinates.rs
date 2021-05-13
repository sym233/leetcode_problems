impl Solution {
    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        let mut res = Vec::<String>::new();
        let len = s.len();
        for i in 2..=len - 2 {
            let left = s[1..i].to_string();
            let right = s[i..len - 1].to_string();
            let lefts = Self::add_point(left);
            let rights = Self::add_point(right);
            // println!("{:?} {:?}", lefts, rights);
            for l in lefts {
                for r in rights.iter() {
                    res.push(format!("({}, {})", l, r));
                }
            }            
        }
        return res;
    }
    
    fn add_point(s: String) -> Vec<String> {
        let mut ss = Vec::<String>::new();
        if Self::is_valid(&s) {
            ss.push(s.clone());
        }
        let len = s.len();
        for i in 1..len {
            let t = format!("{}.{}", &s[..i], &s[i..]);
            if Self::is_valid(&t) {
                ss.push(t.clone());
            }
        }
        return ss;
    }
    
    fn is_valid(s: &String) -> bool {
        let has_point = s.contains('.');
        if has_point {
            if s.ends_with('0') {
                return false;
            }
            if s.starts_with('0') && !s.starts_with("0.") {
                return false;
            }
        } else {
            if s.starts_with('0') && *s != "0" {
                return false;
            }
        }
        return true;
    }
}
