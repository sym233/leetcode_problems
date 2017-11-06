<<<<<<< HEAD
/**
 * @param {string} s
 * @return {string}
 */
function decodeString(s: string): string{
    while(s.includes('[')){
        s = s.replace(/\d+\[\w+\]/, s=>{
            const i = s.indexOf('[');
            const j = s.indexOf(']');
            const r = Number.parseInt(s.slice(0, i));
            return s.slice(i+1, j).repeat(r);
        });
    }
    return s;
}
=======
/**
 * @param {string} s
 * @return {string}
 */
function decodeString(s: string): string{
    while(s.includes('[')){
        s = s.replace(/\d+\[\w+\]/, s=>{
            const i = s.indexOf('[');
            const j = s.indexOf(']');
            const r = Number.parseInt(s.slice(0, i));
            return s.slice(i+1, j).repeat(r);
        });
    }
    return s;
}
>>>>>>> fe31c562656e5af685945d16f91596bda82374e2
