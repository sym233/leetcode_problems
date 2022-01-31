impl Solution {
    pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        let w = 8usize;
        let mut board: Vec<Vec<bool>> = vec![vec![false; w]; w];
        for q in queens {
            board[q[0] as usize][q[1] as usize] = true;
        }
        
        let mut dirs: Vec<(i32, i32)> = Vec::new();
        for i in -1..=1 {
            for j in -1..=1 {
                if !(i == 0 && j == 0) {
                    dirs.push((i, j));
                }
            }
        }
        let mut res: Vec<Vec<i32>> = Vec::new();
        for dir in dirs {
            let mut x = king[0];
            let mut y = king[1];
            loop {
                x += dir.0;
                y += dir.1;
                if 0 <= x && x < w as i32 && 0 <= y && y < w as i32 {
                    if board[x as usize][y as usize] {
                        res.push(vec![x, y]);
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        return res;
    }
}
