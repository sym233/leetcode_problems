class Solution {
public:
    int minFallingPathSum(vector<vector<int>>& A) {
        int l = A.size();
        
        for (int row = 1; row < l; row++) {
            for (int col = 0; col < l; col++) {
                int minUpper = INT_MAX;
                for (int i = -1; i <= 1; i++) {
                    int upperCol = col + i;
                    if (0 <= upperCol && upperCol < l) {
                        if (A[row - 1][upperCol] < minUpper) {
                            minUpper = A[row - 1][upperCol];
                        }
                    }
                }
                A[row][col] += minUpper;
            }
        }
        
        int res = INT_MAX;
        for (int col = 0; col < l; col++) {
            if (A[l - 1][col] < res) {
                res = A[l - 1][col];
            }
        }
        return res;
    }
};
