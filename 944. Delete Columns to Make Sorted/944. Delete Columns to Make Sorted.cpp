static int desyncio = []() {
  std::ios::sync_with_stdio(false);
  cin.tie(nullptr);
  return 0;
}();

class Solution {
public:
    int minDeletionSize(vector<string>& A) {
        int c = A.size();
        int r = A[0].size();
        
        if (r == 0) {
            return 0;
        }
        
        vector<bool> del(r, false);
        for (int j = 1; j < c; j++) {
            for (int i = 0; i < r; i++) {
                if (A[j - 1][i] > A[j][i]) {
                    del[i] = true;
                }
            }
        }
        
        int sum = 0;
        for (bool b : del) {
            sum += b;
        }
        return sum;
    }
};
