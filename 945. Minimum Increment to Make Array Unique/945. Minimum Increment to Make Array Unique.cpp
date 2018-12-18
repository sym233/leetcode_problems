class Solution {
public:
    int minIncrementForUnique(vector<int>& A) {
        map<int, int> m;
        
        for (int a : A) {
            m[a]++;
        }
        
        int minMove = 0;
        
        for (map<int, int>::iterator it = m.begin(); it != m.end(); it++) {
            if (it->second > 1) {
                minMove += it->second - 1;
                m[it->first + 1] += it->second - 1;
                it->second = 1;
            }
        }
        
        return minMove;
    }
};
