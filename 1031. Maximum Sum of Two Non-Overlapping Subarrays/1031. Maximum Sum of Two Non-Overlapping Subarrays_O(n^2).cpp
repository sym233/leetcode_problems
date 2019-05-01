class Solution {
public:
    int maxSumTwoNoOverlap(vector<int>& A, int L, int M) {
        int al = A.size();
        vector<int> prefixSum = { 0 };
        // sum {A[i] : A[j - 1]} = prefixSum[j] - prefixSum[i]
        for (int a : A) {
            prefixSum.push_back(prefixSum.back() + a);
        }
        
        auto overlap = [](int s1, int e1, int s2, int e2){
            return s1 <= s2 && s2 <= e1 || s2 <= s1 && s1 <= e2;
        };
        int max = 0;
        for (int i = 0; i + L - 1 < al; i++) {
            for (int j = 0; j + M - 1 < al; j++) {
                if (!overlap(i, i + L - 1, j, j + M - 1)) {
                    int t = prefixSum[i + L] - prefixSum[i] + prefixSum[j + M] - prefixSum[j];
                    if (t > max) {
                        max = t;
                    }
                }
            }
        }
        
        return max;
    }
};
