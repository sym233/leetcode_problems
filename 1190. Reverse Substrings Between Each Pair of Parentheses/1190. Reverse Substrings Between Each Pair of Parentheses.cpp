class Solution {
public:
    string reverseParentheses(string s) {
        int l = s.size();
        stack<int> st;
        for (int i = 0; i < l; i++) {
            if (s[i] == '(') {
                st.push(i);
            } else if (s[i] == ')') {
                reverse(s.begin() + st.top() + 1, s.begin() + i);
                st.pop();
            }
        }
        string res;
        for (char& c : s) {
            if (c == '(' || c == ')') {
                continue;
            }
            res.push_back(c);
        }
        return res;
    }
};