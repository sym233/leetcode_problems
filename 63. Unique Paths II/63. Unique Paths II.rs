impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        if obstacle_grid[0][0] == 1 || obstacle_grid[m - 1][n - 1] == 1 {
            return 0;
        }
        let mut path: Vec<Vec<i32>> = vec![vec![0; n]; m];
        path[0][0] = 1;
        
        for i in 0..m {
            for j in 0..n {
                if obstacle_grid[i][j] == 1 {
                    continue;
                }
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
