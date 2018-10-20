class Solution {
public:
    vector<vector<int>> res;
    vector<vector<int>> combinationSum3(int k, int n) {
        vector<int> t;
        tryMatch(k, n, 1, t);
        return res;
    }
    
    void tryMatch(int k, int n, int low, vector<int>& vec) {
        int lowerBound = 0;
        for (int i = 0; i < k && low + i <= 9; i++) {
            lowerBound += low + i;
        }
        if (n < lowerBound) {
            return;
        }
        
        int upperBound = 0;
        for (int i = 0; i < k && 9 - i >= low; i++) {
            upperBound += 9 - i;
        }
        if (n > upperBound) {
            return;
        }
        
        for (int i = low; i <= 9; i++) {
            int newK = k - 1;
            int newN = n - i;
            if (newK == 0) {
                if (newN == 0) {
                    vector<int> newVec(vec);
                    newVec.push_back(i);
                    res.push_back(newVec);
                }
            } else {
                if (newN > 0) {
                    vector<int> newVec(vec);
                    newVec.push_back(i);
                    tryMatch(newK, newN, i + 1, newVec);
                }
            }
        }
    }
};
