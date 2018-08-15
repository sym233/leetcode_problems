/**
 * @param {string} s1
 * @param {string} s2
 * @return {boolean}
 */

function strToArr(s) {
    const letters = [];
    const l = s.length;
    for (let i = 0; i < l; i++) {
        const c = s.charCodeAt(i)
        if (letters[c]) {
            letters[c]++;
        } else {
            letters[c] = 1;
        }
    }
    return letters;
}

function compare(s1, s2) {
	// check if 2 strings have same letters
	
    if (s1.length !== s2.length) {
        return false;
    } else if (s1 === s2) {
        return true;
    } else {
        const letters1 = strToArr(s1);
        const letters2 = strToArr(s2);
        const l1 = letters1.length;
        if (l1 !== letters2.length) {
            return false;
        } else {
            for (let i = 0; i < l1; i++) {
                if (letters1[i] !== letters2[i]) {
                    return false;
                }
            }
            return true;
        }
        
    }
}
function trySplit(s1, s2) {
    if (s1 === s2) {
        return true;
    }
    const l = s1.length;
	
	// try to split both string to 2 parts
	// both part should be "scrambled" to the other string's

    for (let i = 1; i < l; i++) {
        // not swapped
        const s1l = s1.slice(0, i);
        const s1r = s1.slice(i);
        
        const s2l = s2.slice(0, i);
        const s2r = s2.slice(i);
        
        if (compare(s1l, s2l) && compare(s1r, s2r)) {
            if (trySplit(s1l, s2l) && trySplit(s1r, s2r)) {
                return true;
            }            
        }
    }
    for (let i = 1; i < l; i++) {
        // swapped
        const s1l = s1.slice(0, i);
        const s1r = s1.slice(i);
        
        const s2l = s2.slice(0, l - i);
        const s2r = s2.slice(l - i);
        
        if (compare(s1l, s2r) && compare(s1r, s2l)) {
            if (trySplit(s1l, s2r) && trySplit(s1r, s2l)) {
                return true;
            }            
        }
    }
    return false;
}

var isScramble = function(s1, s2) {
    return trySplit(s1, s2);
};
