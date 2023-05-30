type Fn = (...params: any) => any;

function memoize(fn: Fn): Fn {
    type Cache = Map<any, any | Cache>;
    const cache: Cache = new Map();
    return function(...args) {
        let c: Cache = cache;
        let cached = true;
        const l = args.length
        for (let i = 0; i < l - 1; i++) {
            if (!c.has(args[i])) {
                cached = false;
                c.set(args[i], new Map())
            }
            c = c.get(args[i]);
        }
        if (cached && c.has(args[l - 1])) {
            return c.get(args[l - 1]);
        }
        const res = fn(...args);
        c.set(args[l - 1], res);
        return res;
    }
}


/** 
 * let callCount = 0;
 * const memoizedFn = memoize(function (a, b) {
 *	 callCount += 1;
 *   return a + b;
 * })
 * memoizedFn(2, 3) // 5
 * memoizedFn(2, 3) // 5
 * console.log(callCount) // 1 
 */
