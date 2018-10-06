class Solution {
public:
    int threeSumClosest(vector<int>& nums, int target) {
        int sum = INT_MAX;
        int distance = abs(INT_MAX);
        
        int l = nums.size();
        
        sort(nums.begin(), nums.end());
        
        for (int i = 0; i < l - 2; i++) {
            if (i > 0 && nums[i] == nums[i - 1]) {
                continue;
            }
            
            int j = i + 1;
            int k = l - 1;
            
            while (j < k) {
                int value = nums[i] + nums[j] + nums[k];
                if (distance > abs(value - target)) {
                    sum = value;
                    distance = abs(value - target);
                }
                if (value > target) {
                    do {
                        k--;
                    } while (nums[k] == nums[k + 1] && j < k);
                } else if (value < target) {
                    do {
                        j++;
                    } while (nums[j] == nums[j - 1] && j < k);
                } else {
                    // value == target
                    return target;
                }
            }   
        }
        return sum;
    }
};
