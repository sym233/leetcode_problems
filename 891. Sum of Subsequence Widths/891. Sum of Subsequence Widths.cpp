constexpr int m = 1e9 + 7;

const int MAX_N = 20002;

vector<long long> pow2(int n) {
    vector<long long> res(n, 1);
    for (int i = 1; i < n; i++) {
        res[i] = res[i - 1] * 2 % m;
    }
    return res;
}

const vector<long long> p2 = pow2(MAX_N);



class Solution {
public:
    int sumSubseqWidths(vector<int>& A) {
        sort(A.begin(), A.end());
        
        const int l = A.size();
        const int m = 1e9 + 7;
        
        long long sum = 0;
        
        // for (int i = 1; i < l; i++) {
        //     for (int j = 0; j < i; j++) {
        //         sum += (A[i] - A[j]) * p2[i - j - 1];
        //         sum %= m;
        //     }
        // }

        // use prefix sum to replace 2-d for-loop
        // decrease time complexity
	    // notice that 2^n - 1 = sum{2^i}, where i in [0, n)

        for (int i = 1; i < l; i++) {
            sum += A[i] * (p2[i] - 1);
            sum %= m;
        }
        
        for (int i = 0; i < l - 1; i++) {
            sum -= A[i] * (p2[l - i - 1] - 1);
            while (sum < 0) {
                sum += m;
            }
        }
        
        return static_cast<int>(sum);
    }
};
