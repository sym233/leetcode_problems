impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut max = 0;
        
        for &n in nums.iter() {
            if n == 0i32 {
                max = if count > max {
                    count
                } else {
                    max
                };
                count = 0;
            } else {
                count += 1;
            }
        }
        max = if count > max {
            count
        } else {
            max
        };
        return max;
    }
}
