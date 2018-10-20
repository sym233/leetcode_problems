class Solution {
public:
    bool containsNearbyDuplicate(vector<int>& nums, int k) {
        unordered_map<int, int> m;
        
        int l = nums.size();
        
        for (int i = 0; i < l; i++) {
            int num = nums[i];
            unordered_map<int, int>::iterator it = m.find(num);
            if (it == m.end()) {
                m[num] = i;
            } else {
                if (i - it->second <= k) {
                    return true;
                } else {
                    it->second = i;
                }
            }
        }
        return false;
    }
};
