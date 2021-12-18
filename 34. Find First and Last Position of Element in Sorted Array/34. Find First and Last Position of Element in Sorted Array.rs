impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let l = nums.len();
        let mut s = 0;
        let mut e = l;
        
        let mut i = (s + e) / 2;
        while s + 1 < e {
            if nums[i] < target {
                s = i;
            } else {
                e = i;
            }
            i = (s + e) / 2;
        }
        let lb = if i < l && nums[i] == target {
            i
        } else if l <= i + 1 || nums[i + 1] != target {
            return vec![-1, -1];
        } else {
            i + 1
        };
        
        s = lb;
        e = l;
        i = (s + e) / 2;
        
        while s + 1 < e {
            if nums[i] <= target {
                s = i;
            } else {
                e = i;
            }
            i = (s + e) / 2;
        }
        
        let ub = i;
        
        return vec![lb as i32, ub as i32];
    }
}
