impl Solution {
    pub fn count_primes(n: i32) -> i32 {        
        let n = n as usize;        
        let mut v: Vec<bool> = vec![true; n + 1];        
        let mut count = 0;
        
        for i in 2..n {
            if v[i] {
                count += 1;
                for j in i.. {
                    let k = i * j;
                    if k > n {
                        break;
                    }
                    v[k] = false;
                }
            }
        }
        return count;
    }
}
