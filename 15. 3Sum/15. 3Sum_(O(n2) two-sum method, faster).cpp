class Solution {
public:
    vector<vector<int>> threeSum(vector<int>& nums) {
        vector<vector<int>> res;
        
        int l = nums.size();
        if (l < 2) {
            return res;
        }
        
        sort(nums.begin(), nums.end());
        
        for (int i = 0; i < l - 2; i++) {
            if (i > 0 && nums[i] == nums[i - 1]) {
                continue;
            }
            int j = i + 1;
            int k = l - 1;
            while (j < k) {
                int value = nums[i] + nums[j] + nums[k];
                
                if (value > 0) {
                    k--;
                } else if (value < 0) {
                    j++;
                } else {
                    res.push_back(vector<int>({nums[i], nums[j], nums[k]}));
                    do {
                        j++;
                    } while (nums[j] == nums[j - 1]);
                    do {
                        k--;
                    } while (nums[k] == nums[k + 1]);
                }
            }   
        }
        return res;
    }
};
