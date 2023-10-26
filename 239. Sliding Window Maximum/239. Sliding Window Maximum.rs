use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let l = nums.len();
        let k = k as usize;
        let mut q = VecDeque::<usize>::new();
        let mut res = Vec::with_capacity(l - k + 1);

        for i in 0..l {
            while let Some(&j) = q.back() {
                if nums[j] < nums[i] {
                    q.pop_back();
                } else {
                    break;
                }
            }
            q.push_back(i);

            while let Some(&j) = q.front() {
                if k <= i - j {
                    q.pop_front();
                } else {
                    break;
                }
            }

            if i >= k - 1 {
                res.push(nums[*q.front().unwrap()]);
            }
        }
        res
    }
}
