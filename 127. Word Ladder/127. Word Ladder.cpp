class Solution {
public:
    int ladderLength(string beginWord, string endWord, vector<string>& wordList) {
        vector<string>::iterator it = find(wordList.begin(), wordList.end(), endWord);
        if (it == wordList.end()) {
            return 0;
        }
        int e = it - wordList.begin();
        wordList.push_back(beginWord);
        int l = wordList.size();
        int s = l - 1;
        
        vector<vector<int>> edges(l, vector<int>());
        
        for (int i = 0; i + 1 < l; i++) {
            for (int j = i + 1; j < l; j++) {
                if (diff(wordList[i], wordList[j])) {
                    edges[i].push_back(j);
                    edges[j].push_back(i);
                }
            }
        }
        
        vector<int> visitState(l, INT_MAX);
        visitState[s] = 1;
        queue<int> q;
        q.push(s);
        
        while (q.size() > 0) {
            int f = q.front();
            if (f == e) {
                return visitState[f];
            }
            for (int next : edges[f]) {
                if (visitState[next] > visitState[f] + 1) {
                    visitState[next] = visitState[f] + 1;
                    q.push(next);
                }
            }
            q.pop();
        }
        return 0;
    }
    
    bool diff(string a, string b) {
        int l = a.size();
        if (l == b.size()) {
            int d = 0;
            for (int i = 0; i < l; i++) {
                if (a[i] != b[i]) {
                    d++;
                    if (d >= 2) {
                        return false;
                    }
                }
            }
            return d == 1;
        } else {
            return false;
        }
    }
};
