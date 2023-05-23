class TimeLimitedCache {
    // key => [expiry, value]
    private m = new Map<number, [number, number]>();
    private now = Date.now;

    set(key: number, value: number, duration: number): boolean {
        const expiry = this.m.get(key)?.[0] ?? 0;
        const now = this.now();
        this.m.set(key, [now + duration, value]);
        return expiry >= now;
    }

    get(key: number): number {
        const [expiry, value] = this.m.get(key) ?? [0, -1];
        if (expiry < this.now()) {
            this.m.delete(key);
            return -1;
        } else {
            return value;
        }
    }

	count(): number {
        const now = this.now();
        let c = 0;
        this.m.forEach(([expiry, _]) => {
            if (expiry >= now) {
                c++;
            }
        });
        return c;
    }
}

/**
 * Your TimeLimitedCache object will be instantiated and called as such:
 * var obj = new TimeLimitedCache()
 * obj.set(1, 42, 1000); // false
 * obj.get(1) // 42
 * obj.count() // 1
 */
