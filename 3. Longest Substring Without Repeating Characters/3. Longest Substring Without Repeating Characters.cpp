class Solution {
private:
    int maxL = 0;
    inline void updateL(int newLen) {
        if (newLen > maxL) {
            maxL = newLen;
        }
        return;
    }
public:
    int lengthOfLongestSubstring(string s) {
        vector<int> charLastOccurs(128, -1);
        int len = s.size();
        int nearestDuplicatedChar = -1;
        for (int i = 0; i < len; i++) {
            if (nearestDuplicatedChar < charLastOccurs[s[i]]) {
                nearestDuplicatedChar = charLastOccurs[s[i]];
            } else {
                updateL(i - nearestDuplicatedChar);
            }
            charLastOccurs[s[i]] = i;
        }
        return maxL;
    }
};
