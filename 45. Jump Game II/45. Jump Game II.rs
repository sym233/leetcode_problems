impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let mut current_step = 0;
        let mut farest_reach = 0;
        let mut next_step_farest_reach = 0;
        for i in 0..l {
            if farest_reach < i as i32 {
                current_step += 1;
                farest_reach = next_step_farest_reach;
            }
            next_step_farest_reach = next_step_farest_reach.max(i as i32 + nums[i]);
        }
        return current_step;
    }
}
