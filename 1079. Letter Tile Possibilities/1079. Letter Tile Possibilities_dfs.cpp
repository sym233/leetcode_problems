class Solution {
public:
    int numTilePossibilities(string tiles) {
        unordered_map<char, int> count;
        for (char c : tiles) {
            count[c]++;
        }
        
        vector<int> nums;
        for (auto it = count.begin(); it != count.end(); it++) {
            nums.push_back(it->second);
        }
        
        function<int(vector<int>)> dfs = [&dfs] (vector<int> nums) -> int {
            int l = nums.size();
            int sum = 1;
            for (int i = 0; i < l; i++) {
                if (nums[i] > 0) {
                    vector<int> newNums(nums);
                    newNums[i]--;
                    sum += dfs(newNums);
                }
            }
            return sum;
        };
        
        return dfs(nums) - 1;
    }
};
