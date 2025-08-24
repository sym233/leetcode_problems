impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let nums_len = nums.len();
        let mut prev_len = 0;
        let mut curr_len = 0;
        let mut max_len = 0;
        for num in nums {
            if num == 1 {
                curr_len += 1;
            } else {
                max_len = max_len.max(prev_len + curr_len);
                prev_len = curr_len;
                curr_len = 0;
            }
        }
        max_len = max_len.max(prev_len + curr_len);
        if max_len == nums_len {
            // in case all num are 1
            max_len as i32 - 1
        } else {
            max_len as i32
        }
    }
}
