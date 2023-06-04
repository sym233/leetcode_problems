function curry(fn: Function): Function {
    return function curried(...args: any[]) {
        const f = fn.bind(undefined, ...args);
        if (f.length === 0) {
            return f();
        } else {
            return curry(f);
        }
    };
};

/**
 * function sum(a, b) { return a + b; }
 * const csum = curry(sum);
 * csum(1)(2) // 3
 */
