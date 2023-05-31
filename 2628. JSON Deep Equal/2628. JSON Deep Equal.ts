function areDeeplyEqual(o1: any, o2: any): boolean {
    if (typeof o1 !== typeof o2) {
        return false;
    }
    if (typeof o1 !== 'object') {
        return o1 === o2;
    }
    if (o1 instanceof Array) {
        if (o2 instanceof Array) {
            if (o1.length !== o2.length) {
                return false;
            }
            return o1.every((el, i) => areDeeplyEqual(el, o2[i]));
        }
        return false;
    }
    if (o2 instanceof Array) {
        return false;
    }
    const keys = new Set();
    for (const k in o1) {
        keys.add(k);
        if (!areDeeplyEqual(o1[k], o2[k])) {
            return false;
        }
    }
    for (const k in o2) {
        if (!keys.has(k)) {
            return false;
        }

    }
    return true;
};
