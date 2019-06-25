class Solution {
public:
    int largestValsFromLabels(vector<int>& values, vector<int>& labels, int num_wanted, int use_limit) {
        unordered_map<int, vector<int>> m;
        vector<int> S;
        int l = values.size();
        for (int i = 0; i < l; i++) {
            m[labels[i]].push_back(values[i]);
        }
        
        auto descend = [](int& a, int& b)-> bool {
            return a > b;
        };
        
        for (auto it = m.begin(); it != m.end(); it++) {
            vector<int>& nums = it->second;
            if (nums.size() <= use_limit) {
                S.insert(S.end(), nums.begin(), nums.end());
            } else {
                sort(nums.begin(), nums.end(), descend);
                S.insert(S.end(), nums.begin(), nums.begin() + use_limit);
            }
        }
        
        if (S.size() <= num_wanted) {
            return accumulate(S.begin(), S.end(), 0);
        } else {
            sort(S.begin(), S.end(), descend);
            return accumulate(S.begin(), S.begin() + num_wanted, 0);
        }
    }
}
