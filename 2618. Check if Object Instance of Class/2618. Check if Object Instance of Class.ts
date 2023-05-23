function checkIfInstanceOf(obj: any, classFunction: any): boolean {
    const prem = {
        number: Number,
        string: String,
        boolean: Boolean,
        symbol: Symbol,
        bigint: BigInt,
    }
    const t = typeof obj;
    if (t in prem && prem[t] === classFunction) {
        return true;
    }
    try {
        return obj instanceof classFunction;
    } catch {
        return false;
    }
};

/**
 * checkIfInstanceOf(new Date(), Date); // true
 */
