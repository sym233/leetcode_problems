use std::collections::VecDeque;
impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let mut q = VecDeque::with_capacity(k as usize);
        let mut sum = 0;
        let mut res = 0;
        for n in arr {
            if q.len() == k as usize {
                let f = q.pop_front().unwrap();
                sum -= f;
            }
            q.push_back(n);
            sum += n;
            if q.len() == k as usize {
                if sum >= k * threshold {
                    res += 1;
                }
            }
        }
        return res;
    }
}
