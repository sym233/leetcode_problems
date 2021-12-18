
impl Solution {
    pub fn friend_requests(n: i32, restrictions: Vec<Vec<i32>>, requests: Vec<Vec<i32>>) -> Vec<bool> {
        let mut uf: Vec<usize> = Vec::new();
        for i in 0..n as usize {
            uf.push(i);
        }
        let mut res: Vec<bool> = Vec::new();
        fn find_root(mut arr: &mut Vec<usize>, el: usize) -> usize {
            while arr[el] != arr[arr[el]] {
                arr[el] = arr[arr[el]];
            }
            return arr[el];
        }
        for req in requests {
            let mut uf2 = uf.clone();
            let rx = find_root(&mut uf2, req[0] as usize);
            let ry = find_root(&mut uf2, req[1] as usize);
            if rx < ry {
                uf2[ry] = rx;
            } else {
                uf2[rx] = ry;
            }
            
            let mut conflict = false;
            for rst in &restrictions {
                let rx = find_root(&mut uf2, rst[0] as usize);
                let ry = find_root(&mut uf2, rst[1] as usize);
                if rx == ry {
                    res.push(false);
                    conflict = true;
                    break;
                }
            }
            if !conflict {
                res.push(true);
                uf = uf2;
            }
        }
        return res;
    }
}
