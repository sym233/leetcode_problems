class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        struct NumAndIndex {
            int num;
            int index;
            NumAndIndex(int num, int index) : num(num), index(index) {}
        };
        
        int l = nums.size();
        
        vector<NumAndIndex> n;
        
        for (int i = 0; i < l; i++) {
            n.push_back(NumAndIndex(nums[i], i));
        }
        
        sort(n.begin(), n.end(), [](NumAndIndex i, NumAndIndex j) {
            return i.num < j.num;
        });
        
        vector<int> res = {-1, -1};
        
        int i = 0;
        int j = l - 1;
        
        while (i < j) {
            int t = n[i].num + n[j].num;
            if (t == target) {
                res[0] = n[i].index;
                res[1] = n[j].index;
                break;
            } else if (t < target) {
                i++;
            } else {
                j--;
            }
        }
        return res;
    }
};
