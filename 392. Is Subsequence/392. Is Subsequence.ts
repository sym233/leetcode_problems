/**
 * @param {string} s
 * @param {string} t
 * @return {boolean}
 */
function isSubsequence(s: string, t: string): boolean{
    const ts = t.length;

    let j: number = 0;
    for(const c of s){
        while(t[j] !== c){
            j++;
            if(j >= ts){
                return false;
            }
        }
        j++;
    }
    return true;
}
