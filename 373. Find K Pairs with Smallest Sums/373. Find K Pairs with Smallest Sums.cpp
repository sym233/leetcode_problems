class Solution {
public:
    vector<pair<int, int>> kSmallestPairs(vector<int>& nums1, vector<int>& nums2, int k) {
        vector<pair<int, int>> result;
        multimap<int, pair<int, int>> m;
        // map: sum => (nums1 index u, nums2 index v)
        // ensure the first element to be the smallest sum
        const int l1 = nums1.size();
        const int l2 = nums2.size();
        if (l1 == 0 || l2 == 0) {
            return result;
        }
        int count = 0; 
        
        m.insert(make_pair(nums1[0] + nums2[0], make_pair(0, 0)));
        // (num1[0], nums2[0]) must be the smallest
        
        vector<int> maxIndexV(l1, -1);
        // maxIndexV[u] = v means (nums1[u], nums2[0: v]) have been picked
        
        auto canPick = [&maxIndexV, l1, l2](int u, int v) {
            if (u >= l1 || v >= l2) {
                // index out of range
                return false;
            }
            if (u == 0) {
                // (u, v - 1) must have been picked
                return maxIndexV[u] >= v - 1;
            } else {
                // (u - 1, v) and (u, v - 1) must have been picked
                return maxIndexV[u - 1] >= v && maxIndexV[u] >= v - 1;
            }
        };
        
        while (result.size() < k) {
            if (m.empty()) {
                break;
            }
            auto p = m.begin();
            int u = p->second.first;
            int v = p->second.second;
            result.push_back(make_pair(nums1[u], nums2[v]));
            // cout << "pick " << u << ", " << v << endl;
            m.erase(p);
            maxIndexV[u] = v;
            if (canPick(u + 1, v)) {
                m.insert(make_pair(nums1[u + 1] + nums2[v], make_pair(u + 1, v)));
            }
            if (canPick(u, v + 1)) {
                m.insert(make_pair(nums1[u] + nums2[v + 1], make_pair(u, v + 1)));
            }
        }
        return result;
    }
};
