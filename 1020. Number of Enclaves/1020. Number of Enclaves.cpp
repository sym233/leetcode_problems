class Solution {
    struct Pos {
        int h;
        int w;
        Pos(int h, int w) : h(h), w(w) {}
        Pos up() {
            return Pos(h - 1, w);
        }
        Pos down() {
            return Pos(h + 1, w);
        }
        Pos left() {
            return Pos(h, w - 1);
        }
        Pos right() {
            return Pos (h, w + 1);
        }
        bool in(int height, int width) {
            return 0 <= h && h < height && 0 <= w && w < width;
        }
        template<typename T>
        T& pick(vector<vector<T>>& M) {
            return M[h][w];
        }
    };
public:
    int numEnclaves(vector<vector<int>>& A) {
        int height = A.size();
        int width = A[0].size();
        queue<Pos> q;
        
        for (int i = 0; i < height; i++) {
            for (int j = 0; j < width; j++) {
                if (i == 0 || i == height - 1 || j == 0 || j == width - 1) {
                    // boundary
                    q.push(Pos(i, j));
                }
            }
        }
        auto add = [&q, &A, height, width](Pos p){
            if (p.in(height, width) && p.pick(A) == 1) {
                q.push(p);
            }
            return;
        };
        while (q.size() > 0) {
            Pos& f = q.front();
            if (f.pick(A) == 1) {
                add(f.up());
                add(f.down());
                add(f.left());
                add(f.right());
                f.pick(A) = 0;
            }
            q.pop();
        }
        
        int sum = 0;
        for (vector<int>& row : A) {
            for (int& cell : row) {
                sum += cell;
            }
        }
        return sum;
    }
};
