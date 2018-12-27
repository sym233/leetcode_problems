class Solution {
private:
    const int max = 8;
public:
    vector<int> prisonAfterNDays(vector<int>& cells, int N) {
        // use bit to represent cells
        vector<int> seq((1 << 8) - 1, 0);
        int c = toInt(cells);
        for (int i = 0; i < N;) {
            if (seq[c] > 0) {
                int period = i - seq[c];
                if (i + period > N) {
                    i++;
                } else {
                    int d = (N - i - 1) / period * period;
                    i += d + 1;
                }
            } else {
                seq[c] = i;
                i++;
            }
            
            c = next(c);
        }
        return toCells(c);
    }
    
private:
    int toInt(vector<int>& cells) {
        int c = 0;
        for (int i = 0; i < max; i++) {
            c |= cells[i] << i;
        }
        return c;
    }
    vector<int> toCells(int c) {
        vector<int> newCells(max, 0);
        for (int i = 0; i < max; i++) {
            newCells[i] = (c >> i) & 1;
        }
        return newCells;
    }
    int next(int cells) {
        return (~((cells >> 1) ^ (cells << 1))) & ((1 << (max - 1)) - 2);
    }
};
