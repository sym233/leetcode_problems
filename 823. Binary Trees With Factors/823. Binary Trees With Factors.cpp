class Solution {
public:
    const int modulo = 1e9 + 7;
    int numFactoredBinaryTrees(vector<int>& A) {
        sort(A.begin(), A.end());
        int l = A.size();
        
        vector<long> numberAsRoot(l, 1);
        
        for (int i = 0; i < l; i++) {
            for (int j = 0; j < i; j++) {
                if (A[i] % A[j] == 0) {
                    vector<int>::iterator p = lower_bound(A.begin(), A.end(), A[i] / A[j]);
                    if (p != A.end() && *p * A[j] == A[i]) {
                        numberAsRoot[i] += numberAsRoot[j] * numberAsRoot[p - A.begin()] % modulo;
                    }
                }
            }
        }
        
        long res = 0;
        
        for (long i : numberAsRoot) {
            res += i;
            res %= modulo;
        }
        
        return static_cast<int>(res);
    }
};
