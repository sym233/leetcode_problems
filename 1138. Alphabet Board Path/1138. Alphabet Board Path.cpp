class Solution {
public:
    string alphabetBoardPath(string target) {
        vector<string> board{"abcde", "fghij", "klmno", "pqrst", "uvwxy", "z"};
        unordered_map<char, pair<int, int>> m;
        for (int i = 0; i < 6; i++) {
            string& row = board[i];
            for (int j = 0; j < 5; j++) {
                if (j < row.size()) {
                    m[row[j]] = make_pair(i, j);
                }
            }
        }
        int r = 0;
        int c = 0;
        string res;
        for (char ch : target) {
            int tr = m[ch].first;
            int tc = m[ch].second;
            if (r < tr) {
                if (c < tc) {
                    for (; c < tc; c++) {
                        res.push_back('R');
                    }
                } else if (tc < c) {
                    for (; tc < c; c--) {
                        res.push_back('L');
                    }
                }
                for (; r < tr; r++) {
                    res.push_back('D');
                }
            } else {
                for (; tr < r; r--) {
                    res.push_back('U');
                }
                if (c < tc) {
                    for (; c < tc; c++) {
                        res.push_back('R');
                    }
                } else if (tc < c) {
                    for (; tc < c; c--) {
                        res.push_back('L');
                    }
                }
            }
            res.push_back('!');
        }
        return res;
    }
};
