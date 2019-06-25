class Solution {
public:
    int shortestPathBinaryMatrix(vector<vector<int>>& grid) {
        struct Cor {
            int r;
            int c;
            Cor(int r, int c) : r(r), c(c) {}
            Cor operator+ (const Cor& cor) {
                return Cor(r + cor.r, c + cor.c);
            }
            bool operator== (const Cor& cor) {
                return r == cor.r && c == cor.c;
            }
            Cor to(int i) {
                // i in range(9)
                int d[] = {1, -1, 0};
                return *this + Cor(d[i % 3], d[i / 3]);
            }
        };
        
        int N = grid.size();
        vector<vector<bool>> visited(N, vector<bool>(N, false));
        visited[0][0] = true;
        
        auto inside = [&N](Cor& cor) -> bool {
            return 0 <= cor.r && cor.r < N && 0 <= cor.c && cor.c < N;
        };
        auto empty = [&grid](Cor& cor) -> bool {
            return grid[cor.r][cor.c] == 0;
        };
        auto visit = [&visited](Cor& cor) -> void {
            visited[cor.r][cor.c] = true;
        };
        auto has_visited = [&visited](Cor& cor) -> bool {
            return visited[cor.r][cor.c];
        };
        
        Cor start = Cor(0, 0);
        Cor dest = Cor(N - 1, N - 1);
        if (!empty(start)) {
            return -1;
        }
        
        struct Rec {
            Cor cor;
            int len;
            Rec(const Cor& cor, int len) : cor(cor), len(len) {}
        };
        queue<Rec> q;
        q.push(Rec(start, 1));
        while (!q.empty()) {
            Rec& rec = q.front();
            Cor& cor = rec.cor;
            if (cor == dest) {
                return rec.len;
            }
            for (int i = 0; i < 9; i++) {
                Cor next_cor = cor.to(i);
                if (inside(next_cor) && empty(next_cor) && !has_visited(next_cor)) {
                    Rec next_rec = Rec(next_cor, rec.len + 1);
                    visit(next_cor);
                    q.push(next_rec);
                }
            }
            q.pop();
        }
        return -1;
    }
};
