class Solution {
public:
    bool possibleBipartition(int N, vector<vector<int>>& dislikes) {
        
        vector<vector<int>> edges(N + 1, vector<int>());
        vector<bool> verticeIsVisited(N + 1, false);
        
        
        for (vector<int>& dislike : dislikes) {
            int a = dislike[0];
            int b = dislike[1];
            edges[a].push_back(b);
        }
        
        for (int i = 1; i <= N; i++) {
            if (verticeIsVisited[i]) {
                continue;
            }
            
            vector<int> verticeLabels(N + 1, 0);
            vector<int> queue(N + 1, 0);
            queue.push_back(i);
            int queueHead = 0;
            verticeIsVisited[i] = true;
            
            while (queueHead < queue.size()) {
                int& prevVertice = queue[queueHead]
                for (int vertice : edges[prevVertice]) {
                    verticeIsVisited[vertice] = true;
                    if (verticeLabels[vertice] == 0) {
                        verticeLabels[vertice] = verticeLabels[prevVertice] + 1;
                        queue.push_back(vertice);
                    } else if ((verticeLabels[vertice] - verticeLabels[prevVertice]) % 2 == 0) {
                        // find a ring which has odd edges
                        return false;
                    } else {
                        // find a ring which has even edges
                        // pass
                    }
                }
                queueHead++;
            }
         
        }
        return true;
    }
};
