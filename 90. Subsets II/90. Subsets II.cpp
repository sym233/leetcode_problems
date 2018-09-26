class Solution {
public:
    vector<vector<int>> subsetsWithDup(vector<int>& nums) {
        map<int, int> m;
        for (int n : nums) {
            map<int, int>::iterator it = m.find(n);
            if (it != m.end()) {
                it->second++;
            } else {
                m.insert(pair<int, int>(n, 1));
            }
        }
        
        
        vector<vector<int>> res(1, vector<int>(0));
        
        for (map<int, int>::iterator it = m.begin(); it != m.end(); it++) {
            vector<vector<int>> temp;
            for (vector<int>& s : res) {
                for (int i = 0; i <= it->second; i++) {
                    vector<int> t(s);
                    for (int j = 0; j < i; j++) {
                        t.push_back(it->first);
                    }
                    temp.push_back(t);
                }
            }
            res = temp;
        }
        
        return res;
        
    }
};
