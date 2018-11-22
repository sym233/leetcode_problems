// 46. Permutations, 47. Permutations II two problems share a solution
// nextPermutation function comes from 31. Next Permutation

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
    vector<vector<int>> permute(vector<int>& nums) {
        vector<vector<int>> res;
        vector<int> newPermutation(nums);
        
        do {
            res.push_back(newPermutation);
            nextPermutation(newPermutation);
        } while (nums != newPermutation);
        
        return res;
    }
};
