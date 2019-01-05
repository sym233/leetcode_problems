class Solution {
private:
    struct Item {
        bool matched;
        int matchedLength;
        char c;
        Item(int matchedLength) : matched(true), matchedLength(matchedLength) {}
        Item(char c) : matched(false), c(c) {}
        bool operator==(char rhs) {
            return !matched && c == rhs;
        }
    };
    vector<Item> st;
    void processBackParenthese() {
        int l = st.size();
        if (l - 1 >= 0 && st[l - 1] == ')') {
            if (l - 2 >= 0 && st[l - 2] == '(') {
                st.pop_back();
                st.pop_back();
                st.push_back(Item(2));
                processMatchedLength();
            } else if (l - 3 >= 0 && st[l - 2].matched && st[l - 3] == '(') {
                int matchedLength = st[l - 2].matchedLength + 2;
                st.pop_back();
                st.pop_back();
                st.pop_back();
                st.push_back(Item(matchedLength));
                processMatchedLength();
            }
        }
    }
    void processMatchedLength() {
        int l = st.size();
        if (l - 1 >= 0 && st[l - 1].matched) {
            if (l - 2 >= 0 && st[l - 2].matched) {
                int matchedLength = st[l - 2].matchedLength + st[l - 1].matchedLength;
                st.pop_back();
                st.pop_back();
                st.push_back(Item(matchedLength));
            }
        }
    }
public:
    int longestValidParentheses(string s) {
        for (char c : s) {
            st.push_back(Item(c));
            if (c == ')') {
                processBackParenthese();
            }
        }
        int maxLength = 0;
        for (Item& i : st) {
            if (i.matched && i.matchedLength > maxLength) {
                maxLength = i.matchedLength;
            }
        }
        return maxLength;
    }
};
