class Solution {
private:
    const int modulo = 1e9 + 7;
    void m(auto& num) {
        while (num < 0) {
            num += modulo;
        }
        if (num > modulo) {
            num %= modulo;
        }
    }
    long long fastExp(long long base, long long exp, int mo) {
        if (base == 1 || exp == 0) {
            return 1;
        } else if (exp == 1) {
            return base;
        }
        if (exp % 2) {
            return fastExp(base, exp - 1, mo) * base % mo;
        } else {
            return fastExp(base * base % mo, exp / 2, mo);
        }
    }
    long long comb(int k, int n) {
        if (2 * k > n) {
            return comb(n - k, n);
        }
        long long res = 1;
        for (int i = 1; i <= k; i++) {
            res *= n - k + i;
            m(res);
            res *= fastExp(i, modulo - 2, modulo);
            m(res);
        }
        // = (n - k + 1) * (n - k + 2) * ... * (n - k + k) / (1 * 2 * ... * k) % modulo
        return res;
    }
public:
    int numRollsToTarget(int d, int f, int target) {
        long long res = 0;
        if (target < d || target > d * f) {
            return 0;
        }
        for (int k = 0; k <= (target - d) / f; k++) {
            if (k % 2) {
                res -= comb(k, d) * comb(target - f * k - d, target - f * k - 1);
            } else {
                res += comb(k, d) * comb(target - f * k - d, target - f * k - 1);
            }
        }
        m(res);
        return static_cast<int>(res);
    }
};
