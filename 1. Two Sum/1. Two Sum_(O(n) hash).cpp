class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        unordered_map<int, int> m;
        
        int l = nums.size();
        
        vector<int> res = {-1, -1};
        
        for (int i = 0; i < l; i++) {
            int rem = target - nums[i];
            unordered_map<int, int>::iterator it = m.find(rem);
            if (it != m.end()) {
                res[0] = it->second;
                res[1] = i;
                break;
            } else {
                m[nums[i]] = i;
            }
        }
        return res;
    }
};
