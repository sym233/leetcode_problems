impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        
        let mut s = 0;
        let mut e = m;
        
        let r = loop {
            let i = (s + e) / 2;
            if e - s <= 1 {
                break i;
            }
            let v = matrix[i][0];
            if v < target {
                s = i;
            } else {
                e = i;
            }
        };
        let r = if matrix[r][0] <= target && target <= matrix[r][n - 1] {
            r
        } else if r + 1 < m && matrix[r + 1][0] <= target && target <= matrix[r + 1][n - 1] {
            r + 1
        } else {
            return false;
        };
        
        s = 0;
        e = n;
        
        let c = loop {
            let i = (s + e) / 2;
            if e - s <= 1 {
                break i;
            }
            let v = matrix[r][i];
            if v < target {
                s = i;
            } else {
                e = i;
            }
        };
        return matrix[r][c] == target || (c + 1 < n && matrix[r][c + 1] == target);
    }
}
