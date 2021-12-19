impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;
        let mut v: Vec<i32> = vec![1, 1];
        for i in 2..=n {
            v.push(v[i - 1] + v[i - 2]);
        }
        return v[n];
    }
}
