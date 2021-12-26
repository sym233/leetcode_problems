impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let l = prices.len();
        let mut profs: Vec<i32> = vec![0; l];
        
        for i in 0..l {
            let mut min_buy = 10_000;
            let mut prof = 0;
            for j in (i + 1)..l {
                min_buy = min_buy.min(prices[j - 1]);
                prof = prof.max(prices[j] - min_buy);
                if i >= 3 {
                    profs[j] = profs[j].max(prof + profs[i - 2]);
                } else {
                    profs[j] = profs[j].max(prof);
                }
            }
        }
        return profs[l - 1];
    }
}
