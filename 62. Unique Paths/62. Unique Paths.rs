impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut path: Vec<Vec<i32>> = vec![vec![0; n]; m];
        path[0][0] = 1;
        
        for i in 0..m {
            for j in 0..n {
                if i > 0 {
                    path[i][j] += path[i - 1][j];
                }
                if j > 0 {
                    path[i][j] += path[i][j - 1];
                }
            }
        }
        return path[m - 1][n - 1];
    }
}
