impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut negs = vec![];
        let mut res = vec![];
        for num in nums {
            let sqr = num * num;
            if num < 0 {
                negs.push(sqr);
            } else {
                while negs.last().map_or(false, |neg| *neg < sqr) {
                    res.push(negs.pop().unwrap());
                }
                res.push(sqr);
            }
        }
        res.extend(negs.into_iter().rev());
        res
    }
}
