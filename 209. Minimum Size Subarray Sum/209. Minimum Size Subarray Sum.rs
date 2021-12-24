impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let mut sum = 0;
        let mut left = 0;
        let mut right = 0;
        let mut res = l + 1;
        while left < l {
            if sum < target {
                if right == l {
                    break;
                } else {
                    sum += nums[right];
                    right += 1;
                }
            } else {
                while target <= sum {
                    res = res.min(right - left);
                    sum -= nums[left];
                    left += 1;
                }
            }
        }
        if res == l + 1 {
            res = 0;
        }
        return res as i32;
    }
}
