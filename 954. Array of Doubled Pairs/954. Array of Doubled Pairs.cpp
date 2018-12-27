class Solution {
public:
    bool canReorderDoubled(vector<int>& A) {
        long long sum = 0;
        for (int a : A) {
            sum += a;
        }
        
        if (sum % 3 != 0) {
            return false;
        }
        
        map<int, int, bool(*)(int, int)> m([](int lhs, int rhs) {
            if (abs(lhs) == abs(rhs)) {
                return lhs < rhs;
            } else {
                return abs(lhs) < abs(rhs);
            }
        });
        
        for (int a : A) {
            m[a]++;
        }
        
        int numbersRemaining = A.size();
        
        for (auto it = m.begin(); it != m.end(); it++) {
            int num = it->first;
            int tot = it->second;
            
            if (tot > 0) {
                if (num == 0) {
                    if (tot % 2 == 0) {
                        numbersRemaining -= tot;
                    } else {
                        return false;
                    }
                } else if (m[num * 2] >= tot) {
                    m[num * 2] -= tot;
                    numbersRemaining -= tot * 2;
                } else {
                    return false;
                }
            }
        }
        
        return numbersRemaining == 0;
    }
};
