impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ps = vec![0];
        for n in &nums {
            ps.push(*ps.last().unwrap() + *n);
        }
        let l = nums.len();        
        let mut j = 0;
        const MIN: i32 = -200_000;
        let mut res = MIN;
        let mut min = 0;
        for i in 1..=l {
            if ps[i - 1] < ps[i] && (i == l || ps[i] >= ps[i + 1]) {
                while j < i {
                    min = min.min(ps[j]);
                    res = res.max(ps[i] - min);
                    j += 1;
                }
            }
        }
        if res == MIN {
            return nums.into_iter().max().unwrap();
        }
        return res;
    }
}
