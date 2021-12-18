impl Solution {
    pub fn fib(n: i32) -> i32 {
        let mut v: Vec<i32> = vec![0, 1];
        for i in 2..=n as usize {
            v.push(v[i - 1] + v[i - 2]);
        }
        return v[n as usize];
    }
}
