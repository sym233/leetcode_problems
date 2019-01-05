static int _ = [](){ 
    std::ios::sync_with_stdio(false); 
    cin.tie(NULL);
    cout.tie(NULL);
    return 0; 
}();
class Solution {
public:
    int divide(int dividend, int divisor) {
        int max = 2147483647;
        int min = -2147483648;
        if (dividend == min && divisor == -1) {
            return max;
        }
        if (divisor == 1) {
            return dividend;
        }
        if (dividend == 0) {
            return 0;
        }
        if (divisor == 0) {
            throw("0");
        }
        int sign = (dividend > 0 ^ divisor > 0) ? -1 : 1;
        double a = dividend;
        a = abs(a);
        
        double b = divisor;
        b = abs(b);
        
        int res = static_cast<int>(exp(log(a) - log(b)));
        return sign * res;
    }
};
