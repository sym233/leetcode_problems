class Solution {
private:
    const int maxLen = 16;
    bool isSuccessor(string& s1, string& s2) {
        int l = s1.size();
        if (l != s2.size() - 1) {
            return false;
        }
        int diff = 0;
        int i = 0;
        int j = 0;
        
        while (i < l) {
            if (s1[i] == s2[j]) {
                i++;
                j++;
            } else {
                j++;
                diff++;
                if (diff > 1) {
                    return false;
                }
            }
        }
        return true;
    }
public:
    int longestStrChain(vector<string>& words) {
        const int l = words.size();
        
        vector<vector<int>> strs(maxLen + 1, vector<int>());
        // strs[word length] = [word indexs]
        
        for (int i = 0; i < l; i++) {
            strs[words[i].size()].push_back(i);
        }
        
        vector<int> hasPredecessor(l, false);
        vector<vector<int>> successors(l, vector<int>());
        // k in successor[j] means words[j] -> words[k];
        
        for (int i = 1; i < maxLen; i++) {
            for (int& j : strs[i]) {
                for (int& k : strs[i + 1]) {
                    if (isSuccessor(words[j], words[k])) {
                        successors[j].push_back(k);
                        hasPredecessor[k] = true;
                    }
                }
            }
        }
        int res = 0;
        function<void(int, int)> dfs = [&] (int index, int len) {
            if (len > res) {
                res = len;
            }
            // for (int i = 0; i < len; i++) {
            //     cout << '-';
            // }
            // cout << words[index] << endl;
            for (int& successor : successors[index]) {
                dfs(successor, len + 1);
            }
            return;
        };
        // for (int i = 0; i < l; i++) {
        //     for (int j : successors[i]) {
        //         cout << words[i] << " -> " << words[j] << endl;
        //     }
        // }
        for (int i = 0; i < l; i++) {
            if (!hasPredecessor[i]) {
                dfs(i, 1);
            }
        }
        return res;
    }
};
