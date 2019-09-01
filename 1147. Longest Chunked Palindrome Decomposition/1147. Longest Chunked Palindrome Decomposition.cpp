class Solution {
private:
    bool eq(string& str, int b1, int e1, int b2, int e2) {
        int l = e1 - b1;
        if (l != e2 - b2) {
            return false;
        }
        for (int i = 0; i < l; i++) {
            if (str[b1 + i] != str[b2 + i]) {
                return false;
            }
        }
        return true;
    }
    int f(string& str, int begin, int end) {
        if (begin == end) {
            return 0;
        }
        int l = end - begin;
        for (int len = 1; len * 2 <= l; len++) {
            if (eq(str, begin, begin + len, end - len, end)) {
                return 2 + f(str, begin + len, end - len);
            }
        }
        return 1;
    }
public:
    int longestDecomposition(string text) {
        int l = text.size();
        return f(text, 0, l);
    }
};
