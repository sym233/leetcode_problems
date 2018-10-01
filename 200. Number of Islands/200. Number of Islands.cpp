class Solution {
public:
    int numIslands(vector<vector<char>>& grid) {
        
        struct Point {
            int h;
            int w;
            Point (int h, int w) : h(h), w(w) {}
            Point operator+ (Point& p) const {
                return Point(h + p.h, w + p.w);
            }
        };
        
        array<Point, 4> directions = {
            Point(1, 0),
            Point(0, 1),
            Point(-1, 0),
            Point(0, -1),
        };
        
        int height = grid.size();
        if (height == 0) {
            return 0;
        }
        int width = grid[0].size();
        
        vector<Point> lands;
        for (int i = 0; i < height; i++) {
            for (int j = 0; j < width; j++) {
                if (grid[i][j] != '0') {
                    lands.push_back(Point(i, j));
                }
            }
        }
        
        int landCount = 0;
        queue<Point> q;
        for (Point& point : lands) {
            if (grid[point.h][point.w] == '1') {
                
                q.push_back(point);
                grid[point.h][point.w] = '0';
                landCount++;
                
                while (q.size() > 0) {
                    Point& p = q.front();
                    
                    for (Point direction : directions) {
                        Point newPoint = direction + p;
                        
                        int h = newPoint.h;
                        int w = newPoint.w;
                        if (0 <= h && h < height && 
                            0 <= w && w < width && 
                            grid[h][w] == '1') {
                            
                            grid[h][w] = '0';
                            q.push_back(Point(h, w));
                        }
                    }

                    q.pop_front();
                }
            }
        }
        return landCount;
    }
};
