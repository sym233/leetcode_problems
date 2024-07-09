impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        fn c_to_n(c: char)-> usize {
            let c = c as u8 as usize;
            let A = 'A' as u8 as usize;
            let a = 'a' as u8 as usize;
            if a <= c { 26 + c - a } else { c - A }
        }
        let mut q = Vec::new();
        let mut f = 0;
        let mut dict = [0; 52];
        let mut curr = [0; 52];
        let mut min_len = s.len() + 1;
        let mut p = 0;
        
        for c in t.chars() {
            dict[c_to_n(c)] += 1;
        }

        for c in s.chars() {
            curr[c_to_n(c)] += 1;
            q.push(c);

            let mut included = true;
            for i in 0..52 {
                if curr[i] < dict[i] {
                    included = false;
                    break;
                }
            }
            while included {
                if q.len() - f < min_len {
                    p = f;
                    min_len = q.len() - f;
                }
                // println!("{}", q.iter().collect::<String>());
                included = false;
                if f < q.len() {
                    if curr[c_to_n(q[f])] > dict[c_to_n(q[f])] {
                        curr[c_to_n(q[f])] -= 1;
                        f += 1;
                        included = true;
                    }
                }
            }
        }
        if min_len == s.len() + 1 {
            return String::new();
        }
        q[p..p + min_len].iter().collect()
    }
}
