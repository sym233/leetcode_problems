function longestPalindrome(words: string[]): number {
    let len = 0;
    const d: { [key: string]: number} = {};
    for (const word of words) {        
        const rev = word[1] + word[0];
        if (d[rev] > 0) {
            d[rev] -= 1;
            len += 4;
        } else {
            d[word] = (d[word] ?? 0) + 1;
        }
    }
    for (const word in d) {
        if (d[word] > 0 && word[0] === word[1]) {
            len += 2;
            break;
        }
    }
    return len;
};
