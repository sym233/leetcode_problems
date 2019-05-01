class Solution {
public:
    vector<int> gridIllumination(int N, vector<vector<int>>& lamps, vector<vector<int>>& queries) {
        int l = lamps.size();
        vector<bool> lit(l, true);
        
        using M = unordered_multimap<int, int>;
        using MI = M::iterator;
        M xPos, yPos, xPlusY, xMinusY;
        
        for (int i = 0; i < l; i++) {
            int x = lamps[i][0];
            int y = lamps[i][1];
            
            xPos.insert(make_pair(x, i));
            yPos.insert(make_pair(y, i));
            xPlusY.insert(make_pair(x + y, i));
            xMinusY.insert(make_pair(x - y, i));
        }
        
        vector<int> answer;
        
        auto checkIlluminated = [&lit](pair<MI, MI>& range) {
            for (MI it = range.first; it != range.second; it++) {
                int i = it->second;
                if (lit[i]) {
                    return true;
                }
            }
            return false;
        };
        
        for (vector<int>& query : queries) {
            int x = query[0];
            int y = query[1];
                        
            // check if illuminated
            pair<MI, MI> xRange = xPos.equal_range(x);
            pair<MI, MI> yRange = yPos.equal_range(y);
            pair<MI, MI> xPlusYRange = xPlusY.equal_range(x + y);
            pair<MI, MI> xMinusYRange = xMinusY.equal_range(x - y);
            bool illuminated = checkIlluminated(xRange) || checkIlluminated(yRange) || checkIlluminated(xPlusYRange) || checkIlluminated (xMinusYRange);
            answer.push_back(illuminated ? 1 : 0);
            
            // turn off lamp
            for (int offsetX = -1; offsetX < 2; offsetX++) {
                // offsetX = -1, 0, 1
                pair<MI, MI> range = xPos.equal_range(x + offsetX);
                for (MI it = range.first; it != range.second; it++) {
                    int i = it->second;
                    int lampY = lamps[i][1];
                    if (abs(lampY - y) <= 1 && lit[i]) {
                        lit[i] = false;
                    }
                }
            }
        }
        return answer;
    }
};
