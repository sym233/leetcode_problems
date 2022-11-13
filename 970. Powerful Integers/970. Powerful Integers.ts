function powerfulIntegers(x: number, y: number, bound: number): number[] {
    const res: Set<number> = new Set();
    const p: number[] = [];
    for (let i = 0;; i++) {
        let rem = bound - x ** i;
        if (rem <= 0) {
            break;
        }
        if (y === 1) {
            p[i] = 0;
        } else {
            p[i] = Math.round(Math.log(rem) / Math.log(y));
        }
        if (x === 1) {
            break;
        }
    }
    
    for (let i = 0; i in p; i++) {
        for (let j = 0; j <= p[i]; j++) {
            const s = x ** i + y ** j;
            if (s <= bound) {
                res.add(s);
            }
        }
    }
    return [...res];
};
