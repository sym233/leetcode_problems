class Solution {
public:
    void nextPermutation(vector<int>& nums) {
        int l = nums.size();
        int k = -1;
        for (int i = l - 2; i >= 0; i--) {
            if (nums[i] < nums[i + 1]) {
                k = i;
                break;
            }
        }
        
        if (k == -1) {
            // nums is descending
            reverse(nums.begin(), nums.end());
            return;
        }
        
        int j;
        for (int i = l - 1; i > k; i--) {
            if (nums[k] < nums[i]) {
                j = i;
                break;
            }
        }
        
        swap(nums[j], nums[k]);
        reverse(nums.begin() + k + 1, nums.end());
    }
};
