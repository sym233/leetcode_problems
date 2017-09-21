/**
 * @param {number} num
 * @return {number[]}
 */
var countBits = function(num) {
  const res = [];
  res[0] = 0;
  for(let i = 1; i <= num; i++){
    if(i & 1){
      // is odd
      res[i] = res[i>>1] + 1;
    }else{
      res[i] = res[i>>1];
    }
  }
  return res;
};
