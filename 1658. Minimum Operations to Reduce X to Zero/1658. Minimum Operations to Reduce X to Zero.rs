impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let l = nums.len();

        let mut prefix = vec![0; l + 1];

        for i in 0..l {
            prefix[i + 1] = prefix[i] + nums[i];
        }

        let val = |left, right| prefix[l] - prefix[0] - (prefix[right] - prefix[left]);
        let mut res = l + 1;
        for i in 0..l {
            let mut left = i;
            let mut right = l + 1;
            while right - left > 1 {
                let mid = (left + right) / 2;
                let s = val(i, mid) - x;
                if s >= 0 {
                    left = mid;
                } else {
                    right = mid;
                }
            }
            if val(i, left) == x {
                res = res.min(l - (left - i));
            }
        }
        if res == l + 1 {
            return -1;
        } else {
            return res as i32;
        }
    }
}
