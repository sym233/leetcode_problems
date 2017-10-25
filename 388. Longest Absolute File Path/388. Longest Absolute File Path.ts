/**
 * @param {string} input
 * @return {number}
 */
function lengthLongestPath(input: string): number{
    const stack: string[] = [];

    const paths = input.split('\n');

    let maxLen = 0;

    function beginningTabs(path: string): {tabs: number, name: string}{
        const splitted = path.split('\t');
        let tabs = 0;
        let name = '';
        for(const str of splitted){
            if(str === ''){
                tabs++;
            }else{
                name = str;
            }
        }
        return {tabs, name};
    }

    function getPathLength(stack: string[]): number{
        // if it's a file
        const l = stack.length;
        if(stack[l-1].includes('.')){
            const path = stack.join('/');
            // console.log(path);
            return path.length;
        }else{
            return 0;
        }
    }

    for(const path of paths){
        const {tabs, name} = beginningTabs(path);
        stack[tabs] = name;
        stack.length = tabs + 1;

        const len = getPathLength(stack);
        if(len > maxLen){
            maxLen = len;
        }
    }

    return maxLen;
}
