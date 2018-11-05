class Solution {
public:
    vector<int> singleNumber(vector<int>& nums) {
        unordered_set<int> s;
        
        for (int num : nums) {
            unordered_set<int>::iterator it = s.find(num);
            if (it == s.end()) {
                s.insert(num);
            } else {
                s.erase(it);
            }
        }
        return vector<int>(s.begin(), s.end());
    }
};
