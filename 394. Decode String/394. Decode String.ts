/**
 * @param {string} s
 * @return {string}
 */
function type(c: string): string{
    if(c === '['){
        return 'bra';
    }else if(c === ']'){
        return 'ket';
    }else if(c.match(/[0-9]/)){
        return 'number';
    }else if(c.match(/[a-z]/)){
        return 'letter';
    }else{
        throw new Error('unknown char');
    }
}
function decodeString(s: string): string{
    let i: number = 0;
    let j: number = 0;

    const l: number = s.length;

    let bras: number = 0; 
    // number of '['
    let kets: number = 0;
    // number of ']'
    let outerBra: number = 0;
    // first position of '['
    let rep: number = 0;
    // repeat number

    const toJoin: string[] = [];
    while(i < l){
        if(type(s[i]) === 'letter'){
            toJoin.push(s[i]);
        i++;
        }else{
            j = i + 1;
            rep = 0;
            while(j < l){
                bras = 0;
                kets = 0;
                if(type(s[j]) === 'bra'){
                    if(rep === 0){
                        rep = Number.parseInt(s.slice(i, j));
                        outerBra = j;
                    }
                    bras++;
                }
                if(type(s[j]) === 'ket'){
                    kets++;
                }
                if(bras === kets && bras > 0){
                    break;
                }
                j++;
            }
            toJoin.push(decodeString(s.slice(outerBra+1, j)).repeat(rep));
            i = j + 1;
        }
    }
    return toJoin.join('');
}
