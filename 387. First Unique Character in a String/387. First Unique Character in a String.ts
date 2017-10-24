/**
 * @param {string} s
 * @return {number}
 */

function firstUniqChar(s: string): number{
    const letters: number[] = [];
    const a = 'a'.codePointAt(0);
    function c2n(c: string): number{
        return c.codePointAt(0) - a;
    }
    function saveLetter(arr: number[], str: string): void{
        Array.prototype.forEach.call(str, element => {
            const i: number = element.codePointAt(0) - a;
            if(i in arr){
                arr[i]++;
            }else{
                arr[i] = 1;
            }
        });
    }

    saveLetter(letters, s);

    const l: number = s.length;
    for(let i = 0; i < l; i++){
        if(s.hasOwnProperty(i) && letters[c2n(s[i])] === 1){
            return i;
        }
    }
    return -1;
}
