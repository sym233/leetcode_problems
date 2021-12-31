impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let l = triangle.len();
        let mut t = triangle;
        for i in 1..l {
            for j in 0..=i {
                let mut prev = i32::max_value();                
                if j < i {
                    prev = prev.min(t[i - 1][j]);
                }
                if 0 < j {
                    prev = prev.min(t[i - 1][j - 1]);
                }
                t[i][j] += prev;
            }
        }
        return *t[l - 1].iter().min().unwrap();
    }
}
