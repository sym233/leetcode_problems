/**
 * Definition for an interval.
 * struct Interval {
 *     int start;
 *     int end;
 *     Interval() : start(0), end(0) {}
 *     Interval(int s, int e) : start(s), end(e) {}
 * };
 */
class SummaryRanges {
    set<int> s;
public:
    /** Initialize your data structure here. */
    SummaryRanges() {
        s.clear();
    }
    
    void addNum(int val) {
        s.insert(val);
    }
    
    vector<Interval> getIntervals() {
        vector<Interval> v;
        if (s.empty()) {
            return v;
        }
        set<int>::iterator it = s.begin();
        int start = *it;
        int end = *it;
        for (; it != s.end(); it++) {
            if (*it - end > 1) {
                v.push_back(Interval(start, end));
                start = *it;
                end = *it;
            } else {
                end = *it;
            }
        }
        v.push_back(Interval(start, end));
        return v;
    }
};

/**
 * Your SummaryRanges object will be instantiated and called as such:
 * SummaryRanges* obj = new SummaryRanges();
 * obj->addNum(val);
 * vector<Interval> param_2 = obj->getIntervals();
 */
 