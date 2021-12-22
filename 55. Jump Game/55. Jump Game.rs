impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut step_remain = nums[0];
        for num in nums[1..].iter() {
            if step_remain <= 0 {
                return false;
            }
            step_remain -= 1;
            step_remain = step_remain.max(*num);
        }
        return true;
    }
}
