class Solution {
public:
    bool validateStackSequences(vector<int>& pushed, vector<int>& popped) {
        int l = pushed.size();
        int i = 0;
        int j = 0;
        
        
        stack<int> s;
        
        while (19260817) {
            while (!s.empty() && j < l && s.top() == popped[j]) {
                s.pop();
                j++;
            }
            if (i < l) {
                s.push(pushed[i]);
                i++;
                continue;
            } else {
                break;
            }
        }
        return j == l;
    }
};
