impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();
        return nums.into_iter().enumerate().filter(|(i, v)| *v == target).map(|(i, v)| i as i32).collect();
    }
}
