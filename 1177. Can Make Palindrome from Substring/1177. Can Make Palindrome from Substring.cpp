class Solution {
public:
    vector<bool> canMakePaliQueries(string s, vector<vector<int>>& queries) {
        vector<vector<int>> letters(1, vector<int>(26, 0));
        int l = s.size();
        for (int i = 0; i < l; i++) {
            letters.push_back(letters[i]);
            letters[i + 1][s[i] - 'a']++;
        }
        vector<bool> res;
        for (vector<int>& query : queries) {
            
            int dif = 0;
            for (int i = 0; i < 26; i++) {
                int li = letters[query[0]][i];
                int ri = letters[query[1] + 1][i];
                int count = ri - li;
                
                if (count % 2) {
                    dif++;
                }
            }
            res.push_back(dif - 2 * query[2] <= 1);
        }
        return res;
    }
};