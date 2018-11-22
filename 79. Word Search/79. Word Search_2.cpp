class Solution {
private:
    int row;
    int col;
    int l;
    bool test(vector<vector<char>>& board, int r, int c, string& word, int index) {
        if (board[r][c] == word[index]) {
            if (index + 1 == l) {
                return true;
            }
            board[r][c] ^= 128;
            bool temp = (r + 1 < row && test(board, r + 1, c, word, index + 1))
                || (c + 1 < col && test(board, r, c + 1, word, index + 1))
                || (r - 1 >= 0 && test(board, r - 1, c, word, index + 1))
                || (c - 1 >= 0 && test(board, r, c - 1, word, index + 1));
            board[r][c] ^= 128;
            return temp;
        } else {
            return false;
        }
    }
    
public:
    bool exist(vector<vector<char>>& board, string word) {
        row = board.size();
        col = board[0].size();
        l = word.size();
        for (int r = 0; r < row; r++) {
            for (int c = 0; c < col; c++) {
                if (test(board, r, c, word, 0)) {
                    return true;
                }
            }
        }
        
        return false;
    }
};
