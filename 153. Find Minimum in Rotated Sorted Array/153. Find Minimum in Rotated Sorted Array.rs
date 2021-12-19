impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let mut s = 0;
        let mut e = l;
        let last = nums[l - 1];
        return loop {
            let i = (s + e) / 2;
            if e - s <= 1 {
                if e < l {
                    break nums[s].min(nums[e]);
                }
                break nums[s];
            }
            if nums[i] > last {
                s = i;
            } else {
                e = i;
            }
        };
    }
}
