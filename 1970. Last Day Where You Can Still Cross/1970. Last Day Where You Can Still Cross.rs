use std::collections::VecDeque;

impl Solution {
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {

        fn pred(row: i32, col: i32, cells: &[Vec<i32>]) -> bool {
            let mut mat = vec![vec![0; col as usize + 1]; row as usize + 1];

            for cell in cells {
                let r = cell[0] as usize;
                let c = cell[1] as usize;
                mat[r][c] = 1;
            }

            let mut q: VecDeque<(usize, usize)> = VecDeque::new();

            for c in 1..=col {
                if mat[1][c as usize] == 0 {
                    q.push_back((1, c as usize));
                }
            }

            while let Some((r, c)) = q.pop_front() {
                if mat[r][c] != 0 {
                    continue;
                }
                mat[r][c] = 2;
                if r == row as usize {
                    return true;
                }
                for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let r2 = (r as i32) + dr;
                    let c2 = (c as i32) + dc;
                    if 0 < r2 && r2 <= row && 0 < c2 && c2 <= col && mat[r2 as usize][c2 as usize] == 0 {
                        q.push_back((r2 as usize, c2 as usize));
                    }
                }
            }

            false
        }
        
        // for d in 1..cells.len() {
        //     if (!pred(row, col, &cells[0..d])) {
        //         return d as i32 - 1;
        //     }
        // }

        let mut l = 1;
        let mut r = cells.len() + 1;
        while r - l > 1 {
            let m = (l + r) / 2; 
            if pred(row, col, &cells[0..m]) {
                l = m;
            } else {
                r = m;
            }
        }
        l as i32
    }
}
