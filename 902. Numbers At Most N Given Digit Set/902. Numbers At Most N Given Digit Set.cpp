class Solution {
public:
    vector<int> nums;
    int nl;
    
    int atMostNGivenDigitSet(vector<string>& D, int N) {
        int sum = 0;
        for (string s : D) {
            nums.push_back(s[0] - '0');
        }
        
        nl = nums.size();
        
        // bit number of N
        int l = log10(N) + 1;
        
        for (int i = 1; i < l; i++) {
            sum += pow(nl, i);
        }
        sum += tryNum(N, l);
        
        return sum;
    }
    
    int tryNum(int N, int l) {
        if (l == 0) {
            return 1;
        }
        int remain = static_cast<int>(pow(10, l - 1));
        int highestBit = N / remain;
        int sum = 0;
        
        for (int num : nums) {
            if (num < highestBit) {
                sum += pow(nl, l - 1);
            } else if (num == highestBit) {
                sum += tryNum(N % remain, l - 1);
            } else {
                break;
            }
        }
        return sum;
    }
};
