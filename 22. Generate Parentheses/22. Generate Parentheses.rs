use std::collections::VecDeque;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let mut q: VecDeque<(i32, i32, String)> = VecDeque::new();
        q.push_back((0, 0, String::new()));
        
        while let Some((a, b, s)) = q.pop_front() {
            if b == n {
                res.push(s);
                continue;
            }
            if a < n {
                q.push_back((a + 1, b, format!("{}(", s)));
            }
            if b < a {
                q.push_back((a, b + 1, format!("{})", s)));
            }
        }
        return res;
    }
}
