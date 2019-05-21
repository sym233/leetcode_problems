class Solution {
public:
    int minKBitFlips(vector<int>& A, int K) {
        int l = A.size();
        int ans = 0;
        queue<int> flips;
        
        for (int i = 0; i < l - K + 1; i++) {
            while (!flips.empty() && flips.front() + K - 1 < i) {
                flips.pop();
            }
            A[i] = (A[i] + flips.size()) % 2;
            
            if (A[i] == 0) {
                A[i] = !A[i];
                ans++;
                flips.push(i);
            }
        }
        for (int i = l - K + 1; i < l; i++) {
            while (!flips.empty() && flips.front() + K - 1 < i) {
                flips.pop();
            }
            A[i] = (A[i] + flips.size()) % 2;
            if (A[i] == 0) {
                return -1;
            }
        }
        return ans;
    }
};
