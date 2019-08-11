class Solution {
public:
    vector<int> shortestAlternatingPaths(int n, vector<vector<int>>& red_edges, vector<vector<int>>& blue_edges) {
        vector<vector<int>> r_edges(n);
        vector<vector<int>> b_edges(n);
        for (vector<int>& r_e : red_edges) {
            int s = r_e[0];
            int e = r_e[1];
            r_edges[s].push_back(e);
        }
        for (vector<int>& b_e : blue_edges) {
            int s = b_e[0];
            int e = b_e[1];
            b_edges[s].push_back(e);
        }
        
        int max = numeric_limits<int>::max();
        vector<int> red_dis(n, max);
        vector<int> blue_dis(n, max);
        
        red_dis[0] = 0;
        blue_dis[0] = 0;
        
        queue<int> q;
        
        q.push(0);
        
        while (!q.empty()) {
            
            int p = q.front();
            // new red paths
            for (int e : r_edges[p]) {
                if (blue_dis[p] != max && blue_dis[p] + 1 < red_dis[e]) {
                    red_dis[e] = blue_dis[p] + 1;
                    q.push(e);
                }
            }
            
            // new blue paths
            for (int e : b_edges[p]) {
                if (red_dis[p] != max && red_dis[p] + 1 < blue_dis[e]) {
                    blue_dis[e] = red_dis[p] + 1;
                    q.push(e);
                }
            }
            q.pop();
        }
        
        vector<int> res(n);
        for (int i = 0; i < n; i++) {
            int l = blue_dis[i] < red_dis[i] ? blue_dis[i] : red_dis[i];
            res[i] = l == max ? -1 : l;
        }
        return res;
    }
};
