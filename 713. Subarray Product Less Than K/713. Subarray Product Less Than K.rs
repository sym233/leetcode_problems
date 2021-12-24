impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        let mut prod = 1;
        let mut left = 0;
        let mut right = 0;
        let mut res = 0;
        while left < len {
            if prod < k && right < len {
                prod *= nums[right];
                right += 1;
            } else {
                if left < right {
                    if prod < k && right == len {
                        let l = right - left;
                        res += l * (l + 1) / 2;
                        break;
                    } else {
                        res += right - left - 1;
                        prod /= nums[left];
                        left += 1;
                    }
                } else {
                    left += 1;
                    right += 1;
                }
            }
        }
        return res as i32;
    }
}
