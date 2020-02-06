class Solution {
private:
    struct Cell {
        int r;
        int c;
        int gold;
        Cell (int r, int c, int gold): r(r), c(c), gold(gold) {}
    };
    vector<vector<int>> cellIndex;
    // index count from 1, 0 indecates no gold
    
    vector<Cell> cells;
    
    vector<int> beginningCells;
    vector<int> secondBeginningCells;
    // store cell indexes
    
    struct State {
        vector<int> path;
        int goldCount;
        State(vector<int> path, int goldCount): path(path), goldCount(goldCount) {}
    };
    
public:
    int getMaximumGold(vector<vector<int>>& grid) {
        const int m = grid.size();
        const int n = grid[0].size();
        cellIndex = vector<vector<int>>(m, vector<int>(n, 0));
        cells = vector<Cell>{};
        
        auto adjustGoldCells = [&grid, this, m, n](int r, int c) -> vector<int> {
            vector<int> adjusts{};
            if (r - 1 >= 0 && grid[r - 1][c]) {
                adjusts.push_back(this->cellIndex[r - 1][c]);
            }
            if (c - 1 >= 0 && grid[r][c - 1]) {
                adjusts.push_back(this->cellIndex[r][c - 1]);
            }
            if (r + 1 < m && grid[r + 1][c]) {
                adjusts.push_back(this->cellIndex[r + 1][c]);
            }
            if (c + 1 < n && grid[r][c + 1]) {
                adjusts.push_back(this->cellIndex[r][c + 1]);
            }
            return adjusts;
        };
        
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (grid[i][j]) {
                    cells.push_back(Cell(i, j, grid[i][j]));
                    cellIndex[i][j] = cells.size();
                    vector<int> adj = adjustGoldCells(i, j);
                    if (adj.size() <= 1) {
                        beginningCells.push_back(cells.size());
                    }
                    if (adj.size() == 2) {
                        secondBeginningCells.push_back(cells.size());
                    }
                }
            }
        }
        
        if (beginningCells.size() == 0) {
            beginningCells = secondBeginningCells;
        }
        
        queue<State> q;
        
        for (int cellIndex : beginningCells) {
            q.push(State(vector<int>{cellIndex}, cells[cellIndex - 1].gold));
        }
        
        int maxGold = 0;
        vector<int> p; 
        
        while (!q.empty()) {
            State& state = q.front();
            if (state.goldCount > maxGold) {
                maxGold = state.goldCount;
                p = state.path;
            }
            vector<int>& path = state.path;
            if (path.size() == cells.size()) {
                break;
            }
            int curr = path.back();
            int r = cells[curr - 1].r;
            int c = cells[curr - 1].c;
            for (int adjust : adjustGoldCells(r, c)) {
                if (find(path.begin(), path.end(), adjust) == path.end()) {
                    vector<int> newPath = vector<int>(path);
                    newPath.push_back(adjust);
                    int newGold = state.goldCount + cells[adjust - 1].gold;
                    q.push(State(newPath, newGold));
                }
            }
            q.pop();
        }
        return maxGold;
    }
};
