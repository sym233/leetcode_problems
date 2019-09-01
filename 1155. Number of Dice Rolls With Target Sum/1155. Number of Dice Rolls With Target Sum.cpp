class Solution {
public:
    int numRollsToTarget(int d, int f, int target) {
        const int modulo = 1e9 + 7;
        vector<unordered_map<int, int>> v(d);
        for (int i = 0; i < d; i++) {
            if (i == 0) {
                for (int j = 1; j <= f; j++) {
                    v[i][j] = 1;
                }
            } else {
                for (pair<const int, int>& p : v[i - 1]) {
                    for (int j = 1; j <= f; j++) {
                        int& n = v[i][p.first + j];
                        n += p.second;
                        if (n > modulo) {
                            n -= modulo;
                        }
                    }
                }
            }
        }
        return v.back()[target];
    }
};
