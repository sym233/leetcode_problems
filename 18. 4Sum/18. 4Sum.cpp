class Solution {
public:
    vector<vector<int>> fourSum(vector<int>& nums, int target) {
        vector<vector<int>> res;
        int l = nums.size();
        if (l < 3) {
            return res;
        }
        
        sort(nums.begin(), nums.end());
        
        for (int i = 0; i < l - 3; /* i++ */) {
            for (int h = i + 1; h < l - 2; /* h++ */) {
            
                int j = h + 1;
                int k = l - 1;

                while (j < k) {
                    int value = nums[i] + nums[h] + nums[j] + nums[k];

                    if (value > target) {
                        do {
                            k--;
                        } while (nums[k] == nums[k + 1] && j < k);
                    } else if (value < target) {
                        do {
                            j++;
                        } while (nums[j] == nums[j - 1] && j < k);
                    } else {
                        res.push_back(vector<int>({nums[i], nums[h], nums[j], nums[k]}));
                        do {
                            j++;
                        } while (nums[j] == nums[j - 1] && j < k);
                        do {
                            k--;
                        } while (nums[k] == nums[k + 1] && j < k);
                    }
                }
                do {
                    h++;
                } while (nums[h] == nums[h - 1] && h < l - 2);
            }
            do {
                i++;
            } while (nums[i] == nums[i - 1] && i < l - 3);
            
        }
        return res;
    }
};
