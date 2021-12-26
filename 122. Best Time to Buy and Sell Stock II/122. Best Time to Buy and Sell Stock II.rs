impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut prof = 0;
        let l = prices.len();
        let mut i = 0;
        while i < l {
            if i + 1 < l && prices[i] < prices[i + 1] {
                let buy = prices[i];
                let mut j = i + 1;
                while j < l - 1 && prices[j] <= prices[j + 1] {
                    j += 1;
                }
                let sell = prices[j];
                prof += sell - buy;
                i = j + 1;
            } else {
                i += 1;
            }
        }
        return prof;
    }
}
