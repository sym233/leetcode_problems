impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        const N: usize = 500;
        let n = n as usize;
        let mut grid = [[0usize; N]; N];
        
        for r in 0..n {
            for c in 0..n {
                grid[r][c] = 1;
            }
        }
        
        for mine in mines {
            let r = mine[0] as usize;
            let c = mine[1] as usize;
            grid[r][c] = 0;
        }
        
        let mut current: Vec<(usize, usize)> = vec![];
        let mut next: Vec<(usize, usize)>;
        
        for r in 0..n {
            for c in 0..n {
                if grid[r][c] > 0 {
                    current.push((r, c));
                }
            }
        }
        
        let mut max_len = 0;
        
        while !current.is_empty() {
            next = vec![];
            for (x, y) in current {
                max_len = grid[x][y];
                if max_len <= x && grid[x - max_len][y] > 0 &&
                    x + max_len < n && grid[x + max_len][y] > 0 &&
                    max_len <= y && grid[x][y - max_len] > 0 &&
                    y + max_len < n && grid[x][y + max_len] > 0
                {
                    grid[x][y] += 1;
                    next.push((x, y));
                }
            }
            current = next;
        }
        
        return max_len as i32;
    }
}
