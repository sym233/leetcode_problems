class Solution {
public:
    string longestPalindrome(string s) {
        int paliBegin = 0;
        int paliLen = 1;
        
        int len = s.size();
        
        // center index
        for (int i = 0; i < len; i++) {
            // odd length
            // length
            for (int l = 1;; l++) {
                int p1 = i - l;
                int p2 = i + l;
                if (0 <= p1 && p2 < len && s[p1] == s[p2]) {
                    if (p2 - p1 + 1 > paliLen) {
                        paliBegin = p1;
                        paliLen = p2 - p1 + 1;
                    }
                } else {
                    break;
                }
            }
            
            // even length
            for (int l = 1;; l++) {
                int p1 = i - l + 1;
                int p2 = i + l;
                if (0 <= p1 && p2 < len && s[p1] == s[p2]) {
                    if (p2 - p1 + 1 > paliLen) {
                        paliBegin = p1;
                        paliLen = p2 - p1 + 1;
                    }
                } else {
                    break;
                }
            }
        }
        return s.substr(paliBegin, paliLen);
    }
};
