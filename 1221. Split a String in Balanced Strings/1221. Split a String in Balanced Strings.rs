impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut stack: Vec<char> = Vec::new();
        let mut count: i32 = 0;
        for c in s.chars() {
            if stack.len() == 0 || stack.last() == Some(&c) {
                stack.push(c);
            } else {
                stack.pop();
            }
            if stack.len() == 0 {
                count += 1;
            }
        }
        return count;
    }
}
