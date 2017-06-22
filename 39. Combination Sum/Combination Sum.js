/**
 * @param {number[]} candidates
 * @param {number} target
 * @return {number[][]}
 */
var combinationSum = function(candidates, target) {
    const res = [];
    function tryit(ar, remain){
        for(let i of candidates){
            const sub = remain - i;
            const l = ar.length;
            if(sub < 0){
                continue;
            }
            if(l === 0 || ar[l-1] <= i){
                if(sub === 0){
                    res.push(ar.concat(i));
                }
                if(sub > 0){
                    // console.log(ar, i);
                    tryit(ar.concat(i), remain - i);
                
                }
            }
        }
    }
    
    tryit([], target);
    return res;
};
