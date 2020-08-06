impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut n = n;
        let mut b = n & 1;
        n >>= 1;
        while n != 0 {
            if b == n & 1 {
                return false;
            }
            b = n & 1;
            n >>= 1;
        }
        return true;
    }
}
