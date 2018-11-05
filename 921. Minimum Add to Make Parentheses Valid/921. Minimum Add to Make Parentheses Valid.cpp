class Solution {
public:
    int minAddToMakeValid(string S) {
        vector<char> stack;
        int res = 0;
        for (char c : S) {
            if (c == '(') {
                stack.push_back(c);
            } else {
                if (stack.empty()) {
                    res++;
                } else {
                    stack.pop_back();
                }
            } 
        }
        
        res += stack.size();
        return res;
    }
};
