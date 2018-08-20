vector<int> res(1, 0);
// res[i] = j stands for j prefect suqare number(s) sum to i

class Solution {
public:
    int numSquares(int n) {
        if (n < res.size()) {
            return res[n];
        } else {
           
            for (int i = res.size(); i <= n; i++) {
                int minCount = res[i - 1] + 1;
                for (int j = 1; j * j <= i; j++) {
                    int k = i - j * j;
                    if (res[k] + 1 < minCount) {
                        minCount = res[k] + 1;
                    }
                }
               
                res.push_back(minCount);
            }

            return res[n];
        }
    }
};
