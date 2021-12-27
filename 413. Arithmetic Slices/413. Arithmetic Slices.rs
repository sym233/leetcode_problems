impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        if l < 3 {
            return 0;
        }
        let mut res = 0;
        let mut i = 0;
        while i < l {
            let mut j = i + 1;
            while j < l - 1 && nums[j] - nums[j - 1] == nums[j + 1] - nums[j] {
                j += 1;
            }
            res += Self::f(j - i + 1);
            i = j;
        }
        return res;
    }
    fn f(len: usize) -> i32 {
        if len < 3 {
            return 0;
        }
        return ((len - 2) * (len - 1) / 2) as i32;
    }
}
