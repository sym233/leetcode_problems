impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        
        let mut vlines: Vec<Vec<(usize, usize)>> = vec![vec![(0, n - 1)]; n];        
        let mut hlines: Vec<Vec<(usize, usize)>> = vec![vec![(0, n - 1)]; n];
        
        for mine in mines {
            let r = mine[0] as usize;
            let c = mine[1] as usize;
            
            let mut new_lines: Vec<(usize, usize)> = vec![];
            for line in vlines[c].iter() {
                if !(line.0 <= r && r <= line.1) {
                    new_lines.push(line.clone());
                } else {
                    if 1 <= r && line.0 <= r - 1 {
                        new_lines.push((line.0, r - 1));
                    }
                    if r + 1 <= line.1 {
                        new_lines.push((r + 1, line.1));
                    }
                }
            }
            vlines[c] = new_lines;
            
            let mut new_lines: Vec<(usize, usize)> = vec![];
            for line in hlines[r].iter() {
                if !(line.0 <= c && c <= line.1) {
                    new_lines.push(line.clone());
                } else {
                    if 1 <= c && line.0 <= c - 1 {
                        new_lines.push((line.0, c - 1));
                    }
                    if c + 1 <= line.1 {
                        new_lines.push((c + 1, line.1));
                    }
                }
            }
            hlines[r] = new_lines;
        }
        
        let mut max_len = 0;
        
        for (r, hline) in hlines.iter().enumerate() {
            for h in hline.iter() {
                // h.1 - h.0 + 1 < 2 * max_len - 1
                // greater than current cross length
                if h.1 - h.0 + 2 < 2 * max_len {
                    continue;
                }
                for (c, vline) in vlines.iter().enumerate() {
                    for v in vline.iter() {
                        if v.1 - v.0 + 2 < 2 * max_len {
                            continue;
                        }
                        if v.0 <= r && r <= v.1 && h.0 <= c && c <= h.1 {
                            let lens = [
                                r -  v.0,
                                v.1 - r,
                                c - h.0,
                                h.1 - c,
                            ];
                            let m = *lens.iter().min().unwrap() + 1;
                            if m > max_len {
                                max_len = m;
                            }
                        }
                    }
                }
                
            }
        }
        return max_len as i32;
    }
}
