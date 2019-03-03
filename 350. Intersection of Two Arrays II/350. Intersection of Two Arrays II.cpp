class Solution {
public:
    vector<int> intersect(vector<int>& nums1, vector<int>& nums2) {
        unordered_map<int, int> m;
        vector<int> res;
        for (int n : nums1) {
            m[n]++;
        }
        
        for (int n : nums2) {
            unordered_map<int, int>::iterator it = m.find(n);
            if (it != m.end() && it->second > 0) {
                res.push_back(n);
                it->second--;
            }
        }
        return res;
    }
};
