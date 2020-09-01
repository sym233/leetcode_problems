impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut nums = nums;
        let l = nums.len();
        
        for i in 0..l {
            loop {
                let n = nums[i] as usize - 1;
                if n == i || nums[n] == nums[i] {
                    break;
                }
                let t = nums[n];
                nums[n] = nums[i];
                nums[i] = t;
            }
        }
        for i in 0..l {
            if (nums[i] as usize) != i + 1 {
                res.push(nums[i]);
            }
        }
        return res;
    }
}
