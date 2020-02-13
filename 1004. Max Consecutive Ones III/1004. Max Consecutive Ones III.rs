impl Solution {
    pub fn longest_ones(a: Vec<i32>, k: i32) -> i32 {
        let l = a.len();
        let mut k = k;
        let mut max = 0;
        
        let mut p = 0usize;
        let mut q = 0usize;
        
        while k > 0 && q < l {
            if a[q] == 0 {
                k -= 1;
            }
            q += 1;
        }
        max = (q - p) as i32;
        
        fn nextStart(p: usize, v: &Vec<i32>) -> usize {
            let l = v.len();
            let mut p = p;
            while p < l && v[p] != 0 {
                p += 1;
            }
            // skip 0 point
            p += 1;
            
            return p;
        }
        
        fn nextEnd(q: usize, v: &Vec<i32>) -> usize {
            let l = v.len();
            let mut q = q;
            while q < l && v[q] != 0 {
                q += 1;
            }
            // skip 0 point
            q += 1;
            while q < l && v[q] != 0 {
                q += 1;
            }
            return q;
        }
		
        while q < l {
            p = nextStart(p, &a);
            q = nextEnd(q, &a);
            let len = (q - p) as i32;
            max = if len > max {
                len
            } else {
                max
            };
        }
        
        return max;
    }
}
