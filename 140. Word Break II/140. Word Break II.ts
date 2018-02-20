/**
 * @param {string} s
 * @param {string[]} wordDict
 * @return {boolean}
 */
import {wordBreak as wb1} from '../139. Word Break/139. Word Break';

const wordBreak = function(s: string, wordDict: string[]): string[] {
    if(!wb1(s, wordDict)){
        return [];
    }

    const l = s.length;
    const dp: string[][] = [];
    // dp[i].length > 0 -> s.slice(0, i) in dict
    dp[0] = [''];
    for(let i = 1; i <= l; i++){
        dp[i] = [];
        for(let j = 0; j < i; j++){
            if(dp[j]){
                const cut = s.slice(j, i);
                if(wordDict.includes(cut)){
                    for(const str of dp[j]){
                        dp[i].push(str + ' ' + cut);
                    }
                }
            }
        }
    }
    return dp[l].map(str=>str.slice(1));
};
