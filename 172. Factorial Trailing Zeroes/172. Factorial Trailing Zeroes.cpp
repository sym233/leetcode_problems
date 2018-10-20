class Solution {
public:
    int trailingZeroes(int n) {
        int fives = 0;
        for (int i = 1;; i++) {
            int f = n / pow(5, i);
            if (f > 0) {
                fives += f;
            } else {
                break;
            }
        }
        return fives;
    }
};
