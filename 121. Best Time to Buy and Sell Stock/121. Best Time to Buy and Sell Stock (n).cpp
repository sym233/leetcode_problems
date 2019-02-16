class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int l = prices.size();
        if (l == 0) {
            return 0;
        }
        int maxProfit = 0;
        int min = prices[0];
        for (int i = 1; i < l; i++) {
            if (prices[i] - min > maxProfit) {
                maxProfit = prices[i] - min;
            } else if (prices[i] < min) {
                min = prices[i];
            }
        }
        return maxProfit;
    }
};
