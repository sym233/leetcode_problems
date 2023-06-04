function jsonStringify(object: any): string {
    if (typeof object === 'object') {
        if (object === null) {
            return 'null';
        }
        if (object instanceof Array) {
            return arr(object.map(v => jsonStringify(v)));
        }
        return obj(
            Object.entries(object)
            .map(([k, v]) => `${jsonStringify(k)}:${jsonStringify(v)}`)
        );
    }
    if (typeof object === 'string') {
        return `"${object}"`;
    }

    return `${object}`;
};

const obj = (items: string[]) => `{${items.join(',')}}`;
const arr = (items: string[]) => `[${items.join(',')}]`;
