impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        fn add(s1: &str, s2: &str) -> String {
            let mut carry = 0;
            let mut res = Vec::new();
            let mut i1 = s1.chars().rev();
            let mut i2 = s2.chars().rev();
            loop {
                let x1 = i1.next().map(|c| c.to_digit(10).unwrap());
                let x2 = i2.next().map(|c| c.to_digit(10).unwrap());
                if x1.is_none() && x2.is_none()  {
                    break;
                }
                carry += x1.unwrap_or(0) + x2.unwrap_or(0);
                res.push(carry % 10);
                carry /= 10;
            }
            if carry > 0 {
                res.push(carry);
            }
            if res.is_empty() {
                return String::from("0");
            }
            return res.iter().rev().map(|v| v.to_string()).collect::<String>();
        }
        fn dfs(n1: String, n2: String, rem: String) -> bool {
            if rem.is_empty() {
                return true;
            }
            if (n1.len() > 1 && n1.starts_with('0') || n2.len() > 1 && n2.starts_with('0')) {
                return false;
            }
            let added = add(&n1, &n2);
            if rem.starts_with(&added) {
                let al = added.len();
                return dfs(n2, added, String::from(&rem[al..]));
            }
            return false;
        }
        
        
        let l = num.len();
        for l1 in 1..=(l / 2) {
            for l2 in 1..=(l / 2) {
                let n1 = String::from(&num[..l1]);
                let n2 = String::from(&num[l1..(l1 + l2)]);
                let rem = String::from(&num[(l1 + l2)..]);
                if !rem.is_empty() && dfs(n1, n2, rem) {
                    return true;
                }
            }
        }
        return false;        
    }
}
