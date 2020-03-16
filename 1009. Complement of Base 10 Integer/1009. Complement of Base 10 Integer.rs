impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        let mut m = n;
        let mut mask = 1;
        
        while m > 1 {
            m >>= 1;
            mask <<= 1;
            mask |= 1;
        }
        
        return !n & mask;
    }
}
