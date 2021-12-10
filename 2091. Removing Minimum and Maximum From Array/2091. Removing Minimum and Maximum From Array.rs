impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        if l <= 2 {
            return l as i32;
        }
        let mut max = -1e5 as i32;
        let mut maxi = 0;
        let mut min = 1e5 as i32;
        let mut mini = 0;
        for i in 0..l {
            if nums[i] > max {
                max = nums[i];
                maxi = i;
            }
            if nums[i] < min {
                min = nums[i];
                mini = i;
            }
        }
        
        let left = maxi.min(mini);
        let right = maxi.max(mini);
        
        let ress = [right + 1, l - left, left + 1 + l - right];        
        return *ress.iter().min().unwrap() as i32;
    }
}
