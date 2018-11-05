class Solution {
public:
    int minFlipsMonoIncr(string S) {
        int l = S.size();
        
        // zeros[i] : '0' counted int S[0] ~ S[i] (S[i] not included)
        vector<int> zeros(l + 1, 0);
        for (int i = 1; i <= l; i++) {
            zeros[i] = zeros[i - 1] + (S[i - 1] == '0');
        }
        
        int minFlip = l;
        for (int i = 0; i <= l; i++) {
            // '0' / '1' counted int S[0] ~ S[i] (S[i] not included)
            int zerosLeft = zeros[i];
            int onesLeft = i - zerosLeft;
                
            // '0' / '1' counted int S[i] ~ S.end
            int zerosRight = zeros[l] - zeros[i];
            int onesRight = l - i - zerosRight;
            
            int flip = onesLeft + zerosRight;
            if (flip < minFlip) {
                minFlip = flip;
            }
        }
        return minFlip;
    }
};
