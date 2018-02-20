/**
 * @param {string} s
 * @param {string[]} wordDict
 * @return {boolean}
 */
export const wordBreak = function(s: string, wordDict: string[]): boolean {
    const l = s.length;
    const dp: boolean[] = [];
    // dp[i] === true -> s.slice(0, i) in dict
    dp[0] = true;
    for(let i = 1; i <= l; i++){
        dp[i] = false;
        for(let j = 0; j < i; j++){
            if(dp[j]){
                const cut = s.slice(j, i);
                if(wordDict.includes(cut)){
                    dp[i] = true;
                    break;
                }
            }
        }
    }
    return dp[l];
};
