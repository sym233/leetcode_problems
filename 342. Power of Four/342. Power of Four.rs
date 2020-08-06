impl Solution {
    pub fn is_power_of_four(num: i32) -> bool {
        let s = (num as f64).sqrt() as i32;
        return (s != 0) & (s * s == num) && (s & (s - 1) == 0);
    }
}
