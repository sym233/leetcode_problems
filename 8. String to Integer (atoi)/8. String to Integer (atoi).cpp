class Solution {
public:
    int myAtoi(string str) {
        bool beginNum = false;
        bool isNegative = false;
        long long res = 0;
        for (char c : str) {
            if (beginNum) {
                if ('0' <= c && c <= '9') {
                    res *= 10;
                    res += c - '0';
                    if (res - 1 > INT_MAX) {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                if (c == '+' || c == '-') {
                    beginNum = true;
                    isNegative = c == '-';
                } else if ('0' <= c && c <= '9') {
                    beginNum = true;
                    res = c - '0';
                } else if (c == ' ') {
                    continue;
                } else {
                    break;
                }
            }
        }
        if (beginNum) {   
            if (isNegative) {
                res *= -1;
                if (res < INT_MIN) {
                    res = INT_MIN;
                }
            } else {
                if (res > INT_MAX) {
                    res = INT_MAX;

                }
            }
        }
        return static_cast<int>(res);
    }
};
