impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let n = n as usize;
        let mut v: Vec<i32> = vec![0, 1, 1];
        for i in 3..=n {
            v.push(v[i - 1] + v[i - 2] + v[i - 3]);
        }
        return v[n];
    }
}
