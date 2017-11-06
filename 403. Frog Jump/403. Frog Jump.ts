/**
 * @param {number[]} stones
 * @return {boolean}
 */
function canCross(stones: number[]): boolean{
    const lastJumps: number[][] = [[0]];
    const l = stones.length;
    const jumpChange = [-1, 0, 1];
    for(let i = 0; i < l; i++){
        if(lastJumps[i] !== undefined){
            // stones[i] is reachable
            for(const distance of lastJumps[i]){
                for(const change of jumpChange){
                    const newJump = distance + change;
                    if(newJump > 0){
                        // only jump forwards
                        const reachableStone = stones.indexOf(stones[i] + newJump);
                        if(reachableStone !== -1){
                            // the frog can reach a stone.
                            if(lastJumps[reachableStone] === undefined){
                                lastJumps[reachableStone] = [newJump];
                            }else if(!lastJumps[reachableStone].includes(newJump)){
                                lastJumps[reachableStone].push(newJump);
                            }
                        }
                    }
                }
            }
        }
    }
    return (lastJumps[l-1] !== undefined);
}
