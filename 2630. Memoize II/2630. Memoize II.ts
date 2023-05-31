type Fn = (...params: any) => any

function memoize(fn: Fn): Fn {
    class Cache {
        private c = new Map<any, Cache>();
        public value?;
        public has = this.c.has.bind(this.c);
        public get = this.c.get.bind(this.c);
        public set = this.c.set.bind(this.c);
    };
    const cache: Cache = new Cache();
    return function(...args) {
        let c: Cache = cache;
        const l = args.length
        for (let i = 0; i < l; i++) {
            if (!c.has(args[i])) {
                c.set(args[i], new Cache());
            }
            c = c.get(args[i]);
        }
        if ('value' in c) {
            return c.value;
        }
        return c.value = fn(...args);
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
