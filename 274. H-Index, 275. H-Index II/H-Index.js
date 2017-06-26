/**
 * @param {number[]} citations
 * @return {number}
 */
var hIndex = function(citations) {
    citations.sort((a,b)=>a-b);
    const l = citations.length;
    for(let i = 0; i < l; i++){
        if(citations[i] >= l-i){
            return l-i;
        }
    }
    return 0;
};

