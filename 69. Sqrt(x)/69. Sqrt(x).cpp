class Solution {
public:
    int mySqrt(int x) {
        if (x == 0) {
            return 0;
        }
        double delta = 0.01;
        double y = x;
        double a = y / 2;
        double lastA = -1;
        for(;;) {
            a = f(y, a);
            if (abs(a - lastA) <= delta) {
                return a;
            }
            lastA = a;
        }
    }
    inline double f(double y, double a) {
        return y / 2 / a + a / 2;
    }
};
