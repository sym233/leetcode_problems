class Solution {
private:
    inline bool check(vector<char>& st, string& c) {
        int sl = st.size();
        int cl = c.size();
        if (sl >= cl) {
            for (int i = 0; i < cl; i++) {
                if (st[sl - i - 1] != c[cl - i - 1]) {
                    return false;
                }
            }
            return true;
        }
        return false;
    }
public:
    bool isValid(string S) {
        string v = "abc";
        vector<char> st;
        
        for (char c : S) {
            st.push_back(c);
            if (check(st, v)) {
                for (int i = 0; i < v.size(); i++) {
                    st.pop_back();
                }
            }
        }
        return st.empty();
    }
};
