/**
 * Definition for an interval.
 * struct Interval {
 *     int start;
 *     int end;
 *     Interval() : start(0), end(0) {}
 *     Interval(int s, int e) : start(s), end(e) {}
 * };
 */


bool intervalsShouldMerge(Interval a, Interval b) {
    // assert Interval.start <= Interval.end 
    if (a.start < b.start) {
        if (a.end < b.start) {
            return false;
        } else {
            return true;
        }
    } else if (b.start < a.start) {
        if (b.end < a.start) {
            return false;
        } else {
            return true;
        }
    } else {
        return true;
    }
}

int max(int a, int b) {
    return a > b ? a : b;
}

Interval mergeIntervals(Interval a, Interval b) {
    // assert Interval.start <= Interval.end 
    if (a.start < b.start) {
        if (a.end < b.start) {
            // error
        } else {
            return Interval(a.start, max(a.end, b.end));
        }
    } else if (b.start < a.start) {
        if (b.end < a.start) {
            // error
        } else {
            return Interval(b.start, max(a.end, b.end));
        }
    } else {
        return Interval(a.start, max(a.end, b.end));
    }
    char s[1000];
    sprintf(s, "merge error for [%d, %d] and [%d, %d].", a.start, a.end, b.start, b.end);
    throw s;
}

void checkInterval(Interval& a) {
    // ensure Interval.start <= Interval.end
    if (a.start > a.end) {
        int t = a.start;
        a.start = a.end;
        a.end = t;
    }
}

bool comp(Interval a, Interval b) {
    if (a.start < b.start) {
        return true;
    } else if (b.start < a.start) {
        return false;
    } else {
        return b.end > a.end;
    }
}

class Solution {
public:
    vector<Interval> merge(vector<Interval>& intervals) {
           
        for (Interval& interval : intervals) {
            checkInterval(interval);
        }
        
        sort(intervals.begin(), intervals.end(), comp);
        vector<Interval> res;
        
        int l = intervals.size();
        
        if (l > 0) {
            res.push_back(intervals[0]);

            for (int i = 1; i < l; i++) {
                if (intervalsShouldMerge(res.back(), intervals[i])) {
                    Interval newInterval = mergeIntervals(res.back(), intervals[i]);
                    res.pop_back();
                    res.push_back(newInterval);
                } else {
                    res.push_back(intervals[i]);
                }
            }
        }
        
        return res;
    }
};
