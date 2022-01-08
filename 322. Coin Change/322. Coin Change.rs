impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        const MAX: i32 = i32::max_value();
        let mut dp: Vec<i32> = vec![MAX; amount + 1];
        dp[0] = 0;
        for i in 1..=amount {
            for &coin in &coins {
                let coin = coin as usize;
                if coin <= i && dp[i - coin] < MAX {
                    dp[i] = dp[i].min(dp[i - coin] + 1);
                }
            }
        }
        return if dp[amount] < MAX { dp[amount] } else { -1 };
    }
}
