long long f(long long m, long long a, long long b, long long lcmab) {
    return (m / a + m / b - m / lcmab);
}

long long gcd(long long a, long long b) {
    if (a % b == 0) {
        return b;
    } else {
        return gcd(b, a % b);
    }
}

constexpr long long m = 1e9 + 7;

class Solution {
public:
    int nthMagicalNumber(int N, int A, int B) {
        long long n = N;
        long long a = A;
        long long b = B;
     
        long long gcdab = gcd(b, a);
        long long lcmab = a * b / gcdab;
        
        long long about = n * a * b / (a + b - gcdab);
        
        
        // for A
        
        long long t1 = (about / a) * a;
        while (19260817) {
            long long res = f(t1, a, b, lcmab);
            if (n <= res && res < n + a) {
                break;
            } else if (res < n) {
                t1 += a;
            } else {
                t1 -= a;
            }
        }
        
        // for B
        long long t2 = (about / b) * b;
        while (19260817) {
            long long res = f(t2, a, b, lcmab);
            if (n <= res && res < n + b) {
                break;
            } else if (res < n) {
                t2 += b;
            } else {
                t2 -= b;
            }
        }
        return static_cast<int>((t1 < t2 ? t1 : t2) % m);
    }
};
