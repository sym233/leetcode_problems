/**
 * @param {number} n
 * @return {number}
 */

// function A(k, n){
//     // Arrangement function
//     // k <= n
//     // A(k, n) = n! / (n-k)!
    
//     let res = 1;
//     for(let i = n-k+1; i <= n; i++){
//         res *= i;
//     }
    
//     return res;
// }

var countNumbersWithUniqueDigits = function(n) {
//     const res = [];
//     res[0] = 1;
//     res[1] = 10;
//     for(let i = 2; i <= 10; i++){
//         res[i] = 0;
//         // numbers begin with 0 (not i-bit numbers)
//         res[i] += res[i-1];
        
//         // numbers begin with 1~9
//         res[i] += 9 * A(i-1, 9);
//     }
    const res = [1, 10, 91, 739, 5275, 32491, 168571, 712891, 2345851, 5611771, 8877691];
    
    return res[n>10? 10: n];
};
