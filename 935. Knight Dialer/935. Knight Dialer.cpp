const int mod = 1e9 + 7;
const int MAX_MOVE = 5000;
vector<vector<int>> result;
unordered_map<int, vector<int>> nextPosition;
static int _ = [&](){
    result = vector<vector<int>>(10, vector<int>(MAX_MOVE, 0));
    nextPosition[0] = {4, 6};
    nextPosition[1] = {8, 6};
    nextPosition[2] = {7, 9};
    nextPosition[3] = {4, 8};
    nextPosition[4] = {3, 9, 0};
    nextPosition[5] = {};
    nextPosition[6] = {1, 7, 0};
    nextPosition[7] = {2, 6};
    nextPosition[8] = {1, 3};
    nextPosition[9] = {2, 4};

    for (int j = 0; j < MAX_MOVE; j++) {
        for (int i = 0; i < 10; i++) {
            if (j == 0) {
                result[i][j] = 1;
            } else {
                long long sum = 0;
                for (int nextPos : nextPosition[i]) {
                    sum += result[nextPos][j - 1] % mod;
                }
                result[i][j] = static_cast<int>(sum % mod);
            }
        }
    }
    return 0;
}();

class Solution {
public:
    int knightDialer(int N) {
        long long sum = 0;
        for (int i = 0; i < 10; i++) {
            sum += result[i][N - 1];
            sum %= mod;
        }
        return static_cast<int>(sum);
    }
};
