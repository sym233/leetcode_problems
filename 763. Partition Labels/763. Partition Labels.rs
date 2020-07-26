impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let s: Vec<char> = s.chars().collect();
        
        let l = s.len();
        
        let mut begin = 0usize;
        let mut end = 1usize;
        
        let mut rpos = [0usize; 26];
        let lton = |c: char| (c as u8 - 'a' as u8) as usize;
        for i in 0..l {
            rpos[lton(s[i])] = i;
        }
        
        for i in 0..l {
            let j = rpos[lton(s[i])];
            if j + 1 > end {
                end = j + 1;
            }
            if i == end - 1 {
                res.push((end - begin) as i32);
                begin = end;
                end = end + 1;
            }
        }
        
        return res;
    }
}
