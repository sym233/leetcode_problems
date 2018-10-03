class Solution {
public:
    vector<vector<int>> threeSum(vector<int>& nums) {
        vector<vector<int>> res;
        
        int l = nums.size();
        if (l == 0) {
            return res;
        }
        
        
        unordered_map<int, int> m;
        
        for (int n : nums) {
            int& found = m[n];
            if (found) {
                found++;
            } else {
                found = 1;
            }
        }
        
        sort(nums.begin(), nums.end());
        
        // a + b + c = 0
        // assume a <= b <= c
        // a < 0, 0 < c
        
        // a <- nums[i] c <- nums[j] below
        
        for (int i = 0; nums[i] < 0; i++) {
            if (i > 0 && nums[i] == nums[i - 1]) {
                continue;
            }
            for (int j = l - 1; nums[j] > 0; j--) {
                if (j < l - 1 && nums[j] == nums[j + 1]) {
                    continue;
                }
                int rem = 0 - nums[i] - nums[j];
                if (rem == nums[i]) {
                    if (m[nums[i]] >= 2) {
                        res.push_back(vector<int>({nums[i], nums[j], nums[i]}));
                    }
                } else if (rem == nums[j]) {
                    if (m[nums[j]] >= 2) {
                        res.push_back(vector<int>({nums[i], nums[j], nums[j]}));
                    }
                } else if (m[rem] && nums[i] < rem && rem < nums[j]) {
                    res.push_back(vector<int>({nums[i], rem, nums[j]}));
                }
            }
        }
        if (m[0] >= 3) {
            res.push_back(c(0, 0, 0));
        }
        return res;
    }
    
    vector<int> c(int a, int b, int c) {
        vector<int> res = {a, b, c};
        return res;
    }
};
