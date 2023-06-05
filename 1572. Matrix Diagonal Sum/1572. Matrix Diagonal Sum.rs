impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let l = mat.len();
        let mut res = 0;
        for (i, row) in mat.into_iter().enumerate() {
            if i == l - i - 1 {
                res += row[i];
            } else {
                res += row[i] + row[l - i - 1];
            }
        }
        res
    }
}
