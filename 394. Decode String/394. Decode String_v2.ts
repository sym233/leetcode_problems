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
