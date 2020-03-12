impl Solution {   
    pub fn rob(nums: Vec<i32>) -> i32 {
        fn max(nums: &[i32]) -> i32 {
            let mut m: i32 = 0;
            for &num in nums {
                if num > m {
                    m = num;
                }
            }
            return m;
        }

        let l = nums.len();

        match l {
            0 => return 0,
            1 => return nums[0],
            2 => return max(&nums),
            _ => (),
        }
        
        // total money with 0th house robbed
        let mut dp: Vec<i32> = vec![0; l];
        dp[0] = nums[0];
        
        // total money without 0th house robbed
        let mut dpwo0: Vec<i32> = vec![0; l];
        dpwo0[1] = nums[1];
        
        
        for i in 2..l {
            dp[i] = max(&dp[0..i - 1]) + nums[i];
            dpwo0[i] = max(&dpwo0[1..i - 1]) + nums[i];
        }
        
        return max(&[dp[l - 3], dp[l - 2], dpwo0[l - 2], dpwo0[l - 1]]);
    }
}
