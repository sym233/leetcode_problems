impl Solution {
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let mut maxl = 0;
        let mut left = 0;
        let mut right = 0;
        let mut negs = 0;
        while left < l {
            if right < l && nums[right] != 0 {
                if nums[right] < 0 {
                    negs += 1;
                }
                right += 1;
                if negs % 2 == 0 {
                    maxl = maxl.max(right - left);
                }
            } else {
                while left < right {
                    if nums[left] < 0 {
                        negs -= 1;
                    }
                    left += 1;
                    if negs % 2 == 0 {
                        maxl = maxl.max(right - left);
                    }
                }
                left += 1;
                right += 1;
            }
        }
        return maxl as i32;
    }
}
