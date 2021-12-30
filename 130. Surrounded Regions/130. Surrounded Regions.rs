use std::collections::{ HashSet, VecDeque };
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let m = board.len();
        let n = board[0].len();
        type P = (usize, usize);
        let mut edge_os: HashSet<P> = HashSet::new();
        let mut q: VecDeque<P> = VecDeque::new();
        for r in 0..m {
            for &c in &[0, n - 1] {
                if board[r][c] == 'O' {
                    edge_os.insert((r, c));
                    q.push_back((r, c));
                }                
            }
        }
        for &r in &[0, m - 1] {
            for c in 0..n {
                if board[r][c] == 'O' {
                    edge_os.insert((r, c));
                    q.push_back((r, c));
                }                
            }
        }
        while let Some((r, c)) = q.pop_front() {
            for &(dr, dc) in &[(-1, 0), (1, 0), (0, 1), (0, -1)] {
                let r2 = r as i32 + dr;
                let c2 = c as i32 + dc;
                if 0 <= r2 && r2 < m as i32 && 0 <= c2 && c2 < n as i32 {
                    let r2 = r2 as usize;
                    let c2 = c2 as usize;
                    if board[r2][c2] == 'O' && !edge_os.contains(&(r2, c2)) {
                        edge_os.insert((r2, c2));
                        q.push_back((r2, c2));
                    }
                }
            }
        }
        for r in 0..m {
            for c in 0..n {
                if board[r][c] == 'O' && !edge_os.contains(&(r, c)) {
                    board[r][c] = 'X';
                }
            }
        }        
    }
}