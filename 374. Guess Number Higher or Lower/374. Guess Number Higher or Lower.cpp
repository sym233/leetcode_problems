// Forward declaration of guess API.
// @param num, your guess
// @return -1 if my number is lower, 1 if my number is higher, otherwise return 0
int guess(int num);

class Solution {
public:
    int guessNumber(int n) {
        return interval(1, n + 1u);
    }
private:
    int interval(unsigned int s, unsigned int e) {
        int mid = (s + e) / 2;
        int r = guess(mid);
        if (r == 1) {
            return interval(mid, e);
        } else if (r == -1) {
            return interval(s, mid);
        } else {
            return mid;
        }
    }
};
