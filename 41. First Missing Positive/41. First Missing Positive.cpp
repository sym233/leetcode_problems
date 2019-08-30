auto indexOf = [](vector<int>& v, int val, int from = 0) -> int {
    for (int i = from; i < v.size(); i++) {
        if (v[i] == val) {
            return i;
        }
    }
    return -1;
};

class Solution {
public:
    int firstMissingPositive(vector<int>& nums) {
        int l = nums.size();
        int comp = 0;
        
        while (comp < l) {
            if (nums[comp] == comp + 1) {
                comp++;
                continue;
            }
            int ind = indexOf(nums, comp + 1, comp);
            if (ind == -1) {
                return comp + 1;
            }
            swap(nums[ind], nums[comp]);
            while (comp < ind && ind < l && nums[ind] != ind && nums[comp] < nums[ind] && nums[ind] < l) {
                swap(nums[ind], nums[nums[ind]]);
                ind = nums[ind];
            }
        }
        return l + 1;
    }
};
