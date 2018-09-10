class Solution {
public:
    int consecutiveNumbersSum(int N) {
        int count = 0;
        
        int i = (-1 + sqrt(1 + 8 * static_cast<long long>(N))) / 2;
        for (; i > 0; i--) {
            
            int t = 2 * N - i * (i - 1);
            if ((t / (2 * i)) * (2 * i) == t) {
                count++;
            }
        }
        return count;
    }
};
