/**
 * @param {number[]} A
 * @return {number}
 */
var numberOfArithmeticSlices = function(A) {
    const l = A.length;
    if(l < 3){
        return 0;
    }
    const d = [];
    for(let i = 0; i < l - 1; i++){
        d[i] = A[i+1] - A[i];
    }
    
    const sames = [];
    
    let lastD = d[0]
    let j = 0
    // count same d
    for(let i = 1; i < l - 1; i++){
        if(d[i] !== lastD){
            sames.push(i-j);
            j = i;
            lastD = d[i];
        }
    }
    
    sames.push(l-1 - j);
    
    let total = 0;
    
    for(const same of sames){
        if(same >= 2){
        // calc sum of [1, 2, 3,... same-1]
            total += (1 + same-1)*(same - 1)/2;
        }
    }
    return total;
};
