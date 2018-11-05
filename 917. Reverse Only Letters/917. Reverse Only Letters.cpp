class Solution {
public:
    string reverseOnlyLetters(string S) {
        int l = S.size();
        int i = 0;
        int j = l - 1;
        
        while (i < j) {
            if (isLetter(S[i])) {
                if (isLetter(S[j])) {
                    swap(S[i], S[j]);
                    i++;
                    j--;
                } else {
                    j--;
                }
            } else {
                i++;
            }
        }
        return S;
    }
    bool isLetter(char c) {
        return 'A' <= c && c <= 'Z' || 'a' <= c && c <= 'z';
    }
};
