impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let m = grid.len();
        let n = grid[0].len();
        for i in 0..m {
            for j in 0..n {
                let mut prev: Vec<i32> = Vec::new();
                if i > 0 {
                    prev.push(grid[i - 1][j]);
                }
                if j > 0 {
                    prev.push(grid[i][j - 1]);
                }
                if let Some(min) = prev.into_iter().min() {
                    grid[i][j] += min;
                }
            }
        }
        return grid[m - 1][n - 1];
    }
}
