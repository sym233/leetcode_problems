declare global {
    interface Array<T> {
        last(): T | -1;
    }
}

Array.prototype.last = function() {
    const l = this.length;
    if (l === 0) {
        return -1;
    } else {
        return this[l - 1];
    }
};

/**
 * const arr = [1, 2, 3];
 * arr.last(); // 3
 */

export {};
