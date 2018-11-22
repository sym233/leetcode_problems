class Solution {
private:
    struct Point {
        int r;
        int c;
        Point(int r, int c) : r(r), c(c) {}
        bool operator==(const Point& p2) {
            return r == p2.r && c == p2.c;
        }
    };
    bool isAdjacent(const Point& pointA, const Point& pointB) {
        int row = abs(pointA.r - pointB.r);
        int col = abs(pointA.c - pointB.c);
        return row * col == 0 && row + col == 1;
    }
public:
    bool exist(vector<vector<char>>& board, string word) {
        vector<vector<Point>> v(128, vector<Point>());
        
        int row = board.size();
        int col = board[0].size();
        
        for (int r = 0; r < row; r++) {
            for (int c = 0; c < col; c++) {
                char letter = board[r][c];
                v[letter].push_back(Point(r, c));
            }
        }
        
        struct SItem {
            vector<Point> path;
            int letterIndex;
            SItem(vector<Point> path) : path(path), letterIndex(0) {}
        };
        stack<SItem> s;
            
        for (Point& p : v[word[0]]) {
            s.push(SItem(vector<Point>({p})));
        }
        int l = word.size();
        
        while (!s.empty()) {
            SItem& top = s.top();
            vector<Point>& path = top.path;
            int pathL = path.size();
            
            if (pathL == l) {
                return true;
            }
            
            vector<Point>& letters = v[word[pathL]];
            
            if (top.letterIndex < letters.size()) {
                Point& thisLetter = letters[top.letterIndex];
                top.letterIndex++;
                if (isAdjacent(path.back(), thisLetter)) {
                    if (find(path.begin(), path.end(), thisLetter) == path.end()) {
                        SItem newItem(path);
                        newItem.path.push_back(thisLetter);
                        s.push(newItem);
                    }
                }
            } else {
                s.pop();
            }
        }
        return false;
    }
};
