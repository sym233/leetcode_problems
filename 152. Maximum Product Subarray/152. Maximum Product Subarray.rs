impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let mut maxn = nums[0];
        
        let mut left = 0;
        let mut right = 0;
        const MIN:i32 = -200_000;
        let mut res = MIN;
        let mut p = 1;
        while left < l {
            if right < l {
                maxn = maxn.max(nums[right]);                
            }
            if right < l && nums[right] != 0 {
                p *= nums[right];
                right += 1;
                res = res.max(p);
            } else {
                while left < right {
                    p /= nums[left];
                    left += 1;
                    if left < right {
                        res = res.max(p);
                    }
                }
                left += 1;
                right += 1;
            }
        }
        
        return res.max(maxn);
    }
}
