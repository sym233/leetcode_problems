class Solution {
public:
    int integerBreak(int n) {
        int max = 0;
        for (int i = 2; i <= n; i++) {
            // break into i numbers
            
            int product = 1;
            int sm = round(static_cast<double>(n) / i);
            if ((i - 1) * sm >= n) {
                continue;
            }
            product *= static_cast<int>(pow(sm, i - 1));
            product *= n - (i - 1) * sm;
            if (product > max) {
                max = product;
            }
        }
        return max;
    }
};
