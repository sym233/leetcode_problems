class Solution {
public:
    int firstMissingPositive(vector<int>& nums) {
        int l = nums.size();
        for (int i = 0; i < l; i++) {
            while (nums[i] != i + 1 && 0 < nums[i] && nums[i] <= l && nums[i] != nums[nums[i] - 1]) {
                swap(nums[i], nums[nums[i] - 1]);
            }
        }
        for (int i = 0; i < l; i++) {
            if (nums[i] != i + 1) {
                return i + 1;
            }
        }
        return l + 1;
    }
};
