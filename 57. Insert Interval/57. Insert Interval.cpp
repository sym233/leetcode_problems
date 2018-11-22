/**
 * Definition for an interval.
 * struct Interval {
 *     int start;
 *     int end;
 *     Interval() : start(0), end(0) {}
 *     Interval(int s, int e) : start(s), end(e) {}
 * };
 */
class Solution {
public:
    vector<Interval> insert(vector<Interval>& intervals, Interval newInterval) {
        // m#second > 0 : interval(s) start at m#first
        // m#second < 0 : interval(s) end at m#first
        // m#second == 0 : interval(s) begin and end offset, or single point interval
        map<int, int> m;
        intervals.push_back(newInterval);
        
        for (Interval& interval : intervals) {
            int s = interval.start;
            int e = interval.end;
            
            map<int, int>::iterator sIt = m.find(s);
            if (sIt == m.end()) {
                m.insert(pair<int, int>(s, 1));
            } else {
                sIt->second++;
            }
            
            map<int, int>::iterator eIt = m.find(e);
            if (eIt == m.end()) {
                m.insert(pair<int, int>(e, -1));
            } else {
                eIt->second--;
            }
        }
        
        vector<Interval> mergedIntervals;
        int intervalStartCount = 0;
        Interval tempInterval;
        
        for (const pair<int, int>& p : m) {
            if (intervalStartCount == 0) {
                if (p.second > 0) {
                    tempInterval.start = p.first;
                    intervalStartCount += p.second;
                } else if (p.second == 0) {
                    // proceed single point interval
                    tempInterval.start = p.first;
                    tempInterval.end = p.first;
                    mergedIntervals.push_back(tempInterval);
                }
            } else {
                intervalStartCount += p.second;
                if (intervalStartCount == 0) {
                    tempInterval.end = p.first;
                    mergedIntervals.push_back(tempInterval);
                }
            }
        }
        return mergedIntervals;
    }
};
