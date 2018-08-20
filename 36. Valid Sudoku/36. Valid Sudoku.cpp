bool valid1To9(vector<char>& chars) {
    vector<int> nums(10, 0);
    for (char c : chars) {
        nums[c - '0']++;
        if (nums[c - '0'] > 1) {
            return false;
        }
    }
    return true;
}

class Solution {
public:
    bool isValidSudoku(vector<vector<char>>& board) {
        // valid row
        for (int i = 0; i < 9; i++) {
            vector<char> chars;
            for (int j = 0; j < 9; j++) {
                if ('0' <= board[i][j] && board[i][j] <= '9') {
                    chars.push_back(board[i][j]);
                }
            }
            
            if (!valid1To9(chars)) {
                return false;
            }
        }
        
        // valid col
        for (int j = 0; j < 9; j++) {
            vector<char> chars;
            for (int i = 0; i < 9; i++) {
                if ('0' <= board[i][j] && board[i][j] <= '9') {
                    chars.push_back(board[i][j]);
                }
            }
            
            if (!valid1To9(chars)) {
                return false;
            }
        }
        
        // valid sub-box
        for (int i = 0; i < 9; i += 3) {
            for (int j = 0; j < 9; j += 3) {
                vector<char> chars;
                for (int subI = 0; subI < 3; subI++) {
                    for (int subJ = 0; subJ < 3; subJ++) {
                        char c = board[i + subI][j + subJ];
                        if ('0' <= c && c <= '9') {
                            chars.push_back(c);
                        }
                    }
                }
                if (!valid1To9(chars)) {
                    return false;
                }
            }
        }
        
        return true;
    }
};
