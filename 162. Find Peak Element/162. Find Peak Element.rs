impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let mut s = 0;
        let mut e = l;
        return loop {
            let i = (s + e) / 2;
            let o = ord(&nums, i);
            if o < 0 {
                e = i;
            } else if o == 0 {
                break i;
            } else {
                s = i;
            }
        } as i32;
    }
}

fn ord(nums: &[i32], i: usize) -> i32 {
    // -1 left, 0 found, 1 right
    let l = nums.len();
    if i == 0 || nums[i - 1] < nums[i] {
        if i == l - 1 || nums[i] > nums[i + 1] {
            return 0;
        } else {
            return 1;
        }
    } else {
        if i == l - 1 || nums[i] > nums[i + 1] {
            return -1;
        } else {
            return -1;
        }
    }
}
