using VI = vector<int>;
class MajorityChecker {
private:
    unordered_map<int, VI> m;
public:
    MajorityChecker(VI& arr) {
        m.clear();
        int l = arr.size();
        for (int i = 0; i < l; i++) {
            m[arr[i]].push_back(i);
        }
    }
    
    int query(int left, int right, int threshold) {
        for (pair<const int, VI>& p : m) {
            const int& n = p.first;
            VI& v = p.second;
            VI::iterator lit = lower_bound(v.begin(), v.end(), left);
            VI::iterator rit = upper_bound(v.begin(), v.end(), right);
            if (rit - lit >= threshold) {
                return n;
            }
        }
        return -1;
    }
};

/**
 * Your MajorityChecker object will be instantiated and called as such:
 * MajorityChecker* obj = new MajorityChecker(arr);
 * int param_1 = obj->query(left,right,threshold);
 */
