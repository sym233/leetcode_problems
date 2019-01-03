static const auto speedup = []() {std::ios::sync_with_stdio(false); std::cin.tie(nullptr); return 0; }();
class Solution {
public:
    double findMedianSortedArrays(vector<int>& nums1, vector<int>& nums2) {
        int len1 = nums1.size();
        int len2 = nums2.size();
        int totLen = len1 + len2;
        bool isOdd = totLen % 2 == 1;
        
        int s = 0;
        int e = len1 + 1;
        // split nums1: [0, mid1) , [mid1, len1)
        // split nums2: [0, mid2) , [mid2, len2)
        // where (mid1 - 0) + (mid2 - 0) = (len1 + len2) / 2
        // all{ nums1[0, mid1), nums2[0, mid2) } <= all{ nums1[mid1, len1) || nums2[mid2, len2) }

        for (;;) {
            int mid1 = (s + e) / 2;
            int mid2 = totLen / 2 - mid1;
            
            if (mid2 < 0) {
                // mid1 too large
                e = mid1;
                continue;
            } else if (mid2 > len2) {
                // mid1 too small
                s = mid1;
                continue;
            } else {
                if (0 < mid1 && mid2 < len2 && nums2[mid2] < nums1[mid1 - 1]) {
                    // mid1 too large
                    e = mid1;
                    continue;
                } else if (mid1 < len1 && 0 < mid2 && nums1[mid1] < nums2[mid2 - 1]) {
                    // mid1 too small
                    s = mid1;
                    continue;
                } else {
                    // split done
                    vector<int> right;
                    if (mid1 < len1) {
                        right.push_back(nums1[mid1]);
                    }
                    if (mid2 < len2) {
                        right.push_back(nums2[mid2]);
                    }
                    int min = *min_element(right.begin(), right.end());
                    if (isOdd) {
                        return min;
                    } else {
                        vector<int> left;
                        if (0 < mid1) {
                            left.push_back(nums1[mid1 - 1]);
                        }
                        if (0 < mid2) {
                            left.push_back(nums2[mid2 - 1]);
                        }
                        int max = *max_element(left.begin(), left.end());
                        return (max + min) / 2.0;
                    }
                }
            }
        }
    }
};
