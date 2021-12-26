impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        const MIN: i32 = -5_000_000;
        let mut res = MIN;
        let l = values.len();
        let mut maxiv = MIN;
        for j in 1..l {
            let jv = values[j] - j as i32;
            let i = j - 1;
            let iv = values[i] + i as i32;
            maxiv = maxiv.max(iv);
            res = res.max(maxiv + jv);
        }
        return res;
    }
}
