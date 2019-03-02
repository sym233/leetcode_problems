class Solution {
private:
    bool isOpen(char c) {
        return c == '(' || c == '{' || c == '[';
    }
    bool match(char c1, char c2) {
        return c1 == '(' && c2 == ')'
            || c1 == '{' && c2 == '}'
            || c1 == '[' && c2 == ']';
    }
public:
    bool isValid(string s) {
        stack<char> st;
        
        for (char c : s) {
            if (isOpen(c)) {
                st.push(c);
            } else if (!st.empty() && match(st.top(), c)) {
                st.pop();
            } else {
                return false;
            }
        }
        
        return st.empty();
    }
};
