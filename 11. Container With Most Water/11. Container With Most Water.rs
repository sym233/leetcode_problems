impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let len = height.len();
        
        let mut l = 0;
        let mut r = len - 1;
        
        while l < r {
            res = res.max(Self::calc(&height, l, r));
            if height[l] > height[r] {
                r -= 1;
            } else {
                l += 1;
            }
        }
        return res;        
    }
    fn calc(height: &Vec<i32>, l: usize, r: usize) -> i32 {
        let h = height[l].min(height[r]);
        let w = (r - l) as i32;
        return h * w;
    }
}
