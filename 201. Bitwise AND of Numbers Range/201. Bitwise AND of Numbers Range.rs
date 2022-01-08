impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut res = left & right;
        for i in 0..(32 - right.leading_zeros() + 1) {
            let o = (1 << i);
            if right - left + 1 > o && (res & o) > 0{
                res -= o;
            }
        }
        return res;
    }
}
