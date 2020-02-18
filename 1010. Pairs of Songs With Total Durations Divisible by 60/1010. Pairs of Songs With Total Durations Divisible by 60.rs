impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        const T: usize = 60usize;
        let mut arr = [0; T];
        let mut count = 0;
        for &n in time.iter() {
            let n = n as usize % T;
            count += arr[(T - n) % T];
            arr[n] += 1;
        }
        return count
    }
}
