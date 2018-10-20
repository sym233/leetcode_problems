class Solution {
public:
    vector<pair<int, int>> getSkyline(vector<vector<int>>& buildings) {
        map<int, pair<int, int>> m;
        
        for (vector<int>& building : buildings) {
            int l = building[0];
            int r = building[1];
            m[l] = make_pair(0, 0);
            m[r] = make_pair(0, 0);
        }
        
        for (vector<int>& building : buildings) {
            int l = building[0];
            int r = building[1];
            int h = building[2];
            
            map<int, pair<int, int>>::iterator left = m.find(l);
            map<int, pair<int, int>>::iterator right = m.find(r);
            
            for (map<int, pair<int, int>>::iterator it = left;; it++) {
                if (it == left){
                    if (it->second.second < h) {
                        it->second.second = h;
                    }
                } else if (it == right) {
                    if (it->second.first < h) {
                        it->second.first = h;
                    }
                    break;
                } else {
                    if (it->second.first < h) {
                        it->second.first = h;
                    }
                    if (it->second.second < h) {
                        it->second.second = h;
                    }
                }
            }
        }
        
        vector<pair<int, int>> res;
        for (map<int, pair<int, int>>::iterator it = m.begin(); it != m.end(); it++) {
            if (it->second.first != it->second.second) {
                res.push_back(make_pair(it->first, it->second.second));
            }
        }
        return res;
    }
};
