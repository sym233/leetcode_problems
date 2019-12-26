class Solution {
public:
    int equalSubstring(string s, string t, int maxCost) {
        int l = s.size();
        vector<int> accu(l + 1, 0);
        for (int i = 0; i < l; i++) {
            int diff = abs(s[i] - t[i]);
            accu[i + 1] = accu[i] + diff;
        }
        int max = 0;
        for (int i = 0; i < l; i++) {
            auto it = upper_bound(accu.begin() + i, accu.end(), accu[i] + maxCost);
            int len = (it - accu.begin()) - i - 1;
            if (len > max) {
                max = len;
            }
        }
        return max;            
    }
};
