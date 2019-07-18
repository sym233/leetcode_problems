/**
 * // This is the MountainArray's API interface.
 * // You should not implement it, or speculate about its implementation
 * class MountainArray {
 *   public:
 *     int get(int index);
 *     int length();
 * };
 */
class Solution {
public:
    int findInMountainArray(int target, MountainArray &mountainArr) {
        int l = mountainArr.length();
        if (l < 10) {
            for (int i = 0; i < l; i++) {
                if (mountainArr.get(i) == target) {
                    return i;
                }
            }
            return -1;
        }
        
        unordered_map<int, int> m;
        function<int(int)> get = [&m, &mountainArr](int i) -> int {
            if (m.find(i) != m.end()) {
                return m[i];
            }
            return m[i] = mountainArr.get(i);
        };
        
        auto bisearch = [](int start, int end, function<int(int)> f) -> int {
            for (int i = 0; i < 50 && start < end; i++) {
                int middle = (start + end) / 2;
                int dir = f(middle);
                if (dir < 0) {
                    end = middle;
                } else if (dir == 0) {
                    return middle;
                } else {
                    start = middle;
                }
            }
            return -1;
        };
        
        // find peak
        int peak = bisearch(0, l, [&get](int mid) {
            int le = get(mid - 1);
            int mi = get(mid);
            int ri = get(mid + 1);
            if (le < mi && mi > ri) {
                return 0;
            } else if (le < mi && mi < ri) {
                return 1;
            } else {
                return -1;
            }
        });
        
        if (get(peak) == target) {
            return peak;
        }
        
        // search left
        int find = bisearch(0, peak, [&get, &target](int mid) {
            int g = get(mid);
            if (g < target) {
                return 1;
            } else if (g == target) {
                return 0;
            } else {
                return -1;
            }
        });
        
        if (find != -1) {
            return find;
        }
        
        // search right
        find = bisearch(peak, l, [&get, &target](int mid) {
            int g = get(mid);
            if (g < target) {
                return -1;
            } else if (g == target) {
                return 0;
            } else {
                return 1;
            }
        });
        return find;
    }
};
