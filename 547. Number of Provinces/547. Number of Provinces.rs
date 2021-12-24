impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut uf: Vec<usize> = (0..n).collect();
        fn pa(mut uf: &mut Vec<usize>, i: usize) -> usize {
            if uf[i] == i {
                return i;
            }
            let mut parents: Vec<usize> = Vec::new();
            let mut j = i;
            while uf[j] != uf[uf[j]] {
                parents.push(uf[j]);
                j = uf[j];
            }            
            let p = uf[j];
            for parent in parents {
                uf[parent] = p;
            }
            return p;
        }
        for i in 0..n {
            for j in (i + 1)..n {
                if is_connected[i][j] == 1 {
                    let pi = pa(&mut uf, i);
                    let pj = pa(&mut uf, j);
                    if pi < pj {
                        uf[pj] = pi;
                    } else {
                        uf[pi] = pj;
                    }
                }
            }
        }
        return uf.into_iter().enumerate().filter(|&(i, n)| i == n).count() as i32;
    }
}
