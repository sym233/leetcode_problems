impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut num = num;
        let mut v: Vec<i32> = Vec::new();
        while num != 0 {
            v.push(num % 10);
            num /= 10;
        }
        let l = v.len();
        
        
        // index 3  2  1  0
        // val   2  7  3  6
        //       |  |
        //       j  i
        
        for j in (0..l).rev() {
            let mut max_v = 0;
            let mut max_i = 0usize;
            for i in 0..j {
                if v[i] > max_v {
                    max_v = v[i];
                    max_i = i;
                }
            }
            
            if max_v > v[j] {
                let t = v[j];
                v[j] = v[max_i];
                v[max_i] = t;
                break;
            }
        }
        
        let mut res = 0;
        while let Some(n) = v.pop() {
            res *= 10;
            res += n;
        }
        return res;
    }
}
