impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let l = *nums.iter().max().unwrap() as usize;
        let mut scores: Vec<i32> = vec![0; l + 1];
        for num in nums {
            scores[num as usize] += num;
        }
        let mut dp: Vec<i32> = vec![0];
        for i in 1..=l {
            if i >= 2 {
                dp.push((dp[i - 2] + scores[i]).max(dp[i - 1]));
            } else {
                dp.push(scores[i].max(dp[i - 1]));
            }
        }
        return dp[l].max(dp[l - 1]);
    }
}
