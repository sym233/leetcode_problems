class Solution {
public:
    int maxProfit(vector<int>& prices) {
        set<int> s;
        int l = prices.size();
        int maxProfit = 0;
        for (int i = l - 2; i >= 0; i--) {
            s.insert(prices[i + 1]);
            int profit = *(s.rbegin()) - prices[i];
            if (profit > maxProfit) {
                maxProfit = profit;
            }
        }
        return maxProfit;
    }
};
