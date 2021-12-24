use std::collections::VecDeque;
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut grid = grid;
        let mut res = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    res += 1;
                    Self::delete_island(&mut grid, i, j);
                }
            }
        }
        return res;
    }
    fn delete_island(mut grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
        let m = grid.len();
        let n = grid[0].len();
        let mut deq: VecDeque<(usize, usize)> = VecDeque::new();
        deq.push_back((i, j));
        while let Some((i, j)) = deq.pop_front() {
            if grid[i][j] == '1' {
                grid[i][j] = '0';
                if 0 < i {
                    deq.push_back((i - 1, j));
                }
                if i + 1 < m {
                    deq.push_back((i + 1, j));
                }
                if 0 < j {
                    deq.push_back((i, j - 1));
                }
                if j + 1 < n {
                    deq.push_back((i, j + 1));
                }
            }
        }
    }
}
