use std::collections::VecDeque;
impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        const INF: i32 = 1_000_000;
        let mut distance = vec![vec![INF; n]; n];
        if grid[0][0] == 1 {
            return -1;
        }
        distance[0][0] = 1;
        // cannot run VecDeque::from([(0, 0)]), strange
        // maybe the rust version is dated
        let mut q: VecDeque<(usize, usize)> = VecDeque::from(vec![(0, 0)]);
        'q: while let Some((r, c)) = q.pop_front() {
            for &dr in &[-1, 0, 1] {
                for &dc in &[-1, 0, 1] {
                    let r2 = r as i32 + dr;
                    let c2 = c as i32 + dc;
                    if 0 <= r2 && r2 < n as i32 && 0 <= c2 && c2 < n as i32 {
                        let r2 = r2 as usize;
                        let c2 = c2 as usize;
                        if grid[r2][c2] == 1 {
                            continue;
                        }
                        if distance[r2][c2] > distance[r][c] + 1 {
                            distance[r2][c2] = distance[r][c] + 1;
                            if r2 == n - 1 && c2 == n - 1 {
                                break 'q;
                            }
                            q.push_back((r2, c2));
                        }
                    }
                }
            }
        }
        let d = distance[n - 1][n - 1];
        return if d == INF { -1 } else { d };
    }
}