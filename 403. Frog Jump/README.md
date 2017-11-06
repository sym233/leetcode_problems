#403. Frog Jump

lastJump[i] presents that frog at previous stone can jump this unit(s) of length to the ith stone.

If lastJump[i] is undefined, it means stone[i] can't be reached yet.

Otherwise, try every possible jump lengths and check if the frog can reach a new stone.

lastJump[l-1] isn't undefined indicates the frog can reach the last stone.

It seems that Array.prototype.includes hasn't been entirely supported in Typescript.
