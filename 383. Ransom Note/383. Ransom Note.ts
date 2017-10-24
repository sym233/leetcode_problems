/**
 * @param {string} ransomNote
 * @param {string} magazine
 * @return {boolean}
 */
function canConstruct(ransomNote: string, magazine: string): boolean{
    if(ransomNote.length > magazine.length){
        return false;
    }

    const mag: number[] = [];
    const ran: number[] = [];

    const a = 'a'.codePointAt(0);

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

    saveLetter(mag, magazine);
    saveLetter(ran, ransomNote);

    for (const i in ran) {
        if (ran.hasOwnProperty(i) && mag.hasOwnProperty(i)) {
            if(ran[i] > mag[i]){
                return false
            }
        }else{
            return false;
        }
    }
    return true;
}
