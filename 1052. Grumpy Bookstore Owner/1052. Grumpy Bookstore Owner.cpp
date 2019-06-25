class Solution {
public:
    int maxSatisfied(vector<int>& customers, vector<int>& grumpy, int X) {
        int max = 0;
        int tot = 0;
        int secretStart = 0;
        int l = customers.size();
        
        for (int i = 0; i < l; i++) {
            if (secretStart <= i && i <secretStart + X) {
                tot += customers[i];
            } else {
                tot += (!grumpy[i]) * customers[i];
            }
        }
        max = tot;
        
        for (secretStart = 1; secretStart + X <= l; secretStart++) {
            int left = secretStart - 1;
            int right = secretStart + X - 1;
            if (grumpy[left]) {
                tot -= customers[left];
            }
            if (grumpy[right]) {
                tot += customers[right];
            }
            if (tot > max) {
                max = tot;
            }
        }
        return max;
    }
};
