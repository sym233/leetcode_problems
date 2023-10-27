use std::collections::VecDeque;

impl Solution {
    pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let l = nums.len();
        let mut res = nums[0];
        let mut sums = Vec::new();
        let mut q = VecDeque::<usize>::new();
        // monotonic incr queue

        for i in 0..l {
            if 0 == i {
                sums.push(nums[0]);
                q.push_back(0);
            } else {
                let sum = sums[*q.front().unwrap()].max(0) + nums[i];
                sums.push(sum);

                while let Some(&j) = q.back() {
                    if sums[j] < sum {
                        q.pop_back();
                    } else {
                        break;
                    }
                }
                q.push_back(i);

                while let Some(&j) = q.front() {
                    if j + k <= i {
                        q.pop_front();
                    } else {
                        break;
                    }
                }
            }
            res = res.max(sums[*q.front().unwrap()]);
        }
        res
    }
}
