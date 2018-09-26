class Solution {
public:
    vector<int> grayCode(int n) {
        if (n == 0) {
            return vector<int>(1, 0);
        } else {
            int l = 1 << (n - 1);
            vector<int> res(grayCode(n - 1));
            
            for (int i = l - 1; i >= 0; i--) {
                res.push_back(l + res[i]);
            }
            return res;
        }
    }
};
